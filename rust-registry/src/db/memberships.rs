//! User and organization membership database operations (multi-org support)

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

/// User identity (org-independent) - new multi-org model
#[derive(Debug, Clone, FromRow)]
pub struct MultiOrgUserRow {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub oauth_provider: Option<String>,
    pub oauth_subject: Option<String>,
    pub active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_login_at: Option<OffsetDateTime>,
    pub mfa_enabled: bool,
    pub mfa_method: Option<String>,
}

/// User's membership in an organization
#[derive(Debug, Clone, FromRow)]
pub struct MembershipRow {
    pub user_id: String,
    pub org_id: String,
    pub role: i32,
    pub invited_by: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    // Denormalized from organizations table (joined)
    #[sqlx(default)]
    pub org_name: Option<String>,
    #[sqlx(default)]
    pub org_type: Option<i32>,
}

impl MultiOrgUserRow {
    pub fn to_proto(&self, memberships: Vec<proto::OrgMembership>) -> proto::User {
        use time::format_description::well_known::Rfc3339;

        proto::User {
            user_id: self.user_id.clone(),
            email: self.email.clone(),
            name: self.name.clone(),
            oauth_provider: self.oauth_provider.clone().unwrap_or_default(),
            oauth_subject: self.oauth_subject.clone().unwrap_or_default(),
            active: self.active,
            created_at: self.created_at.unix_timestamp(),
            updated_at: self.updated_at.unix_timestamp(),
            last_login_at: self.last_login_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            created_at_iso: self.created_at.format(&Rfc3339).unwrap_or_default(),
            updated_at_iso: self.updated_at.format(&Rfc3339).unwrap_or_default(),
            last_login_at_iso: self
                .last_login_at
                .map(|t| t.format(&Rfc3339).unwrap_or_default())
                .unwrap_or_default(),
            mfa_enabled: self.mfa_enabled,
            mfa_method: self.mfa_method.clone().unwrap_or_default(),
            memberships,
        }
    }
}

impl MembershipRow {
    pub fn to_proto(&self) -> proto::OrgMembership {
        use time::format_description::well_known::Rfc3339;

        proto::OrgMembership {
            org_id: self.org_id.clone(),
            org_name: self.org_name.clone().unwrap_or_default(),
            org_type: self.org_type.unwrap_or(4), // Default to COMMUNITY
            role: self.role,
            invited_by: self.invited_by.clone().unwrap_or_default(),
            created_at: self.created_at.unix_timestamp(),
            created_at_iso: self.created_at.format(&Rfc3339).unwrap_or_default(),
        }
    }
}

// =============================================================================
// Multi-Org User CRUD (new model)
// =============================================================================

