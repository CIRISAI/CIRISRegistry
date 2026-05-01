# CIRISRegistry Trust Contract

**Version:** 1.0 (initial — covers v1.3 Phase A registry)
**Status:** Stable. Changes require coordination with consumer teams (CIRISVerify, CIRISAgent, CIRISLens, CIRISPersist).
**Audience:** Consumer code in CIRIS federation peers that fetches and verifies build manifests from `api.registry.ciris-services-1.ai`.

This document is the canonical reference for the trust anchors, signature math, rotation policy, and cross-region replication semantics that consumers can rely on. It complements [`MANIFEST_VALIDATION_API.md`](MANIFEST_VALIDATION_API.md) (which is the wire-format / endpoint reference) by specifying *what trust statements you can extract from each response*.

Filed in response to [CIRISRegistry#5 §1](https://github.com/CIRISAI/CIRISRegistry/issues/5) (verify-side consumption requirements).

---

## 1. Three consumption paths

Three valid ways a consumer can obtain a verified build manifest, in increasing order of federation-correctness:

| Path | Source | Wire format | Signature on response | Trust anchor consumer pins |
|---|---|---|---|---|
| **A** | `GET /v1/verify/function-manifest/{ver}/{target}?project=ciris-<name>` | `FunctionManifestResponse` (JSON) | Original CI signature, stored verbatim by Phase A POST handler | Per-primitive steward pubkey (registered in `trusted_primitive_keys`) |
| **B** | `GET /v1/verify/build-manifest/{primitive}/{build_id}/{target}` (planned, Phase B — not yet shipped) | `BuildManifest` (JSON, byte-identical to POSTed) | Original CI signature over canonical bytes | Per-primitive steward pubkey |
| **C** | GitHub release tarball (e.g. `ciris-verify-v1.8.3-signed-build-manifests.tar.gz`) | `BuildManifest` (JSON, as signed) | Original CI signature | Per-primitive steward pubkey |

Path A is **available today** and is the recommended path for production federation peers. Path B is planned but not shipped; consumers can use Path A for v1.3-era registries. Path C is the federation-fallback when registry is unreachable; CIRISVerify v1.8.3 ships this for every release.

---

## 2. Path A — production trust contract

### 2.1 Endpoint

```
GET https://api.registry.ciris-services-1.ai/v1/verify/function-manifest/{binary_version}/{target}?project=ciris-<name>
```

- **Authentication**: none (public read).
- **Rate limit**: per-IP `Tier::Public` (60/min, 600/hr) — see [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.2 AV-9.
- **Cache TTL guidance**: response is immutable per `(project, binary_version, target)` once written; consumers may cache indefinitely with a registry-pubkey-fingerprint check on staleness.

### 2.2 Response shape

```json
{
  "version": "1.0",
  "target": "x86_64-unknown-linux-gnu",
  "binary_hash": "sha256:abc123...",
  "binary_version": "1.8.3",
  "generated_at": "2026-05-01T20:00:00Z",
  "functions": { ... },
  "manifest_hash": "sha256:def456...",
  "signature": {
    "classical": "<base64 Ed25519 signature>",
    "classical_algorithm": "Ed25519",
    "pqc": "<base64 ML-DSA-65 signature>",
    "pqc_algorithm": "ML-DSA-65",
    "key_id": "<signing key identifier, e.g. verify-steward-2026>"
  },
  "metadata": { ... }
}
```

The `signature.key_id` field identifies which signing key produced the signatures. For manifests POSTed via the new `/v1/verify/build-manifest` endpoint (the recommended path for v1.3+), this is the **publishing primitive's CI key_id** (e.g., `verify-steward-2026`), NOT the registry's steward key_id.

### 2.3 Signature math (which case applies)

Phase A introduces **two distinct POST paths** with different signature provenance. The GET response carries whichever signature the row was stored with:

#### Case (i) — Manifest POSTed via NEW `/v1/verify/build-manifest` endpoint (recommended)

- The publishing primitive's CI signed `BuildManifest::canonical_bytes()` with the per-primitive steward keypair.
- The registry verifies that signature against the trusted pubkey registered in `trusted_primitive_keys` for that primitive (admin-managed via `RegisterTrustedPrimitiveKey`).
- The registry stores the manifest with the **original CI signature preserved**, and serves it back unchanged via the GET.
- **Consumer trust anchor**: the per-primitive pubkey (e.g., for `ciris-verify`, the bytes at `verify-steward-2026`'s public key, which can be obtained via [Path B's `/v1/verify/trusted-primitive-keys/{primitive}` endpoint when shipped, or out-of-band today]).
- **Verify by**: reconstructing canonical bytes (challenge — see §2.5) and checking the hybrid Ed25519 + ML-DSA-65 signatures against the per-primitive pubkey.

#### Case (ii) — Manifest POSTed via LEGACY `/v1/verify/function-manifest` endpoint

- The legacy POST handler (kept for backwards compat) **re-signs server-side** with the registry's steward key over `manifest_hash` (not over canonical bytes).
- The signature stored is the **registry's**, not the CI's.
- **Consumer trust anchor**: the registry's steward pubkey (`GET /v1/steward-key`).
- **Verify by**: hybrid signature check against the registry steward pubkey over `sha256(manifest_hash_string_bytes)`.

**How to tell which case applies**: inspect `signature.key_id`.
- If it matches the registry's steward `key_id` (currently `75c29fccd21f80e4...`), it's Case (ii).
- If it matches a per-primitive key_id (e.g., `verify-steward-2026`), it's Case (i).

### 2.4 Format-translation caveat (Path A limitation, Path B fix)

Even in Case (i), the response is in `FunctionManifestResponse` shape, not the original `BuildManifest` shape. The fields are translated; not all `BuildManifest` fields (`primitive`, `build_id`, `extras`) are preserved in the response envelope.

**Practical impact**: a consumer cannot reconstruct the exact canonical bytes the CI sig was over from the Path A GET response alone. They CAN check the signature is over those bytes if they obtain the original BuildManifest by another means (e.g., Path C tarball), but Path A is not a self-contained verification path for Case (i) signatures.

**Path B (planned)** addresses this by serving the BuildManifest verbatim. Until Path B ships, consumers needing canonical-byte fidelity should fetch from Path C (GitHub release) and use Path A only for liveness/availability checks.

### 2.5 Canonical bytes for verification

For BuildManifest signatures (Case i):
- Canonical form: `serde_json::to_vec(&CanonicalBuildManifest)` where `CanonicalBuildManifest` excludes the `signature` field and field order matches the upstream `ciris-verify-core` BuildManifest definition.
- See `rust-registry/src/build_manifest.rs::BuildManifest::canonical_bytes` for the byte-identical implementation.
- The PQC signature covers `canonical_bytes || classical_signature_bytes` (bound signature) — the registry verifies via this same scheme.

For FunctionManifest signatures (Case ii):
- The registry signs `req.manifest_hash.as_bytes()` (the manifest_hash STRING, not its decoded bytes).
- See `rust-registry/src/api/http.rs::register_function_manifest` for the signing call.

---

## 3. Trust anchor: registry's steward pubkey

### 3.1 Source of truth

```
GET https://api.registry.ciris-services-1.ai/v1/steward-key
```

Returns the registry's current Ed25519 + ML-DSA-65 public keys with their `key_id` and pubkey fingerprints. No auth.

### 3.2 Cross-region invariant

**The registry steward pubkey is identical on US (`registry-us`) and EU (`registry-eu`).** Both regions serve the same `/v1/steward-key` payload:

```json
{
  "classical": {
    "algorithm": "Ed25519",
    "key": "+noh8tqUnMsOIceG5ebQ4gUePPi+y+4wi1nMy2LHit4=",
    "key_id": "75c29fccd21f80e4ddf03f6dfdfd9d4d2ba40db5720717079c1297dea18d265a"
  },
  "pqc": {
    "algorithm": "ML-DSA-65",
    "key_id": "75c29fccd21f80e4ddf03f6dfdfd9d4d2ba40db5720717079c1297dea18d265a",
    "fingerprint": "sha256:1c97265c775ab1ba186e50558cb25cb19949e2908ae66137a176868a64def4cc"
  },
  "signature_mode": "HYBRID_REQUIRED"
}
```

Verified persistent across container restarts and identical across regions as of 2026-05-01.

The keys are mounted on each region's host filesystem via the bridge ansible role (`runbooks/registry-keys-init.yml`), with seeds mirrored US↔EU at deploy time so both regions sign with the same key bytes (closes [`THREAT_MODEL.md`](THREAT_MODEL.md) AV-28).

### 3.3 Persistence guarantee

Steward keys are loaded from disk via `ED25519_KEY_PATH` and `MLDSA_KEY_PATH` env vars at boot. The registry's in-app boot code logs a warning when these are unset and falls back to ephemeral keys — production deployments will have a stable key from boot to next intentional rotation. Consumers can safely pin the pubkey bytes for the lifetime of a release.

### 3.4 Rotation policy

**Current state**: no automated rotation cadence. Rotation is a coordinated operator action with these steps:

1. Operator generates new 32-byte seeds on a secure host.
2. Mirrors the seeds to US and EU host filesystems (mode 0440, `root:999`).
3. Rolls each region's container — containers boot with new keys.
4. `/v1/steward-key` returns the new pubkey from both regions.
5. Operator calls `RegisterTrustedPrimitiveKey` admin RPC with `project='ciris-registry'` to update the registry's self-trusted entry (which Phase A's boot-seed leaves untouched on existing rows post-`f848fe8`).

