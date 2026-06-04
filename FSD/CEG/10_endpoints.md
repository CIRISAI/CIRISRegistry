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

## §10.5 Streaming transport, per-stream logs & delivery receipts (CEG 0.10 addition)

Per [CIRISRegistry#44 absorbed](https://github.com/CIRISAI/CIRISRegistry/issues/44) + [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857) (observer-share driver) + [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142) (streaming substrate prerequisite). §10.5 is the **delivery axis** — the third orthogonal envelope concern alongside visibility (`cohort_scope`) and revocability (`subject_key_ids` per [§4.2](04_envelope.md)). [§3](03_primitives.md)'s 1+4 primitive set is untouched; §10.5 is endpoint + envelope + composition extension, NOT a grammar change.

**Bifurcation (per [§15.6.1](15_gaps.md))**:

| Half | Cardinality | RC1 status |
|---|---|---|
| **Observer-share / directed delivery** (single Contribution → subscriber-set; no `stream_id`) | N=1 typical; per-subscriber `key_grant` | **impl-live**; substrate paths shipping per [§5.6.8.4](05_namespace.md) `key_grant` + [§8.1.13](08_composition.md) Policy M membership |
| **Media / streaming multicast** (`live_stream` chunk-DAG; per-`(stream_id, epoch)` keys) | N>1; flat per-epoch `key_grant` cascade | **spec-now, impl substrate-pending [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142)**; subsections §10.5.2 / §10.5.3 / §10.5.4 ride this dependency |

### §10.5.0 Framing (normative)

A stream is **its own per-stream transparency-log instance** (`log_id = stream_id`). A `live_stream` (CEG 0.5 sub_kind absorbed into 0.10) MUST NOT append chunks into the federation provenance log ([§10.3](#103-sth-cosigning--witness-directory)'s global log carrying builds / licenses / identities) — millions of media chunks would pollute provenance and inflate the global tree. The §10.5 path **reuses** the §10.3 `SignedTreeHead` / `ConsistencyProof` / `WitnessConsistencyProof` / cosign abstractions, instantiated per-stream as separate log instances under the same RFC 6962 algorithm.

The 1+4 wire-format lockdown holds: there are no new `attestation_type` values. Stream chunks ride content addressing; stream-roots ride the existing `SignedTreeHead` shape; delivery receipts ride `scores` against the new `delivery_receipt:{stream_id}` reserved prefix ([§7.9](07_reserved.md)).

### §10.5.1 Per-stream log + stream-root (normative — V1 lock)

For each live_stream:

- `log_id = stream_id`; chunks = leaves; stream-root = `SignedTreeHead{ log_id: stream_id, tree_size: chunk_count, root_hash, timestamp, signature }`
- **Producer signs the STH — MANDATORY** authenticity root; hybrid Ed25519 + ML-DSA-65 per the [§10.3](#103-sth-cosigning--witness-directory) `signing_bytes` discipline; canonical bytes per [§0.9](00_conformance.md) JCS for the envelope-bearing wrapper
- **Witness cosign — OPTIONAL**, via the [§10.3](#103-sth-cosigning--witness-directory) path verbatim. This is the best-effort / accountable split (D5 per [§15.6.2](15_gaps.md)):
  - **Best-effort** (open media) → producer-signed root only; impl-pending [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142) only
  - **Accountable** (paid media, registry propagation, emergency) → witness cosign per [§10.3.1](#1031-consistency-proof-requirement-normative-addresses-ceg-01-distsys-review) consistency-proof, which is the **anti-equivocation guarantee** — producer cannot show different chunk-K to different subscribers nor rewrite mid-stream. Accountable tier is impl-pending BOTH #142 AND [CIRISRegistry#34](https://github.com/CIRISAI/CIRISRegistry/issues/34) (STH consistency-proof enforcement)
- **Cadence**: producer publishes a signed root every `K` chunks OR `T` seconds (whichever first), **always at an epoch boundary** (§10.5.3) + at `sealed_at`. Witness cosign runs **per-epoch** (coarser than per-K to keep cosign-quorum cost off the hot path). Default pins: **K=64, T=2s** (operator-tunable; ratification pending per [§15.6.3](15_gaps.md) RC1-7)
- **Incremental verify** (D4): each leaf's "root-after-K" lets a subscriber verify chunk K's inclusion against the nearest signed STH ≥ K via the [§10.3.1](#1031-consistency-proof-requirement-normative-addresses-ceg-01-distsys-review) consistency path. No new commitment structure beyond the per-leaf chain-link + periodic STH that §10.3 already provides.
- **Accountable-stream quorum**: Policy E ([§8.1.5](08_composition.md) locality-scaled quorum) applies — not a fixed N. Emergency-channel roots want a higher quorum than a paid-media stream; locality scaling provides the gradient.

### §10.5.2 Chunk seal + STREAM nonce (normative — V2 lock)

Per-chunk content sealing uses AES-256-GCM (NIST FIPS 197 + SP 800-38D). The 12-byte (96-bit) nonce follows the **STREAM** layout (Hoang, Reyhanitabar, Rogaway, Vizár — *Online Authenticated-Encryption and its Nonce-Reuse Misuse-Resistance*, CRYPTO 2015):

```
nonce[12] = prefix[7] ‖ counter_be[4] ‖ last_flag[1]
```

- **`prefix[7]`** — derived, NOT transmitted: `prefix = HKDF-SHA256(epoch_dek; info = "ciris-stream-nonce/v1" ‖ stream_id ‖ epoch)[0..7]`. Matches the **`KEY_GRANT_V1_INFO`** versioned-context HKDF pattern at [`CIRISVerify/src/ciris-crypto/src/key_grant.rs:71`](https://github.com/CIRISAI/CIRISVerify/blob/main/src/ciris-crypto/src/key_grant.rs) (`b"cewp-key-grant/v1"`, used at `key_grant.rs:163` in `kdf::hkdf_sha256(shared_secret, &salt, KEY_GRANT_V1_INFO, 32)`). Per-`(stream_id, epoch)` unique; verifiable by any holder of the epoch DEK
- **`counter_be[4]`** — 32-bit big-endian; hard ceiling 2³²−1 chunks per epoch. Substrate MUST force an epoch roll before wrap. Recommended operational cap: **`MAX_CHUNKS_PER_EPOCH = 2²⁴`** (~16.7M chunks/epoch) to keep per-epoch state + proof sizes bounded (operator-tunable; ratification pending per RC1-7)
- **`last_flag[1]`** — `0x01` on the final chunk of an epoch (sealed by `seal_stream` per §10.5.3); `0x00` otherwise. The distinct nonce on the final chunk gives **truncation + append resistance**: an adversary cannot drop the final chunk and pass off a short stream, nor append past a sealed segment

**Cross-epoch counter reset is nonce-safe (normative reasoning)**: GCM's catastrophic case is reuse of a `(key, nonce)` pair. On epoch roll the DEK changes, so a reset counter lives in a different keyspace — `(DEK_e, nonce=0)` and `(DEK_{e+1}, nonce=0)` are distinct pairs. The enforced invariant is therefore only within a single epoch: counter strictly monotonic, never wraps (guaranteed by the forced roll). Across epochs, reset is free.

### §10.5.3 Epoch keying + cascade (normative — D2 / D3; substrate-pending #142)

The stream-epoch DEK seals content **O(1)**; the per-subscriber `key_grant` cascade distributes the 32-byte epoch key **O(N)/epoch** (sender-key / Megolm shape) = [§8.1.12.4](08_composition.md) Policy-L cascade applied to a community roster against a *rotating* key.

**Epoch index is monotonic, per-`stream_id`, greenfield** — a **separate addressing axis** from `key_grant.rotation_chain`:

| Axis | Addressing | Where it lives | Supersession |
|---|---|---|---|
| Content-addressed grant supersession (CEG 0.3) | `(content_sha256, recipient_key_id)` | [`cirisnode_contributions`](https://github.com/CIRISAI/CIRISPersist) V054 partial indexes (planner-AND'd) | `rotation_chain` payload-level lineage (list of prior `key_grant_id`s); walked reader-side |
| Stream/epoch-addressed grant supersession (CEG 0.10) | `(stream_id, epoch[, recipient])` | `federation_stream_chunks(stream_id, seq)` (Persist#142 step 3; v3.9.0 target; **NOT YET LANDED — unowned/unscheduled**) | Same `rotation_chain` payload-level supersession **reused on the new axis** (RC1-1 ✅ Persist on record) |

⚠️ **Not pure-additive at the Persist constraint layer (RC1-1c)**: the V054 cross-column CHECK requires `key_grant` rows be content-addressed (`media_content_sha256 IS NOT NULL`). The §10.5.3 epoch-key axis with NULL `media_content_sha256` would be REJECTED by today's CHECK. Introducing the new axis requires a **parallel CHECK arm migration** at Persist (content-addressed OR stream/epoch-addressed) — a bounded constraint migration, not a pure index-add. The spec text does not claim "purely additive" at the Persist constraint layer.

**Epoch triggers (D3)**:

| Trigger | Behavior | Forward-secrecy implication |
|---|---|---|
| Member removal | **MANDATORY rotation** — the forward-only-unsubscribe enforcement | Subsequent epochs sealed under a DEK the removed member doesn't have |
| Member addition | NO rotation + Option-A catch-up per [§11.7.1](11_governance.md) (subject to `history_on_join`) | New member gets `key_grant`s for the current epoch + (optionally) prior epochs per the `history_on_join` envelope field ([§4](04_envelope.md)) |
| Time / bytes | Optional hygiene rotation | Operator policy; default off |

**Forward secrecy = forward-only, NO PCS** (consistent with the [§11.7.1](11_governance.md) CEG 0.7 Option A discipline). MLS-style O(log N) rekey tree = **1.x, additive** (tree-position rides on the opaque `key_grant` payload; no table migration).

**Catch-up bound (P4)**: `min(operator depth cap [LensCore knob, NOT a substrate constant], chunk-eviction horizon)`. Three distinct windows that are NOT conflated: chunk-eviction horizon ≠ [§10.1.2](#1012-holder-directory-ttl--contentmiss-feedback) `holds_bytes` 24h TTL ≠ grant durability. A catch-up request against an evicted epoch returns **`ContentMiss` — fail-honest, no silent gap** (consistent with the [`MISSION.md`](../../MISSION.md) fail-honest invariant). Operators MUST ship the P4 cap **with** the cascade, else 10⁶ grant Contributions per rekey is the unbounded worst case.

### §10.5.4 Delivery receipts (normative — D5 / V3 lock)

A `delivery_receipt:{stream_id}` Contribution (new reserved prefix per [§7.9](07_reserved.md)) is a subscriber's signed acknowledgement that they received chunk K under the named stream + epoch. **Best-effort default**; opt-in for **accountable** profiles (registry propagation, emergency).

**Canonical bytes** (domain-separated + length-prefixed, matching `SignedTreeHead::signing_bytes` discipline; per [§0.9](00_conformance.md) the envelope-bearing wrapper is JCS-encoded, but the receipt's signed-bytes inner payload follows the explicit-length-prefix shape below):

```
receipt_signing_bytes =
    "ciris-delivery-receipt/v1"                       // domain separator
  ‖ len(subscriber_key) ‖ subscriber_key
  ‖ len(stream_id)      ‖ stream_id
  ‖ epoch        (u64 LE)
  ‖ chunk_root   ([u8; 32])
  ‖ K            (u64 LE)
```

Both `epoch` (key-rotation index — for per-epoch entitlement / billing) and `K`/`chunk_root` (chunk position + its committed root) are independent indices — both required (each names a distinct authorization scope).

**Verify check is a JOIN, NOT a sig-check**:

1. **Signature valid** over the canonical bytes (subscriber hybrid Ed25519 + ML-DSA-65 sig). Necessary but **not sufficient**.
2. **`chunk_root` is a real published STH root** — MUST equal a `SignedTreeHead.root_hash` actually published for `log_id = stream_id` at `tree_size ≥ K`. A phantom / self-invented root → REJECT. For accountable streams, "published" means **witness-cosigned** (the §10.3 path), so the subscriber cannot collude with the producer on a private root.
3. *(Recommended for accountable)* **Inclusion proof** chunk K → `chunk_root`. Upgrades the receipt from "subscriber saw a root" to "subscriber saw a root that provably commits to chunk K".

**Semantics — proof-of-DELIVERY, not proof-of-CONSUMPTION**: the receipt proves the subscriber received bytes committing to chunk K. It does NOT prove they decrypted those bytes (they may not hold the epoch DEK). Consumers MUST NOT overclaim a delivery receipt as proof of consumption. Per the [`MISSION.md`](../../MISSION.md) fail-honest invariant + [§1.4](01_foundation.md) "Verify authenticates origin, does not compose 'delivered'/'owes N'", Verify's role is validation-not-adjudication: emit the validated receipt as an attestation on `delivery_receipt:{stream_id}` — the "delivered" verdict is consumer policy.

**Accountable-stream receipt quorum**: Policy E ([§8.1.5](08_composition.md) locality-scaled) — same shape as the §10.5.1 STH quorum. Ratification pending per RC1-7.

### §10.5.5 Transport — Edge layer (normative — E1–E4 lock per [§15.6.2](15_gaps.md))

| Decision | Behavior |
|---|---|
| **E2 — pull-only RC1** | RC1 multicast = **pull-only**: producer seals chunks under the epoch DEK → emits `holds_bytes:sha256:*` (per [§10.1](#101-transport-substrate-for-byte-level-content)) → subscribers pull via the existing `ContentFetch` path. Relay / fan-out tree → 1.x (CIRISRegistry#46 / #43) |
| **E1 — two-layer crypto (security-critical)** | Transit-key (per [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857) prod-lens-via-transit-key) is a **hop-by-hop transport wrap UNDER the E2E epoch DEK** (two independent crypto layers). MUST NOT replace the cascade — a relay never sees plaintext. Transit-key is for path-confidentiality; epoch-DEK is for end-to-end content confidentiality |
| **E3 — fan-out = entitled ∧ reachable** | **Persist owns durable entitlement** (the roster: signed CEG envelopes, replicated, logged). **Edge owns transport-reachability** via [`reachability.rs`](https://github.com/CIRISAI/CIRISEdge) (CIRISEdge#29) node-local presence tracker. Fan-out targets the intersection. Reachability is NEVER an attestation, never `holds_bytes`, never replicated, never logged — consistent with the [§10.1.4](#1014-structural-invisibility--holds_bytessha256-suppression-for-cohort_scope-self--family-ceg-07-addition) `cohort_scope: self\|family` structural-invisibility shape |
| **E4 — durable side rides existing federation-attestation path** | Durable entitlement (roster + epoch-key grants) rides the **existing federation-attestation Edge path** (CIRISRegistry#41 handler cutover) — just more `federation_attestations` rows. NO net-new Edge transport for the durable side. Net-new is only on §10.5.1 streaming-log endpoints |

### §10.5.6 D6 liveness invariant — entitled vs reachable (normative)

Two sets are NEVER conflated:

- **Entitlement roster** (Persist-owned): signed CEG envelope, Edge-propagated, durable, logged. It's **evidence** — it MUST propagate + be auditable. Per [§8.1.13](08_composition.md) Policy M community-membership composition
- **Live-reachability set** (Edge-owned): generalizes the [§10.1.2](#1012-holder-directory-ttl--contentmiss-feedback) `EdgeConfig.holds_bytes_ttl_seconds` 24h default down to seconds-to-minutes for live-multicast. Node-local presence tracker (Edge `reachability.rs` per CIRISEdge#29). **NEVER an attestation, never `holds_bytes`, never replicated, never logged**

**Fan-out invariant: `fan_out(C) = entitled(C) ∩ reachable(now)`**.

**Heartbeat-suppression discipline**: this is a **producer-side-refusal invariant** (same class as the [§10.1.4](#1014-structural-invisibility--holds_bytessha256-suppression-for-cohort_scope-self--family-ceg-07-addition) `cohort_scope: self|family` `holds_bytes` suppression). Missed (entitled-but-unreachable) members fall back to pull on reconnect — substrate does NOT keep retrying push, does NOT emit a "delivery_failed" attestation, does NOT log liveness state. The reconnect-then-pull catch-up rides §10.5.3 `history_on_join`.

### §10.5.7 What CEG 0.10 documents

- The delivery axis as the third orthogonal envelope concern (visibility + revocability + delivery)
- The bifurcated observer-share (impl-live) vs streaming multicast (substrate-pending #142) split
- Per-stream transparency-log instances ([§10.5.1](#1051-per-stream-log--stream-root-normative--v1-lock))
- STREAM nonce derivation ([§10.5.2](#1052-chunk-seal--stream-nonce-normative--v2-lock)) reusing the `KEY_GRANT_V1_INFO` HKDF pattern
- Epoch-keying cascade on a separate addressing axis from `rotation_chain` ([§10.5.3](#1053-epoch-keying--cascade-normative--d2--d3-substrate-pending-142))
- Delivery-receipt canonical bytes + the validated-not-adjudicated discipline ([§10.5.4](#1054-delivery-receipts-normative--d5--v3-lock))
- Edge transport with two-layer crypto + pull-only RC1 + entitled-∧-reachable fan-out ([§10.5.5](#1055-transport--edge-layer-normative--e1e4-lock-per-156215_gapsmd))
- D6 liveness invariant ([§10.5.6](#1056-d6-liveness-invariant--entitled-vs-reachable-normative))

What CEG 0.10 does NOT do:

- Change the 1+4 primitive set ([§3](03_primitives.md)) — delivery rides existing primitives
- Bundle the streaming-half substrate impl in Persist — that lives at Persist#142 + the RC1-1c parallel CHECK arm migration
- Specify the relay / fan-out tree shape for push-mode multicast — RC1 is pull-only; push tree → 1.x (CIRISRegistry#46 / #43)
- Lock the constants K / T / MAX_CHUNKS_PER_EPOCH or the accountable-stream quorum — operator-tunable; ratification pending per [§15.6.3](15_gaps.md) RC1-7

---

[← §9 HUMANITY_ACCORD](09_humanity_accord.md) | **§10 Endpoints** | [Next: §11 Governance →](11_governance.md)
