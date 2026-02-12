//! Agent database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::{RegistryError, Result};
use crate::proto;

/// Agent record from database
#[derive(Debug, Clone, FromRow)]
pub struct AgentRow {
    pub agent_hash: Vec<u8>,
    pub agent_type: i32,
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,
    pub version_prerelease: Option<String>,
    pub version_build_metadata: Option<String>,
    pub base_capabilities: Vec<String>,
    pub max_autonomy_tier: i32,
    pub build_timestamp: OffsetDateTime,
    pub source_repo: Option<String>,
    pub source_commit: Option<String>,
    pub builder_attestation: Option<Vec<u8>>,
    pub status: i32,
    pub revocation_reason: Option<String>,
    pub revocation_timestamp: Option<OffsetDateTime>,
    pub registered_at: OffsetDateTime,
    pub last_updated: OffsetDateTime,
    pub registry_signature: Option<Vec<u8>>,
    pub is_test_record: bool,
    pub test_tag: Option<String>,
    // Identity template (CIRISVerify enforcement, v1.2.0)
    pub identity_template: Option<String>,
    pub stewardship_tier: Option<i32>,
    pub permitted_actions: Vec<String>,
    pub template_hash: Option<Vec<u8>>,
}

impl AgentRow {
    /// Convert to protobuf AgentRecord
    pub fn to_proto(&self) -> proto::AgentRecord {
        proto::AgentRecord {
            agent_hash: self.agent_hash.clone().into(),
            agent_hash_hex: hex::encode(&self.agent_hash),
            agent_type: self.agent_type,
            version: Some(proto::SemanticVersion {
                major: self.version_major as u32,
                minor: self.version_minor as u32,
                patch: self.version_patch as u32,
                prerelease: self.version_prerelease.clone().unwrap_or_default(),
                build_metadata: self.version_build_metadata.clone().unwrap_or_default(),
            }),
            base_capabilities: self.base_capabilities.clone(),
            max_autonomy_tier: self.max_autonomy_tier,
            build_timestamp: self.build_timestamp.unix_timestamp(),
            source_repo: self.source_repo.clone().unwrap_or_default(),
            source_commit: self.source_commit.clone().unwrap_or_default(),
            builder_attestation: self
                .builder_attestation
                .clone()
                .map(Into::into)
                .unwrap_or_default(),
            status: self.status,
            revocation_reason: self.revocation_reason.clone().unwrap_or_default(),
            revocation_timestamp: self
                .revocation_timestamp
                .map(|t| t.unix_timestamp())
                .unwrap_or(0),
            registered_at: self.registered_at.unix_timestamp(),
            last_updated: self.last_updated.unix_timestamp(),
            registry_signature: None, // Populated separately
            is_test_record: self.is_test_record,
            test_tag: self.test_tag.clone().unwrap_or_default(),
            // Identity template (v1.2.0)
            identity_template: self.identity_template.clone().unwrap_or_default(),
            permitted_actions: self.permitted_actions.clone(),
            stewardship_tier: self.stewardship_tier.unwrap_or(0),
            template_hash: self.template_hash.clone().unwrap_or_default().into(),
        }
    }
}

