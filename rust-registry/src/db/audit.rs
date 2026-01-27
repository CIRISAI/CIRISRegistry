//! Audit log database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct AuditEntryRow {
    pub entry_id: String,
    pub timestamp: OffsetDateTime,
    pub actor_user_id: Option<String>,
    pub actor_org_id: Option<String>,
    pub actor_ip_address: Option<String>,
    pub actor_user_agent: Option<String>,
    pub action: i32,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl AuditEntryRow {
    pub fn to_proto(&self) -> proto::AuditEntry {
        proto::AuditEntry {
            entry_id: self.entry_id.clone(),
            timestamp: self.timestamp.unix_timestamp(),
            actor_user_id: self.actor_user_id.clone().unwrap_or_default(),
            actor_org_id: self.actor_org_id.clone().unwrap_or_default(),
            actor_ip_address: self.actor_ip_address.clone().unwrap_or_default(),
            actor_user_agent: self.actor_user_agent.clone().unwrap_or_default(),
            action: self.action,
            target_type: self.target_type.clone().unwrap_or_default(),
            target_id: self.target_id.clone().unwrap_or_default(),
            description: self.description.clone().unwrap_or_default(),
            metadata: self
                .metadata
                .as_ref()
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            entry_signature: None,
        }
    }
}

pub async fn create_audit_entry(
    pool: &PgPool,
    action: proto::AuditActionType,
    actor_user_id: Option<&str>,
    actor_org_id: Option<&str>,
    actor_ip: Option<&str>,
    target_type: Option<&str>,
    target_id: Option<&str>,
    description: &str,
    metadata: Option<serde_json::Value>,
) -> Result<String> {
    let entry_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO audit_log (
            entry_id, actor_user_id, actor_org_id, actor_ip_address,
            action, target_type, target_id, description, metadata
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(&entry_id)
    .bind(actor_user_id)
    .bind(actor_org_id)
    .bind(actor_ip)
    .bind(action as i32)
    .bind(target_type)
    .bind(target_id)
    .bind(description)
    .bind(metadata)
    .execute(pool)
    .await?;

    Ok(entry_id)
}

pub async fn get_audit_log(
    pool: &PgPool,
    org_id: &str,
    start_time: i64,
    end_time: i64,
    action_types: &[i32],
    page_size: i32,
    offset: i32,
) -> Result<(Vec<AuditEntryRow>, i32)> {
    let rows = if action_types.is_empty() {
        sqlx::query_as::<_, AuditEntryRow>(
            r#"
            SELECT entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
                   actor_user_agent, action, target_type, target_id, description, metadata
            FROM audit_log
            WHERE actor_org_id = $1
              AND timestamp >= to_timestamp($2)
              AND timestamp <= to_timestamp($3)
            ORDER BY timestamp DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(org_id)
        .bind(start_time as f64)
        .bind(end_time as f64)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, AuditEntryRow>(
            r#"
            SELECT entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
                   actor_user_agent, action, target_type, target_id, description, metadata
            FROM audit_log
            WHERE actor_org_id = $1
              AND timestamp >= to_timestamp($2)
              AND timestamp <= to_timestamp($3)
              AND action = ANY($4)
            ORDER BY timestamp DESC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(org_id)
        .bind(start_time as f64)
        .bind(end_time as f64)
        .bind(action_types)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?
    };

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM audit_log
        WHERE actor_org_id = $1
          AND timestamp >= to_timestamp($2)
          AND timestamp <= to_timestamp($3)
        "#,
    )
    .bind(org_id)
    .bind(start_time as f64)
    .bind(end_time as f64)
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0 as i32))
}

/// Export audit log entries (no pagination limit)
pub async fn export_audit_log(
    pool: &PgPool,
    org_id: &str,
    start_time: i64,
    end_time: i64,
    action_types: &[i32],
    user_ids: &[String],
) -> Result<Vec<AuditEntryRow>> {
    let rows = if action_types.is_empty() && user_ids.is_empty() {
        sqlx::query_as::<_, AuditEntryRow>(
            r#"
            SELECT entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
                   actor_user_agent, action, target_type, target_id, description, metadata
            FROM audit_log
            WHERE actor_org_id = $1
              AND timestamp >= to_timestamp($2)
              AND timestamp <= to_timestamp($3)
            ORDER BY timestamp DESC
            "#,
        )
        .bind(org_id)
        .bind(start_time as f64)
        .bind(end_time as f64)
        .fetch_all(pool)
        .await?
    } else if user_ids.is_empty() {
        sqlx::query_as::<_, AuditEntryRow>(
            r#"
            SELECT entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
                   actor_user_agent, action, target_type, target_id, description, metadata
            FROM audit_log
            WHERE actor_org_id = $1
              AND timestamp >= to_timestamp($2)
              AND timestamp <= to_timestamp($3)
              AND action = ANY($4)
            ORDER BY timestamp DESC
            "#,
        )
        .bind(org_id)
        .bind(start_time as f64)
        .bind(end_time as f64)
        .bind(action_types)
        .fetch_all(pool)
        .await?
    } else if action_types.is_empty() {
        sqlx::query_as::<_, AuditEntryRow>(
            r#"
            SELECT entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
                   actor_user_agent, action, target_type, target_id, description, metadata
            FROM audit_log
            WHERE actor_org_id = $1
              AND timestamp >= to_timestamp($2)
              AND timestamp <= to_timestamp($3)
              AND actor_user_id = ANY($4)
            ORDER BY timestamp DESC
            "#,
        )
        .bind(org_id)
        .bind(start_time as f64)
        .bind(end_time as f64)
        .bind(user_ids)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, AuditEntryRow>(
            r#"
            SELECT entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
                   actor_user_agent, action, target_type, target_id, description, metadata
            FROM audit_log
            WHERE actor_org_id = $1
              AND timestamp >= to_timestamp($2)
              AND timestamp <= to_timestamp($3)
              AND action = ANY($4)
              AND actor_user_id = ANY($5)
            ORDER BY timestamp DESC
            "#,
        )
        .bind(org_id)
        .bind(start_time as f64)
        .bind(end_time as f64)
        .bind(action_types)
        .bind(user_ids)
        .fetch_all(pool)
        .await?
    };

    Ok(rows)
}
