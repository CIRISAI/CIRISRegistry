//! Emergency shutdown status database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct EmergencyStatusRow {
    pub id: i32,
    pub is_locked: bool,
    pub locked_at: Option<OffsetDateTime>,
    pub locked_until: Option<OffsetDateTime>,
    pub lock_reason: Option<String>,
    pub severity: Option<i32>,
    pub allowed_operations: Vec<String>,
    pub locked_by: Option<String>,
}

impl EmergencyStatusRow {
    pub fn to_proto(&self) -> proto::EmergencyStatusResponse {
        proto::EmergencyStatusResponse {
            is_locked: self.is_locked,
            lock_reason: self.lock_reason.clone().unwrap_or_default(),
            locked_at: self.locked_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            locked_until: self.locked_until.map(|t| t.unix_timestamp()).unwrap_or(0),
            severity: self.severity.unwrap_or(0),
            allowed_operations: self.allowed_operations.clone(),
            context: None,
        }
    }
}

/// Get current emergency status
pub async fn get_emergency_status(pool: &PgPool) -> Result<EmergencyStatusRow> {
    // Ensure there's always a row (upsert with default values)
    sqlx::query(
        r#"
        INSERT INTO emergency_status (id, is_locked, allowed_operations)
        VALUES (1, false, ARRAY[]::text[])
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    let row = sqlx::query_as::<_, EmergencyStatusRow>(
        r#"
        SELECT id, is_locked, locked_at, locked_until, lock_reason, severity, allowed_operations, locked_by
        FROM emergency_status
        WHERE id = 1
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

/// Set emergency shutdown
pub async fn set_emergency_shutdown(
    pool: &PgPool,
    reason: &str,
    severity: i32,
    duration_seconds: i64,
    allowed_operations: &[String],
    locked_by: &str,
) -> Result<()> {
    let duration_interval = format!("{} seconds", duration_seconds);

    sqlx::query(
        r#"
        INSERT INTO emergency_status (id, is_locked, locked_at, locked_until, lock_reason, severity, allowed_operations, locked_by)
        VALUES (1, true, NOW(), NOW() + $1::interval, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE SET
            is_locked = true,
            locked_at = NOW(),
            locked_until = NOW() + $1::interval,
            lock_reason = $2,
            severity = $3,
            allowed_operations = $4,
            locked_by = $5
        "#,
    )
    .bind(&duration_interval)
    .bind(reason)
    .bind(severity)
    .bind(allowed_operations)
    .bind(locked_by)
    .execute(pool)
    .await?;

    Ok(())
}

/// Clear emergency shutdown
pub async fn clear_emergency_shutdown(pool: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE emergency_status
        SET is_locked = false, locked_at = NULL, locked_until = NULL,
            lock_reason = NULL, severity = NULL, allowed_operations = ARRAY[]::text[],
            locked_by = NULL
        WHERE id = 1
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Check if an operation is allowed during emergency shutdown
pub async fn is_operation_allowed(pool: &PgPool, operation: &str) -> Result<bool> {
    let status = get_emergency_status(pool).await?;

    if !status.is_locked {
        return Ok(true);
    }

    // Check if locked_until has passed
    if let Some(until) = status.locked_until {
        if until < OffsetDateTime::now_utc() {
            // Lock has expired, clear it
            clear_emergency_shutdown(pool).await?;
            return Ok(true);
        }
    }

    // Check if operation is in allowed list
    Ok(status.allowed_operations.contains(&operation.to_string()))
}
