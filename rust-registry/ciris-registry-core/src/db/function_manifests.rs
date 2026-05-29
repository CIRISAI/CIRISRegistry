//! Function manifest database operations
//!
//! Stores function-level integrity manifests for CIRISVerify.
//! Used for runtime verification of individual FFI exports.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;

/// Default project name for function manifests where the caller did not specify one.
pub const DEFAULT_PROJECT: &str = "ciris-agent";

#[derive(Debug, Clone, FromRow)]
pub struct FunctionManifestRow {
    pub project: String,
    pub binary_version: String,
    pub target: String,
    pub manifest_version: String,
    pub binary_hash: String,
    pub manifest_hash: String,
    pub manifest_json: serde_json::Value,
    pub signature_classical: Option<String>,
    pub signature_pqc: Option<String>,
    pub signature_key_id: Option<String>,
    pub generated_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

/// Full function manifest response (matches CIRISVerify expected format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionManifestResponse {
    pub version: String,
    pub target: String,
    pub binary_hash: String,
    pub binary_version: String,
    pub generated_at: String,
    pub functions: serde_json::Value,
    pub manifest_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<ManifestSignature>,
    /// Metadata containing text_section_offset for address calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSignature {
    pub classical: String,
    pub classical_algorithm: String,
    pub pqc: String,
    pub pqc_algorithm: String,
    pub key_id: String,
}

/// List of available targets for a version
#[derive(Debug, Clone, Serialize)]
pub struct AvailableTargetsResponse {
    pub version: String,
    pub targets: Vec<String>,
}

impl FunctionManifestRow {
    pub fn to_response(&self) -> FunctionManifestResponse {
        use time::format_description::well_known::Rfc3339;

        let functions = self
            .manifest_json
            .get("functions")
            .cloned()
            .unwrap_or(serde_json::json!({}));

        // Extract metadata (contains text_section_offset for address calculation)
        let metadata = self.manifest_json.get("metadata").cloned();

        let signature = if let (Some(classical), Some(pqc), Some(key_id)) = (
            &self.signature_classical,
            &self.signature_pqc,
            &self.signature_key_id,
        ) {
            Some(ManifestSignature {
                classical: classical.clone(),
                classical_algorithm: "Ed25519".to_string(),
                pqc: pqc.clone(),
                pqc_algorithm: "ML-DSA-65".to_string(),
                key_id: key_id.clone(),
            })
        } else {
            None
        };

        FunctionManifestResponse {
            version: self.manifest_version.clone(),
            target: self.target.clone(),
            binary_hash: self.binary_hash.clone(),
            binary_version: self.binary_version.clone(),
            generated_at: self
                .generated_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| self.generated_at.to_string()),
            functions,
            manifest_hash: self.manifest_hash.clone(),
            signature,
            metadata,
        }
    }
}

/// Get a function manifest by project + version + target.
/// `project=None` defaults to `ciris-agent` for backwards compat.
pub async fn get_function_manifest(
    pool: &PgPool,
    binary_version: &str,
    target: &str,
    project: Option<&str>,
) -> Result<Option<FunctionManifestRow>> {
    let project = project.unwrap_or(DEFAULT_PROJECT);

    let row = sqlx::query_as::<_, FunctionManifestRow>(
        r#"
        SELECT project, binary_version, target, manifest_version, binary_hash, manifest_hash,
               manifest_json, signature_classical, signature_pqc, signature_key_id,
               generated_at, created_at
        FROM function_manifests
        WHERE project = $1 AND binary_version = $2 AND target = $3
        "#,
    )
    .bind(project)
    .bind(binary_version)
    .bind(target)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// List available targets for a project + version.
pub async fn list_function_manifest_targets(
    pool: &PgPool,
    binary_version: &str,
    project: Option<&str>,
) -> Result<Vec<String>> {
    let project = project.unwrap_or(DEFAULT_PROJECT);

    let rows: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT target FROM function_manifests
        WHERE project = $1 AND binary_version = $2
        ORDER BY target
        "#,
    )
    .bind(project)
    .bind(binary_version)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(t,)| t).collect())
}

