//! Partner database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

/// Partner record from database
#[derive(Debug, Clone, FromRow)]
pub struct PartnerRow {
    pub partner_id: String,
    pub organization_name: String,
    pub organization_id: String,
    pub license_type: i32,
    pub license_id: String,
    pub issued_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    pub capabilities_granted: Vec<String>,
    pub capabilities_denied: Vec<String>,
    pub max_autonomy_tier: i32,
    pub requires_supervisor: bool,
    pub geographic_restrictions: Vec<String>,
    pub deployment_limit: i32,
    pub offline_grace_hours: i32,
    pub technical_contact: Option<String>,
    pub compliance_contact: Option<String>,
    pub status: i32,
    pub suspension_reason: Option<String>,
    pub revocation_reason: Option<String>,
    pub status_changed_at: Option<OffsetDateTime>,
}

impl PartnerRow {
    pub fn to_proto(&self) -> proto::PartnerRecord {
        proto::PartnerRecord {
            partner_id: self.partner_id.clone(),
            organization_name: self.organization_name.clone(),
            organization_id: self.organization_id.clone(),
            license_type: self.license_type,
            license_id: self.license_id.clone(),
            issued_at: self.issued_at.unix_timestamp(),
            expires_at: self.expires_at.unix_timestamp(),
            capabilities_granted: self.capabilities_granted.clone(),
            capabilities_denied: self.capabilities_denied.clone(),
            max_autonomy_tier: self.max_autonomy_tier,
            requires_supervisor: self.requires_supervisor,
            geographic_restrictions: self.geographic_restrictions.clone(),
            deployment_limit: self.deployment_limit,
            offline_grace_hours: self.offline_grace_hours,
            technical_contact: self.technical_contact.clone().unwrap_or_default(),
            compliance_contact: self.compliance_contact.clone().unwrap_or_default(),
            status: self.status,
            suspension_reason: self.suspension_reason.clone().unwrap_or_default(),
            revocation_reason: self.revocation_reason.clone().unwrap_or_default(),
            status_changed_at: self
                .status_changed_at
                .map(|t| t.unix_timestamp())
                .unwrap_or(0),
            license_signature: None,
            registry_signature: None,
        }
    }
}

/// Lookup partner by ID
pub async fn lookup_partner(pool: &PgPool, partner_id: &str) -> Result<Option<PartnerRow>> {
    let row = sqlx::query_as::<_, PartnerRow>(
        r#"
        SELECT
            partner_id, organization_name, organization_id, license_type, license_id,
            issued_at, expires_at, capabilities_granted, capabilities_denied,
            max_autonomy_tier, requires_supervisor, geographic_restrictions,
            deployment_limit, offline_grace_hours, technical_contact, compliance_contact,
            status, suspension_reason, revocation_reason, status_changed_at
        FROM partners
        WHERE partner_id = $1
        "#,
    )
    .bind(partner_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Register a new partner
pub async fn register_partner(pool: &PgPool, record: &proto::PartnerRecord) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO partners (
            partner_id, organization_name, organization_id, license_type, license_id,
            issued_at, expires_at, capabilities_granted, capabilities_denied,
            max_autonomy_tier, requires_supervisor, geographic_restrictions,
            deployment_limit, offline_grace_hours, technical_contact, compliance_contact,
            status
        )
        VALUES ($1, $2, $3, $4, $5, to_timestamp($6), to_timestamp($7), $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        "#,
    )
    .bind(&record.partner_id)
    .bind(&record.organization_name)
    .bind(&record.organization_id)
    .bind(record.license_type)
    .bind(&record.license_id)
    .bind(record.issued_at as f64)
    .bind(record.expires_at as f64)
    .bind(&record.capabilities_granted)
    .bind(&record.capabilities_denied)
    .bind(record.max_autonomy_tier)
    .bind(record.requires_supervisor)
    .bind(&record.geographic_restrictions)
    .bind(record.deployment_limit)
    .bind(record.offline_grace_hours)
    .bind(if record.technical_contact.is_empty() {
        None
    } else {
        Some(&record.technical_contact)
    })
    .bind(if record.compliance_contact.is_empty() {
        None
    } else {
        Some(&record.compliance_contact)
    })
    .bind(record.status)
    .execute(pool)
    .await?;

    Ok(())
}

/// List partners with expiring licenses
pub async fn list_expiring_licenses(
    pool: &PgPool,
    days: i32,
    include_expired: bool,
) -> Result<Vec<PartnerRow>> {
    let query = if include_expired {
        r#"
        SELECT
            partner_id, organization_name, organization_id, license_type, license_id,
            issued_at, expires_at, capabilities_granted, capabilities_denied,
            max_autonomy_tier, requires_supervisor, geographic_restrictions,
            deployment_limit, offline_grace_hours, technical_contact, compliance_contact,
            status, suspension_reason, revocation_reason, status_changed_at
        FROM partners
        WHERE expires_at <= NOW() + INTERVAL '1 day' * $1
          AND status = $2
        ORDER BY expires_at ASC
        "#
    } else {
        r#"
        SELECT
            partner_id, organization_name, organization_id, license_type, license_id,
            issued_at, expires_at, capabilities_granted, capabilities_denied,
            max_autonomy_tier, requires_supervisor, geographic_restrictions,
            deployment_limit, offline_grace_hours, technical_contact, compliance_contact,
            status, suspension_reason, revocation_reason, status_changed_at
        FROM partners
        WHERE expires_at > NOW()
          AND expires_at <= NOW() + INTERVAL '1 day' * $1
          AND status = $2
        ORDER BY expires_at ASC
        "#
    };

    let rows = sqlx::query_as::<_, PartnerRow>(query)
        .bind(days)
        .bind(proto::PartnerStatus::PartnerActive as i32)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}
