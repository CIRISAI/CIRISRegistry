//! Rate limiting and caching for integrity verification endpoints.
//!
//! Provides:
//! - Per-IP rate limiting for nonce generation
//! - Attestation deduplication (one attestation per device)
//! - Assertion result caching with TTL
//!
//! Based on Google Play Integrity and Apple App Attest best practices:
//! - Google: 10,000 requests/day, 5/min warm-up
//! - Apple: "double digits per second", one attestation per install

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

// =============================================================================
// Configuration
// =============================================================================

/// Maximum nonces per IP per minute
const NONCE_RATE_LIMIT_PER_MINUTE: u32 = 10;

/// Maximum nonces per IP per hour
const NONCE_RATE_LIMIT_PER_HOUR: u32 = 100;

/// Assertion cache TTL (5 minutes)
const ASSERTION_CACHE_TTL_SECONDS: u64 = 300;

/// Maximum entries in rate limit cache before cleanup
const MAX_RATE_LIMIT_ENTRIES: usize = 10000;

/// Maximum entries in assertion cache before cleanup
const MAX_ASSERTION_CACHE_ENTRIES: usize = 50000;

// =============================================================================
// Rate Limiter for Nonce Generation
// =============================================================================

#[derive(Clone)]
struct RateLimitEntry {
    /// Timestamps of requests in the last hour (unix seconds)
    request_times: Vec<u64>,
}

impl RateLimitEntry {
    fn new() -> Self {
        Self {
            request_times: Vec::new(),
        }
    }

