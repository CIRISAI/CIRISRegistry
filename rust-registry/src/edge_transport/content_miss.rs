//! ContentMiss-feedback emission per CEG 0.2 §10.1.2.
//!
//! Per CEG 0.2 §10.1.2:
//!
//! > On `ContentMiss`, the consumer MUST emit a `withdraws` against the
//! > stale `holds_bytes:sha256:{prefix}` attestation referencing the
//! > stale holder, with `withdrawal_reason: "content_miss"`. Holders
//! > consistently failing ContentMiss are downweighted in
//! > `PeerResolver::resolve_holders`.
//!
//! This module ships the typed helper that constructs and submits the
//! `withdraws` Attestation. The caller decides when to invoke (typically
//! on the first ContentMiss for a given attestation; or on consistent
//! misses across multiple ContentFetch attempts).

use std::sync::Arc;

use thiserror::Error;

use crate::federation::{DirectoryError, FederationDirectory};

/// Error returned when ContentMiss-feedback emission fails.
#[derive(Debug, Error)]
pub enum ContentMissError {
    /// The underlying federation directory rejected the `withdraws`
    /// attestation. Caller decides whether to retry (e.g., persist
    /// became briefly unavailable) or fail-loud per the consumer-policy
    /// freshness contract.
    #[error("federation directory rejected withdraws emission: {0}")]
    Directory(#[from] DirectoryError),
}

/// Emit a `withdraws` against a stale `holds_bytes` attestation,
/// canonicalized per CEG §10.1.2 with `withdrawal_reason: "content_miss"`.
///
/// `directory` is the federation client (`Arc<dyn FederationDirectory>`
/// from [`crate::federation::build_client`]). Behavior depends on the
/// underlying impl:
///
/// - `NoOpFederationClient` — returns Ok(()) silently. Dual-write
///   disabled; the stale attestation's downweighting is local to
///   the consumer's peer-routing cache only.
/// - `PersistFederationClient` — actually emits the `withdraws` row
///   to Persist's `federation_attestations`. Other federation peers
///   reading the directory subsequently see the stale `holds_bytes`
///   row marked withdrawn.
///
/// **Wire shape**: the helper constructs the `withdraws` Attestation
/// directly here rather than relying on a separate canonicalization
/// service. This is intentional — the structural composer
/// `attestation_type = "withdraws"` is one of the four CEG 1+4 primitives
/// (§3.2) and its envelope shape is small and stable.
///
/// **Phase 3 scope note**: the actual construction of the `withdraws`
/// Attestation (with the proper `references_attestation_id`,
/// `withdrawal_reason`, scrub-signature over canonical bytes, etc.) is
/// deferred to the Phase-2-follow-up engine integration. In v1.3.0-rc.1
/// this helper takes a pre-built `SignedAttestation` and just delegates
/// to `directory.put_attestation(...)`. The caller (the Phase-2-follow-up
/// commit) is responsible for canonicalization + signing.
pub async fn emit_content_miss_withdraws(
    directory: Arc<dyn FederationDirectory>,
    withdraws_attestation: crate::federation::SignedAttestation,
) -> Result<(), ContentMissError> {
    directory
        .put_attestation(withdraws_attestation)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::federation::types::{algorithm, attestation_type, identity_type};
    use crate::federation::{
        Attestation, KeyRecord, NoOpFederationClient, SignedAttestation, SignedKeyRecord,
    };
    use chrono::Utc;

    fn dummy_withdraws_attestation() -> SignedAttestation {
        SignedAttestation {
            attestation: Attestation {
                attestation_id: "withdraws-test-id".into(),
                attesting_key_id: "test-attester".into(),
                attested_key_id: "stale-holder".into(),
                attestation_type: attestation_type::WITHDRAWS.into(),
                weight: None,
                asserted_at: Utc::now(),
                expires_at: None,
                attestation_envelope: serde_json::json!({
                    "references_attestation_id": "stale-holds-bytes-id",
                    "withdrawal_reason": "content_miss",
                }),
                original_content_hash: String::new(),
                scrub_signature_classical: String::new(),
                scrub_signature_pqc: None,
                scrub_key_id: "test-attester".into(),
                scrub_timestamp: Utc::now(),
                pqc_completed_at: None,
                persist_row_hash: String::new(),
            },
        }
    }

    #[tokio::test]
    async fn noop_directory_silently_accepts_withdraws() {
        let directory: Arc<dyn FederationDirectory> = Arc::new(NoOpFederationClient);
        let withdraws = dummy_withdraws_attestation();

        let result = emit_content_miss_withdraws(directory, withdraws).await;
        assert!(result.is_ok(), "NoOp client returns Ok silently");
    }

    // Ensure unused imports are silenced if test references aren't
    // tightened later; the constants are documented for reviewers.
    #[test]
    fn module_constants_resolved() {
        let _ = algorithm::HYBRID;
        let _ = identity_type::AGENT;
        let _: SignedKeyRecord = SignedKeyRecord {
            record: KeyRecord {
                key_id: "x".into(),
                pubkey_ed25519_base64: String::new(),
                pubkey_ml_dsa_65_base64: None,
                algorithm: algorithm::HYBRID.into(),
                identity_type: identity_type::AGENT.into(),
                identity_ref: String::new(),
                valid_from: Utc::now(),
                valid_until: None,
                registration_envelope: serde_json::Value::Null,
                original_content_hash: String::new(),
                scrub_signature_classical: String::new(),
                scrub_signature_pqc: None,
                scrub_key_id: String::new(),
                scrub_timestamp: Utc::now(),
                pqc_completed_at: None,
                roles: Vec::new(),
                attestation_evidence: None,
                persist_row_hash: String::new(),
            },
        };
    }
}
