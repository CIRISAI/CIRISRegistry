-- Migration 015: Convert SERIAL primary keys to UUID for multi-region safety
-- SERIAL auto-increment IDs can conflict when multiple regions have write access.
-- UUIDs are globally unique and safe for concurrent writes from any region.

-- ============================================================================
-- 1. revocations - Add UUID and use revoked_at for delta queries
-- ============================================================================
-- The SERIAL 'id' is problematic for multi-region because:
-- - Delta queries use WHERE id > $1
-- - MAX(id) is used for revision tracking
-- - Different regions will generate conflicting IDs
--
-- Solution: Use revoked_at timestamp for delta queries instead of SERIAL id.
-- UUIDs provide insert uniqueness across regions.

ALTER TABLE revocations ADD COLUMN IF NOT EXISTS revocation_id UUID DEFAULT gen_random_uuid();

-- Backfill existing rows
UPDATE revocations SET revocation_id = gen_random_uuid() WHERE revocation_id IS NULL;

-- Make NOT NULL
ALTER TABLE revocations ALTER COLUMN revocation_id SET NOT NULL;

-- Create unique index for UUID
CREATE UNIQUE INDEX IF NOT EXISTS idx_revocations_uuid ON revocations(revocation_id);

-- Index for timestamp-based delta queries (replaces id-based queries)
CREATE INDEX IF NOT EXISTS idx_revocations_revoked_at ON revocations(revoked_at);

-- Note: Application code should migrate from:
--   WHERE id > $since_id  →  WHERE revoked_at > $since_timestamp
--   MAX(id)               →  MAX(revoked_at) or COUNT(*)

-- ============================================================================
-- 2. registry_snapshots - Add UUID column
-- ============================================================================
ALTER TABLE registry_snapshots ADD COLUMN IF NOT EXISTS snapshot_uuid UUID DEFAULT gen_random_uuid();

UPDATE registry_snapshots SET snapshot_uuid = gen_random_uuid() WHERE snapshot_uuid IS NULL;

ALTER TABLE registry_snapshots ALTER COLUMN snapshot_uuid SET NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_snapshots_uuid ON registry_snapshots(snapshot_uuid);

-- ============================================================================
-- 3. audit_logs - Add UUID column (keep BIGSERIAL for ordering)
-- ============================================================================
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS log_uuid UUID DEFAULT gen_random_uuid();

UPDATE audit_logs SET log_uuid = gen_random_uuid() WHERE log_uuid IS NULL;

ALTER TABLE audit_logs ALTER COLUMN log_uuid SET NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_audit_logs_uuid ON audit_logs(log_uuid);

-- ============================================================================
-- 4. function_manifests - Convert to UUID primary key
-- ============================================================================
-- Add UUID column
ALTER TABLE function_manifests ADD COLUMN IF NOT EXISTS manifest_uuid UUID DEFAULT gen_random_uuid();

UPDATE function_manifests SET manifest_uuid = gen_random_uuid() WHERE manifest_uuid IS NULL;

ALTER TABLE function_manifests ALTER COLUMN manifest_uuid SET NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_function_manifests_uuid ON function_manifests(manifest_uuid);

-- ============================================================================
-- Recommendations for application code:
--
-- 1. For new inserts, always generate UUID client-side or use DEFAULT
-- 2. The SERIAL 'id' columns are kept for:
--    - Backward compatibility with existing queries
--    - Ordering (audit_logs, revocations revision tracking)
-- 3. Use the UUID columns for:
--    - Primary identification in multi-region scenarios
--    - Upsert conflict detection (ON CONFLICT)
-- ============================================================================
