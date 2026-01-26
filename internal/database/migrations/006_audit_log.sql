-- CIRISRegistry Database Schema
-- Migration 006: Audit Log

-- ============================================================================
-- AUDIT LOG
-- ============================================================================

CREATE TABLE audit_log (
    -- Identity
    entry_id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    timestamp               TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Actor
    actor_user_id           UUID REFERENCES org_users(user_id) ON DELETE SET NULL,
    actor_org_id            UUID REFERENCES organizations(org_id) ON DELETE SET NULL,
    actor_ip_address        INET,
    actor_user_agent        TEXT,

    -- Action
    action                  audit_action_type NOT NULL,
    target_type             TEXT NOT NULL,              -- "organization", "user", "key", "partner", "agent"
    target_id               TEXT NOT NULL,              -- UUID or hash of target

    -- Details
    description             TEXT NOT NULL,
    metadata                JSONB NOT NULL DEFAULT '{}',

    -- For sensitive actions, signed by registry
    entry_sig_classical     BYTEA,
    entry_sig_post_quantum  BYTEA,
    entry_sig_timestamp     TIMESTAMPTZ,
    entry_sig_key_id        TEXT
);

-- Indexes for efficient queries
CREATE INDEX idx_audit_org ON audit_log(actor_org_id, timestamp DESC);
CREATE INDEX idx_audit_user ON audit_log(actor_user_id, timestamp DESC);
CREATE INDEX idx_audit_action ON audit_log(action, timestamp DESC);
CREATE INDEX idx_audit_target ON audit_log(target_type, target_id, timestamp DESC);
CREATE INDEX idx_audit_time ON audit_log(timestamp DESC);

-- Partition by month for large deployments (optional, can be enabled later)
-- Note: For active/active replication, consider using declarative partitioning
-- CREATE TABLE audit_log_partitioned (...) PARTITION BY RANGE (timestamp);

COMMENT ON TABLE audit_log IS 'Immutable audit trail for all registry operations';

-- ============================================================================
-- MIGRATION TRACKING
-- ============================================================================

CREATE TABLE IF NOT EXISTS schema_migrations (
    version         TEXT PRIMARY KEY,
    description     TEXT,
    applied_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    checksum        TEXT                                -- SHA-256 of migration file
);

COMMENT ON TABLE schema_migrations IS 'Tracks applied database migrations';
