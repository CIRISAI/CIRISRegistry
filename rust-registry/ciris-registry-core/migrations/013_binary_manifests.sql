-- Migration 013: Binary manifests for CIRISVerify self-verification
-- Stores SHA-256 hashes of CIRISVerify binaries for each platform/version.
-- Used by ciris-verify self-check to verify binary integrity (Level 2).

CREATE TABLE IF NOT EXISTS binary_manifests (
  manifest_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  version TEXT NOT NULL,                          -- Semantic version e.g. "0.5.2"
  binaries JSONB NOT NULL,                        -- Map of target triple -> sha256 hash
  generated_at TIMESTAMPTZ NOT NULL,              -- When manifest was generated (from CI)
  registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  registered_by TEXT,                             -- User/system that registered
  source TEXT,                                    -- 'github_release', 'ci_push', 'manual'
  notes TEXT,
  CONSTRAINT binary_manifests_version_unique UNIQUE (version)
);

CREATE INDEX IF NOT EXISTS idx_binary_manifests_version ON binary_manifests(version);

-- Example binaries JSONB:
-- {
--   "x86_64-unknown-linux-gnu": "sha256:7d36f92ca90116c184024a0f03af7cec12551c609f78de62ced5e3cffd238de3",
--   "aarch64-apple-darwin": "sha256:abc123def456..."
-- }
