//! Hybrid cryptography module (Ed25519 + ML-DSA-65, FIPS 204)
//!
//! Built on `ciris-crypto` v1.14.0 — the same primitives CIRISLens, CIRISAgent,
//! and CIRISPersist consume. Closes AV-27 (vault-mode dummy-key bug) and
//! AV-25 (home-rolled crypto extraction risk) by deletion of the legacy
//! storage_mode plumbing.

use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use ciris_crypto::{
    ClassicalSigner, ClassicalVerifier, Ed25519Signer, Ed25519Verifier,
    MlDsa65Signer, MlDsa65Verifier, PqcSigner, PqcVerifier,
};
use sha2::{Digest, Sha256};

use crate::config::CryptoSettings;
use crate::error::{RegistryError, Result};
use crate::proto::HybridSignature;

const ED25519_SEED_LEN: usize = 32;
const MLDSA65_SEED_LEN: usize = 32;

/// Result of an HSM/connection test (kept for `TestHSMConnection` admin RPC).
#[derive(Debug, Clone)]
pub struct HsmConnectionTest {
    pub connected: bool,
    pub status: String,
    pub latency_ms: u64,
    pub hsm_model: String,
    pub available_slots: i64,
}

/// Hybrid cryptographic provider.
///
/// Wraps `ciris-crypto` Ed25519 + ML-DSA-65 signers. Wire format
/// (proto::HybridSignature with timestamp binding) is preserved.
pub struct HybridCrypto {
    ed25519_seed: [u8; ED25519_SEED_LEN],
    ed25519_signer: Ed25519Signer,
    mldsa_signer: MlDsa65Signer,
    mldsa_public_key: Vec<u8>,
    key_id: String,
}

impl HybridCrypto {
    /// Create a new crypto provider from settings.
    ///
    /// Only `file` mode is supported in production. Empty / unset
    /// `storage_mode` falls back to ephemeral key generation, intended
    /// for development and tests.
    pub fn new(settings: &CryptoSettings) -> Result<Self> {
        if settings.ed25519_key_path.is_some() && settings.mldsa_key_path.is_some() {
            Self::from_files(settings)
        } else {
            tracing::warn!(
                "No key paths configured — generating ephemeral keys. \
                 Records signed by this instance will not verify after restart. \
                 Set ED25519_KEY_PATH and MLDSA_KEY_PATH for persistent keys."
            );
            Self::generate_ephemeral()
        }
    }

