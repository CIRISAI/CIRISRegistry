//! Organization database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct OrganizationRow {
    pub org_id: String,
    pub name: String,
    pub legal_name: String,
    pub tax_id: Option<String>,
    pub partner_id: Option<String>,
    pub primary_email: String,
    pub billing_email: Option<String>,
    pub technical_contact_email: Option<String>,
    pub compliance_contact_email: Option<String>,
    pub oauth_provider: Option<String>,
    pub oauth_domain: Option<String>,
    pub active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: Option<String>,
}

impl OrganizationRow {
    pub fn to_proto(&self) -> proto::Organization {
        proto::Organization {
            org_id: self.org_id.clone(),
            name: self.name.clone(),
            legal_name: self.legal_name.clone(),
            tax_id: self.tax_id.clone().unwrap_or_default(),
            partner_id: self.partner_id.clone().unwrap_or_default(),
            primary_email: self.primary_email.clone(),
            billing_email: self.billing_email.clone().unwrap_or_default(),
            technical_contact_email: self.technical_contact_email.clone().unwrap_or_default(),
            compliance_contact_email: self.compliance_contact_email.clone().unwrap_or_default(),
            oauth_provider: self.oauth_provider.clone().unwrap_or_default(),
            oauth_domain: self.oauth_domain.clone().unwrap_or_default(),
            active: self.active,
            created_at: self.created_at.unix_timestamp(),
            updated_at: self.updated_at.unix_timestamp(),
            created_by: self.created_by.clone().unwrap_or_default(),
            metadata: Default::default(),
        }
    }
}

pub async fn get_organization(pool: &PgPool, org_id: &str) -> Result<Option<OrganizationRow>> {
    let row = sqlx::query_as::<_, OrganizationRow>(
        r#"
        SELECT org_id, name, legal_name, tax_id, partner_id, primary_email,
               billing_email, technical_contact_email, compliance_contact_email,
               oauth_provider, oauth_domain, active, created_at, updated_at, created_by
        FROM organizations
        WHERE org_id = $1
        "#,
    )
    .bind(org_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn create_organization(pool: &PgPool, org: &proto::Organization) -> Result<String> {
    let org_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO organizations (
            org_id, name, legal_name, tax_id, partner_id, primary_email,
            billing_email, technical_contact_email, compliance_contact_email,
            oauth_provider, oauth_domain, active, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
    )
    .bind(&org_id)
    .bind(&org.name)
    .bind(&org.legal_name)
    .bind(if org.tax_id.is_empty() { None } else { Some(&org.tax_id) })
    .bind(if org.partner_id.is_empty() { None } else { Some(&org.partner_id) })
    .bind(&org.primary_email)
    .bind(if org.billing_email.is_empty() { None } else { Some(&org.billing_email) })
    .bind(if org.technical_contact_email.is_empty() { None } else { Some(&org.technical_contact_email) })
    .bind(if org.compliance_contact_email.is_empty() { None } else { Some(&org.compliance_contact_email) })
    .bind(if org.oauth_provider.is_empty() { None } else { Some(&org.oauth_provider) })
    .bind(if org.oauth_domain.is_empty() { None } else { Some(&org.oauth_domain) })
    .bind(org.active)
    .bind(if org.created_by.is_empty() { None } else { Some(&org.created_by) })
    .execute(pool)
    .await?;

    Ok(org_id)
}

pub async fn list_organizations(
    pool: &PgPool,
    page_size: i32,
    offset: i32,
    include_inactive: bool,
) -> Result<(Vec<OrganizationRow>, i32)> {
    let query = if include_inactive {
        r#"
        SELECT org_id, name, legal_name, tax_id, partner_id, primary_email,
               billing_email, technical_contact_email, compliance_contact_email,
               oauth_provider, oauth_domain, active, created_at, updated_at, created_by
        FROM organizations
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#
    } else {
        r#"
        SELECT org_id, name, legal_name, tax_id, partner_id, primary_email,
               billing_email, technical_contact_email, compliance_contact_email,
               oauth_provider, oauth_domain, active, created_at, updated_at, created_by
        FROM organizations
        WHERE active = true
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#
    };

    let rows = sqlx::query_as::<_, OrganizationRow>(query)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let total: (i64,) = sqlx::query_as(
        if include_inactive {
            "SELECT COUNT(*) FROM organizations"
        } else {
            "SELECT COUNT(*) FROM organizations WHERE active = true"
        },
    )
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0 as i32))
}

