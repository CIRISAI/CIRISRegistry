//! Key management database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct PartnerKeyRow {
    pub key_id: String,
    pub org_id: String,
    pub partner_id: Option<String>,
    pub ed25519_public_key: Vec<u8>,
    pub ml_dsa_65_public_key: Vec<u8>,
    pub ed25519_fingerprint: String,
    pub ml_dsa_65_fingerprint: String,
    pub custody_model: i32,
    pub kv_key_ref: Option<String>,
    pub status: i32,
    pub revocation_reason: Option<String>,
    pub created_at: OffsetDateTime,
    pub activated_at: Option<OffsetDateTime>,
    pub rotated_at: Option<OffsetDateTime>,
    pub revoked_at: Option<OffsetDateTime>,
    pub grace_period_expires_at: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    pub rotated_by: Option<String>,
    pub revoked_by: Option<String>,
    pub escrow_id: Option<String>,
}

impl PartnerKeyRow {
    pub fn to_proto(&self) -> proto::PartnerKeyRecord {
        proto::PartnerKeyRecord {
            key_id: self.key_id.clone(),
            org_id: self.org_id.clone(),
            partner_id: self.partner_id.clone().unwrap_or_default(),
            public_keys: Some(proto::PublicKeys {
                ed25519_public_key: self.ed25519_public_key.clone().into(),
                ml_dsa_65_public_key: self.ml_dsa_65_public_key.clone().into(),
            }),
            ed25519_fingerprint: self.ed25519_fingerprint.clone(),
            ml_dsa_65_fingerprint: self.ml_dsa_65_fingerprint.clone(),
            custody_model: self.custody_model,
            kv_key_ref: self.kv_key_ref.clone().unwrap_or_default(),
            status: self.status,
            revocation_reason: self.revocation_reason.clone().unwrap_or_default(),
            created_at: self.created_at.unix_timestamp(),
            activated_at: self.activated_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            rotated_at: self.rotated_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            revoked_at: self.revoked_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            grace_period_expires_at: self
                .grace_period_expires_at
                .map(|t| t.unix_timestamp())
                .unwrap_or(0),
            created_by: self.created_by.clone().unwrap_or_default(),
            rotated_by: self.rotated_by.clone().unwrap_or_default(),
            revoked_by: self.revoked_by.clone().unwrap_or_default(),
            registry_signature: None,
            escrow_id: self.escrow_id.clone().unwrap_or_default(),
        }
    }
}

