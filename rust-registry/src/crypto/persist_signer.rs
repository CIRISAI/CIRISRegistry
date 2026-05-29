//! Local adapter wrapping `ciris_crypto::MlDsa65Signer` for
//! `ciris_keyring::PqcSigner`.
//!
//! CIRISVerify#39 closure shipped `impl PqcSigner for MlDsa65Signer`
//! in `ciris-keyring` v4.1.0. The keyring tag v4.1.0 is not yet
//! reachable from our dep graph — `ciris-persist` v3.3.1, `ciris-edge`
//! v0.18.0, and `ciris-verify-core` v4.0.0 all pin
//! `ciris-keyring = { tag = "v4.0.0" }` transitively, and Cargo cannot
//! unify across distinct git-tag pins from the same source (we tried
//! `[patch."https://github.com/CIRISAI/CIRISVerify"]` which Cargo
//! rejects with "patch must point to a different source").
//!
//! Until the upstream cohabit set (edge / persist / verify-core)
//! releases versions that pin keyring v4.1.0, this module provides the
//! same net effect locally: a wrapper struct `PersistPqcAdapter` that
//! we own (orphan-rules-friendly: local struct + foreign trait) and
//! implements `ciris_keyring::PqcSigner` for whichever keyring version
//! the dep graph resolved to (v4.0.0 in v1.3.0 of Registry).
//!
//! The implementation mirrors CIRISVerify#39's design in
//! `keyring/src/pqc.rs` exactly — same algorithm enum, same
//! `SoftwareOnly` hardware-type, same async-over-sync delegation
//! pattern. When upstream v4.1.0 is reachable, this file deletes
//! cleanly: `Arc::new(mldsa_signer) as Arc<dyn PqcSigner>` will work
//! against the upstream impl directly.

use std::sync::Arc;

use async_trait::async_trait;
use ciris_crypto::{MlDsa65Signer, PqcSigner as CryptoPqcSigner};
use ciris_keyring::{
    HardwareType, KeyringError, PlatformAttestation, PqcAlgorithm, PqcSigner, SoftwareAttestation,
    StorageDescriptor,
};

/// Adapter exposing a `ciris_crypto::MlDsa65Signer` as a
/// `ciris_keyring::PqcSigner`.
///
/// Constructed via [`Self::new`]; pass through `Arc::new(adapter) as
/// Arc<dyn ciris_keyring::PqcSigner>` to satisfy
/// `ciris_persist::signing::LocalSigner::from_parts(.., pqc_signer:
/// Option<Arc<dyn PqcSigner>>, ..)`.
pub struct PersistPqcAdapter {
    inner: MlDsa65Signer,
}

impl PersistPqcAdapter {
    pub fn new(signer: MlDsa65Signer) -> Self {
        Self { inner: signer }
    }
}

#[async_trait]
impl PqcSigner for PersistPqcAdapter {
    fn algorithm(&self) -> PqcAlgorithm {
        PqcAlgorithm::MlDsa65
    }

    fn hardware_type(&self) -> HardwareType {
        // All current PQC implementations are software-only (no
        // production HSM ships ML-DSA primitives in 2026; CIRISVerify#39
        // notes the same).
        HardwareType::SoftwareOnly
    }

    async fn public_key(&self) -> Result<Vec<u8>, KeyringError> {
        CryptoPqcSigner::public_key(&self.inner).map_err(|e| KeyringError::HardwareError {
            reason: format!("ML-DSA-65 public_key (local adapter): {e}"),
        })
    }

    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, KeyringError> {
        CryptoPqcSigner::sign(&self.inner, data).map_err(|e| KeyringError::HardwareError {
            reason: format!("ML-DSA-65 sign (local adapter): {e}"),
        })
    }

    async fn attestation(&self) -> Result<PlatformAttestation, KeyringError> {
        // Software-only attestation — the raw signer has no
        // operator-supplied seed_path, just an in-memory keypair.
        // Mirrors the CIRISVerify#39 v4.1.0 direct impl shape.
        Ok(PlatformAttestation::Software(SoftwareAttestation {
            key_derivation: "in-memory seed bytes (constructed at boot)".to_string(),
            storage: "process memory (no disk path)".to_string(),
            security_warning: "SOFTWARE_ONLY: ciris-crypto MlDsa65Signer via Registry's local \
                               PersistPqcAdapter (no path-bearing attestation). Disk-backed \
                               attestation requires the MlDsa65SoftwareSigner wrapper instead."
                .to_string(),
        }))
    }

    fn current_alias(&self) -> &str {
        // Type-identity alias mirroring upstream v4.1.0's choice for the
        // direct impl (no operator-assigned alias on the raw signer).
        "ciris-registry/PersistPqcAdapter"
    }

    fn storage_descriptor(&self) -> StorageDescriptor {
        // The raw signer doesn't track a path — keys live in memory
        // (constructed from seed bytes at boot).
        StorageDescriptor::InMemory
    }
}

/// Convenience constructor producing an `Arc<dyn PqcSigner>` ready to
/// pass into `LocalSigner::from_parts`.
pub fn arc_persist_pqc(signer: MlDsa65Signer) -> Arc<dyn PqcSigner> {
    Arc::new(PersistPqcAdapter::new(signer))
}
