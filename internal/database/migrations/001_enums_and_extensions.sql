-- CIRISRegistry Database Schema
-- Migration 001: Extensions and Enums
-- PostgreSQL 15+ required for active/active replication compatibility

-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";      -- UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";       -- Cryptographic functions

-- ============================================================================
-- ENUMS (matching proto definitions)
-- ============================================================================

-- Autonomy tiers
CREATE TYPE autonomy_tier AS ENUM (
    'A0_ADVISORY',      -- Information and suggestions only
    'A1_LIMITED',       -- Low-risk automated actions
    'A2_MODERATE',      -- Supervised significant actions
    'A3_HIGH',          -- Independent professional actions
    'A4_CRITICAL'       -- Life-affecting decisions
);

-- Agent types
CREATE TYPE agent_type AS ENUM (
    'CIRISCARE',        -- Community health companion
    'CIRISMEDICAL',     -- Licensed medical deployment
    'CIRISLEGAL',       -- Licensed legal deployment
    'CIRISFINANCIAL',   -- Licensed financial deployment
    'CUSTOM'            -- Partner-specific build
);

-- Agent status
CREATE TYPE agent_status AS ENUM (
    'ACTIVE',           -- Build is current and approved
    'DEPRECATED',       -- Outdated but functional
    'REVOKED'           -- Compromised or unauthorized
);

-- License types
CREATE TYPE license_type AS ENUM (
    'COMMUNITY',                -- No professional capabilities
    'COMMUNITY_PLUS',           -- Limited enhanced features
    'PROFESSIONAL_MEDICAL',     -- Medical domain capabilities
    'PROFESSIONAL_LEGAL',       -- Legal domain capabilities
    'PROFESSIONAL_FINANCIAL',   -- Financial domain capabilities
    'PROFESSIONAL_FULL'         -- All professional capabilities
);

-- Partner status
CREATE TYPE partner_status AS ENUM (
    'ACTIVE',           -- License is valid
    'SUSPENDED',        -- Temporary hold
    'REVOKED'           -- License terminated
);

-- Revocation types
CREATE TYPE revocation_type AS ENUM (
    'AGENT_HASH',       -- Revoking an agent build
    'PARTNER_ID',       -- Revoking a partner
    'LICENSE_ID'        -- Revoking a specific license
);

-- Revocation reasons
CREATE TYPE revocation_reason AS ENUM (
    'SECURITY_COMPROMISED',     -- Agent or key compromised
    'LICENSE_VIOLATION',        -- Terms violation
    'PAYMENT_LAPSED',           -- Payment not received
    'VOLUNTARY_TERMINATION',    -- Partner requested termination
    'REGULATORY_ACTION',        -- Regulatory body required revocation
    'SAFETY_INCIDENT'           -- Patient safety incident
);

-- Revocation severity
CREATE TYPE revocation_severity AS ENUM (
    'ADMINISTRATIVE',           -- Business issue, may be resolved
    'COMPLIANCE',               -- Compliance issue
    'SECURITY_CRITICAL'         -- Security-critical, immediate action
);

-- Organization roles
CREATE TYPE org_role AS ENUM (
    'ORG_ADMIN',        -- Full organization management
    'ORG_KEY_MANAGER',  -- Can manage keys, cannot modify users
    'ORG_OPERATOR',     -- Can view, cannot modify
    'ORG_VIEWER'        -- Read-only access
);

-- Key status
CREATE TYPE key_status AS ENUM (
    'PENDING',          -- Generated, not yet activated
    'ACTIVE',           -- Current signing key
    'ROTATED',          -- Replaced, signatures still valid
    'REVOKED'           -- Compromised or manually revoked
);

-- Key custody model
CREATE TYPE key_custody_model AS ENUM (
    'CUSTODIED',        -- Portal holds private keys
    'SELF_SOVEREIGN'    -- Partner holds private keys
);

-- Audit action types
CREATE TYPE audit_action_type AS ENUM (
    -- Organization actions
    'ORG_CREATED',
    'ORG_UPDATED',
    'ORG_DEACTIVATED',
    -- User actions
    'USER_CREATED',
    'USER_UPDATED',
    'USER_DEACTIVATED',
    'USER_LOGIN',
    'USER_LOGOUT',
    -- Key actions
    'KEY_GENERATED',
    'KEY_ACTIVATED',
    'KEY_ROTATED',
    'KEY_REVOKED',
    'KEY_USED_FOR_SIGNING',
    -- Partner actions
    'PARTNER_REGISTERED',
    'PARTNER_UPDATED',
    'PARTNER_SUSPENDED',
    'PARTNER_REVOKED'
);
