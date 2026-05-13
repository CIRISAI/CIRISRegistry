//! Per-primitive trusted public keys for inbound BuildManifest validation.
//!
//! Backs the AV-26 mitigation. Lookups are by `project` (the CIRIS
//! primitive name, e.g., `ciris-persist`). The registry's own steward
//! pubkey is seeded at boot as `project='ciris-registry'` so the
//! registry can self-verify its own builds without manual setup.

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;

/// Active trusted-primitive-key row (revoked rows excluded).
#[derive(Debug, Clone, FromRow)]
pub struct TrustedPrimitiveKey {
    pub project: String,
    pub ed25519_public_key: Vec<u8>,
    pub ml_dsa_65_public_key: Vec<u8>,
    pub ed25519_fingerprint: String,
    pub ml_dsa_65_fingerprint: String,
    pub added_at: OffsetDateTime,
    pub added_by: Option<String>,
    pub rotated_at: Option<OffsetDateTime>,
    pub revoked_at: Option<OffsetDateTime>,
    pub revocation_reason: Option<String>,
    pub notes: Option<String>,
}

/// Look up the active trusted key for a project. Returns `None` if no
/// row exists, or if the row is revoked.
pub async fn get_trusted_primitive_key(
    pool: &PgPool,
    project: &str,
) -> Result<Option<TrustedPrimitiveKey>> {
    let row = sqlx::query_as::<_, TrustedPrimitiveKey>(
        r#"
        SELECT project, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint,
               added_at, added_by, rotated_at, revoked_at,
               revocation_reason, notes
        FROM trusted_primitive_keys
        WHERE project = $1 AND revoked_at IS NULL
        "#,
    )
    .bind(project)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Insert a trusted key only if no row exists for this project. Used
/// by the boot-seed in `main.rs` to bootstrap the `ciris-registry`
/// trusted key on a fresh install WITHOUT overwriting an operator-set
/// or CI-published key on every restart.
///
/// Returns `true` if a row was inserted, `false` if a row already
/// existed (no-op).
pub async fn insert_trusted_primitive_key_if_absent(
    pool: &PgPool,
    project: &str,
    ed25519_pk: &[u8],
    mldsa_pk: &[u8],
    ed25519_fp: &str,
    mldsa_fp: &str,
    added_by: Option<&str>,
    notes: Option<&str>,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        INSERT INTO trusted_primitive_keys (
            project, ed25519_public_key, ml_dsa_65_public_key,
            ed25519_fingerprint, ml_dsa_65_fingerprint, added_by, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (project) DO NOTHING
        "#,
    )
    .bind(project)
    .bind(ed25519_pk)
    .bind(mldsa_pk)
    .bind(ed25519_fp)
    .bind(mldsa_fp)
    .bind(added_by)
    .bind(notes)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// Register a new trusted key. UPSERTs on conflict — call from the
/// admin RPC handler, which is responsible for SYSTEM_ADMIN gating.
/// Use `rotate_trusted_primitive_key` for rotation rather than calling
/// this twice (rotation tracks the timestamp).
///
/// `rotated_at` is only bumped when the actual key bytes change. An
/// idempotent re-registration with identical bytes preserves the
/// existing timestamp so consumers reading `rotated_at` can distinguish
/// real rotations from no-op admin replays. Closes CIRISRegistry#7.
pub async fn upsert_trusted_primitive_key(
    pool: &PgPool,
    project: &str,
    ed25519_pk: &[u8],
    mldsa_pk: &[u8],
    ed25519_fp: &str,
    mldsa_fp: &str,
    added_by: Option<&str>,
    notes: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO trusted_primitive_keys (
            project, ed25519_public_key, ml_dsa_65_public_key,
            ed25519_fingerprint, ml_dsa_65_fingerprint, added_by, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (project) DO UPDATE SET
            ed25519_public_key   = EXCLUDED.ed25519_public_key,
            ml_dsa_65_public_key = EXCLUDED.ml_dsa_65_public_key,
            ed25519_fingerprint  = EXCLUDED.ed25519_fingerprint,
            ml_dsa_65_fingerprint = EXCLUDED.ml_dsa_65_fingerprint,
            added_by             = EXCLUDED.added_by,
            notes                = EXCLUDED.notes,
            rotated_at           = CASE
                WHEN trusted_primitive_keys.ed25519_public_key   IS DISTINCT FROM EXCLUDED.ed25519_public_key
                  OR trusted_primitive_keys.ml_dsa_65_public_key IS DISTINCT FROM EXCLUDED.ml_dsa_65_public_key
                THEN NOW()
                ELSE trusted_primitive_keys.rotated_at
            END,
            revoked_at           = NULL,
            revocation_reason    = NULL
        "#,
    )
    .bind(project)
    .bind(ed25519_pk)
    .bind(mldsa_pk)
    .bind(ed25519_fp)
    .bind(mldsa_fp)
    .bind(added_by)
    .bind(notes)
    .execute(pool)
    .await?;
    Ok(())
}

/// List all active (non-revoked) trusted keys.
pub async fn list_trusted_primitive_keys(pool: &PgPool) -> Result<Vec<TrustedPrimitiveKey>> {
    let rows = sqlx::query_as::<_, TrustedPrimitiveKey>(
        r#"
        SELECT project, ed25519_public_key, ml_dsa_65_public_key,
               ed25519_fingerprint, ml_dsa_65_fingerprint,
               added_at, added_by, rotated_at, revoked_at,
               revocation_reason, notes
        FROM trusted_primitive_keys
        WHERE revoked_at IS NULL
        ORDER BY project
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Revoke a trusted key. Subsequent lookups return None until a new key
/// is registered for the same project (which clears the revocation via
/// upsert).
pub async fn revoke_trusted_primitive_key(
    pool: &PgPool,
    project: &str,
    reason: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE trusted_primitive_keys
        SET revoked_at = NOW(), revocation_reason = $2
        WHERE project = $1 AND revoked_at IS NULL
        "#,
    )
    .bind(project)
    .bind(reason)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
