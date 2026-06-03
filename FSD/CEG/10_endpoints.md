[← §9 HUMANITY_ACCORD](09_humanity_accord.md) | **§10 Endpoints** | [Next: §11 Governance →](11_governance.md)

---

# §10 Endpoint shapes

CEG specifies five public + one admin HTTP endpoint shape for the discovery + cosigning surfaces. Wire format consumers (CIRISVerify v3.1.0+, CIRISAgent KMP UI, iOS/Android FFI) read these.

## §10.0 Common response shape

All CEG endpoints return:

- **Content-Type**: `application/json` (`Accept: application/json` honored; other types respond `406 Not Acceptable`)
- **CEG-API-Version header**: `CEG-Version: <current spec major.minor>` on every response (track the [README](README.md) `Version:` field; currently `0.9`); clients SHOULD echo `CEG-Accept-Version: <pinned-version>` on request, naming the version they were built against. Per [§0.3](00_conformance.md) SemVer policy, MAJOR mismatch is a wire-incompat reject; MINOR mismatch is compatible (clients MAY warn).
- **Time-Source header**: `X-CEG-Server-Time: <rfc3339_canonical>` per [§0.5](00_conformance.md) for client clock-skew bounds
- **Pagination** (where applicable): `?cursor=` + `?limit=` query params; response includes `next_cursor` (null if exhausted) and `total_estimate` (server's best estimate, may be approximate)

### §10.0.1 Error envelope

All error responses MUST conform to:

```json
{
  "error": {
    "code": "<ENUM_VALUE>",
    "http_status": <int>,
    "message": "<human-readable>",
    "request_id": "<server-assigned>",
    "details": {<error-specific fields>}
  }
}
```

| HTTP status | Error code | Meaning |
|---|---|---|
| 400 | `MALFORMED_REQUEST` | Invalid JSON, missing required field, bad field type |
| 400 | `CANONICAL_BYTES_VIOLATION` | Date-time / hex / encoding doesn't match [§0.5 / §0.6](00_conformance.md) |
| 401 | `UNAUTHENTICATED` | Bearer token missing or invalid (admin endpoints) |
| 403 | `RESERVED_PREFIX_VIOLATION` | Producer attempted to emit under a reserved prefix without authority per [§7](07_reserved.md) |
| 404 | `UNKNOWN_WITNESS` | Witness key_id not registered in directory ([§10.3](#103-sth-cosigning--witness-directory)) |
| 404 | `NOT_FOUND` | Generic resource not found (build, partner, key) |
| 409 | `IDEMPOTENT_CONFLICT` | Replay detected (e.g., duplicate `(tree_size, witness_key_id)` cosignature with different signatures) |
| 422 | `SIGNATURE_VERIFICATION_FAILED` | Ed25519 or ML-DSA-65 failed to verify; `details.algorithm` names which |
| 422 | `CLOCK_SKEW_VIOLATION` | `signed_at` exceeds [§0.7](00_conformance.md) ±5 minute tolerance |
| 422 | `WITNESS_QUORUM_NOT_MET` | Insufficient cosignatures to validate |
| 429 | `RATE_LIMITED` | `X-RateLimit-*` headers set; `Retry-After` honored |
| 500 | `INTERNAL_ERROR` | Server-side fault; request_id usable for support |
| 503 | `WITNESS_DIRECTORY_UNAVAILABLE` | Substrate replication lag exceeds liveness bound |

Rate-limit headers on every response: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` (seconds-until-reset epoch).

## §10.1 Transport substrate for byte-level content

Wire-format Attestations carry claims; they don't carry bytes. When a claim's `evidence_refs[]` cites a SHA-256-addressed blob (e.g., an installer binary, a config file, an adapter package per `agent_files:*` per [§5.6.7 / §5.9](05_namespace.md)), the bytes travel via Edge transport substrate: `MessageType::ContentFetch` + `ContentBody` + `ContentMiss` (per CIRISEdge#21). Holder-discovery via Persist's `holds_bytes:sha256:*` directory (CIRISPersist#103); peer-resolution via Edge's `PeerResolver::resolve_holders`. NodeCore node-mode peers serve the bytes per their MISSION §3.4 cohabitation contract (CIRISNodeCore#11). Attestation envelope shape unchanged; SHA-256 in `evidence_refs[]` becomes universally resolvable to bytes through the substrate.

### §10.1.1 Full-SHA verification before consumption (normative)

A CEG-Conforming Consumer (CCC) MUST verify the full SHA-256 of received bytes against the value in `evidence_refs[]` BEFORE handing the bytes to any consumer (Agent loader, Portal renderer, etc.). The `holds_bytes:sha256:{prefix}` directory ([§5.6.7](05_namespace.md)) carries only a short prefix for index efficiency; the consumer MUST NOT short-circuit verification to the prefix. Bytes that fail the full-SHA check MUST be discarded and the holder MUST be reported via the `holds_bytes:sha256:{prefix}` chain (emit a `withdraws` or negative score per consumer policy).

### §10.1.2 Holder directory TTL + ContentMiss feedback

A `holds_bytes:sha256:{prefix}` attestation has a default validity of **24 hours** from `signed_at`. After that the holder is considered stale; consumer policy MUST attempt at most 2 holders in parallel and accept the first successful full-SHA verification. On `ContentMiss` (holder no longer has the blob), the consumer MUST emit a `withdraws` against the `holds_bytes:sha256:{prefix}` attestation referencing the stale holder, with `withdrawal_reason: "content_miss"`. Holders consistently failing ContentMiss are downweighted in `PeerResolver::resolve_holders`.

### §10.1.4 Structural invisibility — `holds_bytes:sha256:*` suppression for `cohort_scope: self | family` (CEG 0.7 addition)

Per [CIRISRegistry#47](https://github.com/CIRISAI/CIRISRegistry/issues/47) + [ciris.ai/cewp](https://ciris.ai/cewp) load-bearing claim:

> Self and family content never emits the attestation that would tell the rest of the network it exists. You don't need a privacy policy to keep family photos off the federation — the wire format can't carry them in the first place.

CEG 0.7 codifies this as a normative substrate discipline. When a Contribution carries `cohort_scope: self` OR `cohort_scope: family`, the substrate MUST NOT emit a corresponding `holds_bytes:sha256:{prefix}` directory attestation per [§5.6.7](05_namespace.md) — the content's bytes are delivered to admitted members of the relevant self-collective ([§5.6.8.8](05_namespace.md) `identity_occurrence`) or family ([§5.6.8.9](05_namespace.md)) via the at-rest encryption flow (CIRISPersist#152), NOT via the public holder-discovery directory.

**The privacy property is structural, not policy**:

- A non-member peer cannot issue `ContentFetch` for the bytes because no `holds_bytes:sha256:*` attestation names a holder.
- A non-member peer cannot even *discover* the bytes exist via the substrate — the only attestations referencing them are scoped to the self-collective / family and never federate beyond it.
- This is the wire-format-level closure of the cewp **structural invisibility** claim: privacy emerges from format constraints, not from operator policy or legal undertaking.

**Substrate enforcement**:

```
On admission of a Contribution C with cohort_scope ∈ {self, family}:
    substrate MUST NOT emit holds_bytes:sha256:* for C's evidence_refs bytes
    substrate MUST wrap C's DEK via key_grant (§5.6.8.4) to:
        - if cohort_scope == self:   all current identity_occurrences of C.attesting_key_id
        - if cohort_scope == family: all current members of family per C.family_id
    substrate MUST NOT propagate C beyond the self-collective / family scope
    via any other directory or discovery surface
```

**Composition with at-rest encryption flow** (CIRISPersist#152): when content is admitted at `cohort_scope: self`, persist wraps the DEK under each currently-admitted `identity_occurrence`'s `occurrence_key_id`. When content is admitted at `cohort_scope: family`, persist wraps under each `member.key_id` in the named family's current roster. New occurrence / new family-member admission triggers retroactive `key_grant` emission for all extant `cohort_scope: self|family` content (the "I bought a new phone and want my Twitter history" / "I added Carol to the household" flows from §5.6.8.9 worked example).

**Locality dividend** (cewp claim): the structural invisibility mechanism is *why* ~65% of activity stays local in the cewp scaling model — `cohort_scope: self|family` content is the bulk of daily activity (family photos, personal notes, in-household device chatter), and that bulk never federates. Operators do not configure this; the wire format enforces it.

**Boundary cases**:

- `cohort_scope: community | affiliations | federation` content emits `holds_bytes:sha256:*` per status-quo behavior. CEG 0.7 changes ONLY the self/family path.
- A `cohort_scope: self` Contribution that is later promoted via `supersedes` to `cohort_scope: community` (per [§8.1.8.1](08_composition.md) Tiered-Scope promotion) emits `holds_bytes:sha256:*` at promotion time on the NEW Contribution. The original `cohort_scope: self` Contribution's bytes remain structurally-invisible at federation; only the promoted scope's bytes propagate.
- `cohort_scope: self` content with `subject_key_ids` containing a non-self party (e.g., a private note Alice writes ABOUT Bob) is admitted and stays in Alice's self-collective; Bob does NOT receive a key_grant unless Bob is also in Alice's self (not the case for two distinct identities). Bob's [§4.2](04_envelope.md) subject-side revocation authority over the note still composes per CEG 0.6, but the bytes never reach Bob without Alice's explicit re-emit at a higher cohort_scope including Bob.

### §10.1.3 Consent revocations are NOT local-tier-eligible (CEG 0.6 addition)

Per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) Gap 2 + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842). The [CIRISAgent#840](https://github.com/CIRISAI/CIRISAgent/issues/840) CEG-native agent design proposes **local-tier signature deferral** — self-attestations skip the hybrid Ed25519 + ML-DSA-65 signature path locally and only sign at federation-tier promotion. This is sound for producer-only-authority self-attestations (status quo; empty `subject_key_ids`).

**Consent revocations from subjects MUST NOT use the local-tier deferral path.** When a Contribution carries non-empty `subject_key_ids`, any subsequent `consent:state:revoked` emission OR `withdraws` admitted under [§3.2.3 rule 2 or 3](03_primitives.md) from a subject in that set MUST promote to federation-tier within a bounded window. Default window: **24 hours** (operator-tunable per local policy).

**Rationale**: subject-side revocation is the wire-format observability primitive that federation peers depend on to honor consent. If a user revokes consent in the local-tier scope of one agent's substrate, and that revocation is unsigned + unpromoted for an extended window, other federation peers continue propagating the user's data — exactly the failure mode CEG 0.6 exists to close.

**Substrate emission**: substrate MUST emit `hard_case:consent_revocation_promotion_overdue` when a subject-side revocation has been local-tier for longer than the operator-configured window without federation-tier promotion. LensCore composes `detection:consent:promotion_delay_pattern` on top (operator monitoring; not a slashing trigger on its own).

**Composition with [CIRISAgent#840](https://github.com/CIRISAI/CIRISAgent/issues/840) self-attestation pattern**: the CEG-native agent's `consent:partnered:{user_key}` self-attestation (producer-side stance) MAY ride local-tier; the user's `consent:state:revoked` against the agent's stance Contribution (subject-side) MUST NOT. This preserves the cardinality wins from #840 while closing the leak window.

## §10.2 Multi-steward + accord-holder discovery

### `GET /v1/steward-key`

Returns the multi-steward set with M-of-N policy.

Response (`200 OK`):

```json
{
  "stewards": [
    {
      "region": "us",
      "key_id": "us-steward-2026",
      "ed25519_pubkey_b64": "<base64-url>",
      "mldsa65_pubkey_b64": "<base64-url>",
      "hardware_class": "HSM_FIPS_140_3_L3",
      "deployed": true,
      "fingerprint_sha256_hex": "<64-char-lowercase>",
      "cert_validity_self_attest": {
        "valid_until": "<rfc3339_canonical>",
        "signature_b64": "<base64-url>"
      }
    },
    {"region": "eu", ..., "deployed": false},
    {"region": "apac", ..., "deployed": false}
  ],
  "threshold_policy": {"required": 2, "available": 1},
  "response_signature": {
    "signer_key_id": "us-steward-2026",
    "ed25519_b64": "<base64-url>",
    "mldsa65_b64": "<base64-url>",
    "canonical_bytes_label": "ciris.steward_key_response.v1"
  }
}
```

The response itself is hybrid-signed by the serving region's steward over `canonical = "ciris.steward_key_response.v1\n" || sha256_hex_lowercase(canonicalized_json_body_excluding_signature)`. Consumers MUST verify the response signature before trusting any field in the body — placeholder pubkeys without `deployed: true` MUST NOT be promoted to trust roots.

### `GET /v1/accord-holders`

Three named holders with hybrid pubkeys + per-holder `hardware_class` + `provisioned` flag. v1.4 interim ships with placeholder fingerprints + `provisioned: false`; consumers MUST NOT honor CONSTITUTIONAL invocations against placeholders. Response signed by the serving region's steward (same shape as `/v1/steward-key`).

### `GET /v1/accord/holders`

UI wrapper around `/v1/accord-holders` with per-holder `accord_emissions[]` for UI rendering. Same response-signing requirement.

### `GET /v1/rotation-history`

Chronological rotation events from `registry_signing_keys` table. Substrate-conformance migration moves to `federation_keys`.

## §10.3 STH cosigning + witness directory

CIRISVerify v2.12.0+ ships consumer-side `SignedTreeHead::cosign` + `count_valid_witnesses` + `witness_quorum_met`. CEG's emission half:

### `POST /v1/transparency/sth/cosign` (public)

Witness posts cosignature on `(tree_size, root_hash, signed_at)`. Registry verifies hybrid Ed25519 + ML-DSA-65 against witness pubkey in directory; persists on success.

Request body:

```json
{
  "tree_size": <int>,
  "root_hash_sha256_hex": "<64-char-lowercase>",     // per §0.6
  "signed_at": "<rfc3339_canonical>",                // per §0.5
  "witness_key_id": "<string>",
  "ed25519_signature_b64": "<base64-url>",
  "mldsa65_signature_b64": "<base64-url>",
  "consistency_proof_root_hash_sha256_hex": "<64-char-lowercase>",
  "consistency_proof_tree_size": <int>,
  "consistency_proof_path_b64": ["<base64-url>", ...]
}
```

Canonical bytes (witness MUST sign these):

```
canonical = sha256(
    "ciris.sth_cosign.v1\n" ||
    "tree_size=" || decimal_no_leading_zeros || "\n" ||
    "root_hash_sha256=" || sha256_hex_lowercase || "\n" ||  // per §0.6
    "signed_at=" || rfc3339_canonical                       // per §0.5
)
```

Ed25519 over `canonical`; ML-DSA-65 over `canonical || ed25519_sig` (bound payload).

#### §10.3.1 Consistency-proof requirement (normative; addresses CEG 0.1 distsys review)

A witness signing an STH MUST first verify a consistency proof from the prior STH it cosigned (or from genesis if it is the witness's first cosignature against this log). The Registry MUST reject `POST /v1/transparency/sth/cosign` requests that omit the `consistency_proof_*` fields OR whose consistency proof does not verify against the named prior STH. `witness_quorum_met` is therefore "quorum on log consistency," not "quorum on a string."

### `GET /v1/transparency/witnesses` (public)

Directory of registered witnesses. Paginated.

### `GET /v1/transparency/sth/{tree_size}/witnesses` (public)

Cosignatures for an STH with `witness_quorum_met` verdict.

### `POST /v1/transparency/witnesses` (admin; multi-party-gated)

Register a new witness. **0.1 scaffold note**: in 0.1 interim this is bearer-token-gated by `REGISTRY_ADMIN_TOKEN`. **0.2 hardens this to 2-of-3 steward sign-off** (addressing CEG 0.1 cryptographic + red-team review): the request body MUST carry signatures from at least two of the three regional stewards, verified against `GET /v1/steward-key`. Single-token admission is a 0.1 known weakness; production deployments SHOULD operate the 0.1 endpoint behind a corporate IDP gate that enforces multi-party admission out-of-band until the 0.2 multi-sig requirement is normative.

## §10.4 Other Registry endpoints

`GET /v1/builds/{version}` returns the BuildRecordResponse with a `federation_provenance` block (per [§5.2](05_namespace.md) SLSA emission discipline). `GET /v1/verify/build-manifest/{project}/{version}/{target}` (Path B) returns the verbatim signed BuildManifest. `GET /v1/agent_files/{kind}?platform_or_target=...` returns the [§8.1.6](08_composition.md) trust-composition layers. `GET /v1/partner/{key_id}` composes ProfileScorecard data from existing tables.

Full response schemas for these endpoints land in the Rust handlers + OpenAPI export; CEG 0.2 commits to publishing a versioned OpenAPI spec alongside this document.

---

[← §9 HUMANITY_ACCORD](09_humanity_accord.md) | **§10 Endpoints** | [Next: §11 Governance →](11_governance.md)
