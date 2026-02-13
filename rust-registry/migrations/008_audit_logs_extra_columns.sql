-- Add any remaining columns that production triggers reference.
-- Triggers on tables like agents, partners, etc. INSERT into audit_logs
-- with columns from a different schema than what migration 007 created.

ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS details JSONB;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS source TEXT;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS ip_address TEXT;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS user_agent TEXT;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS session_id TEXT;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS request_id TEXT;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS status TEXT;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS duration_ms INTEGER;
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS error_message TEXT;
