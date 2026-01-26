-- CIRISRegistry Database Schema
-- Migration 005: Key Management (CIRISPortal)

-- ============================================================================
-- PARTNER KEY RECORDS
-- ============================================================================

CREATE TABLE partner_keys (
    -- Identity
    key_id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id                  UUID NOT NULL REFERENCES organizations(org_id) ON DELETE CASCADE,
    partner_id              UUID REFERENCES partners(partner_id) ON DELETE SET NULL,

    -- Public keys (always available)
    ed25519_public_key      BYTEA NOT NULL,             -- 32 bytes
    ml_dsa_65_public_key    BYTEA NOT NULL,             -- ~1952 bytes

    -- Key fingerprints for identification
    ed25519_fingerprint     CHAR(64) NOT NULL,          -- SHA-256 hex
    ml_dsa_65_fingerprint   CHAR(64) NOT NULL,          -- SHA-256 hex

    -- Custody
    custody_model           key_custody_model NOT NULL DEFAULT 'CUSTODIED',

    -- For custodied keys: reference to secure storage (NOT the actual keys)
    -- Private keys stored in Cloudflare KV or HSM, never in PostgreSQL
    kv_key_ref              TEXT,                       -- e.g., "keys:org_123:key_456"

    -- Status
    status                  key_status NOT NULL DEFAULT 'PENDING',
    revocation_reason       TEXT,

    -- Timestamps
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    activated_at            TIMESTAMPTZ,                -- When key became active
    rotated_at              TIMESTAMPTZ,                -- When replaced by new key
    revoked_at              TIMESTAMPTZ,

    -- Audit
    created_by              UUID REFERENCES org_users(user_id) ON DELETE SET NULL,
    rotated_by              UUID REFERENCES org_users(user_id) ON DELETE SET NULL,
    revoked_by              UUID REFERENCES org_users(user_id) ON DELETE SET NULL,

    -- Registry signature (proves key is registered)
    registry_sig_classical      BYTEA,
    registry_sig_post_quantum   BYTEA,
    registry_sig_timestamp      TIMESTAMPTZ,
    registry_sig_key_id         TEXT,

    -- Constraints
    CONSTRAINT valid_ed25519_pubkey CHECK (LENGTH(ed25519_public_key) = 32),
    CONSTRAINT unique_ed25519_fingerprint UNIQUE (ed25519_fingerprint),
    CONSTRAINT unique_ml_dsa_fingerprint UNIQUE (ml_dsa_65_fingerprint)
);

-- Only one active key per organization
CREATE UNIQUE INDEX idx_one_active_key_per_org
    ON partner_keys(org_id)
    WHERE status = 'ACTIVE';

-- Index for org lookups
CREATE INDEX idx_partner_keys_org ON partner_keys(org_id);
CREATE INDEX idx_partner_keys_partner ON partner_keys(partner_id) WHERE partner_id IS NOT NULL;

COMMENT ON TABLE partner_keys IS 'Hybrid cryptographic key pairs for organizations';
COMMENT ON COLUMN partner_keys.kv_key_ref IS 'Reference to encrypted private keys in Cloudflare KV/HSM - NEVER store actual private keys here';

-- ============================================================================
-- SIGNING LOG
-- ============================================================================

-- Logs every signature request for audit purposes
CREATE TABLE signing_log (
    log_id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Request info
    org_id                  UUID NOT NULL REFERENCES organizations(org_id) ON DELETE CASCADE,
    key_id                  UUID NOT NULL REFERENCES partner_keys(key_id) ON DELETE CASCADE,
    requester_user_id       UUID REFERENCES org_users(user_id) ON DELETE SET NULL,

    -- What was signed
    data_hash               BYTEA NOT NULL,             -- SHA-256 of signed data (32 bytes)
    purpose                 TEXT NOT NULL,              -- Why this was signed

    -- Timestamp
    signed_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Client info
    ip_address              INET,
    user_agent              TEXT,

    -- Constraints
    CONSTRAINT valid_data_hash CHECK (LENGTH(data_hash) = 32)
);

-- Indexes for audit queries
CREATE INDEX idx_signing_log_org ON signing_log(org_id, signed_at DESC);
CREATE INDEX idx_signing_log_key ON signing_log(key_id, signed_at DESC);
CREATE INDEX idx_signing_log_time ON signing_log(signed_at DESC);

COMMENT ON TABLE signing_log IS 'Audit log of all signing operations';
