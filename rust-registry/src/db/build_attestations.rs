//! Build attestation database operations

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;
use crate::proto;

#[derive(Debug, Clone, FromRow)]
pub struct BuildAttestationRow {
    pub agent_hash: Vec<u8>,
    pub builder_id: String,
    pub invocation_id: Option<String>,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
    pub source_uri: Option<String>,
    pub source_commit: Option<String>,
    pub source_branch: Option<String>,
    pub build_commands: Option<Vec<String>>,
    pub expected_artifact_hash: Option<Vec<u8>>,
    pub reproducible_build_url: Option<String>,
    pub builder_os: Option<String>,
    pub builder_architecture: Option<String>,
    pub builder_env: Option<serde_json::Value>,
    // HybridSignature fields stored flattened
    pub signature_classical: Option<Vec<u8>>,
    pub signature_post_quantum: Option<Vec<u8>>,
    pub signature_timestamp: Option<i64>,
    pub signature_key_id: Option<String>,
    pub verification_count: i32,
    pub last_verified_at: Option<OffsetDateTime>,
}

impl BuildAttestationRow {
    pub fn to_proto(&self) -> proto::BuildAttestation {
        // Build the provenance sub-message
        let provenance = proto::BuildProvenance {
            builder_id: self.builder_id.clone(),
            invocation_id: self.invocation_id.clone().unwrap_or_default(),
            started_at: self.started_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            finished_at: self.finished_at.map(|t| t.unix_timestamp()).unwrap_or(0),
            source_uri: self.source_uri.clone().unwrap_or_default(),
            source_commit: self.source_commit.clone().unwrap_or_default(),
            source_branch: self.source_branch.clone().unwrap_or_default(),
            build_commands: self.build_commands.clone().unwrap_or_default(),
            expected_artifact_hash: self
                .expected_artifact_hash
                .clone()
                .map(|h| h.into())
                .unwrap_or_default(),
            reproducible_build_url: self.reproducible_build_url.clone().unwrap_or_default(),
            builder_os: self.builder_os.clone().unwrap_or_default(),
            builder_architecture: self.builder_architecture.clone().unwrap_or_default(),
            builder_env: self
                .builder_env
                .as_ref()
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        };

        // Build the builder signature sub-message
        let builder_signature = if self.signature_classical.is_some()
            || self.signature_post_quantum.is_some()
        {
            Some(proto::HybridSignature {
                classical_signature: self
                    .signature_classical
                    .clone()
                    .map(|s| s.into())
                    .unwrap_or_default(),
                post_quantum_signature: self
                    .signature_post_quantum
                    .clone()
                    .map(|s| s.into())
                    .unwrap_or_default(),
                timestamp: self.signature_timestamp.unwrap_or(0),
                key_id: self.signature_key_id.clone().unwrap_or_default(),
            })
        } else {
            None
        };

        proto::BuildAttestation {
            provenance: Some(provenance),
            builder_signature,
        }
    }
}

