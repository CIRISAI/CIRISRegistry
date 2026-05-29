//! 24-hour TTL discipline for `holds_bytes:sha256:{prefix}` attestations.
//!
//! Per CEG 0.2 §10.1.2:
//!
//! > A `holds_bytes:sha256:{prefix}` attestation has a default validity of
//! > **24 hours** from `signed_at`. After that the holder is considered
//! > stale; consumer policy MUST attempt at most 2 holders in parallel
//! > and accept the first successful full-SHA verification.
//!
//! This module ships the pure-logic helper for the TTL filter. Holders
//! that exceed `DEFAULT_HOLDS_BYTES_TTL_SECONDS` (24h) are excluded;
//! the caller obtains the filtered set and routes ContentFetch to up
//! to 2 of those.

use chrono::{DateTime, Duration, Utc};

/// Default TTL for `holds_bytes:sha256:{prefix}` attestations per CEG 0.2
/// §10.1.2 — 24 hours from `signed_at`. Mirrors the same constant
/// `ciris_edge::EdgeConfig::holds_bytes_ttl_seconds` defaults to.
pub const DEFAULT_HOLDS_BYTES_TTL_SECONDS: u64 = 86_400;

/// Result of evaluating a single `holds_bytes` attestation's freshness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoldsBytesFreshness {
    /// Attestation is within the TTL window — holder is live.
    Fresh,
    /// Attestation has aged past the TTL — holder is stale; consumer
    /// SHOULD NOT route ContentFetch here and SHOULD emit a `withdraws`
    /// against the attestation per CEG §10.1.2.
    Stale,
}

/// Filter a list of `(holder_key_id, signed_at)` candidates to those
/// whose `signed_at` is within `ttl_seconds` of `now`.
///
/// `ttl_seconds` typically [`DEFAULT_HOLDS_BYTES_TTL_SECONDS`]; tighter
/// thresholds MAY be applied per consumer policy.
///
/// Returns the input order preserved (no implicit sort) so the caller
/// can apply its own ranking (e.g., by Reticulum reachability score)
/// before slicing the top-2 per CEG §10.1.2.
pub fn filter_fresh_holders<'a, I>(
    candidates: I,
    now: DateTime<Utc>,
    ttl_seconds: u64,
) -> Vec<&'a str>
where
    I: IntoIterator<Item = (&'a str, DateTime<Utc>)>,
{
    let cutoff = now - Duration::seconds(ttl_seconds as i64);
    candidates
        .into_iter()
        .filter(|(_, signed_at)| *signed_at >= cutoff)
        .map(|(holder, _)| holder)
        .collect()
}

/// Classify a single holder's freshness given the attestation's
/// `signed_at` and the current time.
pub fn classify_freshness(
    signed_at: DateTime<Utc>,
    now: DateTime<Utc>,
    ttl_seconds: u64,
) -> HoldsBytesFreshness {
    let cutoff = now - Duration::seconds(ttl_seconds as i64);
    if signed_at >= cutoff {
        HoldsBytesFreshness::Fresh
    } else {
        HoldsBytesFreshness::Stale
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn ts(year: i32, month: u32, day: u32, hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap()
    }

    #[test]
    fn filter_fresh_holders_keeps_within_ttl() {
        let now = ts(2026, 5, 29, 12);
        let holders = vec![
            ("fresh-a", ts(2026, 5, 28, 14)), // 22h old → fresh
            ("stale-b", ts(2026, 5, 28, 10)), // 26h old → stale
            ("fresh-c", ts(2026, 5, 29, 11)), // 1h old → fresh
        ];

        let result = filter_fresh_holders(holders, now, DEFAULT_HOLDS_BYTES_TTL_SECONDS);
        assert_eq!(result, vec!["fresh-a", "fresh-c"]);
    }

    #[test]
    fn filter_fresh_holders_preserves_input_order() {
        let now = ts(2026, 5, 29, 12);
        let holders = vec![
            ("c", ts(2026, 5, 29, 11)),
            ("a", ts(2026, 5, 29, 10)),
            ("b", ts(2026, 5, 29, 9)),
        ];

        let result = filter_fresh_holders(holders, now, DEFAULT_HOLDS_BYTES_TTL_SECONDS);
        assert_eq!(result, vec!["c", "a", "b"]);
    }

    #[test]
    fn filter_fresh_holders_empty_input_returns_empty() {
        let now = ts(2026, 5, 29, 12);
        let holders: Vec<(&str, DateTime<Utc>)> = vec![];
        let result = filter_fresh_holders(holders, now, DEFAULT_HOLDS_BYTES_TTL_SECONDS);
        assert!(result.is_empty());
    }

    #[test]
    fn filter_fresh_holders_all_stale_returns_empty() {
        let now = ts(2026, 5, 29, 12);
        let holders = vec![
            ("a", ts(2026, 5, 27, 12)), // 48h
            ("b", ts(2026, 5, 26, 12)), // 72h
        ];
        let result = filter_fresh_holders(holders, now, DEFAULT_HOLDS_BYTES_TTL_SECONDS);
        assert!(result.is_empty());
    }

    #[test]
    fn classify_freshness_at_exact_cutoff_is_fresh() {
        let now = ts(2026, 5, 29, 12);
        let exactly_24h_ago = now - Duration::hours(24);
        assert_eq!(
            classify_freshness(exactly_24h_ago, now, DEFAULT_HOLDS_BYTES_TTL_SECONDS),
            HoldsBytesFreshness::Fresh
        );
    }

    #[test]
    fn classify_freshness_one_second_past_cutoff_is_stale() {
        let now = ts(2026, 5, 29, 12);
        let just_past = now - Duration::hours(24) - Duration::seconds(1);
        assert_eq!(
            classify_freshness(just_past, now, DEFAULT_HOLDS_BYTES_TTL_SECONDS),
            HoldsBytesFreshness::Stale
        );
    }

    #[test]
    fn tighter_ttl_excludes_holders_default_ttl_would_keep() {
        let now = ts(2026, 5, 29, 12);
        let holders = vec![
            ("a", ts(2026, 5, 29, 8)), // 4h ago
            ("b", ts(2026, 5, 29, 11)), // 1h ago
        ];

        // 2-hour TTL — only "b" stays.
        let result = filter_fresh_holders(holders, now, 7200);
        assert_eq!(result, vec!["b"]);
    }
}
