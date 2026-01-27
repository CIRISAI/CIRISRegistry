//! Webhook configuration database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct WebhookConfigRow {
    pub webhook_id: String,
    pub org_id: String,
    pub url: String,
    pub subscribed_events: Vec<String>,
    pub signing_secret: String,
    pub active: bool,
    pub created_at: OffsetDateTime,
    pub last_triggered_at: Option<OffsetDateTime>,
    pub consecutive_failures: i32,
}

impl WebhookConfigRow {
    pub fn to_proto(&self) -> proto::WebhookConfig {
        proto::WebhookConfig {
            webhook_id: self.webhook_id.clone(),
            url: self.url.clone(),
            subscribed_events: self.subscribed_events.clone(),
            signing_secret: self.signing_secret.clone(),
            active: self.active,
            created_at: self.created_at.unix_timestamp(),
            last_triggered_at: self.last_triggered_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            consecutive_failures: self.consecutive_failures,
        }
    }
}

/// Register a new webhook
pub async fn register_webhook(
    pool: &PgPool,
    org_id: &str,
    url: &str,
    subscribed_events: &[String],
) -> Result<(String, String)> {
    let webhook_id = uuid::Uuid::new_v4().to_string();
    // Generate a secure signing secret
    let signing_secret = format!("whsec_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

    sqlx::query(
        r#"
        INSERT INTO webhooks (webhook_id, org_id, url, subscribed_events, signing_secret, active)
        VALUES ($1, $2, $3, $4, $5, true)
        "#,
    )
    .bind(&webhook_id)
    .bind(org_id)
    .bind(url)
    .bind(subscribed_events)
    .bind(&signing_secret)
    .execute(pool)
    .await?;

    Ok((webhook_id, signing_secret))
}

/// List all webhooks for an organization
pub async fn list_webhooks(pool: &PgPool, org_id: &str) -> Result<Vec<WebhookConfigRow>> {
    let rows = sqlx::query_as::<_, WebhookConfigRow>(
        r#"
        SELECT webhook_id, org_id, url, subscribed_events, signing_secret, active,
               created_at, last_triggered_at, consecutive_failures
        FROM webhooks
        WHERE org_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(org_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get a specific webhook by ID
pub async fn get_webhook(pool: &PgPool, webhook_id: &str) -> Result<Option<WebhookConfigRow>> {
    let row = sqlx::query_as::<_, WebhookConfigRow>(
        r#"
        SELECT webhook_id, org_id, url, subscribed_events, signing_secret, active,
               created_at, last_triggered_at, consecutive_failures
        FROM webhooks
        WHERE webhook_id = $1
        "#,
    )
    .bind(webhook_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Delete a webhook (soft delete by deactivating)
pub async fn delete_webhook(pool: &PgPool, webhook_id: &str, org_id: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE webhooks
        SET active = false
        WHERE webhook_id = $1 AND org_id = $2
        "#,
    )
    .bind(webhook_id)
    .bind(org_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Update webhook after a delivery attempt
pub async fn update_webhook_delivery(
    pool: &PgPool,
    webhook_id: &str,
    success: bool,
) -> Result<()> {
    if success {
        sqlx::query(
            r#"
            UPDATE webhooks
            SET last_triggered_at = NOW(), consecutive_failures = 0
            WHERE webhook_id = $1
            "#,
        )
        .bind(webhook_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE webhooks
            SET last_triggered_at = NOW(), consecutive_failures = consecutive_failures + 1
            WHERE webhook_id = $1
            "#,
        )
        .bind(webhook_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Get webhooks subscribed to a specific event type
pub async fn get_webhooks_for_event(
    pool: &PgPool,
    org_id: &str,
    event_type: &str,
) -> Result<Vec<WebhookConfigRow>> {
    let rows = sqlx::query_as::<_, WebhookConfigRow>(
        r#"
        SELECT webhook_id, org_id, url, subscribed_events, signing_secret, active,
               created_at, last_triggered_at, consecutive_failures
        FROM webhooks
        WHERE org_id = $1 AND active = true AND $2 = ANY(subscribed_events)
        "#,
    )
    .bind(org_id)
    .bind(event_type)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Disable webhooks with too many consecutive failures
pub async fn disable_failing_webhooks(pool: &PgPool, max_failures: i32) -> Result<i32> {
    let result = sqlx::query(
        r#"
        UPDATE webhooks
        SET active = false
        WHERE active = true AND consecutive_failures >= $1
        "#,
    )
    .bind(max_failures)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i32)
}
