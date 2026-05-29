//! Registry signing key database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;
use crate::proto::registry_signing_key::SigningKeyStatus;

// Status constants (matching proto enum values)
const STATUS_PENDING: i32 = SigningKeyStatus::SigningKeyPending as i32;
const STATUS_ACTIVE: i32 = SigningKeyStatus::SigningKeyActive as i32;
const STATUS_RETIRED: i32 = SigningKeyStatus::SigningKeyRetired as i32;
const STATUS_ROTATED: i32 = SigningKeyStatus::SigningKeyStandby as i32; // Using standby for rotated

#[derive(Debug, Clone, FromRow)]
pub struct RegistrySigningKeyRow {
    pub key_id: String,
    pub storage_mode: i32,
    pub ed25519_public_key: Vec<u8>,
    pub ed25519_fingerprint: String,
    pub mldsa65_public_key: Vec<u8>,
    pub mldsa65_fingerprint: String,
    pub created_at: OffsetDateTime,
    pub activated_at: Option<OffsetDateTime>,
    pub rotated_at: Option<OffsetDateTime>,
    pub rotated_by: Option<String>,
    pub retired_at: Option<OffsetDateTime>,
    pub usage_count: i64,
    pub last_used: Option<OffsetDateTime>,
    pub status: i32,
    pub hsm_slot_id: Option<String>,
    pub hsm_label: Option<String>,
}

impl RegistrySigningKeyRow {
    pub fn to_proto(&self) -> proto::RegistrySigningKey {
        proto::RegistrySigningKey {
            key_id: self.key_id.clone(),
            storage_mode: self.storage_mode,
            ed25519_public_key: self.ed25519_public_key.clone().into(),
            ed25519_fingerprint: self.ed25519_fingerprint.clone(),
            mldsa65_public_key: self.mldsa65_public_key.clone().into(),
            mldsa65_fingerprint: self.mldsa65_fingerprint.clone(),
            created_at: self.created_at.unix_timestamp(),
            activated_at: self.activated_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            rotated_at: self.rotated_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            rotated_by: self.rotated_by.clone().unwrap_or_default(),
            retired_at: self.retired_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            usage_count: self.usage_count,
            last_used: self.last_used.map(|t| t.unix_timestamp()).unwrap_or(0),
            status: self.status,
            hsm_slot_id: self.hsm_slot_id.clone().unwrap_or_default(),
            hsm_label: self.hsm_label.clone().unwrap_or_default(),
        }
    }
}

