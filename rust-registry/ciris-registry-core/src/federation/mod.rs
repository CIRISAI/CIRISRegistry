//! Federation directory client (PoB §3.1).
//!
//! Registry-side client for CIRISPersist's `FederationDirectory`.
//!
//! As of v1.2.0 (umbrella #33 Phase 2), `PersistFederationClient` wraps
//! an `Arc<ciris_persist::engine::Engine>` directly and delegates to
//! `engine.federation_directory()`. The previous stub state (returning
//! `DirectoryError::NotYetImplemented` on every call) is closed; the
//! `NotYetImplemented` variant remains in the error enum for forward-
//! compat with future Persist trait extensions Registry may not yet wire.
//!
//! When `FEDERATION_DUAL_WRITE_ENABLED=false` (default), all writes/reads
//! no-op via `NoOpFederationClient`. When the flag is on AND the boot path
//! has constructed a Persist engine, `PersistFederationClient` is selected.
//! The misconfiguration guard in `config.rs::FederationSettings` ensures
//! the flag can't be flipped on without a configured endpoint.
//!
//! Module layout:
//! - `mod.rs` — trait, error, factory, no-op impl
//! - `types.rs` — re-exports from `ciris_persist::federation::types`
//!   (single source of truth as of Phase 2)
//! - `persist_client.rs` — engine-backed `FederationDirectory` impl

use async_trait::async_trait;
use thiserror::Error;

pub mod audit;
pub mod metrics;
pub mod persist_client;
pub mod types;

pub use types::{
    Attestation, KeyRecord, Revocation, SignedAttestation, SignedKeyRecord, SignedRevocation,
};

/// Errors returned by federation directory operations.
///
/// Mirrors persist's `DirectoryError` enum at the wire level. Variants
/// chosen to support clear cache-vs-persist failure-mode telemetry per
/// `docs/FEDERATION_CLIENT.md` §"Failure modes".
#[derive(Debug, Error)]
pub enum DirectoryError {
    /// Persist endpoint unreachable / network failure / TLS handshake
    /// failure / etc. Caller decides whether to fall back to cache
    /// (`PERSIST_REQUIRED=false`) or fail-closed (`PERSIST_REQUIRED=true`).
    #[error("persist unreachable: {0}")]
    Unreachable(String),

    /// Persist returned an explicit failure response. Includes
    /// payload-validation rejections (bad scrub-signature, FK violation,
    /// etc.) and quota violations.
    #[error("persist rejected request: {0}")]
    Rejected(String),

    /// Wire-format / serde error. Indicates a schema drift between the
    /// registry's vendored types and persist's actual response shape —
    /// triggers a divergence-counter increment and operator alarm.
    #[error("wire format error: {0}")]
    WireFormat(String),

    /// Caller passed invalid arguments (empty key_id, malformed pubkey, etc.).
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// Used by the no-op client when the flag is off but a caller
    /// expected a real backend. Should never reach production behavior
    /// — handlers branch on the flag before calling.
    #[error("federation directory disabled (FEDERATION_DUAL_WRITE_ENABLED=false)")]
    Disabled,

    /// Reserved for forward-compat with Persist trait extensions Registry
    /// may not yet wire (e.g., the PQC-attach methods on the upstream
    /// trait that Registry's [`FederationDirectory`] does not currently
    /// expose). Pre-v1.2.0 this variant signalled the stub state of
    /// `PersistFederationClient`; that state is closed as of Phase 2 of
    /// umbrella #33.
    #[error("federation client method not yet wired in Registry")]
    NotYetImplemented,
}

pub type Result<T> = std::result::Result<T, DirectoryError>;

/// The persist-as-substrate trait. CRUD over the three federation tables.
///
/// **No `is_trusted()` / `trust_score()` / `trust_path()` methods** —
/// per persist's design, trust is the consumer's policy. The registry
/// composes Policy A (direct-trust on registry-steward attestation)
/// from these primitives in `services::admin::register_trusted_primitive_key`.
///
/// All methods are `async` to match the eventual transport (HTTP or
/// gRPC; TBD on persist's side).
#[async_trait]
pub trait FederationDirectory: Send + Sync {
    // ── Public keys ────────────────────────────────────────────────
    async fn put_public_key(&self, record: SignedKeyRecord) -> Result<()>;
    async fn lookup_public_key(&self, key_id: &str) -> Result<Option<KeyRecord>>;
    async fn lookup_keys_for_identity(&self, identity_ref: &str) -> Result<Vec<KeyRecord>>;

    // ── Attestations ───────────────────────────────────────────────
    async fn put_attestation(&self, attestation: SignedAttestation) -> Result<()>;
    async fn list_attestations_for(&self, attested_key_id: &str) -> Result<Vec<Attestation>>;
    async fn list_attestations_by(&self, attesting_key_id: &str) -> Result<Vec<Attestation>>;

    // ── Revocations ────────────────────────────────────────────────
    async fn put_revocation(&self, revocation: SignedRevocation) -> Result<()>;
    async fn revocations_for(&self, revoked_key_id: &str) -> Result<Vec<Revocation>>;
}

