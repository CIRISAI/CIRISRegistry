-- Fix permitted_actions column to have a DEFAULT and update existing NULLs.
-- The Rust AgentRow struct uses Vec<String> (non-optional) which can't decode NULL.

ALTER TABLE agents ALTER COLUMN permitted_actions SET DEFAULT '{}';
UPDATE agents SET permitted_actions = '{}' WHERE permitted_actions IS NULL;

-- Also fix other identity template columns to have sensible defaults
ALTER TABLE agents ALTER COLUMN identity_template SET DEFAULT '';
ALTER TABLE agents ALTER COLUMN stewardship_tier SET DEFAULT 0;