/// Create a new registry signing key
pub async fn create_signing_key(
    pool: &PgPool,
    storage_mode: i32,
    ed25519_pubkey: &[u8],
    ed25519_fingerprint: &str,
    mldsa_pubkey: &[u8],
    mldsa_fingerprint: &str,
    hsm_slot_id: Option<&str>,
    hsm_label: Option<&str>,
) -> Result<String> {
    let key_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO registry_signing_keys (
            key_id, storage_mode, ed25519_public_key, ed25519_fingerprint,
            mldsa65_public_key, mldsa65_fingerprint, status,
            hsm_slot_id, hsm_label
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(&key_id)
    .bind(storage_mode)
    .bind(ed25519_pubkey)
    .bind(ed25519_fingerprint)
    .bind(mldsa_pubkey)
    .bind(mldsa_fingerprint)
    .bind(STATUS_PENDING)
    .bind(hsm_slot_id)
    .bind(hsm_label)
    .execute(pool)
    .await?;

    Ok(key_id)
}

/// Get the currently active signing key
pub async fn get_active_signing_key(pool: &PgPool) -> Result<Option<RegistrySigningKeyRow>> {
    let row = sqlx::query_as::<_, RegistrySigningKeyRow>(
        r#"
        SELECT key_id, storage_mode, ed25519_public_key, ed25519_fingerprint,
               mldsa65_public_key, mldsa65_fingerprint, created_at, activated_at,
               rotated_at, rotated_by, retired_at, usage_count, last_used,
               status, hsm_slot_id, hsm_label
        FROM registry_signing_keys
        WHERE status = $1
        ORDER BY activated_at DESC
        LIMIT 1
        "#,
    )
    .bind(STATUS_ACTIVE)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Get a specific signing key by ID
pub async fn get_signing_key(pool: &PgPool, key_id: &str) -> Result<Option<RegistrySigningKeyRow>> {
    let row = sqlx::query_as::<_, RegistrySigningKeyRow>(
        r#"
        SELECT key_id, storage_mode, ed25519_public_key, ed25519_fingerprint,
               mldsa65_public_key, mldsa65_fingerprint, created_at, activated_at,
               rotated_at, rotated_by, retired_at, usage_count, last_used,
               status, hsm_slot_id, hsm_label
        FROM registry_signing_keys
        WHERE key_id = $1
        "#,
    )
    .bind(key_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// List all signing keys
pub async fn list_signing_keys(
    pool: &PgPool,
    include_retired: bool,
) -> Result<Vec<RegistrySigningKeyRow>> {
    let query = if include_retired {
        r#"
        SELECT key_id, storage_mode, ed25519_public_key, ed25519_fingerprint,
               mldsa65_public_key, mldsa65_fingerprint, created_at, activated_at,
               rotated_at, rotated_by, retired_at, usage_count, last_used,
               status, hsm_slot_id, hsm_label
        FROM registry_signing_keys
        ORDER BY created_at DESC
        "#
    } else {
        r#"
        SELECT key_id, storage_mode, ed25519_public_key, ed25519_fingerprint,
               mldsa65_public_key, mldsa65_fingerprint, created_at, activated_at,
               rotated_at, rotated_by, retired_at, usage_count, last_used,
               status, hsm_slot_id, hsm_label
        FROM registry_signing_keys
        WHERE status != $1
        ORDER BY created_at DESC
        "#
    };

    let rows = if include_retired {
        sqlx::query_as::<_, RegistrySigningKeyRow>(query)
            .fetch_all(pool)
            .await?
    } else {
        sqlx::query_as::<_, RegistrySigningKeyRow>(query)
            .bind(STATUS_RETIRED)
            .fetch_all(pool)
            .await?
    };

    Ok(rows)
}

/// Rotate signing keys - activate new key and mark old as rotated
pub async fn rotate_signing_key(
    pool: &PgPool,
    old_key_id: &str,
    new_key_id: &str,
    rotated_by: &str,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    // Activate new key
    sqlx::query(
        r#"
        UPDATE registry_signing_keys
        SET status = $1, activated_at = NOW()
        WHERE key_id = $2 AND status = $3
        "#,
    )
    .bind(STATUS_ACTIVE)
    .bind(new_key_id)
    .bind(STATUS_PENDING)
    .execute(&mut *tx)
    .await?;

    // Mark old key as rotated (using standby status)
    sqlx::query(
        r#"
        UPDATE registry_signing_keys
        SET status = $1, rotated_at = NOW(), rotated_by = $2
        WHERE key_id = $3 AND status = $4
        "#,
    )
    .bind(STATUS_ROTATED)
    .bind(rotated_by)
    .bind(old_key_id)
    .bind(STATUS_ACTIVE)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Activate a signing key directly (for bootstrapping first key)
pub async fn activate_signing_key(pool: &PgPool, key_id: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE registry_signing_keys
        SET status = $1, activated_at = NOW()
        WHERE key_id = $2 AND status = $3
        "#,
    )
    .bind(STATUS_ACTIVE)
    .bind(key_id)
    .bind(STATUS_PENDING)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Increment usage count and update last_used timestamp
pub async fn increment_usage(pool: &PgPool, key_id: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE registry_signing_keys
        SET usage_count = usage_count + 1, last_used = NOW()
        WHERE key_id = $1
        "#,
    )
    .bind(key_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Retire a signing key
pub async fn retire_signing_key(pool: &PgPool, key_id: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE registry_signing_keys
        SET status = $1, retired_at = NOW()
        WHERE key_id = $2 AND status != $1
        "#,
    )
    .bind(STATUS_RETIRED)
    .bind(key_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Create or update a Vault-backed signing key
///
/// This upserts a signing key record for a key stored in Vault Transit.
/// The key_id should match the Vault key name for tracking purposes.
pub async fn upsert_vault_signing_key(
    pool: &PgPool,
    vault_key_name: &str,
    ed25519_pubkey: &[u8],
    ed25519_fingerprint: &str,
    mldsa_pubkey: &[u8],
    mldsa_fingerprint: &str,
) -> Result<String> {
    // Use vault key name as the key_id for easier tracking
    let key_id = format!("vault:{}", vault_key_name);

    // Storage mode 3 = VAULT (matching proto SigningKeyStorage)
    const STORAGE_VAULT: i32 = 3;

    sqlx::query(
        r#"
        INSERT INTO registry_signing_keys (
            key_id, storage_mode, ed25519_public_key, ed25519_fingerprint,
            mldsa65_public_key, mldsa65_fingerprint, status,
            hsm_slot_id, hsm_label
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (key_id) DO UPDATE SET
            ed25519_public_key = EXCLUDED.ed25519_public_key,
            ed25519_fingerprint = EXCLUDED.ed25519_fingerprint,
            mldsa65_public_key = EXCLUDED.mldsa65_public_key,
            mldsa65_fingerprint = EXCLUDED.mldsa65_fingerprint
        "#,
    )
    .bind(&key_id)
    .bind(STORAGE_VAULT)
    .bind(ed25519_pubkey)
    .bind(ed25519_fingerprint)
    .bind(mldsa_pubkey)
    .bind(mldsa_fingerprint)
    .bind(STATUS_PENDING)
    .bind(Some("vault-transit"))
    .bind(Some(vault_key_name))
    .execute(pool)
    .await?;

    Ok(key_id)
}
