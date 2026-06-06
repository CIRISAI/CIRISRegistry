[← §14 Glossaries](14_glossaries.md) | **§15 Gaps** | [Next: §16 References →](16_references.md)

---

# §15 Concerns + acknowledged gaps

Three independent methodologies (`PRIOR_ART_SCAN.md` structural comparison, `SOTA_SCAN.md` production-validation comparison, *Magnifica Humanitas* encyclical mapping + CIRISRegistry#30 283-story stress test) surfaced concerns. CEG 0.1 critical-review pass added five more reviewer perspectives (cryptography, distributed systems, standards architecture, adversarial red-team, application development). Concerns named here so external reviewers see them acknowledged rather than discovered.

## §15.1 Closed gaps

| Gap | Status | Resolution |
|---|---|---|
| G1 — Revocation privacy | **RETRACTED** | Wrong threat model. The Registered path's thesis is public verifiability per [`../MISSION.md`](../../MISSION.md) §1.1. |
| G2 — Rules-layer Sybil | **MITIGATED** | [§11.2](11_governance.md) step 5 1-of-6 accord/steward sign-off + §11.2.3 meta-amendment entrenchment. |
| G3 — Narrow-cell fresh-quorum recusal | **MITIGATED** | [§8.1.5](08_composition.md) locality-scaled quorum + §8.1.5.1 sub-quorum fallback. |
| v1.4 T-3 #1 testimonial_witness:{kind} | **CLOSED** via [§5.6.3](05_namespace.md) new prefix; opened to open vocabulary in CEG 0.1. |
| v1.4 T-3 #2 labor:individual_loss | **CLOSED by documentation**. Existing `non_maleficence:*` with `target_key_id = affected_individual` + `witness_relation: external` carries the per-individual claim. |
| v1.4 T-3 #5 Constitutional-constraint grounding | **CLOSED in [§1.2](01_foundation.md) prose**. Wire stays tradition-multiplicity-neutral per [§1.3.1](01_foundation.md). |
| 0.1-CRIT canonical-bytes newline-injection | **SCAFFOLDED** in [§5.2.1](05_namespace.md) with 0.2 redesign committed (TupleHash128 + domain-separation labels) |
| 0.1-CRIT supersedes/withdraws/recants ordering | **CLOSED** in [§6.1](06_relations.md) precedence rule + idempotency dedup |
| 0.1-CRIT cell_pool < min_pool cliff | **CLOSED** in [§8.1.5.1](08_composition.md) sub-quorum fallback paths |
| 0.1-CRIT no RFC 2119 anchor | **CLOSED** in [§0.1](00_conformance.md) |
| 0.1-CRIT no versioning policy | **CLOSED** in [§0.3](00_conformance.md) SemVer mapping |
| 0.1-CRIT no normative References | **CLOSED** in [§0.4](00_conformance.md) |
| 0.1-CRIT endpoint response schemas | **PARTIALLY CLOSED** in [§10.0](10_endpoints.md) common-shape + error-envelope; full OpenAPI committed for 0.2 |
| 0.1-CRIT reserved-prefix enforcement empty pointer | **CLOSED** in [§7.0](07_reserved.md) inline enforcement rule |
| 0.1-HIGH STH cosignature consistency-proof | **CLOSED** in [§10.3.1](10_endpoints.md) |
| 0.1-HIGH holds_bytes full-SHA verify + TTL | **CLOSED** in [§10.1.1 / §10.1.2](10_endpoints.md) |
| 0.1-HIGH delegates_to depth + cycle | **CLOSED** in [§13.3](13_anti_patterns.md) anti-pattern + consumer-policy caps |
| 0.1-HIGH HUMANITY_ACCORD invocation replay | **CLOSED** in [§9.2.1](09_humanity_accord.md) discriminator + nonce in signed bytes |
| 0.1-HIGH `notify` vs CONSTITUTIONAL social-canonicity | **CLOSED** in [§9.2.2](09_humanity_accord.md) consumer-UI requirement |
| 0.1-HIGH /v1/steward-key placeholder authenticity | **CLOSED** in [§10.2](10_endpoints.md) response-signing requirement |
| 0.1-MED open-vocabulary collision | **CLOSED** in [§11.2.2](11_governance.md) collision rule |
| 0.1-MED occurrence_id self-assertion | **ACKNOWLEDGED** in [§4](04_envelope.md) + R6 below |
| 0.1-MED `withdraws` arbitrage | **CLOSED** in [§13.4](13_anti_patterns.md) consumer-policy countermeasure |
| 0.2 — `attestation:l{N}:*` carried ladder-position in wire (T2 violation inherited from FSD-002 v1.0) | **CLOSED in CEG 0.2** by [§5.2](05_namespace.md) wire-break rename to mechanism-only prefixes + [§8.1.9](08_composition.md) Policy I consumer-side Attestation-Ladder Composition + [§13.1](13_anti_patterns.md) deprecation entry. Verify v3.7.0 caught the principle; CEG 0.1 inherited the wrong shape from FSD-002 v1.0 baseline without re-examining against the [§1.3.1](01_foundation.md) T2 gate; CEG 0.2 ratifies the correction. |
| 0.9 — Envelope canonical-bytes round-trip determinism (omit-vs-materialize for optional fields with documented defaults) | **CLOSED in CEG 0.9** by [§0.9](00_conformance.md) — JCS pinned as the canonical encoding (RFC 8785 was already in [§0.4](00_conformance.md) normative refs but not declared as THE envelope encoding); [§0.9.2](00_conformance.md) makes the omit-vs-materialize rule explicit (defaults are interpretation-time, NOT encoding-time; relay MUST preserve member presence/absence as the producer signed); [§0.9.3](00_conformance.md) catalogs every optional [§4](04_envelope.md) field through CEG 0.8; [§0.9.5](00_conformance.md) walks the worked attack the rule closes. Surfaced by external critical review of CEG ≤ 0.8; the rule was implicit and unspecified pre-0.9, which left a real round-trip-determinism hole. Drive-by fixes: [§10.0](10_endpoints.md) `CEG-Version` header un-stuck from `0.1` → tracks README major.minor; [§12.4](12_translation.md) `dimensions.json` aspirational target re-pinned from CEG 0.2 → CEG 1.0 (the 0.3 → 0.8 wave prioritized namespace additions over tooling). |
| 0.10 — Canonical-hash wire form + preimage convention unpinned (subject-identity non-interoperable across producers) | **CLOSED in CEG 0.10** by [§4.2.2.1](04_envelope.md) — per [CIRISRegistry#53](https://github.com/CIRISAI/CIRISRegistry/issues/53) (NodeCore CEG 0.6 ingest implementation surfaced the gap). **PIN-1**: preimage convention `{platform}:{entity_kind}:{id}` (split-on-first-two-colons so colon-bearing ids like Matrix MXIDs survive; lowercased platform+entity_kind from open vocab; immutable id verbatim) + 5 cross-implementation conformance vectors with real sha256 output. **CONFIRM-2**: canonical-hash wire form REQUIRES the `canonical:{hashalg}:{hex}` tag — bare hex is REJECTED because Registry `federation_keys.key_id` is itself `hex(sha256(pubkey))` (lowercase 64-hex), format-indistinguishable from a bare canonical-hash; NodeCore's invented tag form blessed verbatim. **CONFIRM-3**: rule-(3) `delegates_to` proxy (ongoing un-enrolled-subject revocation) vs `canonical_binding` (retroactive identity claim promoting canonical-hash → real key_id) pinned as distinct mechanisms at [§4.2.2.2](04_envelope.md); `canonical_binding` is NOT a new admission rule (composes from `delegates_to`). **CONFIRM-4**: `subject_kind` is a payload-level discriminator, not an envelope field ([§4.2.2.3](04_envelope.md)). **CONFIRM-5**: bilateral ratification confirmed consumer-policy, not registry-normative ([§4.2.2.4](04_envelope.md)). |
| 0.10 — Delivery axis (observer-share + streaming multicast as the missing third envelope axis alongside visibility + revocability) | **CLOSED-OBSERVER-SHARE / STAGED-STREAMING in CEG 0.10** by [§10.5](10_endpoints.md) (new 7-sub-section endpoint group), [§4](04_envelope.md) three new optional envelope fields (`delivery_mode` / `listed` / `history_on_join`), [§7.9](07_reserved.md) `delivery_receipt:{stream_id}` reserved prefix, [§8.1.13.7](08_composition.md) Policy M delivery extension, [§4.2.4](04_envelope.md) 3-axis orthogonality update. **Observer-share (N=1) ships impl-live**; subscriber-set = `community` per Policy M; per-subscriber `key_grant`; no `stream_id`. **Streaming multicast (N>1) ships SPEC-NOW with substrate-pending impl markers** at §10.5.2 / §10.5.3 / §10.5.4 — best-effort tier pending [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142); accountable tier additionally pending [CIRISRegistry#34](https://github.com/CIRISAI/CIRISRegistry/issues/34) STH consistency-proof enforcement. Open coupling caveat at the Persist constraint layer (RC1-1c): V054 cross-column CHECK requires content-addressed `key_grant`s; the §10.5.3 epoch axis needs a parallel CHECK arm migration — bounded, not pure-additive. RC1-7 operational ratification of constants (K=64 / T=2s / cosign per-epoch / MAX_CHUNKS_PER_EPOCH=2²⁴ + Policy E accountable quorum) pending. `rotation_chain` hygiene corrections folded in: [§5.6.8.4](05_namespace.md) disambiguation note, [§1.4](01_foundation.md) path-8 wording, [§11.7.1](11_governance.md) Option B framing, [§5.6.8.9](05_namespace.md) family-cessation wording. Absorbed [CIRISRegistry#44](https://github.com/CIRISAI/CIRISRegistry/issues/44) (closing as superseded). Tenth independent path on the 1+4 minimal-and-adequate claim. |

## §15.2 Acknowledged risks (named as bets)

| Risk | What's bet |
|---|---|
| **R1** — Governance-subject truth-grounding fidelity | NodeCore P6 acknowledges low-fidelity signals for governance subjects. Bet that earned-Credits-weighting still outperforms token-weighting at scale. |
| **R2** — `delegates_to` rename-chain adoption cost | First test was the `correlated_action_v{N+1}:from:emergent_deception_v{N}` chain at RATCHET deployment. |
| **R3** — "Log existence ≠ log monitoring" drift toward TOFU caching | Consumer-policy guidance in `docs/TRUST_CONTRACT.md`. |
| **R4** — Self-attestation under Ubuntu commitment | `witness_relation: self` admissible; consumer policy responsible for appropriate weighting per [§13.5](13_anti_patterns.md) discipline. |
| **R5** — `hardware_class` self-assertion vs cryptographic attestation | Per [§9.4.1](09_humanity_accord.md): no normative attestation-chain verification in 0.1. Bet that placeholder/dev-class rejection + trust-multipliers cover the deployment window until per-platform attestation chains land in 1.x. |
| **R6** — `occurrence_id` / `occurrence_count` / `occurrence_role` self-assertion | Per [§4](04_envelope.md): env-var-driven, no cryptographic fleet-attestation primitive in 0.1. Bet that downstream compliance reviewers can correlate via correlated `signed_at` clusters + `evidence_refs[]` cross-checks; first incident drives a fleet-attestation primitive design workshop. |
| **R7** — Frickerian discipline ([§8.3](08_composition.md)) vocabulary without full method | First-pass shallow Frickerian SHOULD-rules; bet that the structural safeguards ([§5.6.3](05_namespace.md) testimonial_witness disciplines, never-sole-evidence-for-slashing) absorb the gap until a deeper hermeneutical-resource analysis lands as a workshop output. |
| **R8** — Conceptual scope vs governable surface | By 0.14 one grammar spans identity, communities, consent, location, communications, streaming, payments, governance, constitutional mechanisms, addressing, and transparency logs. Historically, projects unifying that many layers fail when one layer dominates the others; the harder risk is *governability* — can a human amendment body ([§11.2](11_governance.md)) steward a system of this breadth? **Bet**: structural minimalism keeps the *amendable structural surface* tiny even as the namespace grows ([§1.4](01_foundation.md) 1+4), and the strict primitive/namespace/composition/verdict separation ([§1.3](01_foundation.md)) means scope grows in the *open-vocab namespace* (locally evolvable) rather than the *governed core*. **Residual**: namespace + composition-policy sprawl can still outrun review capacity; mitigation is the [§11.2](11_governance.md) high evidentiary bar + the post-1.0 candidate backlog ([CIRISRegistry#51](https://github.com/CIRISAI/CIRISRegistry/issues/51)). The remaining challenge is no longer purely technical. |

## §15.3 First-adopter exposures (no prior validation; explicit bets)

| Exposure | Why no precedent |
|---|---|
| **F1** — Earned-Credits federation governance at scale | No prior system separates earned standing from purchasable token at scale. Risk: SPKI/SDSI adoption-gap failure mode. Mitigation: licensure forcing function. |
| **F2** — Ubuntu substrate as wire-format substrate | CARE Principles + African philosophy exist as ethical frameworks; never as protocol substrate. First-adopter risk on how the discipline interacts with engineering trade-offs at scale. |

## §15.4 Deferred to 0.2+ design workshops

| Item | Why deferred |
|---|---|
| Canonical-bytes redesign (TupleHash128 + domain-separation labels) | Phase B 0.2 commitment per CEG 0.1 review |
| Per-platform hardware-attestation chain verification (TPM quote, Apple attestation, FIDO attestation) | Phase D 1.x roadmap per R5 |
| Multi-party witness directory admission (2-of-3 steward sign-off) | Phase C 0.2 commitment per [§10.3](10_endpoints.md) |
| Machine-readable namespace manifest (`FSD/CEG/dimensions.json`) | Phase E 0.2 commitment per [§12.4](12_translation.md) |
| Full OpenAPI export for all endpoints | Phase E 0.2 commitment per [§10.4](10_endpoints.md) |
| `attestation:singular_witness:non_substitutability` | T2 fragility — "non-substitutability" must reference audit-chain count, not moral quality. Needs design workshop. |
| `integrity:finitude_acknowledgment` | LOW priority; `conscience:epistemic_humility` already covers epistemic finitude. |
| `sustained_practice:{kind}` | Conceptually interesting; not load-bearing for current federation work. |
| IEEE EAD Ch5 Affective Computing cluster | Need RATCHET calibration design before T2 gate clears. |
| Various `partner_role:*` specializations | Cross-source design discussion needed. |
| 5 ergonomic considerations from trio Phase 4 audit | Bigger workshop topics (B.3 deontic-strength axis is highest-leverage). |
| SEED_DIMENSIONS RFC (CIRISRegistry#22) | RFC stage; needs discussion. |
| Fleet-attestation primitive (closes R6 occurrence_id self-assertion) | Workshop output |
| Deeper Frickerian instantiation (closes R7) | Workshop output |

## §15.5 Identified overlaps

| Overlap | Resolution |
|---|---|
| **O1** — `epistemic_mode: derivative` ≈ `witness_relation: derived` at edges | Documented as joint-usage pattern; not collapsed. Different concepts at the edges (process vs relational position) even if they often co-vary. |
| **O2** — `detection:distributive:access` could fold into `detection:correlated_action` as axis path | Kept separate for pedagogical weight; possible future revisit. |
| **O3** — `credits:*:substrate_building` was miscounted as new prefix | CORRECTED — recounted as `{subject}` value. |
| **O4** — [§8.1](08_composition.md) reference policy structure (A/B/C base + D/E/F/G/H modifiers) | Cosmetic restructuring; documented inline. |

## §15.6 CEG 0.10 / RC1 — observer-share + streaming multicast (NORMATIVE-LANDED; streaming half substrate-pending)

**STATUS UPDATE 2026-06-03**: CEG 0.10 normatively landed on main (commit folds `DRAFT_0.10_delivery_axis.md` into [§4](04_envelope.md) / [§7.9](07_reserved.md) / [§8.1.13.7](08_composition.md) / [§10.5](10_endpoints.md) + version bump 0.9 → 0.10 + lineage). All §15.6.2 decisions ratified into normative spec text. RC1-1b ✅ confirmed (`KEY_GRANT_V1_INFO` at `CIRISVerify/src/ciris-crypto/src/key_grant.rs:71` as `b"cewp-key-grant/v1"`). Open coupling caveat RC1-1c (V054 parallel-CHECK migration) flagged in §10.5.3 normative text. RC1-7 (operational constants) flagged as TODO in §10.5.1 — operator-tunable; not blocking normative ship.

The §15.6 decision register below is preserved as RC1 audit history.

## §15.6 (legacy decision register — preserved as RC1 audit history)

Decision register for the RC1 delivery-axis fork. CEG today has a **visibility** axis (`cohort_scope`) and a **revocability** axis (`subject_key_ids`, [§4.2](04_envelope.md)) but **no delivery axis** — who actively *receives* + how the substrate fans out. Observer-share (1:1, [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857)) and media/streaming multicast (1:N, absorbs the parked [#44](https://github.com/CIRISAI/CIRISRegistry/issues/44) CEG 0.5 `live_stream`) are the **same primitive at different cardinality**. Cross-team design thread (P1–P4 Persist / V1–V3 Verify / E1–E4 Edge) consolidated here because it is the CEG authority; Edge mirrors its half to its own `FSD/OPEN_QUESTIONS.md`.

**Framing lock (§3 untouched):** §3 is exactly five primitives (`scores` + `delegates_to` / `supersedes` / `withdraws` / `recants`). 0.10 is **NOT a grammar change** — it lands as `delivery_mode:{pull|push}` envelope flag ([§4](04_envelope.md)) + a [§8.1.13 Policy M](08_composition.md) delivery extension + a **new [§10](10_endpoints.md) endpoint section** (proposed `§10.5`; see OQ-RC1-2). `holds_bytes` / `key_grant` / `live_stream` / `has_chunk` are §10/transport-or-nonexistent, not grammar.

### §15.6.1 Bifurcation — DECIDED (Option A, router-confirmed)

| Half | Rides | RC1 status |
|---|---|---|
| Observer-share / directed delivery (single Contribution → subscriber-set; **no `stream_id`**) | community roster ([§8.1.13](08_composition.md) Policy M) + `key_grant` ([§5.6.8.4](05_namespace.md) / [§10.1.4](10_endpoints.md)) — **both shipping** | **impl RC1-live** |
| Media/streaming multicast (`live_stream` chunk-DAG, per-`(stream_id, epoch)` keys) | greenfield substrate (`put_blob_chunk` / `seal_stream` / stream-chunk table — **0 occurrences today**) | **spec now, impl substrate-pending [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142)** |

### §15.6.2 Locked cross-team decisions (grounded)

| Tag | Locked decision | Anchor |
|---|---|---|
| Subscription model | No new subject_kind — subscriber-set = `community` (Policy M) admitted `producer_gated\|open`; only new wire bit is `delivery_mode:{pull\|push}`. Inherits revocation/consensus/structural-invisibility free. | [§8.1.13](08_composition.md) + [§4](04_envelope.md) |
| D1 roster visibility | Substrate-private default; per-membership `listed:public` opt-in (mirrors [§11.8.3](11_governance.md)); never globally enumerable. | producer-side refusal (Edge `cohort_scope` v0.19.1 / #48-A) |
| D2 rekey (long pole) | Stream-epoch DEK seals content **O(1)**; per-subscriber `key_grant` cascade distributes the 32-byte epoch key **O(N)/epoch** (sender-key/Megolm). MLS O(log N) tree = **1.x**, additive (tree-position on the opaque grant payload — no table migration). | [§5.6.8.4](05_namespace.md) + [§8.1.12.4](08_composition.md) |
| D3 epoch triggers | Removal → **mandatory rotation** (= the forward-only-unsubscribe enforcement); add → no rotation + Option-A catch-up; time/bytes → optional. FS = **forward-only, no PCS**. New `history_on_join:{full\|from_join}`. Epoch index **greenfield, per `stream_id`** — NOT `rotation_chain`. | [§11.7.1](11_governance.md) Option A |
| D4 chunk integrity | **Per-stream transparency-log instance** (`log_id = stream_id`), RFC 6962 reused; per-leaf root-after-K verified against nearest signed STH ≥ K. NOT a hash-chain. Stream-log MUST NOT be the federation provenance log. | [§10.3](10_endpoints.md) |
| V1 stream-root | Producer signs the STH (**mandatory** authenticity root); witness cosign **optional** = the best-effort/accountable split; [§10.3.1](10_endpoints.md) consistency-proof (#34 enforcement) = anti-equivocation. Cadence K/T at epoch boundary + `sealed_at`; cosign per-epoch. | [§10.3](10_endpoints.md) / [§10.3.1](10_endpoints.md) |
| V2 nonce | 12B = **7B HKDF-derived prefix ‖ 4B BE counter ‖ 1B last-flag**; prefix `= HKDF(epoch_dek; "ciris-stream-nonce/v1" ‖ stream_id ‖ epoch)`, derived not transmitted; forced epoch-roll before 2³² wrap; cross-epoch reset nonce-safe (DEK changes). | — |
| D5 / V3 receipts | Best-effort default; opt-in signed `delivery_receipt:{stream_id}` (**new [§7](07_reserved.md) reserved prefix**), **validated-not-adjudicated** (MISSION fail-honest). Verify check is a JOIN: sig + `chunk_root` is a **real published STH root** (+ inclusion proof for accountable). **Proof-of-delivery, NOT proof-of-consumption.** | [§7](07_reserved.md) + [§10.3](10_endpoints.md) |
| D6 liveness | Two sets: **entitlement roster** (Persist; signed CEG, edge-propagated, durable, logged) vs **live-reachability set** (**Edge-owned via `reachability.rs` / #29**; node-local, TTL sec/min, **NEVER an attestation / `holds_bytes` / logged**). **Fan-out = entitled ∧ reachable.** | [§10.1.2](10_endpoints.md) TTL; Edge `reachability.rs` #29 |
| P3 scale | Flat-cascade ships RC1; **ship the P4 operator cascade-bound WITH the cascade** (else 10⁶ grant Contributions/rekey). Roster shape doesn't preclude the 1.x tree. | — |
| P4 catch-up | Bound = `min(operator depth cap [Lens-core knob, NOT a substrate constant], chunk-eviction horizon)`. Three distinct windows (chunk-eviction ≠ `holds_bytes` 24h TTL ≠ grant durability). Catch-up over an evicted epoch returns **`ContentMiss` — fail-honest, no silent gap**. | [src/retention] (Persist) |

### §15.6.3 Open items — BLOCKING normative 0.10 text

| OQ | Open item | Owner | Gating |
|---|---|---|---|
| **RC1-1** | ✅ **RESOLVED (Persist, on record)** — V054 = two single-column partial indexes (`media_content_sha256`, `key_grant_recipient_key_id`), planner-AND'd; `rotation_chain` is a payload-level [§5.6.8.4](05_namespace.md) supersession lineage (not a column/index), walked reader-side. **Separate addressing axis** from the §10.5.3 `(stream_id, epoch)` epoch-key; **shared payload-level supersession**. Wire-invisible. | Persist | done |
| **RC1-1c** | ⚠️ **Coupling caveat** — the V054 cross-column CHECK requires content-addressed `key_grant`s; the §10.5.3 epoch axis needs a **parallel CHECK arm** (content- OR stream/epoch-addressed) — a bounded constraint migration, **not a pure index-add**. Recorded so 0.10 doesn't claim "purely additive" at the Persist constraint layer. | Persist (@ #142) | flagged |
| **RC1-1b** | Confirm the `KEY_GRANT_V1_INFO` versioned-context HKDF pattern exists in `key_grant.rs` (the §10.5.2 V2 nonce-prefix derivation reuses it). Unverifiable from Edge. *(Still owed.)* | Persist | 🔴 V2 |
| **RC1-2** | ✅ **RESOLVED (2026-06-01)** — `§10.5` "Streaming transport, per-stream logs & delivery receipts" ratified as the streaming-clause home. *`§10.1.5` does not exist; corrected §15.6.4.* | Registry / router | done |
| **RC1-3** | ✅ **RESOLVED** — E1: transit-key is a **hop-by-hop wrap UNDER the E2E epoch DEK** (two layers), not replacing the cascade. | Edge | done |
| **RC1-4** | ✅ **RESOLVED** — E2: RC1 multicast = **pull-only**; relay/fan-out tree → 1.x (#46/#43). | Edge | done |
| **RC1-5** | ✅ **RESOLVED** — E3: fan-out = **entitled ∧ reachable**. Edge owns transport-reachability (`reachability.rs`, #29); Persist owns durable entitlement. *Supersedes the earlier "Persist holds the live set" — Edge already owns a liveness substrate.* | Edge | done |
| **RC1-6** | ✅ **RESOLVED** — E4: durable entitlement rides the existing federation-attestation edge path ([#41](https://github.com/CIRISAI/CIRISRegistry/issues/41) cutover); no net-new Edge transport. | Edge | done |
| **RC1-7** | Ratify constants (K=64 / T=2s / cosign per-epoch / `MAX_CHUNKS_PER_EPOCH=2²⁴`) + accountable-stream quorum = Policy E ([§8.1.5](08_composition.md) locality-scaled, not fixed N). | router | — |

### §15.6.4 Grounding corrections (recorded so phantom citations do not re-propagate)

| Claimed in the design thread | Corrected against sources |
|---|---|
| "§10.1.5 Merkle" as an existing anchor | **§10.1.5 does not exist.** §10 = `.0/.0.1/.1/.1.1/.1.2/.1.4/.1.3/.2/.3/.3.1/.4`. Streaming = new **§10.5** (proposed; OQ-RC1-2). |
| MISSION:66 / :148 | Misquoted. Fail-honest doctrine at MISSION **182/197/414/447 + AV-42**; ContentFetch/Body/Miss at **223** (#21). |
| `cohort_scope::suppresses_holds_bytes` symbol; "3.9.0/3.9.2" | Not a symbol — **producer-side refusal at `Edge::send_*`**; **v0.19.1 / #48-A** (crate now v1.1.5). |
| "DEK cascade rides `key_grant.rotation_chain`" ([§11.7.1](11_governance.md) + 0.7 lineage) | `rotation_chain` = **grant-supersession lineage** ([§5.6.8.4](05_namespace.md)), not key rotation; epoch rotation is greenfield per `stream_id`. → §11.7.1 + [§1.4](01_foundation.md) path-8 + [§5.6.8.9](05_namespace.md) + [§16.1](16_references.md) 0.7 lineage need a **hygiene edit folded into 0.10**. |
| Streaming as a grammar change | §3 = exactly five primitives, untouched; 0.10 = §4 flag + §8.1.13 ext + new §10 section. |

### §15.6.5 Status

Verify ✅ done (5 original + V1–V3). **Edge ✅ closed** (E1 two-layer transit / E2 pull-only RC1 / E3 entitled∧reachable via `reachability.rs` #29 / E4 existing federation-attestation path; recorded in Edge `FSD/CEG_RC1_BIFURCATION.md` + `FSD/OPEN_QUESTIONS.md` OQ-14..17). **Persist:** P1–P4 done; **RC1-1 ✅ resolved** (V054 separate addressing axis, shared payload-level supersession), **RC1-1b** (`KEY_GRANT_V1_INFO`) still owed, **RC1-1c** CHECK parallel-arm coupling flagged. Router: **RC1-2 ✅ §10.5 ratified**; **RC1-7** (constants) pending. Option A + #44 absorption confirmed.

**Observer-share half — ZERO remaining blockers, normative-ready now** (rides existing community roster [§8.1.13](08_composition.md) + `key_grant`; needs no Persist confirm, no #142, no RC1-7). **Streaming half** final blockers: **RC1-1b** (Persist) + **RC1-7** (router constants) + the **RC1-1c** V054 CHECK parallel-arm; impl **greenfield-blocked on #142 step 3** (`stream_id`, v3.9 target — **unowned/unscheduled**, roadmap call), accountable tier additionally on **[#34](https://github.com/CIRISAI/CIRISRegistry/issues/34)**. The 0.10 skeleton is staged at [`DRAFT_0.10_delivery_axis.md`](DRAFT_0.10_delivery_axis.md); §15.6.4 `rotation_chain` hygiene corrections fold into the weave.

---

[← §14 Glossaries](14_glossaries.md) | **§15 Gaps** | [Next: §16 References →](16_references.md)
