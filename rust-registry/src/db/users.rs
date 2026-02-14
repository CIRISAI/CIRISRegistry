//! User database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct OrgUserRow {
    pub user_id: String,
    pub org_id: String,
    pub email: String,
    pub name: String,
    pub oauth_provider: Option<String>,
    pub oauth_subject: Option<String>,
    pub role: i32,
    pub active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_login_at: Option<OffsetDateTime>,
    pub invited_by: Option<String>,
    pub mfa_enabled: bool,
    pub mfa_method: Option<String>,
}

impl OrgUserRow {
    pub fn to_proto(&self) -> proto::OrgUser {
        use time::format_description::well_known::Rfc3339;

        proto::OrgUser {
            user_id: self.user_id.clone(),
            org_id: self.org_id.clone(),
            email: self.email.clone(),
            name: self.name.clone(),
            oauth_provider: self.oauth_provider.clone().unwrap_or_default(),
            oauth_subject: self.oauth_subject.clone().unwrap_or_default(),
            role: self.role,
            active: self.active,
            created_at: self.created_at.unix_timestamp(),
            updated_at: self.updated_at.unix_timestamp(),
            last_login_at: self.last_login_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            invited_by: self.invited_by.clone().unwrap_or_default(),
            // ISO 8601 timestamp strings for JavaScript compatibility
            created_at_iso: self.created_at.format(&Rfc3339).unwrap_or_default(),
            updated_at_iso: self.updated_at.format(&Rfc3339).unwrap_or_default(),
            last_login_at_iso: self
                .last_login_at
                .map(|t| t.format(&Rfc3339).unwrap_or_default())
                .unwrap_or_default(),
            mfa_enabled: self.mfa_enabled,
            mfa_method: self.mfa_method.clone().unwrap_or_default(),
        }
    }
}

pub async fn get_user(pool: &PgPool, user_id: &str) -> Result<Option<OrgUserRow>> {
    // Query from new tables (users + user_org_memberships) for multi-org support
    // Returns the first org membership found (for backward compatibility)
    let row = sqlx::query_as::<_, OrgUserRow>(
        r#"
        SELECT u.user_id, m.org_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
               m.role, u.active, u.created_at, u.updated_at, u.last_login_at, m.invited_by,
               u.mfa_enabled, u.mfa_method
        FROM users u
        JOIN user_org_memberships m ON u.user_id = m.user_id
        WHERE u.user_id = $1
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<OrgUserRow>> {
    // Query from new tables (users + user_org_memberships)
    let row = sqlx::query_as::<_, OrgUserRow>(
        r#"
        SELECT u.user_id, m.org_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
               m.role, u.active, u.created_at, u.updated_at, u.last_login_at, m.invited_by,
               u.mfa_enabled, u.mfa_method
        FROM users u
        JOIN user_org_memberships m ON u.user_id = m.user_id
        WHERE u.email = $1
        LIMIT 1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn create_user(pool: &PgPool, user: &proto::OrgUser) -> Result<String> {
    let user_id = uuid::Uuid::new_v4().to_string();
    let mut tx = pool.begin().await?;

    // Insert into users table
    sqlx::query(
        r#"
        INSERT INTO users (
            user_id, email, name, oauth_provider, oauth_subject,
            active, mfa_enabled, mfa_method
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&user_id)
    .bind(&user.email)
    .bind(&user.name)
    .bind(if user.oauth_provider.is_empty() {
        None
    } else {
        Some(&user.oauth_provider)
    })
    .bind(if user.oauth_subject.is_empty() {
        None
    } else {
        Some(&user.oauth_subject)
    })
    .bind(user.active)
    .bind(user.mfa_enabled)
    .bind(if user.mfa_method.is_empty() {
        None
    } else {
        Some(&user.mfa_method)
    })
    .execute(&mut *tx)
    .await?;

    // Insert into user_org_memberships table
    sqlx::query(
        r#"
        INSERT INTO user_org_memberships (user_id, org_id, role, invited_by)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&user_id)
    .bind(&user.org_id)
    .bind(user.role)
    .bind(if user.invited_by.is_empty() {
        None
    } else {
        Some(&user.invited_by)
    })
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(user_id)
}

pub async fn list_org_users(
    pool: &PgPool,
    org_id: &str,
    page_size: i32,
    offset: i32,
    include_inactive: bool,
) -> Result<(Vec<OrgUserRow>, i32)> {
    // Query from new tables (users + user_org_memberships)
    let query = if include_inactive {
        r#"
        SELECT u.user_id, m.org_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
               m.role, u.active, u.created_at, u.updated_at, u.last_login_at, m.invited_by,
               u.mfa_enabled, u.mfa_method
        FROM users u
        JOIN user_org_memberships m ON u.user_id = m.user_id
        WHERE m.org_id = $1
        ORDER BY u.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    } else {
        r#"
        SELECT u.user_id, m.org_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
               m.role, u.active, u.created_at, u.updated_at, u.last_login_at, m.invited_by,
               u.mfa_enabled, u.mfa_method
        FROM users u
        JOIN user_org_memberships m ON u.user_id = m.user_id
        WHERE m.org_id = $1 AND u.active = true
        ORDER BY u.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    };

    let rows = sqlx::query_as::<_, OrgUserRow>(query)
        .bind(org_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let total: (i64,) = sqlx::query_as(if include_inactive {
        "SELECT COUNT(*) FROM user_org_memberships WHERE org_id = $1"
    } else {
        r#"
            SELECT COUNT(*) FROM user_org_memberships m
            JOIN users u ON m.user_id = u.user_id
            WHERE m.org_id = $1 AND u.active = true
            "#
    })
    .bind(org_id)
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0 as i32))
}

/// Update a user
pub async fn update_user(pool: &PgPool, user_id: &str, user: &proto::OrgUser) -> Result<bool> {
    let mut tx = pool.begin().await?;

    // Update users table (identity fields)
    let result = sqlx::query(
        r#"
        UPDATE users SET
            name = $2,
            active = $3,
            mfa_enabled = $4,
            mfa_method = $5,
            updated_at = NOW()
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .bind(&user.name)
    .bind(user.active)
    .bind(user.mfa_enabled)
    .bind(if user.mfa_method.is_empty() {
        None
    } else {
        Some(&user.mfa_method)
    })
    .execute(&mut *tx)
    .await?;

    // Update role in user_org_memberships (if org_id provided)
    if !user.org_id.is_empty() {
        sqlx::query(
            r#"
            UPDATE user_org_memberships SET
                role = $3,
                updated_at = NOW()
            WHERE user_id = $1 AND org_id = $2
            "#,
        )
        .bind(user_id)
        .bind(&user.org_id)
        .bind(user.role)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(result.rows_affected() > 0)
}

/// Update user's last login timestamp
pub async fn update_last_login(pool: &PgPool, user_id: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE users SET last_login_at = NOW(), updated_at = NOW()
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
