//! Build registry database operations
//!
//! Builds are separate from agent licenses. A build is a specific version of the
//! agent software with its Tripwire file integrity manifest.

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

/// Default project name for builds where the caller did not specify one.
/// Preserves backwards compat with all pre-v1.4.0 registrations.
pub const DEFAULT_PROJECT: &str = "ciris-agent";

#[derive(Debug, Clone, FromRow)]
pub struct BuildRow {
    pub build_id: sqlx::types::Uuid,
    pub project: String,
    pub version: String,
    pub build_hash: String,
    pub file_manifest_hash: String,
    pub file_manifest_count: i32,
    pub file_manifest_json: serde_json::Value,
    pub includes_modules: Vec<String>,
    pub source_repo: Option<String>,
    pub source_commit: Option<String>,
    pub registered_at: OffsetDateTime,
    pub registered_by: Option<String>,
    pub status: String,
    pub notes: Option<String>,
}

impl BuildRow {
    pub fn to_proto(&self) -> proto::BuildRecord {
        proto::BuildRecord {
            build_id: self.build_id.to_string(),
            version: self.version.clone(),
            build_hash: self.build_hash.clone(),
            file_manifest_hash: self.file_manifest_hash.clone(),
            file_manifest_count: self.file_manifest_count,
            file_manifest_json: serde_json::to_vec(&self.file_manifest_json)
                .unwrap_or_default()
                .into(),
            includes_modules: self.includes_modules.clone(),
            project: self.project.clone(),
            source_repo: self.source_repo.clone().unwrap_or_default(),
            source_commit: self.source_commit.clone().unwrap_or_default(),
            registered_at: self.registered_at.unix_timestamp(),
            registered_by: self.registered_by.clone().unwrap_or_default(),
            status: self.status.clone(),
            notes: self.notes.clone().unwrap_or_default(),
        }
    }
}

/// Register a new build with its file manifest
pub async fn register_build(
    pool: &PgPool,
    build: &proto::BuildRecord,
) -> Result<String> {
    let manifest_json: serde_json::Value = if build.file_manifest_json.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&build.file_manifest_json)
            .unwrap_or(serde_json::json!({}))
    };

    let modules: Vec<String> = if build.includes_modules.is_empty() {
        vec!["core".to_string()]
    } else {
        build.includes_modules.clone()
    };

    let project = if build.project.is_empty() {
        DEFAULT_PROJECT.to_string()
    } else {
        build.project.clone()
    };

    let row: (sqlx::types::Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO builds (
            project, version, build_hash, file_manifest_hash, file_manifest_count,
            file_manifest_json, includes_modules, source_repo, source_commit,
            registered_by, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (build_hash) DO UPDATE SET
            file_manifest_hash = EXCLUDED.file_manifest_hash,
            file_manifest_count = EXCLUDED.file_manifest_count,
            file_manifest_json = EXCLUDED.file_manifest_json,
            notes = EXCLUDED.notes
        RETURNING build_id
        "#,
    )
    .bind(&project)
    .bind(&build.version)
    .bind(&build.build_hash)
    .bind(&build.file_manifest_hash)
    .bind(build.file_manifest_count)
    .bind(&manifest_json)
    .bind(&modules)
    .bind(if build.source_repo.is_empty() { None } else { Some(&build.source_repo) })
    .bind(if build.source_commit.is_empty() { None } else { Some(&build.source_commit) })
    .bind(if build.registered_by.is_empty() { None } else { Some(&build.registered_by) })
    .bind(if build.notes.is_empty() { None } else { Some(&build.notes) })
    .fetch_one(pool)
    .await?;

    Ok(row.0.to_string())
}

/// Get a build by version or build hash, scoped to a project.
///
/// `project=None` is treated as `Some(DEFAULT_PROJECT)` for backwards compat
/// with pre-v1.4.0 callers (CIRISVerify and existing tooling that look up by
/// agent version). Lookup by `build_hash` ignores the project (build hashes
/// are globally unique by SHA-256 construction).
pub async fn get_build(
    pool: &PgPool,
    version: Option<&str>,
    build_hash: Option<&str>,
    project: Option<&str>,
) -> Result<Option<BuildRow>> {
    let project = project.unwrap_or(DEFAULT_PROJECT);

    let row = if let Some(hash) = build_hash {
        sqlx::query_as::<_, BuildRow>(
            r#"
            SELECT build_id, project, version, build_hash, file_manifest_hash, file_manifest_count,
                   file_manifest_json, includes_modules, source_repo, source_commit,
                   registered_at, registered_by, status, notes
            FROM builds
            WHERE build_hash = $1
            "#,
        )
        .bind(hash)
        .fetch_optional(pool)
        .await?
    } else if let Some(ver) = version {
        sqlx::query_as::<_, BuildRow>(
            r#"
            SELECT build_id, project, version, build_hash, file_manifest_hash, file_manifest_count,
                   file_manifest_json, includes_modules, source_repo, source_commit,
                   registered_at, registered_by, status, notes
            FROM builds
            WHERE project = $1 AND version = $2 AND status = 'active'
            ORDER BY registered_at DESC
            LIMIT 1
            "#,
        )
        .bind(project)
        .bind(ver)
        .fetch_optional(pool)
        .await?
    } else {
        None
    };

    Ok(row)
}

/// List all builds
pub async fn list_builds(
    pool: &PgPool,
    status: Option<&str>,
    page_size: i32,
    page_token: Option<&str>,
) -> Result<(Vec<BuildRow>, i64)> {
    let limit = if page_size > 0 { page_size } else { 50 };

    let rows = if let Some(s) = status {
        sqlx::query_as::<_, BuildRow>(
            r#"
            SELECT build_id, project, version, build_hash, file_manifest_hash, file_manifest_count,
                   file_manifest_json, includes_modules, source_repo, source_commit,
                   registered_at, registered_by, status, notes
            FROM builds
            WHERE status = $1
            ORDER BY registered_at DESC
            LIMIT $2
            "#,
        )
        .bind(s)
        .bind(limit as i64)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, BuildRow>(
            r#"
            SELECT build_id, project, version, build_hash, file_manifest_hash, file_manifest_count,
                   file_manifest_json, includes_modules, source_repo, source_commit,
                   registered_at, registered_by, status, notes
            FROM builds
            ORDER BY registered_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(pool)
        .await?
    };

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM builds")
        .fetch_one(pool)
        .await?;

    Ok((rows, count.0))
}
