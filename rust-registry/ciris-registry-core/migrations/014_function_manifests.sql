-- Migration 014: Function manifests for CIRISVerify function-level integrity
-- Stores SHA-256 hashes of individual exported functions for runtime verification.
-- Used by CIRISVerify to validate function integrity at load time.

CREATE TABLE IF NOT EXISTS function_manifests (
    id SERIAL PRIMARY KEY,
    binary_version TEXT NOT NULL,                     -- CIRISVerify version e.g. "0.5.4"
    target TEXT NOT NULL,                             -- Rust target triple e.g. "x86_64-unknown-linux-gnu"
    manifest_version TEXT NOT NULL DEFAULT '1.0.0',   -- Manifest format version
    binary_hash TEXT NOT NULL,                        -- SHA-256 of the binary file
    manifest_hash TEXT NOT NULL,                      -- SHA-256 of the manifest JSON
    manifest_json JSONB NOT NULL,                     -- Full manifest including functions
    signature_classical TEXT,                         -- Ed25519 signature (base64)
    signature_pqc TEXT,                               -- ML-DSA-65 signature (base64)
    signature_key_id TEXT,                            -- Signing key ID
    generated_at TIMESTAMPTZ NOT NULL,                -- When manifest was generated
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(binary_version, target)
);

CREATE INDEX IF NOT EXISTS idx_function_manifests_version ON function_manifests(binary_version);
CREATE INDEX IF NOT EXISTS idx_function_manifests_target ON function_manifests(target);
CREATE INDEX IF NOT EXISTS idx_function_manifests_hash ON function_manifests(manifest_hash);

-- Example manifest_json structure:
-- {
--   "version": "1.0.0",
--   "target": "x86_64-unknown-linux-gnu",
--   "binary_hash": "sha256:...",
--   "binary_version": "0.5.4",
--   "generated_at": "2026-02-21T02:51:03Z",
--   "functions": {
--     "ciris_verify_init": {
--       "name": "ciris_verify_init",
--       "offset": 2401440,
--       "size": 22297,
--       "hash": "sha256:..."
--     }
--   }
-- }
