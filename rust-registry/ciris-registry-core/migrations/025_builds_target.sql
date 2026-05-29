-- Migration 025: Target discriminator on builds
--
-- Closes:
--   - GitHub issue CIRISAI/CIRISRegistry#11 (get_build_by_version returns
--     the wrong manifest when one (project, version) has multiple
--     registered targets).
--
-- Background. CIRISAgent's CI registers two BuildRecords per release —
-- one per --target (python-source-tree, ios-mobile-bundle, …). Each
-- POST writes its own row under a unique build_hash. The existing
-- get_build_by_version SQL orders by registered_at DESC LIMIT 1 with
-- no target filter, so the last-registered target wins all
-- subsequent lookups. Mobile-target rows have shipped over the
-- canonical python-source-tree row for v2.8.9, causing every agent
-- doing L4 file-integrity attestation to fail (#11 evidence: 1426/1664
-- files verified, 43 hash mismatches, 195 missing).
--
-- Fix: add an explicit `target` column to builds so version lookups
-- can disambiguate. Backfill existing rows from `includes_modules`
-- (rows tagged with 'ios' → 'ios-mobile-bundle'; everything else →
-- 'python-source-tree', the canonical default). After backfill, mark
-- NOT NULL — new rows MUST carry an explicit target. The POST
-- /v1/builds wire contract (v2 of CanonicalBuild signed bytes) adds
-- `target` between `version` and `build_hash` in canonical order;
-- ciris-build-sign register cuts over in CIRISVerify#8 / CIRISAgent#729.
--
-- Replication: builds is already enrolled in the default repset via
-- the project-namespace migration (021). No new repset_add_table call.
--
-- Idempotency: ADD COLUMN IF NOT EXISTS + WHERE target IS NULL on the
-- backfill + DO $$ guard on the NOT NULL constraint. Safe to re-run.

-- ============================================================================
-- Column
-- ============================================================================

ALTER TABLE builds
    ADD COLUMN IF NOT EXISTS target TEXT;

-- ============================================================================
-- Backfill — derive from includes_modules
-- ============================================================================
--
-- The two known targets in production today (2.8.x agent releases):
--   - 'python-source-tree' — modules typically ['core'] (no 'ios')
--   - 'ios-mobile-bundle'  — modules contain 'ios'
--
-- Default the unknown to 'python-source-tree' since it's the canonical
-- byte-identical-across-platforms source manifest; iOS rows are the
-- exception we discriminate against.

UPDATE builds
SET target = CASE
    WHEN 'ios' = ANY(COALESCE(includes_modules, ARRAY[]::TEXT[])) THEN 'ios-mobile-bundle'
    ELSE 'python-source-tree'
END
WHERE target IS NULL;

-- ============================================================================
-- NOT NULL — guaranteed after backfill
-- ============================================================================
--
-- Guard against partial application: only SET NOT NULL if every row has
-- a non-null target (the backfill above ensures it, but a concurrent
-- writer racing the migration could theoretically slip an insert).

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM builds WHERE target IS NULL) THEN
        ALTER TABLE builds ALTER COLUMN target SET NOT NULL;
    END IF;
END $$;

-- ============================================================================
-- Lookup index for the target-aware version query
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_builds_project_version_target
    ON builds(project, version, target)
    WHERE status = 'active';
