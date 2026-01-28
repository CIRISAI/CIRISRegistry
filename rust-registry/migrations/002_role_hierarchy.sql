-- CIRISRegistry Role Hierarchy and Multi-Org Membership
-- Version: 1.2.0
--
-- This migration adds:
-- 1. Organization types and hierarchy (parent_org_id)
-- 2. System users (global admins)
-- 3. Multi-org user membership (users + user_org_memberships)

-- =============================================================================
-- 1. Organization Types and Hierarchy
-- =============================================================================

-- Add org_type column
-- 0 = UNSPECIFIED, 1 = INTERNAL, 2 = PARTNER, 3 = LICENSEE, 4 = COMMUNITY
ALTER TABLE organizations
ADD COLUMN IF NOT EXISTS org_type INTEGER NOT NULL DEFAULT 4;

-- Add parent_org_id for hierarchy
ALTER TABLE organizations
ADD COLUMN IF NOT EXISTS parent_org_id TEXT REFERENCES organizations(org_id);

-- Index for hierarchy queries
CREATE INDEX IF NOT EXISTS idx_organizations_parent ON organizations(parent_org_id);
CREATE INDEX IF NOT EXISTS idx_organizations_type ON organizations(org_type);

-- =============================================================================
-- 2. System Users (Global Admins)
-- =============================================================================

CREATE TABLE IF NOT EXISTS system_users (
    user_id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    -- SystemRole: 0=UNSPECIFIED, 1=SYSTEM_ADMIN, 2=SYSTEM_AUDITOR, 3=WISE_AUTHORITY
    role INTEGER NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by TEXT,
    -- SYSTEM_ADMIN must have @ciris.ai email
    CONSTRAINT system_admin_email_check CHECK (
        role != 1 OR email LIKE '%@ciris.ai'
    )
);

CREATE INDEX IF NOT EXISTS idx_system_users_email ON system_users(email);
CREATE INDEX IF NOT EXISTS idx_system_users_role ON system_users(role);

-- =============================================================================
-- 3. Multi-Org User Membership
-- =============================================================================

-- Users table (identity only, no org affiliation)
CREATE TABLE IF NOT EXISTS users (
    user_id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    oauth_provider TEXT,
    oauth_subject TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,
    mfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    mfa_method TEXT
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_active ON users(active);

-- Junction table: User <-> Organization with role
CREATE TABLE IF NOT EXISTS user_org_memberships (
    user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    org_id TEXT NOT NULL REFERENCES organizations(org_id) ON DELETE CASCADE,
    -- OrgRole: 0=UNSPECIFIED, 1=ORG_ADMIN, 2=KEY_MANAGER, 3=OPERATOR, 4=VIEWER
    role INTEGER NOT NULL DEFAULT 4,
    invited_by TEXT REFERENCES users(user_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, org_id)
);

CREATE INDEX IF NOT EXISTS idx_user_org_memberships_org ON user_org_memberships(org_id);
CREATE INDEX IF NOT EXISTS idx_user_org_memberships_role ON user_org_memberships(role);

-- =============================================================================
-- 4. Migrate Data from org_users
-- =============================================================================

-- Insert users (identity) from org_users
INSERT INTO users (user_id, email, name, oauth_provider, oauth_subject, active, created_at, updated_at, last_login_at, mfa_enabled, mfa_method)
SELECT user_id, email, name, oauth_provider, oauth_subject, active, created_at, updated_at, last_login_at, mfa_enabled, mfa_method
FROM org_users
ON CONFLICT (user_id) DO NOTHING;

-- Insert memberships from org_users
INSERT INTO user_org_memberships (user_id, org_id, role, invited_by, created_at, updated_at)
SELECT user_id, org_id, role, invited_by, created_at, updated_at
FROM org_users
ON CONFLICT (user_id, org_id) DO NOTHING;

-- =============================================================================
-- 5. Views for Backward Compatibility
-- =============================================================================

-- View that mimics the old org_users table structure
-- Portal can query this during transition
CREATE OR REPLACE VIEW org_users_compat AS
SELECT
    u.user_id,
    m.org_id,
    u.email,
    u.name,
    u.oauth_provider,
    u.oauth_subject,
    m.role,
    u.active,
    u.created_at,
    u.updated_at,
    u.last_login_at,
    m.invited_by,
    u.mfa_enabled,
    u.mfa_method
FROM users u
JOIN user_org_memberships m ON u.user_id = m.user_id;

-- =============================================================================
-- 6. Update audit_log to track system users
-- =============================================================================

ALTER TABLE audit_log
ADD COLUMN IF NOT EXISTS actor_system_user_id TEXT REFERENCES system_users(user_id);

-- Note: org_users table is preserved for now
-- It can be dropped in a future migration after Portal is updated
