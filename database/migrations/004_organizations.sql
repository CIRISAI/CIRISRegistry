-- CIRISRegistry Database Schema
-- Migration 004: Organization Management (CIRISPortal)

-- ============================================================================
-- ORGANIZATIONS
-- ============================================================================

CREATE TABLE organizations (
    -- Identity
    org_id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                    TEXT NOT NULL,              -- Display name
    legal_name              TEXT,                       -- Legal entity name
    tax_id                  TEXT,                       -- Tax ID / Registration number

    -- Link to partner license (if licensed)
    partner_id              UUID REFERENCES partners(partner_id) ON DELETE SET NULL,

    -- Contact
    primary_email           TEXT NOT NULL,
    billing_email           TEXT,
    technical_contact_email TEXT,
    compliance_contact_email TEXT,

    -- OAuth
    oauth_provider          TEXT NOT NULL DEFAULT 'google',
    oauth_domain            TEXT,                       -- Verified email domain (e.g., "acme.com")

    -- Status
    active                  BOOLEAN NOT NULL DEFAULT TRUE,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by              UUID,                       -- User ID who created

    -- Metadata (flexible key-value)
    metadata                JSONB NOT NULL DEFAULT '{}',

    -- Constraints
    CONSTRAINT unique_org_name UNIQUE (name),
    CONSTRAINT valid_email CHECK (primary_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

-- Index for partner lookup
CREATE INDEX idx_organizations_partner ON organizations(partner_id) WHERE partner_id IS NOT NULL;

COMMENT ON TABLE organizations IS 'Portal-managed organizations (may or may not have partner license)';

-- ============================================================================
-- ORGANIZATION USERS
-- ============================================================================

CREATE TABLE org_users (
    -- Identity
    user_id                 UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id                  UUID NOT NULL REFERENCES organizations(org_id) ON DELETE CASCADE,
    email                   TEXT NOT NULL,
    name                    TEXT NOT NULL,

    -- OAuth
    oauth_provider          TEXT NOT NULL DEFAULT 'google',
    oauth_subject           TEXT,                       -- OAuth sub claim (unique per provider)

    -- Role
    role                    org_role NOT NULL DEFAULT 'ORG_VIEWER',

    -- Status
    active                  BOOLEAN NOT NULL DEFAULT TRUE,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at           TIMESTAMPTZ,
    invited_by              UUID REFERENCES org_users(user_id) ON DELETE SET NULL,

    -- MFA
    mfa_enabled             BOOLEAN NOT NULL DEFAULT FALSE,
    mfa_method              TEXT,                       -- "totp", "webauthn", etc.
    mfa_secret_encrypted    BYTEA,                      -- Encrypted TOTP secret (if applicable)

    -- Constraints
    CONSTRAINT unique_email UNIQUE (email),
    CONSTRAINT unique_oauth UNIQUE (oauth_provider, oauth_subject),
    CONSTRAINT valid_user_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

-- Index for org lookups
CREATE INDEX idx_org_users_org ON org_users(org_id);
CREATE INDEX idx_org_users_email ON org_users(email);

COMMENT ON TABLE org_users IS 'Users within organizations, authenticated via OAuth';

-- ============================================================================
-- USER SESSIONS
-- ============================================================================

CREATE TABLE user_sessions (
    session_id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                 UUID NOT NULL REFERENCES org_users(user_id) ON DELETE CASCADE,

    -- OAuth tokens (encrypted)
    access_token_encrypted  BYTEA,
    refresh_token_encrypted BYTEA,
    id_token_encrypted      BYTEA,

    -- Session info
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at              TIMESTAMPTZ NOT NULL,
    last_activity_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Client info
    ip_address              INET,
    user_agent              TEXT,

    -- Status
    revoked                 BOOLEAN NOT NULL DEFAULT FALSE,
    revoked_at              TIMESTAMPTZ,
    revoked_reason          TEXT
);

-- Index for cleanup
CREATE INDEX idx_sessions_expires ON user_sessions(expires_at) WHERE NOT revoked;
CREATE INDEX idx_sessions_user ON user_sessions(user_id);

COMMENT ON TABLE user_sessions IS 'Active user sessions with encrypted OAuth tokens';
