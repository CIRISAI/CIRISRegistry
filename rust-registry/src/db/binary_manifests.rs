//! Binary manifest database operations
//!
//! Stores SHA-256 hashes of CIRISVerify binaries for each platform/version.
//! Used by ciris-verify self-check to verify binary integrity (Level 2).

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;

/// Default project name for manifests where the caller did not specify one.
pub const DEFAULT_PROJECT: &str = "ciris-agent";

#[derive(Debug, Clone, FromRow)]
pub struct BinaryManifestRow {
    pub manifest_id: sqlx::types::Uuid,
    pub project: String,
    pub version: String,
    pub binaries: serde_json::Value,
    pub generated_at: OffsetDateTime,
    pub registered_at: OffsetDateTime,
    pub registered_by: Option<String>,
    pub source: Option<String>,
    pub notes: Option<String>,
    pub signature_classical: Option<String>,
    pub signature_pqc: Option<String>,
    pub signature_key_id: Option<String>,
}

/// Response format for the binary manifest API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryManifestResponse {
    pub version: String,
    pub binaries: std::collections::HashMap<String, String>,
    pub generated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<BinaryManifestSignature>,
}

/// Signature for binary manifest (steward key)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryManifestSignature {
    pub classical: String,
    pub classical_algorithm: String,
    pub pqc: String,
    pub pqc_algorithm: String,
    pub key_id: String,
}

impl BinaryManifestRow {
    pub fn to_response(&self) -> BinaryManifestResponse {
        use time::format_description::well_known::Rfc3339;

        let binaries: std::collections::HashMap<String, String> = self
            .binaries
            .as_object()
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let signature = if let (Some(classical), Some(pqc), Some(key_id)) = (
            &self.signature_classical,
            &self.signature_pqc,
            &self.signature_key_id,
        ) {
            Some(BinaryManifestSignature {
                classical: classical.clone(),
                classical_algorithm: "Ed25519".to_string(),
                pqc: pqc.clone(),
                pqc_algorithm: "ML-DSA-65".to_string(),
                key_id: key_id.clone(),
            })
        } else {
            None
        };

        BinaryManifestResponse {
            version: self.version.clone(),
            binaries,
            generated_at: self
                .generated_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| self.generated_at.to_string()),
            signature,
        }
    }
}

/// Get a binary manifest by project + version. `project=None` defaults to
/// `ciris-agent` for backwards compat with pre-v1.4.0 callers.
pub async fn get_binary_manifest(
    pool: &PgPool,
    version: &str,
    project: Option<&str>,
) -> Result<Option<BinaryManifestRow>> {
    let project = project.unwrap_or(DEFAULT_PROJECT);

    let row = sqlx::query_as::<_, BinaryManifestRow>(
        r#"
        SELECT manifest_id, project, version, binaries, generated_at, registered_at,
               registered_by, source, notes,
               signature_classical, signature_pqc, signature_key_id
        FROM binary_manifests
        WHERE project = $1 AND version = $2
        "#,
    )
    .bind(project)
    .bind(version)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Register a new binary manifest with optional signature.
/// `project` defaults to `ciris-agent` when empty.
#[allow(clippy::too_many_arguments)]
pub async fn register_binary_manifest(
    pool: &PgPool,
    project: &str,
    version: &str,
    binaries: &serde_json::Value,
    generated_at: OffsetDateTime,
    registered_by: Option<&str>,
    source: Option<&str>,
    notes: Option<&str>,
    signature_classical: Option<&str>,
    signature_pqc: Option<&str>,
    signature_key_id: Option<&str>,
) -> Result<String> {
    let project = if project.is_empty() {
        DEFAULT_PROJECT
    } else {
        project
    };

    let row: (sqlx::types::Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO binary_manifests (
            project, version, binaries, generated_at, registered_by, source, notes,
            signature_classical, signature_pqc, signature_key_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (project, version) DO UPDATE SET
            binaries = EXCLUDED.binaries,
            generated_at = EXCLUDED.generated_at,
            registered_by = EXCLUDED.registered_by,
            source = EXCLUDED.source,
            notes = EXCLUDED.notes,
            signature_classical = EXCLUDED.signature_classical,
            signature_pqc = EXCLUDED.signature_pqc,
            signature_key_id = EXCLUDED.signature_key_id,
            registered_at = NOW()
        RETURNING manifest_id
        "#,
    )
    .bind(project)
    .bind(version)
    .bind(binaries)
    .bind(generated_at)
    .bind(registered_by)
    .bind(source)
    .bind(notes)
    .bind(signature_classical)
    .bind(signature_pqc)
    .bind(signature_key_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0.to_string())
}

/// List all binary manifests
pub async fn list_binary_manifests(
    pool: &PgPool,
    page_size: i32,
) -> Result<Vec<BinaryManifestRow>> {
    let limit = if page_size > 0 { page_size } else { 50 };

    let rows = sqlx::query_as::<_, BinaryManifestRow>(
        r#"
        SELECT manifest_id, project, version, binaries, generated_at, registered_at,
               registered_by, source, notes,
               signature_classical, signature_pqc, signature_key_id
        FROM binary_manifests
        ORDER BY registered_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
