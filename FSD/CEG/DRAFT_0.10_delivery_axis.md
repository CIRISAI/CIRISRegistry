# CEG 0.10 — DRAFT skeleton — Delivery axis: observer-share + streaming multicast

**Status: DRAFT / NOT NORMATIVE.** Staged scaffold for the CEG 0.10 RC1 round per [§15.6](15_gaps.md). The live spec (`00`–`17`) remains **0.9** until this weaves in. This becomes normative only when the open gates close (below) and the version bumps 0.9 → 0.10.

**Driver:** [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857) (observer-share) + [#44](https://github.com/CIRISAI/CIRISRegistry/issues/44) (CEG 0.5 `live_stream`, **absorbed here**) + [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142) (streaming substrate; impl prerequisite for the streaming half).

**Framing (load-bearing):** delivery is the **third orthogonal axis** — visibility (`cohort_scope`) + revocability (`subject_key_ids`, [§4.2](04_envelope.md)) + **delivery (this)**. [§3](03_primitives.md)'s five primitives (`scores` + `delegates_to`/`supersedes`/`withdraws`/`recants`) are **untouched**. 0.10 is a §10-endpoint + §4-envelope + §8.1.13-composition extension, NOT a grammar change.

---

## Open gates — MUST pin before this is normative + version-bumped

- [ ] **RC1-1** (Persist) — confirm `key_grant.rotation_chain` impl-index `(content_sha256, recipient_key_id)` / V054 matches the [§5.6.8.4](05_namespace.md) grant-supersession semantics, and is **distinct from** the new per-`(stream_id, epoch)` epoch-key index in §10.5.3.
- [ ] **RC1-3** (Edge E1) — transit-key = hop-by-hop wrap **under** the E2E epoch DEK (two layers), not replacing the cascade.
- [ ] **RC1-4** (Edge E2) — RC1 multicast = pull-only (relay/fan-out tree → 1.x).
- [ ] **RC1-7** (router) — ratify constants `K=64` / `T=2s` / cosign per-epoch / `MAX_CHUNKS_PER_EPOCH=2²⁴` + accountable-stream quorum = Policy E ([§8.1.5](08_composition.md)).
- [x] **RC1-2** — `§10.5` ratified as the streaming-clause home (2026-06-01).
- (impl, not spec-gating) **CIRISPersist#142** — `put_blob_chunk` / `seal_stream` / stream-chunk table (greenfield; 0 occurrences today).

## File-landing map (when woven into 00–17)

| Piece | Lands in | Kind |
|---|---|---|
| `delivery_mode:{pull\|push}` | [§4](04_envelope.md) envelope | new optional field (default `pull`) |
| `listed:{public}`, `history_on_join:{full\|from_join}` | [§4](04_envelope.md) / per-membership | new optional flags |
| subscriber-set = community + delivery extension | [§8.1.13](08_composition.md) Policy M | composition extension |
| `delivery_receipt:{stream_id}` | [§7](07_reserved.md) | new reserved prefix |
| streaming transport / per-stream log / chunk seal / receipts | **§10.5 (NEW)** | new endpoint section |
| `rotation_chain` hygiene corrections | [§11.7.1](11_governance.md), [§1.4](01_foundation.md) path-8, [§5.6.8.9](05_namespace.md), [§16.1](16_references.md) | editorial |

---

## §4 (envelope) — new fields [LOCKED]

- `delivery_mode: {pull | push}` — default `pull`. `pull` = subscribers discover via `holds_bytes` directory; `push` = substrate fan-out to the live-delivery set (§10.5.5).
- `listed: {public}` — optional per-membership opt-in (**D1**). Default private: roster is producer- + self-queryable, never globally enumerable. Public listing mirrors the [§11.8.3](11_governance.md) location opt-in.
- `history_on_join: {full | from_join}` — per-target (**D3**). `full` = new member gets Option-A retroactive catch-up (trace/registry-export backlog); `from_join` = current epoch forward only (live media). Default `from_join`.

## §8.1.13 Policy M — delivery extension [LOCKED]

Subscriber-set = a `community` admitted `producer_gated | open`; **"subscribe" = join that community**. Inherits revocation, consensus, and structural-invisibility from Policy M unchanged. `delivery_mode` selects pull (discovery) vs push (fan-out). **E2E directed delivery = `key_grant` cascade of the stream-epoch DEK over the roster** (D2/§10.5.3). Cardinality: **N=1 (observer-share)** → a single `key_grant`, no epoch needed; **N>1 (multicast)** → flat per-epoch cascade.

## §10.5 (NEW) — Streaming transport, per-stream logs & delivery receipts

### §10.5.0 Framing [LOCKED]
A stream is **its own per-stream transparency-log instance** (`log_id = stream_id`). It **MUST NOT** be appended into the federation provenance log ([§10.3](10_endpoints.md)'s global log carrying builds/licenses/identities) — millions of media chunks would pollute provenance. It **reuses** the §10.3 `SignedTreeHead` / `ConsistencyProof` / `WitnessConsistencyProof` / cosign abstractions, instantiated per stream (same RFC 6962 algorithm).

### §10.5.1 Per-stream log + stream-root [LOCKED — V1]
- `log_id = stream_id`; chunks = leaves; stream-root = `SignedTreeHead{ log_id: stream_id, tree_size: chunk_count, root_hash, timestamp, signature }`.
- **Producer signs the STH — mandatory** (authenticity root; hybrid Ed25519 + ML-DSA-65 via the §10.3 `signing_bytes` discipline).
- **Witness cosign — optional**, via the [§10.3](10_endpoints.md) path verbatim = the best-effort / accountable split (D5). For accountable streams, [§10.3.1](10_endpoints.md) consistency-proof is the **anti-equivocation** guarantee (producer can't show different chunk-K to different subscribers, can't rewrite mid-stream).
- **Cadence:** signed root every `K` chunks OR `T` sec (whichever first), always at an **epoch boundary** + at `sealed_at`; witness cosign runs **per-epoch** (off the hot path). `K=64` / `T=2s` — TODO(RC1-7 ratify).
- **Incremental verify (D4):** the per-leaf "root-after-K" + verify chunk K's inclusion against the nearest signed STH ≥ K via the consistency path. No new commitment structure.

### §10.5.2 Chunk seal + STREAM nonce [LOCKED — V2]
- ring AES-256-GCM; 12-byte nonce = **7B HKDF-derived prefix ‖ 4B BE counter ‖ 1B last-flag**.
- `prefix = HKDF(epoch_dek; info = "ciris-stream-nonce/v1" ‖ stream_id ‖ epoch)[0..7]` — derived, not transmitted (matches the `KEY_GRANT_V1_INFO` versioned-context pattern).
- 32-bit counter; **forced epoch-roll before 2³² wrap**; operational cap `MAX_CHUNKS_PER_EPOCH = 2²⁴` — TODO(RC1-7). Cross-epoch counter reset is **nonce-safe** (DEK changes ⇒ `(DEK_e, 0) ≠ (DEK_{e+1}, 0)`); the only invariant is strictly-monotonic-within-epoch.
- `last_flag = 0x01` on the final chunk → truncation + append resistance.

### §10.5.3 Epoch keying + cascade [LOCKED — D2/D3; substrate-pending #142]
- **Stream-epoch DEK seals content O(1)**; **per-subscriber `key_grant` cascade distributes the 32-byte epoch key O(N)/epoch** (sender-key/Megolm shape) = [§8.1.12.4](08_composition.md) Policy-L cascade applied to a community roster against a *rotating key*.
- **Epoch index: monotonic, per `stream_id`, greenfield** — NOT `key_grant.rotation_chain` (that's content-addressed grant supersession, [§5.6.8.4](05_namespace.md); separate axis). substrate-pending [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142) (`put_blob_chunk`/`seal_stream`/stream-chunk table); the epoch-key grant is then `key_grant` scoped to `(stream_id, epoch)` — a parallel index, not a new mechanism. TODO(RC1-1) Persist confirm.
- **Triggers (D3):** removal → **mandatory rotation** (the forward-only-unsubscribe enforcement); add → **no rotation** + Option-A catch-up ([§11.7.1](11_governance.md)); time/bytes → optional hygiene.
- **FS = forward-only, no PCS** (consistent with 0.7). MLS O(log N) rekey tree = **1.x, additive** (tree-position on the opaque grant payload; no table migration).
- **Catch-up bound (P4):** `min(operator depth cap [Lens-core knob, NOT a substrate constant], chunk-eviction horizon)`. An evicted-epoch grant returns **`ContentMiss` — fail-honest, no silent gap** (MISSION 182/197/414/447). Ship the operator bound **with** the cascade (P3: 10⁶ grants/rekey otherwise).

### §10.5.4 Delivery receipts [LOCKED — D5/V3]
- `delivery_receipt:{stream_id}` = **new [§7](07_reserved.md) reserved prefix**; **validated-not-adjudicated** (MISSION fail-honest; [§1.4](01_foundation.md) — Verify authenticates origin, does not compose "delivered"/"owes N").
- Canonical bytes (domain-separated + length-prefixed, matching `SignedTreeHead::signing_bytes`): `(subscriber_key, stream_id, epoch, K, chunk_root)`. Keep both `epoch` (entitlement key-epoch, for per-epoch billing) and `K`/`chunk_root` (position + committed root).
- **Verify check = a JOIN, not a sig-check:** (1) sig valid *(necessary, not sufficient)*; (2) `chunk_root` equals a **real published `SignedTreeHead.root_hash`** for `log_id = stream_id` at `tree_size ≥ K` (accountable ⇒ witness-cosigned, defeats producer↔subscriber collusion on a private root); (3) *(recommended, accountable)* inclusion proof chunk K → `chunk_root`.
- **Semantics:** proof-of-**delivery** (received bytes committing to chunk K), **NOT** proof-of-consumption (subscriber may not hold the epoch DEK). Consumers must not overclaim.
- Best-effort default; **accountable opt-in for profiles C/D** (registry propagation, emergency) — receipts are the ACK set. Quorum = Policy E ([§8.1.5](08_composition.md)) — TODO(RC1-7).

### §10.5.5 Transport [TODO — Edge]
- **TODO(E2 / RC1-4):** RC1 multicast = **pull-only** — producer seals chunks under the epoch DEK → `holds_bytes` directory → subscribers pull. Relay/fan-out tree → 1.x (#46/#43).
- **TODO(E1 / RC1-3, security-critical):** transit-key (prod-lens-via-transit-key, #857) = **hop-by-hop transport wrap UNDER the E2E epoch DEK** (two layers); MUST NOT replace the cascade (else a relay reads plaintext).
- **TODO(E3):** live-delivery set is **node-local (Persist holds; Edge sends)**; entitlement∩liveness join happens at fan-out.
- **TODO(E4):** durable entitlement (roster + epoch-key grants) rides the **existing federation-attestation edge path** (#41 cutover) — just more `federation_attestations`; no net-new Edge transport for the durable side.

## §7 — new reserved prefix [LOCKED, emitter TBD]
`delivery_receipt:{stream_id}` — validated-not-adjudicated. Emitter rule TODO: the validating substrate/verify role (per CEG 0.9 [§7.0.1](07_reserved.md) identity_type-as-set, the emitter's role-set must contain the relevant validating role).

## D6 liveness invariant [LOCKED]
Two sets, never conflated: **entitlement roster** (signed CEG envelope, edge-propagated, durable, logged — it's evidence, must propagate + be auditable) vs **live-delivery set** (node-local, TTL sec/min — generalizing the [§10.1.2](10_endpoints.md) `EdgeConfig.holds_bytes_ttl_seconds` 24h default down to seconds/minutes — **NEVER an attestation, never `holds_bytes`, never replicated, never logged**). Heartbeat-suppression is a **producer-side-refusal invariant** (same class as the [§10.1.4](10_endpoints.md) `cohort_scope: self|family` holds_bytes suppression). Missed members fall back to pull on reconnect.

## `rotation_chain` hygiene corrections (fold into the 0.10 pass) [from §15.6.4]
- **[§11.7.1](11_governance.md):** strike "the wire-format primitives are in place (`key_grant.rotation_chain` from CEG 0.3 covers the rotation mechanic)" — `rotation_chain` does not provide key rotation, and Option A does no rotation.
- **[§1.4](01_foundation.md) path-8 + [§5.6.8.9](05_namespace.md) + [§16.1](16_references.md) 0.7 lineage:** "DEK cascade rides existing `key_grant.rotation_chain`" → "rides the `key_grant` wrap + Option-A re-grant." `rotation_chain` stays as the [§5.6.8.4](05_namespace.md) grant-supersession lineage — a separate axis from epoch rotation.
- **[§5.6.8.4](05_namespace.md):** add a one-line disambiguation — `rotation_chain` = grant-supersession lineage (a list of prior `key_grant_id`s); per-`(stream_id, epoch)` epoch keying (§10.5.3) is a distinct parallel index.
