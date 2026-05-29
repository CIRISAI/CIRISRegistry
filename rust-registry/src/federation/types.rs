//! Federation directory wire-format types.
//!
//! As of v1.2.0 (#33 Phase 2), Registry re-exports the upstream
//! `ciris_persist::federation::types` directly rather than carrying
//! a vendored copy. The vendored shapes inherited from FSD-002 v1.0
//! had drifted (single `pubkey_base64` field where upstream now
//! carries the hybrid `pubkey_ed25519_base64` + `pubkey_ml_dsa_65_base64`
//! split per CIRISPersist v0.2.0+; `time::OffsetDateTime` where
//! upstream now uses `chrono::DateTime<Utc>`).
//!
//! Single source of truth eliminates the wire-format-parity drift risk
//! the original vendored copy was designed to guard against (per the
//! comment in CIRISPersist's own `src/federation/mod.rs`: "Registry
//! hashes the vendored shape; persist hashes its own"). Drift is now
//! impossible because the shape *is* the upstream shape.

pub use ciris_persist::federation::types::{
    algorithm,
    attestation_type,
    identity_type,
    Attestation,
    KeyRecord,
    Revocation,
    SignedAttestation,
    SignedKeyRecord,
    SignedRevocation,
};

// The HybridPendingRow type is also part of the trait surface (see
// list_hybrid_pending_{keys,attestations,revocations}). Re-exported
// so consumers can name it without the full upstream path.
pub use ciris_persist::federation::types::HybridPendingRow;