pub async fn get_multiorg_user(pool: &PgPool, user_id: &str) -> Result<Option<MultiOrgUserRow>> {
    let row = sqlx::query_as::<_, MultiOrgUserRow>(
        r#"
        SELECT user_id, email, name, oauth_provider, oauth_subject,
               active, created_at, updated_at, last_login_at, mfa_enabled, mfa_method
        FROM users
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_multiorg_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<MultiOrgUserRow>> {
    let row = sqlx::query_as::<_, MultiOrgUserRow>(
        r#"
        SELECT user_id, email, name, oauth_provider, oauth_subject,
               active, created_at, updated_at, last_login_at, mfa_enabled, mfa_method
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn create_multiorg_user(pool: &PgPool, user: &proto::User) -> Result<String> {
    let user_id = uuid::Uuid::new_v4().to_string();

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
    .execute(pool)
    .await?;

    Ok(user_id)
}

/// Create user and add to org in single transaction
pub async fn create_user_with_membership(
    pool: &PgPool,
    user: &proto::User,
    org_id: &str,
    role: i32,
    invited_by: Option<&str>,
) -> Result<String> {
    let user_id = uuid::Uuid::new_v4().to_string();
    let mut tx = pool.begin().await?;

    // Create user
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

    // Add membership
    sqlx::query(
        r#"
        INSERT INTO user_org_memberships (user_id, org_id, role, invited_by)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&user_id)
    .bind(org_id)
    .bind(role)
    .bind(invited_by)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(user_id)
}

// =============================================================================
// Membership CRUD
// =============================================================================

pub async fn get_user_memberships(pool: &PgPool, user_id: &str) -> Result<Vec<MembershipRow>> {
    let rows = sqlx::query_as::<_, MembershipRow>(
        r#"
        SELECT m.user_id, m.org_id, m.role, m.invited_by, m.created_at, m.updated_at,
               o.name as org_name, o.org_type
        FROM user_org_memberships m
        JOIN organizations o ON m.org_id = o.org_id
        WHERE m.user_id = $1
        ORDER BY m.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn add_user_to_org(
    pool: &PgPool,
    user_id: &str,
    org_id: &str,
    role: i32,
    invited_by: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO user_org_memberships (user_id, org_id, role, invited_by)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, org_id) DO UPDATE SET role = $3, updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(org_id)
    .bind(role)
    .bind(invited_by)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_user_from_org(pool: &PgPool, user_id: &str, org_id: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"
        DELETE FROM user_org_memberships
        WHERE user_id = $1 AND org_id = $2
        "#,
    )
    .bind(user_id)
    .bind(org_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn update_user_org_role(
    pool: &PgPool,
    user_id: &str,
    org_id: &str,
    role: i32,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE user_org_memberships SET role = $3, updated_at = NOW()
        WHERE user_id = $1 AND org_id = $2
        "#,
    )
    .bind(user_id)
    .bind(org_id)
    .bind(role)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// List users in an organization with their membership info
pub async fn list_org_members(
    pool: &PgPool,
    org_id: &str,
    page_size: i32,
    offset: i32,
    include_inactive: bool,
) -> Result<(Vec<(MultiOrgUserRow, MembershipRow)>, i32)> {
    let query = if include_inactive {
        r#"
        SELECT u.user_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
               u.active, u.created_at, u.updated_at, u.last_login_at, u.mfa_enabled, u.mfa_method,
               m.org_id, m.role, m.invited_by, m.created_at as m_created_at, m.updated_at as m_updated_at
        FROM users u
        JOIN user_org_memberships m ON u.user_id = m.user_id
        WHERE m.org_id = $1
        ORDER BY m.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    } else {
        r#"
        SELECT u.user_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
               u.active, u.created_at, u.updated_at, u.last_login_at, u.mfa_enabled, u.mfa_method,
               m.org_id, m.role, m.invited_by, m.created_at as m_created_at, m.updated_at as m_updated_at
        FROM users u
        JOIN user_org_memberships m ON u.user_id = m.user_id
        WHERE m.org_id = $1 AND u.active = true
        ORDER BY m.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    };

    // For simplicity, we'll fetch users and memberships separately
    // In production, this could be optimized with a custom struct
    let users = sqlx::query_as::<_, MultiOrgUserRow>(
        if include_inactive {
            r#"
            SELECT u.user_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
                   u.active, u.created_at, u.updated_at, u.last_login_at, u.mfa_enabled, u.mfa_method
            FROM users u
            JOIN user_org_memberships m ON u.user_id = m.user_id
            WHERE m.org_id = $1
            ORDER BY m.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        } else {
            r#"
            SELECT u.user_id, u.email, u.name, u.oauth_provider, u.oauth_subject,
                   u.active, u.created_at, u.updated_at, u.last_login_at, u.mfa_enabled, u.mfa_method
            FROM users u
            JOIN user_org_memberships m ON u.user_id = m.user_id
            WHERE m.org_id = $1 AND u.active = true
            ORDER BY m.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        },
    )
    .bind(org_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let memberships = sqlx::query_as::<_, MembershipRow>(
        r#"
        SELECT m.user_id, m.org_id, m.role, m.invited_by, m.created_at, m.updated_at,
               o.name as org_name, o.org_type
        FROM user_org_memberships m
        JOIN organizations o ON m.org_id = o.org_id
        WHERE m.org_id = $1
        "#,
    )
    .bind(org_id)
    .fetch_all(pool)
    .await?;

    // Pair users with their memberships
    let results: Vec<(MultiOrgUserRow, MembershipRow)> = users
        .into_iter()
        .filter_map(|user| {
            memberships
                .iter()
                .find(|m| m.user_id == user.user_id)
                .map(|m| (user, m.clone()))
        })
        .collect();

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

    Ok((results, total.0 as i32))
}

/// Get user's role in a specific org (for authorization)
pub async fn get_user_role_in_org(
    pool: &PgPool,
    user_id: &str,
    org_id: &str,
) -> Result<Option<i32>> {
    let result: Option<(i32,)> = sqlx::query_as(
        r#"
        SELECT role FROM user_org_memberships
        WHERE user_id = $1 AND org_id = $2
        "#,
    )
    .bind(user_id)
    .bind(org_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|(r,)| r))
}
