-- Replace audit_logs VIEW with a real TABLE that accepts all trigger columns
-- Production has triggers on tables (agents, partners, etc.) that INSERT into
-- audit_logs with columns from a different schema than audit_log.
-- This table acts as the trigger target; the Rust code continues using audit_log directly.

-- Drop the view and its rules first
DROP VIEW IF EXISTS audit_logs CASCADE;

-- Create audit_logs as a real table matching common audit trigger patterns
CREATE TABLE IF NOT EXISTS audit_logs (
    id BIGSERIAL PRIMARY KEY,
    entry_id TEXT,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    schema_name TEXT,
    table_name TEXT,
    operation TEXT,
    actor_type TEXT,
    actor_id TEXT,
    actor_user_id TEXT,
    actor_org_id TEXT,
    actor_ip_address TEXT,
    actor_user_agent TEXT,
    actor_system_user_id TEXT,
    action INTEGER,
    resource_type TEXT,
    resource_id TEXT,
    target_type TEXT,
    target_id TEXT,
    description TEXT,
    old_data JSONB,
    new_data JSONB,
    metadata JSONB,
    entry_signature BYTEA
);

CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp ON audit_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_audit_logs_table_name ON audit_logs(table_name);
CREATE INDEX IF NOT EXISTS idx_audit_logs_operation ON audit_logs(operation);
