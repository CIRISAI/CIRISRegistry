-- Migration 022: trusted_primitive_keys table for inbound BuildManifest validation
--
-- Stores the per-CIRIS-primitive Ed25519 + ML-DSA-65 public keys that the
-- registry uses to verify hybrid signatures on uploaded BuildManifests
-- (POST /v1/verify/binary-manifest, /v1/verify/function-manifest, gRPC
-- RegisterBuild). Closes THREAT_MODEL.md AV-26.
--
-- The registry's own steward Ed25519 + ML-DSA-65 pubkeys are auto-seeded
-- at boot (project='ciris-registry') so the registry's own builds can be
-- self-verified without manual setup.
--
-- Other primitives (ciris-persist, ciris-lens, ciris-agent, ciris-verify)
-- must be registered via the new admin RPC RegisterTrustedPrimitiveKey
-- by a SYSTEM_ADMIN before their builds will pass inbound verification.

CREATE TABLE IF NOT EXISTS trusted_primitive_keys (
    project              TEXT PRIMARY KEY,
    ed25519_public_key   BYTEA NOT NULL,
    ml_dsa_65_public_key BYTEA NOT NULL,
    ed25519_fingerprint  TEXT NOT NULL,
    ml_dsa_65_fingerprint TEXT NOT NULL,
    added_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by             TEXT,
    rotated_at           TIMESTAMPTZ,
    revoked_at           TIMESTAMPTZ,
    revocation_reason    TEXT,
    notes                TEXT,
    CONSTRAINT trusted_primitive_keys_ed25519_len CHECK (octet_length(ed25519_public_key) = 32),
    CONSTRAINT trusted_primitive_keys_mldsa_len   CHECK (octet_length(ml_dsa_65_public_key) > 1500)
);

CREATE INDEX IF NOT EXISTS idx_trusted_primitive_keys_active
    ON trusted_primitive_keys (project)
    WHERE revoked_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_trusted_primitive_keys_ed25519_fp
    ON trusted_primitive_keys (ed25519_fingerprint);

COMMENT ON TABLE  trusted_primitive_keys IS 'Per-primitive trusted public keys for BuildManifest verify path (AV-26 mitigation).';
COMMENT ON COLUMN trusted_primitive_keys.project IS 'CIRIS primitive name (kebab-case). Matches builds.project / BuildPrimitive::project_name().';
COMMENT ON COLUMN trusted_primitive_keys.revoked_at IS 'When set, lookups skip this row (revoked keys cannot verify uploads).';
