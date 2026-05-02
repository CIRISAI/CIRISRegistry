//! Federation directory wire-format types.
//!
//! Vendored from CIRISPersist's `docs/FEDERATION_DIRECTORY.md` schema
//! sketch (§"Schema sketch — federation_keys / federation_attestations
//! / federation_revocations"). Field ordering matches persist's
//! published doc; serde representation will be validated against a
//! representative `federation_keys` row JSON when persist v0.2.0-pre1
//! ships (per the open follow-up in `docs/FEDERATION_CLIENT.md`).
//!
//! **Wire format parity contract (mirroring `build_manifest.rs`)**:
//! any change to these types must match upstream's row shape exactly.
//! Divergence triggers `federation_dual_write_divergence_total`
//! telemetry on read-through.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Identity classification per persist's `identity_type` column.
///
/// Strings used on the wire (matches persist's text-typed column for
/// forward-compat — new identity types can be added by either side
/// without breaking serde).
pub mod identity_type {
    pub const AGENT: &str = "agent";
    pub const PRIMITIVE: &str = "primitive";
    pub const STEWARD: &str = "steward";
    pub const PARTNER: &str = "partner";
}

/// Algorithm names matching persist's `algorithm` column.
pub mod algorithm {
    pub const ED25519: &str = "ed25519";
    pub const ML_DSA_65: &str = "ml-dsa-65";
    pub const HYBRID: &str = "hybrid";
}

/// Attestation type strings (intentionally an open string set, not an
/// enum — consumers may invent types as trust models evolve).
pub mod attestation_type {
    pub const VOUCHES_FOR: &str = "vouches_for";
    pub const WITNESSES: &str = "witnesses";
    pub const REFERRED: &str = "referred";
    pub const DELEGATED_TO: &str = "delegated_to";
}

/// `federation_keys` row as returned by persist's read methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyRecord {
    pub key_id: String,
    pub pubkey_base64: String,
    pub algorithm: String,
    pub identity_type: String,
    pub identity_ref: String,
    #[serde(with = "time::serde::rfc3339")]
    pub valid_from: OffsetDateTime,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub valid_until: Option<OffsetDateTime>,
    pub registration_envelope: serde_json::Value,
    /// sha256 of `registration_envelope`. Hex-encoded on the wire.
    pub original_content_hash: String,
    /// Ed25519 over `original_content_hash`. Base64-encoded on the wire.
    pub scrub_signature: String,
    pub scrub_key_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub scrub_timestamp: OffsetDateTime,
}