    /// Load keys from filesystem. Both keys are stored as raw 32-byte seeds.
    fn from_files(settings: &CryptoSettings) -> Result<Self> {
        use std::fs;

        let ed25519_path = settings.ed25519_key_path.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("ed25519_key_path is required".to_string())
        })?;

        let mldsa_path = settings.mldsa_key_path.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("mldsa_key_path is required".to_string())
        })?;

        let ed25519_bytes = fs::read(ed25519_path).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to read Ed25519 seed: {}", e))
        })?;
        if ed25519_bytes.len() != ED25519_SEED_LEN {
            return Err(RegistryError::HsmUnavailable(format!(
                "Ed25519 seed must be {} bytes, got {}",
                ED25519_SEED_LEN,
                ed25519_bytes.len()
            )));
        }
        let mut ed25519_seed = [0u8; ED25519_SEED_LEN];
        ed25519_seed.copy_from_slice(&ed25519_bytes);

        let mldsa_bytes = fs::read(mldsa_path).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to read ML-DSA-65 seed: {}", e))
        })?;
        if mldsa_bytes.len() != MLDSA65_SEED_LEN {
            return Err(RegistryError::HsmUnavailable(format!(
                "ML-DSA-65 seed must be {} bytes, got {}",
                MLDSA65_SEED_LEN,
                mldsa_bytes.len()
            )));
        }

        Self::from_seeds(&ed25519_seed, &mldsa_bytes)
    }

    /// Construct from raw seed bytes for both algorithms.
    fn from_seeds(ed25519_seed: &[u8; ED25519_SEED_LEN], mldsa_seed: &[u8]) -> Result<Self> {
        let ed25519_signer = Ed25519Signer::from_seed(ed25519_seed)
            .map_err(|e| RegistryError::HsmUnavailable(format!("Ed25519Signer: {}", e)))?;

        let mldsa_signer = MlDsa65Signer::from_seed(mldsa_seed)
            .map_err(|e| RegistryError::HsmUnavailable(format!("MlDsa65Signer: {}", e)))?;

        let ed25519_pubkey = ed25519_signer
            .public_key()
            .map_err(|e| RegistryError::HsmUnavailable(format!("Ed25519 pubkey: {}", e)))?;

        let mldsa_public_key = mldsa_signer
            .public_key()
            .map_err(|e| RegistryError::HsmUnavailable(format!("ML-DSA-65 pubkey: {}", e)))?;

        let key_id = Self::fingerprint(&ed25519_pubkey);

        let mut seed_owned = [0u8; ED25519_SEED_LEN];
        seed_owned.copy_from_slice(ed25519_seed);

        Ok(Self {
            ed25519_seed: seed_owned,
            ed25519_signer,
            mldsa_signer,
            mldsa_public_key,
            key_id,
        })
    }

    /// Generate ephemeral keys (development / tests / one-shot custodied
    /// keypair generation for partner agents).
    pub fn generate_ephemeral() -> Result<Self> {
        use rand::rngs::OsRng;
        use rand::RngCore;

        let mut ed25519_seed = [0u8; ED25519_SEED_LEN];
        OsRng.fill_bytes(&mut ed25519_seed);

        let mut mldsa_seed = [0u8; MLDSA65_SEED_LEN];
        OsRng.fill_bytes(&mut mldsa_seed);

        Self::from_seeds(&ed25519_seed, &mldsa_seed)
    }

    /// Test connection / key availability (for `TestHSMConnection` admin RPC).
    pub fn test_connection(settings: &CryptoSettings) -> HsmConnectionTest {
        let start = Instant::now();

        let ed25519_ok = settings
            .ed25519_key_path
            .as_ref()
            .map(|p| Path::new(p).exists())
            .unwrap_or(false);
        let mldsa_ok = settings
            .mldsa_key_path
            .as_ref()
            .map(|p| Path::new(p).exists())
            .unwrap_or(false);

        if ed25519_ok && mldsa_ok {
            HsmConnectionTest {
                connected: true,
                status: "Key files found and readable".to_string(),
                latency_ms: start.elapsed().as_millis() as u64,
                hsm_model: "ciris-crypto / file-backed seeds".to_string(),
                available_slots: 2,
            }
        } else if settings.ed25519_key_path.is_none() && settings.mldsa_key_path.is_none() {
            HsmConnectionTest {
                connected: true,
                status: "Ephemeral mode (no key paths configured)".to_string(),
                latency_ms: start.elapsed().as_millis() as u64,
                hsm_model: "ciris-crypto / ephemeral".to_string(),
                available_slots: 0,
            }
        } else {
            HsmConnectionTest {
                connected: false,
                status: format!(
                    "Key files missing: ed25519_seed={}, mldsa_seed={}",
                    ed25519_ok, mldsa_ok
                ),
                latency_ms: start.elapsed().as_millis() as u64,
                hsm_model: "ciris-crypto / file-backed seeds".to_string(),
                available_slots: 0,
            }
        }
    }

    /// Get the key ID (SHA-256 of Ed25519 public key, hex).
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Get Ed25519 public key bytes (32 bytes).
    pub fn ed25519_public_key(&self) -> Vec<u8> {
        self.ed25519_signer
            .public_key()
            .expect("ciris-crypto Ed25519Signer::public_key is infallible after construction")
    }

    /// Get Ed25519 private key bytes (the 32-byte seed).
    /// Used only at generation time for partner-agent custodied keys.
    /// NEVER call this on the steward instance.
    pub fn ed25519_private_key_bytes(&self) -> Vec<u8> {
        self.ed25519_seed.to_vec()
    }

    /// Get ML-DSA-65 public key bytes (FIPS 204 encoding).
    pub fn mldsa_public_key(&self) -> Vec<u8> {
        self.mldsa_public_key.clone()
    }

    /// Sign data using the registry's hybrid signature scheme.
    ///
    /// Wire format (preserved from prior pqcrypto-dilithium implementation):
    /// - Classical: Ed25519(SHA-256(data))
    /// - PQC: ML-DSA-65(classical_sig || data_hash || timestamp_le)
    ///
    /// The timestamp binding is inside the PQC payload to prevent
    /// signature reuse across time windows.
    pub fn sign(&self, data: &[u8]) -> Result<HybridSignature> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RegistryError::Internal(e.to_string()))?
            .as_secs() as i64;

        let mut hasher = Sha256::new();
        hasher.update(data);
        let data_hash = hasher.finalize();

        let classical_sig = self
            .ed25519_signer
            .sign(&data_hash)
            .map_err(|e| RegistryError::Internal(format!("Ed25519 sign failed: {}", e)))?;

        let mut pq_message = Vec::with_capacity(classical_sig.len() + data_hash.len() + 8);
        pq_message.extend_from_slice(&classical_sig);
        pq_message.extend_from_slice(&data_hash);
        pq_message.extend_from_slice(&timestamp.to_le_bytes());

        let pq_signature = self
            .mldsa_signer
            .sign(&pq_message)
            .map_err(|e| RegistryError::Internal(format!("ML-DSA-65 sign failed: {}", e)))?;

        Ok(HybridSignature {
            classical_signature: classical_sig.into(),
            post_quantum_signature: pq_signature.into(),
            timestamp,
            key_id: self.key_id.clone(),
        })
    }

    /// Verify a hybrid signature.
    pub fn verify(
        &self,
        data: &[u8],
        signature: &HybridSignature,
        ed25519_pubkey: &[u8],
        mldsa_pubkey: &[u8],
    ) -> Result<bool> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RegistryError::Internal(e.to_string()))?
            .as_secs() as i64;

        if signature.timestamp > now + 300 || signature.timestamp < now - 300 {
            return Err(RegistryError::SignatureExpired);
        }

        let mut hasher = Sha256::new();
        hasher.update(data);
        let data_hash = hasher.finalize();

        let classical_sig: &[u8] = signature.classical_signature.as_ref();

        let ed_verifier = Ed25519Verifier::new();
        let classical_ok = ed_verifier
            .verify(ed25519_pubkey, &data_hash, classical_sig)
            .map_err(|e| RegistryError::InvalidSignature(format!("Ed25519: {}", e)))?;
        if !classical_ok {
            return Err(RegistryError::InvalidSignature(
                "Ed25519 signature verification failed".to_string(),
            ));
        }

        let mut pq_message = Vec::with_capacity(classical_sig.len() + data_hash.len() + 8);
        pq_message.extend_from_slice(classical_sig);
        pq_message.extend_from_slice(&data_hash);
        pq_message.extend_from_slice(&signature.timestamp.to_le_bytes());

        let pq_verifier = MlDsa65Verifier::new();
        let pq_ok = pq_verifier
            .verify(
                mldsa_pubkey,
                &pq_message,
                signature.post_quantum_signature.as_ref(),
            )
            .map_err(|e| RegistryError::InvalidSignature(format!("ML-DSA-65: {}", e)))?;
        if !pq_ok {
            return Err(RegistryError::InvalidSignature(
                "ML-DSA-65 signature verification failed".to_string(),
            ));
        }

        Ok(true)
    }

    /// Compute SHA-256 fingerprint of key bytes (hex).
    pub fn fingerprint(key_bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key_bytes);
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let crypto = HybridCrypto::generate_ephemeral().unwrap();
        let data = b"Hello, World!";

        let signature = crypto.sign(data).unwrap();

        let result = crypto
            .verify(
                data,
                &signature,
                &crypto.ed25519_public_key(),
                &crypto.mldsa_public_key(),
            )
            .unwrap();

        assert!(result);
    }

    #[test]
    fn test_tampered_data_fails() {
        let crypto = HybridCrypto::generate_ephemeral().unwrap();
        let data = b"Hello, World!";
        let tampered = b"Hello, Tampered!";

        let signature = crypto.sign(data).unwrap();

        let result = crypto.verify(
            tampered,
            &signature,
            &crypto.ed25519_public_key(),
            &crypto.mldsa_public_key(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_seed_roundtrip() {
        let crypto = HybridCrypto::generate_ephemeral().unwrap();
        let seed = crypto.ed25519_private_key_bytes();
        let pubkey = crypto.ed25519_public_key();

        let mut seed_arr = [0u8; ED25519_SEED_LEN];
        seed_arr.copy_from_slice(&seed);

        // Reconstruct from seed and verify pubkey matches
        let signer = Ed25519Signer::from_seed(&seed_arr).unwrap();
        let reconstructed_pubkey = signer.public_key().unwrap();
        assert_eq!(pubkey, reconstructed_pubkey);
    }
}
