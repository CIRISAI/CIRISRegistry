-- Migration 011: Add approved_adapters and org_id to agents (v1.3.0)
-- approved_adapters: runtime adapters this agent build may load
-- org_id: owning organization for signing key lookup

ALTER TABLE agents ADD COLUMN IF NOT EXISTS approved_adapters TEXT[] DEFAULT '{}';
ALTER TABLE agents ADD COLUMN IF NOT EXISTS org_id TEXT;

UPDATE agents SET approved_adapters = '{}' WHERE approved_adapters IS NULL;

CREATE INDEX IF NOT EXISTS idx_agents_org_id ON agents(org_id);
