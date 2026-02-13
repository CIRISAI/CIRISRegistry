-- Add extra columns that production triggers expect but aren't in the base schema
-- These columns are referenced by database triggers on other tables that log to audit_logs

ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS actor_type TEXT;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS old_data JSONB;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS new_data JSONB;

-- Recreate the audit_logs view to pick up new columns
DROP VIEW IF EXISTS audit_logs CASCADE;
CREATE OR REPLACE VIEW audit_logs AS SELECT * FROM audit_log;

-- Recreate INSERT rule: use a simpler DEFAULT-based approach
-- that lets PostgreSQL handle any column subset
CREATE OR REPLACE RULE audit_logs_insert AS
    ON INSERT TO audit_logs
    DO INSTEAD
    INSERT INTO audit_log (
        entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
        actor_user_agent, action, target_type, target_id, description,
        metadata, entry_signature, actor_system_user_id,
        actor_type, old_data, new_data
    ) VALUES (
        NEW.entry_id, NEW.timestamp, NEW.actor_user_id, NEW.actor_org_id,
        NEW.actor_ip_address, NEW.actor_user_agent, NEW.action, NEW.target_type,
        NEW.target_id, NEW.description, NEW.metadata, NEW.entry_signature,
        NEW.actor_system_user_id,
        NEW.actor_type, NEW.old_data, NEW.new_data
    );