    /// Clean old entries and check if request is allowed
    fn check_and_record(&mut self, now: u64) -> RateLimitResult {
        // Remove entries older than 1 hour
        let one_hour_ago = now.saturating_sub(3600);
        self.request_times.retain(|&t| t > one_hour_ago);

        // Count requests in last minute
        let one_minute_ago = now.saturating_sub(60);
        let requests_last_minute = self.request_times.iter().filter(|&&t| t > one_minute_ago).count() as u32;

        // Count requests in last hour
        let requests_last_hour = self.request_times.len() as u32;

        // Check limits
        if requests_last_minute >= NONCE_RATE_LIMIT_PER_MINUTE {
            return RateLimitResult::ExceededMinute {
                limit: NONCE_RATE_LIMIT_PER_MINUTE,
                retry_after_seconds: 60,
            };
        }

        if requests_last_hour >= NONCE_RATE_LIMIT_PER_HOUR {
            return RateLimitResult::ExceededHour {
                limit: NONCE_RATE_LIMIT_PER_HOUR,
                retry_after_seconds: 3600,
            };
        }

        // Record this request
        self.request_times.push(now);

        RateLimitResult::Allowed {
            remaining_minute: NONCE_RATE_LIMIT_PER_MINUTE - requests_last_minute - 1,
            remaining_hour: NONCE_RATE_LIMIT_PER_HOUR - requests_last_hour - 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RateLimitResult {
    Allowed {
        remaining_minute: u32,
        remaining_hour: u32,
    },
    ExceededMinute {
        limit: u32,
        retry_after_seconds: u32,
    },
    ExceededHour {
        limit: u32,
        retry_after_seconds: u32,
    },
}

impl RateLimitResult {
    pub fn is_allowed(&self) -> bool {
        matches!(self, RateLimitResult::Allowed { .. })
    }

    pub fn error_message(&self) -> Option<String> {
        match self {
            RateLimitResult::Allowed { .. } => None,
            RateLimitResult::ExceededMinute { limit, retry_after_seconds } => {
                Some(format!(
                    "Rate limit exceeded: {} requests/minute. Retry after {} seconds.",
                    limit, retry_after_seconds
                ))
            }
            RateLimitResult::ExceededHour { limit, retry_after_seconds } => {
                Some(format!(
                    "Rate limit exceeded: {} requests/hour. Retry after {} seconds.",
                    limit, retry_after_seconds
                ))
            }
        }
    }

    pub fn retry_after(&self) -> Option<u32> {
        match self {
            RateLimitResult::Allowed { .. } => None,
            RateLimitResult::ExceededMinute { retry_after_seconds, .. } => Some(*retry_after_seconds),
            RateLimitResult::ExceededHour { retry_after_seconds, .. } => Some(*retry_after_seconds),
        }
    }
}

/// Global rate limiter for nonce generation
static NONCE_RATE_LIMITER: std::sync::OnceLock<Mutex<HashMap<IpAddr, RateLimitEntry>>> =
    std::sync::OnceLock::new();

fn get_nonce_rate_limiter() -> &'static Mutex<HashMap<IpAddr, RateLimitEntry>> {
    NONCE_RATE_LIMITER.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Check if an IP is allowed to generate a nonce
pub fn check_nonce_rate_limit(ip: IpAddr) -> RateLimitResult {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut limiter = get_nonce_rate_limiter().lock().unwrap();

    // Cleanup if too many entries
    if limiter.len() > MAX_RATE_LIMIT_ENTRIES {
        cleanup_rate_limiter(&mut limiter, now);
    }

    let entry = limiter.entry(ip).or_insert_with(RateLimitEntry::new);
    let result = entry.check_and_record(now);

    if !result.is_allowed() {
        warn!(
            ip = %ip,
            error = ?result.error_message(),
            "nonce_rate_limit_exceeded"
        );
    }

    result
}

/// Check rate limit without recording (for inspection)
pub fn check_nonce_rate_limit_status(ip: IpAddr) -> Option<(u32, u32)> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let limiter = get_nonce_rate_limiter().lock().unwrap();

    limiter.get(&ip).map(|entry| {
        let one_minute_ago = now.saturating_sub(60);
        let one_hour_ago = now.saturating_sub(3600);

        let requests_last_minute = entry.request_times.iter().filter(|&&t| t > one_minute_ago).count() as u32;
        let requests_last_hour = entry.request_times.iter().filter(|&&t| t > one_hour_ago).count() as u32;

        (
            NONCE_RATE_LIMIT_PER_MINUTE.saturating_sub(requests_last_minute),
            NONCE_RATE_LIMIT_PER_HOUR.saturating_sub(requests_last_hour),
        )
    })
}

fn cleanup_rate_limiter(limiter: &mut HashMap<IpAddr, RateLimitEntry>, now: u64) {
    let one_hour_ago = now.saturating_sub(3600);

    // Remove entries with no recent activity
    limiter.retain(|_, entry| {
        entry.request_times.iter().any(|&t| t > one_hour_ago)
    });

    info!(
        remaining_entries = limiter.len(),
        "rate_limiter_cleanup_complete"
    );
}

// =============================================================================
// Assertion Result Cache
// =============================================================================

#[derive(Clone)]
struct CachedAssertionResult {
    verified: bool,
    counter: u32,
    cached_at: u64,
    expires_at: u64,
}

/// Global assertion result cache
/// Key: (key_id, client_data_hash)
static ASSERTION_CACHE: std::sync::OnceLock<Mutex<HashMap<(String, String), CachedAssertionResult>>> =
    std::sync::OnceLock::new();

fn get_assertion_cache() -> &'static Mutex<HashMap<(String, String), CachedAssertionResult>> {
    ASSERTION_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Result of assertion cache lookup
#[derive(Debug, Clone)]
pub enum AssertionCacheResult {
    /// Cache hit - return cached result
    Hit {
        verified: bool,
        counter: u32,
        cached_at: u64,
    },
    /// Cache miss - need to verify
    Miss,
    /// Cache hit but expired - need to re-verify
    Expired,
}

/// Check assertion cache
pub fn check_assertion_cache(key_id: &str, client_data_hash: &str) -> AssertionCacheResult {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let cache = get_assertion_cache().lock().unwrap();
    let cache_key = (key_id.to_string(), client_data_hash.to_string());

    match cache.get(&cache_key) {
        Some(entry) if entry.expires_at > now => {
            info!(
                key_id = key_id,
                ttl_remaining = entry.expires_at - now,
                "assertion_cache_hit"
            );
            AssertionCacheResult::Hit {
                verified: entry.verified,
                counter: entry.counter,
                cached_at: entry.cached_at,
            }
        }
        Some(_) => {
            info!(key_id = key_id, "assertion_cache_expired");
            AssertionCacheResult::Expired
        }
        None => AssertionCacheResult::Miss,
    }
}

/// Store assertion result in cache
pub fn cache_assertion_result(key_id: &str, client_data_hash: &str, verified: bool, counter: u32) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut cache = get_assertion_cache().lock().unwrap();

    // Cleanup if too many entries
    if cache.len() > MAX_ASSERTION_CACHE_ENTRIES {
        cleanup_assertion_cache(&mut cache, now);
    }

    let cache_key = (key_id.to_string(), client_data_hash.to_string());
    cache.insert(
        cache_key,
        CachedAssertionResult {
            verified,
            counter,
            cached_at: now,
            expires_at: now + ASSERTION_CACHE_TTL_SECONDS,
        },
    );

    info!(
        key_id = key_id,
        verified = verified,
        ttl = ASSERTION_CACHE_TTL_SECONDS,
        "assertion_result_cached"
    );
}

/// Invalidate cached assertions for a key (e.g., when counter changes unexpectedly)
pub fn invalidate_assertion_cache(key_id: &str) {
    let mut cache = get_assertion_cache().lock().unwrap();
    cache.retain(|(k, _), _| k != key_id);
    info!(key_id = key_id, "assertion_cache_invalidated");
}

fn cleanup_assertion_cache(cache: &mut HashMap<(String, String), CachedAssertionResult>, now: u64) {
    cache.retain(|_, entry| entry.expires_at > now);
    info!(
        remaining_entries = cache.len(),
        "assertion_cache_cleanup_complete"
    );
}

// =============================================================================
// Attestation Deduplication
// =============================================================================

/// Check if a key_id has already been attested (prevents re-attestation)
pub async fn is_already_attested(pool: &sqlx::PgPool, key_id: &str) -> Result<bool, sqlx::Error> {
    let result: Option<(i32,)> = sqlx::query_as(
        "SELECT 1 FROM app_attest_keys WHERE key_id = $1"
    )
    .bind(key_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}

/// Check if attestation is suspicious (e.g., too many attestations from same characteristics)
/// This could be expanded to check device fingerprints, IP patterns, etc.
pub async fn check_attestation_suspicious(
    pool: &sqlx::PgPool,
    app_id_hash: &[u8],
    ip: Option<IpAddr>,
) -> Result<AttestationCheckResult, sqlx::Error> {
    // Count recent attestations with same app_id_hash (last 24 hours)
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM app_attest_keys
        WHERE app_id_hash = $1
        AND created_at > NOW() - INTERVAL '24 hours'
        "#
    )
    .bind(app_id_hash)
    .fetch_one(pool)
    .await?;

    // More than 100 attestations per app in 24h is suspicious
    if count.0 > 100 {
        warn!(
            app_id_hash = hex::encode(app_id_hash),
            count = count.0,
            "suspicious_attestation_volume"
        );
        return Ok(AttestationCheckResult::Suspicious {
            reason: "High volume of attestations from this app".to_string(),
        });
    }

    // Could add more checks here:
    // - IP-based patterns
    // - Time-based patterns
    // - Device fingerprint clustering

    Ok(AttestationCheckResult::Ok)
}

#[derive(Debug, Clone)]
pub enum AttestationCheckResult {
    Ok,
    Suspicious { reason: String },
}

impl AttestationCheckResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, AttestationCheckResult::Ok)
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Get current rate limiter statistics
pub fn get_rate_limiter_stats() -> RateLimiterStats {
    let nonce_limiter = get_nonce_rate_limiter().lock().unwrap();
    let assertion_cache = get_assertion_cache().lock().unwrap();

    RateLimiterStats {
        nonce_tracked_ips: nonce_limiter.len(),
        assertion_cached_entries: assertion_cache.len(),
    }
}

#[derive(Debug, Clone)]
pub struct RateLimiterStats {
    pub nonce_tracked_ips: usize,
    pub assertion_cached_entries: usize,
}
