-- Migration 016: Change function_manifests PK to composite key for multi-region safety
--
-- The SERIAL 'id' column causes conflicts in bi-directional replication because
-- both regions generate conflicting auto-increment values.
--
-- Solution: Use (binary_version, target) as the primary key since it's already
-- unique and is the natural identifier for a function manifest.

-- Step 1: Drop the existing primary key constraint
ALTER TABLE function_manifests DROP CONSTRAINT IF EXISTS function_manifests_pkey;

-- Step 2: Drop the SERIAL id column (no longer needed)
ALTER TABLE function_manifests DROP COLUMN IF EXISTS id;

-- Step 3: Add composite primary key using the existing unique constraint columns
-- First drop the existing unique constraint since PK will enforce uniqueness
ALTER TABLE function_manifests DROP CONSTRAINT IF EXISTS function_manifests_binary_version_target_key;

-- Add the composite primary key
ALTER TABLE function_manifests ADD PRIMARY KEY (binary_version, target);

-- Note: manifest_uuid from migration 015 is kept for external references if needed
