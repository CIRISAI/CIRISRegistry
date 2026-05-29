-- CIRISRegistry Initial Schema
-- Version: 1.1.0

-- Agents table
CREATE TABLE IF NOT EXISTS agents (
    agent_hash BYTEA PRIMARY KEY,
    agent_type INTEGER NOT NULL DEFAULT 0,
    version_major INTEGER NOT NULL DEFAULT 0,
    version_minor INTEGER NOT NULL DEFAULT 0,
    version_patch INTEGER NOT NULL DEFAULT 0,
    version_prerelease TEXT,
    version_build_metadata TEXT,
    base_capabilities TEXT[] NOT NULL DEFAULT '{}',
    max_autonomy_tier INTEGER NOT NULL DEFAULT 0,
    build_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_repo TEXT,
    source_commit TEXT,
    builder_attestation BYTEA,
    status INTEGER NOT NULL DEFAULT 1,
    revocation_reason TEXT,
    revocation_timestamp TIMESTAMPTZ,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    registry_signature BYTEA,
    is_test_record BOOLEAN NOT NULL DEFAULT FALSE,
    test_tag TEXT
);

CREATE INDEX idx_agents_status ON agents(status);
CREATE INDEX idx_agents_type ON agents(agent_type);
CREATE INDEX idx_agents_test_tag ON agents(test_tag) WHERE is_test_record = TRUE;

