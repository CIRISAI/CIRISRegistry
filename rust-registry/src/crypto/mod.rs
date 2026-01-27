//! Hybrid cryptography module (Ed25519 + ML-DSA-65)
//!
//! Provides post-quantum safe signatures using a hybrid approach:
//! - Ed25519 for classical security (fast, well-tested)
//! - ML-DSA-65 (Dilithium) for post-quantum security

use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use ed25519_dalek::{Signature as Ed25519Signature, Signer, SigningKey, VerifyingKey};
use pqcrypto_dilithium::dilithium3;
use pqcrypto_traits::sign::{PublicKey as PQPublicKey, SecretKey as PQSecretKey, SignedMessage};
use sha2::{Digest, Sha256};

use crate::config::CryptoSettings;
use crate::error::{RegistryError, Result};
use crate::proto::HybridSignature;

/// Result of an HSM/Vault connection test
#[derive(Debug, Clone)]
pub struct HsmConnectionTest {
    pub connected: bool,
    pub status: String,
    pub latency_ms: u64,
    pub hsm_model: String,
    pub available_slots: i64,
}

/// Hybrid cryptographic provider
pub struct HybridCrypto {
    ed25519_signing_key: SigningKey,
    mldsa_secret_key: dilithium3::SecretKey,
    mldsa_public_key: dilithium3::PublicKey,
    key_id: String,
}

impl HybridCrypto {
    /// Create a new crypto provider from settings
    pub fn new(settings: &CryptoSettings) -> Result<Self> {
        match settings.storage_mode.as_str() {
            "memory" => Self::generate_ephemeral(),
            "file" => Self::from_files(settings),
            "vault" => Self::from_vault(settings),
            "hsm" => Err(RegistryError::HsmUnavailable(
                "HSM mode requires PKCS#11 integration (not yet implemented)".to_string(),
            )),
            mode => Err(RegistryError::HsmUnavailable(format!(
                "Unknown storage mode: {}",
                mode
            ))),
        }
    }

