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
    pub federation: FederationSettings,
}

/// PoB §3.1 federation directory client settings (v1.4 R_FLAG).
///
/// Defaults to all-off — registry behaves identically to pre-v1.4 until
/// operators flip flags. See `docs/FEDERATION_CLIENT.md` for the
/// dependency-graph and rollback discipline.
#[derive(Debug, Clone, Deserialize)]
pub struct FederationSettings {
    /// Master switch for the persist-as-substrate code path. When `false`
    /// the registry never reaches out to persist; existing
    /// `trusted_primitive_keys` / `partner_keys` / `registry_signing_keys`
    /// tables remain authoritative. When `true`, registry dual-writes to
    /// persist on every admin RPC and reads through the federation cache
    /// on verify lookups.
    pub dual_write_enabled: bool,

    /// Persist's federation-directory endpoint. Empty string = no client
    /// configured; if `dual_write_enabled` is true and this is empty,
    /// boot fails with a config error so a misconfiguration can't
    /// silently downgrade to no-op.
    pub persist_endpoint: String,

    /// Cache TTL for federation rows. Default 300s (5 min). Above this,
    /// next read re-fetches from persist. Tuned per deployment based on
    /// observed `federation_cache_age_seconds` distribution.
    pub cache_ttl_seconds: u64,

    /// Hard ceiling on cache age. Even in fail-open mode (when
    /// `persist_required = false`), registry refuses to serve cache
    /// older than this. Bounds the deliberate-outage attack window for
    /// revoked-key replay. Default 3600s (1 hour).
    pub max_stale_cache_age_seconds: u64,

    /// When `true` the registry fails closed on any persist
    /// unavailability (no cache fallback). Default `false` — operators
    /// pick stricter posture for high-trust deployments. Independent of
    /// `max_stale_cache_age_seconds` (which is a hard backstop in either
    /// mode).
    pub persist_required: bool,

    /// Persist Engine DSN. URL-sniffed per `ciris_persist::engine::Engine::
    /// with_signer`:
    /// - `postgresql://...` / `postgres://...` → Postgres backend
    /// - `sqlite:///path.db` → SQLite (file at `/path.db`)
    /// - `sqlite::memory:` / `sqlite:///:memory:` → SQLite in-memory
    ///
    /// Default is `sqlite::memory:` — federation directory is ephemeral
    /// and lost on restart. Production deployments override to a
    /// postgres URL (typically the same database Registry uses for its
    /// own tables; Persist runs its own migrations and cohabits cleanly).
    pub persist_dsn: String,
}

impl Default for FederationSettings {
    fn default() -> Self {
        Self {
            dual_write_enabled: false,
            persist_endpoint: String::new(),
            cache_ttl_seconds: 300,
            max_stale_cache_age_seconds: 3600,
            persist_required: false,
            persist_dsn: "sqlite::memory:".to_string(),
        }
    }
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
    /// Path to Ed25519 private key (raw 32-byte seed). When unset, an
    /// ephemeral key is generated at boot — development only.
    pub ed25519_key_path: Option<String>,
    /// Path to ML-DSA-65 private key (raw 32-byte seed, FIPS 204).
    pub mldsa_key_path: Option<String>,
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
            federation: {
                let dual_write_enabled = env::var("FEDERATION_DUAL_WRITE_ENABLED")
                    .map(|v| v == "true" || v == "1")
                    .unwrap_or(false);
                let persist_endpoint =
                    env::var("FEDERATION_PERSIST_ENDPOINT").unwrap_or_default();

                // Misconfiguration guard: if dual-write is on but no
                // endpoint is set, fail loudly at boot rather than
                // silently downgrading to no-op.
                if dual_write_enabled && persist_endpoint.is_empty() {
                    anyhow::bail!(
                        "FEDERATION_DUAL_WRITE_ENABLED=true but \
                         FEDERATION_PERSIST_ENDPOINT is unset. Set the \
                         endpoint or disable dual-write."
                    );
                }

                FederationSettings {
                    dual_write_enabled,
                    persist_endpoint,
                    cache_ttl_seconds: env::var("FEDERATION_CACHE_TTL_SECONDS")
                        .unwrap_or_else(|_| "300".to_string())
                        .parse()
                        .context("Invalid FEDERATION_CACHE_TTL_SECONDS")?,
                    max_stale_cache_age_seconds: env::var(
                        "FEDERATION_MAX_STALE_CACHE_AGE_SECONDS",
                    )
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .context("Invalid FEDERATION_MAX_STALE_CACHE_AGE_SECONDS")?,
                    persist_required: env::var("FEDERATION_PERSIST_REQUIRED")
                        .map(|v| v == "true" || v == "1")
                        .unwrap_or(false),
                    persist_dsn: env::var("FEDERATION_PERSIST_DSN")
                        .unwrap_or_else(|_| "sqlite::memory:".to_string()),
                }
            },
        })
    }
}
