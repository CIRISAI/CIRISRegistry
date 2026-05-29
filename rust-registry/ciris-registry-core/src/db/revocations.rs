//! Revocation management database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct RevocationEntryRow {
    pub id: i32,
    pub target_type: i32,
    pub target_id: String,
    pub revoked_at: OffsetDateTime,
    pub reason_code: i32,
    pub reason_detail: Option<String>,
    pub severity: i32,
    pub authority_signature: Option<Vec<u8>>,
}

impl RevocationEntryRow {
    pub fn to_proto(&self) -> proto::RevocationEntry {
        proto::RevocationEntry {
            target_type: self.target_type,
            target_id: self.target_id.clone(),
            revoked_at: self.revoked_at.unix_timestamp(),
            reason_code: self.reason_code,
            reason_detail: self.reason_detail.clone().unwrap_or_default(),
            severity: self.severity,
            authority_signature: None, // Would need to deserialize HybridSignature if stored
        }
    }
}

// Target type constants matching proto RevocationType enum
pub const TARGET_TYPE_AGENT: i32 = 1;
pub const TARGET_TYPE_PARTNER: i32 = 2;
pub const TARGET_TYPE_KEY: i32 = 3;

/// Create a single revocation entry
pub async fn create_revocation(
    pool: &PgPool,
    target_type: i32,
    target_id: &str,
    reason_code: i32,
    reason_detail: Option<&str>,
    severity: i32,
    authority_signature: Option<&[u8]>,
) -> Result<i32> {
    let row: (i32,) = sqlx::query_as(
        r#"
        INSERT INTO revocations (target_type, target_id, reason_code, reason_detail, severity, authority_signature)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(target_type)
    .bind(target_id)
    .bind(reason_code)
    .bind(reason_detail)
    .bind(severity)
    .bind(authority_signature)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Mass revoke agents by their hashes
pub async fn mass_revoke_agents(
    pool: &PgPool,
    agent_hashes: &[Vec<u8>],
    reason_code: i32,
    reason_detail: &str,
) -> Result<i32> {
    if agent_hashes.is_empty() {
        return Ok(0);
    }

    let mut tx = pool.begin().await?;
    let mut count = 0;

    for hash in agent_hashes {
        // Update agent status
        let result = sqlx::query(
            r#"
            UPDATE agents
            SET status = $1, revocation_reason = $2, revocation_timestamp = NOW()
            WHERE agent_hash = $3 AND status != $1
            "#,
        )
        .bind(proto::AgentStatus::AgentRevoked as i32)
        .bind(reason_detail)
        .bind(hash)
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() > 0 {
            // Create revocation entry
            sqlx::query(
                r#"
                INSERT INTO revocations (target_type, target_id, reason_code, reason_detail, severity)
                VALUES ($1, encode($2, 'hex'), $3, $4, 3)
                "#,
            )
            .bind(TARGET_TYPE_AGENT)
            .bind(hash)
            .bind(reason_code)
            .bind(reason_detail)
            .execute(&mut *tx)
            .await?;

            count += 1;
        }
    }

    tx.commit().await?;
    Ok(count)
}

/// Mass revoke agents by version prefix (e.g., "1.2." revokes all 1.2.x)
pub async fn mass_revoke_by_version_prefix(
    pool: &PgPool,
    version_prefix: &str,
    reason_code: i32,
    reason_detail: &str,
) -> Result<i32> {
    let mut tx = pool.begin().await?;

    // Get matching agent hashes
    let hashes: Vec<(Vec<u8>,)> = sqlx::query_as(
        r#"
        SELECT agent_hash FROM agents
        WHERE CONCAT(version_major, '.', version_minor, '.', version_patch) LIKE $1
          AND status != $2
        "#,
    )
    .bind(format!("{}%", version_prefix))
    .bind(proto::AgentStatus::AgentRevoked as i32)
    .fetch_all(&mut *tx)
    .await?;

    // Update agents
    let result = sqlx::query(
        r#"
        UPDATE agents
        SET status = $1, revocation_reason = $2, revocation_timestamp = NOW()
        WHERE CONCAT(version_major, '.', version_minor, '.', version_patch) LIKE $3
          AND status != $1
        "#,
    )
    .bind(proto::AgentStatus::AgentRevoked as i32)
    .bind(reason_detail)
    .bind(format!("{}%", version_prefix))
    .execute(&mut *tx)
    .await?;

    let count = result.rows_affected() as i32;

    // Create revocation entries
    for (hash,) in hashes {
        sqlx::query(
            r#"
            INSERT INTO revocations (target_type, target_id, reason_code, reason_detail, severity)
            VALUES ($1, encode($2, 'hex'), $3, $4, 3)
            "#,
        )
        .bind(TARGET_TYPE_AGENT)
        .bind(&hash)
        .bind(reason_code)
        .bind(reason_detail)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(count)
}

/// Mass revoke partners by their IDs
pub async fn mass_revoke_partners(
    pool: &PgPool,
    partner_ids: &[String],
    reason_code: i32,
    reason_detail: &str,
) -> Result<i32> {
    if partner_ids.is_empty() {
        return Ok(0);
    }

    let mut tx = pool.begin().await?;

    // Update partners
    let result = sqlx::query(
        r#"
        UPDATE partners
        SET status = $1, revocation_reason = $2
        WHERE partner_id = ANY($3) AND status != $1
        "#,
    )
    .bind(proto::PartnerStatus::PartnerRevoked as i32)
    .bind(reason_detail)
    .bind(partner_ids)
    .execute(&mut *tx)
    .await?;

    let count = result.rows_affected() as i32;

    // Create revocation entries
    for partner_id in partner_ids {
        sqlx::query(
            r#"
            INSERT INTO revocations (target_type, target_id, reason_code, reason_detail, severity)
            VALUES ($1, $2, $3, $4, 3)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(TARGET_TYPE_PARTNER)
        .bind(partner_id)
        .bind(reason_code)
        .bind(reason_detail)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(count)
}

/// Get the revocation list, optionally filtered to entries since a specific version
pub async fn get_revocation_list(
    pool: &PgPool,
    since_id: Option<i32>,
) -> Result<(Vec<RevocationEntryRow>, i32)> {
    let rows = if let Some(since) = since_id {
        sqlx::query_as::<_, RevocationEntryRow>(
            r#"
            SELECT id, target_type, target_id, revoked_at, reason_code, reason_detail, severity, authority_signature
            FROM revocations
            WHERE id > $1
            ORDER BY id ASC
            "#,
        )
        .bind(since)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, RevocationEntryRow>(
            r#"
            SELECT id, target_type, target_id, revoked_at, reason_code, reason_detail, severity, authority_signature
            FROM revocations
            ORDER BY id ASC
            "#,
        )
        .fetch_all(pool)
        .await?
    };

    // Get the latest version (max id)
    let max_id: (Option<i32>,) = sqlx::query_as("SELECT MAX(id) FROM revocations")
        .fetch_one(pool)
        .await?;

    Ok((rows, max_id.0.unwrap_or(0)))
}

/// Count agents matching a version prefix (for dry-run)
pub async fn count_agents_by_version_prefix(pool: &PgPool, version_prefix: &str) -> Result<i32> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM agents
        WHERE CONCAT(version_major, '.', version_minor, '.', version_patch) LIKE $1
          AND status != $2
        "#,
    )
    .bind(format!("{}%", version_prefix))
    .bind(proto::AgentStatus::AgentRevoked as i32)
    .fetch_one(pool)
    .await?;

    Ok(count.0 as i32)
}