impl Default for KeyRecord {
    fn default() -> Self {
        Self {
            key_id: String::new(),
            pubkey_base64: String::new(),
            algorithm: String::new(),
            identity_type: String::new(),
            identity_ref: String::new(),
            valid_from: OffsetDateTime::UNIX_EPOCH,
            valid_until: None,
            registration_envelope: serde_json::Value::Null,
            original_content_hash: String::new(),
            scrub_signature: String::new(),
            scrub_key_id: String::new(),
            scrub_timestamp: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl Default for SignedKeyRecord {
    fn default() -> Self {
        Self { record: KeyRecord::default() }
    }
}

impl Default for Attestation {
    fn default() -> Self {
        Self {
            attestation_id: String::new(),
            attesting_key_id: String::new(),
            attested_key_id: String::new(),
            attestation_type: String::new(),
            weight: None,
            asserted_at: OffsetDateTime::UNIX_EPOCH,
            expires_at: None,
            attestation_envelope: serde_json::Value::Null,
            original_content_hash: String::new(),
            scrub_signature: String::new(),
            scrub_key_id: String::new(),
            scrub_timestamp: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl Default for SignedAttestation {
    fn default() -> Self {
        Self { attestation: Attestation::default() }
    }
}

impl Default for Revocation {
    fn default() -> Self {
        Self {
            revocation_id: String::new(),
            revoked_key_id: String::new(),
            revoking_key_id: String::new(),
            reason: String::new(),
            revoked_at: OffsetDateTime::UNIX_EPOCH,
            effective_at: OffsetDateTime::UNIX_EPOCH,
            revocation_envelope: serde_json::Value::Null,
            original_content_hash: String::new(),
            scrub_signature: String::new(),
            scrub_key_id: String::new(),
            scrub_timestamp: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl Default for SignedRevocation {
    fn default() -> Self {
        Self { revocation: Revocation::default() }
    }
}

impl KeyRecord {
    /// SHA-256 of the canonical row bytes — used for cache-divergence
    /// detection (`persist_row_hash` column on the cache tables).
    /// Computed from a deterministic JSON encoding of the row to avoid
    /// false positives from JSON whitespace differences.
    pub fn canonical_hash(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        // Canonical encoding: serialize to bytes via serde_json (sorted
        // keys via serde derive ordering matches persist's row layout).
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().into()
    }
}

/// Wraps a `KeyRecord` payload that the caller has signed but persist
/// has not yet stored. Persist verifies the scrub-signature on receipt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignedKeyRecord {
    pub record: KeyRecord,
}

/// `federation_attestations` row.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attestation {
    pub attestation_id: String, // UUID as string
    pub attesting_key_id: String,
    pub attested_key_id: String,
    pub attestation_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(with = "time::serde::rfc3339")]
    pub asserted_at: OffsetDateTime,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    pub attestation_envelope: serde_json::Value,
    pub original_content_hash: String,
    pub scrub_signature: String,
    pub scrub_key_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub scrub_timestamp: OffsetDateTime,
}

impl Attestation {
    /// Hash used for the audit-log `attestation_envelope_hash` field
    /// (R_AUDIT join key per FEDERATION_CLIENT.md §"Audit-log").
    pub fn envelope_hash(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let bytes = serde_json::to_vec(&self.attestation_envelope).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignedAttestation {
    pub attestation: Attestation,
}

/// `federation_revocations` row.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Revocation {
    pub revocation_id: String, // UUID as string
    pub revoked_key_id: String,
    pub revoking_key_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(with = "time::serde::rfc3339")]
    pub revoked_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub effective_at: OffsetDateTime,
    pub revocation_envelope: serde_json::Value,
    pub original_content_hash: String,
    pub scrub_signature: String,
    pub scrub_key_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub scrub_timestamp: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignedRevocation {
    pub revocation: Revocation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_record_canonical_hash_is_deterministic() {
        let record = KeyRecord {
            key_id: "test-key".to_string(),
            pubkey_base64: "AAAA".to_string(),
            algorithm: algorithm::ED25519.to_string(),
            identity_type: identity_type::PRIMITIVE.to_string(),
            identity_ref: "ciris-registry".to_string(),
            valid_from: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            valid_until: None,
            registration_envelope: serde_json::json!({"some": "data"}),
            original_content_hash: "abc123".to_string(),
            scrub_signature: "sig".to_string(),
            scrub_key_id: "test-key".to_string(),
            scrub_timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
        };
        let h1 = record.canonical_hash();
        let h2 = record.canonical_hash();
        assert_eq!(h1, h2, "canonical_hash must be deterministic");
    }

    #[test]
    fn key_record_canonical_hash_changes_on_field_change() {
        let mut a = KeyRecord {
            key_id: "test-key".to_string(),
            pubkey_base64: "AAAA".to_string(),
            algorithm: algorithm::ED25519.to_string(),
            identity_type: identity_type::PRIMITIVE.to_string(),
            identity_ref: "ciris-registry".to_string(),
            valid_from: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            valid_until: None,
            registration_envelope: serde_json::json!({}),
            original_content_hash: "abc".to_string(),
            scrub_signature: "sig".to_string(),
            scrub_key_id: "test-key".to_string(),
            scrub_timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
        };
        let h1 = a.canonical_hash();
        a.pubkey_base64 = "BBBB".to_string();
        let h2 = a.canonical_hash();
        assert_ne!(h1, h2, "canonical_hash must change when fields change");
    }

    #[test]
    fn identity_type_constants_match_persist_doc() {
        assert_eq!(identity_type::AGENT, "agent");
        assert_eq!(identity_type::PRIMITIVE, "primitive");
        assert_eq!(identity_type::STEWARD, "steward");
        assert_eq!(identity_type::PARTNER, "partner");
    }

    #[test]
    fn attestation_type_constants_match_persist_doc() {
        assert_eq!(attestation_type::VOUCHES_FOR, "vouches_for");
        assert_eq!(attestation_type::WITNESSES, "witnesses");
        assert_eq!(attestation_type::REFERRED, "referred");
        assert_eq!(attestation_type::DELEGATED_TO, "delegated_to");
    }
}
