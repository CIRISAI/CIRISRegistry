-- Drop legacy audit triggers that reference non-existent columns.
-- Production has manually-created triggers on agents, partners, organizations, etc.
-- that INSERT into audit_logs referencing NEW.id, but these tables use different
-- primary key columns (agent_hash, partner_id, org_id).
-- The Rust application handles audit logging explicitly via db/audit.rs,
-- so these triggers are redundant and cause INSERT failures.

-- Drop ALL triggers on tables that are known to have bad audit triggers.
-- Using DO block to handle cases where triggers may not exist.

DO $$
DECLARE
    r RECORD;
BEGIN
    -- Drop all triggers on agents table
    FOR r IN (SELECT trigger_name FROM information_schema.triggers
              WHERE event_object_table = 'agents'
              AND trigger_schema = 'public') LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS %I ON agents', r.trigger_name);
        RAISE NOTICE 'Dropped trigger % on agents', r.trigger_name;
    END LOOP;

    -- Drop all triggers on partners table
    FOR r IN (SELECT trigger_name FROM information_schema.triggers
              WHERE event_object_table = 'partners'
              AND trigger_schema = 'public') LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS %I ON partners', r.trigger_name);
        RAISE NOTICE 'Dropped trigger % on partners', r.trigger_name;
    END LOOP;

    -- Drop all triggers on organizations table
    FOR r IN (SELECT trigger_name FROM information_schema.triggers
              WHERE event_object_table = 'organizations'
              AND trigger_schema = 'public') LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS %I ON organizations', r.trigger_name);
        RAISE NOTICE 'Dropped trigger % on organizations', r.trigger_name;
    END LOOP;

    -- Drop all triggers on org_users table
    FOR r IN (SELECT trigger_name FROM information_schema.triggers
              WHERE event_object_table = 'org_users'
              AND trigger_schema = 'public') LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS %I ON org_users', r.trigger_name);
        RAISE NOTICE 'Dropped trigger % on org_users', r.trigger_name;
    END LOOP;

    -- Drop all triggers on keys table
    FOR r IN (SELECT trigger_name FROM information_schema.triggers
              WHERE event_object_table = 'keys'
              AND trigger_schema = 'public') LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS %I ON keys', r.trigger_name);
        RAISE NOTICE 'Dropped trigger % on keys', r.trigger_name;
    END LOOP;

    -- Drop all triggers on revocations table
    FOR r IN (SELECT trigger_name FROM information_schema.triggers
              WHERE event_object_table = 'revocations'
              AND trigger_schema = 'public') LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS %I ON revocations', r.trigger_name);
        RAISE NOTICE 'Dropped trigger % on revocations', r.trigger_name;
    END LOOP;

    -- Drop orphaned trigger functions that may reference these triggers
    DROP FUNCTION IF EXISTS audit_agents_changes() CASCADE;
    DROP FUNCTION IF EXISTS audit_partners_changes() CASCADE;
    DROP FUNCTION IF EXISTS audit_organizations_changes() CASCADE;
    DROP FUNCTION IF EXISTS audit_org_users_changes() CASCADE;
    DROP FUNCTION IF EXISTS audit_keys_changes() CASCADE;
    DROP FUNCTION IF EXISTS audit_revocations_changes() CASCADE;
    -- Generic audit function name patterns
    DROP FUNCTION IF EXISTS log_agents_audit() CASCADE;
    DROP FUNCTION IF EXISTS log_partners_audit() CASCADE;
    DROP FUNCTION IF EXISTS log_organizations_audit() CASCADE;
    DROP FUNCTION IF EXISTS log_audit() CASCADE;

    RAISE NOTICE 'Legacy audit triggers cleaned up successfully';
END $$;
