-- Create audit_logs (plural) view for backward compatibility
-- Production DB has triggers or references using the plural form "audit_logs"
-- This view maps to the canonical audit_log (singular) table

CREATE OR REPLACE VIEW audit_logs AS SELECT * FROM audit_log;

-- Also create an INSERT rule so INSERTs to audit_logs go to audit_log
CREATE OR REPLACE RULE audit_logs_insert AS
    ON INSERT TO audit_logs
    DO INSTEAD
    INSERT INTO audit_log (
        entry_id, timestamp, actor_user_id, actor_org_id, actor_ip_address,
        actor_user_agent, action, target_type, target_id, description,
        metadata, entry_signature, actor_system_user_id
    ) VALUES (
        NEW.entry_id, NEW.timestamp, NEW.actor_user_id, NEW.actor_org_id,
        NEW.actor_ip_address, NEW.actor_user_agent, NEW.action, NEW.target_type,
        NEW.target_id, NEW.description, NEW.metadata, NEW.entry_signature,
        NEW.actor_system_user_id
    );
