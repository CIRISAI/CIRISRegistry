-- Migration 023: Declare replication intent for trusted_primitive_keys
--
-- trusted_primitive_keys (created in migration 022) holds build-signing
-- pubkeys per CIRIS primitive (ciris-agent, ciris-persist, etc.). These
-- are deployment-wide identities — a build manifest signed by ciris-agent's
-- CI key must verify in BOTH the US and EU registry deployments. A
-- US/EU split would mean a CIRISVerify client gets different answers
-- depending on which region it hits.
--
-- Pattern: same as partner_keys (cross-region) and unlike
-- registry_signing_keys (per-region steward identity). Closes
-- CIRISRegistry#4.
--
-- This migration enrolls trusted_primitive_keys in Spock's default
-- replication set when Spock is loaded. Idempotent on container restart
-- via DO block + EXCEPTION handler (same pattern as the in-app Spock
-- helper at db/mod.rs::exclude_sqlx_migrations_from_spock_replication
-- after the f848fe8 / 6cf564f hotfixes).
--
-- Convention going forward (also documented in CLAUDE.md):
-- Every new CREATE TABLE migration MUST declare its replication scope:
--   - Cross-region (deployment-wide identity, public lookup data) →
--     wrap a spock.repset_add_table call in this same DO block pattern.
--   - Per-region (node-local state, per-node steward, bookkeeping) →
--     comment in the migration noting the intentional per-node scope.

DO $$
BEGIN
    -- Spock detection guard: no-op when extension isn't loaded
    -- (single-node dev / staging / test environments).
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'spock') THEN
        BEGIN
            -- The third argument (synchronize_data) is FALSE because
            -- the table may already contain functionally-equivalent
            -- boot-seed rows on both sides; we don't want to overwrite
            -- one node's row with the other's. Operator-driven UPSERTs
            -- on either node will replicate from this point forward.
            PERFORM spock.repset_add_table('default', 'public.trusted_primitive_keys', false);
            RAISE NOTICE 'Spock: enrolled public.trusted_primitive_keys in default replication set';
        EXCEPTION
            WHEN others THEN
                -- Already a member, or any other Spock-side error → benign.
                RAISE NOTICE 'Spock repset_add_table for trusted_primitive_keys (likely already enrolled): %', SQLERRM;
        END;
    END IF;
END $$;
