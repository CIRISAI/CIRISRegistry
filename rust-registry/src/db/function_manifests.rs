//! Function manifest database operations
//!
//! Stores function-level integrity manifests for CIRISVerify.
//! Used for runtime verification of individual FFI exports.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;

#[derive(Debug, Clone, FromRow)]
pub struct FunctionManifestRow {
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

/// Get a function manifest by version and target
pub async fn get_function_manifest(
    pool: &PgPool,
    binary_version: &str,
    target: &str,
) -> Result<Option<FunctionManifestRow>> {
    let row = sqlx::query_as::<_, FunctionManifestRow>(
        r#"
        SELECT binary_version, target, manifest_version, binary_hash, manifest_hash,
               manifest_json, signature_classical, signature_pqc, signature_key_id,
               generated_at, created_at
        FROM function_manifests
        WHERE binary_version = $1 AND target = $2
        "#,
    )
    .bind(binary_version)
    .bind(target)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// List available targets for a version
pub async fn list_function_manifest_targets(
    pool: &PgPool,
    binary_version: &str,
) -> Result<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT target FROM function_manifests
        WHERE binary_version = $1
        ORDER BY target
        "#,
    )
    .bind(binary_version)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(t,)| t).collect())
}

/// Register a new function manifest
/// Returns the manifest_hash on success
pub async fn register_function_manifest(
    pool: &PgPool,
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
) -> Result<String> {
    let row: (String,) = sqlx::query_as(
        r#"
        INSERT INTO function_manifests (
            binary_version, target, manifest_version, binary_hash, manifest_hash,
            manifest_json, signature_classical, signature_pqc, signature_key_id, generated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (binary_version, target) DO UPDATE SET
            manifest_version = EXCLUDED.manifest_version,
            binary_hash = EXCLUDED.binary_hash,
            manifest_hash = EXCLUDED.manifest_hash,
            manifest_json = EXCLUDED.manifest_json,
            signature_classical = EXCLUDED.signature_classical,
            signature_pqc = EXCLUDED.signature_pqc,
            signature_key_id = EXCLUDED.signature_key_id,
            generated_at = EXCLUDED.generated_at,
            created_at = NOW()
        RETURNING manifest_hash
        "#,
    )
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
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// List all function manifests (admin)
pub async fn list_function_manifests(
    pool: &PgPool,
    page_size: i32,
) -> Result<Vec<FunctionManifestRow>> {
    let limit = if page_size > 0 { page_size } else { 50 };

    let rows = sqlx::query_as::<_, FunctionManifestRow>(
        r#"
        SELECT binary_version, target, manifest_version, binary_hash, manifest_hash,
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
