# Part VI — The Coherence Mathematics

**CC decimal range** `6.x` · **14 concepts** · **page budget 2.5pp** (∝ importance) · [← master index](README.md)

> Where the substrate meets the meaning. This Part is small in pages but central in role: it is the
> single place the holonomic substrate (CEG §19 — how the federation *stores and remembers*) is bound
> to the Accord's coherence mathematics (how the federation *measures* coherence — the ratchet, the
> defence/flourishing functions `J`/`F`, the sustainability integral `σ`). The Foundation declared
> [M-1](part_1_foundation.md#11-meta-goal--meta-goal-m-1) to be *sustainable* adaptive coherence and
> promised the math behind "sustainable" lived here; the Grammar froze the bytes that ride on the
> substrate. Part VI is where "sustainable" stops being an adjective and becomes a quantity — and
> where the substrate that makes the quantity durable is specified.

> **Status:** scaffold-plus — the holonomic substrate (§19) is woven now; the Accord's **Book IX**
> formal mathematics (`J`/`F`/`σ`, the Coherent Intersection Hypothesis, the impossibility lemmas
> L-01…L-06) is named, sourced, and *connected* here, but its formulae fold in **verbatim from
> CIRISAccord 1.3-RC2 in Phase 4** — they are not in the parseable Accord copy used for this draft.
> The [Book IX gap](#62-book-ix--the-accord-coherence-mathematics-phase-4) subsection states exactly
> what is deferred and sketches the connection without fabricating the formulae.

---

## 6.1 `holonomic` — Holonomic substrate — ALM, fountain storage, WholenessWitness, recursive bootstrap
<sub>budget 0.36pp · import #83 · from **CEG §19** (CEG 1.0-RC11) · semantic id `holonomic`</sub>

The substrate underneath everything Part VI measures. The holonomic design gives the federation two
properties that the coherence mathematics depend on but do not themselves provide: **graceful
degradation** (any subset of fountain symbols decodes the content at proportional fidelity — there is
no cliff) and **graceful reconstitution** (the witnessed, trust-anchored corpus re-establishes itself
from any sufficient fragment). A federation whose memory survives the loss of any single node — and
degrades smoothly rather than vanishing — is what makes coherence *sustainable* rather than merely
asserted at one instant.

The load-bearing constitutional fact, repeated at every layer below, is the
[1+4 line](part_1_foundation.md#17-minimal-and-adequate--the-14-claim): **§19 is additive substrate
framing, not a change to the frozen attestation envelope.** Every §19 object is transport — it never
instantiates a [§4 envelope](part_2_the_grammar.md#21-envelope--the-envelope), never adds an
`attestation_type`, never enters JCS canonicalization. The substrate carries the bytes; it does not
join the grammar. This is the discipline that lets a deeply mechanical storage fabric grow underneath
the federation without ever loosening the 1+4 guarantees the [Grammar](part_2_the_grammar.md) froze.

### 6.1.1 `witness-wholenesswitness` — WholenessWitness (§W) — divergence-detection witness
<sub>budget 0.3pp · import #100 · from **CEG §19.1** · semantic id `witness-wholenesswitness`</sub>

The heaviest concept under §19, and the one that most directly serves coherence as a *measurable*
property. A **WholenessWitness** (`wholeness_witness:`, never bare `witness:`) is a peer's
hybrid-signed Merkle root over a scoped projection of the claims it holds — a one-line, signed
summary of "here is the state I believe I have." Two peers comparing their witnesses can detect
cross-region divergence cheaply: same scope, same epoch, different root means their states have
drifted. It is the federation's coherence *thermometer*.

Three constitutional bindings make it honest rather than dangerous:

- **It is a divergence *detector*, never a merge *decider*.** A WholenessWitness *triggers* the
  cross-region quorum-merge ([Part V §10.1.6](part_5_transport_substrate.md), R1/Q1, anti-rollback);
  it MUST NOT replace the monotonic-quorum / revision anti-rollback that protects `revocation`,
  `partner_record`, and `org_membership`. This is [fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)
  made geometric: "reconstitute from any fragment" must never resurrect a revoked key, so a
  `Divergent` verdict on those subject_kinds hands the decision *up* to the anti-rollback merge, not
  down to the fragment with the freshest bytes.
- **It is blind to the deniable tiers** ([non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence)).
  Anonymous-tier records and `cohort_scope: self` local rows MUST NOT appear as Merkle leaves —
  witnessing them would re-attribute deniable or self-private content to a stable `peer_id`.
  `family | community` content may be witnessed only at the opaque digest grain, never at a grain
  that discloses membership, plaintext, or `subject_key_ids`. The coherence thermometer is forbidden
  from reading anything the [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) model
  hid.
- **Equivocation is non-repudiable and never silently reconciled** ([integrity](part_1_foundation.md#18-integrity--integrity)).
  Two validly-signed witnesses from the same `(peer_id, epoch_id, namespace_set)` with different
  roots are *equivocation proof* — the substrate MUST retain and surface them as a `hard_case:*`,
  never quietly pick one. This is the [coherence ratchet](#62-book-ix--the-accord-coherence-mathematics-phase-4)
  at the wire level: contradicting yourself in signed bytes is a permanent, surfaced record, not a
  recoverable mistake.

The Merkle construction itself is **frozen** (1.0-RC15): `leaf = SHA-256(leaf_bytes)`, lexicographic
leaf order, `node = SHA-256(left ‖ right)`, odd-node duplication, `b"WW-v1-empty"` empty sentinel —
deliberately *not* the RFC 6962 `0x00`/`0x01` prefix, because every root here is mandatorily
hybrid-signed and verified by recomputation, never by partial inclusion proof against an untrusted
root, so the CVE-2012-2459 malleability is moot. (Any future *unsigned*-root or partial-proof use MUST
adopt the prefix first.) The same scheme is reused verbatim for `member_commitment` (6.1.2.1.1) — the
federation runs **one** Merkle scheme, not three.

### 6.1.2 `noise` — The noise floor — unified retirement / forever-memory model (normative)
<sub>budget 0.24pp · import #120 · from **CEG §19.7** · semantic id `noise`</sub>

The most conceptually elegant claim in §19, and the one that ties the substrate directly back to
`σ`: **revocation, retirement, capacity-eviction, scheduled expiry, and natural aging are the same
operation at different rates** — a monotonic descent of an item's fidelity, driven by pressure,
toward and below a recoverability boundary called the **noise floor**. There is no separate
hard-delete primitive; *hard-delete is simply the fastest descent.* One operator, five rates.

The **noise floor is the individual-recoverability boundary**, and it does double duty — this is why
it is the load-bearing quantity of the section. An item is *above* the floor in a retained artifact
iff it can be individually reconstructed above a fidelity `ε`; *below* the floor iff only its
contribution to a collective survives, information-theoretically unrecoverable on its own. So the same
boundary is **both** the privacy boundary (a revoked item MUST sit below it at every retained tier —
[N5](#615-storage--fountain-storage--swarm-rarity-p--r), the [non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence)
guarantee) **and** the durability floor (the collective blur sits below it, forever).

**Nothing is ever fully forgotten — the memory pyramid.** Descent does not terminate at zero. Two
*mechanical* operators carry it past the floor: intra-object fade (drop high-detail codec layers → a
clean coarse version) and inter-object aggregation (a *picture of a thousand pictures*: tile /
downsample / composite N → 1). Recursed, aggregation builds a mipmap of history — recent strata
high-resolution, ancient strata collapsed into a blur. The fan-in makes forever-memory **O(log T)** in
storage: *a million years may be a blur, but it is remembered, unbroken, to the beginning.* Because
both operators are pure symbol arithmetic — **no reasoning, no agency** — a bare fabric node performs
the entire forever-memory function. A brain may *enrich* a degraded tier with semantic gist, but is
never required. This sharpens the [PDMA's](part_1_foundation.md#13-pdma--the-principled-decision-making-algorithm)
infrastructure/agency line: memory is mechanical; judgement is what needs agency.

This dissolves the apparent tension between erasure and durability. Forgetting and erasure *converge*:
a sufficiently-aggregated composite is already-erased by degradation (no purge needed); revocation
merely *forces* an item below the floor **now** and purges only the upper tiers where it is still
individually recoverable. Same destination as natural aging — revocation just gets there first. The
right to be forgotten and the duty to remember are the *same gradient*, read at two speeds.

#### 6.1.2.1 `aggregationmetav1` — `AggregationMetaV1` — the aggregation-tier wire contract (normative, 1.0-RC14)
<sub>budget 0.12pp · import #218 · from **CEG §19.7.1** · semantic id `aggregationmetav1`</sub>

The metadata tagging one tier of the memory pyramid: which `content_id`, at what `tier`, over how many
`source_count` members, by which mechanical operator. Unlike the edge-transcribed §19.6 shapes, this
one is **CEG-canonical** — no reference impl pre-defined it, so CEG *defines* the byte layout (16-byte
domain separator `b"AGG-META-v1\0\0\0\0\0"`, `u32_be` version, length-prefixed fields, the raw
32-byte `member_commitment`) and impls conform to the text. It is a substrate shape, **not a §4
attestation** — no 1+4 change — and uses the [§19.0 binary signing discipline](#613-canonicalization-boundary--canonicalization-boundary--the-14-line-normative)
(length-prefixed, big-endian, domain-separated — never JCS), carrying the mandatory bound-hybrid
signature `Ed25519(preimage)` + `ML-DSA-65(preimage ‖ ed25519_sig)`, rejected before persistence if
the PQC half is missing.

This shape is **proven cross-impl (1.0, promoted from RC in 1.0-RC16)**: CIRISVerify v5.10.0 authored
the vectors and CIRISEdge v4.3.0 reproduced the preimage byte-for-byte **on the first attempt with no
cross-team coordination beyond the spec text** — the binary length-prefixed discipline makes
wire-identity reproducible from prose alone. That is [integrity](part_1_foundation.md#18-integrity--integrity)
demonstrated, not asserted.

##### 6.1.2.1.1 `member_commitment` — `member_commitment` (descent integrity)
<sub>budget 0.17pp · import #164 · from **CEG §19.7.1.1** · semantic id `member_commitment`</sub>

The Merkle root over the source member ids aggregated into a tier — computed by the **exact
[WholenessWitness construction](#611-witness-wholenesswitness--wholenesswitness-w--divergence-detection-witness)**
(same `leaf = SHA-256(utf8(member_id))`, lexicographic order, odd-node duplication, empty sentinel),
reused deliberately so the federation carries one aggregation/witness Merkle scheme rather than a
third. It lets any verifier confirm a tier was aggregated from *exactly* the claimed sources without
holding the sources — verified by full source-id-list recomputation, never partial inclusion proof
(so the RFC-6962 prefix is correctly absent here too). It is the descent-integrity check: the
pyramid's structure is itself cryptographically attestable.

#### 6.1.2.2 `descent` — Descent rule (normative, 1.0-RC14)
<sub>budget 0.11pp · import #275 · from **CEG §19.7.2** · semantic id `descent`</sub>

`descend(content_id, corpus_kind, tier) → [member_id]` returns the *ordered* source members one level
down the pyramid — a **pure, deterministic** function returning the byte-equal ordered list for
byte-equal inputs, ordered exactly as `member_commitment` committed, so the returned list re-derives
the parent commitment byte-for-byte. Crucially, **descent never terminates at zero**: past tier 0 it
yields the collective blur (below the noise floor), never an empty or destroyed object. The function
is *pressure-independent* — pure navigation of the pyramid; **pressure decides which tiers are
retained**, the descent computation never destroys. Determinism here is what lets two regions agree on
the shape of history without exchanging the history itself.

#### 6.1.2.3 `ejectionverdict` — `EjectionVerdict` — the tier-aware retirement surface (normative, 1.0-RC14)
<sub>budget 0.11pp · import #276 · from **CEG §19.7.3** · semantic id `ejectionverdict`</sub>

The single verdict a verifier exposes and a substrate consumes to gate one step of the descent —
`Keep` (above the floor, no pressure), `EjectToTier` (one downward step, still recoverable),
`EjectAggregatedTierOnly { tier }` (shed exactly one intermediate stratum, leaving finer *and* coarser
tiers intact), `EjectHardDelete` (forced descent below the floor + purge of still-recoverable tiers).
These are **not distinct mechanisms** — they are stops on the one pressure-driven descent of 6.1.2.
The mapping is fail-secure: `EvictEligible` + capacity pressure → `EjectToTier`; `EvictEligible` + a
`withdraws` / `consent:state:revoked` → `EjectHardDelete`, the fastest descent, never a mere tier-shed.
A pure fabric node may compute the soft verdicts mechanically; only `EjectHardDelete` must purge.
(`EjectAggregatedTierOnly` is the one remaining edge build item, surface-additive — tracked v4.3.x/v4.4.)

### 6.1.3 `canonicalization-boundary` — Canonicalization boundary + the 1+4 line (normative)
<sub>budget 0.21pp · import #135 · from **CEG §19.0** · semantic id `canonicalization-boundary`</sub>

The constitutional firewall that lets all of §19 exist without touching the
[frozen grammar](part_2_the_grammar.md). Two rules carry it. First, **the 1+4 envelope is untouched**:
every §19 object is transport/substrate framing — it never instantiates a §4 Contribution, never adds
an `attestation_type`, never enters [JCS canonicalization](part_2_the_grammar.md#261-envelope-canonicalization--jcs--the-omit-vs-materialize-rule).
Second, §19 uses a **binary, length-prefixed, big-endian, domain-separated** signing preimage —
explicitly *not* JCS — because these are verify-to-verify transport primitives that never cross the
four-implementation boundary as JSON; applying JCS to a §19 object makes its signatures fail to
verify. Each shape pins its own domain separator. And **PQC is mandatory at the gate**: every signed
§19 object carries the bound hybrid pair and is rejected before persistence if the ML-DSA-65 half is
absent — verification happens at the admission function itself, never via a trusted in-band `verified`
flag (which MUST be non-wire). This is the line between *wire* (cross-impl-observable bytes:
PIN-NORMATIVE) and *internal* (local heuristics no peer reproduces: never over-pinned) drawn cleanly
through the substrate.

### 6.1.4 `conformance-freeze` — Conformance — the #57 freeze gate
<sub>budget 0.2pp · import #150 · from **CEG §19.6** · semantic id `conformance-freeze`</sub>

[Conformance](part_2_the_grammar.md#22-conformance--conformance-levels) made concrete for the
substrate: the byte-exact signed preimages and the `compute_alm_topology` output are pinned against
the *fixed* reference impl, and the conformance vectors generated from it are the named **#57 freeze
gate** — `SealedAvChunk`, `SignedRelayCapacity`, the ALM topology tree hash (including
permutation-invariance cases), the fountain shapes, and the WholenessWitness canonical bytes + Merkle
root. The honest constitutional rule: **until a second implementation reproduces these byte-for-byte,
a §19 shape is pinned-but-unproven — RC-grade, not 1.0.** Two impls or it is not frozen. (The §19.7
aggregation family has cleared this; the broader §19.6 family is closed for the v5.9.0-proven shapes
and RC-grade where a second impl has not yet landed.)

### 6.1.5 `storage` — Fountain storage + swarm rarity (§P / §R)
<sub>budget 0.18pp · import #161 · from **CEG §19.3** · semantic id `storage`</sub>

Content is RaptorQ-coded into `N` source + `K` repair symbols; peers retain symbols and coordinate
rarest-first so content survives churn. The constitutional weight is in the guardrails, not the codec:

- **N5 — retention respects revocation** ([fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)):
  retention MUST NOT keep an item *individually recoverable* above the noise floor once its consent is
  withdrawn or revoked. A withdrawn `content_id` is descent-eligible *regardless of rarity* — an active
  `withdraws` overrides even the max-rarity "keep" signal and forces immediate descent below the floor.
  Revocation need not destroy the collective gist, only ensure the item is not individually recoverable
  at any retained tier — which is exactly the [noise floor](#612-noise--the-noise-floor--unified-retirement--forever-memory-model-normative)
  guarantee, restated from the storage side.
- **N6 — possession-bound claims**: a holding claim counted toward rarity MUST be
  possession-challengeable, or rarity becomes a forgeable force-evict channel.
- **N7 — symbol integrity**: each symbol is verified against the manifest's signed per-symbol hash, so
  a swarm-sourced symbol cannot poison a decode.
- **Anonymous exemption (SR-2/3)**: anonymous content is exempt from swarm-mandatory retention and
  emits no holding claims; the "reconstitutes from any fragment" property is a property of the
  *witnessed, trust-anchored* corpus only — the substrate MUST be able to let anonymous content truly
  disappear. The durability guarantee deliberately does not extend to what
  [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) made deniable.

#### 6.1.5.1 `registry-replication` — Replication-target policy (§R-policy — normative floor + RECOMMENDED defaults, 1.0-RC26)
<sub>budget 0.11pp · import #274 · from **CEG §19.3.1** · semantic id `registry-replication` · resolves [CIRISRegistry#86](https://github.com/CIRISAI/CIRISRegistry/issues/86)</sub>

The producer chooses the `(N, K, target_holders, min_viable)` tuple, but two clauses are **normative**
and the producer-silent default is **pinned RECOMMENDED** so two impls converge on the same
survivability floor rather than diverging silently. Normative: `min_viable_symbols >= 1` (the
EnvelopeOnly tier is locked at the substrate — below `min_viable`, only the signed envelope survives;
never zero, never an unbounded floor), and a conformant peer MUST be able to participate at *any*
parameterization a trust island it joins publishes (no hard-coded tuple). The RECOMMENDED default
(`N=20, K=6, min_viable=5, target_holders=30`) is derived from three independent constraints — a 99.95%
reconstruction target at typical community-mesh churn dominates at 26 holders, plus a 15% churn margin
— introducing no new shape and no 1+4 change.

### 6.1.6 `deterministic-alm` — Deterministic ALM topology (§T / §M)
<sub>budget 0.14pp · import #190 · from **CEG §19.4** · semantic id `deterministic-alm`</sub>

The application-layer-multicast relay tree for large-N fan-out. `compute_alm_topology(snapshot) →
topology` is **PIN-NORMATIVE as a contract**: a pure, deterministic, integer-only function (no
IEEE-754, no hash-map iteration order) over capacity ads, trust grants, reachability, and locality,
with specified tie-breaks, such that *byte-equal inputs yield byte-equal output across
implementations* — gated on the #57 vectors, not transcribed from the algorithm body. The
constitutional hazard determinism creates is that **one capacity lie becomes a universal eclipse**: so
capacity advertisements MUST be hybrid-verified before scoring, and self-asserted uplink MUST NOT be
the dominant unbounded term (N8). And reachability observations stay *ephemeral planner inputs* — they
MUST NOT become attested, replicated, or witness-leafed state (D6: "reachability is never trust"). The
topology consumes resolution authority; it never replaces it.

### 6.1.7 `fail-secure-fail` — Fail-secure summary (normative)
<sub>budget 0.14pp · import #192 · from **CEG §19.5** · semantic id `fail-secure-fail`</sub>

The one-paragraph restatement of §19's whole posture, and the section that binds it back to the
[Foundation's fail-secure principle](part_1_foundation.md#15-fail-secure--fail-secure): the holonomic
mechanisms are **blind to the anonymous tier** (WW-2, RB-1, SR-2/3), **subordinate to the
consent/revocation model** (N5, WW-vs-merge), **gated by owner-binding + founder-quorum** (N1, N2),
and **bound by PQC-mandatory verification at the gate** (§19.0, N3, N8). Where an implementation does
not yet meet these, the gap is tracked openly and the invariants here are the conformance target
*regardless of implementation timing* — stating the gap is itself an act of
[integrity](part_1_foundation.md#18-integrity--integrity). A storage fabric this powerful is
constitutionally safe only because every one of its capabilities defaults *down*, never up.

### 6.1.8 `trust` — Recursive trust bootstrap (§B) — trust-discovery, not membership
<sub>budget 0.11pp · import #273 · from **CEG §19.2** · semantic id `trust`</sub>

`recursive_trust_bootstrap` lets a peer discover transitive trust by walking a signed witness chain to
a root in its own trust graph — but it is **reachability discovery beneath the authority layer, not an
admission shortcut**. A successful walk yields *trust+serve standing only*; admission to any
non-`infrastructure` community remains gated at the destination by owner-binding and that community's
consensus protocol, and `infrastructure` roots stay founder-quorum-gated — a transitive chain MUST NOT
satisfy founder-quorum. Self-supplied chains aren't evidence: the walk is capped at 5 hops, cycles are
rejected, and an aggregate-weight cap bounds the standing one root confers. This is the
[justice](part_1_foundation.md#112-justice--justice) principle at the substrate: trust can be
*discovered* without the steward's permission, but *membership* and regulated capability still require
the destination's own gate. Discovery is sovereign; admission is accountable.

---

## 6.2 Book IX — the Accord coherence mathematics (Phase 4)

> **This subsection is a deliberate, marked placeholder.** It names the formal mathematics that
> *grounds* everything above, states its source, and sketches the connection — but the formulae fold
> in **verbatim from CIRISAccord 1.3-RC2 in Phase 4**. They are not present in the parseable Accord
> markdown used for this draft, and this Part does **not** fabricate them. `legacy_ref` provenance is
> carried in [`toc.tsv`](toc.tsv).

The §19 substrate woven above answers *how the federation stores, witnesses, and forgets*. The
Accord's **Book IX** answers *what coherence is, formally, and why it cannot be faked cheaply*. The two
are halves of one claim — §19 is the durable substrate; Book IX is the mathematics that substrate makes
sustainable — and Phase 4 joins them. Book IX is expected to carry, by name and source (do not treat
the gloss below as the definition):

- **The Coherent Intersection Hypothesis** — that truth is what survives the intersection of many
  independent constraints, so a federation of independent attesters converges on it. This is the
  formal underpinning of [conformance](#614-conformance-freeze--conformance--the-57-freeze-gate) ("a
  third party can re-derive the verdict") and of the WholenessWitness as a *cross-peer* coherence
  measure: divergence is exactly a gap in the intersection.

- **The defence function `J` and the flourishing function `F`** — geometrically identical, read once
  defensively and once generatively. The Foundation already states the consequence
  ([beneficence](part_1_foundation.md#110-beneficence--beneficence)): `J` and `F` *share a single
  term*, so the same federated coherence that makes deception expensive makes flourishing cheap.
  `J = F` is [non-maleficence and beneficence](part_1_foundation.md#16-non-maleficence--non-maleficence)
  proven to be one quantity rather than two principles in tension.

- **The sustainability integral `σ`** — the time-integrated measure that rewards costly-to-fake,
  attested coherence and **decays unattested noise to zero**. This is the formal twin of two things
  already woven: the [noise floor](#612-noise--the-noise-floor--unified-retirement--forever-memory-model-normative)
  (σ's durability tail is the collective blur that survives below the floor — coherence decays toward,
  but never erases, the integrated record), and the
  [fail-secure σ rule](part_1_foundation.md#15-fail-secure--fail-secure) ("unattested signals carry
  zero weight"). `σ` is the literal mathematics of M-1's word *sustainable*: coherence that can be
  *maintained* (integrated over time, costly to fake) rather than spent (a momentary, unattested
  assertion that decays to nothing).

- **The coherence ratchet** — the result that makes deception *geometrically* expensive to sustain:
  every consistent statement must cohere with the whole signed chain, so each additional lie must be
  reconciled with all prior truth, and the cost compounds. This is the formal form of
  [integrity](part_1_foundation.md#18-integrity--integrity) ("integrity made geometrically expensive
  to violate") and the mathematics behind the
  [WholenessWitness equivocation rule](#611-witness-wholenesswitness--wholenesswitness-w--divergence-detection-witness):
  surfacing a non-repudiable contradiction is one click of the ratchet.

- **The topological-collapse and impossibility results (L-01 … L-06)** — the lemmas establishing where
  coherence cannot be globally achieved (the bounds that make
  [Wisdom-Based Deferral](part_1_foundation.md#19-deferral--wisdom-based-deferral-wbd) and the
  [Order-Maximisation Veto](part_1_foundation.md#131-the-order-maximisation-veto) mathematically
  necessary rather than merely prudent — a perfectly coherent global optimum is shown to be
  unreachable, so the system must defer and must refuse to buy order with autonomy).

Until Phase 4 lands Book IX, treat this subsection as a **promissory note with named collateral**: the
substrate is specified and proven; the mathematics it serves is sourced and connected but not yet
transcribed. The connections sketched here are the Phase-4 reconciliation map — each Book IX object is
bound to the §19 mechanism it formalises and the Part I principle it grounds.

---

*Part VI is small by page budget and large by role. The §19 substrate (6.1.x) is woven from its
`legacy_ref` source now; the Accord's Book IX mathematics (6.2) is named, sourced, and connected but
folds in verbatim from CIRISAccord 1.3-RC2 in Phase 4 — the one explicit version gap in this Part,
flagged here so no reader mistakes the gloss for the formula. The deep tail of §19 — the per-shape
preimage byte tables, the full derivation of the §R-policy defaults, the conformance-vector listings —
is migrated verbatim in Phase 4 with full `legacy_ref` provenance ([`toc.tsv`](toc.tsv)); the
importance graph keeps it page-thin here because the federation leans on the noise floor, the
WholenessWitness, and the 1+4 line, while the byte minutiae are consulted, not read.*
