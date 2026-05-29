-- Migration 021: Project namespace for build/manifest tables
--
-- Adds a `project` discriminator column to builds, binary_manifests, and
-- function_manifests so non-agent CIRIS primitives (ciris-persist,
-- ciris-lens, future peers) can register their own builds without
-- colliding with ciris-agent's version namespace.
--
-- Closes:
--   - GitHub issue CIRISAI/CIRISRegistry#1
--   - THREAT_MODEL.md AV-1 (project-name collision blocks non-agent peers)
--
-- Design:
--   - DEFAULT 'ciris-agent' on the column → all existing rows backfill
--     automatically; existing CIRISAgent v0.1.7 stays at v0.1.7 under
--     project=ciris-agent, no app changes required.
--   - Unique constraints lead with `project` so each project gets its
--     own version namespace.
--   - Project name is validated at the application layer (^[a-z][a-z0-9-]{0,63}$).
--
-- Lesson from migration 020 fix (commit 07f50b6): PostgreSQL does not
-- support `ADD CONSTRAINT IF NOT EXISTS`. Use DO $$ ... $$ blocks for
-- conditional constraint operations.

-- ============================================================================
-- builds
-- ============================================================================

ALTER TABLE builds
    ADD COLUMN IF NOT EXISTS project TEXT NOT NULL DEFAULT 'ciris-agent';

CREATE INDEX IF NOT EXISTS idx_builds_project ON builds(project);

-- Replace UNIQUE(version, includes_modules) with UNIQUE(project, version, includes_modules)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'builds_version_modules_unique'
    ) THEN
        ALTER TABLE builds DROP CONSTRAINT builds_version_modules_unique;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'builds_project_version_modules_unique'
    ) THEN
        ALTER TABLE builds
            ADD CONSTRAINT builds_project_version_modules_unique
            UNIQUE (project, version, includes_modules);
    END IF;
END $$;

-- Note: builds.build_hash UNIQUE constraint is preserved as-is. Build hash is a
-- SHA-256 of source files; it's globally unique by construction regardless of
-- project. Two projects cannot legitimately produce the same build hash.

-- ============================================================================
-- binary_manifests
-- ============================================================================

ALTER TABLE binary_manifests
    ADD COLUMN IF NOT EXISTS project TEXT NOT NULL DEFAULT 'ciris-agent';

CREATE INDEX IF NOT EXISTS idx_binary_manifests_project ON binary_manifests(project);

-- Replace UNIQUE(version) with UNIQUE(project, version)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'binary_manifests_version_unique'
    ) THEN
        ALTER TABLE binary_manifests DROP CONSTRAINT binary_manifests_version_unique;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'binary_manifests_project_version_unique'
    ) THEN
        ALTER TABLE binary_manifests
            ADD CONSTRAINT binary_manifests_project_version_unique
            UNIQUE (project, version);
    END IF;
END $$;

-- ============================================================================
-- function_manifests
-- ============================================================================

ALTER TABLE function_manifests
    ADD COLUMN IF NOT EXISTS project TEXT NOT NULL DEFAULT 'ciris-agent';

CREATE INDEX IF NOT EXISTS idx_function_manifests_project ON function_manifests(project);

-- Migration 016 made (binary_version, target) the primary key. Replace it
-- with (project, binary_version, target) so per-project namespaces hold.
DO $$
BEGIN
    -- Drop the existing PK if present
    IF EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'function_manifests_pkey'
    ) THEN
        ALTER TABLE function_manifests DROP CONSTRAINT function_manifests_pkey;
    END IF;

    -- Add the new project-scoped PK
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'function_manifests_pkey'
          AND contype = 'p'
    ) THEN
        ALTER TABLE function_manifests
            ADD CONSTRAINT function_manifests_pkey
            PRIMARY KEY (project, binary_version, target);
    END IF;
END $$;

COMMENT ON COLUMN builds.project IS 'CIRIS primitive name (kebab-case). Default: ciris-agent.';
COMMENT ON COLUMN binary_manifests.project IS 'CIRIS primitive name (kebab-case). Default: ciris-agent.';
COMMENT ON COLUMN function_manifests.project IS 'CIRIS primitive name (kebab-case). Default: ciris-agent.';
