# Federation Client — registry as policy layer over persist

**Status:** architectural sketch (v1.4 track). Companion to:
- CIRISPersist's [`docs/FEDERATION_DIRECTORY.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/docs/FEDERATION_DIRECTORY.md) (substrate definition)
- Registry's [`docs/TRUST_CONTRACT.md`](TRUST_CONTRACT.md) (existing consumer-facing trust contract)
- [`docs/THREAT_MODEL.md`](THREAT_MODEL.md) AV-14 closure path

**Not yet implemented.** This document captures the registry-side complement to persist's federation directory: how the registry consumes persist's substrate, what trust policy the registry composes, where the registry's existing admin RPCs map under federation, and what cache / failure / migration semantics the registry guarantees to its own consumers.

Cross-team coordination thread between the registry, persist, lens, and agent teams established the positions encoded here.

---

## TL;DR

Three rules, mirroring persist's three:

1. **Registry consumes; registry composes; registry attests.** Persist holds the substrate (pubkey rows, attestations, revocations); registry reads those rows, composes a policy verdict ("does this key verify build manifests for primitive X?"), and writes its own steward attestations back to persist when an admin operation issues one. Registry does **not** hold pubkeys as authoritative state.
2. **Cache, don't authoritatively store.** The existing `trusted_primitive_keys`, `partner_keys`, and `registry_signing_keys` tables become local read-through caches with bounded TTL. Authoritative state lives in persist's `federation_keys`. Cache survives persist outages up to a hard ceiling, then fails closed.
3. **Default to direct trust, evolve to score-weighted.** v1.4 ships steward-anchored direct trust (Persist's "Policy A") for backwards compat with today's TRUST_CONTRACT.md §2.3 case (i). Policies B and C (transitive referrer chains, Coherence Stake score-weighted consensus) are tracked for v1.5; the persist substrate exposes the edges, the registry composes the walk above.

---

## What changes from today

| Today (registry-as-authority) | Under federation (registry-as-peer) |
|---|---|
| `trusted_primitive_keys` table is the source of truth for "registry trusts primitive X to sign builds" | Cache. Authoritative state: `federation_attestations` rows where `attesting_key_id = registry-steward AND attestation_type = "vouches_for"`, joined to `federation_keys` for the pubkey bytes. |
| `partner_keys` table is source of truth for per-org keys | Cache. Authoritative state: `federation_keys` rows where `identity_type = "partner"`, with `federation_attestations` recording which stewards/peers vouch. |
| `registry_signing_keys` table is source of truth for the registry's own steward keys | Cache. Authoritative state: `federation_keys` row where `identity_type = "steward" AND identity_ref = "registry"`, self-signed (the bootstrap row per persist's doc §"Schema sketch — Bootstrap"). |
| `RegisterTrustedPrimitiveKey` RPC is an *issuance* call (write to `trusted_primitive_keys`) | *Attestation* call: write `federation_attestations.put(attesting_key_id=registry-steward, attested_key_id=primitive-key, type="vouches_for")`. The pubkey row itself is published by the primitive's own CI (per persist Q2 resolution: self-publish + post-hoc attestation). |
| `RevokeTrustedPrimitiveKey` RPC writes a row to a registry-local revocation set | Writes `federation_revocations.put(revoking_key_id=registry-steward, revoked_key_id=primitive-key, reason=…)`. Consumers compute "is revoked" from the federation table per their policy. |
| `ListTrustedPrimitiveKeys` returns a registry-local table dump | Returns the join of `federation_keys` ⨝ `federation_attestations WHERE attesting_key_id = registry-steward AND type = "vouches_for"`. |
| Verify endpoint (`POST /v1/verify/build-manifest`) looks up trusted key from registry-local table | Looks up via cache (TTL); on miss, fetches from persist's federation tables; runs Policy A by default; returns verdict + cache-age telemetry to consumer. |

**What survives unchanged from the consumer's perspective**: `GET /v1/steward-key`, `GET /v1/verify/function-manifest/{ver}/{target}?project=...`, `GET /v1/verify/build-manifest/{primitive}/{ver}/{target}` (when Path B ships), and the auth model (`REGISTRY_ADMIN_TOKEN` HTTP, HS256 JWT gRPC) all behave identically. Federation is a backend swap, not a wire-format change.

