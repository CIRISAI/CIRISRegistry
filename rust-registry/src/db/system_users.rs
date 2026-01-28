//! System user database operations (global admins)

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct SystemUserRow {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub role: i32,
    pub active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: Option<String>,
}

impl SystemUserRow {
    pub fn to_proto(&self) -> proto::SystemUser {
        use time::format_description::well_known::Rfc3339;

        proto::SystemUser {
            user_id: self.user_id.clone(),
            email: self.email.clone(),
            name: self.name.clone(),
            role: self.role,
            active: self.active,
            created_at: self.created_at.unix_timestamp(),
            updated_at: self.updated_at.unix_timestamp(),
            created_at_iso: self.created_at.format(&Rfc3339).unwrap_or_default(),
            updated_at_iso: self.updated_at.format(&Rfc3339).unwrap_or_default(),
            created_by: self.created_by.clone().unwrap_or_default(),
        }
    }
}

pub async fn get_system_user(pool: &PgPool, user_id: &str) -> Result<Option<SystemUserRow>> {
    let row = sqlx::query_as::<_, SystemUserRow>(
        r#"
        SELECT user_id, email, name, role, active, created_at, updated_at, created_by
        FROM system_users
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_system_user_by_email(pool: &PgPool, email: &str) -> Result<Option<SystemUserRow>> {
    let row = sqlx::query_as::<_, SystemUserRow>(
        r#"
        SELECT user_id, email, name, role, active, created_at, updated_at, created_by
        FROM system_users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn create_system_user(
    pool: &PgPool,
    user: &proto::SystemUser,
    created_by: Option<&str>,
) -> Result<String> {
    let user_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO system_users (user_id, email, name, role, active, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(&user_id)
    .bind(&user.email)
    .bind(&user.name)
    .bind(user.role)
    .bind(user.active)
    .bind(created_by)
    .execute(pool)
    .await?;

    Ok(user_id)
}

pub async fn list_system_users(
    pool: &PgPool,
    page_size: i32,
    offset: i32,
    include_inactive: bool,
) -> Result<(Vec<SystemUserRow>, i32)> {
    let query = if include_inactive {
        r#"
        SELECT user_id, email, name, role, active, created_at, updated_at, created_by
        FROM system_users
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#
    } else {
        r#"
        SELECT user_id, email, name, role, active, created_at, updated_at, created_by
        FROM system_users
        WHERE active = true
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#
    };

    let rows = sqlx::query_as::<_, SystemUserRow>(query)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let total: (i64,) = sqlx::query_as(
        if include_inactive {
            "SELECT COUNT(*) FROM system_users"
        } else {
            "SELECT COUNT(*) FROM system_users WHERE active = true"
        },
    )
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0 as i32))
}

pub async fn update_system_user(pool: &PgPool, user_id: &str, user: &proto::SystemUser) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE system_users SET
            name = $2,
            role = $3,
            active = $4,
            updated_at = NOW()
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .bind(&user.name)
    .bind(user.role)
    .bind(user.active)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Check if a user has system admin privileges
pub async fn is_system_admin(pool: &PgPool, email: &str) -> Result<bool> {
    let result: Option<(i32,)> = sqlx::query_as(
        r#"
        SELECT role FROM system_users
        WHERE email = $1 AND active = true AND role = 1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}
