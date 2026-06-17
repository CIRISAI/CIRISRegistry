[← §18 Interop](18_interop.md) | **§19 Holonomic substrate** | [README](README.md)

---

# §19 Holonomic substrate — ALM, fountain storage, WholenessWitness, recursive bootstrap (CEG 1.0-RC11)

The CIRISEdge v4.0.0 "holonomic substrate" gives the federation **graceful degradation** (any subset of fountain symbols decodes at proportional fidelity) and **graceful reconstitution** (the witnessed corpus re-establishes from any sufficient fragment). This section absorbs its wire shapes as additive normative CEG sections **with normative guardrails** that bind them to CEG's existing trust, PQC, consent, replication, and anonymous-tier invariants.

> **Absorb-with-guardrails, not verbatim (normative posture, [CIRISRegistry#85](https://github.com/CIRISAI/CIRISRegistry/issues/85)).** A four-perspective review found the holonomic concept sound and **additive at the wire layer — no [§3](03_primitives.md)/[§4](04_envelope.md) 1+4 change** — but that absorbing the v4.0.0 implementation *verbatim* would invert several ratified CEG invariants (owner-binding, PQC-mandatory, consent/withdraws, quorum-merge). This section therefore states the **guardrail invariants** (§19.1–§19.5) as normative MUSTs **now**; the matching edge implementation fixes are tracked at [CIRISEdge#143](https://github.com/CIRISAI/CIRISEdge/issues/143) (in-flight hardening). The **byte-exact signed preimages** for each shape are pinned against the **fixed** v4.0.x reference impl (several shapes change under #143 — e.g. `SignedClaim` gains owner-binding fields), and **conformance vectors generated from that fixed impl are the named [#57](https://github.com/CIRISAI/CIRISRegistry/issues/57) freeze gate** ([§19.6](#196-conformance--the-57-freeze-gate)).

## §19.0 Canonicalization boundary + the 1+4 line (normative)

- **The frozen 1+4 attestation envelope ([§4](04_envelope.md)) is untouched.** Every §19 object is **transport/substrate framing** — it never instantiates a §4 Contribution, never adds an `attestation_type`, never enters §0.9 JCS canonicalization. The already-landed realtime A/V chunk wire ([§10.5.8.3–.5](10_endpoints.md)) is the same category.
- **§19 uses a binary, length-prefixed, big-endian, domain-separated signing preimage — NOT [§0.9](00_conformance.md) JCS.** These are verify-to-verify transport primitives that never cross the four-impl boundary as JSON (the same boundary [§10.1.5.3](10_endpoints.md) drew for Verify's `signing_bytes` framing). An implementer MUST NOT apply JCS to a §19 object or its signatures will not verify cross-impl. Each object's domain separator (`b"CIRISALM-CAPv2\0\0"`, `b"ciris-edge/holding-claim/v1"`, `b"ciris-edge/compress-request/v1"`, `b"CIRIS-CLAIM-v1\0\0"`, the WholenessWitness `b"WW-v1-empty"` empty-sentinel, etc.) is pinned by its subsection.
- **PQC-mandatory ([§10.1.5.1.1](10_endpoints.md)) binds every §19 signed object.** Each carries the bound hybrid pair (Ed25519 over the canonical preimage; ML-DSA-65 over `preimage ‖ ed25519_sig`); a verifier MUST reject a §19 object lacking a valid ML-DSA-65 half **at ingest and before persistence** (no store-then-quarantine — the RC8 store-path rule). Verification happens **at the gate**: an admission/verdict function MUST verify signatures itself and MUST NOT trust an in-band `verified` flag (such a flag MUST be non-wire / `serde(skip)`).
- **What is wire vs internal (the [§1.1](01_foundation.md) line).** Cross-impl-observable bytes (signed preimages, content-addressed hashes, and the deterministic topology output) are **PIN-NORMATIVE**. Local heuristics whose output no other peer reproduces are **edge-internal** and MUST NOT be over-pinned: specifically **ALM parent-selection** (`AlmJoinPlanner` — over per-peer RTT/reachability) and **`retention_priority`** (never on the wire) are edge-internal; **rarity scoring** is a **recommendation**, not a MUST.

## §19.1 WholenessWitness (§W) — divergence-detection witness

A `wholeness_witness:` object is a peer's **hybrid-signed Merkle root over a scoped projection of the claims it holds**, used to detect cross-peer/cross-region state divergence and to drive reconciliation.

**Namespace + scope (normative).**
- **WW-naming** — the namespace is **`wholeness_witness:`**, never bare `witness:`. It is a *self-published state-root snapshot*, the **inverse** of the [§10.3](10_endpoints.md) transparency-log "witness" (an independent STH cosigner). A WholenessWitness does **not** provide append-only / consistency / anti-equivocation guarantees and MUST NOT be substituted for §10.3.1 or the [§10.5.1](10_endpoints.md) per-stream STH.
- **WW-1** — the root MUST cover **only** the namespaces in the object's `claim_namespaces` field. A conformant peer is **not** required to witness everything it holds; coverage is per-namespace opt-in.
- **WW-2 (anonymous/self exclusion — fail-secure)** — a WholenessWitness MUST NOT include anonymous-tier records ([CIRISNodeCore#22](https://github.com/CIRISAI/CIRISNodeCore/issues/22)) or `cohort_scope: self` local-tier rows ([§10.1.5](10_endpoints.md)) as Merkle leaves, and `claim_namespaces` MUST NOT name such a namespace. Witnessing them would re-attribute deniable / self-private content to a stable `peer_id`. (Edge#143: the leaf-walk MUST filter these out before computing the root.)
- **WW-3** — `cohort_scope: family | community` content MAY be witnessed **only at the opaque `content_id`/manifest-digest grain**, never at a grain disclosing membership, plaintext, or `subject_key_ids` ([§10.1.4](10_endpoints.md) confidentiality preserved).

**Construction (normative).**
- **Leaf order MUST be lexicographic** over leaf bytes (the [§0.9.2.1](00_conformance.md) set-semantics rule). The v4.0.0 "either order as long as both peers agree" is **non-conformant** — it is the §0.9-class divergence hazard.
- The Merkle scheme is `leaf = SHA-256(leaf_bytes)`, `node = SHA-256(left ‖ right)`, odd-node duplication, `b"WW-v1-empty"` empty sentinel — the construction CIRISEdge v4.0.x shipped and CIRISVerify v5.9.0 proved cross-impl. **Frozen as-is (1.0-RC15 — resolves the prior open `#143` question): CEG does NOT adopt the RFC 6962 `0x00`/`0x01` leaf/node prefix here.** Rationale — the CVE-2012-2459 odd-node-duplication malleability is **not exploitable in this construction's uses**: (1) every WholenessWitness and `member_commitment` ([§19.7.1.1](#19711-member_commitment-descent-integrity)) root is **mandatorily hybrid-signed** — no consumer ever relies on an *unsigned* root; (2) `member_commitment` is verified by **recomputation from the full source-id list**, never by partial inclusion proofs against the bare root (malleability moot); (3) the §19.1 reconciliation Merkle-proof exchange (N4) is between **accountable, signed, equivocation-checked peers** — not third-party forgery of an untrusted root. **Caveat (normative):** any *future* use that relies on an **unsigned** root, or verifies **partial inclusion proofs against an untrusted root**, MUST first adopt the RFC 6962 `0x00`/`0x01` prefix + lone-node promotion (and re-cut the vectors). This is a **distinct construction from the [§10.3](10_endpoints.md) RFC 6962 log** (different algorithm + leaf domain); the two MUST NOT be cross-verified. Changing this scheme is a vector-invalidating wire change — not an editorial tweak.

**Authority (normative).**
- **N3** — a WholenessWitness is a federation-tier attestation: hybrid PQC verified at ingest **and before** persistence to the witness corpus ([§19.0](#190-canonicalization-boundary--the-14-line-normative)); `compare_witnesses` MUST NOT run on an unverified witness.
- **N4 (equivocation)** — two validly-signed witnesses from the same `(peer_id, epoch_id, claim_namespace_set)` with different `merkle_root` are **non-repudiable equivocation proof**; the substrate MUST retain and surface them as a `hard_case:*` ([§7](07_reserved.md) reserved-prefix candidate), never silently reconcile. Per-peer `epoch_id` MUST be anti-rollback-checked before `EpochBehind` is used as a reconciliation input (eclipse guard). Full cross-witness BFT MAY be deferred ([§15](15_gaps.md) named bet) — but observed equivocation MUST NOT be discarded.
- **WW vs replication (the highest-value reconciliation)** — a WholenessWitness is a **divergence detector that *triggers*** the [§10.1.6](10_endpoints.md) quorum-merge; it does **NOT** decide a merge and MUST NOT replace `monotonic_quorum` / `revision` anti-rollback for `revocation` / `partner_record` / `org_membership`. A `Divergent` verdict on those subject_kinds hands the decision to the §10.1.6 R1/Q1 quorum-merge (quorum-ordered, anti-rollback) — otherwise a "reconstitute from any fragment" path could resurrect a revoked key (rollback).

## §19.2 Recursive trust bootstrap (§B) — trust-discovery, not membership

`recursive_trust_bootstrap(SignedClaim, TrustGraph, WitnessChain) → verdict` lets a peer discover transitive trust by walking a signed witness chain to a root in its own trust graph. **It is reachability discovery beneath CEG's authority layer, not an admission shortcut.**

- **N1 (trust ≠ membership)** — a successful chain walk yields **trust+serve standing only** ([§5.6.8.10](05_namespace.md) TRUST≠MEMBERSHIP). Admission to any **non-`infrastructure`** community remains gated, at the destination, by (a) the §5.6.8.10 **owner-binding** precondition (a live `user`-owner `delegates_to`, an admitted `identity_occurrence`) and (b) that community's `consensus_protocol`. `infrastructure` roots stay **founder-quorum**-gated; a transitive chain MUST NOT satisfy founder-quorum. (Edge#143: `SignedClaim` must carry the owner-binding fields so the gate is expressible.)
- **N2 (self-supplied chains aren't evidence)** — the chain-length budget MUST be ≤ the [§13.3](13_anti_patterns.md) **5-hop cap**, trust-graph **cycles MUST be rejected** (§13.3), and the §13.3 **aggregate-weight cap** (default 0.5 × root_trust) MUST bound the standing one root confers transitively. A caller-supplied chain proves only its signatures, not a real lineage.
- **RB-1 (anonymous coexistence)** — anonymous-tier content MUST be ingestible / retainable / serveable with **no trust-graph position**; `recursive_trust_bootstrap` MUST NOT be required for, or invoked on, anonymous records.

## §19.3 Fountain storage + swarm rarity (§P / §R)

Content is RaptorQ-coded into `N` source + `K` repair symbols (`FountainManifestV1` / `FountainSymbolV1`); peers retain symbols and coordinate rarest-first so content survives churn.

- **Holder directory (no duplication)** — a `FountainHoldingClaim` ("peer X holds symbols S of content C") is a **specialization of `holds_bytes:sha256:*`** ([§10.1.2](10_endpoints.md) / [§8.1.13.3](08_composition.md)), reusing its TTL + `ContentMiss` feedback. It MUST NOT create a second who-holds-what directory. It **inherits the [§10.1.4](10_endpoints.md) `cohort_scope: self | family` suppression** — no holding claim is emitted for self/family content (else fountain claims leak the existence of structurally-invisible blobs).
- **N5 (retention respects revocation — fail-secure)** — retention MUST NOT keep alive, **above the [§19.7](#197-the-noise-floor--unified-retirement--forever-memory-model-normative) noise floor**, content whose consent is withdrawn ([§3.2.3](03_primitives.md)) or revoked. A withdrawn `content_id` is descent-eligible **regardless of rarity**; an active `withdraws` / `consent:state:revoked` overrides the max-rarity "keep" signal and forces immediate descent below the noise floor (the fastest form of the one retirement operation — see §19.7); unknown consent state defaults to *not retained as rare*. The [§8.1.11.3](08_composition.md) deletion-SLA + [§8.1.11.5](08_composition.md) decay stages take precedence over swarm coverage at all times. **Revocation does not require destroying the collective gist** — only that the item be **not individually recoverable** at any retained tier (§19.7); it MUST purge any tier where it still is.
- **N6 (possession-bound claims)** — a `FountainHoldingClaim` counted toward rarity MUST be possession-challengeable (a holder answers a symbol request, or the claim carries a proof-of-possession). Unverified holding claims MUST NOT lower another peer's retention priority — otherwise rarity is a forgeable force-evict channel.
- **N7 (symbol integrity)** — reconstruction MUST verify each symbol against the manifest's signed per-symbol SHA-256 (a swarm-sourced symbol cannot poison a decode).
- **SR-2 / SR-3 (anonymous + reconstitution scope)** — anonymous-tier content is **exempt from swarm-mandatory retention** (governed by LRU-only per NodeCore#22): no `FountainHoldingClaim` / `FountainCompressRequest`, no rarest-first biasing. The "reconstitutes from any sufficient fragment" property is a property of the **witnessed, trust-anchored** corpus **only** — it does not extend to anonymous content, which the substrate MUST be able to let truly disappear.
- **PIN line** — `FountainHoldingClaim` / `FountainCompressRequest` signed preimages = **PIN-NORMATIVE** (with `symbol_ids` sorted ascending before signing); `compute_rarity_score` = **PIN-AS-RECOMMENDATION**; `retention_priority` = **edge-internal** (never on the wire).

### §19.3.1 Replication-target policy (§R-policy — normative floor + RECOMMENDED defaults, 1.0-RC26 — resolves [CIRISRegistry#86](https://github.com/CIRISAI/CIRISRegistry/issues/86))

The fountain `(N, K, target_holders, min_viable)` parameterization a producer chooses is **producer-set, not fixed by the substrate** — but two clauses are **normative**, and the default tuple a conformant peer assumes when the producer is silent is **pinned RECOMMENDED** (so two impls converge on the same survivability floor rather than diverging silently).

**Normative.** A CEWP-1.0 conformant peer:
- MUST set `min_viable_symbols >= 1` — the [§19.7](#197-the-noise-floor--unified-retirement--forever-memory-model-normative) EnvelopeOnly tier is locked at the substrate (below `min_viable`, only the signed envelope survives — never zero, never an unbounded floor of 0).
- MUST be able to participate in fountain content at **any** `(N, K, target_holders)` parameterization a trust island it joins publishes — a peer MUST NOT hard-code one tuple and refuse others. The defaults below bind only the *producer-silent* case.

**RECOMMENDED default policy (informative — the producer-silent tuple).**

```
DEFAULT_N_SOURCE       = 20   // source symbols (lossless threshold)
DEFAULT_K_REPAIR       =  6   // FEC headroom, ~30% over N (RFC 6330 overhead profile)
DEFAULT_MIN_VIABLE     =  5   // N/4 BLINKING_DOT floor; below this → EnvelopeOnly
DEFAULT_TARGET_HOLDERS = 30   // distinct peers holding ≥1 symbol
```

**Derivation (informative — three independent constraints, max-binds).** `target_holders >= max(C_1, C_2, C_3)`:
- **C_1 survival floor (dominant) = 26.** With `N+K` symbols spread 1-per-peer over `R = target_holders` peers at per-peer fetch availability `q`, reconstruction needs `>= N` symbols reachable (binomial `P(X >= N)`). The design target is **99.95% reconstruction at q=0.85** (typical wifi / community-mesh churn): at N=20, K=6 this binds `R >= 26` (mean 25.5 reachable at R=30). Datacenter q=0.95 → 0.99996; high-churn q=0.80 → 0.974.
- **C_2 demand-spike capacity = 7 (not binding).** ALM at fanout 12 ([§19.4](#194-deterministic-alm-topology-t--m), 720p30/30Mbps interior-LAN budget) serves 157 viewers/copy at depth 2; 5 copies × depth-2 = 785 simultaneous. Demand binds only when content is **cold AND suddenly viral** — and the [CIRISEdge#134](https://github.com/CIRISAI/CIRISEdge/issues/134) swarm-rarity layer elevates copy count organically.
- **C_3 locality reach = 10.** Per the CEWP locality dividend, each populated locality serves LAN-internally; inter-locality is signed-claim bridge, not synchronous relay. C_3 = 10 for a typical 10-locality mission deployment.
- **Compose:** `max(26, 7, 10) = 26`, then `26 × 1.15` (15% churn-safety margin) `≈ 30`, rounded for human ergonomics.

**Why these and not 22 / 40 (informative).** `N=20` keeps K=6 a meaningful ~30% FEC while one-symbol-per-peer holds across a 30-peer trust island without crowding, and sits in the RaptorQ O(N²)-decode sweet spot (microsecond scale). `K=6` matches RFC 6330's empirical overhead for 99.9% decode; higher gives diminishing returns, lower drops decode below 99% at q=0.85. `min_viable=5` is the N/4 BLINKING_DOT floor. `target_holders=30` is C_1's 26 plus churn margin.

**No wire change.** §R-policy pins *defaults and a floor* over the existing [§19.3](#193-fountain-storage--swarm-rarity-p--r) `FountainManifestV1` `(N, K)` fields and `min_viable_symbols`; it introduces no new shape and no 1+4 change.

## §19.4 Deterministic ALM topology (§T / §M)

The application-layer-multicast relay tree for large-N fan-out — the [§10.5.8](10_endpoints.md) "realtime large group (SFU/relay-tree)" profile previously marked → 1.x, now filled.

- **`compute_alm_topology(snapshot) → topology` is PIN-NORMATIVE as a contract** — a **pure, deterministic, integer-only** function (no IEEE-754; no `HashMap` iteration order) over `(capacity_ads, trust_grants, reachability_observations, locality)`, with specified lexicographic tie-breaks and canonical output order, such that **byte-equal inputs yield byte-equal output** across implementations. The byte-exact output is gated on the [§19.6](#196-conformance--the-57-freeze-gate) vectors (incl. permutation-invariance cases) — **not** transcribed from the algorithm body as the source of truth.
- **N8 (capacity authenticity)** — capacity advertisements feeding the topology MUST be hybrid-verified (`SignedRelayCapacity`, domain `b"CIRISALM-CAPv2\0\0"`) **before scoring**; self-asserted `uplink_mbps` MUST NOT be the dominant, unbounded selection term (cap per owner-bound identity or make it throughput-challengeable). Determinism amplifies one capacity lie into a *universal* eclipse — "verified by an unspecified upstream tier" is not a guarantee.
- **D6 preserved** — `reachability_observations` are **ephemeral planner inputs**; they MUST NOT become attested, replicated, or witness-leafed state ([§10.5.6](10_endpoints.md) "reachability is never trust"). Resolution authority stays in [§8.1.13.1.1](08_composition.md); the topology consumes it, does not replace it. The determinism comparator reconciles to the §8.1.13.1.1 / R1/Q1 family.
- **PIN line** — the topology *function contract* + `SignedRelayCapacity` / `SubStreamCommitment` signed preimages = **PIN-NORMATIVE**; **ALM parent-selection** (`AlmJoinPlanner`, over per-peer RTT) = **edge-internal**.

## §19.5 Fail-secure summary (normative)

The holonomic mechanisms are blind to the anonymous tier (WW-2, RB-1, SR-2/3), subordinate to the consent/revocation model (N5, WW vs §10.1.6), gated by owner-binding + founder-quorum (N1, N2), and bound by PQC-mandatory verification at the gate (§19.0, N3, N8, F-5). Where the v4.0.0 implementation does not yet meet these, the gap is tracked at [CIRISEdge#143](https://github.com/CIRISAI/CIRISEdge/issues/143); the invariants here are the conformance target regardless of implementation timing.

## §19.6 Conformance — the #57 freeze gate

The byte-exact signed preimages and the `compute_alm_topology` output are pinned against the **fixed** v4.0.x reference impl, and **conformance vectors generated from it are the named [#57](https://github.com/CIRISAI/CIRISRegistry/issues/57) freeze gate** (extending [CIRISConformance#9](https://github.com/CIRISAI/CIRISConformance/issues/9)): input → expected bytes for `SealedAvChunk` + the two AV nonces, `SignedRelayCapacity`, ALM topology (input snapshot → expected tree hash, incl. permutation invariance), `FountainManifestV1`/`SymbolV1` + `retention_priority`, `FountainHoldingClaim`/`CompressRequest`, and `WholenessWitness` canonical bytes + Merkle root (incl. the empty sentinel + odd-node duplication). Until a second implementation reproduces these byte-for-byte, the §19 shapes are **pinned-but-unproven — RC-grade, not 1.0.**

## §19.7 The noise floor — unified retirement / forever-memory model (normative)

**One operation, not many.** Revocation, retirement, capacity-eviction, scheduled expiry, and natural aging are **the same operation at different rates**: a **monotonic descent of an item's fidelity, driven by pressure**, toward and below a recoverability boundary called the **noise floor**. There is no separate "hard delete" primitive — *hard-delete is the fastest descent* (forced immediately below the floor); capacity-eviction is a slow one; aging is the slowest. All are equally valid instances of the one retirement operator.

**The noise floor = the individual-recoverability boundary.** An item is **above** the floor in a retained artifact iff it can be individually reconstructed from that artifact above a fidelity ε; it is **below** the floor iff only its *contribution to a collective* survives — the item itself is information-theoretically unrecoverable. The floor does double duty and is the load-bearing normative quantity of this section: it is **both** the privacy boundary (a revoked item MUST be below it at every retained tier) **and** the durability floor (the collective blur sits below it, forever).

**Nothing is ever fully forgotten — the memory pyramid.** Descent does not terminate at zero. Two **mechanical** degradation operators (no reasoning, no agency — see below) carry it:
1. **Intra-object fade** — scalable/layered codec ([§10.5.8.4](10_endpoints.md) `ChunkLayer` spatial/temporal/quality) + RaptorQ per layer: drop high-detail symbols → a clean coarse version of the same item.
2. **Inter-object aggregation** — *a picture of a thousand pictures*: tile / downsample / statistically composite **N → 1**. Recursed, this builds a pyramid (mipmap) of history: recent strata high-resolution, ancient strata collapsed into the blur. Steady-state storage to remember **all** of history is **O(log T)** in the amount remembered, not O(T) — the N→1 fan-in makes forever-memory **sublinear**. *A million years may be a blur, but it is remembered, unbroken, to the beginning.*

**Pressure-driven (normative).** The descent rate and the pyramid's level transitions are driven by **pressure** (disk pressure, age, or an explicit force), never a fixed schedule. Pressure sources, slowest→fastest: natural aging < scheduled retirement < capacity (disk) eviction < **revocation (immediate forced descent below the floor)**.

**Forgetting and erasure converge (this dissolves the [§19.3](#193-fountain-storage--swarm-rarity-p--r) N5 tension).** The N5 erasure guarantee is exactly "not individually recoverable at or below the noise floor." A sufficiently-aggregated composite (a picture of a thousand pictures contains < 1/1000 of any source) is **already-erased by degradation** — no purge needed. Revocation simply *forces* an item below the floor **now**, and MUST purge only the retained tiers where it is **still individually recoverable** (the high-fidelity upper layers). It need not — and MUST NOT be required to — destroy the collective gist. Capacity-eviction reaches the identical end-state gradually. Same destination; revocation just gets there first.

**Infrastructure is self-sufficient for memory (sharpens [§1.3](01_foundation.md)).** Both degradation operators are **mechanical** (symbol arithmetic + resampling) — they require **no reasoning and no agency**, so a pure fabric node performs the entire forever-memory function. A brain MAY enrich a degraded tier with a richer semantic gist, but is **never required**: infrastructure remembers without agency. (Earlier drafts wrongly treated semantic summarization as the mechanism; the mechanism is mechanical degradation — the brain is optional enrichment.)

**Disposition mapping.** The [§19.3](#193-fountain-storage--swarm-rarity-p--r) `EjectionVerdict` values are points on this one axis: `Keep` = above-floor, no pressure; `EjectToTier` = a downward step (still recoverable, lower fidelity); aggregation = N→1 downward step; `EjectHardDelete` = forced descent below the floor + purge-still-recoverable-tiers. They are not distinct mechanisms — they are stops on the single pressure-driven descent.

### §19.7.1 `AggregationMetaV1` — the aggregation-tier wire contract (normative, 1.0-RC14)

The metadata that tags one tier of the §19.7 memory pyramid: which content, at what aggregation tier, over which source members, by which mechanical operator. **CEG-canonical** — unlike the [§19.6](#196-conformance--the-57-freeze-gate) shapes (transcribed from edge v4.0.x), no reference impl yet defines these bytes ([CIRISPersist](https://github.com/CIRISAI/CIRISPersist) v8.3.0's `content_aggregation` stores `aggregation_meta` **opaque** — the wire-churn firewall), so this section **defines** the byte layout; impls conform to it. Resolves [CIRISRegistry#89](https://github.com/CIRISAI/CIRISRegistry/issues/89); unblocks [CIRISVerify#79](https://github.com/CIRISAI/CIRISVerify/issues/79).

`AggregationMetaV1` is a **substrate wire shape, NOT a [§4](04_envelope.md) attestation** — no 1+4 change. Its signing preimage uses the [§19.0](#190-canonicalization-boundary--the-14-line-normative) binary discipline (length-prefixed, big-endian, domain-separated — **NOT** [§0.9](00_conformance.md) JCS). Preimage byte order (normative):

```
preimage = b"AGG-META-v1\0\0\0\0\0"          // 16-byte domain separator (exact)
         ‖ u32_be(version = 1)
         ‖ lp(content_id)                     // the root content this pyramid is for
         ‖ lp(corpus_kind)                     // "trace" | "blob" | "av_chunk" | …
         ‖ u32_be(tier)                        // 0 = source granularity; higher = more aggregated
         ‖ lp(aggregation_algorithm_id)        // opaque codec id, e.g. "raptorq-pyramid-v1"
         ‖ u32_be(source_count)                // N members aggregated into this tier (descent fan-in)
         ‖ member_commitment[32]               // §19.7.1.1 Merkle root over the source member ids
         ‖ lp(noise_floor_descriptor)          // what survives below the floor (codec-specific, canonical)
//  lp(x) = u32_be(byte_len(utf8(x))) ‖ utf8(x)     // length-prefixed UTF-8
```

`content_id`, byte-valued ids, and `member_commitment` are lowercase-hex per [§0.6](00_conformance.md) where rendered as strings; `member_commitment` on the wire is the raw 32 bytes. **Bound-hybrid signature** (the [§19.0](#190-canonicalization-boundary--the-14-line-normative) rule): `Ed25519(preimage)` + `ML-DSA-65(preimage ‖ ed25519_sig)`; a verifier MUST reject a tier lacking a valid ML-DSA-65 half **at ingest and before persistence** (the [§10.1.5.1.1](10_endpoints.md) store-path rule applies — `AggregationMetaV1` is federation-tier).

#### §19.7.1.1 `member_commitment` (descent integrity)

`member_commitment` is the Merkle root over the **source member ids aggregated into this tier**, computed by the **[§19.1](#191-wholenesswitness-divergence-detection-witness) WholenessWitness Merkle construction** (same `leaf = SHA-256(utf8(member_id))`, **lexicographic** leaf order, `node = SHA-256(left ‖ right)`, odd-node duplication, and empty-set sentinel) — reused deliberately so the federation carries **one** aggregation/witness Merkle scheme, not a third. (It therefore uses the §19.1 scheme as frozen in RC15 — **no** RFC-6962 prefix; safe here because `member_commitment` is verified by **full source-id-list recomputation**, never partial inclusion proofs, so the CVE-2012-2459 malleability is moot. See §19.1.) `member_commitment` lets any verifier confirm a tier was aggregated from exactly the claimed sources without holding the sources.

### §19.7.2 Descent rule (normative, 1.0-RC14)

`descend(content_id, corpus_kind, tier) → [member_id]` returns the **ordered** source members aggregated into the tier-`tier` composite — the tier-`(tier−1)` members one level down the pyramid. It MUST be a **pure, deterministic** function: two impls return the **byte-equal ordered list** for byte-equal inputs. The order is the **lexicographic member-id order** `member_commitment` (§19.7.1.1) committed to — so a returned list re-derives the parent's `member_commitment` byte-for-byte (the descent-integrity check).

**Descent never terminates at zero (the forever-memory floor).** Below tier 0 (source granularity) the content's **collective gist persists as the lowest retained tier** — a composite whose members are no longer *individually* recoverable (it is **below the noise floor**, §19.7) but whose blur survives. `descend` past the noise floor yields the blur, never an empty/destroyed object. The function is **pressure-independent** (pure navigation); **pressure drives which tiers are *retained*** (§19.7), not the descent computation. Ascending (aggregation, operator 2) is the N→1 inverse with fan-in `source_count`.

### §19.7.3 `EjectionVerdict` — the tier-aware retirement surface (normative, 1.0-RC14)

The single verdict surface a verifier exposes and a substrate consumes to gate one step of the §19.7 descent. CEG pins it as the canonical superset of the rarity-only `RetentionDecision` that [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) v5.9.0 already ships:

```
EjectionVerdict ::= Keep            // above the floor, no pressure step
                  | EjectToTier      // one downward step: still recoverable, lower fidelity
                                     //   (intra-object layer-drop OR N→1 aggregation)
                  | EjectAggregatedTierOnly { tier }
                                     // shed exactly one pyramid stratum — the tier-`tier`
                                     //   composite — leaving finer AND coarser tiers intact
                  | EjectHardDelete  // forced descent below the floor + purge still-recoverable tiers
```

Mapping (normative): the v5.9.0 `RetentionDecision{RetainRare|RetainNonRare|EvictEligible}` is the rarity sub-decision *within* `EjectToTier`/`Keep`; `EvictEligible` + capacity pressure → `EjectToTier`; `EvictEligible` + a `withdraws`/`consent:state:revoked` (§19.3 N5) → `EjectHardDelete` (the fastest descent, never tier-shed — §19.7). **`EjectAggregatedTierOnly { tier }`** is the tier-granular form of `EjectToTier`: it sheds a single intermediate stratum of the §19.7.1 pyramid (the tier-`tier` `AggregationMetaV1` composite) under targeted pressure, leaving both finer and coarser tiers — composing with the hard-delete trait (a `tier` below the noise floor is unreachable, so this never resurrects erased content). A pure fabric node MAY compute `EjectToTier` / `EjectAggregatedTierOnly` mechanically; `EjectHardDelete` MUST purge per §19.3 N5. Verify exposes `EjectionVerdict`; persist consumes it to drive `put_aggregated_tier` / the tier-tagged evict (EjectToTier, EjectAggregatedTierOnly) vs `evict_fountain_content_hard_delete` (EjectHardDelete).

**Conformance — PROVEN cross-impl (1.0, promoted from RC, 1.0-RC16).** §19.7.1–.3 are **byte-equivalent across implementations**: [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) v5.10.0 authored the vector family and [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) v4.3.0 (`src/holonomic/aggregation.rs` + `tests/conformance_vectors_v19_7.rs`, 5 vectors) reproduces them **byte-for-byte** — the `AggregationMetaV1` preimage + signature, `member_commitment`, and `descend` ordered output. The `AggregationMetaV1` preimage matched **on the first attempt with no cross-team coordination beyond this spec** — the [§19.0](#190-canonicalization-boundary--the-14-line-normative) binary-length-prefixed discipline makes wire-identity reproducible from the text alone. `member_commitment` reuses the [§19.1](#191-wholenesswitness-divergence-detection-witness) WholenessWitness Merkle **verbatim** (same `compute_merkle_root`, same `WW-v1-empty` sentinel) — the federation runs **one** Merkle scheme across §19.1 (witness leaves) and §19.7 (member commitments), no schema fork. With persist v8.4.0 + verify v5.10.0 + edge v4.3.0 all on the §19.7 baseline at PyPI, the [§19.6](#196-conformance--the-57-freeze-gate)/[#57](https://github.com/CIRISAI/CIRISRegistry/issues/57) vector family for §19.7 is **closed**; §19.7 is **1.0, not RC**. (The earlier v5.9.0-proven §19.6 vectors are unaffected.) The `EjectAggregatedTierOnly { tier }` verdict is the one remaining edge build item (tracked v4.3.x/v4.4) — surface-additive, composes with the existing trait.

---

[← §18 Interop](18_interop.md) | **§19 Holonomic substrate** | [README](README.md)