/// No-op client used when `FEDERATION_DUAL_WRITE_ENABLED=false`.
///
/// All writes succeed silently (Ok(())). All reads return None / empty.
/// Callers that need real federation reads should branch on the flag
/// before calling — falling through to no-op reads here would return
/// "no key registered" which would surface as 403 to consumers, exactly
/// the expected pre-federation behavior since the registry's local
/// table is the actual source of truth in this mode.
pub struct NoOpFederationClient;

#[async_trait]
impl FederationDirectory for NoOpFederationClient {
    async fn put_public_key(&self, _record: SignedKeyRecord) -> Result<()> {
        Ok(())
    }
    async fn lookup_public_key(&self, _key_id: &str) -> Result<Option<KeyRecord>> {
        Ok(None)
    }
    async fn lookup_keys_for_identity(&self, _identity_ref: &str) -> Result<Vec<KeyRecord>> {
        Ok(Vec::new())
    }
    async fn put_attestation(&self, _attestation: SignedAttestation) -> Result<()> {
        Ok(())
    }
    async fn list_attestations_for(&self, _attested_key_id: &str) -> Result<Vec<Attestation>> {
        Ok(Vec::new())
    }
    async fn list_attestations_by(&self, _attesting_key_id: &str) -> Result<Vec<Attestation>> {
        Ok(Vec::new())
    }
    async fn put_revocation(&self, _revocation: SignedRevocation) -> Result<()> {
        Ok(())
    }
    async fn revocations_for(&self, _revoked_key_id: &str) -> Result<Vec<Revocation>> {
        Ok(Vec::new())
    }
}

/// Build the appropriate federation client based on settings.
///
/// - `NoOpFederationClient` when `dual_write_enabled=false`.
/// - `PersistFederationClient` when `dual_write_enabled=true` AND `engine`
///   is `Some`. The boot-time misconfiguration guard at `config.rs` ensures
///   `persist_endpoint` is non-empty when `dual_write_enabled=true`, and
///   the boot path is responsible for constructing the engine before
///   calling this. If `dual_write_enabled=true` is somehow reached with
///   `engine=None`, this falls back to `NoOpFederationClient` rather than
///   panicking — the misconfig is already caught upstream at boot.
pub fn build_client(
    engine: Option<std::sync::Arc<ciris_persist::engine::Engine>>,
    settings: &crate::config::FederationSettings,
) -> std::sync::Arc<dyn FederationDirectory> {
    if settings.dual_write_enabled {
        if let Some(e) = engine {
            std::sync::Arc::new(persist_client::PersistFederationClient::new(e))
        } else {
            std::sync::Arc::new(NoOpFederationClient)
        }
    } else {
        std::sync::Arc::new(NoOpFederationClient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use types::{algorithm, identity_type};

    /// Construct a minimal `SignedKeyRecord` for NoOp client tests.
    ///
    /// Upstream `ciris_persist::federation::types::SignedKeyRecord` does
    /// not implement `Default` (intentional — every field is load-bearing
    /// on the persist write path). For NoOp tests we don't care about
    /// field validity; we only need a value of the right type.
    fn dummy_signed_key_record() -> SignedKeyRecord {
        SignedKeyRecord {
            record: KeyRecord {
                key_id: "noop-test".into(),
                pubkey_ed25519_base64: String::new(),
                pubkey_ml_dsa_65_base64: None,
                algorithm: algorithm::HYBRID.into(),
                identity_type: identity_type::AGENT.into(),
                identity_ref: "noop-test".into(),
                valid_from: Utc::now(),
                valid_until: None,
                registration_envelope: serde_json::Value::Null,
                original_content_hash: String::new(),
                scrub_signature_classical: String::new(),
                scrub_signature_pqc: None,
                scrub_key_id: "noop-test".into(),
                scrub_timestamp: Utc::now(),
                pqc_completed_at: None,
                capability_roles: Vec::new(),
                consent_role: None,
                additional_scrubs: Vec::new(),
                attestation_evidence: None,
                persist_row_hash: String::new(),
            },
        }
    }

    #[tokio::test]
    async fn noop_client_writes_succeed_silently() {
        let client = NoOpFederationClient;
        let record = dummy_signed_key_record();
        assert!(client.put_public_key(record).await.is_ok());
    }

    #[tokio::test]
    async fn noop_client_reads_return_empty() {
        let client = NoOpFederationClient;
        let result = client.lookup_public_key("nonexistent").await.unwrap();
        assert!(result.is_none());

        let attestations = client.list_attestations_for("nonexistent").await.unwrap();
        assert!(attestations.is_empty());

        let revocations = client.revocations_for("nonexistent").await.unwrap();
        assert!(revocations.is_empty());
    }

    #[test]
    fn build_client_noop_when_flag_off() {
        let settings = crate::config::FederationSettings::default();
        let client = build_client(None, &settings);
        // Smoke test: client is constructed without panic.
        // Behavior verified by the noop_client_* tests above.
        let _ = client;
    }

    #[test]
    fn build_client_noop_when_dual_write_on_but_engine_missing() {
        // Defense-in-depth: if dual_write is enabled but no engine was
        // constructed at boot (a config-vs-impl contract violation that
        // the boot guard should catch), fall back to no-op rather than
        // panic.
        let mut settings = crate::config::FederationSettings::default();
        settings.dual_write_enabled = true;
        let client = build_client(None, &settings);
        let _ = client;
    }
}
