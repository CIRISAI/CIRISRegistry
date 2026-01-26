-- CIRISRegistry Database Schema
-- Migration 003: Partner Registry Tables

-- ============================================================================
-- PARTNER REGISTRY
-- ============================================================================

CREATE TABLE partners (
    -- Identity
    partner_id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_name       TEXT NOT NULL,
    organization_id         TEXT,                       -- Tax ID / Registration number

    -- License
    license_type            license_type NOT NULL,
    license_id              TEXT UNIQUE NOT NULL,
    issued_at               TIMESTAMPTZ NOT NULL,
    expires_at              TIMESTAMPTZ NOT NULL,

    -- Grants
    capabilities_granted    TEXT[] NOT NULL DEFAULT '{}',
    capabilities_denied     TEXT[] NOT NULL DEFAULT '{}',
    max_autonomy_tier       autonomy_tier NOT NULL DEFAULT 'A0_ADVISORY',

    -- Constraints
    requires_supervisor     BOOLEAN NOT NULL DEFAULT TRUE,
    geographic_restrictions TEXT[] NOT NULL DEFAULT '{}',  -- ISO country codes
    deployment_limit        INTEGER NOT NULL DEFAULT 1,
    offline_grace_hours     INTEGER NOT NULL DEFAULT 72,

    -- Contact
    technical_contact       TEXT,
    compliance_contact      TEXT,

    -- Status
    status                  partner_status NOT NULL DEFAULT 'ACTIVE',
    suspension_reason       TEXT,
    revocation_reason       TEXT,
    status_changed_at       TIMESTAMPTZ,

    -- Timestamps
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- License signature (steward signature on license)
    license_sig_classical       BYTEA,
    license_sig_post_quantum    BYTEA,
    license_sig_timestamp       TIMESTAMPTZ,
    license_sig_key_id          TEXT,

    -- Registry signature (registry signature on record)
    registry_sig_classical      BYTEA,
    registry_sig_post_quantum   BYTEA,
    registry_sig_timestamp      TIMESTAMPTZ,
    registry_sig_key_id         TEXT
);

-- Comment on table
COMMENT ON TABLE partners IS 'Licensed partner organizations authorized to deploy CIRIS agents';
COMMENT ON COLUMN partners.capabilities_granted IS 'Specific capabilities granted beyond base license';
COMMENT ON COLUMN partners.capabilities_denied IS 'Explicit capability denials (overrides grants)';

-- ============================================================================
-- REVOCATION LIST
-- ============================================================================

CREATE TABLE revocations (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Target
    target_type             revocation_type NOT NULL,
    target_id               TEXT NOT NULL,              -- Hash (hex) or ID being revoked

    -- Details
    revoked_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reason_code             revocation_reason NOT NULL,
    reason_detail           TEXT,
    severity                revocation_severity NOT NULL DEFAULT 'ADMINISTRATIVE',

    -- Authority signature
    authority_sig_classical     BYTEA,
    authority_sig_post_quantum  BYTEA,
    authority_sig_timestamp     TIMESTAMPTZ,
    authority_sig_key_id        TEXT,

    -- Constraints
    CONSTRAINT unique_revocation UNIQUE (target_type, target_id)
);

-- Index for quick lookups
CREATE INDEX idx_revocations_target ON revocations(target_type, target_id);

COMMENT ON TABLE revocations IS 'Central revocation list for agents, partners, and licenses';

-- ============================================================================
-- REGISTRY SNAPSHOTS (for offline operation)
-- ============================================================================

CREATE TABLE registry_snapshots (
    snapshot_version        BIGSERIAL PRIMARY KEY,
    generated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Merkle roots
    agents_merkle_root      BYTEA NOT NULL,             -- 32 bytes
    partners_merkle_root    BYTEA NOT NULL,             -- 32 bytes
    revocations_merkle_root BYTEA NOT NULL,             -- 32 bytes

    -- Signature
    sig_classical           BYTEA,
    sig_post_quantum        BYTEA,
    sig_timestamp           TIMESTAMPTZ,
    sig_key_id              TEXT,

    CONSTRAINT valid_merkle_roots CHECK (
        LENGTH(agents_merkle_root) = 32 AND
        LENGTH(partners_merkle_root) = 32 AND
        LENGTH(revocations_merkle_root) = 32
    )
);

COMMENT ON TABLE registry_snapshots IS 'Signed snapshots for offline verification';
