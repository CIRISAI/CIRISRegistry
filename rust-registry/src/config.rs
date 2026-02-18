//! Configuration management for CIRISRegistry

use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

impl Environment {
    /// Convert config environment to proto RegistryEnvironment i32 value.
    /// Proto values: ENV_UNSPECIFIED=0, ENV_PRODUCTION=1, ENV_STAGING=2,
    ///               ENV_CANARY=3, ENV_DEVELOPMENT=4
    pub fn to_proto_i32(self) -> i32 {
        match self {
            Environment::Production => 1,  // ENV_PRODUCTION
            Environment::Staging => 2,     // ENV_STAGING
            Environment::Canary => 3,      // ENV_CANARY
            Environment::Development => 4, // ENV_DEVELOPMENT
        }
    }
}

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
    /// Path to ML-DSA-65 public key
    pub mldsa_public_key_path: Option<String>,
    /// Key storage mode: memory, vault, cloudkms, etc.
    pub storage_mode: String,
    /// Vault address (if using HashiCorp Vault)
    pub vault_addr: Option<String>,
    /// Vault token (if using HashiCorp Vault)
    pub vault_token: Option<String>,
    /// Vault Transit key name (default: registry-signing)
    pub vault_key_name: Option<String>,
    /// Skip TLS verification for Vault (for self-signed certs)
    pub vault_skip_verify: bool,
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

        let is_production = environment == Environment::Production;
        let is_production_like = matches!(
            environment,
            Environment::Production | Environment::Staging | Environment::Canary
        );

        // Get database password with production validation
        let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "ciris_dev".to_string());
        if is_production && (db_password == "ciris_dev" || db_password.is_empty()) {
            anyhow::bail!(
                "SECURITY: DB_PASSWORD must be set to a secure value in production. \
                 The default 'ciris_dev' password is not allowed."
            );
        }

        // Get JWT secret with production validation
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "development-secret-do-not-use-in-production".to_string());
        if is_production_like {
            if jwt_secret.contains("development") || jwt_secret.contains("do-not-use") {
                anyhow::bail!(
                    "SECURITY: JWT_SECRET must be set to a secure value in {}. \
                     Development secrets are not allowed.",
                    format!("{:?}", environment).to_lowercase()
                );
            }
            if jwt_secret.len() < 32 {
                anyhow::bail!(
                    "SECURITY: JWT_SECRET must be at least 32 characters in {}. \
                     Current length: {}",
                    format!("{:?}", environment).to_lowercase(),
                    jwt_secret.len()
                );
            }
        }

        // Default SSL mode based on environment
        let default_sslmode = if is_production_like {
            "require"
        } else {
            "disable"
        };
        let sslmode = env::var("DB_SSLMODE").unwrap_or_else(|_| default_sslmode.to_string());

        // Warn if SSL is disabled in production-like environments
        if is_production_like && sslmode == "disable" {
            tracing::warn!(
                "SECURITY WARNING: DB_SSLMODE is 'disable' in {}. \
                 Consider using 'require' or 'verify-full' for encrypted database connections.",
                format!("{:?}", environment).to_lowercase()
            );
        }

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
                password: db_password,
                name: env::var("DB_NAME").unwrap_or_else(|_| "ciris_registry".to_string()),
                sslmode,
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
                mldsa_public_key_path: env::var("MLDSA_PUBLIC_KEY_PATH").ok(),
                storage_mode: env::var("KEY_STORAGE_MODE").unwrap_or_else(|_| "memory".to_string()),
                vault_addr: env::var("VAULT_ADDR").ok(),
                vault_token: env::var("VAULT_TOKEN").ok(),
                vault_key_name: env::var("VAULT_KEY_NAME").ok(),
                vault_skip_verify: env::var("VAULT_SKIP_VERIFY")
                    .map(|v| v == "true" || v == "1")
                    .unwrap_or(false),
            },
            auth: AuthSettings {
                jwt_secret,
                jwt_issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "ciris-registry".to_string()),
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
