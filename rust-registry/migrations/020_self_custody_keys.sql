-- Self-Custody Key Registration (FSD-002)
-- Version: 1.3.0
--
-- Adds support for agent-generated keys where only public keys are stored.
-- See FSD-002_SELF_CUSTODY_KEYS.md for full specification.

-- Registration challenges table
-- Stores 32-byte nonces issued for proof-of-possession during key registration
CREATE TABLE IF NOT EXISTS registration_challenges (
    org_id TEXT NOT NULL PRIMARY KEY,
    challenge BYTEA NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_registration_challenges_expires ON registration_challenges(expires_at);

-- Activation challenges table
-- Stores challenges for the second step of self-custody key activation
CREATE TABLE IF NOT EXISTS activation_challenges (
    key_id TEXT NOT NULL PRIMARY KEY,
    challenge BYTEA NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_activation_challenges_expires ON activation_challenges(expires_at);

-- Add public_key_hash column to partner_keys for duplicate detection
-- This enables O(1) lookup to detect if a public key is already registered
ALTER TABLE partner_keys
ADD COLUMN IF NOT EXISTS public_key_hash TEXT;

-- Create index for fast duplicate detection
CREATE INDEX IF NOT EXISTS idx_partner_keys_public_key_hash ON partner_keys(public_key_hash);

-- Add unique constraint to prevent same public key registered to multiple orgs
-- This is a critical security control for Sybil resistance
ALTER TABLE partner_keys
ADD CONSTRAINT IF NOT EXISTS unique_public_key_hash UNIQUE (public_key_hash);

-- Backfill public_key_hash for existing keys
UPDATE partner_keys
SET public_key_hash = encode(sha256(ed25519_public_key), 'hex')
WHERE public_key_hash IS NULL;
