-- Migration 017: Support multiple OAuth providers per user
--
-- Problem: Users logging in with Apple after Google (same email) get rejected
-- because email is unique but there's no way to link multiple OAuth identities.
--
-- Solution: Create oauth_identities tables to link multiple providers to one user.
-- Lookup order: (provider, subject) -> if not found, lookup by email -> link identity

-- =============================================================================
-- 1. OAuth identities for system_users (global admins)
-- =============================================================================
CREATE TABLE IF NOT EXISTS system_user_oauth_identities (
    user_id TEXT NOT NULL REFERENCES system_users(user_id) ON DELETE CASCADE,
    oauth_provider TEXT NOT NULL,      -- 'google', 'apple', 'github', etc.
    oauth_subject TEXT NOT NULL,       -- Provider's unique user ID
    email_at_link TEXT,                -- Email used when this identity was linked
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (oauth_provider, oauth_subject)
);

CREATE INDEX IF NOT EXISTS idx_system_user_oauth_user ON system_user_oauth_identities(user_id);

-- =============================================================================
-- 2. OAuth identities for regular users
-- =============================================================================
CREATE TABLE IF NOT EXISTS user_oauth_identities (
    user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    oauth_provider TEXT NOT NULL,
    oauth_subject TEXT NOT NULL,
    email_at_link TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (oauth_provider, oauth_subject)
);

CREATE INDEX IF NOT EXISTS idx_user_oauth_user ON user_oauth_identities(user_id);

-- =============================================================================
-- 3. Migrate existing OAuth data from users table to identities table
-- =============================================================================
INSERT INTO user_oauth_identities (user_id, oauth_provider, oauth_subject, email_at_link)
SELECT user_id, oauth_provider, oauth_subject, email
FROM users
WHERE oauth_provider IS NOT NULL AND oauth_subject IS NOT NULL
ON CONFLICT (oauth_provider, oauth_subject) DO NOTHING;

-- =============================================================================
-- Login flow (application code):
--
-- 1. Lookup (provider, subject) in oauth_identities table
-- 2. If found -> return linked user
-- 3. If not found -> lookup by email in users/system_users table
-- 4. If email exists -> link this (provider, subject) to existing user, return user
-- 5. If email not found -> create new user, link identity, return user
-- =============================================================================
