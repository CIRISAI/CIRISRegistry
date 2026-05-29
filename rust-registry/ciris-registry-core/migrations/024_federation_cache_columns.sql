-- Migration 024: Federation cache columns on existing pubkey tables
--
-- Repurposes trusted_primitive_keys, partner_keys, and registry_signing_keys
-- as local caches over persist's federation_keys (when
-- FEDERATION_DUAL_WRITE_ENABLED=true). Pre-v1.4 behavior is unchanged when
-- the flag is off — these columns sit unused.
--
-- See `docs/FEDERATION_CLIENT.md` §"Cache shape" for the design.
--
-- Replication intent: per-region (this is a CACHE, not authoritative state;
-- each node's cache reflects its own local view of persist's state).
-- Authoritative state lives in persist's federation_keys table — replicated
-- across regions by persist's own Spock setup, NOT by registry's.
--
-- Defaults:
--   cached_at:               NOW()  (treats existing rows as freshly cached
--                                    at migration time so v1.4-rc1 reads
--                                    don't immediately TTL-expire)
--   cache_ttl_seconds:       300    (5 min, per FEDERATION_CLIENT.md)
--   persist_row_hash:        zero   (32 zero bytes — divergence telemetry
--                                    will fire on first read-through and
--                                    populate with the real hash)
--   persist_witnessed_at:    NULL   (not yet witnessed; populated on first
--                                    read-through from persist)

-- =============================================================================
-- trusted_primitive_keys
-- =============================================================================

ALTER TABLE trusted_primitive_keys
    ADD COLUMN IF NOT EXISTS cached_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ADD COLUMN IF NOT EXISTS cache_ttl_seconds    INTEGER     NOT NULL DEFAULT 300,
    ADD COLUMN IF NOT EXISTS persist_row_hash     BYTEA       NOT NULL DEFAULT decode('0000000000000000000000000000000000000000000000000000000000000000', 'hex'),
    ADD COLUMN IF NOT EXISTS persist_witnessed_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_trusted_primitive_keys_cached_at
    ON trusted_primitive_keys(cached_at);

COMMENT ON COLUMN trusted_primitive_keys.cached_at IS
    'When this row was last refreshed from persist (or row-creation time pre-federation). Used with cache_ttl_seconds for staleness checks.';
COMMENT ON COLUMN trusted_primitive_keys.cache_ttl_seconds IS
    'Per-row TTL override. Default 300s (5 min) per FEDERATION_CLIENT.md. Tunable per deployment.';
COMMENT ON COLUMN trusted_primitive_keys.persist_row_hash IS
    'sha256 of the federation_keys row this entry caches. Compared on read-through to detect divergence (federation_dual_write_divergence_total counter).';
COMMENT ON COLUMN trusted_primitive_keys.persist_witnessed_at IS
    'federation_keys.scrub_timestamp from persist at the moment we last fetched this row. NULL until the first read-through.';

-- =============================================================================
-- partner_keys
-- =============================================================================

ALTER TABLE partner_keys
    ADD COLUMN IF NOT EXISTS cached_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ADD COLUMN IF NOT EXISTS cache_ttl_seconds    INTEGER     NOT NULL DEFAULT 300,
    ADD COLUMN IF NOT EXISTS persist_row_hash     BYTEA       NOT NULL DEFAULT decode('0000000000000000000000000000000000000000000000000000000000000000', 'hex'),
    ADD COLUMN IF NOT EXISTS persist_witnessed_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_partner_keys_cached_at
    ON partner_keys(cached_at);

-- =============================================================================
-- registry_signing_keys
-- =============================================================================

ALTER TABLE registry_signing_keys
    ADD COLUMN IF NOT EXISTS cached_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ADD COLUMN IF NOT EXISTS cache_ttl_seconds    INTEGER     NOT NULL DEFAULT 300,
    ADD COLUMN IF NOT EXISTS persist_row_hash     BYTEA       NOT NULL DEFAULT decode('0000000000000000000000000000000000000000000000000000000000000000', 'hex'),
    ADD COLUMN IF NOT EXISTS persist_witnessed_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_registry_signing_keys_cached_at
    ON registry_signing_keys(cached_at);

-- =============================================================================
-- Spock replication intent (CIRISRegistry#4 convention)
-- =============================================================================
--
-- These three tables remain enrolled in their existing replication scope
-- (whatever it was pre-v1.4). The new cache columns piggyback on that
-- replication. If persist becomes the source of truth post-v1.4-cutover,
-- these caches CAN diverge per-region intentionally — each node's cache
-- reflects what it has read from persist, not what its peer node read.
-- That's the point: cache is per-node by design, even though
-- pre-federation these tables were Spock-replicated for redundancy.
--
-- No spock.repset_* calls in this migration — we don't change replication
-- scope, we just add columns to existing tables. Future migration that
-- removes these tables (v1.6.0 cache-only state) will handle Spock
-- de-enrollment then.
