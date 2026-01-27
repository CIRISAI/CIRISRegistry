//! Configuration management for CIRISRegistry

use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: Environment,
    pub grpc_port: u16,
    pub http_port: u16,
    pub database: DatabaseSettings,
    pub crypto: CryptoSettings,
    pub auth: AuthSettings,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Canary,
    Production,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
    pub sslmode: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.user, self.password, self.host, self.port, self.name, self.sslmode
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CryptoSettings {
    /// Path to Ed25519 private key (PEM or raw)
    pub ed25519_key_path: Option<String>,
    /// Path to ML-DSA-65 private key
    pub mldsa_key_path: Option<String>,
    /// Key storage mode: memory, vault, cloudkms, etc.
    pub storage_mode: String,
    /// Vault address (if using HashiCorp Vault)
    pub vault_addr: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    /// JWT secret for token validation
    pub jwt_secret: String,
    /// JWT issuer
    pub jwt_issuer: String,
    /// Enable mTLS
    pub mtls_enabled: bool,
    /// Path to TLS certificate
    pub tls_cert_path: Option<String>,
    /// Path to TLS private key
    pub tls_key_path: Option<String>,
    /// Path to CA certificate for client verification
    pub ca_cert_path: Option<String>,
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        // Load .env file if present
        dotenvy::dotenv().ok();

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            "staging" => Environment::Staging,
            "canary" => Environment::Canary,
            _ => Environment::Development,
        };

        Ok(Settings {
            environment,
            grpc_port: env::var("GRPC_PORT")
                .unwrap_or_else(|_| "50051".to_string())
                .parse()
                .context("Invalid GRPC_PORT")?,
            http_port: env::var("HTTP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("Invalid HTTP_PORT")?,
            database: DatabaseSettings {
                host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("DB_PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .context("Invalid DB_PORT")?,
                user: env::var("DB_USER").unwrap_or_else(|_| "ciris".to_string()),
                password: env::var("DB_PASSWORD").unwrap_or_else(|_| "ciris_dev".to_string()),
                name: env::var("DB_NAME").unwrap_or_else(|_| "ciris_registry".to_string()),
                sslmode: env::var("DB_SSLMODE").unwrap_or_else(|_| "disable".to_string()),
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .context("Invalid DB_MAX_CONNECTIONS")?,
                min_connections: env::var("DB_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()
                    .context("Invalid DB_MIN_CONNECTIONS")?,
            },
            crypto: CryptoSettings {
                ed25519_key_path: env::var("ED25519_KEY_PATH").ok(),
                mldsa_key_path: env::var("MLDSA_KEY_PATH").ok(),
                storage_mode: env::var("KEY_STORAGE_MODE")
                    .unwrap_or_else(|_| "memory".to_string()),
                vault_addr: env::var("VAULT_ADDR").ok(),
            },
            auth: AuthSettings {
                jwt_secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "development-secret-do-not-use-in-production".to_string()),
                jwt_issuer: env::var("JWT_ISSUER")
                    .unwrap_or_else(|_| "ciris-registry".to_string()),
                mtls_enabled: env::var("MTLS_ENABLED")
                    .map(|v| v == "true" || v == "1")
                    .unwrap_or(false),
                tls_cert_path: env::var("TLS_CERT_PATH").ok(),
                tls_key_path: env::var("TLS_KEY_PATH").ok(),
                ca_cert_path: env::var("CA_CERT_PATH").ok(),
            },
        })
    }
}
