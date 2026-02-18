//! HashiCorp Vault Transit integration for signing operations
//!
//! This module provides integration with Vault Transit secrets engine
//! for Ed25519 signing operations. ML-DSA-65 (post-quantum) keys are
//! stored in Vault KV secrets engine since Transit doesn't support PQC.

use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::CryptoSettings;
use crate::error::{RegistryError, Result};

/// ML-DSA keypair stored in Vault KV
#[derive(Debug, Clone)]
pub struct MldsaKeyPair {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Vault Transit client for signing operations
pub struct VaultClient {
    client: Client,
    addr: String,
    token: String,
    key_name: String,
    /// Cached Ed25519 public key
    cached_pubkey: Arc<RwLock<Option<Vec<u8>>>>,
}

#[derive(Debug, Serialize)]
struct SignRequest {
    input: String,
    hash_algorithm: String,
    prehashed: bool,
}

#[derive(Debug, Deserialize)]
struct VaultResponse<T> {
    data: Option<T>,
    errors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct SignResponseData {
    signature: String,
}

#[derive(Debug, Deserialize)]
struct KeyResponseData {
    keys: std::collections::HashMap<String, KeyVersionData>,
    latest_version: i32,
}

#[derive(Debug, Deserialize)]
struct KeyVersionData {
    public_key: Option<String>,
}

impl VaultClient {
    /// Create a new Vault client from settings
    pub fn new(settings: &CryptoSettings) -> Result<Self> {
        let addr = settings.vault_addr.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("VAULT_ADDR is required for vault mode".to_string())
        })?;

