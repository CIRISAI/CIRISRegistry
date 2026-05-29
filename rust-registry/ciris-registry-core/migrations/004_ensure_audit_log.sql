-- Ensure audit_log table exists (may be missing if 001 was applied before it was added)

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
    entry_signature BYTEA,
    actor_system_user_id TEXT
);

CREATE INDEX IF NOT EXISTS idx_audit_log_org_id ON audit_log(actor_org_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_audit_log_action ON audit_log(action);
