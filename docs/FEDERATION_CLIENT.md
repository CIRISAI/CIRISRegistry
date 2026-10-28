# Federation Client — registry as policy layer over persist

**Status:** the registry-side complement to persist's federation directory. Companion to:
- CIRISPersist's [`docs/FEDERATION_DIRECTORY.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/docs/FEDERATION_DIRECTORY.md) (substrate definition)
- Registry's [`docs/TRUST_CONTRACT.md`](TRUST_CONTRACT.md) (consumer-facing trust contract)
- [`docs/THREAT_MODEL.md`](THREAT_MODEL.md) AV-14

This document describes how the registry consumes persist's substrate, what trust policy the registry composes, where the registry's admin RPCs map under federation, and what cache, failure and telemetry semantics the registry guarantees to its own consumers. The code is `ciris-registry-core/src/federation/` (the `FederationDirectory` client) and `config.rs::FederationSettings`.

---

## TL;DR

Three rules, mirroring persist's three:

1. **Registry consumes; registry composes; registry attests.** Persist holds the substrate (pubkey rows, attestations, revocations); registry reads those rows, composes a policy verdict ("does this key verify build manifests for primitive X?"), and writes its own attestations back to persist when an admin operation issues one. Registry does **not** hold pubkeys as authoritative state.
2. **Cache, don't authoritatively store.** The standalone binary's `trusted_primitive_keys`, `partner_keys`, and `registry_signing_keys` tables are local read-through caches with bounded TTL. Authoritative state lives in persist's `federation_keys`. Cache survives persist outages up to a hard ceiling, then fails closed.
3. **Direct trust by default.** The registry composes steward-anchored direct trust (persist's "Policy A"). Transitive referrer chains and score-weighted consensus (Policies B and C in `FSD/FSD-002_FEDERATION_SURFACE.md` §6.1) are consumer-side compositions over the same edges.

The registry slice composed into CIRISServer has no tables of its own: it reads the server's shared persist `Engine` directly (`FSD/FSD-004_POST_FOLD_SURFACE.md` §5). The cache described below is the standalone binary's.

---

## Where each piece of state lives

| Registry-local table | Role | Authoritative state in persist |
|---|---|---|
| `trusted_primitive_keys` | Cache of "registry trusts primitive X to sign builds" | `federation_attestations` rows where `attesting_key_id = registry-steward AND attestation_type = "vouches_for"`, joined to `federation_keys` for the pubkey bytes. |
| `partner_keys` | Cache of per-org keys | `federation_keys` rows where `identity_type = "partner"`, with `federation_attestations` recording which stewards/peers vouch. |
| `registry_signing_keys` | Cache of the registry's own identity keys | `federation_keys` row where `identity_type = "steward" AND identity_ref = "registry"`, self-signed (the bootstrap row per persist's doc §"Schema sketch — Bootstrap"). |

| RPC | Under federation |
|---|---|
| `RegisterTrustedPrimitiveKey` | An *attestation* call: `federation_attestations.put(attesting_key_id=registry-steward, attested_key_id=primitive-key, type="vouches_for")`. The pubkey row itself is published by the primitive's own CI (self-publish + post-hoc attestation). |
| `RevokeTrustedPrimitiveKey` | `federation_revocations.put(revoking_key_id=registry-steward, revoked_key_id=primitive-key, reason=…)`. Consumers compute "is revoked" from the federation table per their policy. |
| `ListTrustedPrimitiveKeys` | The join of `federation_keys` ⨝ `federation_attestations WHERE attesting_key_id = registry-steward AND type = "vouches_for"`. |
| `POST /v1/verify/build-manifest` | Looks up the trusted key via the cache (TTL); on miss, fetches from persist's federation tables; runs Policy A; returns the verdict plus cache-age telemetry. |

**What is unchanged from the consumer's perspective**: `GET /v1/steward-key`, `GET /v1/verify/function-manifest/{ver}/{target}?project=...`, `GET /v1/verify/build-manifest/{primitive}/{ver}/{target}`, and the auth model (`REGISTRY_ADMIN_TOKEN` HTTP, HS256 JWT gRPC) behave identically whether the federation client is the persist client or the no-op client. Federation is a backend, not a wire-format change.

---

## Cache shape

### Layout

The `trusted_primitive_keys` / `partner_keys` / `registry_signing_keys` tables stay in the registry's PostgreSQL DB as caches. Each row carries four columns (`migrations/024_federation_cache_columns.sql`):