pub async fn get_active_key(pool: &PgPool, org_id: &str) -> Result<Option<PartnerKeyRow>> {
    let row = sqlx::query_as::<_, PartnerKeyRow>(
        r#"
        SELECT key_id, org_id, partner_id, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model, kv_key_ref,
               status, revocation_reason, created_at, activated_at, rotated_at,
               revoked_at, grace_period_expires_at, created_by, rotated_by, revoked_by, escrow_id
        FROM partner_keys
        WHERE org_id = $1 AND status = $2
        ORDER BY activated_at DESC
        LIMIT 1
        "#,
    )
    .bind(org_id)
    .bind(proto::KeyStatus::KeyActive as i32)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_key(pool: &PgPool, key_id: &str) -> Result<Option<PartnerKeyRow>> {
    let row = sqlx::query_as::<_, PartnerKeyRow>(
        r#"
        SELECT key_id, org_id, partner_id, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model, kv_key_ref,
               status, revocation_reason, created_at, activated_at, rotated_at,
               revoked_at, grace_period_expires_at, created_by, rotated_by, revoked_by, escrow_id
        FROM partner_keys
        WHERE key_id = $1
        "#,
    )
    .bind(key_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_key_by_fingerprint(
    pool: &PgPool,
    fingerprint: &str,
) -> Result<Option<PartnerKeyRow>> {
    let row = sqlx::query_as::<_, PartnerKeyRow>(
        r#"
        SELECT key_id, org_id, partner_id, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model, kv_key_ref,
               status, revocation_reason, created_at, activated_at, rotated_at,
               revoked_at, grace_period_expires_at, created_by, rotated_by, revoked_by, escrow_id
        FROM partner_keys
        WHERE ed25519_fingerprint = $1 AND status = $2
        ORDER BY activated_at DESC
        LIMIT 1
        "#,
    )
    .bind(fingerprint)
    .bind(proto::KeyStatus::KeyActive as i32)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Lookup key by Ed25519 fingerprint (any status - for CIRISVerify validation)
pub async fn lookup_key_by_fingerprint(
    pool: &PgPool,
    fingerprint: &str,
) -> Result<Option<PartnerKeyRow>> {
    let row = sqlx::query_as::<_, PartnerKeyRow>(
        r#"
        SELECT key_id, org_id, partner_id, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model, kv_key_ref,
               status, revocation_reason, created_at, activated_at, rotated_at,
               revoked_at, grace_period_expires_at, created_by, rotated_by, revoked_by, escrow_id
        FROM partner_keys
        WHERE ed25519_fingerprint = $1
        ORDER BY activated_at DESC NULLS LAST, created_at DESC
        LIMIT 1
        "#,
    )
    .bind(fingerprint)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn create_key(
    pool: &PgPool,
    org_id: &str,
    ed25519_pubkey: &[u8],
    mldsa_pubkey: &[u8],
    ed25519_fp: &str,
    mldsa_fp: &str,
    custody_model: i32,
    created_by: &str,
) -> Result<String> {
    let key_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO partner_keys (
            key_id, org_id, ed25519_public_key, ml_dsa_65_public_key,
            ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model,
            status, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(&key_id)
    .bind(org_id)
    .bind(ed25519_pubkey)
    .bind(mldsa_pubkey)
    .bind(ed25519_fp)
    .bind(mldsa_fp)
    .bind(custody_model)
    .bind(proto::KeyStatus::KeyPending as i32)
    .bind(created_by)
    .execute(pool)
    .await?;

    Ok(key_id)
}

pub async fn activate_key(pool: &PgPool, key_id: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE partner_keys
        SET status = $1, activated_at = NOW()
        WHERE key_id = $2 AND status = $3
        "#,
    )
    .bind(proto::KeyStatus::KeyActive as i32)
    .bind(key_id)
    .bind(proto::KeyStatus::KeyPending as i32)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Revoke a key
pub async fn revoke_key(
    pool: &PgPool,
    key_id: &str,
    reason: &str,
    revoked_by: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE partner_keys
        SET status = $1, revocation_reason = $2, revoked_at = NOW(), revoked_by = $3
        WHERE key_id = $4 AND status != $1
        "#,
    )
    .bind(proto::KeyStatus::KeyRevoked as i32)
    .bind(reason)
    .bind(revoked_by)
    .bind(key_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Rotate a key - creates new key and optionally marks old as rotated
pub async fn rotate_key(
    pool: &PgPool,
    old_key_id: &str,
    new_key_id: &str,
    org_id: &str,
    ed25519_pubkey: &[u8],
    mldsa_pubkey: &[u8],
    ed25519_fp: &str,
    mldsa_fp: &str,
    custody_model: i32,
    rotated_by: &str,
    grace_period_hours: i32,
    immediate: bool,
) -> Result<()> {
    // Start transaction
    let mut tx = pool.begin().await?;

    // Create new key
    sqlx::query(
        r#"
        INSERT INTO partner_keys (
            key_id, org_id, ed25519_public_key, ml_dsa_65_public_key,
            ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model,
            status, created_by, activated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CASE WHEN $10 THEN NOW() ELSE NULL END)
        "#,
    )
    .bind(new_key_id)
    .bind(org_id)
    .bind(ed25519_pubkey)
    .bind(mldsa_pubkey)
    .bind(ed25519_fp)
    .bind(mldsa_fp)
    .bind(custody_model)
    .bind(if immediate {
        proto::KeyStatus::KeyActive as i32
    } else {
        proto::KeyStatus::KeyPending as i32
    })
    .bind(rotated_by)
    .bind(immediate)
    .execute(&mut *tx)
    .await?;

    // Update old key status
    let grace_interval = format!("{} hours", grace_period_hours);
    if immediate {
        // Mark old key as rotated immediately
        sqlx::query(
            r#"
            UPDATE partner_keys
            SET status = $1, rotated_at = NOW(), rotated_by = $2,
                grace_period_expires_at = CASE WHEN $3 > 0 THEN NOW() + $4::interval ELSE NULL END
            WHERE key_id = $5 AND status = $6
            "#,
        )
        .bind(proto::KeyStatus::KeyRotated as i32)
        .bind(rotated_by)
        .bind(grace_period_hours)
        .bind(&grace_interval)
        .bind(old_key_id)
        .bind(proto::KeyStatus::KeyActive as i32)
        .execute(&mut *tx)
        .await?;
    } else {
        // Staged rotation - old key stays active, new key is pending
        sqlx::query(
            r#"
            UPDATE partner_keys
            SET rotated_by = $1, grace_period_expires_at = NOW() + $2::interval
            WHERE key_id = $3 AND status = $4
            "#,
        )
        .bind(rotated_by)
        .bind(&grace_interval)
        .bind(old_key_id)
        .bind(proto::KeyStatus::KeyActive as i32)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// Complete staged rotation - activates new key and marks old as rotated
pub async fn complete_rotation(pool: &PgPool, old_key_id: &str, new_key_id: &str) -> Result<bool> {
    let mut tx = pool.begin().await?;

    // Activate new key
    let activated = sqlx::query(
        r#"
        UPDATE partner_keys
        SET status = $1, activated_at = NOW()
        WHERE key_id = $2 AND status = $3
        "#,
    )
    .bind(proto::KeyStatus::KeyActive as i32)
    .bind(new_key_id)
    .bind(proto::KeyStatus::KeyPending as i32)
    .execute(&mut *tx)
    .await?;

    if activated.rows_affected() == 0 {
        return Ok(false);
    }

    // Mark old key as rotated
    sqlx::query(
        r#"
        UPDATE partner_keys
        SET status = $1, rotated_at = NOW()
        WHERE key_id = $2 AND status = $3
        "#,
    )
    .bind(proto::KeyStatus::KeyRotated as i32)
    .bind(old_key_id)
    .bind(proto::KeyStatus::KeyActive as i32)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(true)
}

pub async fn list_keys(
    pool: &PgPool,
    org_id: &str,
    include_revoked: bool,
) -> Result<Vec<PartnerKeyRow>> {
    let query = if include_revoked {
        r#"
        SELECT key_id, org_id, partner_id, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model, kv_key_ref,
               status, revocation_reason, created_at, activated_at, rotated_at,
               revoked_at, grace_period_expires_at, created_by, rotated_by, revoked_by, escrow_id
        FROM partner_keys
        WHERE org_id = $1
        ORDER BY created_at DESC
        "#
    } else {
        r#"
        SELECT key_id, org_id, partner_id, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model, kv_key_ref,
               status, revocation_reason, created_at, activated_at, rotated_at,
               revoked_at, grace_period_expires_at, created_by, rotated_by, revoked_by, escrow_id
        FROM partner_keys
        WHERE org_id = $1 AND status != $2
        ORDER BY created_at DESC
        "#
    };

    let rows = if include_revoked {
        sqlx::query_as::<_, PartnerKeyRow>(query)
            .bind(org_id)
            .fetch_all(pool)
            .await?
    } else {
        sqlx::query_as::<_, PartnerKeyRow>(query)
            .bind(org_id)
            .bind(proto::KeyStatus::KeyRevoked as i32)
            .fetch_all(pool)
            .await?
    };

    Ok(rows)
}

// =============================================================================
// Self-Custody Key Management (v1.3.0)
// =============================================================================

/// Store a registration challenge for an organization
pub async fn store_registration_challenge(
    pool: &PgPool,
    org_id: &str,
    challenge: &[u8],
    expires_at: i64,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO registration_challenges (org_id, challenge, expires_at)
        VALUES ($1, $2, to_timestamp($3))
        ON CONFLICT (org_id) DO UPDATE
        SET challenge = EXCLUDED.challenge, expires_at = EXCLUDED.expires_at, created_at = NOW()
        "#,
    )
    .bind(org_id)
    .bind(challenge)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get and remove a registration challenge (single-use)
pub async fn get_and_remove_registration_challenge(
    pool: &PgPool,
    org_id: &str,
) -> Result<Option<Vec<u8>>> {
    let row: Option<(Vec<u8>,)> = sqlx::query_as(
        r#"
        DELETE FROM registration_challenges
        WHERE org_id = $1 AND expires_at > NOW()
        RETURNING challenge
        "#,
    )
    .bind(org_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(challenge,)| challenge))
}

/// Store an activation challenge for a key
pub async fn store_activation_challenge(
    pool: &PgPool,
    key_id: &str,
    challenge: &[u8],
    expires_at: i64,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO activation_challenges (key_id, challenge, expires_at)
        VALUES ($1, $2, to_timestamp($3))
        ON CONFLICT (key_id) DO UPDATE
        SET challenge = EXCLUDED.challenge, expires_at = EXCLUDED.expires_at, created_at = NOW()
        "#,
    )
    .bind(key_id)
    .bind(challenge)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get and remove an activation challenge (single-use)
pub async fn get_and_remove_activation_challenge(
    pool: &PgPool,
    key_id: &str,
) -> Result<Option<Vec<u8>>> {
    let row: Option<(Vec<u8>,)> = sqlx::query_as(
        r#"
        DELETE FROM activation_challenges
        WHERE key_id = $1 AND expires_at > NOW()
        RETURNING challenge
        "#,
    )
    .bind(key_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(challenge,)| challenge))
}

/// Check if a public key hash already exists (for duplicate detection)
pub async fn public_key_exists(pool: &PgPool, public_key_hash: &str) -> Result<bool> {
    let row: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT 1 FROM partner_keys WHERE public_key_hash = $1 LIMIT 1
        "#,
    )
    .bind(public_key_hash)
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

/// Create a self-custody key (public key only, SELF_SOVEREIGN custody model)
#[allow(clippy::too_many_arguments)]
pub async fn create_self_custody_key(
    pool: &PgPool,
    org_id: &str,
    ed25519_pubkey: &[u8],
    mldsa_pubkey: &[u8],
    public_key_hash: &str,
    mldsa_fp: &str,
    created_by: &str,
    key_label: Option<&str>,
) -> Result<String> {
    let key_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO partner_keys (
            key_id, org_id, ed25519_public_key, ml_dsa_65_public_key,
            ed25519_fingerprint, ml_dsa_65_fingerprint, custody_model,
            status, created_by, public_key_hash, kv_key_ref
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&key_id)
    .bind(org_id)
    .bind(ed25519_pubkey)
    .bind(mldsa_pubkey)
    .bind(public_key_hash) // Use hash as fingerprint for self-custody
    .bind(mldsa_fp)
    .bind(proto::KeyCustodyModel::SelfSovereign as i32)
    .bind(proto::KeyStatus::KeyPending as i32)
    .bind(created_by)
    .bind(public_key_hash)
    .bind(key_label) // Store label in kv_key_ref field for self-custody keys
    .execute(pool)
    .await?;

    Ok(key_id)
}

/// Mark a key as rotated with grace period
pub async fn mark_key_rotated(pool: &PgPool, key_id: &str, grace_period_hours: i32) -> Result<()> {
    let grace_interval = format!("{} hours", grace_period_hours);

    sqlx::query(
        r#"
        UPDATE partner_keys
        SET status = $1, rotated_at = NOW(),
            grace_period_expires_at = NOW() + $2::interval
        WHERE key_id = $3 AND status = $4
        "#,
    )
    .bind(proto::KeyStatus::KeyRotated as i32)
    .bind(&grace_interval)
    .bind(key_id)
    .bind(proto::KeyStatus::KeyActive as i32)
    .execute(pool)
    .await?;

    Ok(())
}

/// Cleanup expired challenges (should be run periodically)
pub async fn cleanup_expired_challenges(pool: &PgPool) -> Result<(i64, i64)> {
    let reg_deleted = sqlx::query("DELETE FROM registration_challenges WHERE expires_at < NOW()")
        .execute(pool)
        .await?
        .rows_affected();

    let act_deleted = sqlx::query("DELETE FROM activation_challenges WHERE expires_at < NOW()")
        .execute(pool)
        .await?
        .rows_affected();

    Ok((reg_deleted as i64, act_deleted as i64))
}
