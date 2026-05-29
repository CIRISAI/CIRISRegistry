//! Audit-log metadata keys for federation operations (v1.4 R_AUDIT).
//!
//! Per `docs/FEDERATION_CLIENT.md` §"Audit-log shape — RESOLVED":
//!
//! > Forensic query "who attested key K" walks
//! > `audit_log WHERE metadata->>envelope_hash = ...` joined to persist's
//! > `journal WHERE original_content_hash = ...`.
//!
//! The registry's existing `audit_log.metadata` JSONB column carries
//! the join key — no schema change needed. This module locks down the
//! key names and provides a typed helper so the dual-write handlers
//! (R_DUAL_WRITE, future) can populate them without inventing
//! ad-hoc strings at the call site.
//!
//! **Wire contract**: these key names are forensic-query-stable. Don't
//! rename them without coordinating with persist + downstream incident
//! tooling. If you add a new key, document it here and in
//! `docs/FEDERATION_CLIENT.md` §"Audit-log".

use serde_json::{Map, Value};

/// SHA-256 of the `attestation_envelope` JSON the registry sent to
/// persist's `federation_attestations.put()`. Hex-encoded in the
/// audit-log metadata for human readability + grep-ability.
pub const KEY_ATTESTATION_ENVELOPE_HASH: &str = "attestation_envelope_hash";

/// SHA-256 of the `revocation_envelope` JSON sent to
/// `federation_revocations.put()`.
pub const KEY_REVOCATION_ENVELOPE_HASH: &str = "revocation_envelope_hash";

/// SHA-256 of the `registration_envelope` JSON for self-published
/// `federation_keys` rows (registry's own steward bootstrap row only —
/// primitives self-publish via their own CI).
pub const KEY_KEY_REGISTRATION_ENVELOPE_HASH: &str = "key_registration_envelope_hash";

/// Persist's row scrub-timestamp (when persist witnessed the write).
/// Lets forensic queries bound the post-hoc journal scan window.
pub const KEY_PERSIST_WITNESSED_AT: &str = "persist_witnessed_at";

/// The trust policy the registry composed for this operation
/// (`direct_trust_steward` is Policy A in v1.4; v1.5 may add others).
pub const KEY_FEDERATION_POLICY: &str = "federation_policy";

/// Merge federation metadata into an existing audit-log metadata blob.
///
/// `base_metadata` is the call site's existing JSON (e.g.,
/// `{"ed25519_fingerprint": "...", "ml_dsa_65_fingerprint": "..."}`).
/// `envelope_hash` is `Some(hash)` when the dual-write actually
/// happened (R_DUAL_WRITE on, persist reachable, write accepted) and
/// `None` when the federation path was skipped (flag off, persist
/// degraded with `PERSIST_REQUIRED=false`, etc.). Returns the merged
/// JSON suitable for passing as the `metadata` argument to
/// `db::create_audit_entry`.
///
/// **Why an option, not always-required**: we want every audit-log
/// row to carry a consistent shape. When federation is off, the slot
/// is omitted (not set to null) so audit-log queries can branch on
/// presence without ambiguity. The same handler code path serves both
/// pre-federation and federation-enabled deployments.
pub fn merge_federation_metadata(
    mut base_metadata: Map<String, Value>,
    attestation_envelope_hash: Option<&[u8; 32]>,
    persist_witnessed_at: Option<i64>,
    federation_policy: Option<&str>,
) -> Map<String, Value> {
    if let Some(hash) = attestation_envelope_hash {
        base_metadata.insert(
            KEY_ATTESTATION_ENVELOPE_HASH.to_string(),
            Value::String(hex::encode(hash)),
        );
    }
    if let Some(ts) = persist_witnessed_at {
        base_metadata.insert(
            KEY_PERSIST_WITNESSED_AT.to_string(),
            Value::Number(ts.into()),
        );
    }
    if let Some(policy) = federation_policy {
        base_metadata.insert(
            KEY_FEDERATION_POLICY.to_string(),
            Value::String(policy.to_string()),
        );
    }
    base_metadata
}

