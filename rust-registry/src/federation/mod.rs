//! Federation directory client (PoB §3.1, v1.4 R_SCAFFOLD).
//!
//! Registry-side client for CIRISPersist's `FederationDirectory`.
//! Mirrors the trait surface and serde models defined in
//! [`docs/FEDERATION_DIRECTORY.md`](../../docs/FEDERATION_DIRECTORY.md)
//! upstream and consumed per
//! [`docs/FEDERATION_CLIENT.md`](../../docs/FEDERATION_CLIENT.md).
//!
//! **Status: scaffolding.** When `FEDERATION_DUAL_WRITE_ENABLED=false`
//! (default), all writes/reads no-op via `NoOpFederationClient`. When
//! the flag is on, `PersistFederationClient` is selected at boot —
//! its method bodies currently `unimplemented!()` and will be filled
//! in once persist v0.2.0-pre1 publishes the wire format and trait
//! crate location. The misconfiguration guard in
//! `config.rs::FederationSettings` ensures the flag can't be flipped
//! on without a configured endpoint.
//!
//! Module layout:
//! - `mod.rs` — trait, error, factory, no-op impl
//! - `types.rs` — serde models matching persist's row shapes
//! - `persist_client.rs` — real client stub (HTTP/gRPC TBD)
//!
//! The shapes here are derived from persist's published doc and may
//! adjust slightly when v0.2.0-pre1 ships a representative
//! `federation_keys` row JSON. Track in `docs/FEDERATION_CLIENT.md`
//! "Last updated" note.

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

    /// Stub `unimplemented!()` returns from `PersistFederationClient` —
    /// should be replaced with a real impl once persist v0.2.0-pre1 ships.
    #[error("federation client not yet implemented (waiting on persist v0.2.0-pre1)")]
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
/// Returns `NoOpFederationClient` when `dual_write_enabled=false`.
/// Returns `PersistFederationClient` when on. The misconfiguration
/// guard at boot (`config.rs`) ensures we never reach this with
/// `dual_write_enabled=true && persist_endpoint.is_empty()`.
pub fn build_client(
    settings: &crate::config::FederationSettings,
) -> std::sync::Arc<dyn FederationDirectory> {
    if settings.dual_write_enabled {
        std::sync::Arc::new(persist_client::PersistFederationClient::new(
            settings.persist_endpoint.clone(),
        ))
    } else {
        std::sync::Arc::new(NoOpFederationClient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn noop_client_writes_succeed_silently() {
        let client = NoOpFederationClient;
        let record = SignedKeyRecord::default();
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
        let client = build_client(&settings);
        // Smoke test: client is constructed without panic.
        // Behavior verified by the noop_client_* tests above.
        let _ = client;
    }
}