**Consumer impact during rotation**: signatures previously emitted with the old key remain valid (verifiable against the cached old pubkey). New signatures emitted post-rotation require the consumer to refresh `/v1/steward-key` and re-pin. Manifests stored with the old key remain in the database with their original signatures.

**Recommended consumer policy**:
- Pin the pubkey bytes from `/v1/steward-key` at startup.
- Re-fetch on cache miss / verification failure → if pubkey changed, log a `registry-steward-rotated` warning and update the cache.
- DO NOT silently re-trust on pubkey change. A surprise rotation should be operator-confirmed before production code accepts the new key. See [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.6 AV-25 for the threat model behind this.

A rotation cadence will be defined in v1.4. Until then, treat rotations as exceptional events.

---

## 4. Path C — GitHub release fallback

CIRISVerify v1.8.3+ releases include a `signed-build-manifests.tar.gz` artifact alongside the binaries. Each contained file is a `BuildManifest` JSON signed by the publishing primitive's CI key, with sigstore signatures around the tarball itself.

**Use case**: federation peers that cannot reach `api.registry.ciris-services-1.ai` (network-isolated deployments, offline verification, registry outage).

**Trust anchor**: the per-primitive steward pubkey (same as Path B). The tarball's sigstore signature also chains to GitHub's OIDC infrastructure for a second integrity claim.

**Limitation**: not real-time. A revoked build is still in the tarball.

---

## 5. Path B — planned (post-Phase 4 of v1.3 hardening waterfall)

Will provide a symmetric GET endpoint that returns the original `BuildManifest` JSON byte-identical to what was POSTed:

```
GET /v1/verify/build-manifest/{primitive}/{build_id}/{target}
```

- No auth (public read).
- Response: raw `BuildManifest` JSON, suitable for direct verification against the per-primitive steward pubkey.
- Backed by a new `verified_build_manifests` table cross-region replicated (per [`THREAT_MODEL.md`](THREAT_MODEL.md) AV-33 / Registry#4 conventions).

When Path B ships, Path A becomes the legacy / convenience read; Path B becomes the federation-correct read.

---

## 6. Per-primitive steward pubkey distribution (current state)

Until the federation-trusted-key endpoint (CIRISRegistry#5 §4) ships, consumers obtain per-primitive steward pubkeys out-of-band:

- **CIRISVerify** (`verify-steward-2026`): pubkey embedded in CIRISVerify's own source (consumed by self-check). Available at `~/.ciris-build-sign-verify/{ed25519,mldsa65}.pub` on a CI runner that has signed at least once.
- **CIRISAgent** (`agent-steward-2026`): TBD.
- **CIRISLens** (`lens-steward-2026`): TBD.
- **CIRISPersist** (`persist-steward-2026`): TBD.
- **CIRISRegistry** (`ciris-registry-steward`): not yet pinned to a per-primitive identity; uses the registry's own steward key for self-attestation.

Bridge-ops will register each primitive's pubkey via `RegisterTrustedPrimitiveKey` admin RPC as the team supplies bytes. This unblocks inbound POSTs for that primitive.

A coordinated trusted-keys discovery endpoint with registry-signed envelopes is tracked as Phase C in the v1.3 hardening waterfall and will replace the out-of-band distribution.

---

## 7. Threat model references

For the threat-model reasoning behind this contract:

- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.6 AV-25 — steward signing-key compromise.
- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.7 AV-28 — ephemeral signing keys in production (operationally mitigated 2026-05-01).
- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.4 AV-26 — uploaded BuildManifest hybrid-sig verification (mitigated v1.3 Phase A).
- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.4 AV-33 — Spock multi-master migration desync (mitigated v1.3 Phase 1 + #4 closure).

---

## 8. Update cadence

This document is updated:
- On every signing-key rotation (full pubkey + fingerprint refresh).
- On every Path A/B/C endpoint change.
- On every BuildManifest wire-format bump (coordinate with `ciris-verify-core` upstream).

**Last updated**: 2026-05-01 (initial draft, post-v1.3 Phase A — closes CIRISRegistry#5 §1).
