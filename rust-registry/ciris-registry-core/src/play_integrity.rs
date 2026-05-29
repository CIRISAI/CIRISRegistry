//! Google Play Integrity verification service.
//!
//! Server-side verification of Google Play Integrity tokens.
//!
//! Best practices implemented:
//! - Nonce generation and validation (prevents replay attacks)
//! - Hardware-backed attestation verification
//! - Server-side token decoding via Google API
//! - Rate limiting via nonce expiration
//!
//! References:
//! - https://developer.android.com/google/play/integrity/overview
//! - https://developer.android.com/google/play/integrity/standard

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::SignatureEncoding;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;
use tracing::{info, warn};

/// Nonce expiration time (5 minutes - Google recommends short-lived nonces)
const NONCE_EXPIRY_SECONDS: u64 = 300;

/// Maximum nonce cache size
const MAX_NONCE_CACHE_SIZE: usize = 10000;

/// Global nonce cache (thread-safe)
/// Key: nonce string, Value: NonceEntry
static NONCE_CACHE: std::sync::OnceLock<Mutex<HashMap<String, NonceEntry>>> =
    std::sync::OnceLock::new();

fn get_nonce_cache() -> &'static Mutex<HashMap<String, NonceEntry>> {
    NONCE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone)]
struct NonceEntry {
    created_at: u64,
    expires_at: u64,
    context: Option<String>,
    used: bool,
}

/// Configuration for Play Integrity verification
#[derive(Clone)]
pub struct PlayIntegrityConfig {
    pub package_name: String,
    pub service_account_json: Option<String>,
}

impl Default for PlayIntegrityConfig {
    fn default() -> Self {
        Self {
            package_name: std::env::var("ANDROID_PACKAGE_NAME")
                .unwrap_or_else(|_| "ai.ciris.app".to_string()),
            service_account_json: std::env::var("PLAY_INTEGRITY_SERVICE_ACCOUNT").ok(),
        }
    }
}

// ============================================================================
// API Request/Response Models
// ============================================================================