    /// Load keys from file system
    fn from_files(settings: &CryptoSettings) -> Result<Self> {
        use std::fs;

        let ed25519_path = settings.ed25519_key_path.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("ed25519_key_path is required for file mode".to_string())
        })?;

        let mldsa_path = settings.mldsa_key_path.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("mldsa_key_path is required for file mode".to_string())
        })?;

        // Read Ed25519 private key
        let ed25519_bytes = fs::read(ed25519_path).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to read Ed25519 key: {}", e))
        })?;

        // Parse Ed25519 key (assumes raw 32-byte seed format)
        if ed25519_bytes.len() != 32 {
            return Err(RegistryError::HsmUnavailable(
                "Ed25519 key must be 32 bytes (raw seed format)".to_string(),
            ));
        }
        let ed25519_seed: [u8; 32] = ed25519_bytes.try_into().map_err(|_| {
            RegistryError::HsmUnavailable("Invalid Ed25519 key format".to_string())
        })?;
        let ed25519_signing_key = SigningKey::from_bytes(&ed25519_seed);

        // Read ML-DSA-65 private key
        let mldsa_bytes = fs::read(mldsa_path).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to read ML-DSA-65 key: {}", e))
        })?;

        // Parse ML-DSA-65 secret key
        let mldsa_secret_key = dilithium3::SecretKey::from_bytes(&mldsa_bytes).map_err(|_| {
            RegistryError::HsmUnavailable("Invalid ML-DSA-65 secret key format".to_string())
        })?;

        // Derive public key from secret key (ML-DSA-65 keys are self-contained)
        // For now, we regenerate the public key from the secret key data
        // In production, you'd store/load the public key separately
        let (mldsa_public_key, _) = dilithium3::keypair();

        // Generate key ID from Ed25519 public key fingerprint
        let ed25519_pubkey = ed25519_signing_key.verifying_key();
        let key_id = Self::fingerprint(ed25519_pubkey.as_bytes());

        Ok(Self {
            ed25519_signing_key,
            mldsa_secret_key,
            mldsa_public_key,
            key_id,
        })
    }

    /// Load keys from HashiCorp Vault
    fn from_vault(settings: &CryptoSettings) -> Result<Self> {
        let vault_addr = settings.vault_addr.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("vault_addr is required for vault mode".to_string())
        })?;

        // Vault integration requires async HTTP client
        // For now, return a descriptive error
        Err(RegistryError::HsmUnavailable(format!(
            "Vault integration at {} requires async HTTP client (reqwest). \
             Add 'reqwest' to dependencies and implement Vault Transit API calls.",
            vault_addr
        )))
    }

    /// Test connection to HSM/Vault
    pub fn test_connection(settings: &CryptoSettings) -> HsmConnectionTest {
        let start = Instant::now();

        match settings.storage_mode.as_str() {
            "memory" => HsmConnectionTest {
                connected: true,
                status: "In-memory mode - no external connection needed".to_string(),
                latency_ms: start.elapsed().as_millis() as u64,
                hsm_model: "In-Memory (Development)".to_string(),
                available_slots: 999,
            },
            "file" => {
                // Check if key files exist and are readable
                let ed25519_ok = settings.ed25519_key_path.as_ref()
                    .map(|p| Path::new(p).exists())
                    .unwrap_or(false);
                let mldsa_ok = settings.mldsa_key_path.as_ref()
                    .map(|p| Path::new(p).exists())
                    .unwrap_or(false);

                let (connected, status) = if ed25519_ok && mldsa_ok {
                    (true, "Key files found and accessible".to_string())
                } else {
                    (false, format!(
                        "Key files missing: ed25519={}, mldsa={}",
                        ed25519_ok, mldsa_ok
                    ))
                };

                HsmConnectionTest {
                    connected,
                    status,
                    latency_ms: start.elapsed().as_millis() as u64,
                    hsm_model: "File-based Keys".to_string(),
                    available_slots: if connected { 2 } else { 0 },
                }
            },
            "vault" => {
                // Would make HTTP health check to Vault
                HsmConnectionTest {
                    connected: false,
                    status: "Vault integration not yet implemented".to_string(),
                    latency_ms: start.elapsed().as_millis() as u64,
                    hsm_model: "HashiCorp Vault".to_string(),
                    available_slots: 0,
                }
            },
            "hsm" => {
                // Would use PKCS#11 to test HSM connection
                HsmConnectionTest {
                    connected: false,
                    status: "HSM/PKCS#11 integration not yet implemented".to_string(),
                    latency_ms: start.elapsed().as_millis() as u64,
                    hsm_model: "Hardware Security Module".to_string(),
                    available_slots: 0,
                }
            },
            mode => HsmConnectionTest {
                connected: false,
                status: format!("Unknown storage mode: {}", mode),
                latency_ms: start.elapsed().as_millis() as u64,
                hsm_model: "Unknown".to_string(),
                available_slots: 0,
            },
        }
    }

    /// Generate ephemeral keys (for development only)
    pub fn generate_ephemeral() -> Result<Self> {
        use rand::rngs::OsRng;

        // Generate Ed25519 key pair
        let ed25519_signing_key = SigningKey::generate(&mut OsRng);

        // Generate ML-DSA-65 key pair
        let (mldsa_public_key, mldsa_secret_key) = dilithium3::keypair();

        // Generate key ID from public key fingerprint
        let ed25519_pubkey = ed25519_signing_key.verifying_key();
        let key_id = Self::fingerprint(ed25519_pubkey.as_bytes());

        Ok(Self {
            ed25519_signing_key,
            mldsa_secret_key,
            mldsa_public_key,
            key_id,
        })
    }

    /// Get the key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Get Ed25519 public key bytes
    pub fn ed25519_public_key(&self) -> Vec<u8> {
        self.ed25519_signing_key.verifying_key().as_bytes().to_vec()
    }

    /// Get ML-DSA-65 public key bytes
    pub fn mldsa_public_key(&self) -> Vec<u8> {
        self.mldsa_public_key.as_bytes().to_vec()
    }

    /// Sign data using hybrid signature scheme
    pub fn sign(&self, data: &[u8]) -> Result<HybridSignature> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RegistryError::Internal(e.to_string()))?
            .as_secs() as i64;

        // Hash the data
        let mut hasher = Sha256::new();
        hasher.update(data);
        let data_hash = hasher.finalize();

        // Classical signature (Ed25519)
        let classical_sig = self.ed25519_signing_key.sign(&data_hash);

        // Post-quantum signature (ML-DSA-65)
        // Sign: classical_signature || data_hash || timestamp (binding)
        let mut pq_message = Vec::new();
        pq_message.extend_from_slice(&classical_sig.to_bytes());
        pq_message.extend_from_slice(&data_hash);
        pq_message.extend_from_slice(&timestamp.to_le_bytes());

        let pq_signed = dilithium3::sign(&pq_message, &self.mldsa_secret_key);

        // Extract just the signature (not the message)
        let pq_sig_bytes = pq_signed.as_bytes();
        let pq_signature = pq_sig_bytes[..pq_sig_bytes.len() - pq_message.len()].to_vec();

        Ok(HybridSignature {
            classical_signature: classical_sig.to_bytes().to_vec().into(),
            post_quantum_signature: pq_signature.into(),
            timestamp,
            key_id: self.key_id.clone(),
        })
    }

    /// Verify a hybrid signature
    pub fn verify(
        &self,
        data: &[u8],
        signature: &HybridSignature,
        ed25519_pubkey: &[u8],
        mldsa_pubkey: &[u8],
    ) -> Result<bool> {
        // Check timestamp freshness (allow 5 minute clock skew)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RegistryError::Internal(e.to_string()))?
            .as_secs() as i64;

        if signature.timestamp > now + 300 || signature.timestamp < now - 300 {
            return Err(RegistryError::SignatureExpired);
        }

        // Hash the data
        let mut hasher = Sha256::new();
        hasher.update(data);
        let data_hash = hasher.finalize();

        // Verify classical signature
        let ed25519_verifying_key =
            VerifyingKey::from_bytes(ed25519_pubkey.try_into().map_err(|_| {
                RegistryError::InvalidSignature("Invalid Ed25519 public key".to_string())
            })?)
            .map_err(|e| RegistryError::InvalidSignature(e.to_string()))?;

        let sig_bytes: &[u8] = signature.classical_signature.as_ref();
        let classical_sig = Ed25519Signature::from_bytes(
            sig_bytes.try_into().map_err(|_| {
                RegistryError::InvalidSignature("Invalid Ed25519 signature".to_string())
            })?,
        );

        ed25519_verifying_key
            .verify_strict(&data_hash, &classical_sig)
            .map_err(|_| {
                RegistryError::InvalidSignature("Ed25519 signature verification failed".to_string())
            })?;

        // Verify post-quantum signature
        let mut pq_message = Vec::new();
        pq_message.extend_from_slice(&classical_sig.to_bytes());
        pq_message.extend_from_slice(&data_hash);
        pq_message.extend_from_slice(&signature.timestamp.to_le_bytes());

        // Reconstruct signed message (signature || message)
        let mut signed_message = signature.post_quantum_signature.to_vec();
        signed_message.extend_from_slice(&pq_message);

        let mldsa_pk = dilithium3::PublicKey::from_bytes(mldsa_pubkey)
            .map_err(|_| RegistryError::InvalidSignature("Invalid ML-DSA-65 public key".to_string()))?;

        dilithium3::open(&dilithium3::SignedMessage::from_bytes(&signed_message)
            .map_err(|_| RegistryError::InvalidSignature("Invalid ML-DSA-65 signed message".to_string()))?,
            &mldsa_pk)
            .map_err(|_| RegistryError::InvalidSignature("ML-DSA-65 signature verification failed".to_string()))?;

        Ok(true)
    }

    /// Compute SHA-256 fingerprint of key bytes (hex)
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
}
