//! Persist-backed [`FederationDirectory`] implementation.
//!
//! As of v1.2.0 (#33 Phase 2), wraps an [`Arc<ciris_persist::engine::Engine>`]
//! and delegates the 8 CRUD methods to [`Engine::federation_directory`].
//! The previous stub returned `DirectoryError::NotYetImplemented` on every
//! call; now each call routes through Persist's real backend (postgres or
//! sqlite per the DSN passed at engine construction).
//!
//! Engine construction is the caller's responsibility — typically performed
//! at boot in `main.rs` once the federation `LocalSigner` is loaded.
//! `build_client` in [`super`] selects this client when
//! `FEDERATION_DUAL_WRITE_ENABLED=true` AND an engine has been wired.
//!
//! Type compatibility: as of Phase 2, [`crate::federation::types`]
//! re-exports `ciris_persist::federation::types` directly. The vendored
//! wire-format-parity contract that previously lived in this module's
//! comments is now structurally enforced — there is no parallel type
//! definition that could drift.

use std::sync::Arc;

use async_trait::async_trait;
use ciris_persist::engine::Engine;

use super::{
    Attestation, DirectoryError, FederationDirectory, KeyRecord, Result, Revocation,
    SignedAttestation, SignedKeyRecord, SignedRevocation,
};

/// Persist-backed federation directory client.
///
/// Holds an [`Arc<Engine>`] so the underlying backend (postgres or sqlite)
/// can be shared across all federation-directory consumers in the process.
/// Cheap to clone; each method clones the engine's internal `Arc` once
/// per call when materializing the trait object.
pub struct PersistFederationClient {
    engine: Arc<Engine>,
}

impl PersistFederationClient {
    /// Construct a new client wrapping the given engine.
    ///
    /// Caller is responsible for engine lifecycle. Typical use:
    /// build engine once at boot, share it via `Arc::clone` to any
    /// component that needs federation-directory access.
    pub fn new(engine: Arc<Engine>) -> Self {
        Self { engine }
    }
}

/// Convert upstream's [`ciris_persist::federation::Error`] into our
/// [`DirectoryError`].
///
/// The mapping favors specificity where the upstream variant has a
/// clear consumer-facing analog (`InvalidArgument` → `InvalidArgument`).
/// Variants without a direct analog (rate-limiting, backend errors,
/// signature-verification failures) collapse to `Rejected` with the
/// upstream error's `Display` string preserved for forensic queries.
impl From<ciris_persist::federation::Error> for DirectoryError {
    fn from(e: ciris_persist::federation::Error) -> Self {
        use ciris_persist::federation::Error as E;
        match e {
            E::InvalidArgument(s) => DirectoryError::InvalidArgument(s),
            other => DirectoryError::Rejected(other.to_string()),
        }
    }
}

#[async_trait]
impl FederationDirectory for PersistFederationClient {
    async fn put_public_key(&self, record: SignedKeyRecord) -> Result<()> {
        self.engine
            .federation_directory()
            .put_public_key(record)
            .await
            .map_err(Into::into)
    }

    async fn lookup_public_key(&self, key_id: &str) -> Result<Option<KeyRecord>> {
        self.engine
            .federation_directory()
            .lookup_public_key(key_id)
            .await
            .map_err(Into::into)
    }

    async fn lookup_keys_for_identity(&self, identity_ref: &str) -> Result<Vec<KeyRecord>> {
        self.engine
            .federation_directory()
            .lookup_keys_for_identity(identity_ref)
            .await
            .map_err(Into::into)
    }

    async fn put_attestation(&self, attestation: SignedAttestation) -> Result<()> {
        self.engine
            .federation_directory()
            .put_attestation(attestation)
            .await
            .map_err(Into::into)
    }

    async fn list_attestations_for(&self, attested_key_id: &str) -> Result<Vec<Attestation>> {
        self.engine
            .federation_directory()
            .list_attestations_for(attested_key_id)
            .await
            .map_err(Into::into)
    }

    async fn list_attestations_by(&self, attesting_key_id: &str) -> Result<Vec<Attestation>> {
        self.engine
            .federation_directory()
            .list_attestations_by(attesting_key_id)
            .await
            .map_err(Into::into)
    }

    async fn put_revocation(&self, revocation: SignedRevocation) -> Result<()> {
        self.engine
            .federation_directory()
            .put_revocation(revocation)
            .await
            .map_err(Into::into)
    }

    async fn revocations_for(&self, revoked_key_id: &str) -> Result<Vec<Revocation>> {
        self.engine
            .federation_directory()
            .revocations_for(revoked_key_id)
            .await
            .map_err(Into::into)
    }
}