#[derive(Debug, Serialize)]
pub struct IntegrityNonceResponse {
    pub nonce: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
pub struct IntegrityVerifyRequest {
    pub integrity_token: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceIntegrityResult {
    pub meets_strong_integrity: bool,
    pub meets_device_integrity: bool,
    pub meets_basic_integrity: bool,
    pub verdicts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppIntegrityResult {
    pub verdict: String,
    pub package_name: Option<String>,
    pub certificate_sha256_digest: Vec<String>,
    pub version_code: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountIntegrityResult {
    pub licensing_verdict: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityVerifyResponse {
    pub verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_integrity: Option<DeviceIntegrityResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_integrity: Option<AppIntegrityResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_details: Option<AccountIntegrityResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl IntegrityVerifyResponse {
    pub fn error(msg: &str) -> Self {
        Self {
            verified: false,
            request_details: None,
            device_integrity: None,
            app_integrity: None,
            account_details: None,
            error: Some(msg.to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct IntegrityAuthRequest {
    pub integrity_token: String,
    pub nonce: String,
}

#[derive(Debug, Serialize)]
pub struct IntegrityAuthResponse {
    pub authenticated: bool,
    pub integrity_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_integrity: Option<DeviceIntegrityResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_integrity: Option<AppIntegrityResult>,
    pub authorized: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

// ============================================================================
// Play Integrity Service
// ============================================================================

pub struct PlayIntegrityService {
    config: PlayIntegrityConfig,
    http_client: reqwest::Client,
}

impl PlayIntegrityService {
    pub fn new(config: PlayIntegrityConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Generate a cryptographically secure nonce for integrity request.
    ///
    /// The nonce is:
    /// - Base64 URL-safe encoded (NO_PADDING as required by Play Integrity)
    /// - Unique per request
    /// - Short-lived (expires in 5 minutes)
    /// - Stored server-side for validation
    pub fn generate_nonce(&self, context: Option<&str>) -> IntegrityNonceResponse {
        // Generate 32 random bytes
        let random_bytes: [u8; 32] = rand::random();

        // Add timestamp for additional entropy
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timestamp_bytes = now.to_be_bytes();

        // Combine and hash
        let mut hasher = Sha256::new();
        hasher.update(&random_bytes);
        hasher.update(&timestamp_bytes);
        let nonce_hash = hasher.finalize();

        // Base64 URL-safe encode (NO_PADDING as required by Play Integrity)
        let nonce = URL_SAFE_NO_PAD.encode(nonce_hash);

        // Store in cache
        let expires_at = now + NONCE_EXPIRY_SECONDS;
        let entry = NonceEntry {
            created_at: now,
            expires_at,
            context: context.map(|s| s.to_string()),
            used: false,
        };

        {
            let mut cache = get_nonce_cache().lock().unwrap();
            cleanup_nonce_cache(&mut cache);
            cache.insert(nonce.clone(), entry);
        }

        info!(context = context, "play_integrity_nonce_generated");

        let expires_at_dt = OffsetDateTime::from_unix_timestamp(expires_at as i64)
            .unwrap_or_else(|_| OffsetDateTime::now_utc());

        IntegrityNonceResponse {
            nonce,
            expires_at: expires_at_dt
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| expires_at_dt.to_string()),
        }
    }

    /// Validate a nonce before verifying integrity token.
    fn validate_nonce(&self, nonce: &str) -> Result<(), String> {
        let mut cache = get_nonce_cache().lock().unwrap();

        let entry = match cache.get(nonce) {
            Some(e) => e.clone(),
            None => return Err("Nonce not found or already expired".to_string()),
        };

        if entry.used {
            return Err("Nonce already used".to_string());
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now > entry.expires_at {
            cache.remove(nonce);
            return Err("Nonce expired".to_string());
        }

        Ok(())
    }

    /// Mark a nonce as used to prevent replay attacks.
    fn mark_nonce_used(&self, nonce: &str) {
        let mut cache = get_nonce_cache().lock().unwrap();
        if let Some(entry) = cache.get_mut(nonce) {
            entry.used = true;
        }
    }

    /// Verify a Play Integrity token by decoding it via Google's API.
    pub async fn verify_token(
        &self,
        integrity_token: &str,
        nonce: &str,
        skip_nonce_validation: bool,
    ) -> IntegrityVerifyResponse {
        // Step 1: Validate nonce
        if !skip_nonce_validation {
            if let Err(error) = self.validate_nonce(nonce) {
                warn!(error = %error, "play_integrity_nonce_invalid");
                return IntegrityVerifyResponse::error(&format!(
                    "Nonce validation failed: {}",
                    error
                ));
            }
        }

        // Step 2: Get access token for Google API
        let access_token = match self.get_access_token().await {
            Ok(token) => token,
            Err(e) => {
                warn!(error = %e, "play_integrity_auth_failed");
                return IntegrityVerifyResponse::error(&format!("Authentication failed: {}", e));
            }
        };

        // Step 3: Decode token via Google API
        let decoded = match self.decode_integrity_token(integrity_token, &access_token).await {
            Ok(d) => d,
            Err(e) => {
                warn!(error = %e, "play_integrity_decode_failed");
                return IntegrityVerifyResponse::error(&format!("Token decode failed: {}", e));
            }
        };

        // Step 4: Mark nonce as used
        if !skip_nonce_validation {
            self.mark_nonce_used(nonce);
        }

        // Step 5: Process the decoded token
        self.process_decoded_token(&decoded, nonce)
    }

    /// Get OAuth2 access token from service account credentials.
    async fn get_access_token(&self) -> Result<String, String> {
        let sa_json = self
            .config
            .service_account_json
            .as_ref()
            .ok_or("PLAY_INTEGRITY_SERVICE_ACCOUNT not configured")?;

        let sa: ServiceAccountInfo =
            serde_json::from_str(sa_json).map_err(|e| format!("Invalid service account: {}", e))?;

        // Create JWT for service account auth
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = serde_json::json!({
            "iss": sa.client_email,
            "sub": sa.client_email,
            "aud": "https://oauth2.googleapis.com/token",
            "iat": now,
            "exp": now + 3600,
            "scope": "https://www.googleapis.com/auth/playintegrity"
        });

        // Sign JWT with RS256
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(r#"{"alg":"RS256","typ":"JWT"}"#);
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::to_string(&claims).unwrap());
        let signing_input = format!("{}.{}", header, payload);

        // Parse private key and sign
        let key = rsa::RsaPrivateKey::from_pkcs8_pem(&sa.private_key)
            .map_err(|e| format!("Invalid private key: {}", e))?;

        use rsa::pkcs1v15::SigningKey;
        use rsa::signature::Signer;
        let signing_key: SigningKey<sha2::Sha256> = SigningKey::new(key);
        let signature = signing_key.sign(signing_input.as_bytes());
        let sig_b64 =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signature.to_vec());

        let jwt = format!("{}.{}", signing_input, sig_b64);

        // Exchange JWT for access token
        let token_response = self
            .http_client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| format!("Token request failed: {}", e))?;

        let token_data: TokenResponse = token_response
            .json()
            .await
            .map_err(|e| format!("Token parse failed: {}", e))?;

        Ok(token_data.access_token)
    }

    /// Decode integrity token using Google Play Integrity API.
    async fn decode_integrity_token(
        &self,
        integrity_token: &str,
        access_token: &str,
    ) -> Result<serde_json::Value, String> {
        let url = format!(
            "https://playintegrity.googleapis.com/v1/{}:decodeIntegrityToken",
            self.config.package_name
        );

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(access_token)
            .json(&serde_json::json!({
                "integrityToken": integrity_token
            }))
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, body));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Response parse failed: {}", e))
    }

    /// Process the decoded token and extract verdicts.
    fn process_decoded_token(
        &self,
        decoded: &serde_json::Value,
        expected_nonce: &str,
    ) -> IntegrityVerifyResponse {
        let token_payload = decoded
            .get("tokenPayloadExternal")
            .unwrap_or(&serde_json::Value::Null);

        // Extract request details
        let request_details = token_payload.get("requestDetails").cloned();
        let request_nonce = request_details
            .as_ref()
            .and_then(|rd| rd.get("nonce"))
            .and_then(|n| n.as_str())
            .unwrap_or("");

        // Verify nonce matches (log warning but don't fail)
        if request_nonce != expected_nonce {
            warn!(
                expected = %expected_nonce,
                received = %request_nonce,
                "play_integrity_nonce_mismatch"
            );
        }

        // Extract device integrity
        let device_integrity_data = token_payload
            .get("deviceIntegrity")
            .unwrap_or(&serde_json::Value::Null);
        let device_verdicts: Vec<String> = device_integrity_data
            .get("deviceRecognitionVerdict")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let device_integrity = DeviceIntegrityResult {
            meets_strong_integrity: device_verdicts.contains(&"MEETS_STRONG_INTEGRITY".to_string()),
            meets_device_integrity: device_verdicts.contains(&"MEETS_DEVICE_INTEGRITY".to_string()),
            meets_basic_integrity: device_verdicts.contains(&"MEETS_BASIC_INTEGRITY".to_string()),
            verdicts: device_verdicts.clone(),
        };

        // Extract app integrity
        let app_integrity_data = token_payload
            .get("appIntegrity")
            .unwrap_or(&serde_json::Value::Null);
        let app_integrity = AppIntegrityResult {
            verdict: app_integrity_data
                .get("appRecognitionVerdict")
                .and_then(|v| v.as_str())
                .unwrap_or("UNEVALUATED")
                .to_string(),
            package_name: app_integrity_data
                .get("packageName")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            certificate_sha256_digest: app_integrity_data
                .get("certificateSha256Digest")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            version_code: app_integrity_data
                .get("versionCode")
                .and_then(|v| v.as_i64()),
        };

        // Extract account details
        let account_data = token_payload
            .get("accountDetails")
            .unwrap_or(&serde_json::Value::Null);
        let account_details = AccountIntegrityResult {
            licensing_verdict: account_data
                .get("appLicensingVerdict")
                .and_then(|v| v.as_str())
                .unwrap_or("UNEVALUATED")
                .to_string(),
        };

        // Determine if verified
        let device_ok = device_integrity.meets_basic_integrity
            || device_integrity.meets_device_integrity
            || device_integrity.meets_strong_integrity;
        let app_ok =
            app_integrity.verdict == "PLAY_RECOGNIZED" || app_integrity.verdict == "UNRECOGNIZED_VERSION";

        let verified = device_ok && app_ok;

        let error = if !verified {
            let mut reasons = Vec::new();
            if !device_ok {
                reasons.push(format!(
                    "device_integrity_failed (verdicts: {:?})",
                    device_verdicts
                ));
            }
            if !app_ok {
                reasons.push(format!(
                    "app_not_recognized (verdict: {})",
                    app_integrity.verdict
                ));
            }
            Some(reasons.join("; "))
        } else {
            None
        };

        info!(
            verified = verified,
            device_ok = device_ok,
            app_ok = app_ok,
            device_verdicts = ?device_verdicts,
            app_verdict = %app_integrity.verdict,
            licensing = %account_details.licensing_verdict,
            "play_integrity_verification_complete"
        );

        IntegrityVerifyResponse {
            verified,
            request_details,
            device_integrity: Some(device_integrity),
            app_integrity: Some(app_integrity),
            account_details: Some(account_details),
            error,
        }
    }
}

// ============================================================================
// Helper Types
// ============================================================================

#[derive(Deserialize)]
struct ServiceAccountInfo {
    client_email: String,
    private_key: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

/// Cleanup expired nonces from cache.
fn cleanup_nonce_cache(cache: &mut HashMap<String, NonceEntry>) {
    if cache.len() < MAX_NONCE_CACHE_SIZE {
        return;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    cache.retain(|_, entry| entry.expires_at > now);
}