/// Create an organization with an initial admin user in a single transaction
/// This ensures the org exists before the user is created (avoids FK violation race)
pub async fn create_organization_with_admin(
    pool: &PgPool,
    org: &proto::Organization,
    admin_user: &proto::OrgUser,
) -> Result<(String, String)> {
    let org_id = uuid::Uuid::new_v4().to_string();
    let user_id = uuid::Uuid::new_v4().to_string();

    let mut tx = pool.begin().await?;

    // Create organization
    sqlx::query(
        r#"
        INSERT INTO organizations (
            org_id, name, legal_name, tax_id, partner_id, primary_email,
            billing_email, technical_contact_email, compliance_contact_email,
            oauth_provider, oauth_domain, active, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
    )
    .bind(&org_id)
    .bind(&org.name)
    .bind(&org.legal_name)
    .bind(if org.tax_id.is_empty() { None } else { Some(&org.tax_id) })
    .bind(if org.partner_id.is_empty() { None } else { Some(&org.partner_id) })
    .bind(&org.primary_email)
    .bind(if org.billing_email.is_empty() { None } else { Some(&org.billing_email) })
    .bind(if org.technical_contact_email.is_empty() { None } else { Some(&org.technical_contact_email) })
    .bind(if org.compliance_contact_email.is_empty() { None } else { Some(&org.compliance_contact_email) })
    .bind(if org.oauth_provider.is_empty() { None } else { Some(&org.oauth_provider) })
    .bind(if org.oauth_domain.is_empty() { None } else { Some(&org.oauth_domain) })
    .bind(org.active)
    .bind(if org.created_by.is_empty() { None } else { Some(&org.created_by) })
    .execute(&mut *tx)
    .await?;

    // Create admin user in the same transaction
    sqlx::query(
        r#"
        INSERT INTO org_users (
            user_id, org_id, email, name, oauth_provider, oauth_subject,
            role, active, invited_by, mfa_enabled, mfa_method
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&user_id)
    .bind(&org_id)
    .bind(&admin_user.email)
    .bind(&admin_user.name)
    .bind(if admin_user.oauth_provider.is_empty() { None } else { Some(&admin_user.oauth_provider) })
    .bind(if admin_user.oauth_subject.is_empty() { None } else { Some(&admin_user.oauth_subject) })
    .bind(admin_user.role)
    .bind(admin_user.active)
    .bind(if admin_user.invited_by.is_empty() { None } else { Some(&admin_user.invited_by) })
    .bind(admin_user.mfa_enabled)
    .bind(if admin_user.mfa_method.is_empty() { None } else { Some(&admin_user.mfa_method) })
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((org_id, user_id))
}

/// Update an organization
pub async fn update_organization(pool: &PgPool, org_id: &str, org: &proto::Organization) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE organizations SET
            name = $2,
            legal_name = $3,
            tax_id = $4,
            partner_id = $5,
            primary_email = $6,
            billing_email = $7,
            technical_contact_email = $8,
            compliance_contact_email = $9,
            oauth_provider = $10,
            oauth_domain = $11,
            active = $12,
            updated_at = NOW()
        WHERE org_id = $1
        "#,
    )
    .bind(org_id)
    .bind(&org.name)
    .bind(&org.legal_name)
    .bind(if org.tax_id.is_empty() { None } else { Some(&org.tax_id) })
    .bind(if org.partner_id.is_empty() { None } else { Some(&org.partner_id) })
    .bind(&org.primary_email)
    .bind(if org.billing_email.is_empty() { None } else { Some(&org.billing_email) })
    .bind(if org.technical_contact_email.is_empty() { None } else { Some(&org.technical_contact_email) })
    .bind(if org.compliance_contact_email.is_empty() { None } else { Some(&org.compliance_contact_email) })
    .bind(if org.oauth_provider.is_empty() { None } else { Some(&org.oauth_provider) })
    .bind(if org.oauth_domain.is_empty() { None } else { Some(&org.oauth_domain) })
    .bind(org.active)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