/// Register or update a build attestation
pub async fn register_attestation(
    pool: &PgPool,
    agent_hash: &[u8],
    attestation: &proto::BuildAttestation,
) -> Result<()> {
    // Extract provenance fields (with defaults if not present)
    let provenance = attestation.provenance.as_ref();
    let builder_id = provenance
        .map(|p| p.builder_id.as_str())
        .unwrap_or_default();
    let invocation_id = provenance.and_then(|p| {
        if p.invocation_id.is_empty() {
            None
        } else {
            Some(p.invocation_id.as_str())
        }
    });
    let started_at = provenance.map(|p| p.started_at).unwrap_or(0);
    let finished_at = provenance.map(|p| p.finished_at).unwrap_or(0);
    let source_uri = provenance.and_then(|p| {
        if p.source_uri.is_empty() {
            None
        } else {
            Some(p.source_uri.as_str())
        }
    });
    let source_commit = provenance.and_then(|p| {
        if p.source_commit.is_empty() {
            None
        } else {
            Some(p.source_commit.as_str())
        }
    });
    let source_branch = provenance.and_then(|p| {
        if p.source_branch.is_empty() {
            None
        } else {
            Some(p.source_branch.as_str())
        }
    });
    let build_commands: Option<Vec<String>> = provenance.and_then(|p| {
        if p.build_commands.is_empty() {
            None
        } else {
            Some(p.build_commands.clone())
        }
    });
    let expected_artifact_hash: Option<&[u8]> = provenance.and_then(|p| {
        if p.expected_artifact_hash.is_empty() {
            None
        } else {
            Some(p.expected_artifact_hash.as_ref())
        }
    });
    let reproducible_build_url = provenance.and_then(|p| {
        if p.reproducible_build_url.is_empty() {
            None
        } else {
            Some(p.reproducible_build_url.as_str())
        }
    });
    let builder_os = provenance.and_then(|p| {
        if p.builder_os.is_empty() {
            None
        } else {
            Some(p.builder_os.as_str())
        }
    });
    let builder_architecture = provenance.and_then(|p| {
        if p.builder_architecture.is_empty() {
            None
        } else {
            Some(p.builder_architecture.as_str())
        }
    });
    let builder_env: Option<serde_json::Value> = provenance.and_then(|p| {
        if p.builder_env.is_empty() {
            None
        } else {
            Some(serde_json::json!(p.builder_env))
        }
    });

    // Extract signature fields
    let sig = attestation.builder_signature.as_ref();
    let signature_classical: Option<&[u8]> = sig.and_then(|s| {
        if s.classical_signature.is_empty() {
            None
        } else {
            Some(s.classical_signature.as_ref())
        }
    });
    let signature_post_quantum: Option<&[u8]> = sig.and_then(|s| {
        if s.post_quantum_signature.is_empty() {
            None
        } else {
            Some(s.post_quantum_signature.as_ref())
        }
    });
    let signature_timestamp = sig.map(|s| s.timestamp);
    let signature_key_id = sig.and_then(|s| {
        if s.key_id.is_empty() {
            None
        } else {
            Some(s.key_id.as_str())
        }
    });

    sqlx::query(
        r#"
        INSERT INTO build_attestations (
            agent_hash, builder_id, invocation_id, started_at, finished_at,
            source_uri, source_commit, source_branch, build_commands,
            expected_artifact_hash, reproducible_build_url, builder_os,
            builder_architecture, builder_env,
            signature_classical, signature_post_quantum, signature_timestamp, signature_key_id
        )
        VALUES (
            $1, $2, $3, to_timestamp($4), to_timestamp($5),
            $6, $7, $8, $9,
            $10, $11, $12,
            $13, $14,
            $15, $16, $17, $18
        )
        ON CONFLICT (agent_hash) DO UPDATE SET
            builder_id = EXCLUDED.builder_id,
            invocation_id = EXCLUDED.invocation_id,
            started_at = EXCLUDED.started_at,
            finished_at = EXCLUDED.finished_at,
            source_uri = EXCLUDED.source_uri,
            source_commit = EXCLUDED.source_commit,
            source_branch = EXCLUDED.source_branch,
            build_commands = EXCLUDED.build_commands,
            expected_artifact_hash = EXCLUDED.expected_artifact_hash,
            reproducible_build_url = EXCLUDED.reproducible_build_url,
            builder_os = EXCLUDED.builder_os,
            builder_architecture = EXCLUDED.builder_architecture,
            builder_env = EXCLUDED.builder_env,
            signature_classical = EXCLUDED.signature_classical,
            signature_post_quantum = EXCLUDED.signature_post_quantum,
            signature_timestamp = EXCLUDED.signature_timestamp,
            signature_key_id = EXCLUDED.signature_key_id
        "#,
    )
    .bind(agent_hash)
    .bind(builder_id)
    .bind(invocation_id)
    .bind(if started_at > 0 { started_at as f64 } else { 0.0 })
    .bind(if finished_at > 0 {
        finished_at as f64
    } else {
        0.0
    })
    .bind(source_uri)
    .bind(source_commit)
    .bind(source_branch)
    .bind(&build_commands)
    .bind(expected_artifact_hash)
    .bind(reproducible_build_url)
    .bind(builder_os)
    .bind(builder_architecture)
    .bind(builder_env)
    .bind(signature_classical)
    .bind(signature_post_quantum)
    .bind(signature_timestamp)
    .bind(signature_key_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a build attestation by agent hash
pub async fn get_attestation(
    pool: &PgPool,
    agent_hash: &[u8],
) -> Result<Option<BuildAttestationRow>> {
    let row = sqlx::query_as::<_, BuildAttestationRow>(
        r#"
        SELECT agent_hash, builder_id, invocation_id, started_at, finished_at,
               source_uri, source_commit, source_branch, build_commands,
               expected_artifact_hash, reproducible_build_url, builder_os,
               builder_architecture, builder_env,
               signature_classical, signature_post_quantum, signature_timestamp, signature_key_id,
               verification_count, last_verified_at
        FROM build_attestations
        WHERE agent_hash = $1
        "#,
    )
    .bind(agent_hash)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Increment verification count for an attestation
pub async fn increment_verification_count(pool: &PgPool, agent_hash: &[u8]) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE build_attestations
        SET verification_count = verification_count + 1, last_verified_at = NOW()
        WHERE agent_hash = $1
        "#,
    )
    .bind(agent_hash)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
