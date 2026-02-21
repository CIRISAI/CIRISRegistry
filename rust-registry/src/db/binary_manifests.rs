//! Binary manifest database operations
//!
//! Stores SHA-256 hashes of CIRISVerify binaries for each platform/version.
//! Used by ciris-verify self-check to verify binary integrity (Level 2).

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;

#[derive(Debug, Clone, FromRow)]
pub struct BinaryManifestRow {
    pub manifest_id: sqlx::types::Uuid,
    pub version: String,
    pub binaries: serde_json::Value,
    pub generated_at: OffsetDateTime,
    pub registered_at: OffsetDateTime,
    pub registered_by: Option<String>,
    pub source: Option<String>,
    pub notes: Option<String>,
}

/// Response format for the binary manifest API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryManifestResponse {
    pub version: String,
    pub binaries: std::collections::HashMap<String, String>,
    pub generated_at: String,
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

        BinaryManifestResponse {
            version: self.version.clone(),
            binaries,
            generated_at: self
                .generated_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| self.generated_at.to_string()),
        }
    }
}

/// Get a binary manifest by version
pub async fn get_binary_manifest(
    pool: &PgPool,
    version: &str,
) -> Result<Option<BinaryManifestRow>> {
    let row = sqlx::query_as::<_, BinaryManifestRow>(
        r#"
        SELECT manifest_id, version, binaries, generated_at, registered_at,
               registered_by, source, notes
        FROM binary_manifests
        WHERE version = $1
        "#,
    )
    .bind(version)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Register a new binary manifest
pub async fn register_binary_manifest(
    pool: &PgPool,
    version: &str,
    binaries: &serde_json::Value,
    generated_at: OffsetDateTime,
    registered_by: Option<&str>,
    source: Option<&str>,
    notes: Option<&str>,
) -> Result<String> {
    let row: (sqlx::types::Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO binary_manifests (version, binaries, generated_at, registered_by, source, notes)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (version) DO UPDATE SET
            binaries = EXCLUDED.binaries,
            generated_at = EXCLUDED.generated_at,
            registered_by = EXCLUDED.registered_by,
            source = EXCLUDED.source,
            notes = EXCLUDED.notes,
            registered_at = NOW()
        RETURNING manifest_id
        "#,
    )
    .bind(version)
    .bind(binaries)
    .bind(generated_at)
    .bind(registered_by)
    .bind(source)
    .bind(notes)
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
        SELECT manifest_id, version, binaries, generated_at, registered_at,
               registered_by, source, notes
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
