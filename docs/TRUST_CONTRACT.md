# CIRISRegistry Trust Contract

**Status:** Stable. Changes require coordination with consumer teams (CIRISVerify, CIRISAgent, CIRISLens, CIRISPersist).
**Audience:** Consumer code in CIRIS federation peers that fetches the trust root and verifies build manifests from `api.registry.ciris-services-1.ai`.

This document is the canonical reference for the trust anchors, signature math, rotation policy, and cross-region semantics that consumers can rely on. It complements [`MANIFEST_VALIDATION_API.md`](MANIFEST_VALIDATION_API.md) (which is the wire-format / endpoint reference) by specifying *what trust statements you can extract from each response*.

---

## 1. Three consumption paths

Three valid ways a consumer can obtain a verified build manifest, in increasing order of federation-correctness:

| Path | Source | Wire format | Signature on response | Trust anchor consumer pins |
|---|---|---|---|---|
| **A** | `GET /v1/verify/function-manifest/{ver}/{target}?project=ciris-<name>` | `FunctionManifestResponse` (JSON) | Original CI signature, stored verbatim by the build-manifest POST handler | Per-primitive pipeline pubkey |
| **B** | `GET /v1/verify/build-manifest/{project}/{version}/{target}` | `BuildManifest` (JSON, byte-identical to POSTed) | Original CI signature over canonical bytes | Per-primitive pipeline pubkey |
| **C** | GitHub release tarball (e.g. `ciris-verify-vX.Y.Z-signed-build-manifests.tar.gz`) | `BuildManifest` (JSON, as signed) | Original CI signature | Per-primitive pipeline pubkey |

Path A is the convenience path for runtime self-check. Path B is the federation-correct path when byte-identical canonical-byte verification against the publishing primitive's CI signature matters; only rows POSTed via `/v1/verify/build-manifest` (Case (i)) are served. Path C is the federation fallback when the registry is unreachable; CIRISVerify ships it for every release.

The CEG-native surface, `/v1/builds`, serves the signed build-manifest claim and the manifest bytes directly from the federation directory; see [`MANIFEST_VALIDATION_API.md`](MANIFEST_VALIDATION_API.md) and `FSD/FSD-004_POST_FOLD_SURFACE.md` §4.1.

---

## 2. Path A — production trust contract

### 2.1 Endpoint

```
GET https://api.registry.ciris-services-1.ai/v1/verify/function-manifest/{binary_version}/{target}?project=ciris-<name>
```

