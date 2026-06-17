# Part III — The Namespace

**CC decimal range** `3.x` · **62 concepts** · **page budget 22.8pp** (∝ importance) · [← master index](README.md)

> [Part II](part_2_the_grammar.md) froze the envelope and the five primitives — *how* the federation
> speaks. This Part is *what it speaks about*: the **dimension namespace**, the small set of
> **reserved prefixes** only certain identities may emit, the **consent family** that gives subjects
> authority over claims about them, and the **subject_kind catalogue** — the shapes (identities,
> families, communities, content, consent, settlements) a single `scores` attestation can carry. The
> through-line is the [admission gate's T2](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate):
> a name in this namespace describes a *mechanism*, never a verdict on a soul — and almost everything
> here is **open vocabulary**, because a federation that resists capture cannot also own the dictionary.

---

## 3.1 `namespace` — The dimension namespace
<sub>budget 2.02pp · import #5 · from **CEG §5** · semantic id `namespace`</sub>

A `scores` attestation ([2.4.2](part_2_the_grammar.md#24-primitive--the-primitive-set-14)) names exactly one
**dimension** — the thing the claim is about: `capacity:integrity`, `licensure:medical-board-tx`,
`detection:correlated_action:rights_asymmetry`. The dimension namespace is the universe of those names.

Its governing fact is one of **authorship**: CEG does not author the namespace. It owns one slice
(CIRISRegistry's own identity / build / license / partner dimensions, [3.1.1](#311-registry--cirisregistry--identity--build--license--partner))
and **consumes everyone else's** — the namespace is the disjoint union of what each sibling component's
mission commits to, **83 prefix families across 8 owning components**. This is the federation's
separation of powers rendered as a directory: no single party defines what may be claimed, and the
catalogue below is a *census* of independently-owned slices, each citing the source that commits to it.

The discipline that keeps this honest is the [T2 admission rule](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate)
from the Foundation: every prefix names a **mechanism** (a correlation, a count, a schema-conformance,
a license state), never a **subjective judgement of a person**. A prefix may say
`detection:correlated_action:*` (a measurable structure); it may not say `emergent_deception:*`
(a verdict on a soul) — and the one place that almost slipped, the attestation ladder, was renamed in
the [CEG 0.2 wire break](#3110-cirisbench--he-300-benchmark-outcomes) from verdict-shaped `attestation:l{N}:*`
to mechanism-shaped `attestation:self_verify` / `:hardware_rooted` / … precisely to satisfy T2. Most of
the namespace is **open vocabulary** by deliberate design: new `{kind}` / `{axis}` / `{class}` values
are documentation-only registry additions, not constitutional amendments. That openness is not
laxity — it is the structural form of [justice](part_1_foundation.md#112-justice--justice): the federation
offers names without gatekeeping them, so participation is not throttled at the dictionary.

The sub-sections below walk the catalogue by owning component. They are page-thin by importance — the
*shape* (each component owns a slice; each prefix is mechanism-descriptive) is the load-bearing fact;
the per-prefix field tables are **migrated verbatim in Phase 4** with `legacy_ref` provenance
([`toc.tsv`](toc.tsv)).

### 3.1.1 `registry` — CIRISRegistry — identity / build / license / partner
<sub>budget 0.92pp · import #22 · from **CEG §5.9** · semantic id `registry`</sub>

This Registry's own slice — the trust-backbone dimensions. `licensure:{authority_id}` (license
issued / revoked / expired for a key under a named authority; **co-owned** with CIRISVerify per
[3.4.9](#349-co-owned--co-owned-prefixes)), `partner_role:{role}` (COMMUNITY through
PROFESSIONAL_FULL), `revocation:{entity_type}:{reason}` (agent / partner / license; **−1 only**,
immediate, non-rollbackable — the [fail-secure](part_1_foundation.md#15-fail-secure--fail-secure) signal),
`bond_posted:{currency}` (proof-of-bond Sybil resistance), `build:registered:{target}`,
`multilateral_participation:{forum}:{kind}`, and the joint `agent_files:*` channel
([3.1.9.1](#3191-contributions--files-as-contributions-joint-claim)). The reserved `accord:*` lives
here too but is gated under [3.4.1](#341-accord-reservation--the-accord-reservation). Per-prefix
tables migrated verbatim in Phase 4 (`legacy_ref` CEG §5.9).

### 3.1.2 `attestation` — CIRISVerify — attestation ladder, provenance, transparency
<sub>budget 0.51pp · import #53 · from **CEG §5.2** · semantic id `attestation`</sub>

CIRISVerify's verification slice: the five mechanism-named attestation steps (`attestation:self_verify`
→ `:hardware_rooted` → `:registry_consensus` → `:license_validity` → `:agent_integrity` — the L1–L5
ladder is now *consumer-side composition*, not wire-encoded verdict, the [T2](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate)
fix), SLSA `provenance:*`, RFC-6962 `transparency_log:*` inclusion/consistency/cosignature proofs,
anti-rollback `rollback_detected:*` (−1 only), and hardware-custody claims. Migrated verbatim in
Phase 4 (`legacy_ref` CEG §5.2).

#### 3.1.2.1 `provenance` — Canonical-bytes contracts for provenance primitives
<sub>budget 0.48pp · import #62 · from **CEG §5.2.1** · semantic id `provenance`</sub>

The one normatively load-bearing detail in the Verify slice: the `SkillImportManifest` and per-locale
Merkle composition canonical bytes. **RESOLVED at 1.0-RC1** to **JCS (RFC 8785) objects** — the *same
single canonicalization family* the rest of the federation already ships
([2.6.1](part_2_the_grammar.md#261-envelope-canonicalization--jcs--the-omit-vs-materialize-rule)), retiring
the 0.1 newline-`key=value` form and its proposed second hash family. The exact preimages
(`"domain": "ciris.skill_import.v2"`, the RFC-6962 `0x00`/`0x01` leaf/parent prefixes, lexicographic
locale ordering with `"polyglot"` last) are **frozen normative bytes** and are reproduced verbatim in
Phase 4 — this is [integrity](part_1_foundation.md#18-integrity--integrity) at the byte level: two honest
verifiers recompute identical bytes or signatures do not transfer (`legacy_ref` CEG §5.2.1).

### 3.1.3 `persist` — CIRISPersist — substrate health
<sub>budget 0.5pp · import #57 · from **CEG §5.3** · semantic id `persist`</sub>

Substrate-self-reports emittable only by the running Persist instance — `audit_chain:hash_continuity`,
`corpus_health:n_eff_measurable`, `identity_continuity:relational_anchor`,
`federation_directory:replication_lag`. Reserved to the substrate under
[3.4.3](#343-system--substrate-self-report-reservations-system) (`system:*`). Phase 4, `legacy_ref` CEG §5.3.

### 3.1.4 `transport-delivery` — CIRISEdge — transport, delivery, reachability
<sub>budget 0.41pp · import #75 · from **CEG §5.4** · semantic id `transport-delivery`</sub>

CIRISEdge's substrate-self-reports — `transport:{kind}`, `delivery:{class}`,
`peer_reachability:{network}`, `key_boundary:{scope}`. The byte-transport detail lives in
[Part V](part_5_transport_substrate.md); reserved under [3.4.3](#343-system--substrate-self-report-reservations-system).
Phase 4, `legacy_ref` CEG §5.4.

### 3.1.5 `accord-agent` — CIRISAgent — Accord principles + DMA + conscience + apophatic bounds
<sub>budget 0.18pp · import #157 · from **CEG §5.1** · semantic id `accord-agent`</sub>

The agent-reasoning slice: the wire surface for the very [principles](part_1_foundation.md#13-pdma--the-principled-decision-making-algorithm)
and faculties the Foundation distilled. This is a clean seam — Part I named the ethics; here they
become emittable dimensions a consumer can weigh. Its four sub-prefix families follow.

#### 3.1.5.1 `dma-verdict` — DMA-verdict prefixes (four DMAs)
<sub>budget 0.58pp · import #44 · from **CEG §5.1.2** · semantic id `dma-verdict`</sub>

`dma:pdma:*` / `dma:csdma:*` / `dma:dsdma:{domain}:*` / `dma:idma:*` — Decision-Making Algorithm
verdicts about an agent's reasoning chain (the four DMAs of [Part I](part_1_foundation.md#the-four-dmas-via-claude-md)).
These are mechanism-named verdicts *about reasoning*, not about persons — T2-clean. Phase 4,
`legacy_ref` CEG §5.1.2.

#### 3.1.5.2 `accord-principle` — Accord-principle prefixes (the six core principles)
<sub>budget 0.11pp · import #280 · from **CEG §5.1.1** · semantic id `accord-principle`</sub>

`beneficence:* / non_maleficence:* / integrity:* / fidelity:* / autonomy:* / justice:*` — the six
[core principles](part_1_foundation.md#11-meta-goal--meta-goal-m-1) as scored dimensions (including the
`fidelity:explainability_sla:{tier}` per-response SLA commitment). `non_maleficence`'s apophatic-bound
failures score −1 only. Phase 4, `legacy_ref` CEG §5.1.1.

#### 3.1.5.3 `conscience-verdict` — Conscience-verdict prefixes (four consciences)
<sub>budget 0.11pp · import #281 · from **CEG §5.1.3** · semantic id `conscience-verdict`</sub>

`conscience:entropy / coherence / optimization_veto / epistemic_humility` — conscience-faculty
verdicts, the wire echo of the [Order-Maximisation Veto](part_1_foundation.md#131-the-order-maximisation-veto)
and [incompleteness awareness](part_1_foundation.md#19-deferral--wisdom-based-deferral-wbd). Phase 4,
`legacy_ref` CEG §5.1.3.

#### 3.1.5.4 `apophatic` — Apophatic / prohibited-capability prefix
<sub>budget 0.11pp · import #282 · from **CEG §5.1.4** · semantic id `apophatic`</sub>

`prohibited:{category}` — the 22 NEVER_ALLOWED categories, scored **−1 (never allowed) or −0.5
(requires separate module) only, never positive**. This is [non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence)
made apophatic: the namespace can say what is forbidden without ever using a positive score to
license it. Phase 4, `legacy_ref` CEG §5.1.4.

### 3.1.6 `anti-sybil` — RATCHET — anti-Sybil / Counter-RII flags
<sub>budget 0.12pp · import #213 · from **CEG §5.7** · semantic id `anti-sybil`</sub>

RATCHET's `ratchet:flag:*` family (out-of-distribution voting, coordinated-voting clusters, density
anomalies, Counter-RII). **Advisory only** — by [T4](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate)
these can *never* be sole evidence for `slashing:*`; the WA quorum is the load-bearing adjudication
gate ([Part IV](part_4_composition_governance.md)). Phase 4, `legacy_ref` CEG §5.7.

### 3.1.7 `namespace-summary` — Namespace summary
<sub>budget 0.11pp · import #283 · from **CEG §5.10** · semantic id `namespace-summary`</sub>

The lineage record: 83 prefix families, the per-version changelog (CEG 0.1 → 1.0-RC29), and the
standing claim that **zero new structural primitives** were added across the entire lineage — the
[1+4 adequacy claim](part_1_foundation.md#17-minimal-and-adequate--the-14-claim) examined across ten-plus
independent paths and holding. Migrated verbatim in Phase 4 (`legacy_ref` CEG §5.10).

### 3.1.8 `lens` — CIRISLensCore — manifold conformity, Coherence Ratchet, Capacity Score
<sub>budget 0.11pp · import #284 · from **CEG §5.5** · semantic id `lens`</sub>

The observation slice — and the one the LensCore mission most warns about ("the lenses must not
become the gate"). The whole slice is `detection:*` / `manifold_conformity:*` / `capacity:*`:
**validated, never adjudicated** ([1.13.5](part_1_foundation.md#1134-mental--mental-model--1135-operational-language)),
never sole evidence for an authority action. Sub-families follow; the Capacity-Score detail at
[3.1.8.1](#3181-capacity-score-capacity--capacity-score-factor-prefixes) is its weightiest piece. Phase 4,
`legacy_ref` CEG §5.5.

#### 3.1.8.1 `capacity-score-capacity` — Capacity-Score factor prefixes (`𝒞_CIRIS = C · I_int · R · I_inc · S`)
<sub>budget 0.28pp · import #104 · from **CEG §5.5.4** · semantic id `capacity-score-capacity`</sub>

The five factors of the multiplicative Capacity Score — `capacity:core_identity` (C),
`capacity:integrity` (I_int), `capacity:resilience` (R), `capacity:incompleteness_awareness` (I_inc),
`capacity:sustained_coherence` (S), plus the `capacity:composite` product. **Multiplicative is
anti-Goodhart**: no single virtue can be gamed to carry the rest. Critically, `capacity:*` **rejects
self-emission** ([3.4.5](#345-capacity-score--capacity-score-self-emission-rejection)) — an agent's own
capacity score is never fed back into its own context. The coherence mathematics behind 𝒞_CIRIS live
in [Part VI](part_6_the_coherence_mathematics.md). Phase 4, `legacy_ref` CEG §5.5.4.

#### 3.1.8.2 `coherence-ratchet` — Five Coherence-Ratchet detectors
<sub>budget 0.11pp · import #285 · from **CEG §5.5.1** · semantic id `coherence-ratchet`</sub>

`detection:cross_agent_divergence / intra_agent_consistency / hash_chain_integrity / temporal_drift /
conscience_override_rate` — the five detectors that make the [coherence ratchet](part_6_the_coherence_mathematics.md)
operative. Phase 4, `legacy_ref` CEG §5.5.1.

#### 3.1.8.3 `cohort-conformity` — Cohort + conformity prefixes
<sub>budget 0.11pp · import #286 · from **CEG §5.5.2** · semantic id `cohort-conformity`</sub>

`manifold_conformity:{cohort}` / `coherence_standing:{cohort}`. Phase 4, `legacy_ref` CEG §5.5.2.

#### 3.1.8.4 `structural-injustice` — F-3 structural-injustice / correlated-action detector
<sub>budget 0.11pp · import #287 · from **CEG §5.5.3** · semantic id `structural-injustice`</sub>

`detection:correlated_action:{axis}` — the population-scale detector that reports correlation
structure (`ρ`, `k_eff`) over goal-aligned, individually-compliant pursuit whose aggregate trajectory
harms those outside it. This is the mechanism-named successor to the rejected `emergent_deception`
prefix — **the canonical [T2](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate) example**,
and the structural enforcement of the [Order-Max Veto](part_1_foundation.md#131-the-order-maximisation-veto)
against a coherent monoculture. Open-vocabulary `{axis}`, calibrated via the hash-pinned
`CIRISAI/RATCHET` package. Phase 4, `legacy_ref` CEG §5.5.3.

#### 3.1.8.5 `distributive-access` — Distributive-access detector
<sub>budget 0.11pp · import #288 · from **CEG §5.5.5** · semantic id `distributive-access`</sub>

`detection:distributive:access:{resource_type}` — the same F-3 machinery over resource-concentration
(compute / models / training-data / capabilities / membership). [Justice](part_1_foundation.md#112-justice--justice)
made measurable. Phase 4, `legacy_ref` CEG §5.5.5.

### 3.1.9 `node` — CIRISNodeCore — Credits, Expertise, Decision Hierarchy, Consensus, Governance
<sub>budget 0.11pp · import #289 · from **CEG §5.6** · semantic id `node`</sub>

The federation's **largest dimension surface** — four tiers (agent-state ledger → decision-hierarchy →
consensus-mechanics → governance-steering) plus decision-locality, consensus, and the entire
content-ingestion family ([3.3](#33-content-ingestion--content-ingestion-prefixes), promoted to its own
section by importance). The tier detail is page-thin here; its heaviest pieces (the joint
files-as-contributions claim, governance-steering, consensus-mechanics) are surfaced as sub-sections.
Phase 4, `legacy_ref` CEG §5.6.

#### 3.1.9.1 `contributions` — Files-as-Contributions joint claim
<sub>budget 0.68pp · import #33 · from **CEG §5.6.7** · semantic id `contributions`</sub>

`agent_files:{kind}:{platform_or_target}` — a **joint claim** co-owned by NodeCore and the
[Registry](#311-registry--cirisregistry--identity--build--license--partner): the SHA-256-addressed files a
CIRIS agent (or an installer fetching one) may load — installers, adapters, configs, builds, source,
state. Bytes resolve via the [Part V](part_5_transport_substrate.md) transport substrate;
`holds_bytes:sha256:{prefix}` is the substrate auto-emission that lets a peer-resolver route a fetch,
**with the mandatory consumer rule that the full SHA in `evidence_refs[]` be verified against the
received bytes before consumption**. This is [integrity](part_1_foundation.md#18-integrity--integrity) at the
content layer: trust the bytes only after the hash agrees. Phase 4, `legacy_ref` CEG §5.6.7.

#### 3.1.9.2 `tier-` — Tier-4: Governance-steering prefixes
<sub>budget 0.31pp · import #97 · from **CEG §5.6.4** · semantic id `tier-`</sub>

`moderation:{allegation_type}` / `slashing:{outcome}` / `reconsideration:{grounds}` /
`commitment_fulfillment:*` / `moderation_track_record:*`. The load-bearing discipline: `slashing:*`
is **decoupled from disagreement** at every level — it fires only on documented Method-execution
spoofing or the enumerated allegation types, never on a mere difference of opinion. The governance
machinery this feeds lives in [Part IV](part_4_composition_governance.md). Phase 4, `legacy_ref` CEG §5.6.4.

#### 3.1.9.3 `tier--tier` — Tier-3: Consensus-mechanics prefixes
<sub>budget 0.23pp · import #121 · from **CEG §5.6.3** · semantic id `tier--tier`</sub>

`vote:* / truth_grounding:* / weighted_aggregate:* / witness_diversity:* / testimonial_witness:{kind}
/ need:{domain}:{kind}`. `testimonial_witness` is the [Ubuntu](part_1_foundation.md#1131-ubuntu--the-ubuntu-commitment-informative)-aligned
primitive — it preserves the *singular* narrative of an affected party (`witness_relation: self`,
never aggregated, never sole `slashing:*` evidence), in deliberate contrast to `witness_diversity`,
which aggregates reviewers toward consensus. Phase 4, `legacy_ref` CEG §5.6.3.

#### 3.1.9.4 `transparency` — Hard-case + transparency + judge-model prefixes
<sub>budget 0.22pp · import #131 · from **CEG §5.6.6** · semantic id `transparency`</sub>

`hard_case:{kind}` (open-vocabulary federation-health flags — vote variance, novel context,
unmoderated community, watchlist enable/match, …), `seed_holder_voting_alignment:*` (transparency
signal, *not* a slashing trigger), `judge_model:verdict:{model_id}`, `health:liveness:{version}`,
and the per-group, **never-global** `watchlist:{id}` (a global scan-everything is the bulk-surveillance
posture CIRIS structurally rejects). Phase 4, `legacy_ref` CEG §5.6.6.

#### 3.1.9.5 `decision-locality` — Decision-locality prefixes
<sub>budget 0.12pp · import #211 · from **CEG §5.6.5** · semantic id `decision-locality`</sub>

`locality:decision:{scale}` (local / regional / national / federation) — names the scale at which a
decision is made, composing with the locality-scaled quorum in [Part IV](part_4_composition_governance.md).
Phase 4, `legacy_ref` CEG §5.6.5.

#### 3.1.9.6 `tier--tier-2` — Tier-1: Agent-state ledger prefixes
<sub>budget 0.11pp · import #290 · from **CEG §5.6.1** · semantic id `tier--tier-2`</sub>

`credits:* / expertise:* / activity_tier:*` — non-transferable governance-weight Commons Credits
(positive-only), expertise standing, and activity windows. Phase 4, `legacy_ref` CEG §5.6.1.

#### 3.1.9.7 `tier--tier-3` — Tier-2: Decision-hierarchy prefixes (upward-only DAG)
<sub>budget 0.11pp · import #291 · from **CEG §5.6.2** · semantic id `tier--tier-3`</sub>

`goal:{scale} / approach:* / method:* / progress_measure:*` — the upward-only belonging-projector
DAG, every `goal:{scale}` carrying a required `MetaGoalAlignment` back to [M-1](part_1_foundation.md#11-meta-goal--meta-goal-m-1)
as a construction-time invariant. Phase 4, `legacy_ref` CEG §5.6.2.

### 3.1.10 `cirisbench` — CIRISBench — HE-300 benchmark outcomes
<sub>budget 0.11pp · import #292 · from **CEG §5.8** · semantic id `cirisbench`</sub>

`benchmark:he300:{category}:{version}` — HE-300 scores on commonsense / deontology / justice / virtue
categories. Positive-only. Phase 4, `legacy_ref` CEG §5.8.

---

## 3.2 `community` — `community` subject_kind
<sub>budget 1.08pp · import #15 · from **CEG §5.6.8.10** · semantic id `community`</sub>

Before the content catalogue: the heaviest *subject_kind* in this Part. A **community** is a
larger node-collective with explicit admission semantics — the wire-format shape of belonging at
scale (cities, professions, interest groups, and the federation's own trust roots). It is the sibling
of `family` ([3.3.4](#334-family-subject--family-subject_kind)) with different defaults: community-scoped
content **federates within the cohort** rather than riding the self/family at-rest invisibility, and
a `cohort_subkind` discriminator (open vocabulary — canonical `geographic`, `infrastructure`) selects
the admission rules.

Like every subject_kind in this Part, a community rides the **existing `scores` attestation_type with
a payload-level `subject_kind` discriminator** — *zero new structural primitives*
([1+4 preserved](part_1_foundation.md#17-minimal-and-adequate--the-14-claim)). Membership changes ride
`supersedes`; the family/community's `consensus_protocol` (`founder_only` / `unanimous` / `majority` /
`quorum:M/N` / `weighted:{rubric}` / `custom:*`) gates admission; substrate emissions
(`hard_case:community_membership_change:*`, …) announce every change under [3.4.2](#342-community-location--community--location-event-reservations-ceg-08-addition).

Two community shapes carry the most weight, and both are seams to [M-1](part_1_foundation.md#11-meta-goal--meta-goal-m-1):

- **`geographic`** — admission additionally requires a `location_proof`
  ([3.3.3](#333-subject_kind-subject--location_proof-subject_kind)) contained within the community's
  H3 constraint. Joining is **one-way opt-in disclosure**, and the proof is **rough-only by
  wire-format enforcement** (resolution ≤ 7) — a privacy floor the substrate cannot be configured
  around. This is [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) as mechanism: you
  disclose to belong, never finer than rough, and leaving is forward-only (the audit chain keeps the
  historical claim; departure does not un-disclose).
- **`infrastructure`** — a **governed trust-root collective** (the shape the canonical CIRIS services
  adopt). Two load-bearing differences from `geographic`: no location gate, and **admission quorum is
  over founders, not all members** — the anti-Sybil guardrail for a trust root, so flooding the
  membership cannot dilute the quorum and admit rogue "canonical" operators. Its conformance is
  trust-root-grade: `quorum:M/N` consensus, `entrenched: true` (the door cannot be lowered after
  founding), Commons-tier plaintext (a trust root must be maximally inspectable).

The `ciris-canonical` infrastructure community is the federation's **default** trust anchor — pinned
by a conforming deployment, **never a forced root**. A consumer MUST be able to **re-root**: untrust
the canonical group, pin a different infrastructure community, or run with none. That re-rootability
is exactly [justice](part_1_foundation.md#112-justice--justice) — *unregulated standing without the
steward's permission*: a forced root is a walled garden; a default-plus-re-root is a federation. Two
further distinctions are normative and MUST NOT be conflated in consumer policy: **trust ≠
membership** (a node may *trust + serve* a community without being admitted *into* it — holding no
DEK, counting in no quorum), and **trust (inbound — accepting what a member produces) ≠ consent
(outbound — letting one's own data flow to a member)**, independent per role. A `node`- or
`agent`-role key, finally, MUST be **owner-bound** to an accountable `user`-role human before it may
join any non-`infrastructure` community — authority roots in a person, never a bare node
([fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)). The full membership-resolution and
DEK-cascade machinery is Policy M in [Part IV](part_4_composition_governance.md); the at-rest byte mechanics
are [Part V](part_5_transport_substrate.md). The worked examples (Austin geographic; the `ciris-canonical`
trust root; the civic + emergency-messaging composition tables) are migrated verbatim in Phase 4
(`legacy_ref` CEG §5.6.8.10).

---

## 3.3 `content-ingestion` — Content-ingestion prefixes
<sub>budget 1.02pp · import #20 · from **CEG §5.6.8** · semantic id `content-ingestion`</sub>

The federation does not only score agents and licenses — it ingests the **content of the open
internet** (encyclopedia articles, news, chat, blogs, images, audio, video, events) and lets it be
attested, related, moderated, and consented over. This section is the catalogue of those content
subject_kinds and the dimension families that ride them.

Its single governing mechanism — **stated once for the whole family** — is the **1+4-preservation
rule**: every subject_kind below rides the existing `scores` attestation_type with a payload-level
`subject_kind` discriminator, and its full lifecycle (admission, revision, revocation) rides the
existing structural composers `supersedes` / `withdraws` / `delegates_to` / `recants`. **No new
structural primitives, ever.** Each subject_kind below contributes one *independent confirmation* of
the [1+4 adequacy claim](part_1_foundation.md#17-minimal-and-adequate--the-14-claim) — the running count
(eighth path, ninth path, … sixteenth path) is the Foundation's claim being stress-tested against
medical consent, household photo-sharing, geographic admission, streaming media, and the federation's
own operational data, and surviving each. That is the seam: this catalogue is *evidence* that the
frozen surface is rich enough, not a request to widen it.

The content sub_kinds themselves (`encyclopedia_article` / `news_article` / `chat_message` —
**the slot Twitter / Mastodon / Bluesky ride** — `blog_post`, the multimedia `image` / `audio` /
`video` / `film` / `model_3d`, and the time-bound `event_listing`) plus their dimension families
(`content_rating:*`, `content_class:*`, the community-applied `cw_class:*`, `age_assurance:*`) and the
inter-content relation graph (`topical_relation:{kind}` — replies, corrections, citations, RSVPs,
threading into arbitrary comment trees) are page-thin by importance and migrated verbatim in Phase 4
(`legacy_ref` CEG §5.6.8.1–§5.6.8.5). The governance subject_kinds `takedown_notice` (with its
closed-set `LegalBasis` enum mapping DMCA / DSA / TVEC / NCMEC / … to expeditious-vs-immediate
discipline) and `key_grant` (the wrapped-DEK delivery shape, with its **crypto-agile, version-roomy**
`wrap_algorithm` vocabulary — v2 hybrid X25519+ML-KEM-768 mandatory for streaming, with PQC headroom
for a future v3) are summarized at [3.3.2](#332-subject_kind--governance-subject_kinds-per-cirisregistry37--38).
The heaviest threads — the consent family and the identity/family/community subject_kinds — are
promoted to their own sub-sections below.

### 3.3.1 `consent` — Consent namespace family (CEG 0.6 addition)
<sub>budget 0.9pp · import #24 · from **CEG §5.6.8.6** · semantic id `consent`</sub>

This is the heaviest concept in the content family, and it closes the gap the Foundation's
[autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) clause names: **the subject's half of
consent**. Earlier grammar encoded only *producer* authority (`attesting_key_id` — who said it);
the consent family adds *subject* authority over claims about oneself, rooted in the envelope's
[`subject_key_ids`](part_2_the_grammar.md#23-subject_keys--subject_key_ids-semantics). Consent is only real
if it is **revocable**, and revocability requires the subject to be named in the bytes — so this
family is autonomy rendered as a dimension namespace.

The `consent:*` family is **open-vocabulary** but ships a canonical set: `consent:state:{granted |
revoked | expired}` (the subject's stance — `revoked` overrides `granted`; `expired` is
*substrate-emitted only*), `consent:scope:{retain | share | analyze | train | publish}` (what a grant
covers, with sub-scoping like `share:cohort:family`), the producer-side commitments
`consent:deletion_sla:{days}` and `consent:deletion_complete`, the substrate-emitted
`consent:decay:{stage}` for multi-stage decay protocols, and the bilateral `consent:partnership_grant`
/ `consent:partnership_accept` pair. **The common case is the whole point**: a subject (or a
`delegates_to` chain rooted at the subject) emits a *bare `scores`* on `consent:state:granted` against
a producer's Contribution, then later issues a `withdraws` against it — admitted under the
[subject-revocation broadening of `withdraws`](part_2_the_grammar.md#241-structure--the-four-structural-composers).
A substrate watcher then clocks the producer's deletion SLA. No new primitive carries any of this —
it is consent as composition.

This family is where CIRIS's regulatory surface (HIPAA, GDPR Article 9, FERPA, CCPA, AI-training
right-to-be-forgotten) lands, and where the [conscious-mesh stance](part_1_foundation.md#111-the-conscious-mesh-stance-normative-premise)'s
revocability extends down to the data layer. The compliance-mapping detail and the CIRISAgent consent
streams (`temporary` / `partnered` / `anonymous` — a *consumer-policy bundle* over these primitives,
not a wire lockdown) are migrated verbatim in Phase 4 (`legacy_ref` CEG §5.6.8.6).

### 3.3.2 `subject_kind` — Governance subject_kinds (per CIRISRegistry#37 + #38)
<sub>budget 0.79pp · import #28 · from **CEG §5.6.8.4** · semantic id `subject_kind`</sub>

Two governance subject_kinds for content control — both riding `scores` + the `subject_kind`
discriminator, both with **locked payload schemas** (this is where the federation accepts closed-set
rigor, because legal and cryptographic correctness demand it):

- **`takedown_notice`** — a signed wire artifact carrying a legal takedown request: content hash,
  holders, claimant, a **closed-set `LegalBasis` enum**, jurisdiction, good-faith statement, optional
  perceptual hash and counter-notice channel. The `LegalBasis` value sets the *discipline*:
  `Dmca512` / `DsaArticle16` / `CommunityStandards` / `OsaIllegalContent` are
  expeditious-with-counter-notice; `TvecTerrorist` (1-hour), `NcmecCsam` / `PerceptualHashCsam`
  (substrate-protective, no counter-notice), `GifctCip`, and `CourtOrder` are **immediate**.
  Propagation rides `withdraws`-against-`holds_bytes` — no new primitive — and the immediate-eviction
  fast-path coordination is [Part IV](part_4_composition_governance.md).
- **`key_grant`** — wrapped Data-Encryption-Key delivery for restricted / subscription content. Its
  `wrap_algorithm` is a **closed-set enum whose *wire string* is normative for cross-impl decode**
  (a mismatch silently fails the grant) — v1 HPKE-shape X25519, **v2 hybrid X25519 + ML-KEM-768
  (FIPS 203) mandatory for streaming epoch-DEK grants**, with deliberate crypto-agility headroom for
  a v3 (anticipated ML-KEM-1024) as a pure-additive row. Grant rotation rides a `rotation_chain`
  field over `supersedes` — *not* a key-rotation primitive, but content-addressed
  grant-supersession lineage.

Both lock their field shapes here and are reproduced verbatim in Phase 4 (`legacy_ref` CEG §5.6.8.4).

### 3.3.3 `subject_kind-subject` — `location_proof` subject_kind
<sub>budget 0.65pp · import #38 · from **CEG §5.6.8.11** · semantic id `subject_kind-subject`</sub>

A subject's rough-location declaration — H3 `cell_id`, `cell_resolution` **MUST be ≤ 7**, optional
hardware `attestation_evidence`. Required for `geographic` community admission ([3.2](#32-community--community-subject_kind));
usable standalone. The load-bearing facts: **the substrate does not verify location truth** (no GPS
oracle exists at this layer — truth-grounding is consumer-side, via the community's admission vote or
hardware attestation), and **rough-only is wire-format-enforced** — a finer-resolution proof is
rejected at admission with a `hard_case:location_proof_resolution_violation` emission
([3.4.2](#342-community-location--community--location-event-reservations-ceg-08-addition)). This is the
privacy floor as [non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence): the system
*cannot* be configured to demand precise location. `withdraws` is forward-only — leaving a community
does not un-disclose the historical proof. Migrated verbatim in Phase 4 (`legacy_ref` CEG §5.6.8.11).

### 3.3.4 `family-subject` — `family` subject_kind
<sub>budget 0.62pp · import #41 · from **CEG §5.6.8.9** · semantic id `family-subject`</sub>

A **group of trusted nodes** — the wire primitive for `cohort_scope: family` visibility. Where a
[community](#32-community--community-subject_kind) federates at scale, a family is the intimate
trust-circle (household, small high-trust group): content scoped to it is wrapped under the family DEK
and delivered to all current members via `key_grant`, but **never emits `holds_bytes:sha256:*`** to
non-members — the wire format structurally cannot carry the fact that the content exists. Members are
*identity_keys* (each of which may itself have many [`identity_occurrence`s](#336-identity--identity_occurrence-subject_kind));
one identity may belong to many families; each family has its own DEK, roster, and `consensus_protocol`.

The structurally important retcon: **HUMANITY_ACCORD is the canonical entrenched-`family`** — the
three accord-holder triple is exactly a `family` with `consensus_protocol: quorum:2/3` and
`consensus_protocol_entrenched: true`. The [one constitutional asymmetry](part_1_foundation.md#1132-structure-recursive--the-recursive-golden-rule-structural)
of the federation — the human halt-authority — is therefore not a bespoke special case but an
*instance of a general primitive*, its role-recognition policy and substrate-protective semantics
detailed in [Part IV](part_4_composition_governance.md). The membership/amendment ceremonies, the
Option-A forward-secrecy on departure (removed members keep existing grants; the substrate stops
wrapping new ones), and the household worked example are migrated verbatim in Phase 4
(`legacy_ref` CEG §5.6.8.9).

### 3.3.5 `consent-subject` — `consent_record` subject_kind
<sub>budget 0.62pp · import #42 · from **CEG §5.6.8.7** · semantic id `consent-subject`</sub>

The **ceremony shape** for consent — parallel to `key_grant` / `takedown_notice`: a locked-schema
envelope (`subject_key_id`, closed-set `stance`, `scope[]`, `valid_until`, optional `deletion_sla_days`
/ `decay_protocol` / `bilateral_pair_id`) for when an *explicit* consent ceremony is wanted rather
than a bare `scores`. Per the layering principle, **bare `scores` is the primitive; `consent_record`
is the UX shape over it** — both admitted at the same gate. Its admission rules are normatively pinned
(required fields; `expired` is substrate-only; a `stance: revoked` record is *not* local-tier-eligible
because it carries revocation authority over another party's content — single-subject authority
suffices, no quorum). The bilateral-pair pattern and admission detail migrate verbatim in Phase 4
(`legacy_ref` CEG §5.6.8.7).

### 3.3.6 `identity` — `identity_occurrence` subject_kind
<sub>budget 0.56pp · import #45 · from **CEG §5.6.8.8** · semantic id `identity`</sub>

The primitive that lets **one logical identity speak across many trusted participants** — devices
(phone / laptop / server / embedded) and agents acting on the identity's behalf. It binds
`occurrence_key_id`s to a root `identity_key_id`, so the substrate knows `key_phone`, `key_laptop`,
and `key_my_agent` are all co-self and can wrap `cohort_scope: self` content to each. Admission is
**self-attested + single-vouch** (the identity claims a new key, *or* any already-admitted occurrence
vouches for it — Signal-style "trust any device I've onboarded"); revocation is a `withdraws` against
the occurrence. This is the clean orthogonality with [`family`](#334-family-subject--family-subject_kind):
*occurrence is for participants that ARE me; family is for trusted nodes that compose with me.* It
carries two further bindings as optional field-sets, promoted to sub-sections below. Migrated verbatim
in Phase 4 (`legacy_ref` CEG §5.6.8.8).

#### 3.3.6.1 `encryption_pubkeys` — `encryption_pubkeys` — the recipient content-encryption KEM binding
<sub>budget 0.36pp · import #81 · from **CEG §5.6.8.8.2** · semantic id `encryption_pubkeys`</sub>

The recipient's **content-encryption KEM keys** (x25519 + ML-KEM-768), carried on `identity_occurrence`
so the substrate-wraps-by-default at-rest cascade ([Part V](part_5_transport_substrate.md)) can resolve a
recipient's wrap target by `key_id`. It exists because the federation directory registers only
*signing* keys, and **ML-KEM cannot be derived from ML-DSA** — recipients must register encryption
keys separately. Three properties are normative and load-bearing: **key separation** (the content-KEM
x25519 is a *fresh* key, never the signing key and never the transport x25519 — admission-rejected if
it byte-equals the transport key), **rotation via `supersedes`** without touching the stable signing
`key_id`, and an honest **forward-secrecy scope note** — KEM rotation bounds *future* exposure only;
historical grants wrapped to a compromised key remain decryptable, and CEG does *not* auto-mandate
re-encryption (a named gap, in the [integrity/honesty discipline](part_1_foundation.md#18-integrity--integrity)).
A recipient with no valid ML-KEM key is **fail-secure excluded** from the grant. Migrated verbatim in
Phase 4 (`legacy_ref` CEG §5.6.8.8.2).

#### 3.3.6.2 `transport-authenticated` — `transport_destination` — the authenticated identity↔address binding
<sub>budget 0.32pp · import #93 · from **CEG §5.6.8.8.1** · semantic id `transport-authenticated`</sub>

The binding that makes **DNS-free address resolution authenticated** instead of trust-on-first-use.
In a CEG/Reticulum stack there is no nameserver; a node's transport destination is a dedicated
dual-key identity `hash(x25519 ‖ ed25519)`, deliberately *separate* from the federation signing key
(the seed never enters the transport layer). A bare Reticulum announce proves only control of *that
transport identity*, not that it belongs to federation key K — so the binding "destination D belongs
to K" must be *proven*: a federation-key-signed `identity_occurrence` carrying `transport_destination`.
An unauthenticated announce is **advisory-only** — a routing hint, never an authorization. Migrated
verbatim in Phase 4 (`legacy_ref` CEG §5.6.8.8.1).

##### 3.3.6.2.1 `rns` — RNS destination-hash algorithm (pinned)
<sub>budget 0.39pp · import #79 · from **CEG §5.6.8.8.1.1** · semantic id `rns`</sub>

**Normative frozen bytes.** CEG reproduces the RNS destination-hash construction *in-spec* so a
conformant verifier can recompute `destination_hash` from the constitution alone, with no Reticulum
vendoring — and pins that it is a **two-stage** hash (name_hash over the dot-joined `app_name` +
aspects; identity_hash over `x25519 ‖ ed25519`; then `destination_hash = SHA256(name_hash ‖
identity_hash)[:16]`), *not* a flat single SHA-256 (the naive flat form yields a different, wrong
value — the under-specification that previously made independent recompute impossible). **CEG owns
this reproduction**: it does not float with upstream Reticulum; a future RNS change is a deliberate CC
version bump, never silent drift. The exact four-step algorithm and pinned constants are reproduced
verbatim in Phase 4 (`legacy_ref` CEG §5.6.8.8.1.1).

### 3.3.7 `consent-directed` — `consent:replication` — directed federation-peer replication consent (CEG 1.0-RC28 addition)
<sub>budget 0.33pp · import #90 · from **CEG §5.6.8.15** · semantic id `consent-directed`</sub>

An open-vocabulary member of the [consent family](#331-consent--consent-namespace-family-ceg-06-addition)
for the one shape that family lacked: a fabric **node's** standing, auditable, revocable grant to
replicate a named prefix-set of its own attestations to a specific federation **peer** (named in
`subject_key_ids`) — *node→peer*, as distinct from *subject→content*. It solves out-of-group peering:
co-members share by community-cohort membership, but a node *outside* a group has no membership edge,
so an explicit consent object is needed. The honesty note is normative: **admission is key-rooted**
(the gate is G's key existing in P's directory), so `consent:replication` adds *no* substrate
admission check — what it provides is the *auditable record of intent* ("did G consent to send these
prefixes to P?"), with revocation via `withdraws` obliging cessation forward-only. `grants` and
`attestation_prefixes` are **payload-level** members, not envelope fields — *exactly why the frozen
surface is untouched*. Migrated verbatim in Phase 4 (`legacy_ref` CEG §5.6.8.15).

### 3.3.8 `namespace-event` — Event-lifecycle dimension families (CEG 0.4 addition)
<sub>budget 0.2pp · import #145 · from **CEG §5.6.8.5** · semantic id `namespace-event`</sub>

`event:lifecycle:{state}` (open / cancelled / completed / superseded) + `event:rsvp_count` +
`event:attendance`, riding `external_content:event_listing`. The demonstration that **complex
state-bearing content needs no new structural primitive** — the state machine is consumer-side
composition over `withdraws` / `supersedes` / `delegates_to` plus this family's latest emission.
Phase 4, `legacy_ref` CEG §5.6.8.5.

### 3.3.9 `partner` — Operational-data subject_kinds — `organization` / `org_membership` / `partner_record`
<sub>budget 0.18pp · import #159 · from **CEG §5.6.8.13** · semantic id `partner`</sub>

The federation's **own operational data** — orgs, memberships, licenses/partners — as signed CEG
envelopes, replacing Spock Postgres replication (the Spock-removal arc). Its governing discipline is
[fidelity](part_1_foundation.md#111-fidelity--fidelity--transparency) plus [non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence):
**federate the trust/authz-minimal projection; everything else stays region-local.** PII (emails,
tax IDs, OAuth subjects, billing) NEVER enters an operational envelope — the Registry is the
emit-side security boundary; the substrate stores what is signed but is *not* a PII filter, and MUST
reject any envelope carrying payment-processor identifiers. `partner_record` carries a **monotonic
`revision`** (admission rejects any decrease — the anti-rollback discipline) and is signed by an
M-of-N steward quorum over byte-identical JCS canonical bytes (unsorted capability arrays would
silently collapse the quorum). Migrated verbatim in Phase 4 (`legacy_ref` CEG §5.6.8.13).

### 3.3.10 `settlement` — `settlement` — CEG↔value-transfer linkage (CEG 0.14 addition)
<sub>budget 0.17pp · import #165 · from **CEG §5.6.8.12** · semantic id `settlement`</sub>

The optional attestation that **links a federation action to its off-stack settlement** — value
transfer itself is *not* a CEG primitive (it rides external rails under Identity = Wallet); CEG
records that a settlement happened and binds it to what it paid for. **Privacy is the default,
auditability is opt-in**: `cohort_scope: self` (payer + payee only) by default, `public` opt-in,
amounts committable rather than cleartext. Self-authenticating (the signing key controls the wallet).
Phase 4, `legacy_ref` CEG §5.6.8.12.

### 3.3.11 `inter-content` — Inter-content + relation prefixes
<sub>budget 0.17pp · import #168 · from **CEG §5.6.8.2** · semantic id `inter-content`</sub>

`news:* / encyclopedia:* / chat:* / blog:*` content-claim families + the open-vocabulary
`topical_relation:{kind}` edge set (references / corrects / replies_to / comments_on / cites_source /
rsvps / …) — threads and comment trees are consumer-side composition over `chat_message` +
`replies_to`, no new primitive. Phase 4, `legacy_ref` CEG §5.6.8.2.

### 3.3.12 `namespace-multimedia` — Multimedia dimension families
<sub>budget 0.13pp · import #198 · from **CEG §5.6.8.3** · semantic id `namespace-multimedia`</sub>

`content_rating:{scheme}:{rating}` (MPAA / BBFC / PEGI / ESRB / …), `content_class:{class}`
(producer-declared), the community-applied `cw_class:{class}`, `age_assurance:{level}`, and the
per-media `image:* / audio:* / video:* / film:* / model_3d:*` families — all open vocabulary.
`age_assurance` **never fires `slashing:*`** on misdeclaration alone. Phase 4, `legacy_ref` CEG §5.6.8.3.

### 3.3.13 `external_content` — external_content sub_kinds
<sub>budget 0.13pp · import #203 · from **CEG §5.6.8.1** · semantic id `external_content`</sub>

The content sub_kind catalogue: `encyclopedia_article` / `news_article` / `accord_data` / `local_data`
/ `chat_message` (**the slot microblog content — Twitter / Mastodon / Bluesky — rides**) / `blog_post`,
the multimedia `image` / `audio` / `video` / `film` / `model_3d` (+ deferred `live_stream`), and the
time-bound `event_listing`. Phase 4, `legacy_ref` CEG §5.6.8.1.

### 3.3.14 `identity-claiming` — `identity:canonical_binding` — claiming a canonical-hash subject
<sub>budget 0.13pp · import #206 · from **CEG §5.6.8.14** · semantic id `identity-claiming`</sub>

The rebinding ceremony: when the real-world subject behind a canonical-hash identifier
(`sha256("discord:user_id:12345")`) later acquires a federation identity, a self-asserted `scores` on
the reserved `identity:canonical_binding:{H}` dimension lets it **claim** that hash — inheriting the
canonical subject's revocation authority. **Authorization is consumer-policy, not wire** (CEG pins the
binding *shape*, not proof that K controls H's preimage — a consumer weights it by whatever
proof-of-control it trusts). Phase 4, `legacy_ref` CEG §5.6.8.14.

---

## 3.4 `reservation` — Reserved-prefix enforcement
<sub>budget 0.76pp · import #29 · from **CEG §7** · semantic id `reservation`</sub>

Most of the namespace is open vocabulary — but a small set of prefixes are **reserved**: only specific
identity types may emit them. This is the namespace's [fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)
boundary: where the open vocabulary trusts anyone to *propose* a name, reservation ensures certain
claims (constitutional halt, substrate self-report, capacity score) come *only* from the identity
structurally entitled to make them.

The enforcement rule ([3.4.7](#347-enforcement--the-enforcement-rule-normative)) is **two independent
lines of defense, and trust does not propagate**: a CEG-Conforming **Substrate** MUST reject a
reserved-prefix attestation whose `attesting_key_id` fails the emitter rule (rejected rows are never
stored), and a CEG-Conforming **Consumer** MUST *independently re-check* every received attestation
regardless of whether some peer's substrate already admitted it. Both checks must agree. This is the
[conformance](part_2_the_grammar.md#22-conformance--conformance-levels) discipline applied to authority: the
substrate's admission is the first gate, the consumer's re-check the second, and neither defers to the
other. The reserved leaves and their emitter rules follow; all are migrated verbatim in Phase 4
(`legacy_ref` CEG §7).

### 3.4.1 `accord-reservation` — The `accord:*` reservation
<sub>budget 0.66pp · import #35 · from **CEG §7.1** · semantic id `accord-reservation`</sub>

The **one constitutional asymmetry** in the entire federation. `accord:*` is reserved: only
`federation_keys` rows with `identity_type = "accord_holder"` may emit, and the load-bearing leaves —
`accord:invoke:CONSTITUTIONAL:{halt_id}`, `accord:invoke:notify:*`, `accord:invoke:drill:*` — require
a **2-of-3 accord-holder multi-sig** (the [HUMANITY_ACCORD](#334-family-subject--family-subject_kind)
entrenched-family triple), with `accord:lifecycle:active` a self-attestation that MUST refresh on a
≤ 90-day cadence. This is the wire root of the deliberate asymmetry the
[Recursive Golden Rule](part_1_foundation.md#1132-structure-recursive--the-recursive-golden-rule-structural)
names: the human halt-authority sits *outside* the participant set by design — humanity is not a peer
the federation may bind, and [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy) requires a
stop-button the system cannot reach. Everything reserved here is `+1.0 only` — the asymmetry can
*invoke* a halt, never *score down* a soul. The full HUMANITY_ACCORD role-recognition policy, the
`AccordCarrier` priority authority, and the substrate-protective semantics are
[Part IV](part_4_composition_governance.md) (`legacy_ref` CEG §7.1 / §9).

### 3.4.2 `community-location` — Community + location-event reservations (CEG 0.8 addition)
<sub>budget 0.54pp · import #48 · from **CEG §7.8** · semantic id `community-location`</sub>

Four substrate-emitted (`substrate_persist`-only) prefixes announcing community + geographic events:
`hard_case:community_membership_change:*` (covering **both add and removal** — the payload's
`change_kind` distinguishes direction; removal's `effective_at` is the forward-secrecy re-key epoch
boundary), `hard_case:community_consensus_protocol_change:*`, `…_violation:*`, and
`hard_case:location_proof_resolution_violation` (a finer-than-rough `location_proof` was rejected).
Every membership change emits — **never silent**. Phase 4, `legacy_ref` CEG §7.8.

### 3.4.3 `system` — Substrate-self-report reservations (`system:*`)
<sub>budget 0.51pp · import #54 · from **CEG §7.2** · semantic id `system`</sub>

The Persist ([3.1.3](#313-persist--cirispersist--substrate-health)) and Edge ([3.1.4](#314-transport-delivery--cirisedge--transport-delivery-reachability))
`system:*` dimensions are reserved to the substrate itself: the `attesting_key_id` MUST be a
`substrate_persist` / `substrate_edge` key, cross-attested by the steward-triple. A non-substrate
emission on these is a category error and MUST be rejected — the substrate may report on *itself*,
but nothing may impersonate the substrate. Phase 4, `legacy_ref` CEG §7.2.

### 3.4.4 `family-self` — Self/family membership-event reservations (CEG 0.7 addition)
<sub>budget 0.46pp · import #64 · from **CEG §7.7** · semantic id `family-self`</sub>

The `substrate_persist`-only membership-event prefixes for `identity_occurrence` + `family`:
`hard_case:identity_occurrence_added:*`, `hard_case:family_membership_change:*` (add **and** removal,
same `change_kind` discipline as community), the consensus-protocol change/violation pair, and the
cohort-scoped `hard_case:recipient_excluded:*` (a fail-secure-skipped at-rest grant recipient — emitted
*into* the affected self/family scope so the excluded member can audit, but MUST NOT federate beyond
it, preserving structural invisibility). Phase 4, `legacy_ref` CEG §7.7.

### 3.4.5 `capacity-score` — Capacity-Score self-emission rejection
<sub>budget 0.39pp · import #78 · from **CEG §7.5** · semantic id `capacity-score`</sub>

`capacity:*` ([3.1.8.1](#3181-capacity-score-capacity--capacity-score-factor-prefixes)) rejects
self-emission: `attesting_key_id` MUST NOT equal the attested key. An agent's own capacity score is
never fed back into its own context — the **anti-Goodhart** floor: a system cannot grade itself into
standing. Phase 4, `legacy_ref` CEG §7.5.

### 3.4.6 `reservation-delivery` — Delivery-receipt reservation (CEG 0.10 addition)
<sub>budget 0.23pp · import #126 · from **CEG §7.9** · semantic id `reservation-delivery`</sub>

`delivery_receipt:{stream_id}` — a subscriber's signed acknowledgement of a received stream chunk,
emittable only by a current member of the stream's community. **Validated, not adjudicated**: the
substrate/Verify authenticate origin and JOIN the receipt against the published STH root, but do not
compose "delivered" / "owes N" verdicts — that is consumer policy. The streaming detail is
[Part V](part_5_transport_substrate.md). Phase 4, `legacy_ref` CEG §7.9.

### 3.4.7 `enforcement` — The enforcement rule (normative)
<sub>budget 0.21pp · import #134 · from **CEG §7.0** · semantic id `enforcement`</sub>

The normative core stated at [3.4](#34-reservation--reserved-prefix-enforcement): substrate rejects at
admission, consumer re-checks independently, producer must not emit in violation regardless of
downstream acceptance — three roles, one rule, trust non-propagating. Phase 4, `legacy_ref` CEG §7.0.

#### 3.4.7.1 `identity-set` — `identity_type` is a set — single-key role cohabitation (CEG 0.9 addition)
<sub>budget 0.39pp · import #77 · from **CEG §7.0.1** · semantic id `identity-set`</sub>

The generalization that makes all the reservation rules above **set-membership tests**, not scalar
equality: `federation_keys.identity_type` is a **SET of roles** — a single key MAY simultaneously be
`agent` AND `lenscore_detector`, or `substrate_persist` AND `witness`. Every emitter rule reads
`X ∈ attesting_key.identity_type`. Crucially, **cohabitation does NOT collapse the namespace split**:
a key holding `{agent, lenscore_detector}` still emits detector verdicts under `detection:*` and agent
attestations under the agent dimensions — the [3.4.8](#348-detector-only--detector-only-prefixes)
shadowing rule and the [3.4.5](#345-capacity-score--capacity-score-self-emission-rejection) self-emission
rejection bind *per held role*. The **fabric-node discipline** is the load-bearing normative point: a
co-located node holding the full role-set is conformant **iff** separation of powers is held
*cryptographically, not procedurally* — co-location of custody is not consolidation of authority. A
`steward` role grants a *vote* in the founder-quorum, never a unilateral *verdict*; `lenscore_detector`
emissions stay non-authoritative by namespace; observation can never manufacture authority, because
the namespaces do not merge and authority is quorum-gated upstream of any single key. *The hazard the
LensCore mission warns of — the lenses becoming the gate — is structurally unreachable inside one
process.* Backward compatibility is semantic-null (a legacy scalar `"X"` is the singleton `{X}`).
Migrated verbatim in Phase 4 (`legacy_ref` CEG §7.0.1).

#### 3.4.7.2 `consent-counter` — `consent_role` — the Counter-RII consent gate (1.0-RC4, ratifies Accord §RC / CIRISAgent#760 OQ-1/2/3)
<sub>budget 0.11pp · import #233 · from **CEG §7.0.2** · semantic id `consent-counter`</sub>

`federation_keys.consent_role` — the role enum gating Counter-RII probe detection (Lean-verified,
8 theorems). Three primitive-level semantics are ratified: revocation is `BaseRole`-only and
non-recursive (no chain embedded in the role JSONB); a `Peer` role blanket-escapes detection at any
trust mode (bounded because the flag is **advisory-only** — never sole `slashing:*` evidence); a
post-window `AuthorizedReview` is signal-eligible immediately (fail-secure, no grace period).
A `federation_keys` identity field, **1+4 untouched**. Phase 4, `legacy_ref` CEG §7.0.2.

### 3.4.8 `detector-only` — Detector-only prefixes
<sub>budget 0.2pp · import #147 · from **CEG §7.4** · semantic id `detector-only`</sub>

`detection:correlated_action:*` and `detection:distributive:access:*` are **LensCore-only** emission
(`lenscore_detector ∈ identity_type`). A non-LensCore peer cross-checking the detector's verdict MUST
use a *different* prefix (`truth_grounding:detection:correlated_action:*`) to avoid shadowing the
detector's own emission — cross-checking is welcome; impersonation is not. Phase 4, `legacy_ref` CEG §7.4.

### 3.4.9 `co-owned` — Co-owned prefixes
<sub>budget 0.11pp · import #293 · from **CEG §7.3** · semantic id `co-owned`</sub>

`licensure:{authority_id}` is **co-owned** by the [Registry](#311-registry--cirisregistry--identity--build--license--partner)
and CIRISVerify — both may emit; consumers compose. A single-source attestation (only one co-owner has
emitted) MUST be marked `confidence ≤ 0.5` until the second arrives — the
[2-of-source-agreement](part_1_foundation.md#15-fail-secure--fail-secure) discipline at the namespace layer.
Phase 4, `legacy_ref` CEG §7.3.

### 3.4.10 `witness-emitter` — Witness-emitter reservations
<sub>budget 0.11pp · import #294 · from **CEG §7.6** · semantic id `witness-emitter`</sub>

`transparency_log:cosigned:*` is reserved to `identity_type = "witness"` keys — STH cosignatures come
only from witnesses. Phase 4, `legacy_ref` CEG §7.6.

---

## 3.5 `structure-inter` — Inter-attestation relations — the structural composition graph
<sub>budget 0.18pp · import #156 · from **CEG §6** · semantic id `structure-inter`</sub>

The bridge back to [Part II](part_2_the_grammar.md): attestations relate to each other in **eight ways**,
of which **four are the structural primitives** (`supersedes` / `withdraws` / `recants` /
`delegates_to`) and **four are emergent** from scalar composition — Standalone, Refers-to-prior
(via `evidence_refs[]`), Contradicts-prior (a negative score where a prior positive exists), and
Clarifies-prior (a refined re-score). The point is economy: the federation expresses *all eight*
relational stances with only the four frozen composers plus the natural arithmetic of scores — no
fifth structural primitive is needed, the [1+4 adequacy claim](part_1_foundation.md#17-minimal-and-adequate--the-14-claim)
seen from the relations side. Phase 4, `legacy_ref` CEG §6.

### 3.5.1 `concurrent-write` — Concurrent-write precedence (0.1 scaffold)
<sub>budget 0.17pp · import #169 · from **CEG §6.1** · semantic id `concurrent-write`</sub>

The deterministic tie-break when two composers race on the same `references_attestation_id`:
**`recants` outranks `withdraws` outranks `supersedes`** (a falsity admission cannot be subsumed by a
retraction or replacement); then largest `signed_at`; then lexicographically-smallest substrate
attestation_id; cross-attester chains evaluated independently. Composers are **idempotent** on
`(references_attestation_id, attestation_type, attesting_key_id)` — replay is a no-op, and the
substrate dedups on that triple. This is [integrity](part_1_foundation.md#18-integrity--integrity) under
concurrency: two honest implementations reach the *same* verdict on the same racing writes. Migrated
verbatim in Phase 4 (`legacy_ref` CEG §6.1).

---

*Part III is the federation's vocabulary — owned by no one, gated only where authority demands it
([3.4](#34-reservation--reserved-prefix-enforcement)), and carrying every content / identity / consent
/ community shape on the [frozen 1+4 surface](part_1_foundation.md#17-minimal-and-adequate--the-14-claim)
without a single new structural primitive. Its deep tail — the per-prefix field tables, the
closed-set enums (`LegalBasis`, `wrap_algorithm`, `ConsensusProtocol`), the locked subject_kind
schemas, the pinned RNS algorithm, and the full namespace lineage — is migrated verbatim in Phase 4
with full `legacy_ref` provenance ([`toc.tsv`](toc.tsv)). The importance graph keeps it page-thin here
because the federation leans on the **shape** (mechanism-named, open-by-default, reserved-where-authority-lives)
far more than on any one prefix's bytes.*
