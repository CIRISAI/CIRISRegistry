//! Key escrow database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct KeyEscrowRow {
    pub escrow_id: String,
    pub key_id: String,
    pub org_id: String,
    pub escrow_type: i32,
    pub custodian: String,
    pub created_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
    pub status: String,
}

impl KeyEscrowRow {
    pub fn to_proto(&self) -> proto::KeyEscrow {
        proto::KeyEscrow {
            escrow_id: self.escrow_id.clone(),
            key_id: self.key_id.clone(),
            org_id: self.org_id.clone(),
            escrow_type: self.escrow_type,
            custodian: self.custodian.clone(),
            created_at: self.created_at.unix_timestamp(),
            expires_at: self.expires_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            status: self.status.clone(),
        }
    }
}

/// Create a new key escrow entry
pub async fn create_escrow(
    pool: &PgPool,
    key_id: &str,
    org_id: &str,
    escrow_type: i32,
    custodian: &str,
) -> Result<String> {
    let escrow_id = uuid::Uuid::new_v4().to_string();

    // Default expiration is 1 year from creation
    sqlx::query(
        r#"
        INSERT INTO key_escrows (escrow_id, key_id, org_id, escrow_type, custodian, expires_at, status)
        VALUES ($1, $2, $3, $4, $5, NOW() + interval '1 year', 'ACTIVE')
        "#,
    )
    .bind(&escrow_id)
    .bind(key_id)
    .bind(org_id)
    .bind(escrow_type)
    .bind(custodian)
    .execute(pool)
    .await?;

    // Update the key's escrow_id reference
    sqlx::query(
        r#"
        UPDATE partner_keys
        SET escrow_id = $1
        WHERE key_id = $2
        "#,
    )
    .bind(&escrow_id)
    .bind(key_id)
    .execute(pool)
    .await?;

    Ok(escrow_id)
}

/// List all escrows for an organization
pub async fn list_escrows(pool: &PgPool, org_id: &str) -> Result<Vec<KeyEscrowRow>> {
    let rows = sqlx::query_as::<_, KeyEscrowRow>(
        r#"
        SELECT escrow_id, key_id, org_id, escrow_type, custodian, created_at, expires_at, status
        FROM key_escrows
        WHERE org_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(org_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get a specific escrow by ID
pub async fn get_escrow(pool: &PgPool, escrow_id: &str) -> Result<Option<KeyEscrowRow>> {
    let row = sqlx::query_as::<_, KeyEscrowRow>(
        r#"
        SELECT escrow_id, key_id, org_id, escrow_type, custodian, created_at, expires_at, status
        FROM key_escrows
        WHERE escrow_id = $1
        "#,
    )
    .bind(escrow_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Update escrow status
pub async fn update_escrow_status(pool: &PgPool, escrow_id: &str, status: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE key_escrows
        SET status = $1
        WHERE escrow_id = $2
        "#,
    )
    .bind(status)
    .bind(escrow_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Get escrow for a specific key
pub async fn get_escrow_for_key(pool: &PgPool, key_id: &str) -> Result<Option<KeyEscrowRow>> {
    let row = sqlx::query_as::<_, KeyEscrowRow>(
        r#"
        SELECT escrow_id, key_id, org_id, escrow_type, custodian, created_at, expires_at, status
        FROM key_escrows
        WHERE key_id = $1 AND status = 'ACTIVE'
        "#,
    )
    .bind(key_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Cleanup expired escrows
pub async fn cleanup_expired_escrows(pool: &PgPool) -> Result<i32> {
    let result = sqlx::query(
        r#"
        UPDATE key_escrows
        SET status = 'EXPIRED'
        WHERE status = 'ACTIVE' AND expires_at < NOW()
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i32)
}
