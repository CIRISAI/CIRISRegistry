-- CIRISRegistry Database Schema
-- Migration 002: Agent Registry Tables

-- ============================================================================
-- AGENT REGISTRY
-- ============================================================================

CREATE TABLE agents (
    -- Identity
    agent_hash              BYTEA PRIMARY KEY,          -- SHA-256 of canonical agent build (32 bytes)
    agent_type              agent_type NOT NULL,
    version_major           INTEGER NOT NULL DEFAULT 0,
    version_minor           INTEGER NOT NULL DEFAULT 0,
    version_patch           INTEGER NOT NULL DEFAULT 0,
    version_prerelease      TEXT,                       -- e.g., "beta.1"
    version_build_metadata  TEXT,                       -- e.g., "20250125"

    -- Capabilities
    base_capabilities       TEXT[] NOT NULL DEFAULT '{}',
    max_autonomy_tier       autonomy_tier NOT NULL DEFAULT 'A0_ADVISORY',

    -- Provenance
    build_timestamp         TIMESTAMPTZ NOT NULL,
    source_repo             TEXT NOT NULL,              -- Git repository URL
    source_commit           CHAR(40) NOT NULL,          -- Git commit hash
    builder_attestation     BYTEA,                      -- Reproducible build attestation

    -- Status
    status                  agent_status NOT NULL DEFAULT 'ACTIVE',
    revocation_reason       TEXT,
    revocation_timestamp    TIMESTAMPTZ,

    -- Metadata
    registered_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated            TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Hybrid signature (registry signature on record)
    sig_classical           BYTEA,                      -- Ed25519 signature (64 bytes)
    sig_post_quantum        BYTEA,                      -- ML-DSA-65 signature (~3300 bytes)
    sig_timestamp           TIMESTAMPTZ,
    sig_key_id              TEXT,

    -- Constraints
    CONSTRAINT valid_agent_hash CHECK (LENGTH(agent_hash) = 32),
    CONSTRAINT valid_source_commit CHECK (source_commit ~ '^[a-f0-9]{40}$')
);

-- Comment on table
COMMENT ON TABLE agents IS 'Registry of verified CIRIS agent builds';
COMMENT ON COLUMN agents.agent_hash IS 'SHA-256 hash of canonical agent build - primary identity';
COMMENT ON COLUMN agents.base_capabilities IS 'Array of capability strings this agent can support';