/// Register a new function manifest. Returns the manifest_hash on success.
/// `project` defaults to `ciris-agent` when empty.
///
/// `raw_manifest_body` is the verbatim POST body for rows POSTed via
/// `/v1/verify/build-manifest` (Path B fidelity, CIRISRegistry#5 §2).
/// Pass `None` from legacy callers that don't have the raw bytes —
/// the column stays NULL and Path B's GET will 404 for that row.
#[allow(clippy::too_many_arguments)]
pub async fn register_function_manifest(
    pool: &PgPool,
    project: &str,
    binary_version: &str,
    target: &str,
    manifest_version: &str,
    binary_hash: &str,
    manifest_hash: &str,
    manifest_json: &serde_json::Value,
    signature_classical: Option<&str>,
    signature_pqc: Option<&str>,
    signature_key_id: Option<&str>,
    generated_at: OffsetDateTime,
    raw_manifest_body: Option<&[u8]>,
) -> Result<String> {
    let project = if project.is_empty() {
        DEFAULT_PROJECT
    } else {
        project
    };

    let row: (String,) = sqlx::query_as(
        r#"
        INSERT INTO function_manifests (
            project, binary_version, target, manifest_version, binary_hash, manifest_hash,
            manifest_json, signature_classical, signature_pqc, signature_key_id, generated_at,
            raw_manifest_body
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ON CONFLICT (project, binary_version, target) DO UPDATE SET
            manifest_version = EXCLUDED.manifest_version,
            binary_hash = EXCLUDED.binary_hash,
            manifest_hash = EXCLUDED.manifest_hash,
            manifest_json = EXCLUDED.manifest_json,
            signature_classical = EXCLUDED.signature_classical,
            signature_pqc = EXCLUDED.signature_pqc,
            signature_key_id = EXCLUDED.signature_key_id,
            generated_at = EXCLUDED.generated_at,
            raw_manifest_body = COALESCE(EXCLUDED.raw_manifest_body, function_manifests.raw_manifest_body),
            created_at = NOW()
        RETURNING manifest_hash
        "#,
    )
    .bind(project)
    .bind(binary_version)
    .bind(target)
    .bind(manifest_version)
    .bind(binary_hash)
    .bind(manifest_hash)
    .bind(manifest_json)
    .bind(signature_classical)
    .bind(signature_pqc)
    .bind(signature_key_id)
    .bind(generated_at)
    .bind(raw_manifest_body)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Fetch the verbatim raw POST body of a previously-stored BuildManifest.
/// Returns `None` if the row exists but was POSTed via the legacy
/// `/v1/verify/function-manifest` endpoint (no raw body captured), or
/// if the row doesn't exist at all. Callers distinguish by checking
/// row existence separately if they need to. Backs Path B
/// (CIRISRegistry#5 §2).
pub async fn get_function_manifest_raw_body(
    pool: &PgPool,
    project: &str,
    binary_version: &str,
    target: &str,
) -> Result<Option<Vec<u8>>> {
    let row: Option<(Option<Vec<u8>>,)> = sqlx::query_as(
        r#"
        SELECT raw_manifest_body
        FROM function_manifests
        WHERE project = $1 AND binary_version = $2 AND target = $3
        "#,
    )
    .bind(project)
    .bind(binary_version)
    .bind(target)
    .fetch_optional(pool)
    .await?;

    Ok(row.and_then(|(body,)| body))
}

/// List all function manifests (admin)
pub async fn list_function_manifests(
    pool: &PgPool,
    page_size: i32,
) -> Result<Vec<FunctionManifestRow>> {
    let limit = if page_size > 0 { page_size } else { 50 };

    let rows = sqlx::query_as::<_, FunctionManifestRow>(
        r#"
        SELECT project, binary_version, target, manifest_version, binary_hash, manifest_hash,
               manifest_json, signature_classical, signature_pqc, signature_key_id,
               generated_at, created_at
        FROM function_manifests
        ORDER BY created_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
