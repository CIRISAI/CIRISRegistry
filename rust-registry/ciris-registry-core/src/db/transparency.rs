//! Transparency-log witness directory + STH cosignatures.
//!
//! v1.4 interim per CIRISRegistry#24 Ask 3 + FSD-002 §7.8. Substrate-
//! conformance migration (CIRISRegistry#17 + CIRISPersist#102 witness
//! vocabulary extension) will replace the per-region storage here with
//! cross-region federation_keys (identity_type=witness) +
//! federation_attestations (transparency_log:cosigned:{tree_size}).

use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

use crate::error::Result;

#[derive(Debug, Clone, FromRow)]
pub struct WitnessRow {
    pub witness_key_id: String,
    pub ed25519_pubkey: Vec<u8>,
    pub mldsa65_pubkey: Vec<u8>,
    pub fingerprint: String,
    pub hardware_class: String,
    pub trusted_since: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CosignatureRow {
    pub tree_size: i64,
    pub root_hash: Vec<u8>,
    pub witness_key_id: String,
    pub ed25519_signature: Vec<u8>,
    pub mldsa65_signature: Vec<u8>,
    pub signed_at: OffsetDateTime,
    pub cosigned_at: OffsetDateTime,
}

/// List all active (non-revoked) witnesses for the directory endpoint.
pub async fn list_witnesses(pool: &PgPool) -> Result<Vec<WitnessRow>> {
    let rows = sqlx::query_as::<_, WitnessRow>(
        r#"
        SELECT witness_key_id, ed25519_pubkey, mldsa65_pubkey, fingerprint,
               hardware_class, trusted_since, revoked_at
        FROM registry_witnesses
        WHERE revoked_at IS NULL
        ORDER BY trusted_since DESC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Lookup a witness for signature verification.
pub async fn lookup_witness(pool: &PgPool, witness_key_id: &str) -> Result<Option<WitnessRow>> {
    let row = sqlx::query_as::<_, WitnessRow>(
        r#"
        SELECT witness_key_id, ed25519_pubkey, mldsa65_pubkey, fingerprint,
               hardware_class, trusted_since, revoked_at
        FROM registry_witnesses
        WHERE witness_key_id = $1
          AND revoked_at IS NULL
        "#,
    )
    .bind(witness_key_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Register a new witness (admin operation per CIRISRegistry#24).
#[allow(clippy::too_many_arguments)]
pub async fn register_witness(
    pool: &PgPool,
    witness_key_id: &str,
    ed25519_pubkey: &[u8],
    mldsa65_pubkey: &[u8],
    fingerprint: &str,
    hardware_class: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO registry_witnesses (
            witness_key_id, ed25519_pubkey, mldsa65_pubkey,
            fingerprint, hardware_class
        )
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (witness_key_id) DO UPDATE SET
            ed25519_pubkey = EXCLUDED.ed25519_pubkey,
            mldsa65_pubkey = EXCLUDED.mldsa65_pubkey,
            fingerprint    = EXCLUDED.fingerprint,
            hardware_class = EXCLUDED.hardware_class,
            revoked_at     = NULL
        "#,
    )
    .bind(witness_key_id)
    .bind(ed25519_pubkey)
    .bind(mldsa65_pubkey)
    .bind(fingerprint)
    .bind(hardware_class)
    .execute(pool)
    .await?;
    Ok(())
}

/// Record a witness cosignature on an STH. Idempotent on
/// (tree_size, witness_key_id) primary key.
#[allow(clippy::too_many_arguments)]
pub async fn record_cosignature(
    pool: &PgPool,
    tree_size: i64,
    root_hash: &[u8],
    witness_key_id: &str,
    ed25519_signature: &[u8],
    mldsa65_signature: &[u8],
    signed_at: OffsetDateTime,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO registry_sth_cosignatures (
            tree_size, root_hash, witness_key_id,
            ed25519_signature, mldsa65_signature, signed_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (tree_size, witness_key_id) DO UPDATE SET
            root_hash         = EXCLUDED.root_hash,
            ed25519_signature = EXCLUDED.ed25519_signature,
            mldsa65_signature = EXCLUDED.mldsa65_signature,
            signed_at         = EXCLUDED.signed_at,
            cosigned_at       = NOW()
        "#,
    )
    .bind(tree_size)
    .bind(root_hash)
    .bind(witness_key_id)
    .bind(ed25519_signature)
    .bind(mldsa65_signature)
    .bind(signed_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// Fetch all cosignatures for a given STH tree size.
pub async fn list_cosignatures_for_sth(
    pool: &PgPool,
    tree_size: i64,
) -> Result<Vec<CosignatureRow>> {
    let rows = sqlx::query_as::<_, CosignatureRow>(
        r#"
        SELECT tree_size, root_hash, witness_key_id,
               ed25519_signature, mldsa65_signature, signed_at, cosigned_at
        FROM registry_sth_cosignatures
        WHERE tree_size = $1
        ORDER BY cosigned_at ASC
        "#,
    )
    .bind(tree_size)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// The most-recent (highest `tree_size`) STH this witness has previously
/// cosigned, as `(tree_size, root_hash)`. `None` if the witness has never
/// cosigned (the "from genesis" case in CEG 0.2 §10.3.1). Used by the
/// cosign admission gate to anchor the consistency-proof check against
/// what the Registry itself recorded last time — NOT against a root the
/// requester claims.
pub async fn latest_cosignature_by_witness(
    pool: &PgPool,
    witness_key_id: &str,
) -> Result<Option<(i64, Vec<u8>)>> {
    let row: Option<(i64, Vec<u8>)> = sqlx::query_as(
        r#"
        SELECT tree_size, root_hash
        FROM registry_sth_cosignatures
        WHERE witness_key_id = $1
        ORDER BY tree_size DESC
        LIMIT 1
        "#,
    )
    .bind(witness_key_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