---

## Cache shape

### Layout

The existing `trusted_primitive_keys` / `partner_keys` / `registry_signing_keys` tables stay in the registry's PostgreSQL DB but are repurposed as cache. Each row gains four columns:

```sql
ALTER TABLE trusted_primitive_keys
    ADD COLUMN cached_at        timestamptz NOT NULL DEFAULT NOW(),
    ADD COLUMN cache_ttl_seconds integer    NOT NULL DEFAULT 300,
    ADD COLUMN persist_row_hash bytea       NOT NULL,  -- sha256 of the federation_keys row we cached
    ADD COLUMN persist_witnessed_at timestamptz NOT NULL;  -- federation_keys.scrub_timestamp from persist
```

`cache_ttl_seconds` is the TTL beyond which the row is considered stale and must be re-fetched from persist before it can be served to a verify request. Default 300s (5 min); operators can tune per deployment.

`persist_row_hash` lets the registry detect divergence: on every read-through, the registry hashes the freshly-fetched `federation_keys` row and compares against `persist_row_hash`. Mismatch → cache invalidation + telemetry counter increment.

### Invalidation strategy

Three triggers, in order of preference:

1. **Invalidate-on-attestation (registry's own writes)**: when registry calls `RegisterTrustedPrimitiveKey` or `RevokeTrustedPrimitiveKey`, it writes to persist first, then immediately re-fetches the affected `federation_keys` rows and updates the local cache with the new `cached_at`. Pre-warming with zero staleness for the common case (registry IS the writer).
2. **TTL-based re-fetch (default 5 min)**: cache rows older than `cache_ttl_seconds` are re-fetched on next read. Bounded staleness for peer-published changes.
3. **Optional `PG NOTIFY` pubsub (v1.5)**: persist publishes a notification on every `federation_keys` insert/update; registry subscribes and pre-warms. Not in v1.4 — adds infrastructure that hurts single-node dev. Tracked as v1.5 if the 5-minute TTL becomes operationally painful.

### Telemetry

New metrics under the `federation_` namespace:

- `federation_cache_hits_total{table}` — cached row served within TTL, no persist round-trip.
- `federation_cache_misses_total{table, reason="ttl_expired"|"invalidated"|"absent"}` — fell through to persist.
- `federation_dual_write_divergence_total{table}` — registry's local row differs from persist's authoritative row on read-through. Should be 0 in steady state; non-zero is a strong signal something is wrong.
- `federation_cache_age_seconds{table, quantile}` — distribution of cache-row ages at read time. Useful for tuning `cache_ttl_seconds`.
- `federation_persist_request_latency_seconds{operation, quantile}` — round-trip latency to persist for cache misses + writes. Informs whether to dial up `cache_ttl_seconds` if persist is consistently slow.

---

## Failure modes

### Two operator knobs

```
PERSIST_REQUIRED                  bool    default false
PERSIST_MAX_STALE_CACHE_SECONDS   int     default 3600  (1 hour)
```

### Decision matrix for verify requests when persist is unreachable

| Cache state | `PERSIST_REQUIRED=false` (permissive) | `PERSIST_REQUIRED=true` (strict) |
|---|---|---|
| Cache hit, age < `PERSIST_MAX_STALE_CACHE_SECONDS` | Serve from cache; `cache_age_seconds` in response | Fail-closed: 503 with `Retry-After` |
| Cache hit, age ≥ `PERSIST_MAX_STALE_CACHE_SECONDS` | Fail-closed: 503 with `Retry-After` | Fail-closed: 503 with `Retry-After` |
| Cache miss (key not previously cached) | Fail-closed: 404 with `cause: persist_unreachable` | Fail-closed: 503 with `Retry-After` |

The hard ceiling on cache age is the backstop: even in permissive mode, registry refuses to serve cache older than `PERSIST_MAX_STALE_CACHE_SECONDS`. This bounds the attack window for "deliberate persist outage to extend the validity of a key that has been peer-revoked."

### Verify-response shape under federation

Consumers see a new optional field in the verify response:

```json
{
  "primitive": "ciris-persist",
  "binary_version": "0.2.0",
  "target": "x86_64-unknown-linux-gnu",
  "manifest_hash": "sha256:...",
  "verifying_key_fingerprint": "abc123...",
  "federation_provenance": {
    "policy": "direct_trust_steward",
    "cache_age_seconds": 47,
    "persist_row_hash": "sha256:..."
  }
}
```

`federation_provenance.policy` is a string identifying which trust policy the registry composed (`direct_trust_steward` is Policy A; v1.5 may add `transitive_referrer` or `coherence_stake`). `cache_age_seconds` lets consumers decide whether to trust the verdict for their own freshness threshold (CIRISVerify's pubkey-pinning cache, e.g.).

### Telemetry alarms

Operators should set alarms on:

- `federation_persist_request_latency_seconds{quantile="0.99"} > 500ms` — persist is slow; cache hit ratio about to drop.
- `federation_cache_age_seconds{quantile="0.99"} > 0.5 * PERSIST_MAX_STALE_CACHE_SECONDS` — half the cache is approaching the ceiling; persist may be unreachable.
- `federation_dual_write_divergence_total > 0` — divergence detected; investigate immediately.
- `verify_requests_failed_total{cause="persist_unreachable"} > 0` — fail-closed responses being emitted; persist outage in progress.

---

## Trust policy (default: Policy A — direct trust on steward attestation)

The registry's verify endpoint composes its verdict by calling the equivalent of persist's Policy A from the directory doc:

```rust
async fn verify_primitive_key(
    persist: &dyn FederationDirectory,
    cache: &Cache,
    key_id: &str,
    expected_primitive: BuildPrimitive,
) -> VerifyResult {
    // 1. Get the key's federation_keys row (cache + persist fallback).
    let key = cache.get_or_fetch(persist, key_id).await?;

    // 2. Check identity_ref matches the primitive we're expecting.
    if key.identity_type != "primitive" || key.identity_ref != expected_primitive.project_name() {
        return VerifyResult::WrongPrimitive;
    }

    // 3. Look for unrevoked direct-trust attestation from registry-steward.
    let attestations = persist.list_attestations_for(key_id).await?;
    let revocations = persist.revocations_for(key_id).await?;
    let now = Utc::now();

    let revoked = revocations.iter().any(|r| {
        r.effective_at <= now
            && (r.revoking_key_id == STEWARD_KEY_ID || known_peer_steward(&r.revoking_key_id))
    });
    if revoked { return VerifyResult::Revoked; }

    let trusted = attestations.iter().any(|a| {
        a.attestation_type == "vouches_for"
            && a.attesting_key_id == STEWARD_KEY_ID
            && a.expires_at.map_or(true, |t| t > now)
    });

    if trusted { VerifyResult::Verified } else { VerifyResult::Untrusted }
}
```

Identical semantics to today's `trusted_primitive_keys` lookup; the only difference is *where the data comes from*. CIRISVerify clients pinning the registry's `/v1/steward-key` continue to see the same trust property because the registry IS the steward whose attestation is being checked.

### Why not Policy B or C in v1.4

- **Policy B (transitive referrer chains)**: requires a notion of "trusted root keys other than the registry's own steward" — the lens, agent, and persist stewards become co-equal trust roots. That's a TRUST_CONTRACT.md change CIRISVerify needs to absorb, and it should ship after at least one peer steward (lens / agent) is in the federation directory with attestations of its own. Track for v1.5.
- **Policy C (Coherence Stake score-weighted consensus)**: requires `peer_weights` input from CIRISLens's scoring layer. The PoB §2.4 N_eff scoring is operational on the lens side but isn't exposed as a registry-consumable API yet. v1.5 once the lens scoring API is stable.

v1.4 ships Policy A only; the cache + persist substrate is the load-bearing change.

---

## Write paths — where today's admin RPCs go under federation

### `RegisterTrustedPrimitiveKey`

Today: writes a row to `trusted_primitive_keys`.

Under federation:
1. Validate the request (admin JWT, project name slug, pubkey lengths) — unchanged.
2. **Do not write `federation_keys` itself.** That row is published by the primitive's own CI (per persist Q2 resolution: self-publish). Registry assumes the primitive has already published the key to persist via its own CI workflow.
3. Write `federation_attestations.put({ attesting_key_id: registry-steward, attested_key_id: primitive-key, attestation_type: "vouches_for", weight: 1.0, asserted_at: now })` with the registry steward's scrub-signature.
4. Pre-warm the local cache by re-fetching the joined `federation_keys` ⨝ `federation_attestations` view.
5. Audit log entry: `AUDIT_TRUSTED_KEY_ATTESTED` (new enum value).

If the primitive's `federation_keys` row doesn't exist when registry tries to attest, the call fails with `FailedPrecondition: "primitive has not published its federation_keys row yet"`. Operators (or CI) re-run after the primitive's self-publish lands.

### `RevokeTrustedPrimitiveKey`

Today: writes a row to revocations + marks `trusted_primitive_keys.revoked_at`.

Under federation:
1. Validate request — unchanged.
2. Write `federation_revocations.put({ revoking_key_id: registry-steward, revoked_key_id: primitive-key, reason, revoked_at: now, effective_at: now })`.
3. Invalidate local cache for the affected key.
4. Audit log entry: `AUDIT_TRUSTED_KEY_REVOKED` (existing).

### `ListTrustedPrimitiveKeys`

Returns the join of `federation_keys WHERE identity_type='primitive'` ⨝ `federation_attestations WHERE attesting_key_id=registry-steward AND attestation_type='vouches_for'`, with `federation_revocations` filtered out.

The wire format (proto `TrustedPrimitiveKey` message) stays identical — clients see no difference. Registry computes the join on read.

### Per-primitive write quota (operational rate-limit)

To bound storage growth from any single primitive's CI publishing keys, persist enforces a per-primitive quota:

```
PERSIST_FEDERATION_KEYS_PER_PRIMITIVE_PER_DAY  default 10
```

Most primitives will publish 1-2 keys per quarter (rotation cadence). 10/day leaves headroom for rotation drills + emergency re-publish without enabling spam. Excess writes return 429 + `Retry-After: 86400`.

This sits alongside the per-source-IP rate limit (existing in registry's `rate_limiter`, persist mirrors). Two-axis defense: rate-by-network and quota-by-identity.

---

## Steward identity under federation

### Registry's steward becomes a federation steward

The registry's existing steward key (currently boot-seeded into `trusted_primitive_keys` as `project='ciris-registry'`, fingerprint `567513a0c139412b...`) is published as a `federation_keys` row with `identity_type='steward'`, `identity_ref='registry'`. The row is self-signed (`scrub_key_id == key_id` — the bootstrap case from persist's doc).

Other federation peers (lens-steward, agent-steward, persist-steward) become co-equal stewards in the same directory, each self-signing their own bootstrap rows. Consumers anchor "trust the registry steward" out-of-band exactly as today (TRUST_CONTRACT.md §3.3) — but the consumer can ALSO anchor "trust the lens steward" / "trust the agent steward" if its policy wants to compose Policy B (transitive) or C (consensus).

### `/v1/steward-key` continues to work

The endpoint reads from the local cache (which reads from persist on miss). Same wire format. Same fingerprint stability guarantees per TRUST_CONTRACT.md §3.3. Same persistent-across-restarts property per AV-28.

### Rotation

Steward-key rotation under federation:
1. Operator generates new keypair, mounts new seeds on both regions.
2. Registry boots with new keypair.
3. Registry writes a new `federation_keys` row for the new steward identity, self-signed.
4. Registry writes `federation_attestations.put({ attesting_key_id: OLD_STEWARD, attested_key_id: NEW_STEWARD, attestation_type: "delegated_to" })` to chain the new identity to the old (federation peers can verify the rotation didn't drop trust on the floor).
5. Optionally writes `federation_revocations.put({ revoking_key_id: OLD_STEWARD, revoked_key_id: OLD_STEWARD })` to retire the old key.

Consumers that pinned the old fingerprint: their next `/v1/steward-key` re-fetch surfaces the new pubkey; their policy code can verify the chained attestation from old → new before re-pinning.

---

## Migration timeline

Tracks persist's v0.2.x → v0.3.x cadence per cross-team agreement:

| Persist version | Registry version | What lands registry-side |
|---|---|---|
| v0.2.0 | v1.4.0 | Feature flag `FEDERATION_DUAL_WRITE_ENABLED=false` (default off). Code path exists but behind flag. New `federation_*` cache columns added via migration 024. Telemetry counters wired to `/metrics`. |
| v0.2.x | v1.4.x | Selectively enable `FEDERATION_DUAL_WRITE_ENABLED=true` on staging. Validate divergence counter stays at zero across realistic admin operations. Tune `cache_ttl_seconds` based on observed `federation_cache_age_seconds` distribution. |
| v0.3.0 | v1.4.x | Read-path cutover: cache reads fall through to persist on miss, not just on TTL expiry for unknown keys. Registry-local tables become strictly write-through cache. |
| v0.3.x | v1.5.0 | TRUST_CONTRACT.md update with Path A1/A2 split + Path D for consumer-aggregated multi-peer attestations. Optional Policy B (transitive) shipped behind opt-in flag. |
| v0.4.x | v1.6.0 | Drop registry-local tables entirely. Cache-only state. Policy C (Coherence Stake) shipped if lens scoring API is stable. |

Each step is independently rollback-able: the feature flag at each phase lets registry revert to the prior behavior without persist version coordination.

---

## Open questions for follow-up

These are gating questions registry needs answers to **before** v1.4 ships:

1. **Bootstrap order**: when a fresh deployment comes up with empty caches, what's the order of operations to populate the registry-steward row in `federation_keys` vs the cache vs the steward-key file mount? Per persist Q2, registry's CI self-publishes; registry's boot code asserts the row exists (or writes it if missing on first boot).

2. **Backfill of historical attestations**: the existing `trusted_primitive_keys` table has rows for all 5 primitives that bridge-ops registered today. Migration 024 should backfill those into `federation_attestations` on the cutover boot, with `attesting_key_id=registry-steward, attestation_type="vouches_for"`. Without this, a fresh registry post-cutover would correctly show "no trusted keys" but verify-side would break for everyone.

3. **Policy versioning in the verify response**: if v1.5 introduces Policy B and operators flag-flip to it, the response shape changes (`federation_provenance.policy: "transitive_referrer"`). CIRISVerify's consumer code needs to know which policies it's prepared to honor. Worth a registry-side enum + a CIRISVerify-side allowlist negotiation. Track for v1.5.

4. **Audit-log shape under federation**: today the `audit_log` table has `actor_org_id`. Under federation, the actor of a key registration is a federation peer (registry-steward, lens-steward, etc.) — not an org. Either widen `actor_org_id` to allow steward identities, or add a parallel `actor_steward_key_id` column. Lower-priority; track for v1.5.

5. **CIRISPortal coordination**: Portal's admin UI currently calls `RegisterTrustedPrimitiveKey` directly. Post-federation, the call shape stays the same (Portal still issues a registry-steward attestation) but Portal needs to be aware that the underlying storage moved. Worth a heads-up issue once v1.4 is close to landing.

---

## What's not in this doc

- **Implementation details.** No PR-ready code; no migration SQL beyond the cache-column sketch in §"Cache shape"; no test plan. Those come once persist v0.2.0 is shipped and we can wire registry's dual-write against a real schema.
- **CIRISLens or CIRISAgent integration.** This doc covers registry-as-consumer of persist. Lens and agent will be peer-stewards in the federation directory but their consumer code lives in their own repos.
- **Coherence Stake math.** Policy C in §"Trust policy" is a sketch; the actual scoring math lives in CIRISLens's scoring API and is referenced from PoB §4 / §2.4.

This doc is a **client-side surface alignment artifact** — paired with persist's `FEDERATION_DIRECTORY.md`, the two together define the registry ↔ persist contract for v1.4 onward.

---

## Update cadence

Updated:
- On every persist version bump within the v0.2.x ↔ v0.3.x window (track schema drift).
- On every registry policy addition (Policy B, C, future).
- On every TRUST_CONTRACT.md revision (kept consistent with consumer-facing trust shape).

Last updated: 2026-05-02 (initial draft, post-persist-FEDERATION_DIRECTORY.md alignment thread).