- **Authentication**: none (public read).
- **Rate limit**: per-IP `Tier::Public` (60/min, 600/hr) — see [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.2 AV-9.
- **Cache TTL guidance**: response is immutable per `(project, binary_version, target)` once written; consumers may cache indefinitely with a pubkey-fingerprint check on staleness.

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

The `signature.key_id` field identifies which signing key produced the signatures. For manifests POSTed via `/v1/verify/build-manifest` (the recommended path), this is the **publishing primitive's CI key_id** (e.g., `verify-steward-2026`), NOT the registry's own key_id.

### 2.3 Signature math (which case applies)

There are **two POST paths** with different signature provenance. The GET response carries whichever signature the row was stored with:

#### Case (i) — Manifest POSTed via `/v1/verify/build-manifest` (recommended)

- The publishing primitive's CI signed `BuildManifest::canonical_bytes()` with the per-primitive pipeline keypair.
- The registry verifies that signature against the trusted pubkey registered in `trusted_primitive_keys` for that primitive (admin-managed via `RegisterTrustedPrimitiveKey`).
- The registry stores the manifest with the **original CI signature preserved**, and serves it back unchanged via the GET.
- **Consumer trust anchor**: the per-primitive pubkey (§6).
- **Verify by**: reconstructing canonical bytes (see §2.5) and checking the hybrid Ed25519 + ML-DSA-65 signatures against the per-primitive pubkey.

#### Case (ii) — Manifest POSTed via the legacy `/v1/verify/function-manifest` endpoint

- The legacy POST handler (kept for backwards compatibility) **re-signs server-side** with the registry's own signing key over `manifest_hash` (not over canonical bytes).
- The signature stored is the **registry's**, not the CI's.
- **Consumer trust anchor**: the registry's own identity keys (`GET /v1/identity`, §3.3).
- **Verify by**: hybrid signature check against the registry's pubkey over `sha256(manifest_hash_string_bytes)`.

**How to tell which case applies**: inspect `signature.key_id`.
- If it matches the registry's own `key_id` (served at `GET /v1/identity`), it's Case (ii).
- If it matches a per-primitive key_id (e.g., `verify-steward-2026`), it's Case (i).

### 2.4 Format-translation caveat (Path A limitation, Path B fix)

Even in Case (i), the response is in `FunctionManifestResponse` shape, not the original `BuildManifest` shape. The fields are translated; not all `BuildManifest` fields (`primitive`, `build_id`, `extras`) are preserved in the response envelope.

**Practical impact**: a consumer cannot reconstruct the exact canonical bytes the CI signature was over from the Path A GET response alone. Path B serves the `BuildManifest` verbatim; consumers needing canonical-byte fidelity use Path B (or Path C) and Path A only for liveness and availability checks.

### 2.5 Canonical bytes for verification

For BuildManifest signatures (Case i):
- Canonical form: `serde_json::to_vec(&CanonicalBuildManifest)` where `CanonicalBuildManifest` excludes the `signature` field and field order matches the upstream `ciris-verify-core` BuildManifest definition.
- See `rust-registry/ciris-registry-core/src/build_manifest.rs::BuildManifest::canonical_bytes` for the byte-identical implementation.
- The PQC signature covers `canonical_bytes || classical_signature_bytes` (bound signature) — the registry verifies via this same scheme.

For FunctionManifest signatures (Case ii):
- The registry signs `req.manifest_hash.as_bytes()` (the manifest_hash STRING, not its decoded bytes).
- See `rust-registry/ciris-registry-core/src/api/http.rs::register_function_manifest` for the signing call.

---

## 3. Trust anchor: the genesis trust-root bundle

### 3.1 Source of truth

```
GET https://api.registry.ciris-services-1.ai/v1/steward-key
GET https://api.registry.ciris-services-1.ai/v1/trust-root/bundle
```

Both routes return the same body: the genesis trust-root bundle persist bakes. It carries the `humanity-accord` charter, the accord-holder roster with custody evidence, the `infra:*` scopes the charter confers, the serve-node grants, and the `ciris-canonical` community birth record in the `community` member (`null` when the bundle carries none). No auth.

The schema is field for field the one CIRISServer serves at its `GET /v1/trust-root/bundle`, so a consumer pinned to the registry sees the same trust root as a consumer pinned to a node.

**Pin the bundle, not a per-steward fingerprint.** The bundle is self-authenticating: the hybrid accord-holder signatures over the charter are re-derived by the consumer from its own records. The outer envelope carries **no `response_signature`** by design — signing the wrapper would prove only that the relay said so. The one anchor a consumer pins is the `ciris-canonical` community (`MISSION.md` §2.1.1); the member set is resolved through `resolve_community`.

### 3.2 Cross-region invariant

The bundle is identical on US (`registry-us`) and EU (`registry-eu`). A consumer that fetches it from more than one region, or from a region and a CIRISServer node, compares equal bytes.

### 3.3 The registry's own identity

The registry has signing keys of its own, used for the legacy Case (ii) re-signing, for the federation signer on its Edge announces and for its persist `LocalSigner`. They are served at `GET /v1/identity` and are loaded from disk at boot via `ED25519_KEY_PATH` and `MLDSA_KEY_PATH`, mirrored across regions so both regions sign with the same key bytes. The registry logs a warning when these are unset and falls back to ephemeral keys ([`THREAT_MODEL.md`](THREAT_MODEL.md) §3.6 AV-28); production deployments have a stable identity from boot to the next intentional rotation.

These keys are **not** a trust root. Nothing a consumer verifies against them confers federation authority; they identify the relay.

### 3.4 Rotation policy

Rotation of the registry's own identity keys is a coordinated operator action:

1. Operator generates new 32-byte seeds on a secure host.
2. Mirrors the seeds to US and EU host filesystems (mode 0440, `root:999`).
3. Rolls each region's container — containers boot with new keys.
4. `GET /v1/identity` returns the new pubkeys from both regions.
5. Operator calls `RegisterTrustedPrimitiveKey` with `project='ciris-registry'` to update the registry's self-trusted entry.

**Consumer impact during rotation**: signatures previously emitted with the old key remain valid (verifiable against the cached old pubkey). New signatures emitted post-rotation require the consumer to refresh `/v1/identity` and re-pin. Manifests stored with the old key remain in the database with their original signatures.

**Recommended consumer policy**:
- Pin the pubkey bytes from `/v1/identity` at startup.
- Re-fetch on cache miss / verification failure → if pubkey changed, log a `registry-identity-rotated` warning and update the cache.
- DO NOT silently re-trust on pubkey change. A surprise rotation should be operator-confirmed before production code accepts the new key. See [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.6 AV-25 for the threat model behind this.

Rotation of the trust root itself (steward conferral, install admission, accord-holder replacement) is the human-signed rotation arc in `MISSION.md` §2.1; a consumer observes it as a new bundle.

---

## 4. Path C — GitHub release fallback

CIRISVerify releases include a `signed-build-manifests.tar.gz` artifact alongside the binaries. Each contained file is a `BuildManifest` JSON signed by the publishing primitive's CI key, with sigstore signatures around the tarball itself.

**Use case**: federation peers that cannot reach `api.registry.ciris-services-1.ai` (network-isolated deployments, offline verification, registry outage).

**Trust anchor**: the per-primitive pipeline pubkey (same as Path B). The tarball's sigstore signature also chains to GitHub's OIDC infrastructure for a second integrity claim.

**Limitation**: not real-time. A revoked build is still in the tarball.

---

## 5. Path B — byte-identical build manifest

Symmetric GET endpoint that returns the original `BuildManifest` JSON byte-identical to what was POSTed:

```
GET /v1/verify/build-manifest/{project}/{version}/{target}
```

- **No auth** (public read).
- **Response**: raw `BuildManifest` JSON, byte-identical to the POST body. Content-Type: `application/json`. Includes the original `signature.classical` + `signature.pqc` + `signature.key_id` fields under the publishing primitive's identity.
- **Routing**: `{project}` is the primitive's `project_name()` (e.g., `ciris-verify`, `ciris-agent`); `{version}` is the BuildManifest's `binary_version`; `{target}` is the build target (e.g., `aarch64-apple-ios`, `python-source-tree`).
- **Storage**: backed by the `function_manifests` table (`raw_manifest_body BYTEA`), replicated between regions.
- **404 conditions**:
  1. No row exists for `(project, version, target)`.
  2. Row exists but was POSTed via the legacy `/v1/verify/function-manifest` endpoint (Case (ii) — server re-signed; no original CI body captured). For those rows, use Path A.

**Verification by consumer**:

1. GET the response bytes.
2. Parse to extract `signature.classical` + `signature.pqc` + `signature.key_id`.
3. Compute canonical bytes by serializing the BuildManifest with the `signature` field removed, using the byte-identical scheme upstream `ciris-verify-core::BuildManifest::canonical_bytes()` defines.
4. Verify Ed25519 sig over canonical_bytes; verify ML-DSA-65 sig over `canonical_bytes || classical_sig_bytes` (bound payload).
5. Match `signature.key_id` against the expected per-primitive `key_id`.

Path A remains the convenience path for runtime self-check (returns a derived `FunctionManifestResponse` shape). Path B is the federation-correct path when canonical-byte signature fidelity matters.

---

## 6. Per-primitive pipeline keys

Two mechanisms bless a pipeline key:

- **Federation directory (the CEG-native path).** `POST /v1/builds` admits a manifest whose signing key carries a `delegates_to(root → pipeline, infra:attest)` grant or the accord role on its own key record, and verifies the bound hybrid signature against the pubkeys pinned in the directory. A consumer resolves the pipeline key the same way, from the directory; there is nothing to obtain out of band.
- **`trusted_primitive_keys` (the `/v1/verify/build-manifest` path).** The registry verifies inbound manifests against the pubkey registered for the primitive via the `RegisterTrustedPrimitiveKey` admin RPC. Consumers of Paths A, B and C pin the same per-primitive pubkey; CIRISVerify embeds its own in its source for self-check.

**Admin RPC auth shape** (distinct from the `REGISTRY_ADMIN_TOKEN` HTTP bearer used by manifest POST endpoints): the gRPC `RegistryAdminService` requires an HS256 JWT minted from `JWT_SECRET` with `role: 1` (ROLE_SYSTEM_ADMIN per `middleware/auth.rs`) and `iss: "ciris-registry"`. Quick mint command for the operator runbook:

```python
import jwt, time
secret = open('/path/to/JWT_SECRET').read().strip()
now = int(time.time())
print(jwt.encode(
    {"sub": "bridge-ops", "role": 1, "iat": now, "exp": now + 300, "iss": "ciris-registry"},
    secret,
    algorithm="HS256",
))
```

`REGISTRY_ADMIN_TOKEN` (used for HTTP `POST /v1/builds` and the `/v1/verify/*-manifest` POSTs) and `JWT_SECRET` (used to mint admin JWTs for gRPC) are two distinct secrets serving two distinct auth surfaces — both must be provisioned in the deployment.

---

## 7. Threat model references

For the threat-model reasoning behind this contract:

- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.6 AV-25 — signing-key compromise.
- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.6 AV-28 — ephemeral signing keys in production.
- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.6 AV-26 — uploaded BuildManifest hybrid-sig verification.
- [`THREAT_MODEL.md`](THREAT_MODEL.md) §3.7 AV-33 — multi-master migration desync.

---

## 8. Update cadence

This document is updated:
- On every rotation of the registry's identity keys.
- On every change to the trust-root bundle's schema.
- On every Path A/B/C endpoint change.
- On every BuildManifest wire-format bump (coordinate with `ciris-verify-core` upstream).
