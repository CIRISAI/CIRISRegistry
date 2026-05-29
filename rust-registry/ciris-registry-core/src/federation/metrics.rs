//! Federation-namespace Prometheus metrics (v1.4 R_TEL).
//!
//! Per `docs/FEDERATION_CLIENT.md` §"Telemetry":
//!
//! - `federation_cache_hits_total{table}` — cached row served within TTL.
//! - `federation_cache_misses_total{table, reason}` — fell through to persist.
//!   `reason ∈ {ttl_expired, invalidated, absent}`.
//! - `federation_dual_write_divergence_total{table}` — cached row hash
//!   ≠ persist's authoritative row hash on read-through. Steady state 0;
//!   non-zero is a strong incident signal.
//! - `federation_cache_age_seconds{table}` (histogram) — distribution of
//!   cache-row ages at read time. Tunes `cache_ttl_seconds`.
//! - `federation_persist_request_latency_seconds{operation}` (histogram)
//!   — round-trip latency to persist for misses + writes.
//!
//! Call sites land alongside the `PersistFederationClient` real impl
//! once persist v0.2.0-pre1 ships. Until then, `register()` is invoked
//! at boot so all five families appear in `/metrics` with zero values
//! (so dashboards/alerts can be authored before federation traffic
//! actually flows).

use metrics::{counter, describe_counter, describe_histogram, histogram, Unit};

/// Cache table label values. Matches the three v1.4 R_MIG cache tables.
pub mod table {
    pub const TRUSTED_PRIMITIVE_KEYS: &str = "trusted_primitive_keys";
    pub const PARTNER_KEYS: &str = "partner_keys";
    pub const REGISTRY_SIGNING_KEYS: &str = "registry_signing_keys";
}

/// Cache-miss reason label values.
pub mod miss_reason {
    /// Cached row exists but `cached_at + cache_ttl_seconds < now`.
    pub const TTL_EXPIRED: &str = "ttl_expired";
    /// Cached row was invalidated by a registry-side write or
    /// divergence-detection event.
    pub const INVALIDATED: &str = "invalidated";
    /// No cached row at all.
    pub const ABSENT: &str = "absent";
}

/// Persist round-trip operation label values.
pub mod operation {
    pub const PUT_PUBLIC_KEY: &str = "put_public_key";
    pub const LOOKUP_PUBLIC_KEY: &str = "lookup_public_key";
    pub const LOOKUP_KEYS_FOR_IDENTITY: &str = "lookup_keys_for_identity";
    pub const PUT_ATTESTATION: &str = "put_attestation";
    pub const LIST_ATTESTATIONS_FOR: &str = "list_attestations_for";
    pub const LIST_ATTESTATIONS_BY: &str = "list_attestations_by";
    pub const PUT_REVOCATION: &str = "put_revocation";
    pub const REVOCATIONS_FOR: &str = "revocations_for";
}

/// Register `describe_*` metadata for all federation metric families.
///
/// Called once at boot from `main.rs` so the families appear in
/// `/metrics` with zero values from boot — lets dashboards and alerts
/// reference them before federation traffic flows.
pub fn register() {
    describe_counter!(
        "federation_cache_hits_total",
        Unit::Count,
        "Federation cache hits (row served within TTL, no persist round-trip)"
    );
    describe_counter!(
        "federation_cache_misses_total",
        Unit::Count,
        "Federation cache misses (fell through to persist). \
         reason ∈ {ttl_expired, invalidated, absent}"
    );
    describe_counter!(
        "federation_dual_write_divergence_total",
        Unit::Count,
        "Cached row hash differs from persist's authoritative row hash on \
         read-through. Steady state 0; non-zero is an incident signal."
    );
    describe_histogram!(
        "federation_cache_age_seconds",
        Unit::Seconds,
        "Distribution of cache-row ages at read time. Tunes cache_ttl_seconds."
    );
    describe_histogram!(
        "federation_persist_request_latency_seconds",
        Unit::Seconds,
        "Persist round-trip latency for cache misses + writes."
    );
}

#[inline]
pub fn cache_hit(table_label: &'static str) {
    counter!("federation_cache_hits_total", "table" => table_label).increment(1);
}

#[inline]
pub fn cache_miss(table_label: &'static str, reason_label: &'static str) {
    counter!(
        "federation_cache_misses_total",
        "table" => table_label,
        "reason" => reason_label
    )
    .increment(1);
}

#[inline]
pub fn divergence(table_label: &'static str) {
    counter!(
        "federation_dual_write_divergence_total",
        "table" => table_label
    )
    .increment(1);
}

#[inline]
pub fn record_cache_age(table_label: &'static str, age_seconds: f64) {
    histogram!("federation_cache_age_seconds", "table" => table_label).record(age_seconds);
}

#[inline]
pub fn record_persist_latency(operation_label: &'static str, latency_seconds: f64) {
    histogram!(
        "federation_persist_request_latency_seconds",
        "operation" => operation_label
    )
    .record(latency_seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_constants_are_stable() {
        // Wire contract: these labels must not silently change — alerts
        // and dashboards reference them by string. If you change a
        // value, update docs/FEDERATION_CLIENT.md §"Telemetry" too.
        assert_eq!(table::TRUSTED_PRIMITIVE_KEYS, "trusted_primitive_keys");
        assert_eq!(table::PARTNER_KEYS, "partner_keys");
        assert_eq!(table::REGISTRY_SIGNING_KEYS, "registry_signing_keys");
        assert_eq!(miss_reason::TTL_EXPIRED, "ttl_expired");
        assert_eq!(miss_reason::INVALIDATED, "invalidated");
        assert_eq!(miss_reason::ABSENT, "absent");
        assert_eq!(operation::PUT_PUBLIC_KEY, "put_public_key");
        assert_eq!(operation::LOOKUP_PUBLIC_KEY, "lookup_public_key");
    }

    #[test]
    fn register_does_not_panic() {
        register();
    }

    #[test]
    fn helpers_do_not_panic() {
        // Smoke test — recorder may not be installed in unit tests but
        // metrics! macros tolerate that path.
        cache_hit(table::TRUSTED_PRIMITIVE_KEYS);
        cache_miss(table::PARTNER_KEYS, miss_reason::TTL_EXPIRED);
        divergence(table::REGISTRY_SIGNING_KEYS);
        record_cache_age(table::TRUSTED_PRIMITIVE_KEYS, 12.5);
        record_persist_latency(operation::LOOKUP_PUBLIC_KEY, 0.025);
    }
}