/// Same as `merge_federation_metadata` but for revocation operations
/// (uses `revocation_envelope_hash` instead of `attestation_envelope_hash`).
pub fn merge_federation_revocation_metadata(
    mut base_metadata: Map<String, Value>,
    revocation_envelope_hash: Option<&[u8; 32]>,
    persist_witnessed_at: Option<i64>,
) -> Map<String, Value> {
    if let Some(hash) = revocation_envelope_hash {
        base_metadata.insert(
            KEY_REVOCATION_ENVELOPE_HASH.to_string(),
            Value::String(hex::encode(hash)),
        );
    }
    if let Some(ts) = persist_witnessed_at {
        base_metadata.insert(
            KEY_PERSIST_WITNESSED_AT.to_string(),
            Value::Number(ts.into()),
        );
    }
    base_metadata
}

/// Trust-policy string constants (audit-log values for `KEY_FEDERATION_POLICY`).
pub mod policy {
    /// Policy A: registry steward's own attestation is sufficient.
    /// Default for v1.4.
    pub const DIRECT_TRUST_STEWARD: &str = "direct_trust_steward";
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn key_constants_are_forensic_query_stable() {
        // Wire contract — see module docstring. If you have to change
        // any of these, update docs/FEDERATION_CLIENT.md and coordinate
        // with persist + downstream incident tooling.
        assert_eq!(KEY_ATTESTATION_ENVELOPE_HASH, "attestation_envelope_hash");
        assert_eq!(KEY_REVOCATION_ENVELOPE_HASH, "revocation_envelope_hash");
        assert_eq!(
            KEY_KEY_REGISTRATION_ENVELOPE_HASH,
            "key_registration_envelope_hash"
        );
        assert_eq!(KEY_PERSIST_WITNESSED_AT, "persist_witnessed_at");
        assert_eq!(KEY_FEDERATION_POLICY, "federation_policy");
        assert_eq!(policy::DIRECT_TRUST_STEWARD, "direct_trust_steward");
    }

    #[test]
    fn merge_federation_metadata_omits_slot_when_off() {
        // Federation off path — base metadata returned unchanged.
        let base = json!({"ed25519_fingerprint": "abc"});
        let merged = merge_federation_metadata(
            base.as_object().unwrap().clone(),
            None,
            None,
            None,
        );
        assert!(merged.get(KEY_ATTESTATION_ENVELOPE_HASH).is_none());
        assert!(merged.get(KEY_PERSIST_WITNESSED_AT).is_none());
        assert!(merged.get(KEY_FEDERATION_POLICY).is_none());
        assert_eq!(merged["ed25519_fingerprint"], "abc");
    }

    #[test]
    fn merge_federation_metadata_populates_slot_when_on() {
        let base = json!({"ed25519_fingerprint": "abc"});
        let hash = [0xAB; 32];
        let merged = merge_federation_metadata(
            base.as_object().unwrap().clone(),
            Some(&hash),
            Some(1735000000),
            Some(policy::DIRECT_TRUST_STEWARD),
        );
        assert_eq!(
            merged[KEY_ATTESTATION_ENVELOPE_HASH],
            Value::String(hex::encode(hash))
        );
        assert_eq!(merged[KEY_PERSIST_WITNESSED_AT], json!(1735000000));
        assert_eq!(
            merged[KEY_FEDERATION_POLICY],
            Value::String("direct_trust_steward".to_string())
        );
        // base metadata preserved
        assert_eq!(merged["ed25519_fingerprint"], "abc");
    }

    #[test]
    fn revocation_helper_uses_revocation_key() {
        let base = Map::new();
        let hash = [0xCD; 32];
        let merged = merge_federation_revocation_metadata(base, Some(&hash), None);
        assert_eq!(
            merged[KEY_REVOCATION_ENVELOPE_HASH],
            Value::String(hex::encode(hash))
        );
        // attestation key NOT present on revocation paths
        assert!(merged.get(KEY_ATTESTATION_ENVELOPE_HASH).is_none());
    }
}
