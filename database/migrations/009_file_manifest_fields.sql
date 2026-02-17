-- Migration 009: Build registry with file integrity manifests
-- Builds are separate from agent licenses. A build is a specific version of the
-- agent software (core engine + optional professional module files).
-- CIRISVerify fetches the manifest to validate file integrity at runtime.

CREATE TABLE IF NOT EXISTS builds (
  build_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  version TEXT NOT NULL,                          -- Semantic version e.g. "2.0.0"
  build_hash TEXT NOT NULL UNIQUE,                -- SHA-256 of all source files concatenated
  file_manifest_hash TEXT NOT NULL,               -- SHA-256 of the manifest JSON itself
  file_manifest_count INTEGER NOT NULL DEFAULT 0, -- Number of files in manifest
  file_manifest_json JSONB NOT NULL,              -- Full manifest: {"version":"...","files":{"path":"sha256",...}}
  includes_modules TEXT[] DEFAULT '{}',           -- e.g. {'core'} or {'core','medical','legal'}
  source_repo TEXT,                               -- Git repository URL
  source_commit TEXT,                             -- Git commit hash
  registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  registered_by TEXT,                             -- User who registered the build
  status TEXT NOT NULL DEFAULT 'active',          -- active, deprecated, revoked
  notes TEXT,
  CONSTRAINT builds_version_modules_unique UNIQUE (version, includes_modules)
);

CREATE INDEX IF NOT EXISTS idx_builds_version ON builds(version);
CREATE INDEX IF NOT EXISTS idx_builds_build_hash ON builds(build_hash);
CREATE INDEX IF NOT EXISTS idx_builds_status ON builds(status);

COMMENT ON TABLE builds IS
  'Registered agent builds with Tripwire-style file integrity manifests. Separate from agent licenses.';
COMMENT ON COLUMN builds.build_hash IS
  'SHA-256 of all source files sorted and concatenated. Uniquely identifies this build.';
COMMENT ON COLUMN builds.file_manifest_json IS
  'Full per-file SHA-256 manifest. Served to CIRISVerify for runtime integrity checks.';
COMMENT ON COLUMN builds.includes_modules IS
  'Which modules are included: core (always), plus optional medical, legal, financial.';
