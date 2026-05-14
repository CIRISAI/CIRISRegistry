-- Migration 028: Replace builds UNIQUE (project, version, includes_modules)
--                 with UNIQUE (project, version, target)
--
-- Closes:
--   - GitHub issue CIRISAI/CIRISRegistry#13 (POST /v1/builds 500 for
--     ciris-persist's multi-target v0.9.2 register sequence).
--
-- Background. Migration 021 introduced
-- `builds_project_version_modules_unique UNIQUE (project, version,
-- includes_modules)` as the disambiguator inside a project's version
-- namespace. That choice predates target-aware multi-target releases
-- — at the time, every release was one row per (project, version) so
-- `includes_modules` was a stand-in for "what does this build contain".
--
-- Migration 025 added the `target` column (closes #11) so version
-- lookups can disambiguate between `python-source-tree`,
-- `ios-mobile-bundle`, `aarch64-apple-darwin`, etc. for a single
-- `(project, version)`. But it did NOT update the unique constraint —
-- so a `ciris-build-sign register` invocation with multiple `--target`
-- arguments for the same `(project, version)` and same `includes_modules`
-- still trips the old constraint on the second POST. With the same
-- `includes_modules` across all four target POSTs (the default — modules
-- describes what code the build CONTAINS, not the platform target), the
-- constraint surface is `(project, version, ['core'])` for all four
-- rows. POST #1 succeeds; POSTs #2-4 fail with 500 because
-- `register_build`'s `ON CONFLICT (build_hash)` clause only handles the
-- build-hash collision (each target has a different hash by content),
-- not the (project, version, includes_modules) collision.
--
-- Even POST #1 fails if a prior partial run left a stale row at the
-- same (project, version, includes_modules) — which is what
-- CIRISRegistry#13 reports for ciris-persist's first v0.9.2 attempt.
--
-- Fix: the right discriminator for multi-target releases is `target`.
-- Drop the modules-based constraint, add a target-based one. The
-- `register_build` SQL is updated in the same change set to
-- `ON CONFLICT (project, version, target)` so retries with the same
-- target succeed (re-built artifact / re-tagged release).
--
-- Replication: builds is in the default repset (mig 021). Constraint
-- changes are DDL, executed per-node (Spock excludes _sqlx_migrations
-- from replication). Both regions must run this migration to stay in
-- sync — unavoidable for any constraint change.
--
-- Idempotency: DO $$ blocks check pg_constraint before each ALTER.
-- Safe to re-run.
--
-- Pre-flight risk: if any current data has duplicate (project, version,
-- target) — would happen if a prior bug allowed it — the new
-- constraint will fail to add. Migration logs and skips gracefully via
-- the EXCEPTION handler, leaving the new constraint absent so the
-- registry stays on the OLD constraint until ops dedupe. The
-- application code defaults to the new ON CONFLICT clause regardless;
-- the worst case is a redundant 500 with a clearer error message
-- pointing operators at the duplicate row.

DO $$
BEGIN
    -- Drop the legacy modules-based constraint (mig 021).
    IF EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'builds_project_version_modules_unique'
    ) THEN
        ALTER TABLE builds DROP CONSTRAINT builds_project_version_modules_unique;
        RAISE NOTICE 'Dropped builds_project_version_modules_unique';
    END IF;

    -- Add the target-aware constraint (mig 028).
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'builds_project_version_target_unique'
    ) THEN
        BEGIN
            ALTER TABLE builds
                ADD CONSTRAINT builds_project_version_target_unique
                UNIQUE (project, version, target);
            RAISE NOTICE 'Added builds_project_version_target_unique';
        EXCEPTION WHEN unique_violation THEN
            RAISE WARNING
                'Could not add builds_project_version_target_unique — '
                'duplicate (project, version, target) rows exist. '
                'Dedupe needed before this constraint can be added. '
                'Application falls back to the build_hash UNIQUE check.';
        END;
    END IF;
END $$;
