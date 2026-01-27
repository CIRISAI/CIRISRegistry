//! Registry snapshot database operations for offline verification

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

use super::{AgentRow, PartnerRow};

#[derive(Debug, Clone, FromRow)]
pub struct RegistrySnapshotRow {
    pub snapshot_id: i32,
    pub snapshot_version: i64,
    pub generated_at: OffsetDateTime,
    pub agents_merkle_root: Vec<u8>,
    pub partners_merkle_root: Vec<u8>,
    pub revocations_merkle_root: Vec<u8>,
    pub snapshot_signature: Option<Vec<u8>>,
}

impl RegistrySnapshotRow {
    pub fn to_proto(&self) -> proto::RegistrySnapshot {
        proto::RegistrySnapshot {
            snapshot_version: self.snapshot_version,
            generated_at: self.generated_at.unix_timestamp(),
            agents_merkle_root: self.agents_merkle_root.clone().into(),
            partners_merkle_root: self.partners_merkle_root.clone().into(),
            revocations_merkle_root: self.revocations_merkle_root.clone().into(),
            // TODO: Properly deserialize HybridSignature when stored
            snapshot_signature: None,
        }
    }
}

/// Create a new snapshot
pub async fn create_snapshot(
    pool: &PgPool,
    agents_root: &[u8],
    partners_root: &[u8],
    revocations_root: &[u8],
    signature: Option<&[u8]>,
) -> Result<i64> {
    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO registry_snapshots (agents_merkle_root, partners_merkle_root, revocations_merkle_root, snapshot_signature)
        VALUES ($1, $2, $3, $4)
        RETURNING snapshot_version
        "#,
    )
    .bind(agents_root)
    .bind(partners_root)
    .bind(revocations_root)
    .bind(signature)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Get the latest snapshot
pub async fn get_latest_snapshot(pool: &PgPool) -> Result<Option<RegistrySnapshotRow>> {
    let row = sqlx::query_as::<_, RegistrySnapshotRow>(
        r#"
        SELECT snapshot_id, snapshot_version, generated_at, agents_merkle_root,
               partners_merkle_root, revocations_merkle_root, snapshot_signature
        FROM registry_snapshots
        ORDER BY snapshot_version DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Get a specific snapshot by version
pub async fn get_snapshot(pool: &PgPool, version: i64) -> Result<Option<RegistrySnapshotRow>> {
    let row = sqlx::query_as::<_, RegistrySnapshotRow>(
        r#"
        SELECT snapshot_id, snapshot_version, generated_at, agents_merkle_root,
               partners_merkle_root, revocations_merkle_root, snapshot_signature
        FROM registry_snapshots
        WHERE snapshot_version = $1
        "#,
    )
    .bind(version)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Get all active agents for snapshot generation
pub async fn get_all_agents_for_snapshot(pool: &PgPool) -> Result<Vec<AgentRow>> {
    let rows = sqlx::query_as::<_, AgentRow>(
        r#"
        SELECT agent_hash, agent_type, version_major, version_minor, version_patch,
               version_prerelease, version_build_metadata, base_capabilities,
               max_autonomy_tier, build_timestamp, source_repo, source_commit,
               builder_attestation, status, revocation_reason, revocation_timestamp,
               registered_at, last_updated, registry_signature, is_test_record, test_tag
        FROM agents
        WHERE status != $1 AND (is_test_record = false OR is_test_record IS NULL)
        ORDER BY registered_at ASC
        "#,
    )
    .bind(proto::AgentStatus::AgentRevoked as i32)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get all active partners for snapshot generation
pub async fn get_all_partners_for_snapshot(pool: &PgPool) -> Result<Vec<PartnerRow>> {
    let rows = sqlx::query_as::<_, PartnerRow>(
        r#"
        SELECT partner_id, organization_name, organization_id, license_type, license_id,
               issued_at, expires_at, capabilities_granted, capabilities_denied,
               max_autonomy_tier, geographic_restrictions, deployment_limit,
               offline_grace_hours, technical_contact, compliance_contact,
               status, suspension_reason, revocation_reason
        FROM partners
        WHERE status != $1
        ORDER BY issued_at ASC
        "#,
    )
    .bind(proto::PartnerStatus::PartnerRevoked as i32)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get agents modified since a specific snapshot version
pub async fn get_agents_since_snapshot(
    pool: &PgPool,
    since_timestamp: i64,
) -> Result<Vec<AgentRow>> {
    let rows = sqlx::query_as::<_, AgentRow>(
        r#"
        SELECT agent_hash, agent_type, version_major, version_minor, version_patch,
               version_prerelease, version_build_metadata, base_capabilities,
               max_autonomy_tier, build_timestamp, source_repo, source_commit,
               builder_attestation, status, revocation_reason, revocation_timestamp,
               registered_at, last_updated, registry_signature, is_test_record, test_tag
        FROM agents
        WHERE last_updated > to_timestamp($1)
          AND (is_test_record = false OR is_test_record IS NULL)
        ORDER BY last_updated ASC
        "#,
    )
    .bind(since_timestamp as f64)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get partners modified since a specific snapshot version
pub async fn get_partners_since_snapshot(
    pool: &PgPool,
    since_timestamp: i64,
) -> Result<Vec<PartnerRow>> {
    // Note: partners table doesn't have last_updated, using issued_at as proxy
    let rows = sqlx::query_as::<_, PartnerRow>(
        r#"
        SELECT partner_id, organization_name, organization_id, license_type, license_id,
               issued_at, expires_at, capabilities_granted, capabilities_denied,
               max_autonomy_tier, geographic_restrictions, deployment_limit,
               offline_grace_hours, technical_contact, compliance_contact,
               status, suspension_reason, revocation_reason
        FROM partners
        WHERE issued_at > to_timestamp($1)
        ORDER BY issued_at ASC
        "#,
    )
    .bind(since_timestamp as f64)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// List recent snapshots
pub async fn list_snapshots(pool: &PgPool, limit: i32) -> Result<Vec<RegistrySnapshotRow>> {
    let rows = sqlx::query_as::<_, RegistrySnapshotRow>(
        r#"
        SELECT snapshot_id, snapshot_version, generated_at, agents_merkle_root,
               partners_merkle_root, revocations_merkle_root, snapshot_signature
        FROM registry_snapshots
        ORDER BY snapshot_version DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
