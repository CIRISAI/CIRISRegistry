//! OAuth identity management for multi-provider login
//!
//! Allows users to log in with multiple OAuth providers (Google, Apple, etc.)
//! while maintaining a single account per email address.

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;

/// OAuth identity linked to a system user
#[derive(Debug, Clone, FromRow)]
pub struct SystemUserOAuthIdentity {
    pub user_id: String,
    pub oauth_provider: String,
    pub oauth_subject: String,
    pub email_at_link: Option<String>,
    pub created_at: OffsetDateTime,
}

/// OAuth identity linked to a regular user
#[derive(Debug, Clone, FromRow)]
pub struct UserOAuthIdentity {
    pub user_id: String,
    pub oauth_provider: String,
    pub oauth_subject: String,
    pub email_at_link: Option<String>,
    pub created_at: OffsetDateTime,
}

// =============================================================================
// System User OAuth Identities
// =============================================================================

/// Look up a system user by OAuth provider and subject
pub async fn get_system_user_by_oauth(
    pool: &PgPool,
    oauth_provider: &str,
    oauth_subject: &str,
) -> Result<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT user_id FROM system_user_oauth_identities
        WHERE oauth_provider = $1 AND oauth_subject = $2
        "#,
    )
    .bind(oauth_provider)
    .bind(oauth_subject)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(id,)| id))
}

/// Link an OAuth identity to an existing system user
pub async fn link_system_user_oauth(
    pool: &PgPool,
    user_id: &str,
    oauth_provider: &str,
    oauth_subject: &str,
    email: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO system_user_oauth_identities (user_id, oauth_provider, oauth_subject, email_at_link)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (oauth_provider, oauth_subject) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(oauth_provider)
    .bind(oauth_subject)
    .bind(email)
    .execute(pool)
    .await?;

    Ok(())
}

/// List all OAuth identities for a system user
pub async fn list_system_user_oauth_identities(
    pool: &PgPool,
    user_id: &str,
) -> Result<Vec<SystemUserOAuthIdentity>> {
    let rows = sqlx::query_as::<_, SystemUserOAuthIdentity>(
        r#"
        SELECT user_id, oauth_provider, oauth_subject, email_at_link, created_at
        FROM system_user_oauth_identities
        WHERE user_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

// =============================================================================
// Regular User OAuth Identities
// =============================================================================

/// Look up a user by OAuth provider and subject
pub async fn get_user_by_oauth(
    pool: &PgPool,
    oauth_provider: &str,
    oauth_subject: &str,
) -> Result<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT user_id FROM user_oauth_identities
        WHERE oauth_provider = $1 AND oauth_subject = $2
        "#,
    )
    .bind(oauth_provider)
    .bind(oauth_subject)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(id,)| id))
}

/// Link an OAuth identity to an existing user
pub async fn link_user_oauth(
    pool: &PgPool,
    user_id: &str,
    oauth_provider: &str,
    oauth_subject: &str,
    email: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO user_oauth_identities (user_id, oauth_provider, oauth_subject, email_at_link)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (oauth_provider, oauth_subject) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(oauth_provider)
    .bind(oauth_subject)
    .bind(email)
    .execute(pool)
    .await?;

    Ok(())
}

/// List all OAuth identities for a user
pub async fn list_user_oauth_identities(
    pool: &PgPool,
    user_id: &str,
) -> Result<Vec<UserOAuthIdentity>> {
    let rows = sqlx::query_as::<_, UserOAuthIdentity>(
        r#"
        SELECT user_id, oauth_provider, oauth_subject, email_at_link, created_at
        FROM user_oauth_identities
        WHERE user_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

// =============================================================================
// Combined Login Flow Helper
// =============================================================================

/// Result of OAuth login lookup
#[derive(Debug)]
pub enum OAuthLookupResult {
    /// Found existing user via OAuth identity
    FoundByOAuth(String),
    /// Found existing user via email (new OAuth identity should be linked)
    FoundByEmail(String),
    /// No user found (new user should be created)
    NotFound,
}

/// Look up a system user by OAuth credentials, falling back to email
pub async fn lookup_system_user_for_login(
    pool: &PgPool,
    oauth_provider: &str,
    oauth_subject: &str,
    email: &str,
) -> Result<OAuthLookupResult> {
    // First, try to find by OAuth identity
    if let Some(user_id) = get_system_user_by_oauth(pool, oauth_provider, oauth_subject).await? {
        return Ok(OAuthLookupResult::FoundByOAuth(user_id));
    }

    // If not found, try to find by email
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT user_id FROM system_users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    match row {
        Some((user_id,)) => Ok(OAuthLookupResult::FoundByEmail(user_id)),
        None => Ok(OAuthLookupResult::NotFound),
    }
}

/// Look up a regular user by OAuth credentials, falling back to email
pub async fn lookup_user_for_login(
    pool: &PgPool,
    oauth_provider: &str,
    oauth_subject: &str,
    email: &str,
) -> Result<OAuthLookupResult> {
    // First, try to find by OAuth identity
    if let Some(user_id) = get_user_by_oauth(pool, oauth_provider, oauth_subject).await? {
        return Ok(OAuthLookupResult::FoundByOAuth(user_id));
    }

    // If not found, try to find by email
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT user_id FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    match row {
        Some((user_id,)) => Ok(OAuthLookupResult::FoundByEmail(user_id)),
        None => Ok(OAuthLookupResult::NotFound),
    }
}
