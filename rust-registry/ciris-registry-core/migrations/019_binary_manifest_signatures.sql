-- Migration 019: Add signature columns to binary_manifests
-- Implements registry-side signing for CI-pushed manifests.
-- The steward key signs manifests upon registration.

ALTER TABLE binary_manifests
ADD COLUMN IF NOT EXISTS signature_classical TEXT,      -- Ed25519 signature (base64)
ADD COLUMN IF NOT EXISTS signature_pqc TEXT,            -- ML-DSA-65 signature (base64)
ADD COLUMN IF NOT EXISTS signature_key_id TEXT;         -- Signing key ID (fingerprint)

COMMENT ON COLUMN binary_manifests.signature_classical IS 'Ed25519 signature of manifest hash (base64)';
COMMENT ON COLUMN binary_manifests.signature_pqc IS 'ML-DSA-65 signature of manifest hash (base64)';
COMMENT ON COLUMN binary_manifests.signature_key_id IS 'Steward key ID that signed the manifest';
