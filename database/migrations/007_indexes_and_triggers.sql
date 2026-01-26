-- CIRISRegistry Database Schema
-- Migration 007: Additional Indexes and Triggers

-- ============================================================================
-- UPDATED_AT TRIGGERS
-- ============================================================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply to all tables with updated_at
CREATE TRIGGER update_agents_updated_at
    BEFORE UPDATE ON agents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_partners_updated_at
    BEFORE UPDATE ON partners
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_organizations_updated_at
    BEFORE UPDATE ON organizations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_org_users_updated_at
    BEFORE UPDATE ON org_users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- ADDITIONAL PERFORMANCE INDEXES
-- ============================================================================

-- Agents
CREATE INDEX idx_agents_status ON agents(status);
CREATE INDEX idx_agents_type ON agents(agent_type);
CREATE INDEX idx_agents_registered ON agents(registered_at DESC);

-- Partners
CREATE INDEX idx_partners_status ON partners(status);
CREATE INDEX idx_partners_license_type ON partners(license_type);
CREATE INDEX idx_partners_expires ON partners(expires_at);

-- Organizations
CREATE INDEX idx_organizations_active ON organizations(active) WHERE active = TRUE;
CREATE INDEX idx_organizations_oauth_domain ON organizations(oauth_domain) WHERE oauth_domain IS NOT NULL;

-- Users
CREATE INDEX idx_org_users_active ON org_users(active) WHERE active = TRUE;
CREATE INDEX idx_org_users_role ON org_users(org_id, role);
CREATE INDEX idx_org_users_last_login ON org_users(last_login_at DESC NULLS LAST);

-- Keys
CREATE INDEX idx_partner_keys_status ON partner_keys(status);
CREATE INDEX idx_partner_keys_created ON partner_keys(created_at DESC);

-- ============================================================================
-- FULL TEXT SEARCH (Optional)
-- ============================================================================

-- Organization search
CREATE INDEX idx_organizations_search ON organizations
    USING GIN (to_tsvector('english', name || ' ' || COALESCE(legal_name, '')));

-- ============================================================================
-- REPLICATION SAFETY
-- ============================================================================

-- For active/active replication, ensure no sequence conflicts
-- The bridge team manages replication, but we prepare the schema

-- Use UUID primary keys (already done) to avoid sequence conflicts
-- All PKs are UUIDs except agents (hash-based) and registry_snapshots (bigserial)

-- For registry_snapshots, if active/active is needed:
-- ALTER SEQUENCE registry_snapshots_snapshot_version_seq
--     INCREMENT BY 2 START WITH 1;  -- Odd on primary, even on replica

COMMENT ON TABLE agents IS 'Uses content-hash PK, safe for replication';
COMMENT ON TABLE partners IS 'Uses UUID PK, safe for replication';
COMMENT ON TABLE organizations IS 'Uses UUID PK, safe for replication';
COMMENT ON TABLE org_users IS 'Uses UUID PK, safe for replication';
COMMENT ON TABLE partner_keys IS 'Uses UUID PK, safe for replication';