```sql
cached_at            timestamptz NOT NULL DEFAULT NOW(),
cache_ttl_seconds    integer     NOT NULL DEFAULT 300,
persist_row_hash     bytea       NOT NULL,  -- sha256 of the federation_keys row we cached
persist_witnessed_at timestamptz             -- federation_keys.scrub_timestamp from persist
```

`cache_ttl_seconds` is the TTL beyond which the row is considered stale and must be re-fetched from persist before it can be served to a verify request. Default 300s (5 min); `FEDERATION_CACHE_TTL_SECONDS` tunes it per deployment.

`persist_row_hash` lets the registry detect divergence: on every read-through, the registry hashes the freshly-fetched `federation_keys` row and compares against `persist_row_hash`. Mismatch → cache invalidation + telemetry counter increment.

### Invalidation

Two triggers, in order of preference:

1. **Invalidate-on-attestation (registry's own writes)**: when registry calls `RegisterTrustedPrimitiveKey` or `RevokeTrustedPrimitiveKey`, it writes to persist first, then immediately re-fetches the affected `federation_keys` rows and updates the local cache with the new `cached_at`. Zero staleness for the common case (registry IS the writer).
2. **TTL-based re-fetch**: cache rows older than `cache_ttl_seconds` are re-fetched on next read. Bounded staleness for peer-published changes.

### Telemetry

Metrics under the `federation_` namespace:

- `federation_cache_hits_total{table}` — cached row served within TTL, no persist round-trip.
- `federation_cache_misses_total{table, reason="ttl_expired"|"invalidated"|"absent"}` — fell through to persist.
- `federation_dual_write_divergence_total{table}` — registry's local row differs from persist's authoritative row on read-through. Should be 0 in steady state; non-zero is a strong signal something is wrong.
- `federation_cache_age_seconds{table, quantile}` — distribution of cache-row ages at read time. Useful for tuning `cache_ttl_seconds`.
- `federation_persist_request_latency_seconds{operation, quantile}` — round-trip latency to persist for cache misses + writes. Informs whether to dial up `cache_ttl_seconds` if persist is consistently slow.

---

## Configuration

| Env var | Default | Purpose |
|---|---|---|
| `FEDERATION_DUAL_WRITE_ENABLED` | `false` | Use the persist client for federation reads and writes. When `false` the registry uses the no-op client and serves from its local tables. Also governs the `/v1/builds/{version}` provenance walk: with it off, the manifest dimension is always absent. |
| `FEDERATION_PERSIST_ENDPOINT` | (required when dual-write is on) | The persist endpoint the client targets. |
| `FEDERATION_PERSIST_DSN` | `sqlite::memory:` | The persist `Engine` the registry constructs at boot; production points at Postgres, typically the same database the registry uses for its own tables. |
| `FEDERATION_CACHE_TTL_SECONDS` | `300` | Cache row TTL. |
| `FEDERATION_MAX_STALE_CACHE_AGE_SECONDS` | `3600` | Hard ceiling on cache age served while persist is unreachable. |
| `FEDERATION_PERSIST_REQUIRED` | `false` | Strict mode: refuse to serve from cache when persist is unreachable. |

---

## Failure modes

### Decision matrix for verify requests when persist is unreachable

| Cache state | `FEDERATION_PERSIST_REQUIRED=false` (permissive) | `FEDERATION_PERSIST_REQUIRED=true` (strict) |
|---|---|---|
| Cache hit, age < `FEDERATION_MAX_STALE_CACHE_AGE_SECONDS` | Serve from cache; `cache_age_seconds` in response | Fail-closed: 503 with `Retry-After` |
| Cache hit, age ≥ `FEDERATION_MAX_STALE_CACHE_AGE_SECONDS` | Fail-closed: 503 with `Retry-After` | Fail-closed: 503 with `Retry-After` |
| Cache miss (key not previously cached) | Fail-closed: 404 with `cause: persist_unreachable` | Fail-closed: 503 with `Retry-After` |

The hard ceiling on cache age is the backstop: even in permissive mode, registry refuses to serve cache older than the ceiling. This bounds the attack window for "deliberate persist outage to extend the validity of a key that has been peer-revoked."

### The `federation_provenance` block

`GET /v1/builds/{version}` carries a `federation_provenance` block composed from the federation directory, not from the build row:

```json
{
  "federation_provenance": {
    "attestations_consumed": [
      {"dimension": "provenance:build_manifest:x86_64-unknown-linux-gnu:v1", "score": 1.0, "confidence": 1.0, "attester_key_id": "<pipeline key>", "evidence_summary": "..."}
    ],
    "slsa_level": 1,
    "note": null
  }
}
```

The manifest entry is present only when a pipeline-signed Contribution for the exact `(target, binary_version, manifest_hash)` exists in the directory, signed by a key carrying the accord-conferred `infra:attest` role, whose bound-hybrid signature re-verifies against the pipeline's directory-pinned pubkeys. Absent that, the dimension is absent and `note` says so; the registry never synthesizes a pass from the row. The SLSA level is derived from the facts the row carries (source repo and commit, build attestation).

### Telemetry alarms

Operators should set alarms on:

- `federation_persist_request_latency_seconds{quantile="0.99"} > 500ms` — persist is slow; cache hit ratio about to drop.
- `federation_cache_age_seconds{quantile="0.99"} > 0.5 * FEDERATION_MAX_STALE_CACHE_AGE_SECONDS` — half the cache is approaching the ceiling; persist may be unreachable.
- `federation_dual_write_divergence_total > 0` — divergence detected; investigate immediately.
- `verify_requests_failed_total{cause="persist_unreachable"} > 0` — fail-closed responses being emitted; persist outage in progress.

---

## Trust policy (Policy A — direct trust on steward attestation)

The registry's verify endpoint composes its verdict by the equivalent of persist's Policy A:

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

Identical semantics to a `trusted_primitive_keys` lookup; the only difference is *where the data comes from*. Reads page persist's bounded `AttestationQuery` (`federation::persist_client::collect_pages`), narrowed to the attestation family where one is known.

**Policies B and C** (transitive referrer chains; Coherence Stake score-weighted consensus) are consumer-side compositions over the same substrate edges. Policy B needs trusted roots other than the registry's own steward, which is a `TRUST_CONTRACT.md` change consumers absorb; Policy C needs `peer_weights` from CIRISLens's scoring layer.

---

## Write paths

### `RegisterTrustedPrimitiveKey`

1. Validate the request (admin JWT, project name slug, pubkey lengths).
2. **Do not write `federation_keys` itself.** That row is published by the primitive's own CI (self-publish). Registry assumes the primitive has already published the key to persist.
3. Write `federation_attestations.put({ attesting_key_id: registry-steward, attested_key_id: primitive-key, attestation_type: "vouches_for", weight: 1.0, asserted_at: now })` with the registry's scrub-signature.
4. Pre-warm the local cache by re-fetching the joined `federation_keys` ⨝ `federation_attestations` view.
5. Audit log entry with the `attestation_envelope` hash in its metadata.

If the primitive's `federation_keys` row doesn't exist when registry tries to attest, the call fails with `FailedPrecondition: "primitive has not published its federation_keys row yet"`. Operators (or CI) re-run after the primitive's self-publish.

### `RevokeTrustedPrimitiveKey`

1. Validate request.
2. Write `federation_revocations.put({ revoking_key_id: registry-steward, revoked_key_id: primitive-key, reason, revoked_at: now, effective_at: now })`.
3. Invalidate local cache for the affected key.
4. Audit log entry: `AUDIT_TRUSTED_KEY_REVOKED`.

### `ListTrustedPrimitiveKeys`

Returns the join of `federation_keys WHERE identity_type='primitive'` ⨝ `federation_attestations WHERE attesting_key_id=registry-steward AND attestation_type='vouches_for'`, with `federation_revocations` filtered out. The wire format (proto `TrustedPrimitiveKey` message) is unchanged — clients see no difference. Registry computes the join on read.

### Per-primitive write quota (operational rate-limit)

To bound storage growth from any single primitive's CI publishing keys, persist enforces a per-primitive quota:

```
PERSIST_FEDERATION_KEYS_PER_PRIMITIVE_PER_DAY  default 10
```

Most primitives publish 1-2 keys per quarter (rotation cadence). 10/day leaves headroom for rotation drills + emergency re-publish without enabling spam. Excess writes return 429 + `Retry-After: 86400`.

This sits alongside the per-source-IP rate limit (registry's `rate_limiter`; persist mirrors). Two-axis defense: rate-by-network and quota-by-identity.

---

## The registry's identity under federation

The registry's own signing keys (loaded from `ED25519_KEY_PATH` / `MLDSA_KEY_PATH`) are its federation identity: `HybridCrypto::build_persist_local_signer` derives the persist `LocalSigner` the `Engine` is constructed with at boot, and `HybridCrypto::build_edge_local_signer` gives the Edge transport the same identity for its announces. The identity is published as a `federation_keys` row with `identity_type='steward'`, `identity_ref='registry'`, self-signed (`scrub_key_id == key_id` — the bootstrap case from persist's doc).

This identity is **not** the trust root consumers pin. The trust root is the genesis bundle served at `GET /v1/steward-key` (`TRUST_CONTRACT.md` §3); the registry's identity keys are served at `GET /v1/identity` and identify the relay. Other federation peers (lens, agent, persist) hold their own identities in the same directory.

### Rotation

Rotation of the registry's identity under federation:
1. Operator generates new keypair, mounts new seeds on both regions.
2. Registry boots with new keypair.
3. Registry writes a new `federation_keys` row for the new identity, self-signed.
4. Registry writes `federation_attestations.put({ attesting_key_id: OLD, attested_key_id: NEW, attestation_type: "delegates_to" })` to chain the new identity to the old (federation peers can verify the rotation didn't drop trust on the floor).
5. Optionally writes `federation_revocations.put({ revoking_key_id: OLD, revoked_key_id: OLD })` to retire the old key.

Consumers that pinned the old fingerprint: their next `/v1/identity` re-fetch surfaces the new pubkey; their policy code can verify the chained attestation from old → new before re-pinning.

---

## Positions

Persist owns substrate; registry owns policy + cache + write-mapping; CIRISVerify owns consumer trust.

### 1. Bootstrap order

Self-signed roots, mutual attestation, out-of-band anchored:

- Persist's migration creates a self-signed `federation_keys` row for `persist-steward` (`scrub_key_id == key_id`). Constant in the migration; persist's CI doesn't write it.
- The registry writes its own self-signed row to persist via `FederationDirectory::put_public_key`.
- **Mutual attestation**: persist-steward writes `vouches_for` against registry-steward, and vice versa.
- **Out-of-band anchoring**: the genesis bundle is the trust root consumers pin (`TRUST_CONTRACT.md` §3).

### 2. Backfill of `trusted_primitive_keys`

Owned by the registry's deployment script. The script iterates `trusted_primitive_keys`, calls `FederationDirectory::put_public_key` for each row signed by registry-steward, then `FederationDirectory::put_attestation` with `attestation_type="vouches_for"`. Persist's `put_public_key` is idempotent on duplicate `key_id` with matching content (no-op). One-shot; afterwards new writes flow through `RegisterTrustedPrimitiveKey`.

At the registry's scale (a handful of primitive and partner keys), per-row writes within persist's rate limits are sufficient.

### 3. Policy versioning

The registry's verify response names the policy it composed (`federation_provenance`). CIRISVerify consumer code can pin a specific version, accept-any-from-allowlist, or reject-unknown per its own config. Persist serves the same rows for every policy version — no policy knob on persist's surface. Chain traversal for Policy B uses persist's `as_of` parameter on `list_attestations_*` and `revocations_for` so the walk sees a consistent snapshot when it crosses a peer-revocation boundary.

### 4. Audit-log shape

The federation keeps a clean separation:

- **Persist's journal**: "who wrote what row when, with what key". Cryptographic provenance, append-only, every row carries `(timestamp, scrub_key_id, scrub_signature, original_content_hash)`.
- **Registry's audit** (`audit_log` table): "why the registry-steward decided to attest this key" — admin RPC source, operator user_id (`claims.sub`), ticket reference / business reason.
- **Join key**: registry's `audit_log` entry for `RegisterTrustedPrimitiveKey` records the `attestation_envelope` hash in its `metadata` JSONB. Persist's journal records the row carrying that envelope. Forensic query "who attested key K" walks `audit_log WHERE metadata->>envelope_hash = ...` joined to `journal WHERE original_content_hash = ...`.

UX queries like "show me everything related to key K" become `list_attestations_for(K) ∪ revocations_for(K) ∪ journal_entries_referencing(K)` — composable from persist's existing surface.

### 5. CIRISPortal

Persist exposes the data; Portal reads it like any other consumer. Whatever auth model the Portal uses to read persist is operational, not architectural. `FSD/FSD-004_POST_FOLD_SURFACE.md` §4.5 maps each Portal surface to the CIRISClient screen that carries it in the registry slice.

---

## What's not in this doc

- **CIRISLens or CIRISAgent integration.** This doc covers registry-as-consumer of persist. Lens and agent are peers in the federation directory, but their consumer code lives in their own repos.
- **Coherence Stake math.** Policy C is a sketch; the actual scoring math lives in CIRISLens's scoring API.

This doc is a **client-side surface alignment artifact** — paired with persist's `FEDERATION_DIRECTORY.md`, the two together define the registry ↔ persist contract.

---

## Update cadence

Updated:
- On every persist contract change the registry depends on.
- On every registry policy addition (Policy B, C, future).
- On every TRUST_CONTRACT.md revision (kept consistent with consumer-facing trust shape).
