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

/// Default target for legacy lookup paths (`GET /v1/builds/{version}` without
/// `?target=`). The canonical Python source manifest is byte-identical across
/// every platform (iOS, Android, desktop, server), so it's the right answer
/// when a caller hasn't said otherwise. Closes CIRISRegistry#11 — iOS-tagged
/// rows must not win the version-lookup race.
pub const DEFAULT_TARGET: &str = "python-source-tree";

#[derive(Debug, Clone, FromRow)]
pub struct BuildRow {
    pub build_id: sqlx::types::Uuid,
    pub project: String,
    pub version: String,
    pub target: String,
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
            target: self.target.clone(),
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

    // target is required at the application layer; empty → DEFAULT_TARGET to
    // preserve the legacy gRPC callers that predate v1.4.1. The HTTP handler
    // (register_build_http) rejects empty target before calling here.
    let target = if build.target.is_empty() {
        DEFAULT_TARGET.to_string()
    } else {
        build.target.clone()
    };

    // ON CONFLICT (project, version, target): the target-aware UNIQUE
    // constraint (migration 028) is the right discriminator for
    // multi-target releases — a single `ciris-build-sign register`
    // invocation POSTs N rows at the same (project, version) with
    // different targets and (in the common case) the same
    // includes_modules. The legacy ON CONFLICT (build_hash) clause
    // didn't help: each target has a distinct build_hash by content,
    // so ON CONFLICT never fired and the now-removed
    // builds_project_version_modules_unique constraint tripped
    // → 500. The build_hash UNIQUE constraint stays as a global
    // cross-project sanity check but is no longer the conflict target.
    // Re-POSTing the same (project, version, target) with a different
    // build_hash (re-built artifact, re-tagged release) updates in
    // place rather than failing. Closes CIRISRegistry#13.
    let row: (sqlx::types::Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO builds (
            project, version, target, build_hash, file_manifest_hash, file_manifest_count,
            file_manifest_json, includes_modules, source_repo, source_commit,
            registered_by, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ON CONFLICT (project, version, target) DO UPDATE SET
            build_hash = EXCLUDED.build_hash,
            file_manifest_hash = EXCLUDED.file_manifest_hash,
            file_manifest_count = EXCLUDED.file_manifest_count,
            file_manifest_json = EXCLUDED.file_manifest_json,
            includes_modules = EXCLUDED.includes_modules,
            source_repo = EXCLUDED.source_repo,
            source_commit = EXCLUDED.source_commit,
            registered_by = EXCLUDED.registered_by,
            notes = EXCLUDED.notes
        RETURNING build_id
        "#,
    )
    .bind(&project)
    .bind(&build.version)
    .bind(&target)
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

/// Get a build by version or build hash, scoped to a project and target.
///
/// `project=None` → `DEFAULT_PROJECT` (`ciris-agent`) for pre-v1.4.0 callers.
/// `target=None`  → `DEFAULT_TARGET` (`python-source-tree`) for pre-v1.4.1
/// callers. The canonical Python-source manifest is byte-identical across all
/// platforms — picking it as the default keeps L4 file-integrity attestation
/// working for legacy verify clients that don't pass a target yet.
///
/// Lookup by `build_hash` ignores project + target (build hashes are globally
/// unique by SHA-256 construction).
///
/// Closes CIRISRegistry#11 — eliminates the "iOS row wins the version lookup
/// race" failure mode that broke L4 attestation on every agent in v2.8.9.
pub async fn get_build(
    pool: &PgPool,
    version: Option<&str>,
    build_hash: Option<&str>,
    project: Option<&str>,
    target: Option<&str>,
) -> Result<Option<BuildRow>> {
    let project = project.unwrap_or(DEFAULT_PROJECT);
    let target = target.unwrap_or(DEFAULT_TARGET);

    let row = if let Some(hash) = build_hash {
        sqlx::query_as::<_, BuildRow>(
            r#"
            SELECT build_id, project, version, target, build_hash, file_manifest_hash, file_manifest_count,
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
            SELECT build_id, project, version, target, build_hash, file_manifest_hash, file_manifest_count,
                   file_manifest_json, includes_modules, source_repo, source_commit,
                   registered_at, registered_by, status, notes
            FROM builds
            WHERE project = $1 AND version = $2 AND target = $3 AND status = 'active'
            ORDER BY registered_at DESC
            LIMIT 1
            "#,
        )
        .bind(project)
        .bind(ver)
        .bind(target)
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
            SELECT build_id, project, version, target, build_hash, file_manifest_hash, file_manifest_count,
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
            SELECT build_id, project, version, target, build_hash, file_manifest_hash, file_manifest_count,
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