        let token = settings.vault_token.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("VAULT_TOKEN is required for vault mode".to_string())
        })?;

        let key_name = settings
            .vault_key_name
            .clone()
            .unwrap_or_else(|| "registry-signing".to_string());

        let client = if settings.vault_skip_verify {
            Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .map_err(|e| RegistryError::HsmUnavailable(format!("Failed to create HTTP client: {}", e)))?
        } else {
            Client::new()
        };

        Ok(Self {
            client,
            addr: addr.clone(),
            token: token.clone(),
            key_name,
            cached_pubkey: Arc::new(RwLock::new(None)),
        })
    }

    /// Get the Ed25519 public key from Vault Transit
    pub async fn get_public_key(&self) -> Result<Vec<u8>> {
        // Check cache first
        {
            let cache = self.cached_pubkey.read().await;
            if let Some(ref key) = *cache {
                return Ok(key.clone());
            }
        }

        let url = format!("{}/v1/transit/keys/{}", self.addr, self.key_name);

        let response = self
            .client
            .get(&url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Vault request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RegistryError::HsmUnavailable(format!(
                "Vault returned {}: {}",
                status, body
            )));
        }

        let vault_response: VaultResponse<KeyResponseData> = response
            .json()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Failed to parse Vault response: {}", e)))?;

        let data = vault_response.data.ok_or_else(|| {
            RegistryError::HsmUnavailable("Vault response missing data".to_string())
        })?;

        // Get the latest key version's public key
        let version = data.latest_version.to_string();
        let key_version = data.keys.get(&version).ok_or_else(|| {
            RegistryError::HsmUnavailable(format!("Key version {} not found", version))
        })?;

        let pubkey_b64 = key_version.public_key.as_ref().ok_or_else(|| {
            RegistryError::HsmUnavailable("Key version missing public_key".to_string())
        })?;

        let b64 = base64::engine::general_purpose::STANDARD;
        let pubkey = b64.decode(pubkey_b64).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to decode public key: {}", e))
        })?;

        // Cache the public key
        {
            let mut cache = self.cached_pubkey.write().await;
            *cache = Some(pubkey.clone());
        }

        Ok(pubkey)
    }

    /// Sign data using Vault Transit (Ed25519)
    ///
    /// The input should be the raw data to sign (will be hashed internally).
    pub async fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let url = format!("{}/v1/transit/sign/{}", self.addr, self.key_name);

        // Base64 encode the input data
        let b64 = base64::engine::general_purpose::STANDARD;
        let input = b64.encode(data);

        let request = SignRequest {
            input,
            hash_algorithm: "sha2-256".to_string(),
            prehashed: false,
        };

        let response = self
            .client
            .post(&url)
            .header("X-Vault-Token", &self.token)
            .json(&request)
            .send()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Vault sign request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RegistryError::HsmUnavailable(format!(
                "Vault sign returned {}: {}",
                status, body
            )));
        }

        let vault_response: VaultResponse<SignResponseData> = response
            .json()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Failed to parse sign response: {}", e)))?;

        let data = vault_response.data.ok_or_else(|| {
            RegistryError::HsmUnavailable("Vault sign response missing data".to_string())
        })?;

        // Vault signature format: "vault:v1:base64_signature"
        let sig_parts: Vec<&str> = data.signature.split(':').collect();
        if sig_parts.len() != 3 {
            return Err(RegistryError::HsmUnavailable(format!(
                "Invalid Vault signature format: {}",
                data.signature
            )));
        }

        let signature = b64.decode(sig_parts[2]).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to decode signature: {}", e))
        })?;

        Ok(signature)
    }

    /// Test connection to Vault
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/v1/sys/health", self.addr);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Vault health check failed: {}", e)))?;

        // Vault returns 200 for initialized+unsealed+active
        // 429 for standby, 472 for DR secondary, 473 for perf standby, 501 for uninitialized, 503 for sealed
        Ok(response.status().is_success() || response.status().as_u16() == 429)
    }

    /// Get key name
    pub fn key_name(&self) -> &str {
        &self.key_name
    }

    /// Get ML-DSA keypair from Vault KV secrets engine
    ///
    /// Path: secret/data/registry/mldsa-keys
    pub async fn get_mldsa_keys(&self) -> Result<Option<MldsaKeyPair>> {
        let url = format!("{}/v1/secret/data/registry/mldsa-keys", self.addr);

        let response = self
            .client
            .get(&url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Vault KV request failed: {}", e)))?;

        // 404 means key doesn't exist yet
        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RegistryError::HsmUnavailable(format!(
                "Vault KV returned {}: {}",
                status, body
            )));
        }

        let vault_response: VaultResponse<KvResponseData> = response
            .json()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Failed to parse KV response: {}", e)))?;

        let data = vault_response.data.ok_or_else(|| {
            RegistryError::HsmUnavailable("Vault KV response missing data".to_string())
        })?;

        let kv_data = data.data;
        let b64 = base64::engine::general_purpose::STANDARD;

        let secret_key = kv_data.get("secret_key").ok_or_else(|| {
            RegistryError::HsmUnavailable("ML-DSA secret_key missing from Vault".to_string())
        })?;
        let public_key = kv_data.get("public_key").ok_or_else(|| {
            RegistryError::HsmUnavailable("ML-DSA public_key missing from Vault".to_string())
        })?;

        let secret_key_bytes = b64.decode(secret_key).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to decode ML-DSA secret key: {}", e))
        })?;
        let public_key_bytes = b64.decode(public_key).map_err(|e| {
            RegistryError::HsmUnavailable(format!("Failed to decode ML-DSA public key: {}", e))
        })?;

        Ok(Some(MldsaKeyPair {
            secret_key: secret_key_bytes,
            public_key: public_key_bytes,
        }))
    }

    /// Store ML-DSA keypair in Vault KV secrets engine
    ///
    /// Path: secret/data/registry/mldsa-keys
    pub async fn store_mldsa_keys(&self, keypair: &MldsaKeyPair) -> Result<()> {
        let url = format!("{}/v1/secret/data/registry/mldsa-keys", self.addr);
        let b64 = base64::engine::general_purpose::STANDARD;

        let mut kv_data = std::collections::HashMap::new();
        kv_data.insert("secret_key", b64.encode(&keypair.secret_key));
        kv_data.insert("public_key", b64.encode(&keypair.public_key));

        let request = KvWriteRequest { data: kv_data };

        let response = self
            .client
            .post(&url)
            .header("X-Vault-Token", &self.token)
            .json(&request)
            .send()
            .await
            .map_err(|e| RegistryError::HsmUnavailable(format!("Vault KV write failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RegistryError::HsmUnavailable(format!(
                "Vault KV write returned {}: {}",
                status, body
            )));
        }

        tracing::info!("Stored ML-DSA keypair in Vault KV");
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct KvResponseData {
    data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct KvWriteRequest {
    data: std::collections::HashMap<&'static str, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_client_requires_addr() {
        let settings = CryptoSettings {
            ed25519_key_path: None,
            mldsa_key_path: None,
            mldsa_public_key_path: None,
            storage_mode: "vault".to_string(),
            vault_addr: None,
            vault_token: Some("test".to_string()),
            vault_key_name: None,
            vault_skip_verify: false,
        };

        let result = VaultClient::new(&settings);
        assert!(result.is_err());
    }

    #[test]
    fn test_vault_client_requires_token() {
        let settings = CryptoSettings {
            ed25519_key_path: None,
            mldsa_key_path: None,
            mldsa_public_key_path: None,
            storage_mode: "vault".to_string(),
            vault_addr: Some("http://localhost:8200".to_string()),
            vault_token: None,
            vault_key_name: None,
            vault_skip_verify: false,
        };

        let result = VaultClient::new(&settings);
        assert!(result.is_err());
    }
}