/// Lookup agent by hash
pub async fn lookup_agent(pool: &PgPool, agent_hash: &[u8]) -> Result<Option<AgentRow>> {
    let row = sqlx::query_as::<_, AgentRow>(
        r#"
        SELECT
            agent_hash, agent_type, version_major, version_minor, version_patch,
            version_prerelease, version_build_metadata, base_capabilities,
            max_autonomy_tier, build_timestamp, source_repo, source_commit,
            builder_attestation, status, revocation_reason, revocation_timestamp,
            registered_at, last_updated, registry_signature, is_test_record, test_tag,
            identity_template, stewardship_tier, permitted_actions, template_hash
        FROM agents
        WHERE agent_hash = $1
        "#,
    )
    .bind(agent_hash)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Batch lookup agents by hashes
pub async fn batch_lookup_agents(
    pool: &PgPool,
    hashes: &[Vec<u8>],
) -> Result<Vec<(Vec<u8>, Option<AgentRow>)>> {
    let rows = sqlx::query_as::<_, AgentRow>(
        r#"
        SELECT
            agent_hash, agent_type, version_major, version_minor, version_patch,
            version_prerelease, version_build_metadata, base_capabilities,
            max_autonomy_tier, build_timestamp, source_repo, source_commit,
            builder_attestation, status, revocation_reason, revocation_timestamp,
            registered_at, last_updated, registry_signature, is_test_record, test_tag,
            identity_template, stewardship_tier, permitted_actions, template_hash
        FROM agents
        WHERE agent_hash = ANY($1)
        "#,
    )
    .bind(hashes)
    .fetch_all(pool)
    .await?;

    // Map results back to input order
    let results: Vec<(Vec<u8>, Option<AgentRow>)> = hashes
        .iter()
        .map(|hash| {
            let found = rows.iter().find(|r| r.agent_hash == *hash).cloned();
            (hash.clone(), found)
        })
        .collect();

    Ok(results)
}

/// Register a new agent
pub async fn register_agent(pool: &PgPool, record: &proto::AgentRecord) -> Result<()> {
    let version = record.version.as_ref().ok_or_else(|| {
        RegistryError::InvalidArgument("version is required".to_string())
    })?;

    sqlx::query(
        r#"
        INSERT INTO agents (
            agent_hash, agent_type, version_major, version_minor, version_patch,
            version_prerelease, version_build_metadata, base_capabilities,
            max_autonomy_tier, build_timestamp, source_repo, source_commit,
            builder_attestation, status, is_test_record, test_tag
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, to_timestamp($10), $11, $12, $13, $14, $15, $16)
        "#,
    )
    .bind(record.agent_hash.as_ref() as &[u8])
    .bind(record.agent_type)
    .bind(version.major as i32)
    .bind(version.minor as i32)
    .bind(version.patch as i32)
    .bind(if version.prerelease.is_empty() {
        None
    } else {
        Some(&version.prerelease)
    })
    .bind(if version.build_metadata.is_empty() {
        None
    } else {
        Some(&version.build_metadata)
    })
    .bind(&record.base_capabilities)
    .bind(record.max_autonomy_tier)
    .bind(record.build_timestamp as f64)
    .bind(if record.source_repo.is_empty() {
        None
    } else {
        Some(&record.source_repo)
    })
    .bind(if record.source_commit.is_empty() {
        None
    } else {
        Some(&record.source_commit)
    })
    .bind(if record.builder_attestation.is_empty() {
        None::<&[u8]>
    } else {
        Some(record.builder_attestation.as_ref() as &[u8])
    })
    .bind(record.status)
    .bind(record.is_test_record)
    .bind(if record.test_tag.is_empty() {
        None
    } else {
        Some(&record.test_tag)
    })
    .execute(pool)
    .await?;

    Ok(())
}

/// Revoke an agent
pub async fn revoke_agent(
    pool: &PgPool,
    agent_hash: &[u8],
    reason: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE agents
        SET status = $1, revocation_reason = $2, revocation_timestamp = NOW(), last_updated = NOW()
        WHERE agent_hash = $3 AND status != $1
        "#,
    )
    .bind(proto::AgentStatus::AgentRevoked as i32)
    .bind(reason)
    .bind(agent_hash)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Cleanup test records by tag
pub async fn cleanup_test_records(pool: &PgPool, test_tag: &str) -> Result<i64> {
    let result = sqlx::query(
        r#"
        DELETE FROM agents
        WHERE is_test_record = true AND test_tag = $1
        "#,
    )
    .bind(test_tag)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

/// List agents result with pagination info
#[derive(Debug)]
pub struct ListAgentsResult {
    pub agents: Vec<AgentRow>,
    pub total_count: i32,
    pub active_count: i32,
    pub deprecated_count: i32,
    pub revoked_count: i32,
}

/// List registered agents with filtering and pagination
pub async fn list_registered_agents(
    pool: &PgPool,
    agent_type: Option<i32>,
    status: Option<i32>,
    version_prefix: Option<&str>,
    search_query: Option<&str>,
    include_test_records: bool,
    page_size: i32,
    offset: i32,
    order_by: &str,
    descending: bool,
) -> Result<ListAgentsResult> {
    // Build dynamic WHERE clause
    let mut conditions = Vec::new();
    let mut param_idx = 1;

    if !include_test_records {
        conditions.push(format!("is_test_record = false"));
    }

    // We'll build the query with placeholders
    let agent_type_filter = agent_type.filter(|t| *t != 0);
    let status_filter = status.filter(|s| *s != 0);

    let mut query = String::from(
        r#"
        SELECT
            agent_hash, agent_type, version_major, version_minor, version_patch,
            version_prerelease, version_build_metadata, base_capabilities,
            max_autonomy_tier, build_timestamp, source_repo, source_commit,
            builder_attestation, status, revocation_reason, revocation_timestamp,
            registered_at, last_updated, registry_signature, is_test_record, test_tag,
            identity_template, stewardship_tier, permitted_actions, template_hash
        FROM agents
        WHERE 1=1
        "#,
    );

    if !include_test_records {
        query.push_str(" AND is_test_record = false");
    }

    if agent_type_filter.is_some() {
        query.push_str(&format!(" AND agent_type = ${}", param_idx));
        param_idx += 1;
    }

    if status_filter.is_some() {
        query.push_str(&format!(" AND status = ${}", param_idx));
        param_idx += 1;
    }

    if version_prefix.is_some() && !version_prefix.unwrap().is_empty() {
        query.push_str(&format!(
            " AND CONCAT(version_major, '.', version_minor, '.', version_patch) LIKE ${}",
            param_idx
        ));
        param_idx += 1;
    }

    if search_query.is_some() && !search_query.unwrap().is_empty() {
        query.push_str(&format!(
            " AND (source_repo ILIKE ${0} OR source_commit ILIKE ${0})",
            param_idx
        ));
        param_idx += 1;
    }

    // Validate and apply ordering
    let order_col = match order_by {
        "version" => "version_major, version_minor, version_patch",
        "agent_type" => "agent_type",
        "status" => "status",
        _ => "registered_at",
    };

    let order_dir = if descending { "DESC" } else { "ASC" };
    query.push_str(&format!(" ORDER BY {} {}", order_col, order_dir));

    // Pagination
    query.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

    // Build the query with bindings
    let mut sqlx_query = sqlx::query_as::<_, AgentRow>(&query);

    if let Some(t) = agent_type_filter {
        sqlx_query = sqlx_query.bind(t);
    }
    if let Some(s) = status_filter {
        sqlx_query = sqlx_query.bind(s);
    }
    if let Some(vp) = version_prefix {
        if !vp.is_empty() {
            sqlx_query = sqlx_query.bind(format!("{}%", vp));
        }
    }
    if let Some(sq) = search_query {
        if !sq.is_empty() {
            sqlx_query = sqlx_query.bind(format!("%{}%", sq));
        }
    }

    sqlx_query = sqlx_query.bind(page_size).bind(offset);

    let agents = sqlx_query.fetch_all(pool).await?;

    // Get counts
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COUNT(*)::bigint as total,
            COUNT(*) FILTER (WHERE status = 1)::bigint as active,
            COUNT(*) FILTER (WHERE status = 2)::bigint as deprecated,
            COUNT(*) FILTER (WHERE status = 3)::bigint as revoked
        FROM agents
        WHERE ($1 = true OR is_test_record = false)
        "#,
    )
    .bind(include_test_records)
    .fetch_one(pool)
    .await?;

    Ok(ListAgentsResult {
        agents,
        total_count: counts.0 as i32,
        active_count: counts.1 as i32,
        deprecated_count: counts.2 as i32,
        revoked_count: counts.3 as i32,
    })
}
