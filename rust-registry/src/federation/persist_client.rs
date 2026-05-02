//! Persist-backed `FederationDirectory` implementation — STUB.
//!
//! All methods currently return `DirectoryError::NotYetImplemented`.
//! The real implementation lands when CIRISPersist v0.2.0-pre1 publishes:
//!
//! 1. The `FederationDirectory` trait crate location (whether shipped as
//!    a published `ciris-federation-directory` crate, or as a vendored
//!    shape — see `docs/FEDERATION_CLIENT.md` "Last updated" note).
//! 2. The wire transport (HTTP / gRPC / direct DB connection).
//! 3. A representative `federation_keys` row JSON for serde validation
//!    against `crate::federation::types`.
//!
//! Until then, this module exists so `crate::federation::build_client`
//! can return a typed-but-non-functional client when
//! `FEDERATION_DUAL_WRITE_ENABLED=true` and an endpoint is configured.
//! The misconfiguration guard in `config.rs::FederationSettings` prevents
//! anyone from accidentally enabling this in production today.
//!
//! Once persist v0.2.0-pre1 is published:
//! 1. Replace the `unimplemented!()` bodies with actual transport calls.
//! 2. Add cache-aside read-through logic in the registry's lookup paths.
//! 3. Increment telemetry counters per `docs/FEDERATION_CLIENT.md` §"Telemetry".

use async_trait::async_trait;

use super::{
    Attestation, DirectoryError, FederationDirectory, KeyRecord, Result, Revocation,
    SignedAttestation, SignedKeyRecord, SignedRevocation,
};

pub struct PersistFederationClient {
    /// Endpoint URL or socket path; opaque until persist v0.2.0-pre1
    /// specifies the transport.
    #[allow(dead_code)]
    endpoint: String,
}

impl PersistFederationClient {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[async_trait]
impl FederationDirectory for PersistFederationClient {
    async fn put_public_key(&self, _record: SignedKeyRecord) -> Result<()> {
        Err(DirectoryError::NotYetImplemented)
    }

    async fn lookup_public_key(&self, _key_id: &str) -> Result<Option<KeyRecord>> {
        Err(DirectoryError::NotYetImplemented)
    }

    async fn lookup_keys_for_identity(&self, _identity_ref: &str) -> Result<Vec<KeyRecord>> {
        Err(DirectoryError::NotYetImplemented)
    }

    async fn put_attestation(&self, _attestation: SignedAttestation) -> Result<()> {
        Err(DirectoryError::NotYetImplemented)
    }

    async fn list_attestations_for(&self, _attested_key_id: &str) -> Result<Vec<Attestation>> {
        Err(DirectoryError::NotYetImplemented)
    }

    async fn list_attestations_by(&self, _attesting_key_id: &str) -> Result<Vec<Attestation>> {
        Err(DirectoryError::NotYetImplemented)
    }

    async fn put_revocation(&self, _revocation: SignedRevocation) -> Result<()> {
        Err(DirectoryError::NotYetImplemented)
    }

    async fn revocations_for(&self, _revoked_key_id: &str) -> Result<Vec<Revocation>> {
        Err(DirectoryError::NotYetImplemented)
    }
}
