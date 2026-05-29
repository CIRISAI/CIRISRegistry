-- Migration 029: Drop builds.build_hash UNIQUE constraint
--
-- Closes:
--   - GitHub issue CIRISAI/CIRISRegistry#14 (GET /v1/builds/{version}?
--     target=<binary-target> returns 404 for fresh binary-mode rows;
--     POST silently collapses 4 multi-target POSTs onto one row).
--
-- Background. The original `builds` table (migration 012) declared
-- `build_hash TEXT NOT NULL UNIQUE` with the comment "SHA-256 of all
-- source files concatenated" — built on the assumption that one
-- release == one build_hash == one builds row. Migration 021's note
-- reinforced this: "Build hash is a SHA-256 of source files; it's
-- globally unique by construction regardless of project. Two projects
-- cannot legitimately produce the same build hash."
--
-- That single-row-per-release world ended at CIRISVerify v2.0.3 +
-- CIRISRegistry v1.4.1 (closes #11). `ciris-build-sign register`
-- now derives ONE `build_hash` from the COMBINATION of all per-target
-- binary hashes — the comment in upstream `register.rs::derive_build_hash`
-- says "matching the registry's `(project, build_hash)` UPSERT key" —
-- and POSTs N rows under the same `(project, version)` with that
-- shared `build_hash` and N distinct `target` values. `(project,
-- version, target)` becomes the row-uniqueness anchor (added in
-- migration 028 as `builds_project_version_target_unique`).
--
-- With the legacy `build_hash UNIQUE` still in force, only the FIRST
-- target POST in a multi-target register sequence succeeds:
--
--   POST 1: INSERT (ciris-persist, 0.9.3, python-source-tree, hash=H)
--           → succeeds, row 1 created
--   POST 2: INSERT (ciris-persist, 0.9.3, x86_64..., hash=H)
--           → conflict on (project, version, target) — DIFFERENT, no
--             match
--           → conflict on build_hash — MATCHES, but ON CONFLICT
--             (project, version, target) doesn't catch a different
--             constraint
--           → ERROR: duplicate key on builds_build_hash_key (post-#13
--             behavior, returns 500/409)
--
-- Pre-#13 (`ON CONFLICT (build_hash) DO UPDATE SET target =
-- EXCLUDED.target`), POSTs 2-N silently overwrote row 1's target,
-- leaving the LAST POST's target winning — the source of CIRISRegistry#14's
-- "GET 404 for non-darwin targets" symptom on persist's CI: 4 POSTs
-- collapsed onto 1 row with target=aarch64-apple-darwin (or whichever
-- target hit last); GETs for the other three targets 404'd because
-- those rows never existed.
--
-- Fix: drop the UNIQUE constraint. The implicit
-- `idx_builds_build_hash` non-unique index from migration 012 stays in
-- place for fast hash-based lookups (used by `get_build_by_hash`). The
-- per-row uniqueness contract is now `(project, version, target)`.
--
-- `get_build_by_hash` semantics: a `build_hash` may now match N rows
-- (one per target in a multi-target release). The handler returns the
-- first row arbitrarily, sufficient for "does this build exist /
-- is it active" liveness checks. Callers needing target-specific data
-- should use `GET /v1/builds/{version}?project=&target=` instead.
--
-- Replication: builds is in the default Spock repset (mig 021).
-- Constraint changes are DDL, executed per-node (Spock excludes
-- `_sqlx_migrations` from replication). Both regions must run this
-- migration to converge.
--
-- Idempotency: DO $$ block checks pg_constraint before DROP. Safe to
-- re-run.
--
-- Backwards compatibility: dropping a UNIQUE constraint is permissive
-- — every previously-valid INSERT still works. Existing rows are
-- unaffected; no backfill needed.

DO $$
BEGIN
    -- The constraint name from `build_hash TEXT NOT NULL UNIQUE` in
    -- migration 012 is `builds_build_hash_key` (PostgreSQL's default
    -- naming for inline UNIQUE on a column).
    IF EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'builds_build_hash_key'
    ) THEN
        ALTER TABLE builds DROP CONSTRAINT builds_build_hash_key;
        RAISE NOTICE 'Dropped builds_build_hash_key — multi-target releases share build_hash by design (CIRISRegistry#14)';
    END IF;
END $$;

-- The non-unique idx_builds_build_hash index from migration 012 is
-- preserved as-is for fast lookup-by-hash (get_build_by_hash). Dropping
-- the UNIQUE constraint does NOT drop the index when they were created
-- separately; verify and add if missing.
CREATE INDEX IF NOT EXISTS idx_builds_build_hash ON builds(build_hash);
