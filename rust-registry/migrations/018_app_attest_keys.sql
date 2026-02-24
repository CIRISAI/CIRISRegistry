-- iOS App Attest Keys
-- Stores attested public keys for assertion verification
-- Part of the CIRISVerify iOS device integrity flow

CREATE TABLE IF NOT EXISTS app_attest_keys (
    key_id VARCHAR(255) PRIMARY KEY,
    public_key BYTEA NOT NULL,
    counter INTEGER NOT NULL DEFAULT 0,
    app_id_hash BYTEA NOT NULL,
    environment VARCHAR(50) NOT NULL DEFAULT 'production',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for efficient lookup by app_id_hash (for multi-app scenarios)
CREATE INDEX IF NOT EXISTS idx_app_attest_keys_app_id_hash ON app_attest_keys(app_id_hash);

-- Index for environment filtering
CREATE INDEX IF NOT EXISTS idx_app_attest_keys_environment ON app_attest_keys(environment);

COMMENT ON TABLE app_attest_keys IS 'iOS App Attest public keys for device/app integrity verification';
COMMENT ON COLUMN app_attest_keys.key_id IS 'SHA-256 hash of the public key (base64 encoded)';
COMMENT ON COLUMN app_attest_keys.public_key IS 'P-256 ECDSA public key (uncompressed, 65 bytes)';
COMMENT ON COLUMN app_attest_keys.counter IS 'Monotonic counter for replay protection';
COMMENT ON COLUMN app_attest_keys.app_id_hash IS 'SHA-256 hash of the App ID (Team ID + Bundle ID)';
COMMENT ON COLUMN app_attest_keys.environment IS 'Attestation environment: production or development';
