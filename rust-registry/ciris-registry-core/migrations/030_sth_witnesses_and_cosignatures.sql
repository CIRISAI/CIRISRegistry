-- Migration 030: STH witness directory + cosignatures (v1.4.1+ CIRISRegistry#24 Ask 3)
--
-- Closes:
--   - GitHub issue CIRISAI/CIRISRegistry#24 Ask 3 (STH witness cosigning endpoints).
--
-- Background. CIRISVerify v2.12.0+ shipped `SignedTreeHead::cosign` +
-- `TrustedWitness` + `count_valid_witnesses` + `witness_quorum_met`
-- (verify-side receivers ready; emission half blocked on Registry).
-- FSD-002 v1.4.3 §7.8 pins the three endpoint shapes:
--
--   POST /v1/transparency/sth/cosign
--   GET  /v1/transparency/witnesses
--   GET  /v1/transparency/sth/{tree_size}/witnesses
--
-- Plus an admin-token-gated POST /v1/transparency/witnesses for v1.4
-- interim witness registration (substrate-conformance CIRISRegistry#17 +
-- CIRISPersist#102 `identity_type="witness"` vocabulary extension will
-- supersede this with federation_keys lookups later).
--
-- Two tables:
--
--   registry_witnesses
--     - witness_key_id            TEXT PRIMARY KEY
--     - ed25519_pubkey            BYTEA NOT NULL (32 bytes)
--     - mldsa65_pubkey            BYTEA NOT NULL (1952 bytes)
--     - fingerprint               TEXT NOT NULL UNIQUE
--     - hardware_class            TEXT NOT NULL DEFAULT 'placeholder_pending_provisioning'
--     - trusted_since             TIMESTAMPTZ NOT NULL DEFAULT NOW()
--     - revoked_at                TIMESTAMPTZ NULLABLE
--
--   registry_sth_cosignatures
--     - tree_size                 BIGINT
--     - root_hash                 BYTEA NOT NULL (32 bytes)
--     - witness_key_id            TEXT NOT NULL → registry_witnesses(witness_key_id)
--     - ed25519_signature         BYTEA NOT NULL
--     - mldsa65_signature         BYTEA NOT NULL
--     - signed_at                 TIMESTAMPTZ NOT NULL
--     - cosigned_at               TIMESTAMPTZ NOT NULL DEFAULT NOW()
--     - PRIMARY KEY (tree_size, witness_key_id)
--     - INDEX on (tree_size) for the GET endpoint
--
-- Replication: both tables are per-region for v1.4 interim (witnesses
-- and cosignatures are operational state, not federation-wide trust
-- anchors yet). Substrate-conformance migration moves the witness
-- directory to federation_keys (cross-region) per CIRISPersist#102
-- extension comment + this issue's Persist dependency.
--
-- Idempotency: ALL tables/indexes use IF NOT EXISTS. Safe to re-run.

CREATE TABLE IF NOT EXISTS registry_witnesses (
    witness_key_id     TEXT PRIMARY KEY,
    ed25519_pubkey     BYTEA NOT NULL,
    mldsa65_pubkey     BYTEA NOT NULL,
    fingerprint        TEXT NOT NULL UNIQUE,
    hardware_class     TEXT NOT NULL DEFAULT 'placeholder_pending_provisioning',
    trusted_since      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at         TIMESTAMPTZ
);

COMMENT ON TABLE registry_witnesses IS
    'RFC-6962-style transparency-log witness directory. v1.4 interim per CIRISRegistry#24 Ask 3; substrate-conformance moves to federation_keys with identity_type=witness per CIRISPersist#102 vocabulary extension.';

CREATE TABLE IF NOT EXISTS registry_sth_cosignatures (
    tree_size          BIGINT NOT NULL,
    root_hash          BYTEA NOT NULL,
    witness_key_id     TEXT NOT NULL REFERENCES registry_witnesses(witness_key_id),
    ed25519_signature  BYTEA NOT NULL,
    mldsa65_signature  BYTEA NOT NULL,
    signed_at          TIMESTAMPTZ NOT NULL,
    cosigned_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tree_size, witness_key_id)
);

COMMENT ON TABLE registry_sth_cosignatures IS
    'Witness cosignatures on Signed Tree Heads. CIRISVerify v2.12.0+ consumes via SignedTreeHead::cosign + count_valid_witnesses + witness_quorum_met. Storage is per-region in v1.4 interim; substrate-conformance migration moves to federation_attestations as transparency_log:cosigned:{tree_size} scores attestations.';

CREATE INDEX IF NOT EXISTS idx_sth_cosignatures_tree_size
    ON registry_sth_cosignatures(tree_size);