-- Partners table
CREATE TABLE IF NOT EXISTS partners (
    partner_id TEXT PRIMARY KEY,
    organization_name TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    license_type INTEGER NOT NULL DEFAULT 0,
    license_id TEXT NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    capabilities_granted TEXT[] NOT NULL DEFAULT '{}',
    capabilities_denied TEXT[] NOT NULL DEFAULT '{}',
    max_autonomy_tier INTEGER NOT NULL DEFAULT 0,
    requires_supervisor BOOLEAN NOT NULL DEFAULT FALSE,
    geographic_restrictions TEXT[] NOT NULL DEFAULT '{}',
    deployment_limit INTEGER NOT NULL DEFAULT 1,
    offline_grace_hours INTEGER NOT NULL DEFAULT 72,
    technical_contact TEXT,
    compliance_contact TEXT,
    status INTEGER NOT NULL DEFAULT 1,
    suspension_reason TEXT,
    revocation_reason TEXT,
    status_changed_at TIMESTAMPTZ,
    license_signature BYTEA,
    registry_signature BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_partners_status ON partners(status);
CREATE INDEX idx_partners_expires_at ON partners(expires_at);

-- Organizations table
CREATE TABLE IF NOT EXISTS organizations (
    org_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    legal_name TEXT NOT NULL,
    tax_id TEXT,
    partner_id TEXT REFERENCES partners(partner_id),
    primary_email TEXT NOT NULL,
    billing_email TEXT,
    technical_contact_email TEXT,
    compliance_contact_email TEXT,
    oauth_provider TEXT,
    oauth_domain TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by TEXT,
    metadata JSONB DEFAULT '{}'
);

CREATE INDEX idx_organizations_active ON organizations(active);
CREATE INDEX idx_organizations_partner_id ON organizations(partner_id);

-- Organization users table
CREATE TABLE IF NOT EXISTS org_users (
    user_id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL REFERENCES organizations(org_id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    oauth_provider TEXT,
    oauth_subject TEXT,
    role INTEGER NOT NULL DEFAULT 4,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,
    invited_by TEXT,
    mfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    mfa_method TEXT
);

CREATE INDEX idx_org_users_org_id ON org_users(org_id);
CREATE INDEX idx_org_users_email ON org_users(email);
CREATE INDEX idx_org_users_active ON org_users(active);

-- Partner keys table
CREATE TABLE IF NOT EXISTS partner_keys (
    key_id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL REFERENCES organizations(org_id),
    partner_id TEXT REFERENCES partners(partner_id),
    ed25519_public_key BYTEA NOT NULL,
    ml_dsa_65_public_key BYTEA NOT NULL,
    ed25519_fingerprint TEXT NOT NULL,
    ml_dsa_65_fingerprint TEXT NOT NULL,
    custody_model INTEGER NOT NULL DEFAULT 1,
    kv_key_ref TEXT,
    status INTEGER NOT NULL DEFAULT 4,
    revocation_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    activated_at TIMESTAMPTZ,
    rotated_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    grace_period_expires_at TIMESTAMPTZ,
    created_by TEXT,
    rotated_by TEXT,
    revoked_by TEXT,
    registry_signature BYTEA,
    escrow_id TEXT
);

CREATE INDEX idx_partner_keys_org_id ON partner_keys(org_id);
CREATE INDEX idx_partner_keys_status ON partner_keys(status);

-- Key escrows table
CREATE TABLE IF NOT EXISTS key_escrows (
    escrow_id TEXT PRIMARY KEY,
    key_id TEXT NOT NULL REFERENCES partner_keys(key_id),
    org_id TEXT NOT NULL REFERENCES organizations(org_id),
    escrow_type INTEGER NOT NULL,
    custodian TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'ACTIVE'
);

CREATE INDEX idx_key_escrows_org_id ON key_escrows(org_id);

-- Revocations table
CREATE TABLE IF NOT EXISTS revocations (
    id SERIAL PRIMARY KEY,
    target_type INTEGER NOT NULL,
    target_id TEXT NOT NULL,
    revoked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reason_code INTEGER NOT NULL,
    reason_detail TEXT,
    severity INTEGER NOT NULL DEFAULT 1,
    authority_signature BYTEA
);

CREATE INDEX idx_revocations_target ON revocations(target_type, target_id);

-- Registry snapshots table
CREATE TABLE IF NOT EXISTS registry_snapshots (
    snapshot_id SERIAL PRIMARY KEY,
    snapshot_version BIGINT NOT NULL,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    agents_merkle_root BYTEA NOT NULL,
    partners_merkle_root BYTEA NOT NULL,
    revocations_merkle_root BYTEA NOT NULL,
    snapshot_signature BYTEA
);

CREATE INDEX idx_snapshots_version ON registry_snapshots(snapshot_version);

-- Audit log table
CREATE TABLE IF NOT EXISTS audit_log (
    entry_id TEXT PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    actor_user_id TEXT,
    actor_org_id TEXT,
    actor_ip_address TEXT,
    actor_user_agent TEXT,
    action INTEGER NOT NULL,
    target_type TEXT,
    target_id TEXT,
    description TEXT,
    metadata JSONB,
    entry_signature BYTEA
);

CREATE INDEX idx_audit_log_org_id ON audit_log(actor_org_id);
CREATE INDEX idx_audit_log_timestamp ON audit_log(timestamp);
CREATE INDEX idx_audit_log_action ON audit_log(action);

-- Webhooks table
CREATE TABLE IF NOT EXISTS webhooks (
    webhook_id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL REFERENCES organizations(org_id),
    url TEXT NOT NULL,
    subscribed_events TEXT[] NOT NULL DEFAULT '{}',
    signing_secret TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_triggered_at TIMESTAMPTZ,
    consecutive_failures INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_webhooks_org_id ON webhooks(org_id);

-- Build attestations table
CREATE TABLE IF NOT EXISTS build_attestations (
    agent_hash BYTEA PRIMARY KEY REFERENCES agents(agent_hash),
    builder_id TEXT NOT NULL,
    invocation_id TEXT,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    source_uri TEXT,
    source_commit TEXT,
    source_branch TEXT,
    build_commands TEXT[],
    expected_artifact_hash BYTEA,
    reproducible_build_url TEXT,
    builder_os TEXT,
    builder_architecture TEXT,
    builder_env JSONB,
    builder_signature BYTEA,
    verification_count INTEGER NOT NULL DEFAULT 0,
    last_verified_at TIMESTAMPTZ
);

-- Registry signing keys table
CREATE TABLE IF NOT EXISTS registry_signing_keys (
    key_id TEXT PRIMARY KEY,
    storage_mode INTEGER NOT NULL,
    ed25519_public_key BYTEA NOT NULL,
    ed25519_fingerprint TEXT NOT NULL,
    mldsa65_public_key BYTEA NOT NULL,
    mldsa65_fingerprint TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    activated_at TIMESTAMPTZ,
    rotated_at TIMESTAMPTZ,
    rotated_by TEXT,
    retired_at TIMESTAMPTZ,
    usage_count BIGINT NOT NULL DEFAULT 0,
    last_used TIMESTAMPTZ,
    status INTEGER NOT NULL DEFAULT 1,
    hsm_slot_id TEXT,
    hsm_label TEXT
);

CREATE INDEX idx_signing_keys_status ON registry_signing_keys(status);

-- Emergency status table
CREATE TABLE IF NOT EXISTS emergency_status (
    id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    locked_at TIMESTAMPTZ,
    locked_until TIMESTAMPTZ,
    lock_reason TEXT,
    severity INTEGER,
    allowed_operations TEXT[] DEFAULT '{}',
    locked_by TEXT
);

INSERT INTO emergency_status (id) VALUES (1) ON CONFLICT DO NOTHING;
