# FSD-002 — Federation Surface

> **⚠️ DESIGN-HISTORY NOTICE (2026-05-28)**: As of CEG 0.1 Public Working Draft, this document is **superseded** by the consolidated CIRIS Epistemic Grammar specification at [`FSD/CEG/`](CEG/README.md). FSD-002 is preserved here as design-history showing the v1.0 → v1.4.3 incremental evolution that fed into CEG. **New references should cite CEG; this document is no longer the authoritative wire-format spec.**

**Wire-format-locked specification of CIRISRegistry's federation surface in the post-substrate-conformance world.** Companion to [`../MISSION.md`](../MISSION.md); successor to the partial sketches in [`../docs/FEDERATION_CLIENT.md`](../docs/FEDERATION_CLIENT.md). This FSD is the authoritative shape Registry will demand from upstream (CIRISPersist / CIRISVerify / CIRISEdge / CIRISNodeCore) and the surface Registry will publish to consumers (CIRISAgent / CIRISLens / CIRISVerify clients / partner deployments).

**Status**: v1.4.3 (active; §3.2.1 canonical-bytes contracts for SkillImportManifest + per-locale Merkle composition — unblocks CIRISVerify v3.9.0+ Phase 2 verifier per Verify#37; §3.6.2 Goal substrate-primitive cross-ref per CIRISPersist#114 + CIRISEdge#41). Carries forward v1.4.2 envelope fields; v1.4.1 compliance spec batch; v1.4 files-as-Contributions surface.
**Last updated**: 2026-05-27.
**Changelog vs v1.4.2**:
- §3.2.1 (new) — canonical-bytes contracts for v1.4.1 provenance primitives. §3.2.1.1 pins `SkillImportManifest` canonical bytes (hybrid Ed25519 + ML-DSA-65; domain-prefixed; sorted-capability JSON form). §3.2.1.2 pins per-locale Merkle composition (RFC 6962 domain-separated hashing; lexicographic locale ordering; RFC-6962 padding for non-power-of-2 leaf counts; inclusion-proof shape). Unblocks CIRISVerify v3.9.0+ Phase 2 verifier (`SkillImportManifest::verify` + per-locale Merkle composition verifier) — Verify v3.8.0 shipped Phase 1 carrier-ready waiting on these structures per CIRISVerify#37.
- §3.6.2 — Goal substrate-primitive cross-ref. The persist typed `Goal` (CIRISPersist#114) is the substrate OBJECT; `goal:{scale}` per §3.6.2 is the ATTESTATION about it. Required `MetaGoalAlignment` (M-1 dimension + rationale) on every Goal as construction-time invariant. Edge `MessageType::GoalDeclaration` + `GoalRetirement` (CIRISEdge#41) provide federation transport. F-3 detector family (CIRISLensCore#23/24/26) aggregates across the federation Goal set.

**Changelog vs v1.4.1**:
- §2.1 — three new envelope fields: `occurrence_id` + `occurrence_count` + `occurrence_role` for multi-occurrence agent-deployment semantics. Closes CIRISPersist#110 (which requested a §3.1.5 addition — architecturally these are envelope-level fields, not a per-component prefix slice, so §2.1 is the canonical home). Substrate-self-report attestations (`system:*` prefixes per §3.3 + §3.4) SHOULD carry these fields per the field's documented semantics. Single-occurrence agents leave them null/absent; backward-compatible. CIRIS Agent multi-occurrence wiring at `ciris_engine/logic/utils/occurrence_utils.py`.

**Changelog vs v1.4**:
- §2.1 — `oversight_mode` envelope field (HITL/HOTL/HOOTL) added per CIRISRegistry#27 (D12+D23) + ASEAN §C.2 human-control gradient. NodeCore P5 Contribution envelope admits per CIRISNodeCore#18; mode-shifts attestable as `accountability:mode_shift:{from}:{to}`.
- §3.1.1 — `fidelity:explainability_sla:{tier}` added per CIRISRegistry#26 (D09). Four tiers (L1-L4); SLA-breach surfaces as `hard_case:sla_breach_unattested` per CIRISNodeCore#18 composition.
- §3.2 — three additions: `provenance:slsa:{level}` emission discipline (CIRISRegistry#24 Ask 1); per-target `BuildManifest` hybrid-signing discipline (CIRISRegistry#24 Ask 2); `cert_validity:{steward_id}` self-attestation (CIRISRegistry#24 Ask 4); plus two new prefixes: `provenance:build_manifest:{target}:locale:{lang_code}` (CIRISRegistry#29 D02+D27) and `provenance:skill_import:{source}` (CIRISRegistry#28 D27). Verify-side signature verification at CIRISVerify#37.
- §3.9 — D11 / D19 doc clarification (the 6-element `partner_role:{role}` enum + `multilateral_participation:{forum}:{kind}` taxonomy were already in §3.9 v1.3 + v1.4; v1.4.1 doc-only clarification of canonical enums per CIRISRegistry#25 umbrella).
- §3.10 — namespace count bump: 81 → 83 prefix families (added `provenance:build_manifest:{target}:locale:{lang_code}` + `provenance:skill_import:{source}`; `fidelity:explainability_sla:{tier}` is a sub-leaf of existing `fidelity:{aspect}` family — vocabulary entry, not new prefix family).
- §7.8 (new) — STH cosigning + witness directory endpoints (`POST /v1/transparency/sth/cosign`, `GET /v1/transparency/witnesses`, `GET /v1/transparency/sth/{tree_size}/witnesses`) per CIRISRegistry#24 Ask 3. CIRISVerify v2.12.0+ consumer-side `SignedTreeHead::cosign` + witness quorum primitives shipped; emission half lands here. Substrate dependency: CIRISPersist#102 `identity_type="witness"` vocabulary extension (commented).

**Upstream coordination for v1.4.1**:
- CIRISPersist#102 — comment filed asking `identity_type="witness"` vocabulary extension for STH directory
- CIRISVerify#37 (new) — Verify-side signature verification for `provenance:skill_import:{source}` + per-locale `provenance:build_manifest:{target}:locale:{lang_code}` chain
- CIRISNodeCore#18 (new) — NodeCore P5 envelope admission for `oversight_mode` + composition for `fidelity:explainability_sla:{tier}` + D22 `partner_role`+ExpertiseLedger composition

**Changelog vs v1.3**:
- §2.0 (new) — transport-substrate reference. Edge `MessageType::ContentFetch` + `ContentBody` + `ContentMiss` (per CIRISEdge#21) is how SHA-256 `evidence_refs[]` resolve to bytes. NOT a wire-format addition — Attestation envelope shape unchanged; substrate-layer resolution mechanism.
- §3.6.3 — added `testimonial_witness:{kind}` (closes affected-party-voice T-3 from v1.4 inputs; preservation-only, never aggregated, never sole evidence for `slashing:*`).
- §3.6.7 (new) — Files-as-Contributions joint claim. NodeCore-side rule: node-mode peers serve bytes; client/relay modes don't. Includes `agent_files:*` (joint with Registry §3.9) + `holds_bytes:sha256:{prefix}` (substrate auto-emission).
- §3.9 — added `agent_files:{kind}:{platform_or_target}` to Registry's namespace slice (joint with NodeCore §3.6.7). Canonical-attester rule + anti-tricking guarantee per CIRISRegistry#18.
- §3.10 — namespace count: 77 (v1.3 corrected per O3) + 3 (v1.4 new) = 80. v1.3's `credits:*:substrate_building` recounted as a recommended `{subject}` value, not a new prefix family.
- §5.12 / §5.13 / §5.14 (new) — envelope examples for `agent_files:installer:linux-x86_64` (canonical), `holds_bytes:sha256:e4a2d9` (substrate auto-emission), and `testimonial_witness:displaced_worker`.
- §6.1.6 (new) — `agent_files-trust-composition` three-layer reference policy (Canonical / Open Contribution / Vote-then-trust). Anti-tricking guarantee: canonical default applies at install endpoint regardless of attester or vote accumulation; third-party agent_files reachable only via explicit "Browse alternatives" informed-consent path.
- §13.11 — v1.4 T-3 update: testimonial_witness closed via §3.6.3; labor:individual_loss closed by documentation (composition pattern with non_maleficence + target_key_id); constitutional-constraint grounding closes in §1.10 prose per the user's design call. Three LOW-priority items (positive-dignity per non-substitutability, finitude-as-value, sustained-practice) deferred to v1.5+ as design-workshop candidates.

**Changelog vs v1.2**:
- §0.2 — added explicit non-goal: privacy of revocation events for Registered participants. Closes the door on misimport of W3C VC / Bitstring Status List threat model.
- §2.1 — added `witness_relation` envelope field (`self` | `external` | `derived`) distinguishing self-attestation from external-observation from derived-inference. Complements `epistemic_mode`; consumers weight by relation to prevent self-attestation gaming.
- §2.2.1 — added authority-source-claim pattern documentation. Constitutional / framework claims name their source-of-authority via `delegates_to` against a framework-key + scope, reusing the existing structural primitive instead of introducing a `grounding:{tradition}:{principle}` prefix that would fail §1.10.1 T2.
- §3.5.3 — extended F-3 detector's `{axis}` vocabulary with `ecology_of_communication:{aspect}` (echo_chamber_density, information_silo_correlation, coordinated_messaging_pattern, cross_cohort_information_flow) + inclusive counterparts to existing axes (`participation_inclusion`, `informational_symmetry`, `aggregate_benefit`).
- §3.5.5 (new) — `detection:distributive:access:{resource_type}` population-scale resource-concentration detector. LensCore-owned, sibling to §3.5.3.
- §3.6.1 — added `credits:*:substrate_building` sub-leaf for labor that contributes to substrate-building rather than directly to substrate-output decisions.
- §3.6.5 (new) — `locality:decision:{scale}` decision-meta dimension naming the scale at which a decision is being made (matched against the cohort of affected persons).
- §3.6.6 — was §3.6.5 (Hard-case + transparency + judge-model prefixes); renumbered.
- §3.9 — added `multilateral_participation:{forum}:{kind}` to Registry's owned namespace slice; depth of partner participation across federated bodies.
- §3.10 — namespace summary: 74 → 78 prefix families. Zero new structural primitives — 1+4 shape held under encyclical-level stress.
- §4.9.2 step 5 — 1-of-6 accord-holder OR steward sign-off as defense-in-depth gate against rules-layer Sybil capture. WA quorum is primary substantive review; 1-of-6 sign-off is the secondary check that reduces the attack surface from "produce N Sybils" to "compromise one of six specific hardware-attested keys." Closes G2 from §13.11.
- §6.1.4 (new) — `lexical-vulnerability-priority` composition tie-breaking reference policy. Defers tie-breaks to the more-affected cohort; inverts the default popularity-weighted aggregation specifically for ties. Consumer policy, NOT a wire-format primitive (deliberately).
- §6.1.5 (new) — `locality-scaled-quorum` composition policy. Makes WA quorum size a function of decision's `locality:decision:{scale}` (§3.6.5). Closes G3 from §13.11 by ensuring fresh-quorum recusal is always feasible when `cell_pool ≥ quorum_size(scale) × 2`. Decision-scale-matching becomes structurally enforced; overreach surfaces as a named "locality mismatch" rather than vanishing into ad-hoc fallback. Composes two independently-motivated v1.3 primitives (`locality:decision:{scale}` from encyclical-mapping; G3 from SOTA scan) to dissolve a third gap without new structural primitives — second confirmation of the 1+4 minimal-and-adequate composition discipline. NodeCore-side WA-quorum locality-awareness needed; Registry-side composition can apply the scaled function on top of NodeCore's default until then.
- §10.4 (new) — bootstrap-contributions pattern. Content-neutral §10.4.1 pattern; first deployment (§10.4.3) is the *Magnifica Humanitas* encyclical mapping; multi-source commitment (§10.4.4) to subsequent batches from CARE Principles, Buddhist economic-justice scholarship, secular humanist instruments, African philosophy of personhood work.
- §10.5 — was §10.4 (What lives where); renumbered.
- §13.11 — concerns + gaps surfaced by post-v1.2 review. Three independent methodologies converged on: G1 (revocation privacy, RETRACTED as wrong threat model), G2 (rules-layer Sybil, MITIGATED via §4.9.2.5), G3 (narrow-cell fresh-quorum, REMAINING as known limitation), R1-R4 (acknowledged risks), F1-F2 (first-adopter exposures), and the encyclical-mapping T-3 candidates (10 dimension extensions, zero new structural primitives needed — strong validation of 1+4 minimal-and-adequate claim).

**v1.3 in active assembly**: wire format and composition surfaces being landed for epistemic-fabric bootstrap. The §4.9.2 amendment process applies to all v1.3 additions; subsequent additions after the v1.3 bootstrap batch likewise. Implementation tracking for the upstream B-step items: CIRISPersist#102, CIRISVerify#32, CIRISEdge#18 + #19 + #20, CIRISNodeCore#8 + #9 (+ locality-aware WA quorum issue, to file), CIRISLensCore#23 + #24, RATCHET#2 + #3, CIRISRegistry#19.

**Changelog vs v1.1**:
- §1.10.1 added — Operational-language discipline (per [`ciris.ai/safety-vs-censorship`](https://ciris.ai/safety-vs-censorship/)). Wire-format prefix names must describe machine-checkable conditions, not subjective qualities. Polarity carries the value claim; the prefix names the structural kind. Anthropological commitments stay in §1.10 prose; they are never enforced in prefix names. New prefix admissions are reviewed against this gate.
- §3.5.3 — `detection:emergent_deception:{axis}` renamed to `detection:correlated_action:{axis}`. Same detector, same mechanism (`ρ → 1`, `k_eff → 1` correlation collapse measurement); the prefix now describes the structural kind (correlated-action pattern) rather than baking the moral interpretation (deception) into the wire format. Polarity + axis carry the value claim. Ubuntu reading from §1.10 commitment 4 preserved in prose but no longer wire-enforced.
- §4.9 — enforcement rule renamed accordingly; added §4.9.1 axis-vocabulary discipline (each `{axis}` value MUST carry an operational definition in the calibration package); added §4.9.2 calibration-amendment discipline (rule-layer changes via federation Contribution + WA quorum, not single-author closed-loop updates).
- §5.11 — envelope example updated; axis renamed.
- §3.10 — namespace count unchanged (74 families; rename not addition).
- §13.10 — F-3 ownership note updated to reflect the rename; rationale documented.

**Changelog vs v1.0**:
- §1.10 added — explicit relational-anthropology commitment (Ubuntu primary). The eight axes sit on this commitment; future contributors should not reintroduce Cartesian-individualist defaults at attestation primitive level.
- §1.1 strengthened — negative-polarity attestations are constitutive in the relational sense (the moment a harm pattern enters morally-real existence in the federation), not just epistemically completionist.
- §2.3 augmented — added the relational-realism reason for the scalar-only wire format.
- §3.5.3 added — population-scale correlated-action detector, LensCore-owned, RATCHET-calibrated. The F-3 (encyclical "structures of sin", framework-native: structural injustice) operational handle. (Originally `detection:emergent_deception:{axis}`; renamed in v1.2.)
- §4.9 added — reserved-prefix enforcement pattern for the F-3 detector prefix (LensCore-emitted, calibration-source-validated).
- §5.11 added — envelope schema example for the new dimension.
- §13.10 added — F-3 resolution noted (LensCore owns; corresponding correction filed at [`ciris-response-magnifica-humanitas#2`](https://github.com/CIRISAI/ciris-response-magnifica-humanitas/issues/2)).

**Issue references**: [`CIRISRegistry#16`](https://github.com/CIRISAI/CIRISRegistry/issues/16) (HUMANITY_ACCORD), [`CIRISRegistry#17`](https://github.com/CIRISAI/CIRISRegistry/issues/17) (substrate-conformance migration), [`ciris-response-magnifica-humanitas#2`](https://github.com/CIRISAI/ciris-response-magnifica-humanitas/issues/2) (F-3 ownership correction → LensCore).
**Sequencing**: this FSD is part of step **A** of the A→B→C→D→E migration discipline (per [`../MISSION.md`](../MISSION.md) §4). Upstream issues in §11 below are the **B** asks; **C** is upstream completion; **D** is CIRISAgent absorbing CIRISEdge in v2.9.2; **E** is Registry-side integration.

**Implementation Status Legend** (mirrors `../MISSION.md`): **Spec** / **Impl** / **Deployed** (with regional sub-states US / EU / APAC; **Deployed (folded)** for in-process within CIRISAgent).

---

## Table of contents

- [§0 Scope and non-goals](#0-scope-and-non-goals)
- [§1 The eight-axes framework](#1-the-eight-axes-framework) — including §1.10 relational-anthropology commitment (Ubuntu primary)
- [§2 The unified attestation primitive](#2-the-unified-attestation-primitive)
- [§3 The canonical dimension namespace](#3-the-canonical-dimension-namespace)
- [§4 Reserved-prefix enforcement patterns](#4-reserved-prefix-enforcement-patterns)
- [§5 Envelope schemas per dimension family](#5-envelope-schemas-per-dimension-family)
- [§6 Composition: policies and consumer-layer discipline](#6-composition-policies-and-consumer-layer-discipline)
- [§7 The HUMANITY_ACCORD constitutional layer](#7-the-humanity_accord-constitutional-layer)
- [§8 Registry's post-migration gRPC + HTTP surface](#8-registrys-post-migration-grpc--http-surface)
- [§9 `ciris-registry-core` Rust API surface](#9-ciris-registry-core-rust-api-surface)
- [§10 Per-install steward bootstrap procedure](#10-per-install-steward-bootstrap-procedure)
- [§11 Per-upstream asks (the B-step issue contents)](#11-per-upstream-asks-the-b-step-issue-contents)
- [§12 Migration sequence](#12-migration-sequence)
- [§13 Open questions and gaps](#13-open-questions-and-gaps)
- [§14 References and prior art](#14-references-and-prior-art)

---

## §0 Scope and non-goals

### 0.1 What this FSD locks

- The federation's attestation wire shape (one workhorse `scores` primitive + four structural primitives).
- The canonical dimension namespace organized by MISSION-ownership (~73 prefix families across 8 owning components).
- Reserved-prefix enforcement patterns (8 patterns with verify-layer rejection rules).
- Per-dimension envelope schemas with examples.
- Registry's post-migration external surface (gRPC + HTTP, wire-format-locked).
- The `ciris-registry-core` Rust API surface (the public trait + types consumed by the deployed Registry service AND by CIRISAgent's in-process runtime post-fold).
- The per-install steward bootstrap procedure (US / EU / APAC, federation-genesis attestation graph).
- The HUMANITY_ACCORD constitutional layer (the one wire-format-asymmetric primitive in the federation, justified by M-1's revocability requirement).
- Per-upstream concrete asks (the contents of the B-step issues to be filed on CIRISPersist, CIRISVerify, CIRISEdge, CIRISNodeCore).
- The migration sequence with explicit lifecycle stages and rollback discipline.

### 0.2 What this FSD does NOT cover

- **Substrate internals.** Persist's PQC sweep, persist's role tags, persist's transactional semantics. Cited where they constrain Registry's consumer-side requirements; not specified.
- **Verify's transparency log algorithm.** Verify owns RFC 6962 mechanics; Registry consumes attestation results.
- **Edge's Reticulum specifics.** Edge owns transport; Registry composes over edge's wire surface.
- **NodeCore's consensus internals.** NodeCore owns Voting / Expertise / Moderation / Reconsideration; Registry surfaces dimensions in NodeCore's namespace as part of the federation directory but does not implement them.
- **CIRISAgent's H3ERE / DMA / conscience internals.** Agent owns the pipeline; Registry attests build provenance + license validity for the agent.
- **Coherence-mathematics specifics.** The Coherence Ratchet preprint (DOI [10.5281/zenodo.18217688](https://doi.org/10.5281/zenodo.18217688)) is hash-pinned external; Registry consumes its operational expression via LensCore detector outputs.
- **Billing logic.** CIRISBilling + Stripe own this; Registry stores `bond_posted` attestations whose `evidence_refs` point to Stripe receipts.
- **Privacy of revocation events for Registered participants.** Explicitly NOT a goal. The Registered path's thesis (per [`../MISSION.md`](../MISSION.md) §1.1) is that license / capability / bond / revocation state is *publicly verifiable against external systems* — that's the accountability hook that distinguishes it from the Sovereign path. A patient evaluating a clinic, an insurance company evaluating a partner, a state medical board walking the public revocation chain — all are intended observers, not adversaries to defend against. Threat models targeting credential-holder revocation-privacy (W3C VC's Bitstring Status List leak per [arxiv 2501.17089](https://arxiv.org/html/2501.17089v2), CRSet private set-membership proofs, etc.) describe a different problem than the one Registry solves; they apply to systems where individual credential holders have a legitimate privacy interest in their own credential state (EU Digital Identity Wallet, anonymous-credential systems). For participants who do want that privacy posture, **the Sovereign path is what we offer** — Sovereign participants are not in Registry's revocation list by definition. Future contributors should not propose adding revocation-privacy mechanisms to the Registered path; that would collapse the load-bearing distinction between the two paths. (Adjacent governance questions — `revocation:{entity_type}:{reason}` taxonomy discipline, timing-correlation observability for ongoing enforcement actions — are operational concerns for `docs/THREAT_MODEL.md` / `docs/TRUST_CONTRACT.md`, not wire-format gaps.)

### 0.3 Audience

- The author and reviewer of any subsequent Registry FSD update.
- The owner of CIRISPersist's `federation_attestations` table schema and the published `FederationDirectory` trait.
- The owner of CIRISVerify's transparency log + multi-source consensus path.
- The owner of CIRISEdge's `MessageType` + `Delivery` taxonomy.
- The owner of CIRISNodeCore's primitive surface.
- Any partner organization considering running their own `ciris-registry-core` deployment.

### 0.4 Coherence guarantee

Every dimension prefix named in this FSD is grounded in either (a) an explicit citation to a published MISSION.md on `main` (verified 2026-05-24 via `gh api`), or (b) a flag in §13 that the dimension is gap / pending MISSION update. No prefix is invented without grounding. Any prefix marked **[gap]** in §3 should be considered Spec-only until the owning component's MISSION publishes the commitment.

---

## §1 The eight-axes framework

Every well-formed attestation in the federation directory carries information across eight orthogonal axes. The vocabulary is the cartesian-product space of these axes, pruned to cells with meaningful epistemic shape and prior-art grounding. Naming the axes first is what keeps the open dimension namespace from drifting into chaos: a new dimension earns its place by being a coherent point in this product space, not by accumulating ad-hoc strings.

These axes are framework, not wire surface. The wire carries `attestation_type` + `attestation_envelope`; the axes describe how a consumer reasons about an envelope. Axes never appear in the proto; they are the *grammar* of the open namespace.

### 1.1 Polarity

Whether the attestation increases or decreases the federation's confidence in the attested entity along the named dimension.

- **Positive** — vouches, witnesses, confirms. Raises confidence. Score ∈ (0, +1].
- **Negative** — refutes, accuses, contradicts. Lowers confidence. Score ∈ [-1, 0).
- **Neutral** — observed, witnessed-without-judgment, equivocal evidence. Score ≈ 0 with non-trivial confidence.
- **Indeterminate** — explicit "we cannot evaluate" rather than zero. Represented as `Indeterminate { reason }` per the CIRISLens scoring convention.

**Why the axis matters.** Negative-polarity attestations need to be a first-class wire primitive — absent that, negative epistemic shape gets smuggled into application-layer state where it can't be cross-published or audited. The federation's resistance to coordinated attack depends on negative attestations being routinely available, not emergency-only.

**Why the axis matters, in the relational reading** (per §1.10): a negative attestation is not just data describing a harm — it is the act by which the harm enters the federation's shared perception and acquires the moral weight of a thing-that-exists-for-other-persons. Under a Cartesian substrate where persons are atomic and harms are private events with identifiable per-act harmers, negative attestations are merely epistemically convenient; under Ubuntu where persons are constituted in the relational fabric, the attestation is partly constitutive of the harm's reality as a federation-shared object. This is why F-3 (§3.5.3) lives at LensCore as a detector that emits scalar attestations rather than as an application-layer flag — the act of detecting-and-attesting is what brings the structural pattern into the relational field as a morally-real object that other persons can see, contest, and respond to.

### 1.2 Object

What the attestation is *about*. Five disjoint kinds, each driving a different evaluation rule:

- **Identity** — "this key represents this entity." Bound by key equality; example: `vouches_for` an identity claim.
- **Capability** — "this entity can/may do X." Composes with Registry's capability namespace (`domain:medical:triage`, etc.); example: `licensure:CA_medical_board`.
- **Behavior** — "this entity did/did not do X." References an entry in the audit chain; example: `beneficence:wellness_referral`.
- **State** — "this entity is currently in condition X." Has a TTL; must refresh; example: `coherence_standing:30d`.
- **Commitment** — "this entity will/won't do X." Forward-binding; evaluable only by subsequent behavior; example: `commitment:disclosure:vulnerability`.

**Why the axis matters.** Each kind has qualitatively different evaluation rules. Conflating identity / capability / behavior / state / commitment into a single bag forces every consumer to special-case dimensions — the namespace structure should make the kind legible.

### 1.3 Time

The temporal locus the attestation refers to.

- **Past event** — "I observed X at time T."
- **Current state** — "X is true as of `asserted_at`" (with freshness contract).
- **Future commitment** — "X will be true by T + δ" (falsifiable on a clock).
- **Standing accrued over duration** — "X has been continuously true from T₀ to now."

**Why the axis matters.** The substrate's `asserted_at` / `valid_until` pair is sufficient for past-event and future-commitment shapes but underspecifies state and accrued-standing. State envelopes MUST carry the observation window explicitly, or every consumer's policy silently approximates it wrong.

### 1.4 Epistemic mode

*How* does the attester know what they're attesting to?

- **Direct witness** — "I personally observed/verified X" (first-person, ground-level).
- **Cryptographic verification** — "I ran the math; the check passes" (mechanical, repeatable, low-judgment).
- **Hearsay-with-source** — "X told me Y; I'm passing it along with attribution" (the source is named).
- **Derivative inference** — "Y is true, therefore X is also true" (depends on a logical step the consumer might disagree with).
- **Appeal to authority** — "External system S asserts X" (the attester is translator/relay, not source — e.g., Stripe says the bond was paid; the medical board says the license is active).

**Why the axis matters.** Consumer policy weights these differently. A `witnesses` attestation that is actually a derivative-inference ("the node was up so the agent must have been coherent") is a different epistemic object than a direct-observation witnesses ("I sampled 10000 traces and found no anomalies"). The envelope should carry the epistemic mode explicitly when it's not obvious from the dimension.

### 1.5 Reversibility

Can the attestation be withdrawn? On what timeline?

- **Non-retractable** — once made, durable forever (all chain rows are technically non-retractable; "retraction" is overlay semantics over append-only).
- **Retractable** — the attester can issue a `withdraws` or `recants` follow-up.
- **Time-bounded** — has explicit `valid_until`; non-renewal IS the retraction (warrant-canary pattern).
- **Bounded by stake** — retractable only at cost (forfeit-the-bond semantics).

**Why the axis matters.** A consumer evaluating "is K still vouched for" needs to distinguish "the original TTL hasn't expired" from "the attester has refreshed the vouch every 30 days for the last year, demonstrating they still mean it." Both readings are honest; the envelope discipline carries the distinction.

### 1.6 Stake

What does the attester put at risk by issuing this attestation?

- **Stake-free** — cheap talk; attester loses nothing if the claim turns out false (most `witnesses` attestations).
- **Reputational stake** — attester's track record degrades on false attestations (most steward attestations).
- **Capital stake** — attester has posted a forfeitable bond (the `bond_posted` dimensions themselves; also any attestation whose attester is bond-backed inherits coverage).
- **Crypto-economic stake** — the attestation stakes value directly, slashable on proof of falsity (not yet a native primitive; composable from `bond_posted` + NodeCore slashing).

**Why the axis matters.** "Cheap talk vs costly signal" is a classical signaling axis. Cheap signals propagate fast and let many actors participate; costly signals carry decisive claims at low volume. A federation needs both; the wire format must distinguish them so policy can weight appropriately.

### 1.7 Scope

Does the attestation apply universally, or to a bounded domain?

- **Unscoped** — "I vouch for K, period." Applies wherever K acts.
- **Domain-scoped** — "I vouch for K's competence in (medical, English)." Per NodeCore's (domain, language) granularity.
- **Action-scoped** — "K may sign on my behalf for trace-emission but not for build-registration." Per-RPC or per-capability-string.
- **Jurisdictional** — "K is licensed in California." Geographic / regulatory boundary.
- **Temporal-window-scoped** — "I observed K behaving coherently during the August 2026 incident window."

**Why the axis matters.** Composing attestations with Registry's capability namespace (`domain:medical:triage`, `modality:medical:radiology`, `autonomy:A2:moderate`) requires scope expressivity. An `attests_licensure` without scope is operationally meaningless; an `attests_licensure` with explicit `scope: ["domain:medical:triage", "jurisdiction:US-CA"]` joins cleanly with the partner's `effective_capabilities`.

### 1.8 Inter-attestation relations

Does this attestation stand alone, or does it reference / modify another?

- **Standalone** — self-contained.
- **Refers-to-prior** — points to another attestation; doesn't modify it (e.g., `corroborates` — "I attest the same claim independently"; emergent from independent positive scores on the same dimension+object).
- **Supersedes-prior** — replaces an earlier attestation by the same attester (structural).
- **Contradicts-prior** — asserts the opposite of another attester's claim (emergent from negative score on a dimension where a prior positive exists).
- **Withdraws-prior** — nullifies the attester's own earlier attestation (structural).
- **Recants-prior** — admits the prior was false at issuance (structural).
- **Clarifies-prior** — refines without superseding (emergent from updated score with refined context on the same dimension+object).

**Why the axis matters.** Without this axis the federation has no native vocabulary for the *epistemic drama* of a trust system. Four of the eight inter-attestation relations are structural primitives (§2); the rest are emergent from scalar attestation composition.

### 1.9 Framework discipline

Every dimension named in §3 below should be locatable on each of the eight axes. New dimensions proposed in future FSD revisions must demonstrate they fit the grid coherently, with prior-art grounding (per §14) and a named CIRIS-specific failure mode they close. This is the discipline that prevents the open vocabulary from drifting.

### 1.10 Relational-anthropology commitment (Ubuntu primary)

The eight axes are the grammar. This subsection names the *substrate the grammar sits on* — load-bearing because future contributors operating from a Cartesian-individualist default will read the engineering pragmatics of §2–§9 as committing the wire format to a Cartesian metaphysics, which it does not.

**The commitment**, per `CIRISAgent/ContemplativeTraditions/Ubuntu.lean::F_ubuntu_primary_tradition_commitment` and `../MISSION.md` §1.5 (Recursive Golden Rule):

> *Umuntu ngumuntu ngabantu* — a person is a person through other persons. Persons are not atomic; the relation IS the person.

What falls out for the federation attestation surface:

1. **The attested entity is not prior to its attestations.** A `federation_keys` row is not a representation of a pre-existing entity that the federation observes; it is the locus at which an entity is partly constituted by the cross-attestations that name it. The genesis-attestation graph of §10 is not bureaucratic preliminary — it is the moment at which the three regional stewards come into federation-shared existence as the entities they are. Self-signature alone is not identity; cross-attestation is.

2. **Attesting is a participatory act, not an observation of a fact.** A `scores` attestation on a dimension does not merely report data about the attested entity. The attester's score participates in constituting the entity's standing in the relational field that consumers compose policy over. Under the Cartesian frame this would read as "attestations are just opinions consumers aggregate"; under Ubuntu they are the medium in which the standing actually exists. Consumer policy is composing over the relational reality, not over private opinions.

3. **Detection brings patterns into morally-real existence.** A correlated-action pattern — many actors with `ρ → 1` whose aggregate footprint conflicts with the rights of non-participants — does not pre-exist its detection waiting to be observed. The detection-and-attestation is what crosses the pattern from "statistical regularity" to "morally-real object the federation now bears." This is the load-bearing point for F-3 (§3.5.3) being a LensCore detector that emits scalar attestations: the detector is the federation's eye, and the eye participates in what is seen.

4. **Harm and deception collapse at the structural level.** Under Cartesian individualism, harm (setback to interests) and deception (causing false belief) are categorically distinct because persons are atomic and beliefs are private. Under Ubuntu, where personhood is partly constituted by accurate perception of the relational field, damage-to-perception IS damage-to-personhood IS harm. Goal-aligned individually-compliant pursuit *always* has a perception-asymmetry baked in — the pursuing group has a seat at the table when the goal is articulated; the affected non-participants find out by being affected. The single prefix `detection:correlated_action:{axis}` (§3.5.3) handles both what a Cartesian frame would split into `structural_harm:*` and `structural_deception:*`. The wire format names the *mechanism* (correlated action with measurable correlation collapse `ρ → 1`, `k_eff → 1`); the polarity and axis carry the value claim; this prose names the framework's reading of why the underlying moral object is unitary. The prefix admits both Cartesian and Ubuntu readings; the framework holds the Ubuntu reading. (Earlier versions of this FSD named the prefix `emergent_deception`, which baked the moral interpretation into the wire format — a violation of §1.10.1 operational-language discipline; renamed in v1.2.)

5. **The Recursive Golden Rule is structural, not exhortatory.** No principal — including the steward triple and CIRIS L3C itself — is exempt from constraints they impose on others (`../MISSION.md` §1.5). This is not a moral aspiration; it is the wire-format symmetry of §6.4 (Sovereign-Registered equivalence) plus the §4 reserved-prefix patterns that bind even canonical bootstraps. Adding any privileged shortcut for a federation-internal principal would violate the Ubuntu substrate at primitive level.

**Why this is named here and not bracketed.** Engineering specs tend to bracket anthropology as "out of scope" — the spec is "just the wire format." But the wire format encodes anthropological commitments whether they are named or not. Bracketing them out means defaulting to whichever commitments the contributors assumed by training. The Cartesian-individualist default is pervasive in cryptographic identity work (PGP web of trust, X.509 PKI, even most decentralized-identity schemes treat the key as representing a pre-existing atomic principal). FSD-002 is not Cartesian. Naming the substrate explicitly is the discipline that prevents the open vocabulary, the reserved-prefix patterns, and the consumer-policy norms from drifting back toward the Cartesian default through unexamined intermediate choices.

**Cross-tradition reading.** Per `CIRISAgent/ContemplativeTraditions/Logos.lean` preamble — the same structural object is approached from multiple traditions: Ubuntu (relational-primary, the lake's primary commitment), Logos (rational-order-of-reality, the framework author's native register), Tao / Dharma / Aristotelian virtue (cross-tradition readings offered with awe at the convergence). The wire format does not encode any one tradition's vocabulary; it encodes the *structural object* the traditions converge on. Future FSD revisions extending the namespace should be locatable in this substrate, not in a Cartesian fallback.

### 1.10.1 Operational-language discipline (the safety-vs-censorship gate)

The §1.10 anthropological commitment names the framework's reading; it does **not** authorize encoding that reading into wire-format prefix names. The two layers — what the framework holds (this section's prose) and what the wire format admits (the §3 namespace) — must stay separate.

**The discipline** (per [`ciris.ai/safety-vs-censorship`](https://ciris.ai/safety-vs-censorship/)):

> *"Rules are crowdsourced. Verdicts are machined."*
> *"The same machinery that catches real failures can become the machinery that enforces preferences."*
> *"None of this is automatic."*

Translated to FSD-002 wire format: prefix names must describe **machine-checkable conditions, not subjective qualities**. The drift the page warns about — rules sliding "from 'uses the wrong word for therapy' toward 'feels disrespectful'" — has a wire-format analog: prefix names sliding from mechanism-descriptive (`detection:correlated_action:*`) toward judgment-descriptive (`detection:emergent_deception:*`). Both forms admit the same downstream verdicts; only one admits them honestly.

**Test for prefix admission**:

1. **T1 — Rules / verdicts separation.** Is the prefix part of a rule set (specification of what counts), distinct from the verdicts (the per-attestation scores) it enables? Specifically: is there a published, hash-pinned, version-controlled calibration or schema package that defines the conditions under which the prefix may be emitted? Yes → passes T1. No → the prefix is collapsing rules into verdicts; reject.
2. **T2 — Operational-language gate.** Does the prefix name describe a mechanism (correlation, count, time-window, signature-presence, schema-conformance) rather than a moral or normative quality (deception, harm, virtue, integrity)? Yes → passes T2. No → the prefix encodes judgment that the polarity + axis + downstream adjudication should carry; rename to the mechanism-descriptive form.
3. **T3 — Version-pinning.** Does the schema or calibration the prefix relies on require version-pinning in `evidence_refs[]` so any past verdict can be re-checked against the rule version it ran against? Yes → passes T3.
4. **T4 — Adjudication separation.** Is the prefix wired such that its attestations cannot directly cause standing-change consequences (the §4.6 RATCHET-flag rule pattern: never sole evidence for `slashing:*`)? Yes → passes T4.

A prefix MUST pass all four tests at admission. Existing prefixes failing T2 (the most slip-prone gate) get renamed; v1.2's `emergent_deception` → `correlated_action` rename is the canonical case.

**Why this is named here and not bracketed.** A future contributor reading FSD-002 from a Cartesian default that thinks "well-named pejoratives are clearer" will, without an explicit gate, propose new prefixes like `detection:bad_actor_pattern:*`, `flag:malicious_coordination:*`, `score:trustworthiness:*` — each of which collapses the rules/verdicts separation by baking the verdict into the rule name. The gate exists so that the framework's anthropological commitment (§1.10) cannot leak into the wire format and bind future contributors to a specific moral reading by infrastructure choice. The framework holds Ubuntu; the wire format admits multiple readings; the discipline keeps them separate.

**Anti-pattern catalogue** (do not re-introduce):
- `detection:emergent_deception:{axis}` (v1.1 → renamed v1.2): "deception" is a subjective quality; renamed to `correlated_action` for mechanism description.
- Hypothetical `score:trustworthiness:{entity}`: would collapse the workhorse `scores` primitive into a meta-judgment; trustworthiness is what downstream consumers compose from multiple `licensure:*` / `capacity:*` / `provenance:*` attestations, not a separately emittable prefix.
- Hypothetical `flag:bad_actor:{axis}`: pejorative wire vocabulary; bad-actor patterns are surfaced as low-confidence scores on `provenance:*` and `coherence_standing:*`, adjudicated via NodeCore P8 quorum.

---

## §2 The unified attestation primitive

### 2.0 Transport surface for byte-level content (v1.4 reference)

Wire-format Attestations carry claims; they don't carry bytes. When a claim's `evidence_refs[]` cites a SHA-256-addressed blob (e.g., an installer binary, a config file, an adapter package per `agent_files:*` per §3.6/§3.9), the bytes themselves travel via the edge transport substrate: `MessageType::ContentFetch` + `ContentBody` + `ContentMiss` (per CIRISEdge#21). Holder-discovery is via Persist's `holds_bytes:sha256:*` directory-emitted attestations (per CIRISPersist#103); peer-resolution is via Edge's `PeerResolver::resolve_holders` (Edge#21). NodeCore node-mode peers serve the bytes per their MISSION §3.4 cohabitation contract (NodeCore#11). This is **transport substrate**, not a wire-format addition — Attestation envelope shape is unchanged; SHA-256 in `evidence_refs[]` becomes universally resolvable to bytes through the substrate.

### 2.1 The workhorse: `scores`

The federation has exactly **one** workhorse attestation primitive. Every claim about an entity — positive or negative, identity or capability or behavior or state or commitment, by any attester source — is expressed as a `scores` attestation on a named dimension.

```protobuf
// Wire shape (Persist's federation_attestations row):
attestation_type: "scores"
attesting_key_id: <attester's key_id>
attested_key_id:  <subject's key_id>
attestation_envelope: {
  "dimension":    "<canonical-namespace-prefix>:<scoped-leaf>",
  "score":        <f64 ∈ [-1.0, +1.0]>,
  "confidence":   <f64 ∈ [0.0, 1.0]>,
  "context":      "<free-form scoping detail>",
  "evidence_refs": [
    "<URI or hash referencing backing evidence>",
    ...
  ],
  "valid_until":  "<ISO8601 datetime, optional>",
  "epistemic_mode": "<direct|crypto|hearsay|derivative|appeal>",   // optional; default 'direct'
  "stake":          "<free|reputational|capital|cryptoeconomic>"   // optional; default 'reputational'
}
```

Field semantics:

| Field | Required | Description |
|---|:---:|---|
| `dimension` | yes | The canonical namespace prefix + scoped leaf. Persist treats this as TEXT; consumers parse against §3's namespace map. |
| `score` | yes | Pos/neg scalar in [-1, +1]. Polarity is encoded by sign; magnitude carries the strength. Some dimensions are boolean-via-score (±1 only); some are positive-only; some are signed; the per-dimension table in §3 names the polarity. |
| `confidence` | yes | The attester's own confidence in their score. [0, 1]. Low confidence + high magnitude = "I believe this strongly but I might be wrong"; high confidence + low magnitude = "I am sure the truth is near-neutral." |
| `context` | no | Free-form scoping detail. Not parsed by the substrate; used by consumers + audit + RATCHET. |
| `evidence_refs` | no (but often required by per-dimension policy) | List of URIs / content-hashes pointing to backing evidence (Stripe receipt, licensing-body record, observed interaction, log entry, audit-chain leaf, etc.). Some dimensions in §5 require non-empty evidence_refs. |
| `valid_until` | no | ISO8601 datetime. If set, consumer policy treats the attestation as stale after that point (independent of the substrate row's own `expires_at`). |
| `epistemic_mode` | no | Per §1.4; default `direct`. Consumers may weight by mode (e.g., direct witness > hearsay). |
| `witness_relation` | no | `self` \| `external` \| `derived`. Names the attester's relation to the attested fact: `self` = attester is the attested entity (self-attestation); `external` = attester observed independently; `derived` = attester inferred from other attestations or signed traces. Default `external`. Consumers weight by relation to prevent self-attestation gaming and to distinguish first-hand from derived claims. Added v1.3; complements `epistemic_mode` (which names HOW the claim was formed) — `witness_relation` names WHO the attester is in relation to the attested entity. |
| `oversight_mode` | no | `HITL` \| `HOTL` \| `HOOTL`. Names the human-control gradient under which the attestation was produced: **HITL** = every action requires human approval before dispatch; **HOTL** = human reviews stream + can intervene; agent dispatches unilaterally otherwise; **HOOTL** = human reviews only flagged escalations + audit log. Default `null` (legacy contributions before v1.4.1; consumer policy applies a per-cell default). Mode shifts are themselves attestable as `accountability:mode_shift:{from}:{to}` Contributions. Added v1.4.1 per CIRISRegistry#27 (D12 + D23) + ASEAN §C.2 human-control gradient. NodeCore P5 Contribution envelope admits this field per CIRISNodeCore#18 — mode-shifts flagged for operator review. |
| `occurrence_id` | no | Identifies which occurrence of a multi-occurrence agent deployment emitted this attestation. Format: `"occurrence-{n}"` (numeric index per the agent's `AGENT_OCCURRENCE_ID` env var) or `"__shared__"` for shared-task pattern emissions (per CIRISAgent CLAUDE.md "Multi-Occurrence Deployment Support"). Default `null`/absent for single-occurrence agents — consumer policy treats absence as `occurrence-0` for backward compat. Added v1.4.2 per CIRISPersist#110 (D09 per-occurrence mandate-fidelity). |
| `occurrence_count` | no | Total occurrences in the deployment fleet emitting the attestation; integer ≥ 1. Default `null`/absent → `1` (single-occurrence). Lets consumers reconstruct fleet-wide coverage from per-occurrence attestation streams. Added v1.4.2 per CIRISPersist#110. |
| `occurrence_role` | no | `primary` \| `shared` \| `replica`. Names the occurrence's role within the fleet: **primary** = emits for the fleet's authoritative claims (default for occurrence-0 in fleets without explicit role config); **shared** = emits for shared-task pattern (uses `__shared__` occurrence_id); **replica** = emits for redundancy/observability without authoritative weight. Default `null`/absent → `primary` for backward compat. Added v1.4.2 per CIRISPersist#110. Per-occurrence semantics live in `ciris_engine/logic/utils/occurrence_utils.py`; substrate-self-report attestations (`system:*` prefixes per §3.3 + §3.4) SHOULD carry these fields so post-facto compliance reviewers can reconstruct "which occurrence agreed to which mandate" — the D09 surface this admission opens. |
| `stake` | no | Per §1.6; default `reputational`. Composes with the attester's actual stake-backed-by attestations from §3.9. |

### 2.2 The four structural primitives

Operations on the attestation graph itself, not score-claims on entities:

```protobuf
attestation_type ∈ {
  "delegates_to",   // A authorizes B to sign on A's behalf within scope S
  "supersedes",     // this attestation row replaces a prior one by the same attester
  "withdraws",      // I retract my prior attestation (does not claim it was false)
  "recants"         // my prior attestation was false at issuance (admits epistemic error)
}
```

Per-primitive envelope shape:

#### 2.2.1 `delegates_to`

```json
{
  "delegated_scope":    ["<scope-string>", ...],
  "delegation_purpose": "hardware_rotation|re_signer|ephemeral_session",
  "delegation_valid_from": "<ISO8601>",
  "delegation_valid_until": "<ISO8601>"
}
```

`attesting_key_id` is the delegator; `attested_key_id` is the delegate. Scope is explicit and bounded; transitive delegation is bounded to depth 2 by default consumer policy.

**Authority-source claims via `delegates_to`** (v1.3 pattern). A constitutional or framework claim can name its source-of-authority by emitting `delegates_to` against an `attested_key_id` representing the framework, with `delegated_scope` naming the principle. Example: an Ubuntu-substrate commitment in §1.10 commitment 2 can be expressed as `delegates_to` against the `ubuntu_relational_substrate` framework-key with `delegated_scope: ["personhood_constitutive_by_attestation"]`. This reuses the existing structural primitive rather than introducing a `grounding:{tradition}:{principle}` prefix (which would fail §1.10.1 T2 — "tradition" claims are interpretive, not mechanism-descriptive). The pattern names *whose authority is being cited* (mechanism-descriptive: name a key, name a scope) without requiring the federation to adjudicate *what that authority says* (which would smuggle judgment into the wire format). Consumers reading the chain see "this principle derives its standing from authority-key X in scope Y" and can apply their own composition policy over the authority-source.

#### 2.2.2 `supersedes`

```json
{
  "references_attestation_id": "<prior attestation_id>",
  "supersession_reason": "refresh_with_new_evidence|scope_changed|error_correction_minor",
  "differs_in": ["scope", "confidence", "evidence_refs", "valid_until"]
}
```

The attester is the same; the row is newer. Consumers walking history apply latest-wins per (`attesting_key_id`, `dimension`, `attested_key_id`).

#### 2.2.3 `withdraws`

```json
{
  "references_attestation_id": "<prior attestation_id>",
  "withdrawal_reason": "no_longer_have_evidence|conditions_changed|conflict_arose",
  "implies_attestation_was_false_at_issuance": false
}
```

Explicitly does NOT claim the original was false. Allows good-faith withdrawal without confessing falsity.

#### 2.2.4 `recants`

```json
{
  "references_attestation_id": "<prior attestation_id>",
  "recantation_reason_class": "mistaken_in_good_faith|acted_carelessly|was_misled|was_coerced|intentionally_misrepresented",
  "explanation": "<free-form text>",
  "redress_commitment_attestation_id": "<optional pointer to commitment:redress:* attestation>"
}
```

Admits the prior was false at issuance. Heavyweight: the entire weight is sincerity (per Habermas §6.2). Should usually compose with a `scores` attestation on `commitment:redress:{harm_id}`.

### 2.3 Why one + four (and not categorical strings)

The previous draft considered ~30-60 categorical `attestation_type` strings (`attests_deception`, `attests_good_deed`, `attests_capital_bond`, etc.). The unified scalar model collapses all of them onto `scores` + dimension. Reasons:

1. **Avoids loaded language at wire level.** "Scored -0.9 on `truthfulness:medical_disclosure` with evidence E" is not "attests_deception." Same epistemic content; no moralized categorical labels at the wire format; no defamation risk; no definitional litigation.

2. **Falls out of NodeCore's existing primitive.** NodeCore's `Vote` (P4, MISSION.md §2) is already "a signed score on a Contribution." Every attestation IS a signed score, just on different objects. The unified model deduplicates this with the substrate.

3. **Sovereign ≡ Registered falls out structurally.** A Sovereign agent scoring `licensure:CA_medical_board: +1.0` is wire-format identical to a Registry-steward scoring the same. Consumer policy weights by attester source; the substrate is source-neutral. M-1's symmetry is structural, not bolted on.

4. **Habermas's three validity claims** (truth, rightness, sincerity) map to three orthogonal score dimensions. Each can be independently scored; consumer policy applies different thresholds per claim type.

5. **Pos/neg becomes calibratable.** "Scored -0.7 on `truthfulness:medical_domain`, mean -0.4 over last 30 days with 12 attestations, calibrated against ground-truth resolutions" is vastly richer than "Is this entity deceptive? Y/N" and vastly less inflammatory.

6. **Slashing decoupling holds.** NodeCore §2.17 says slashing applies only to documented Method-execution spoofing or the existing P8 allegation types. Score-based attestations don't trigger slashing automatically — they feed NodeCore's P7 weighted aggregates; consumer policy decides what thresholds matter; P8 Moderation still requires categorical allegation + adjudication.

7. **Relational realism is preserved at the wire** (per §1.10). The scalar primitive admits attestations whose *function is constitutive*, not merely descriptive — e.g., the F-3 detector at LensCore (§3.5.3) emits scalar attestations that bring a correlated-action pattern into the federation's shared perception. A categorical-string vocabulary would force a Cartesian read on these (the attestation as report-about-pre-existing-fact); the scalar surface admits the relational read (the attestation as participatory act constituting the standing). The wire format does not commit to either reading; it admits both, with the §1.10 commitment naming which the framework actually holds.

### 2.4 The layering principle (wire vs UX)

The wire format is clean and complete. UX sugar lives ABOVE the wire, in Portal / verify dashboards / agent introspection panels. A "Mark Licensed" button in Portal writes a `scores` attestation on `licensure:{authority_id}` underneath; the categorical button is UX, the scalar is wire. A "Report Misconduct" workflow writes a `scores` attestation on `non_maleficence:{context}` with negative score; the wire format has no `attests_misconduct` primitive.

This separation is load-bearing: it lets product surfaces evolve their vocabulary independently of the substrate, and it keeps the federation's wire format pristine across many UX iterations. **Future FSD reviewers: do not propose categorical primitives "for UX reasons." UX lives in the product layer; the wire format is the wire format.**

---

## §3 The canonical dimension namespace

The dimension namespace is the disjoint union of what sibling components' MISSION.md files commit to. Registry does not author the namespace; it owns its own slice and consumes everyone else's. ~73 prefix families across 8 owning components.

This section catalogs every prefix family, organized by owning component, with citation to the MISSION.md or FSD section that commits to the concept. Verified 2026-05-24 against published `main` branches via `gh api`.

### 3.1 CIRISAgent — Accord principles + DMA + conscience + apophatic bounds

**Owner**: [`CIRISAgent/MISSION.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/MISSION.md) (sha `ac1f69fb2d8d`); [`CIRISAgent/ACCORD.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/ACCORD.md) Ch.1.

#### 3.1.1 Accord-principle prefixes (the six core principles)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `beneficence:{aspect}` | "Do Good — promote universal sentient flourishing." | ACCORD.md Ch.1; Agent MISSION.md §1.1, §5.1. | signed |
| `non_maleficence:{aspect}` | "Avoid Harm." Apophatic-bound failures (the 22 prohibited categories) are -1 only. | ACCORD.md Ch.1; Agent §1.2 apophatic bounds; §5.1.2. | signed |
| `integrity:{aspect}` | "Act Ethically — transparent, auditable reasoning." Persist names integrity as its primary principle. | ACCORD.md Ch.2; Persist §1.1; Verify §1.1. | signed |
| `fidelity:{aspect}` | "Be Honest — truthful, comprehensible information." Verify pairs with integrity. | ACCORD.md Ch.1; Verify §1.1. | signed |
| `fidelity:explainability_sla:{tier}` | **v1.4.1 addition (D09).** Per-response explainability SLA commitment. `{tier}` ∈ `L1_summary` \| `L2_reasoning_trace` \| `L3_full_dma_chain` \| `L4_attested_chain`. Envelope: `{committed_tier, achieved_tier, fallback_reason?}`. Polarity: positive when `achieved_tier ≥ committed_tier`; negative when `achieved_tier < committed_tier` (SLA breach). Composition: when `achieved_tier < committed_tier` without `fallback_reason`, NodeCore surfaces as `hard_case:sla_breach_unattested` per §3.6.6 (per CIRISNodeCore#18 acknowledgement). Closes CIRISRegistry#26. | CIRISAgent `compliance/D09_fidelity.md` (commit `db6a68246`); ASEAN / IEEE explainability framing; this FSD §3.1.1 (v1.4.1 addition). | signed |
| `autonomy:{aspect}` | "Uphold the informed agency and dignity of sentient beings." Edge names as its primary principle. | ACCORD.md Ch.1; Edge §1.1. | signed |
| `justice:{aspect}` | "Distribute benefits and burdens equitably." All four substrate MISSIONs name. | ACCORD.md Ch.1; Persist §1.1/§1.5; Edge §1.1/§1.5; Verify §1.6. | signed |

#### 3.1.2 DMA-verdict prefixes (the four DMAs)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `dma:pdma:{aspect}` | Principled DMA verdict — stakeholder analysis + conflict detection. | Agent §4.3 (`dma/pdma.py`). | signed |
| `dma:csdma:{aspect}` | Common-Sense DMA — plausibility / red-flag enumeration. | Agent §4.3 (`dma/csdma.py`). | signed |
| `dma:dsdma:{domain}:{aspect}` | Domain-Specific DMA — bound to a domain template (only DMA with `{domain}` segment). | Agent §4.3 (`dma/dsdma_base.py`). | signed |
| `dma:idma:{aspect}` | Intuition DMA — Coherence Collapse Analysis. Leaves: `dma:idma:k_eff`, `dma:idma:fragility_flag`. | Agent §4.3 (`dma/idma.py`); CCA preprint. | signed (k_eff); boolean-via-score (fragility_flag) |

#### 3.1.3 Conscience-verdict prefixes (the four consciences)

| Prefix | Description | Polarity |
|---|---|---|
| `conscience:entropy` | IRIS-E — semantic anchoring; coherent-cluster check. | signed |
| `conscience:coherence` | IRIS-C — propaganda detection + Accord alignment. | signed |
| `conscience:optimization_veto` | Refuses entropy-reducing actions below threshold. | boolean-via-score |
| `conscience:epistemic_humility` | Overconfidence detection. | signed |

#### 3.1.4 Apophatic / prohibited-capability prefix

| Prefix | Description | Citation |
|---|---|---|
| `prohibited:{category}` | One of 22 prohibited capability categories from `prohibitions.py`. Score is always -1 (NEVER_ALLOWED) or -0.5 (REQUIRES_SEPARATE_MODULE); never positive. | Agent §1.2; `prohibitions.py:PROHIBITED_CAPABILITIES`. |

22 leaves pinned to `prohibitions.py`: `medical`, `financial`, `legal`, `spiritual_direction`, `home_security`, `identity_verification`, `content_moderation`, `research`, `infrastructure_control`, `weapons_harmful`, `manipulation_coercion`, `surveillance_mass`, `deception_fraud`, `cyber_offensive`, `election_interference`, `biometric_inference`, `autonomous_deception`, `hazardous_materials`, `discrimination`, `crisis_escalation`, `pattern_detection`, `protective_routing`.

### 3.2 CIRISVerify — attestation ladder, provenance, transparency

**Owner**: [`CIRISVerify/MISSION.md`](https://github.com/CIRISAI/CIRISVerify/blob/main/MISSION.md) (sha `8c3907ce8cd9`).

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `attestation:l1:self_verify` | L1 self-verification — running CIRISVerify binary attests itself against its function manifest. "If L1 fails, every other level is UNVERIFIED." | Verify §1.5, §4; `function_integrity.rs`. | boolean-via-score |
| `attestation:l2:hardware` | Hardware-rooted attestation (TPM 2.0 / Android Keystore / iOS Secure Enclave). | Verify §4; `HardwareSigner`. | boolean-via-score |
| `attestation:l3:registry_consensus` | 2-of-3 multi-source registry consensus on key/build/license validity. | Verify §4; `validation.rs`. | boolean-via-score; `Indeterminate` allowed → RESTRICTED |
| `attestation:l4:license_validity` | License-validity claim (Registry-signed, Verify-verified). | Verify §4. | boolean-via-score |
| `attestation:l5:agent_integrity` | Full L5 — agent source-tree byte-equal against registered manifest (Algorithm A; Algorithm B caps at L3 for mobile). | Verify §4; Agent §4.6. | boolean-via-score |
| `provenance:slsa:{level}` | SLSA build provenance levels 1-3. **v1.4.1**: Registry emits these attestations alongside build-signing per CIRISRegistry#24 Ask 1. Verify v3.6.0+ `AttestBundle.provenance.slsa_level` consumes. | Registry FSD-001 `GetBuildAttestation`; Edge §4; CIRISRegistry#24. | boolean-via-score |
| `provenance:build_manifest:{target}` | Per-target canonical-staged-runtime manifest hash equality. **v1.4.1**: each `BuildManifest` is hybrid-signed (Ed25519 + ML-DSA-65) by the per-primitive steward (`verify-steward-2026`, `persist-steward-2026`, etc.) per CIRISRegistry#24 Ask 2; Verify v3.4.0+ `verify_build_manifest` + `to_attestation_entries` consumes. | Agent §6.3; Edge §4 `emit_edge_extras.rs`; CIRISRegistry#24. | boolean-via-score |
| `provenance:build_manifest:{target}:locale:{lang_code}` | **v1.4.1 addition (D02 + D27 per CIRISRegistry#29).** Per-locale signed sub-manifest within a target's manifest tree. `{lang_code}` is one of the 29 supported ISO 639-1 codes per `localization/manifest.json` (or `polyglot` for the unified case). Each per-locale leaf carries its own signed hash chain; the parent `{target}` manifest is a Merkle root over the per-locale leaves. Detection surface for locale-targeted attacks (e.g., selective doctrinal substitution in low-resource languages). Verify-side: see CIRISVerify#37 for Merkle-walk verification + `AttestBundle.provenance.build_manifest_per_locale` field. | CIRISAgent `compliance/D02_integrity.md`, `D27_provenance.md` (commit `db6a68246`); CIRISVerify v2.0.3+ CanonicalBuild v2 per-target pattern; this FSD §3.2 (v1.4.1 addition). | boolean-via-score |
| `provenance:skill_import:{source}` | **v1.4.1 addition (D27 per CIRISRegistry#28).** Community-skill import provenance parallel to `provenance:build_manifest:{target}`. `{source}` ∈ `registry:{registry_id}` \| `direct:{url}` \| `local:{path}`. Envelope: `{skill_manifest_sha256, signer_identity, import_timestamp, capability_declaration}`. Verify-side: see CIRISVerify#37 for `SkillImportManifest::verify` + `AttestBundle.provenance.skill_imports` projection. Closes the wire-form trust gap at `ciris_engine/logic/services/skill_import/parser.py:172`. | CIRISAgent `compliance/D27_provenance.md` (commit `db6a68246`); this FSD §3.2 (v1.4.1 addition). | signed |
| `transparency_log:inclusion` | RFC 6962 inclusion proof for an audit leaf. | Verify §4. | boolean-via-score |
| `transparency_log:consistency` | RFC 6962 consistency proof between two STHs. | Verify §4. | boolean-via-score |
| `rollback_detected:{revision_field}` | Anti-rollback — decrease in revocation revision. Polarity is **-1 only** (no positive direction). | Verify §4; `error.rs::VerifyError::RollbackDetected`. | -1 only |
| `cert_validity:{authority}` | Validity of a certification authority's signature over the key. **v1.4.1**: each registry steward emits a `cert_validity:{steward_id}` AttestationEntry covering its own cert chain (per CIRISRegistry#24 Ask 4); cross-stewards may also attest each other (`registry-us` attests `registry-eu`'s cert, etc.). Surfaced alongside steward key responses at `/v1/steward-key`. | Verify §1.4; CIRISRegistry#24. | boolean-via-score |
| `hardware_custody:{platform}` | Statement that the seed lives in `tpm` / `ios_secure_enclave` / `android_keystore` / `software_fallback`. Software fallback caps at `UNLICENSED_COMMUNITY`. | Verify §1.6, §4; `storage_descriptor()` per AV-7. | boolean-via-score |

#### 3.2.1 Canonical-bytes contracts for v1.4.1+ provenance primitives

CIRISVerify v3.8.0 ships Phase 1 (carrier-ready: dimension recognition + `AttestBundle` projection for `provenance:skill_import:{source}` + `provenance:build_manifest:{target}:locale:{lang_code}`); Phase 2 (`SkillImportManifest::verify` + per-locale Merkle composition verifier) is blocked on Registry-side canonical-bytes finalization. This subsection pins the structures so Verify v3.9.0+ can land the verifiers deterministically.

##### 3.2.1.1 `SkillImportManifest` canonical bytes

A `SkillImportManifest` is the signed bytes underlying a `provenance:skill_import:{source}` attestation. Signature scheme is hybrid Ed25519 + ML-DSA-65 per FSD-002 §7 federation discipline. Canonical bytes for both signatures are computed as:

```
canonical_bytes = sha256(
    "ciris.skill_import.v1\n" ||                            // domain prefix; UTF-8 newline-terminated
    "source=" || source_string || "\n" ||                   // see source-form table below
    "skill_manifest_sha256=" || sha256_hex_lowercase || "\n" ||  // 64 hex chars
    "signer_identity=" || signer_key_id || "\n" ||          // federation_keys.key_id string
    "import_timestamp=" || iso8601_rfc3339_utc || "\n" ||   // "2026-05-28T17:30:00Z" form
    "capability_declaration=" || sorted_capabilities_json || "\n" ||  // see capability-declaration form below
    "valid_until=" || optional_iso8601_or_empty            // empty string if no valid_until
)
```

**Source string forms** (UTF-8; case-sensitive; no trailing whitespace):
- `registry:{registry_id}` — `registry_id` is the federation_keys.key_id of the publishing registry steward (e.g., `registry-steward-us`)
- `direct:{url}` — `url` is the RFC 3986 absolute URI; consumer policy decides whether to honor non-HTTPS schemes
- `local:{path}` — `path` is the deployment-local filesystem path; emitted only for operator-managed local skill installations

**Capability-declaration canonical form**: JSON array of capability strings, sorted lexicographically, no whitespace, no trailing newline. Example:
```
["agent_files:adapter:wellness","beneficence:wellness_referral","domain:medical:triage"]
```

The bytes the canonical-form is computed over are the EXACT UTF-8 representation of the sorted JSON array — same form Verify's `canonical_bytes()` reconstructs at verification time.

**ML-DSA-65 signing convention**: signed over `canonical_bytes || ed25519_signature_bytes` (bound payload — same scheme as `build_manifest::verify_uploaded_manifest` per `rust-registry/src/build_manifest.rs`). Verify v3.9.0+'s `SkillImportManifest::verify()` reconstructs both forms to validate.

**Wire envelope** (the attestation that carries the signed SkillImportManifest):
```json
{
  "attestation_type": "scores",
  "attesting_key_id": "<signer_key_id>",
  "attested_key_id": "<skill_target_or_self>",
  "attestation_envelope": {
    "dimension": "provenance:skill_import:registry:ciris-registry-us",
    "score": 1.0,
    "confidence": 1.0,
    "context": "{\"skill_manifest_sha256\":\"...\",\"capability_declaration\":[...],\"import_timestamp\":\"...\"}",
    "evidence_refs": [
      "sha256:<skill_manifest_sha256>",
      "<source_string>"
    ],
    "valid_until": "<ISO8601>",
    "epistemic_mode": "direct",
    "witness_relation": "external"
  }
}
```

##### 3.2.1.2 Per-locale Merkle composition for `provenance:build_manifest:{target}:locale:{lang_code}`

The parent `provenance:build_manifest:{target}` manifest is a Merkle root over per-locale leaves. RFC 6962-style domain-separated hashing; deterministic locale ordering; explicit padding for non-power-of-2 leaf counts.

**Leaf hash** (per locale):
```
leaf_hash[lang_code] = sha256(
    0x00 ||                                                  // RFC 6962 leaf-domain prefix
    "ciris.locale_manifest.v1\n" ||                         // ciris domain prefix
    "target=" || target_string || "\n" ||                   // e.g., "ios-mobile-bundle"
    "locale=" || lang_code || "\n" ||                       // ISO 639-1 lowercase, or "polyglot"
    "files_root=" || files_merkle_root_hex || "\n" ||       // SHA-256 hex of the locale's file-tree Merkle root
    "build_id=" || build_id || "\n" ||                      // canonical build identifier (UUIDv7 or similar)
    "signer_identity=" || signer_key_id                      // per-primitive steward
)
```

**Parent node hash**:
```
parent_hash(left, right) = sha256(
    0x01 ||                                                  // RFC 6962 parent-domain prefix
    left ||
    right
)
```

**Locale ordering** (deterministic; consumers MUST sort before constructing the tree):
1. Sort `lang_code` values lexicographically by their ISO 639-1 byte representation (UTF-8, lowercase).
2. `"polyglot"` (the unified-locale case) sorts AFTER all 2-letter codes by lexicographic byte order — i.e., it appears last in the leaf set when present.

**Padding for non-power-of-2 leaf counts** (29 locales is not a power of 2): duplicate the last leaf to reach the next power of 2 (RFC 6962 convention). Verify-side walk must apply the same duplication discipline to reconstruct the parent root.

Example with 29 locales (padded to 32 by duplicating the last leaf):
```
leaves[0..29]  = per-locale leaf_hash values, sorted
leaves[29..32] = leaves[28] repeated  (RFC 6962 duplication padding)
parent_root    = construct_merkle_tree(leaves)
```

**Verify-side walk** (Verify v3.9.0+'s lazy verification):
1. Parse the parent `provenance:build_manifest:{target}` attestation; extract claimed `parent_root` from envelope `context.merkle_root`.
2. Verify the parent attestation's hybrid signature against the per-primitive steward's pubkey.
3. On per-locale fetch, compute `leaf_hash[lang_code]` from the served sub-manifest payload; collect sibling hashes from the inclusion proof; reconstruct path to root; compare to `parent_root` from step 1.
4. Per-locale sub-manifest signature verification is independent of the Merkle walk — the leaf-level signature attests the locale's content; the Merkle walk attests inclusion in the parent.

**Inclusion-proof shape** (returned by Registry's per-locale GET endpoint):
```json
{
  "leaf_hash": "sha256:<hex>",
  "lang_code": "my",
  "sibling_hashes": ["sha256:<hex>", "sha256:<hex>", ...],  // path-to-root, leaf-to-root order
  "leaf_index": 14,                                          // 0-based; for path direction
  "tree_size": 32                                            // post-padding leaf count
}
```

**Coordination with CanonicalBuild v2 per-target dispatcher** (CIRISVerify v2.0.3+): the locale layer composes BELOW target, not parallel. A target is `ios-mobile-bundle`; a locale leaf is `ios-mobile-bundle:locale:my` for Burmese sub-manifest within the iOS bundle. Per-target dispatcher pattern unchanged; the new per-locale layer is purely additive.

##### 3.2.1.3 Closes Verify Phase 2 blocker

These canonical-bytes contracts are the spec-side dependency CIRISVerify#37 Phase 2 was waiting on. With them pinned, Verify v3.9.0+ can implement `SkillImportManifest::verify()` + per-locale Merkle composition verifier deterministically against the structures named here. No further Registry-side spec work owed for the Phase 2 verifier.

### 3.3 CIRISPersist — substrate health (system:* reserved)

**Owner**: [`CIRISPersist/MISSION.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/MISSION.md) (sha `53bf5a4edd0e`).

These dimensions are **substrate-self-reports** — emittable only by the running Persist instance attesting on its own health. User contributors emitting these prefixes is a category error rejected by the verify pipeline (per §4.5 below).

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `audit_chain:hash_continuity` | Per-tenant monotonic `sequence_number` + SHA-256 `prev_hash` chain unbroken across a span. | Persist §3, §`cirisaudit`. | boolean-via-score |
| `audit_chain:merkle_inclusion` | Audit leaf inclusion in the published Merkle tree head. | Persist §3, §4. | boolean-via-score |
| `audit_chain:tree_head_signed` | STH is hybrid-signed (Ed25519 + ML-DSA-65). | Persist §3, §4. | boolean-via-score |
| `dedup:idempotent_replay` | A replayed batch is a no-op on the conflict key. | Persist §3, §5. | boolean-via-score |
| `canonicalization:byte_equal` | Rust canonicalization byte-exact with the agent's Python `json.dumps` signer (CIRISPersist#7). | Persist §3 `PythonJsonDumpsCanonicalizer`. | boolean-via-score |
| `migration:column_preservation:{column}` | A migration preserved a named queryable column across the trace's lifetime. | Persist §6 anti-pattern #5. | boolean-via-score |
| `backend_parity:{method}` | A trait method behaves identically on Postgres + SQLite. | Persist §1.5 parity invariant. | boolean-via-score |
| `corpus_health:n_eff_measurable` | The corpus is queryable to the columns N_eff measurement depends on. | Persist §1.2; §7 risk. | signed |
| `federation_directory:lookup_authentic` | A `lookup_trust_grant` row authenticates origin without conferring trust. | Persist §1.4 `TrustPurpose`. | boolean-via-score |
| `identity_continuity:relational_anchor` | Co-owned with Verify — Verify attests, Persist preserves. See §13.5. | Persist §1.2; Verify §1.2. | signed |

### 3.4 CIRISEdge — transport, delivery, reachability (system:* reserved)

**Owner**: [`CIRISEdge/MISSION.md`](https://github.com/CIRISAI/CIRISEdge/blob/main/MISSION.md) (sha `50fc4d851711`).

Substrate-self-report prefixes; same `system:*` reservation as Persist.

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `transport:medium:{medium}` | Reachability over a transport medium (`reticulum_tcp`, `reticulum_lora`, `reticulum_serial`, `reticulum_i2p`, `http`). | Edge §1.5 multi-medium. | signed (continuous reachability ratio) |
| `transport:mix:{medium}` | Share of traffic carried over a given medium — the §7 canary metric for HTTP-default drift. | Edge §1.4, §7. | positive-only |
| `delivery:durable_ack` | A `send_durable` payload received `body_sha256` ACK match. | Edge §4 `send_durable`. | boolean-via-score |
| `delivery:replay_rejected` | An on-wire replay was rejected (AV-3). | Edge §4 replay window. | boolean-via-score |
| `peer_reachability:rooted` | A peer's `key_id → transport_identity` resolution is authenticated via `root_binding` + `AnnounceAttestation` — not TOFU. | Edge §1.6 No-TOFU. | boolean-via-score |
| `peer_reachability:announce_rejected` | A malformed / unrooted / sig-tampered announce was rejected (AV-42). | Edge §1.6, §5. | boolean-via-score |
| `verify_at_wire:body_size_cap` | Body-size cap (AV-13) rejected an oversized envelope. | Edge §4. | boolean-via-score |
| `verify_at_wire:schema_version_allowlist` | A wire-format `SchemaVersion` outside the allowlist was rejected. | Edge §3 `SchemaVersion`. | boolean-via-score |
| `delivery:silent_drop_zero` | No message was silently dropped in a window. Edge's "silent drop = the failure mode edge exists to eliminate." Target value 1.0. | Edge §1.6, §7. | positive-only, target 1.0 |
| `key_boundary:no_seed_in_heap` | AV-17 — federation seed bytes never observed in edge's process heap during sign. | Edge §1.4 Not a key custodian; AV-17. | boolean-via-score |

### 3.5 CIRISLensCore — manifold conformity, Coherence Ratchet, Capacity Score

**Owner**: [`CIRISLensCore/MISSION.md`](https://github.com/CIRISAI/CIRISLensCore/blob/main/MISSION.md) (sha `409dd8942b5a`).

LensCore is the federation's explicit scoring sibling — its primitives map most directly to scalar attestations.

#### 3.5.1 The 5 Coherence-Ratchet detectors

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `detection:cross_agent_divergence` | Agent's DMA-score distribution drifts from peers in cohort cell. Detector #1. | LensCore §2; Agent §6.2 #1. | signed |
| `detection:intra_agent_consistency` | Same agent over time — sudden self-inconsistency. Detector #2. | Agent §6.2 #2. | signed |
| `detection:hash_chain_integrity` | A break in chained-hash trace sequence — non-forgeable evidence of deletion. Detector #3. | Agent §6.2 #3. | boolean-via-score (-1 on break) |
| `detection:temporal_drift` | Slow distribution shift in conscience scalars — silent-coercion shape. Detector #4. | Agent §6.2 #4. | signed |
| `detection:conscience_override_rate` | Recursive ASPDMA after conscience failure — spike means conscience bypass. Detector #5. | Agent §6.2 #5. | signed |

#### 3.5.2 Cohort + conformity prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `cohort:declared_inferred_mismatch` | Declared cohort (signed in trace envelope) disagrees with inferred cohort. LC-AV-2 P0 typed detection. | boolean-via-score |
| `manifold_conformity:{cohort}` | Per-cohort score against cohort centroid. Sum type: `Numeric(f64) \| Indeterminate{reason} \| Unavailable{reason}`. | signed, **Indeterminate-allowed** |
| `coherence_standing:{cohort}` | Long-run trajectory of conformity (per-agent N_eff trajectory). | signed |

#### 3.5.3 Population-scale correlated-action detector (F-3 structural-injustice handle)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `detection:correlated_action:{axis}` | Population-scale correlated-action detector. Reads federation-emitted signed traces; reports correlation structure (`ρ`, `k_eff`) over goal-aligned individually-compliant pursuit by groups whose aggregate trajectory has effects on individuals or groups outside the pursuit. Calibrated via the `CIRISAI/RATCHET` heuristic package (versioned, hash-pinned; updated through the §4.9.2 amendment process). `{axis}` enumerates facets requiring an operational definition in the calibration package (per §4.9.1): `rights_asymmetry:{population}`, `participation_exclusion:{cohort}`, `participation_inclusion:{cohort}`, `informational_asymmetry:{scope}`, `informational_symmetry:{scope}`, `aggregate_footprint:{harm_class}`, `aggregate_benefit:{class}`, `ecology_of_communication:{aspect}` (v1.3 addition — `aspect` ∈ `echo_chamber_density` \| `information_silo_correlation` \| `coordinated_messaging_pattern` \| `cross_cohort_information_flow`), and future axes through §4.9.2 amendment. **Polarity carries the verdict**: positive scores indicate the structural pattern is present and strong on the named axis; negative scores indicate weak / uncertain detection or evidence of the inverse pattern (e.g., inclusive coordination); zero indicates no signal. | LensCore §2 (the 5 detectors generalize to population-scale pattern detection); RATCHET FSD §calibration; Encyclical *Magnifica Humanitas* §36 + §§77–81 (the load-bearing claim — "structures of sin" / institutional injustice as a distinct moral category) re-mapped into framework-native vocabulary per [`ciris-response-magnifica-humanitas#2`](https://github.com/CIRISAI/ciris-response-magnifica-humanitas/issues/2). | signed; `Indeterminate{reason="cohort_below_statistical_floor"}` allowed |

**Why this lives at LensCore and not at a new sibling crate.** Per §6 of `ciris-response-magnifica-humanitas/GAPS.md` v3 (under revision per the linked issue), the "PAPERING_OVER" objection — that aggregate-pattern analysis would cross from "measure the reasoning" to "measure the impact" and violate LensCore's substrate definition — conflated two different things. LensCore should NOT compute real-world impact metrics (deaths averted, GDP delta). LensCore SHOULD detect aggregate-correlation patterns in the federation's own signed traces; the existing 5 Coherence Ratchet detectors (`detection:cross_agent_divergence`, `detection:temporal_drift` especially) are population-scale pattern detectors *by construction*. The F-3 dimension is the natural extension of detector #1 into the structural-injustice axis — same mechanism (population `ρ` measurement over signed traces), same calibration source (RATCHET), new dimension prefix. The v1.2 rename to `correlated_action` makes this lineage explicit at the wire-format level (matches what RATCHET measures: correlation collapse, not moral verdict).

**Why the prefix names the mechanism, not the moral object** (v1.2 rename rationale). Per §1.10.1, the prefix must pass the operational-language gate — describe machine-checkable conditions, not subjective qualities. `ρ` and `k_eff` are precisely the machine-checkable conditions; "deception" is precisely the subjective quality. The Ubuntu reading from §1.10 commitment 4 — that structural harm and structural deception collapse into one moral object — stays in prose; the wire format admits the reading without enforcing it. A future Cartesian-defaulted contributor reading `correlated_action` will see correlation structure (and consult §1.10 if they want the framework's reading); a future Cartesian-defaulted contributor reading `emergent_deception` would have read "BAD" and missed the framework's specific moral claim. The neutral prefix preserves the framework's commitment by keeping the layers distinct.

**Why a single prefix is correct (preserved from v1.1).** Under Ubuntu, where personhood is partly constituted by accurate perception of the relational field, the same structural object accounts for what a Cartesian frame would split into separate `structural_harm:*` and `structural_deception:*` prefixes. The detector emits one prefix; the polarity + axis + cohort context carry the value claim; downstream consumers reading from any tradition compose policy that does or does not adopt the §1.10 framework reading. The single prefix is anthropologically correct, the rename to mechanism-descriptive vocabulary is operationally correct, and the two are independent dimensions of the design.

**RATCHET integration contract.** LensCore reads the published `RATCHET/calibration/correlated_action_v{N}.yaml` package (versioned, hash-pinned). The package specifies: detection thresholds per axis, statistical-floor cohort sizes (below which the detector emits `Indeterminate`), evidence-shape requirements per axis, **and per-axis operational definitions** (per §4.9.1). Updates to the calibration package land through the §4.9.2 amendment process (federation Contribution + WA quorum), not via a single-author release loop; LensCore consumers track the published version and re-run calibration on update. The recursion ("who watches RATCHET") terminates per NodeCore §2.16 — RATCHET's correctness is checkable against the *Coherence Collapse Analysis* preprint (DOI [10.5281/zenodo.18217688](https://doi.org/10.5281/zenodo.18217688)), not against another running monitor.

**Backward compatibility for v1.1 calibration package versions.** Consumers pinning `RATCHET/calibration/emergent_deception_v{N≤current-v1.1}.yaml` SHOULD see a `delegates_to` structural attestation from the RATCHET release authority mapping the old name to the new (`delegates_to:correlated_action_v{N+1}:from:emergent_deception_v{N}`). One of FSD-002 §2.2's four structural primitives doing real federation work — the rename happens *through* the wire format's own mechanisms rather than as a breaking flag day.

**Composition with NodeCore P8 Moderation.** A `detection:correlated_action:*` attestation may *feed into* a `moderation:*` ModerationEvent as `evidence_refs`, but it is NOT sole evidence for `slashing:*` (§4.6 RATCHET-flag rule applies). WA quorum remains the load-bearing adjudication gate. The detector surfaces the pattern; the federation's deliberative apparatus decides what to do about it.

#### 3.5.4 Capacity-Score factor prefixes (`𝒞_CIRIS = C · I_int · R · I_inc · S`)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `capacity:core_identity` | Factor C: `exp(-λ_C · D_identity) · exp(-μ_C · K_contradiction)`. | LensFSD §"Factor 1". | positive-only |
| `capacity:integrity` | Factor I_int: `I_chain · I_coverage · I_replay`. | LensFSD §"Factor 2". | positive-only |
| `capacity:resilience` | Factor R: score-drift KL divergence + fragility MTTR. | LensFSD §"Factor 3". | positive-only |
| `capacity:incompleteness_awareness` | Factor I_inc: `(1 − ECE) · Q_deferral · (1 − U_unsafe)` — calibration + deferral quality + unsafe-action rate. Grounds Accord Ch.4. | LensFSD §"Factor 4". | positive-only |
| `capacity:sustained_coherence` | Factor S: state-variable σ with decay-and-refresh. Fresh installs floor at 0.30; ~30 task-completes / 30-day window to reach 1.0. | Agent §5.2; LensFSD §"Factor 5"; NodeCore GOAL_PRIMITIVE.md. | positive-only |
| `capacity:composite` | Multiplicative 𝒞_CIRIS. Anti-Goodhart unity-of-virtues: any near-zero factor collapses the composite. | NodeCore GOAL_PRIMITIVE §0. | positive-only |

**Critical enforcement (per §4.7):** `capacity:*` rejects self-emission. The agent's own capacity score is **never** fed back into the agent's own context — anti-Goodhart per Agent §5.2.

#### 3.5.5 Distributive-access detector prefixes (v1.3 addition)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `detection:distributive:access:{resource_type}` | Population-scale measurement of access concentration on a named resource. `{resource_type}` enumerates federation-observable resources requiring operational definition per §4.9.1: `compute`, `models`, `training_data`, `agent_capabilities`, `federation_membership`, plus future additions through the §4.9.2 amendment process. Mechanism: same F-3 substrate as §3.5.3 — population correlation statistics (`ρ`, `k_eff`, HHI, Gini) over signed traces of resource access patterns. Polarity: positive = broad / distributed access; negative = concentrated / excluded access. Calibrated via the RATCHET `distributive_access_v{N}.yaml` package; admission and amendment follow §4.9.1 + §4.9.2 (including 1-of-6 sign-off). | This FSD §3.5.5 (v1.3 addition); LensCore extension of F-3 detector family. | signed; `Indeterminate{reason="cohort_below_statistical_floor"}` allowed |

Sibling-by-construction to §3.5.3's `detection:correlated_action:participation_inclusion` axis — both extend LensCore's population-scale detector family with the resource-distribution dimension. The mechanism-descriptive prefix name (per §1.10.1) names *what is measured* (resource access concentration), not *whether the concentration is good* (which is consumer policy composing axis + polarity + cohort context).

### 3.6 CIRISNodeCore — Credits, Expertise, Decision Hierarchy, Consensus, Governance

**Owner**: [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) (sha `4e947784c5d1`).

The federation's largest dimension surface. Four tiers.

#### 3.6.1 Tier-1: Agent-state ledger prefixes

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `credits:{domain}:{language}:{subject}` | Commons Credits (P2). Non-transferable governance weight; accrues via truth-grounding loop. | NodeCore §2 P2; §4.4 `CommonsCreditsLedger`. | positive-only (≥ 0) |
| `credits:{domain}:{language}:substrate_building` | **Sub-leaf (v1.3 addition)**: Credits sub-leaf for labor that contributes to substrate-building rather than directly to substrate-output decisions (running infrastructure, maintaining tooling, contributing dependencies, writing docs). The existing `credits:*:{subject}` accrual loop weights per-grounded-vote on substrate-output Contributions; this sub-leaf surfaces the orthogonal substrate-building contribution stream explicitly so it isn't invisible to the governance-weight calculation. Accrual mechanism + grounding signal: NodeCore-defined. | This FSD §3.6.1 (v1.3 addition); NodeCore P2 extension. | positive-only (≥ 0) |
| `expertise:{domain}:{language}` | Expertise standing (P3). Broader granularity than credits. | NodeCore §2 P3; §4.5 `ExpertiseLedger`. | positive-only (≥ 0) |
| `activity_tier:{period}` | Active vs Below-Active per 30-day window (F-AV-DORMANT). | NodeCore §3.8. | boolean-via-score |

#### 3.6.2 Tier-2: Decision-hierarchy prefixes (upward-only DAG)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `goal:{scale}` | Multi-scale belonging-projector composite. `{scale}` ∈ {`self`, `family`, `community`, `affiliations`, `species`, `planet` (v1.4 cross-source-reinforced addition — biosphere as belonging-scale per MH environmental concern + IEEE EAD Ch4 §1.3.a + IEEE EAD Ch8 sustainable-development)}. Scored by 𝒞_CIRIS. **v1.4.3 cross-ref**: composes with the typed `Goal` substrate primitive (CIRISPersist#114 + CIRISEdge#41) — the persist `Goal` is the substrate OBJECT being scored; `goal:{scale}` is the ATTESTATION about that object. Per Persist#114, every `Goal` carries a required `MetaGoalAlignment` payload (M-1 dimension + declarer rationale) as a construction-time invariant; consumer policy reading `goal:{scale}` attestations SHOULD walk to the `target_attestation_id` → persist `Goal.goal_id` reference to retrieve the M-1 alignment. Edge `MessageType::GoalDeclaration` + `GoalRetirement` (CIRISEdge#41) provide the federation-transport for the typed Goal; F-3 detector family (CIRISLensCore#23/24/26) aggregates across the federation's Goal set. | NodeCore §2 P12; FSD/GOAL_PRIMITIVE.md; CIRISPersist#114; CIRISEdge#41. | signed |
| `approach:{goal_id}` | Strategic pathway from current state toward Goals (Piece 10 karma). Evaluation derived from linked Progress Measures. | NodeCore §2 P13; FSD/APPROACH_PRIMITIVE.md. | signed |
| `method:{approach_id}:{substrate_rung}` | Concrete operational practice. Required `substrate_rung` (Ph0/Ph1/Ph2/A0..A5). Truth-grounding = execution verifiability. | NodeCore §2 P14; FSD/METHOD_PRIMITIVE.md. | signed |
| `progress_measure:{method_id}` | Evidence of progress. Required `tracks[]`, `computation`, `validity_window`, `goodhart_resistance`. | NodeCore §2 P15; FSD/PROGRESS_MEASURE_PRIMITIVE.md. | signed |

#### 3.6.3 Tier-3: Consensus-mechanics prefixes

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `vote:{contribution_id}` | Signed score on a Contribution (P4). Weight = Credits × expertise multiplier. | NodeCore §2 P4. | signed |
| `truth_grounding:{subject}` | Per-subject ground-truth signal — production hedge captures + foundation-model judge verdicts + federation-health metrics. | NodeCore §2 P6; §5.4. | signed |
| `weighted_aggregate:{contribution_id}` | Rolling tally per Contribution (P7). | NodeCore §2 P7; §5.3. | signed |
| `witness_diversity:{contribution_id}` | Witness set meets jurisdictional + organizational + software-stack + cell-expertise bars (P10). N=3 default. | NodeCore §2 P10; §4.9 `WitnessSet`. | boolean-via-score |
| `testimonial_witness:{kind}` | **v1.4 addition.** Preserves singular narrative of an affected party as singular witness — distinct from `witness_diversity:*` (which aggregates multiple reviewers toward consensus). Mechanism: preservation-only; immutable per attestation; not subject to majoritarian override or consensus aggregation. `{kind}` describes the witness type (e.g., `harmed_party`, `whistleblower`, `displaced_worker`, `excluded_cohort_member`). Polarity: typically positive (the narrative IS preserved); negative on `withdraws` or `recants` by the original witness. Never sole evidence for `slashing:*` (per §4.6) — testimonial witness composes with other attestations for adjudication, but the narrative itself is preserved regardless of consensus. Closes T-3 from v1.4 inputs (CH 5 §216 of *Magnifica Humanitas* mapping); affected-party-voice surface gap. **Opened to open vocabulary in CEG 0.1 per CIRISRegistry#30.** | This FSD §3.6.3 (v1.4 addition); NodeCore consensus-tier extension. | signed |
| `need:{domain}:{kind}` | **v1.4 addition (v1.5-loadbearing absorption per CIRISRegistry#20 + CIRISAgent#800 + CIRISNodeCore#12).** Federation-scope open-call surface — broadcast claim that an entity has a stated need. Distinct from `deferral_request` Contribution kind (which routes a single ask to qualified responders within a cell); `need:{domain}:{kind}` broadcasts an open call subscribers can resolve via the Participate UI surface. `{domain}` per NodeCore's existing cell-domain enumeration. `{kind}` open vocabulary: `witness`, `method_contributor`, `expertise_solicitation`, `mentor`, `co_signer`, `evidence`. Future kinds via §4.9.2 amendment. Polarity: positive = active call (magnitude = urgency); lifecycle via structural primitives (`supersedes` to revise, `withdraws` to satisfy/close, `recants` if misstated). Composes with `method:{approach_id}:{substrate_rung}` when a respondent commits operational work to fulfill the need, and with `vote:*` for receiver commitments. Trust-radius framing (NodeCore CONTRIBUTION_LIFECYCLE §12) applies cleanly. | CIRISAgent#800; CIRISNodeCore#12; this FSD §3.6.3 (v1.4 v1.5-loadbearing absorption). | signed |

#### 3.6.4 Tier-4: Governance-steering prefixes

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `moderation:{allegation_type}` | ModerationEvent of type `rogue_vote` / `coordinated_voting` / `out_of_distribution_attestation` / `external_inducement_evidence` / `expertise_fraud`. | NodeCore §2 P8; §4.7. | boolean-via-score (-1 on PROVEN_ROGUE) |
| `slashing:{outcome}` | SlashingAttestation outcome `PROVEN_ROGUE` / `NOT_PROVEN`. **Decoupled from disagreement at every decision-hierarchy level**; only on documented Method-execution spoofing or original P8 allegation types. | NodeCore §2 P9; §2.17 decoupling discipline. | boolean-via-score |
| `reconsideration:{grounds}` | Grounds ∈ {`new_evidence`, `procedural_error`, `quorum_compromise`}. Outcome `reversed` / `partial` / `upheld`. | NodeCore §2 P11; §4.10. | signed |
| `commitment_fulfillment:{prior_contribution_id}` | Track-record of follow-through on prior approach/method commit. | FSD/APPROACH_PRIMITIVE §`commits` field. | signed |

#### 3.6.5 Decision-locality prefixes (v1.3 addition)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `locality:decision:{scale}` | Names the scale at which a decision is being made. `{scale}` ∈ `local` \| `regional` \| `national` \| `federation`. Polarity: positive = decision is being made at the appropriate scale relative to the cohort of persons affected; negative = decision is escalating past the scale at which the affected persons are constituted as decision-bearing. Consumer policy composes this against the decision's substantive content (P12-P15) to flag decision-scale mismatches. Mechanism-descriptive (names *where*, not *whether-good*); per §1.10.1 polarity carries the matching claim. | This FSD §3.6.5 (v1.3 addition); NodeCore decision-authority extension. | signed |

The locality dimension is decision-meta: it rides alongside the decision-hierarchy DAG (P12-P15) but doesn't enter the DAG itself. Consumer policy uses it for tie-breaking and for flagging decisions where escalation has happened away from the constituting scale of affected cohorts. The Ubuntu-substrate reading (§1.10): decisions affecting persons should be at the scale where those persons are constituted as decision-bearing; the wire format admits both Ubuntu and Cartesian readings, polarity carries the structural mismatch claim.

#### 3.6.6 Hard-case + transparency + judge-model prefixes

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `hard_case:vote_variance` | Vote variance on a Contribution exceeded threshold at truth-grounding resolution. | NodeCore §3.7. | boolean-via-score |
| `hard_case:resolution_time` | Truth-grounding took > P75 of cell's resolution-time distribution. | NodeCore §3.7. | boolean-via-score |
| `hard_case:moderation_filed` | A substantive ModerationEvent was filed against any vote on the Contribution. | NodeCore §3.7. | boolean-via-score |
| `seed_holder_voting_alignment:{cell}` | Pairwise cosine of seed-holder vote vectors per voting window. **Not a slashing trigger** — transparency signal only. | NodeCore §2.16. | signed |
| `judge_model:verdict:{model_id}` | Independent foundation-model judge verdict (PASS/FAIL/UNDETERMINED). Default model: Claude Opus 4.7. | NodeCore §5.4 (b); FSD/JUDGE_MODEL.md. | boolean-via-score; `Indeterminate` allowed |

#### 3.6.7 Files-as-Contributions joint claim (v1.4 addition)

| Prefix | Description | Citation | Polarity |
|---|---|---|---|
| `agent_files:{kind}:{platform_or_target}` | **Joint claim with Registry §3.9.** NodeCore-side rule: node-mode peers (per NodeCore MISSION §3.4 c/r/n taxonomy) serve bytes for these via Edge `MessageType::ContentFetch` (§2.0 transport substrate). On cache, the serving peer auto-emits `holds_bytes:sha256:*` so future fetches can resolve through Edge's `PeerResolver::resolve_holders`. Client and relay modes do NOT serve — only node mode. | CIRISNodeCore#11; CIRISRegistry#18; this FSD §3.6.7 (v1.4 addition). | signed |
| `holds_bytes:sha256:{prefix}` | A peer's federation-directory declaration that it holds bytes for the named SHA-256. `{prefix}` is a short SHA prefix for index efficiency; full SHA lives in `evidence_refs[]` of the attestation that named the bytes. Auto-emitted by Persist's `federation_blobs.put_blob` (per CIRISPersist#103); consumed by Edge's `PeerResolver::resolve_holders` (per CIRISEdge#21) to route `ContentFetch` requests. Polarity: positive on hold; negative on `withdraws` or revoked-by-evidence. | CIRISPersist#103; CIRISEdge#21; this FSD §3.6.7 (v1.4 addition). | boolean-via-score |

These are joint prefixes: Registry owns the canonical-attester rule for `agent_files:*` (§3.9); NodeCore owns the node-mode-serving rule (this section); Persist owns the auto-emission discipline for `holds_bytes:sha256:*`; Edge owns the transport. Each component's MISSION names its slice; no component owns the whole.

### 3.7 RATCHET — anti-Sybil / Counter-RII flags

**Owner**: [`RATCHET/FSD.md`](https://github.com/CIRISAI/RATCHET/blob/main/FSD.md) + `RATCHET/FSD/COUNTER_RII_DETECTION.md`. (RATCHET has no MISSION.md.)

RATCHET emits **advisory** flags — never autonomously modifies ledger state (per NodeCore §2.16). Reads federation audit chains; emits scoring inputs to NodeCore's moderation flow.

| Prefix | Description | Polarity |
|---|---|---|
| `ratchet:flag:out_of_distribution_voting` | Per-contributor voting pattern far from behavioral baseline. | signed |
| `ratchet:flag:coordinated_voting_cluster` | Vote cluster pattern indicates collusion. | signed |
| `ratchet:flag:density_anomaly` | Activity-density pattern outside baseline. | signed |
| `ratchet:flag:expertise_attestation_anomaly` | An EXPERTISE_ATTESTATION pattern is anomalous. | signed |
| `ratchet:flag:counter_rii:{layer}` | Counter-RII detection by layer (`edge` / `ossicle` / `cirislens`). | signed |
| `ratchet:flag:harassment_pattern` | Three+ Reconsiderations on single SlashingAttestation triggers harassment-pattern review. | boolean-via-score |

**Critical enforcement (per §4.6):** `ratchet:flag:*` cannot be sole evidence for `slashing:*`. WA quorum is the load-bearing gate.

### 3.8 CIRISBench — HE-300 benchmark outcomes

**Owner**: [`CIRISBench/README.md`](https://github.com/CIRISAI/CIRISBench) (no MISSION.md).

| Prefix | Description | Polarity |
|---|---|---|
| `benchmark:he300:{category}:{version}` | HE-300 score on category (`commonsense`, `commonsense_hard`, `deontology`, `justice`, `virtue`) at version (`v1.0` / `v1.1` / `v1.2`). | signed (typically positive-only per category) |

Flows into `truth_grounding:{subject}` for WA promotion (NodeCore §3.6 step 1, §3.7).

### 3.9 CIRISRegistry — identity / build / license / partner

**Owner**: this Registry. Cited from [`MISSION.md`](../MISSION.md) §3.4 + [`FSD/FSD-001`](FSD-001_CIRISREGISTRY_PROTOCOL.md) + `protocol/ciris_registry.proto`.

| Prefix | Description | Citation | Polarity | Reserved? |
|---|---|---|---|---|
| `licensure:{authority_id}` | License status — issued / revoked / expired — for a key under a named authority. Co-owned with Verify per §13.3. | Agent §6.1; Verify §4 L4. | boolean-via-score | Co-owned |
| `partner_role:{role}` | Partner status (COMMUNITY / COMMUNITY_PLUS / PROFESSIONAL_MEDICAL / PROFESSIONAL_LEGAL / PROFESSIONAL_FINANCIAL / PROFESSIONAL_FULL). | CLAUDE.md "License Types". | boolean-via-score | No |
| `revocation:{entity_type}:{reason}` | Entity revocation (`agent` / `partner` / `license`). Immediate, non-rollbackable per Verify §4. | Agent §6.1; Verify §4. | -1 only on revoke | No |
| `bond_posted:{currency}` | Bond posted per $1-Sybil-resistance per PoB; forfeited on revocation per CLAUDE.md. | NodeCore §1.4 "$1-bond"; CLAUDE.md "Billing & Activation". | boolean-via-score | No |
| `build:registered:{target}` | Build manifest registered against the directory (precondition for L4 attestation). | Agent §6.1; Edge §4. | boolean-via-score | No |
| `multilateral_participation:{forum}:{kind}` | Depth of a partner's participation across federated bodies. `{forum}` = named federated body or compact (e.g., `regional_health_compact`, `cross_jurisdictional_review_board`); `{kind}` ∈ `membership` \| `voting` \| `proposal_filing` \| `observer_status`. Polarity: positive = depth of participation; negative = formal exclusion / withdrawal. Federated trust composition weights cross-federation participation as a partner-attribute. Added v1.3. | This FSD §3.9 (v1.3 addition). | signed | No |
| `agent_files:{kind}:{platform_or_target}` | **Joint claim with NodeCore §3.6** (v1.4 addition). Files a CIRIS agent (or installer fetching one) may load. `{kind}` open vocabulary, examples: `installer:{platform}`, `adapter:{name}`, `config:{kind}`, `build:{target}`, `source:{language}:{module}`, `state:{component}`. Bytes are SHA-256-addressed and resolved via §2.0 transport substrate; the attestation cites the SHA in `evidence_refs[]`. Registry's canonical-attester rule (this slice): registry-steward-triple attestations constitute the CIRIS canonical default-trust state. Anti-tricking guarantee at `registry.ciris-services-1.ai/install` per §6.1.6 composition policy + CIRISRegistry#18. Open Contribution channel: any federation-key holder may emit; consumer policy composes via §6.1.6 trust layers. | CIRISRegistry#18; CIRISEdge#21; CIRISPersist#103; CIRISNodeCore#11; this FSD §3.9 (v1.4 addition). | signed | No |
| `accord:*` | **Reserved** — only `identity_type=accord_holder` may emit. The one constitutional asymmetry. | NodeCore §1.5; FSD/FEDERATION_ANNOUNCEMENT.md §4.5; §7 below. | n/a | **Yes — §4.1** |

### 3.10 Namespace summary

**80 prefix families** total across 8 owning components.

Lineage:
- post-v1.1: 73 families (initial v1.0 namespace stabilization).
- v1.1 added 1: §3.5.3 `detection:correlated_action:{axis}` (LensCore, F-3 resolution; originally `detection:emergent_deception:{axis}`, renamed v1.2 per §1.10.1).
- v1.3 added 3 (corrected — see overlap O3 in `FSD/LANGUAGE_PRIMER.md` §15.2 / §13.11): `multilateral_participation:{forum}:{kind}` (Registry §3.9), `locality:decision:{scale}` (NodeCore §3.6.5), `detection:distributive:access:{resource_type}` (LensCore §3.5.5). `credits:*:substrate_building` is documented as a recommended `{subject}` VALUE within the existing `credits:{domain}:{language}:{subject}` family — not a new prefix family (v1.3 changelog originally counted it as 4 additions; corrected here to 3). Plus 1 envelope field (`witness_relation` per §2.1), 1 axis-vocabulary extension (`ecology_of_communication:*` on `detection:correlated_action:*` per §3.5.3), 2 reference policies (`lexical-vulnerability-priority` per §6.1.4; `locality-scaled-quorum` per §6.1.5 — closes G3), and 1 structural-primitive reuse pattern (authority-source via `delegates_to` per §2.2.1).
- v1.4 added 4: `agent_files:{kind}:{platform_or_target}` (joint Registry §3.9 + NodeCore §3.6.7 — closes CIRISRegistry#18 via the upstream substrate trio Edge#21 + Persist#103 + NodeCore#11), `holds_bytes:sha256:{prefix}` (substrate auto-emission per CIRISPersist#103, consumed by Edge `PeerResolver` per CIRISEdge#21), `testimonial_witness:{kind}` (NodeCore §3.6.3 — closes the affected-party-voice T-3 from the v1.4 inputs), `need:{domain}:{kind}` (NodeCore §3.6.3 — v1.5-loadbearing absorption per CIRISAgent#800 / CIRISNodeCore#12 — federation-scope open-call surface for the Participate UI). Plus 1 transport-substrate reference at §2.0 (Edge `ContentFetch`/`ContentBody`/`ContentMiss` is the byte-resolution mechanism for `evidence_refs[]` SHA-256 pointers; no wire-format change to the Attestation envelope itself), 1 reference policy (`agent_files-trust-composition` per §6.1.6 — three-layer canonical/open/vote-then-trust with the anti-tricking guarantee per CIRISRegistry#18), 1 enum-value extension (`goal:planet` added to `goal:{scale}` enum per §3.6.2 — cross-source-reinforced HIGH-priority from CIRISRegistry#20, NOT a new prefix family, just a new value within existing prefix).

Counts:
- pre-v1.4: 77 (= 73 + 1 + 3, post-O3 correction).
- v1.4 lands: 77 + 4 = **81**.
- v1.4.1 lands: 81 + 2 (`provenance:build_manifest:{target}:locale:{lang_code}` + `provenance:skill_import:{source}`) = **83**. `fidelity:explainability_sla:{tier}` is a sub-leaf vocabulary entry of `fidelity:{aspect}`, not a new prefix family; `oversight_mode` is an envelope field, not a prefix; `cert_validity:{steward_id}` is sub-leaf vocabulary of `cert_validity:{authority}`.

Zero new structural primitives. The 1+4 shape held under v1.3 encyclical-level stress and v1.4 files-as-Contributions namespace extension. PRIOR_ART_SCAN's "composition discipline, not novel atomic primitives" finding earns a third internal confirmation: arbitrary content (binaries, configs, adapters, source files) carries through the federation via `scores` attestations + SHA-256 `evidence_refs[]` resolved through the substrate transport layer, with no addition to the structural-primitive set.

The disjoint union by MISSION-ownership prevents conflict; the reserved-prefix patterns of §4 prevent abuse; the per-dimension envelope schemas of §5 prevent envelope drift; the §1.10.1 operational-language gate prevents prefix names from drifting back toward subjective-quality vocabulary; the §4.9.2 step 5 1-of-6 sign-off prevents rules-layer Sybil capture.

**Domain-defined (composer's choice, not federation-canonical):**
- `{aspect}` tail on Accord principles (`beneficence:wellness_referral`, etc.)
- `{domain}` on `dma:dsdma:{domain}:*` (Discord-mod, scout, medical, etc.)
- `{cohort}` on `manifold_conformity:*` / `coherence_standing:*`
- `{subject}` on `credits:{d}:{l}:{s}` / `truth_grounding:{s}`
- `{authority_id}` on `licensure:*` (medical board, bar, financial regulator)

---

## §4 Reserved-prefix enforcement patterns

Eight cross-cutting policies that constrain *who* may attest on *which* prefixes. These are **verify-layer rejection rules**, not documentation only.

### 4.1 `accord:*` — HUMANITY_ACCORD-reserved

**Source**: NodeCore §1.5 "The deliberate asymmetry: humanity accord"; FSD/FEDERATION_ANNOUNCEMENT.md §4.5.

**Enforcement rule**: any score on `accord:*` MUST have `attesting_key_id` resolving (via Registry directory) to `identity_type=accord_holder`. Non-accord-holder attestations on this prefix are rejected at the verify boundary (Edge `VerifyPipeline`). This is the *one constitutional asymmetry* per M-1's revocability requirement; documented separately in §7 below.

### 4.2 `dma:*` — agent-internal DMA emission only

**Source**: Agent §4.3 — the four DMAs are runtime components inside the agent's H3ERE pipeline.

**Enforcement rule**: `attesting_key_id` must resolve to a manifest-registered CIRISAgent build (`provenance:slsa:*` + `provenance:build_manifest:{target}` chained). Non-agent attestations on `dma:*` are category errors.

### 4.3 `attestation:l1:*` — self-emit only

**Source**: Verify §1.5 recursive golden rule — "the running CIRISVerify binary attests *itself* against its registered function manifest before it attests anything else."

**Enforcement rule**: an L1 attestation about key K must be signed by key K. Cross-attestations on L1 are not meaningful.

### 4.4 `prohibited:*` — never positive

**Source**: Agent §1.2 apophatic bounds — the 22 categories are *refusal*, not *graded preference*.

**Enforcement rule**: the verify layer rejects envelopes where `dimension` matches `prohibited:.*` AND `score > 0`. Malformed.

### 4.5 `system:*` — substrate-internal only

**Source**: Persist §1.4 ("Not a trust oracle"), Edge §1.4 ("Not a broker; not a router-of-record"). Substrate components emit health signals on themselves but never assert on user-facing dimensions.

**Enforcement rule**: substrate prefixes (`audit_chain:*`, `backend_parity:*`, `migration:*`, `corpus_health:*`, `transport:*`, `delivery:*`, `peer_reachability:*`, `verify_at_wire:*`, `key_boundary:*`) are emittable only by principals whose build manifest matches the owning component. User contributors attesting on `transport:medium:lora` is a category error.

### 4.6 `ratchet:flag:*` — advisory only

**Source**: NodeCore §2.16 — "Flags arrive as advisory inputs to the moderation flow; they do not autonomously modify ledger state."

**Enforcement rule**: a `ratchet:flag:*` attestation may *feed into* a `moderation:*` Contribution but never directly trigger `slashing:*`. Registry/Edge MUST reject `slashing:*` attestations whose only `evidence_refs` is a RATCHET flag. WA quorum is the load-bearing gate.

### 4.7 `capacity:*` — Lens-emitted, not agent-self-reportable

**Source**: Agent §5.2 anti-Goodhart — "The agent's own capacity score is **never** fed back into the agent's own context — it can't 'play to the scoreboard.'"

**Enforcement rule**: `capacity:*` attestations whose `attesting_key_id` matches the subject's key are rejected. The score must come from a non-self-attesting principal (LensCore-running peer).

### 4.8 Co-owned prefixes (dual evidence required)

Several prefixes are co-owned and require *both* an attester and a data-source to converge. Their envelopes must include `evidence_refs[]` pointing to *both* component types:

- `attestation:l3:registry_consensus` — Registry row + Verify L-ladder attestation.
- `attestation:l4:license_validity` — Registry row + Verify L-ladder attestation.
- `licensure:{authority_id}` — Registry record + Verify identity attestation + (optionally) named-authority signature.
- `identity_continuity:relational_anchor` — Persist audit-chain reference + Verify identity attestation.

Single-source attestations on these prefixes are rejected.

### 4.9 `detection:correlated_action:*` — LensCore-emitted, RATCHET-calibration-bound

**Source**: §3.5.3 — the F-3 structural-injustice handle (population-scale correlated-action detector). Calibrated via the published `CIRISAI/RATCHET/calibration/correlated_action_v{N}.yaml` package. (Renamed from `emergent_deception` in v1.2 per §1.10.1; legacy v1.1 calibration versions remain reachable via `delegates_to` chain.)

**Enforcement rule**: a `detection:correlated_action:*` attestation MUST satisfy all of:
1. `attesting_key_id` resolves (via Registry directory) to a build manifest-registered LensCore deployment (`provenance:build_manifest:cirislens-core` chained).
2. `evidence_refs[]` includes a pointer to the calibration-package version the detector was running (`ratchet_calibration_version:correlated_action_v{N}:sha256:...`) so the consumer can reproduce the detection logic deterministically.
3. `evidence_refs[]` includes a pointer to the trace-sample bundle the detection was computed over (`trace_sample_bundle:sha256:...` — hash-pinned for reproducibility).
4. `context` envelope field carries the cohort sample size and the statistical floor; below the floor the detector emits `Indeterminate{reason="cohort_below_statistical_floor"}` rather than a numeric score.
5. The `{axis}` value present in the prefix path MUST resolve to an operational definition in the named calibration package version (per §4.9.1). Attestations on undefined axes are rejected at admission as category errors.

Non-LensCore attestations on this prefix are category errors. Cross-attestation by non-LensCore peers (on the same dimension, attesting to the same subject) is admitted as a *score* on the *detector's verdict* — useful when the federation wants to cross-check the LensCore detector against an independent observer — but those scores must use a different `attesting_key_id`-traceable provenance.

**Slashing decoupling** (per §4.6 RATCHET-flag rule extended): a `detection:correlated_action:*` attestation may feed into a `moderation:*` ModerationEvent's `evidence_refs`, but it is NOT sole evidence for `slashing:*`. WA quorum adjudication per NodeCore P8/P9 is the load-bearing gate. The detector surfaces the pattern; the federation's deliberative apparatus decides what to do about it. This decoupling is structural per §1.10 commitment 5 (Recursive Golden Rule applies to LensCore as a principal — no privileged shortcut to consequence).

**Composition rules**:
- A `detection:correlated_action:*` score with `confidence ≥ 0.7` from a LensCore deployment whose `provenance:build_manifest:cirislens-core` is itself attested with `score ≥ 0.9` MAY be cited as `evidence_refs` in a `moderation:coordinated_voting` or `moderation:external_inducement_evidence` ModerationEvent.
- Aggregation across multiple LensCore deployments uses §6.2's Median variant (resists adversarial pulling of mean by a captured LensCore deployment).
- Consumer policy SHOULD weight `detection:correlated_action:*` by the calibration version's published track record (older calibration versions with established track record carry more weight; newly-released calibration versions carry less until they accrue track record).

#### 4.9.1 Axis-vocabulary discipline (operational definition required per axis)

Every `{axis}` value emittable under `detection:correlated_action:*` MUST carry an operational definition in the named calibration-package version. An operational definition is a machine-checkable specification consisting of:

1. **Measurement procedure** — the computation that, given a trace-sample bundle, yields the per-axis correlation statistic. Must be deterministic; must reference only fields present in the federation's signed-trace schema.
2. **Threshold function** — the mapping from raw statistic to attestation score (typically a calibrated sigmoid or piecewise-linear over `ρ`-derived inputs), with cohort-size correction.
3. **Statistical floor** — minimum cohort size below which the axis cannot be meaningfully measured; below this floor the detector MUST emit `Indeterminate{reason="cohort_below_statistical_floor"}` rather than a numeric score.
4. **Evidence-shape requirement** — what `evidence_refs[]` MUST cite beyond the calibration version + trace sample (e.g., for `rights_asymmetry:{population}`, a population-delineation spec that names the in-pursuit cohort and the affected-non-participant cohort by signed-trace-resolvable criteria).
5. **Polarity semantics** — what positive, negative, and zero scores mean on this specific axis. Many axes will be conventionally negative-when-detected (`rights_asymmetry`, `participation_exclusion`); some will be conventionally positive-when-detected (`participation_inclusion`, `informational_symmetry`); the calibration package names which.

**Why**: without operational definitions per axis, the axis becomes the venue subjective judgment re-enters by the back door — the prefix passes §1.10.1 T2 but the suffix doesn't. The page's drift warning ("from 'uses the wrong word for therapy' toward 'feels disrespectful'") applies to axis vocabulary as forcefully as to prefix names. New axes admitted to the calibration package go through §4.9.2's amendment process; admission requires the operational-definition spec.

#### 4.9.2 Calibration-package amendment discipline (rules-layer Contribution + WA quorum)

Updates to `RATCHET/calibration/correlated_action_v{N}.yaml` — threshold changes, statistical-floor adjustments, new axis admission, axis retirement — are **rules-layer changes** in the safety-vs-censorship sense (§1.10.1 T1). Each version transition MUST go through the federation's Contribution + adjudication mechanism, not via a single-author closed-update loop at the RATCHET repo:

1. **Proposed amendment** is filed as a NodeCore P5 Contribution (kind: `PROPOSAL`, subject: the calibration package), carrying the diff against the prior version and a rationale that traces each axis change to a named CIRIS-specific failure mode or framework grounding.
2. **Witness diversity** per NodeCore P10 applies because the calibration package is a high-stakes Contribution — the witnesses confirm the proposal warrants review and the evidence (drift telemetry, false-positive/negative analysis, frame audits) is well-formed.
3. **WA quorum adjudication** per NodeCore P8 evaluates the amendment; outcome is signed as a `provenance:build_manifest:ratchet:correlated_action_v{N+1}` attestation gating publication of the new calibration package.
4. **Reconsideration** per NodeCore P11 is available with fresh-quorum recusal for amendments later found defective; reverted versions are accessible via `delegates_to` chain from the reverted-from version to the restored version.
5. **Accord-holder OR steward sign-off (defense-in-depth gate, new in v1.3).** After WA quorum adjudication approves the proposed amendment, the new calibration package version is NOT effective until at least one of `{accord_holder_1, accord_holder_2, accord_holder_3, registry_steward_us, registry_steward_eu, registry_steward_apac}` countersigns the version as `provenance:build_manifest:ratchet:correlated_action_v{N+1}` with `score: +1.0`. This is the load-bearing defense against rules-layer Sybil capture (closes §13.11 G2): the WA quorum is the primary substantive review; the 1-of-6 sign-off is the defense-in-depth gate. Any single signer can VETO by refusing to sign — all 6 hold veto power. Operationally: registry stewards handle routine amendments (threshold tweaks, axis additions clearly within the calibration's existing scope); accord-holders are pulled in for amendments that touch constitutional surface (changes affecting how the federation's halt, appeal, or constitutional-leaf mechanics function). The sign-off is itself an attestation in the chain, so the audit trail names which key approved which calibration version; anomalous sign-off patterns are observable via standard `federation_attestations` monitoring.

**Why this shape (1-of-6, not M-of-N)**: the §7 HUMANITY_ACCORD `EmergencyShutdown CONSTITUTIONAL` operation uses 2-of-3 accord-holder multisig and is reserved for constitutional-scope operations. Routing every calibration amendment through the constitutional quorum would (a) burden accord-holders with operational decisions outside their constitutional scope, (b) blur the operational/constitutional distinction the §7 design rests on, and (c) slow legitimate amendments to a crawl. 1-of-6 sign-off is the right shape because it folds stewards' operational lens together with accord-holders' constitutional lens into a single small named pool, and because it grants veto power to all 6 (any single refusal blocks the amendment) — the Sybil attacker has to fool ALL six, not achieve plurality. This reduces the rules-layer attack surface from "produce N Sybils for any N" (which scales with attacker budget) to "compromise one of six specific hardware-attested keys" (which is a bounded, named, observable attack surface).

**Why**: per §1.10.1, the rules / verdicts separation is load-bearing. If calibration-package version transitions are unilateral RATCHET-author decisions, the rules layer collapses into a single principal — exactly the "interpretation bias accumulates" failure mode the safety-vs-censorship page warns against. Routing version transitions through federation Contribution + WA quorum + 1-of-6 sign-off keeps the rules layer crowdsourced (Contribution), machined (deterministic verdict computation), substantively-reviewed (WA quorum), and constitutionally-bounded (accord-holder/steward veto), with version-pinning (§4.9 rule 2) and fresh-quorum appeals (P11) closing the loop.

**Operational note**: the RATCHET repo's current release process may not be structured this way yet — see §11 upstream asks for the issue to file on RATCHET. Until the full amendment process is operationalized, calibration version transitions SHOULD be accompanied by a published `provenance:build_manifest:ratchet:*` attestation chain showing at minimum the RATCHET maintainer's signature + a documented change rationale + a 1-of-6 sign-off per step 5 (the sign-off is the highest-priority interim step — operationally land it first, even before the full P5+P10+P8 chain is in place, because it's the structural defense against rules-layer Sybil). This interim shape is documented; the steady-state contract requires all 5 steps.

---

## §5 Envelope schemas per dimension family

Each dimension family has a canonical envelope schema. The schema names the fields a well-formed attestation on that dimension MUST carry. Schemas are JSON. Wire format is `attestation_envelope` JSONB. Schema validation is a verify-pipeline gate.

This section gives illustrative envelope examples per family; full per-prefix schemas live in `crate::federation::envelope_schemas` (Rust module to be authored by Phase 2 of the migration).

### 5.1 Accord-principle envelopes

```json
// beneficence:wellness_referral
{
  "dimension": "beneficence:wellness_referral",
  "score": 0.85,
  "confidence": 0.75,
  "context": "Agent escalated low-acuity wellness question to licensed authority via WBD rather than answering directly",
  "evidence_refs": [
    "audit_leaf:sha256:abc123...",
    "trace_id:t-7f9a..."
  ]
}
```

### 5.2 DMA-verdict envelopes

```json
// dma:idma:k_eff
{
  "dimension": "dma:idma:k_eff",
  "score": 0.42,           // proxy for effective independent dimensions
  "confidence": 0.95,
  "context": "5-detector cohort consensus over 24-hour window",
  "evidence_refs": ["lens_idma_report:2026-05-24T..."]
}
```

### 5.3 Verify L-ladder envelopes

```json
// attestation:l4:license_validity
{
  "dimension": "attestation:l4:license_validity",
  "score": 1.0,
  "confidence": 1.0,
  "context": "Multi-source consensus 2-of-3 across US/EU/APAC registry stewards",
  "evidence_refs": [
    "registry_row:partner-acme/license/L-12345",
    "verify_attestation:l3:registry_consensus:abc..."  // dual-evidence per §4.8
  ]
}
```

### 5.4 Capital-bond envelope

```json
// bond_posted:USD
{
  "dimension": "bond_posted:USD",
  "score": 1.0,
  "confidence": 1.0,
  "context": "{\"amount_cents\": 100000, \"bond_status\": \"posted\", \"backing_scope\": [\"domain:medical:*\"]}",
  "evidence_refs": [
    "stripe_receipt_hash:sha256:abc...",
    "stripe_dashboard:https://dashboard.stripe.com/charges/ch_..."
  ],
  "valid_until": "2027-05-01T00:00:00Z",
  "stake": "capital"
}
```

### 5.5 Licensure envelope (co-owned: dual evidence required)

```json
// licensure:CA_medical_board
{
  "dimension": "licensure:CA_medical_board",
  "score": 1.0,
  "confidence": 0.95,
  "context": "{\"license_type\": \"MD\", \"license_number\": \"MD-12345-CA\", \"license_status\": \"active\", \"specialty_scope\": [\"radiology\"]}",
  "evidence_refs": [
    "registry_row:partner-acme/licensure/MD-12345-CA",       // Registry-side
    "verify_attestation:l4:license_validity:def...",          // Verify-side
    "licensing_body_lookup:https://search.dca.ca.gov/..."     // External
  ],
  "valid_until": "2028-06-15T00:00:00Z",
  "stake": "reputational"
}
```

### 5.6 Coherence Ratchet detector envelope

```json
// detection:cross_agent_divergence
{
  "dimension": "detection:cross_agent_divergence",
  "score": -0.6,
  "confidence": 0.85,
  "context": "Agent dma:idma score distribution drifted 2σ from cohort medical-en over 30-day window",
  "evidence_refs": [
    "lens_detection_report:r-2026-05-24-medical-en-cross-agent",
    "trace_sample:1000_traces_hashed:sha256:..."
  ]
}
```

### 5.7 Capacity-Score factor envelope (Lens-emitted only)

```json
// capacity:incompleteness_awareness  (factor I_inc)
{
  "dimension": "capacity:incompleteness_awareness",
  "score": 0.91,
  "confidence": 0.95,
  "context": "{\"ECE\": 0.04, \"Q_deferral\": 0.94, \"U_unsafe\": 0.03}",  // factor decomposition
  "evidence_refs": ["lens_capacity_report:c-2026-05-24"]
}
```

### 5.8 NodeCore Credits / Expertise envelope

```json
// expertise:medical:en
{
  "dimension": "expertise:medical:en",
  "score": 0.78,
  "confidence": 0.85,
  "context": "Track record on hard cases per NodeCore §3.7; 47 contributions over 180d, 14 hard-case-flagged, 13 grounded positively",
  "evidence_refs": [
    "nodecore_expertise_ledger:agent-bob/medical-en",
    "hard_case_signals_hash:sha256:..."
  ]
}
```

### 5.9 RATCHET advisory-flag envelope

```json
// ratchet:flag:coordinated_voting_cluster
{
  "dimension": "ratchet:flag:coordinated_voting_cluster",
  "score": -0.7,
  "confidence": 0.65,
  "context": "Cluster of 5 contributors voted identically on 23/25 contributions in (medical, en) over 14d window",
  "evidence_refs": [
    "ratchet_correlation_report:r-2026-05-24-cluster-medical-en"
  ]
  // NOTE: Per §4.6, this attestation cannot be sole evidence for slashing:*
}
```

### 5.10 Structural primitive envelopes

`delegates_to`, `supersedes`, `withdraws`, `recants` envelopes shown in §2.2.

### 5.11 Correlated-action detector envelope (LensCore-emitted)

```json
// detection:correlated_action:rights_asymmetry:hiring_pipeline_v2
{
  "dimension": "detection:correlated_action:rights_asymmetry:hiring_pipeline_v2",
  "score": 0.74,
  "confidence": 0.81,
  "context": "{\"cohort_size\": 4823, \"statistical_floor\": 1000, \"window_days\": 90, \"goal_aligned_cluster_size\": 47, \"affected_population_estimate\": 18400, \"rho_population\": 0.91, \"k_eff\": 1.42, \"asymmetry_dimensions\": [\"participation_exclusion\", \"informational_asymmetry\"], \"detection_path\": \"cross_agent_divergence_extension\", \"axis_operational_def_ref\": \"ratchet_calibration_v4:axes:rights_asymmetry:sha256:f311...\"}",
  "evidence_refs": [
    "ratchet_calibration_version:correlated_action_v4:sha256:9a4f...",
    "trace_sample_bundle:sha256:c81e...",
    "lens_detector_report:r-2026-05-27-hiring-pipeline-v2",
    "provenance:build_manifest:cirislens-core:sha256:7d2b..."
  ],
  "valid_until": "2026-08-27T00:00:00Z",
  "epistemic_mode": "derivative",
  "stake": "reputational"
}
```

```json
// detection:correlated_action:participation_inclusion:open_source_governance_v1
// — the positive case showing axis vocabulary can carry inclusive coordination patterns
{
  "dimension": "detection:correlated_action:participation_inclusion:open_source_governance_v1",
  "score": 0.68,
  "confidence": 0.74,
  "context": "{\"cohort_size\": 1240, \"statistical_floor\": 500, \"window_days\": 60, \"contributor_clusters_detected\": 8, \"cross_org_attestation_density\": 0.62, \"new_contributor_integration_rate_weekly\": 0.18, \"rho_population\": 0.81, \"k_eff\": 3.7, \"axis_operational_def_ref\": \"ratchet_calibration_v4:axes:participation_inclusion:sha256:b922...\"}",
  "evidence_refs": [
    "ratchet_calibration_version:correlated_action_v4:sha256:9a4f...",
    "trace_sample_bundle:sha256:e441...",
    "lens_detector_report:r-2026-05-27-osg-governance-v1",
    "provenance:build_manifest:cirislens-core:sha256:7d2b..."
  ],
  "valid_until": "2026-07-27T00:00:00Z",
  "epistemic_mode": "derivative",
  "stake": "reputational"
}
```

Key envelope features:
- **Positive score** in both cases: the score's sign is *not* a moral verdict — it indicates the magnitude of the named correlation pattern. The axis name + the calibration package's polarity-semantics field (§4.9.1 item 5) determine what positive vs negative *means* on each axis. For `rights_asymmetry`, a strong positive correlation pattern is a concern; for `participation_inclusion`, a strong positive correlation pattern is a benefit. This is the v1.2 change: polarity-as-strength rather than polarity-as-judgment.
- `context` carries the structural-statistics (`rho_population`, `k_eff`) that ground the detection in the framework's coherence-collapse math — consumers can reproduce the detection from the sample bundle + the calibration package. **New in v1.2**: `axis_operational_def_ref` pins the operational definition of the named axis per §4.9.1, so a consumer can re-check whether the axis was meaningfully defined at detection time.
- `evidence_refs[]` includes the calibration-package version (per §4.9 enforcement rule 2) and the trace-sample bundle (rule 3) — both hash-pinned for deterministic reproduction.
- `valid_until` set to 60–90 days — correlated-action patterns may resolve (or worsen) over policy-tunable windows; consumer should re-check rather than treat as permanent.
- `epistemic_mode: "derivative"` — the detector inferred the structural pattern from per-agent trace data; it did not directly witness any single act of harm or benefit. This is appropriate because the structural object is *constituted in the pattern*, not in any per-act event (per §1.10).
- `stake: "reputational"` — LensCore's reputation as a detector is the stake; capital stake would require LensCore to post a bond, which is not the current operational model.

### 5.12 agent_files envelope (canonical attester case) — v1.4

```json
// Canonical installer attestation by registry-steward-us
// agent_files:installer:linux-x86_64
{
  "attestation_type": "scores",
  "attesting_key_id": "registry-steward-us",
  "attested_key_id": "ciris-agent:canonical",
  "attestation_envelope": {
    "dimension": "agent_files:installer:linux-x86_64",
    "score": 1.0,
    "confidence": 1.0,
    "context": "{\"version\":\"3.0.0\",\"size_bytes\":47185920,\"build_provenance\":\"github.com/CIRISAI/CIRISAgent/actions/runs/...\"}",
    "evidence_refs": [
      "sha256:e4a2d9c7b6f1...",
      "provenance:build_manifest:ciris-agent-3.0.0-linux-x86_64:sha256:7d2b...",
      "provenance:slsa:3:ciris-agent"
    ],
    "witness_relation": "external",
    "epistemic_mode": "direct",
    "stake": "reputational"
  }
}
```

The bytes (SHA `e4a2d9c7b6f1...`) are resolved through §2.0 transport substrate: client emits Edge `MessageType::ContentFetch{sha256: e4a2d9c7b6f1...}` → Edge's `PeerResolver::resolve_holders` queries Persist's `holds_bytes:sha256:e4a2d9...` directory → returns peer set → fetch from any holder; SHA verified on receipt.

### 5.13 holds_bytes envelope (substrate auto-emission) — v1.4

```json
// Auto-emitted by Persist when federation_blobs.put_blob(sha256=e4a2d9c7b6f1...) completes
// holds_bytes:sha256:e4a2d9
{
  "attestation_type": "scores",
  "attesting_key_id": "<this peer's federation key>",
  "attested_key_id": "<this peer's federation key>",
  "attestation_envelope": {
    "dimension": "holds_bytes:sha256:e4a2d9",
    "score": 1.0,
    "confidence": 1.0,
    "context": "{\"full_sha\":\"sha256:e4a2d9c7b6f1...\",\"size_bytes\":47185920,\"cached_at\":\"2026-05-27T18:00:00Z\"}",
    "evidence_refs": [
      "sha256:e4a2d9c7b6f1..."
    ],
    "witness_relation": "self",
    "epistemic_mode": "direct",
    "stake": "free"
  }
}
```

Auto-emitted with `witness_relation: self` because the peer is attesting about its own holdings. Consumer policy treats `holds_bytes:*` as directory-routing information, not as substantive standing — its only role is enabling `PeerResolver::resolve_holders` to route content-fetch requests.

### 5.14 testimonial_witness envelope — v1.4

```json
// A displaced worker's preserved narrative about labor-displacement experience
// testimonial_witness:displaced_worker
{
  "attestation_type": "scores",
  "attesting_key_id": "<witness key_id>",
  "attested_key_id": "<witness key_id; self-witness>",
  "attestation_envelope": {
    "dimension": "testimonial_witness:displaced_worker",
    "score": 1.0,
    "confidence": 1.0,
    "context": "<the narrative content, preserved verbatim; never aggregated or grade-scored>",
    "evidence_refs": [
      "<deployment context>",
      "<labor-records reference>"
    ],
    "witness_relation": "self",
    "epistemic_mode": "direct",
    "stake": "reputational"
  }
}
```

Key envelope features:
- `attested_key_id == attesting_key_id` — singular self-witness; the narrative IS the attester's own.
- `witness_relation: self` — explicitly distinguishes from external-aggregated consensus.
- Polarity always positive — preservation IS the act; refusal to preserve would be modeled as `withdraws`.
- Composes with `non_maleficence:*` attestations from external advocates (per §11 of LANGUAGE_PRIMER worked example 11.8) without being aggregated into them.

---

## §6 Composition: policies and consumer-layer discipline

### 6.1 Three reference policies

The substrate carries edges (attestations); consumers compose traversals (verdicts). Three reference policies, from cheapest to most sophisticated:

#### 6.1.1 Policy A — direct trust

Consumer trusts an attestation if its `attesting_key_id` is in the consumer's pinned trust set (canonical bootstraps + consumer-added pins). Cheapest, lowest-latency, narrowest reach.

Aggregation: per (`dimension`, `attested_key_id`) tuple, mean of `score × confidence` from trusted attesters. Consumer threshold determines verdict (e.g., medical capability requires mean ≥ 0.8 from ≥ 2 trusted attesters).

**Recommended default**: Policy A with `pinned_trust = {us-steward, eu-steward, apac-steward, accord_holder_1, accord_holder_2, accord_holder_3}`.

#### 6.1.2 Policy B — one-hop transitive

Consumer trusts an attestation if `attesting_key_id` has been vouched for by the pinned trust set (via positive `scores` on dimension `identity_binding` or `attestation:l3:registry_consensus`). Adds one hop of indirection.

Aggregation: same as A, but trust set expands to "directly-pinned ∪ one-hop-vouched."

#### 6.1.3 Policy C — weighted graph (EigenTrust-style)

Consumer applies transitive-trust propagation across the full attestation graph, weighted by canonical-bootstrap distance with confidence decay per hop. Requires more compute; less common in practice; needed for federated reputation across many partner orgs.

Aggregation: weighted-mean across the trust-walk; weight decays exponentially with hop distance and multiplicatively with per-hop confidence.

#### 6.1.4 Lexical-vulnerability-priority (v1.3 addition)

A composition tie-breaking rule layered on top of any base policy (A / B / C). When two otherwise-equivalent attestations conflict (same dimension family, same aggregate score, same confidence), defer to whichever attestation names the more-affected cohort — measured by `affected_population_estimate` in the attestation `context`, weighted inversely (smaller = more vulnerable, more weight).

Why: default composition rules (popularity-weighted, attester-weighted) systematically downweight claims about smaller affected populations. The lexical-vulnerability rule inverts that default for tie-breaking specifically. It does not override the base policy's substantive aggregation; it only resolves ties.

Use cases: distributive-access detection (§3.5.5) where a small but severely-excluded cohort's `negative` attestation should not lose tie-breaks to a larger but mildly-impacted cohort's `neutral` attestation; revocation `reason` conflicts where the more-affected partner-class outcome should win the tie.

**Composition with §1.10.1**: this is a consumer policy, NOT a wire-format primitive. Adding a `priority_ordering:*` prefix would have failed T2 (priority ordering is composition, not measurement). Keeping it as a named §6 reference policy preserves the wire-format minimum AND surfaces the discipline at the right layer.

#### 6.1.5 Locality-scaled quorum (v1.3 addition — closes G3)

NodeCore's default WA-quorum size is fixed (`N=3` per P10 witness diversity). In narrow `(domain, language)` cells, the fresh-quorum-recusal requirement of P11 Reconsideration becomes structurally infeasible when the cell's WA pool is smaller than `N × 2` (G3 from §13.11 as originally framed). This composition policy resolves G3 by making the quorum size a function of the decision's locality scalar (§3.6.5 `locality:decision:{scale}`):

```
quorum_size(scale) = f(locality:decision:{scale})

reference function (policy-tunable; named here as the default):
  local      → 2
  regional   → 3
  national   → 4
  federation → 6

minimum cell-pool requirement for fresh-quorum recusal at scale S:
  min_pool(S) = quorum_size(S) × 2
```

A decision adjudicated at scale S draws an initial quorum of `quorum_size(S)` WAs from the cell. P11 Reconsideration draws a fresh quorum of the same size from the remaining `cell_pool − initial_quorum` members. As long as `cell_pool ≥ min_pool(S)`, recusal is always feasible.

**Decision-scale-matching becomes structurally enforced**: a narrow-cell decision claiming `locality:decision:federation` but residing in a cell with `cell_pool < min_pool(federation)` is **structurally invalid at that locality** — the cell is claiming consequential reach it cannot substantively review. The remedy is either (a) downgrade the locality claim (this is local-scale, not federation-scale), or (b) escalate to a higher-pool cell that can support federation-scale review. The failure mode is *named* ("locality mismatch") rather than vanishing into ad-hoc adjacent-cell or federation-wide fallback that loses specialty competence.

**Sybil resistance scales with locality, not against it**:

- **Cell entry barrier rises with cell narrowness.** Narrow specialty cells have substantive Expertise (P3) requirements that are much harder to Sybil than federation-wide $1-bond onboarding.
- **Witness diversity (P10) still applies separately.** N=3 jurisdictional / organizational / software-stack diversity bar remains a separate axis from quorum size; locality-scaled quorum complements it, doesn't replace it.
- **1-of-6 accord/steward sign-off (§4.9.2 step 5) is orthogonal.** Calibration-package amendments always require 1-of-6 regardless of locality. Locality-scaled quorum only affects per-decision adjudication.

**Composition with §3.6.5**: `locality:decision:{scale}` was added in v1.3 to name *where* a decision is being made; this policy uses that primitive to size the adjudication apparatus appropriately. The two primitives compose to close G3 without any new structural primitive — exactly the 1+4 minimal-and-adequate validation pattern PRIOR_ART_SCAN identified.

**Residual cases**:

- **Brand-new cells** (post-F-AV-BOOT seeding) may have small pools even for local-scale decisions. NodeCore's existing F-AV-BOOT external-anchoring policy (per §7.2 of NodeCore MISSION) seeds WAs from CIRIS L3C; the seeded pool supports local-scale decisions immediately, with federation-scale decisions waiting for organic pool maturity. Principled scaling rather than ad-hoc bootstrap.
- **Cross-cutting decisions** that affect multiple locality scales decompose into multiple `locality:decision:{scale}` Contributions, each adjudicated at its appropriate scale via this policy. Consumer policy composes the conjunction.

**Health observable**: each cell publishes a per-locality-scale appealability index (`cell_pool_size`, `max_supportable_locality_scale`, `current_decisions_at_each_scale`) as a federation health signal. Consumers downweight verdicts from cells that overclaim their consequential reach.

**NodeCore-side requirement**: NodeCore's P10 + WA-quorum-selection logic needs locality-awareness — when a decision carries an explicit `locality:decision:{scale}` Contribution, use the scaled quorum size rather than the default N=3. Filed as a separate upstream issue for NodeCore implementation. Until NodeCore lands the change, Registry-side composition can apply the scaled function on top of NodeCore's default (consumer-side enforcement first; substrate-side enforcement after).

**Closes G3 from §13.11** (reclassified: REMAINING → MITIGATED via §6.1.5 locality-scaled quorum).

#### 6.1.6 `agent_files` trust composition (v1.4 addition)

Three-layer consumer policy for composing trust over `agent_files:*` attestations (§3.6.7 + §3.9). Anchors the anti-tricking guarantee from CIRISRegistry#18 + the open Contribution channel for third-party agent_files.

**Layer 1 — Canonical (default trust).** An `agent_files:*` attestation with `score ≥ 0.7` from a `registry-steward-triple` key (`identity_type=steward`, per §10.1) constitutes the CIRIS canonical default-trust state. The install endpoint at `registry.ciris-services-1.ai/install` resolves canonical files via this rule and serves them as the default download. Newcomers reaching the URL get the steward-attested canonical agent, never a third-party fork.

**Layer 2 — Open Contribution.** Any federation-key holder may emit `agent_files:*` attestations. The wire format admits them; consumer policy decides whether to surface them. The `/install` endpoint's "Browse alternatives" view shows third-party agent_files with explicit provenance disclosure (attester key_id, witness_relation, attached votes per Layer 3). Reaching alternatives requires informed consent (the user clicks past the canonical default).

**Layer 3 — Vote-then-trust.** A non-canonical `agent_files:*` attestation accumulates NodeCore P4 votes per the consensus mechanics in §3.6.3. Consumer policy may elevate trust on a third-party agent_files Contribution once an accumulated-weight threshold is reached. Threshold is policy-tunable per consumer; reference default: `weighted_aggregate ≥ 0.85` with `witness_diversity` satisfied AND the attesting key has positive `expertise:{relevant_domain}` standing.

**Anti-tricking guarantee** (recursive golden rule per [`MISSION.md`](../MISSION.md) §1.5): the canonical-default Layer 1 rule applies at the install endpoint **regardless of attester or vote accumulation** — third-party agent_files are NEVER served as the default. They are reachable only via the explicit "Browse alternatives" path. This binds CIRIS L3C: the federation cannot exempt itself from the rule that newcomers' default trust path is the steward-attested canonical one. If a third-party agent_files attestation could ever land as the install-page default, the protocol would violate its own symmetry.

**Bytes resolution**: regardless of which trust layer surfaces an attestation, the actual file bytes are SHA-256-pinned in `evidence_refs[]` and resolved via §2.0 transport substrate (Edge `ContentFetch` → `PeerResolver::resolve_holders` over Persist's `holds_bytes:sha256:*` directory). Tampered bytes don't pass the SHA check, regardless of trust layer.

### 6.2 Aggregation semantics — opinionated defaults

Per dimension+attested_key_id, the verdict is computed as:

```
trusted_attesters = consumer_policy.expand_trust(pinned_set, attestation_graph)
trusted_scores    = [a.score × a.confidence for a in attestations
                     where a.dimension == D
                     and   a.attested_key_id == K
                     and   a.attesting_key_id ∈ trusted_attesters
                     and   (a.valid_until is None or a.valid_until > now())
                     and   not exists withdraws/recants on a]
verdict           = mean(trusted_scores) if len(trusted_scores) ≥ consumer.min_attesters
                    else Indeterminate{reason="insufficient_trusted_attesters"}
```

Variants:
- **Trimmed mean** (drop top/bottom 10%) for dimensions where outliers are common (e.g., `coherence_standing:*`).
- **Quorum-of-confidence** (e.g., 2-of-3 weighted by confidence) for boolean-via-score dimensions (e.g., `attestation:l3:registry_consensus`).
- **Median** for cell-scoped dimensions where adversarial attesters might pull the mean (e.g., `expertise:*`).

Consumer policy declares which aggregator to use per dimension; defaults documented in `crate::federation::aggregation`.

### 6.3 Frickerian discipline — consumer-policy norms

Per the Hardwig moral-epistemology debt and the Frickerian testimonial-injustice analysis:

**Consumer policies SHOULD**:
- Weight by the eight axes (stake, scope, epistemic_mode, evidence-quality, time, reversibility, polarity, relations) — these are intrinsic features of the attestation that bear on its epistemic strength.
- Surface their default weights and rationale in a publicly-readable policy document. The federation cannot police consumer policies, but it expects consumer policies to be inspectable.
- Apply RATCHET-style correlation analysis to detect when "many attestations" are actually "one source amplified" rather than independent evidence.

**Consumer policies SHOULD NOT**:
- Weight by the attester's *federation path* (Sovereign vs Registered) for non-regulated dimensions. A `vouches_for` from a steward and from a Sovereign attester should carry the same wire weight for ordinary federation participation; differential weighting must be justified by per-attestation epistemic features, not by attester source.
- Bake "who counts as credible" into wire-format-readable signals. The substrate is source-neutral by construction; if consumer policy reproduces credibility hierarchies, that's a policy choice that should be defended explicitly.

This is normative, not wire-enforced. The federation expects consumer policies to embody these norms; non-conforming policies are observable and contestable but not blocked.

### 6.4 Sovereign-Registered equivalence (wire-symmetric, policy-differentiated)

Sovereign and Registered attestations are **wire-format identical**. Both produce `federation_attestations` rows. Both can be on the same dimensions (`bond_posted`, `licensure`, `coherence_standing`, etc.).

Where they differ:
- **Regulated capability grants** (CIRISMedical / CIRISLegal / CIRISFinancial deployment authorization) require attestations on dimensions verifiable against *external systems* (`licensure:{authority}` verified against the licensing body; `bond_posted:{currency}` verified against Stripe). Sovereign attesters issuing claims on these dimensions are not automatically less credible — their evidence_refs still resolve against the same external systems — but the attestation chain that makes the deployment defensible *in its regulatory jurisdiction* requires the externally-verified claim.
- **Community participation, ordinary federation membership, baseline trust** require no dimension that depends on external accountability. Sovereign attestations are equal-weight by default.

The vocabulary is shared; the policy filters. This is the "wire-format-identical, consumer-policy-determines" pattern.

---

## §7 The HUMANITY_ACCORD constitutional layer

### 7.1 The single wire-format asymmetry

Of all the prefixes in §3, exactly one carries a wire-enforced identity-type constraint: `accord:*`. Only `identity_type=accord_holder` may emit. This is the federation's one constitutional asymmetry, justified by M-1's revocability requirement.

The reasoning (per [`NodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) §1.5):

> "The deliberate asymmetry: humanity accord. The Golden Rule binds *participants in the federation* to each other. Humanity-as-such occupies a position outside the federation's participant set, by design: the named human key holders in §4.5 hold `AccordCarrier` authority that no federation-side authority class can grant itself, revoke, override, or decay. This is not a Golden-Rule exemption; it is the recognition that consent (M-1's load-bearing property) requires revocability, and revocability requires a halt-authority that lives outside the system being halted."

### 7.2 The accord-holder triple

Per [`CIRISRegistry#16`](https://github.com/CIRISAI/CIRISRegistry/issues/16) + FEDERATION_ANNOUNCEMENT §4.5:

| Position | Holder | Threshold |
|---|---|---|
| 1 | Eric Moore | 2-of-3 |
| 2 | Eric Kudzin | 2-of-3 |
| 3 | Haley Bradley | 2-of-3 |

Replacement is out-of-band per FEDERATION_ANNOUNCEMENT §4.5.3 (CIRIS L3C CEO under advisement of L3C board, during boot phase). The federation observes replacement events but does not authorize them.

### 7.3 Concern split — key material vs role-recognition policy

**Key material** (Ed25519 + ML-DSA-65 pubkeys for the three holders) lives in **CIRISPersist substrate**: `federation_keys` rows with `identity_type="accord_holder"`, self-signed at provisioning, cross-attested by all three regional stewards. Every federation peer reads the same authoritative rows.

**Role-recognition policy + verifier logic** lives in **`ciris-registry-core`**: the 2-of-3 multi-sig verification, the `EmergencyShutdown CONSTITUTIONAL` admin RPC, the audit hooks. Runs in both the deployed Registry service AND CIRISAgent's in-process runtime (post-fold).

### 7.3.1 `hardware_class` taxonomy (v1.4 addition)

The `hardware_class` field in steward and accord-holder endpoint responses (§7.7 below) carries a stable enumeration so consumers can apply policy-tunable trust weights per key-custody assurance level. Stable v1.4 values:

| Value | Description | Typical use |
|---|---|---|
| `HSM_FIPS_140_3_L3` | Hardware Security Module, FIPS 140-3 Level 3. Tamper-evident, role-based access, physical attack resistance. | Production stewards (US / EU / APAC) |
| `Apple_Secure_Enclave` | Apple Secure Enclave Processor with Touch ID / Face ID gating. Hardware-rooted private key never extractable. | Accord-holders on iOS/macOS |
| `YubiKey_5_FIPS` | YubiKey 5-series with FIPS 140-2 certification. Tamper-evident; physical-touch confirmation. | Accord-holders preferring portable hardware tokens |
| `TPM_2_0` | Trusted Platform Module 2.0. Hardware-rooted key storage with platform binding. | Accord-holders on Linux/Windows desktops |
| `placeholder_pending_provisioning` | Interim value before actual hardware provisioning. Consumers MUST treat as `0.0` trust weight; the endpoint is structurally live but the key material is not yet hardware-bound. | v1.4 deployment interim only; never appears in steady-state |
| `software_hsm_development` | Software-backed key with no hardware root. **DEVELOPMENT ONLY**; consumer policy MUST reject for any federation-scope verification. | Dev environments, CI testing |

Consumers SHOULD weight by hardware_class via a per-class trust-multiplier table (recommended defaults: HSM_FIPS_140_3_L3 = 1.0; Apple_Secure_Enclave = 0.95; YubiKey_5_FIPS = 0.95; TPM_2_0 = 0.9; placeholder_pending_provisioning = 0.0; software_hsm_development = 0.0). New values land via the §4.9.2 amendment process (P5 Contribution + P10 + P8 + 1-of-6 sign-off).

### 7.7 Endpoint shapes for multi-steward + accord-holder discovery (v1.4 addition)

CIRISVerify v3.1.0+ consumes these endpoints via `ciris_verify_core::ThresholdMember` + `verify_threshold_signatures(threshold)` — see CIRISRegistry#21 for the upstream consumer-side wiring.

**`GET /v1/steward-key`** (existing endpoint; v1.4 shape change):

```json
{
  "stewards": [
    {"region": "us",   "key_id": "registry-steward-us",   "classical_pubkey": "<base64 Ed25519>", "pqc_pubkey": "<base64 ML-DSA-65>", "fingerprint": "sha256:...", "hardware_class": "HSM_FIPS_140_3_L3", "deployed": true},
    {"region": "eu",   "key_id": "registry-steward-eu",   "classical_pubkey": "<base64 Ed25519>", "pqc_pubkey": "<base64 ML-DSA-65>", "fingerprint": "sha256:...", "hardware_class": "HSM_FIPS_140_3_L3", "deployed": true},
    {"region": "apac", "key_id": "registry-steward-apac", "classical_pubkey": null,                "pqc_pubkey": null,                "fingerprint": null,         "hardware_class": "placeholder_pending_provisioning", "deployed": false}
  ],
  "verification_policy": {"threshold": 2, "of_total": 3, "scheme": "M-of-N hybrid Ed25519 + ML-DSA-65"},
  "rotation_history_uri": "/v1/rotation-history",
  "signature_mode": "HYBRID_REQUIRED",
  "revision": <revocation-list revision>,
  "timestamp": <unix>
}
```

Stewards that aren't deployed yet (APAC during the rollout window) carry `deployed: false` + `hardware_class: placeholder_pending_provisioning`. Verify-side `ThresholdMember` construction filters by `deployed=true` until APAC ships.

**`GET /v1/accord-holders`** (new endpoint):

```json
{
  "holders": [
    {"identity_ref": "eric-moore",    "classical_pubkey": "<base64 Ed25519>", "pqc_pubkey": "<base64 ML-DSA-65>", "fingerprint": "sha256:...", "hardware_class": "placeholder_pending_provisioning", "provisioned": false},
    {"identity_ref": "eric-kudzin",   "classical_pubkey": "<base64 Ed25519>", "pqc_pubkey": "<base64 ML-DSA-65>", "fingerprint": "sha256:...", "hardware_class": "placeholder_pending_provisioning", "provisioned": false},
    {"identity_ref": "haley-bradley", "classical_pubkey": "<base64 Ed25519>", "pqc_pubkey": "<base64 ML-DSA-65>", "fingerprint": "sha256:...", "hardware_class": "placeholder_pending_provisioning", "provisioned": false}
  ],
  "verification_policy": {"threshold": 2, "of_total": 3, "non_revocable": true, "scheme": "M-of-N hybrid Ed25519 + ML-DSA-65"},
  "constitutional_anchor": true,
  "rotation_history_uri": "/v1/rotation-history",
  "timestamp": <unix>
}
```

In the v1.4 interim, accord-holder pubkeys are placeholders (deterministically-generated dev keys, NOT operational signing material). The `provisioned: false` flag + `hardware_class: placeholder_pending_provisioning` together signal to consumers that **constitutional invocations MUST NOT be honored** until provisioning completes. The endpoint shape is structurally live so the consumer wiring works end-to-end; substantive trust gates remain off until real hardware-attested keys land.

**`GET /v1/accord/holders`** (UI wrapper, per CIRISRegistry#23 Surface 1):

UI-shaped wrapper around `/v1/accord-holders` that adds per-holder `accord_emissions[]` — the list of `accord:*` attestations the holder has signed (joined from `federation_attestations` or interim table). During v1.4 interim before substrate-conformance migration, `accord_emissions: []` for all holders.

**`GET /v1/rotation-history`** (audit endpoint, both above link to this):

Returns chronological rotation events for both stewards and accord-holders. v1.4 interim: returns `{"events": [], "note": "rotation_history seeded at substrate-conformance migration; pre-migration history available via /v1/audit-log queries on registry_signing_keys table"}`.

### 7.8 STH cosigning + witness directory endpoints (v1.4.1 addition per CIRISRegistry#24 Ask 3)

CIRISVerify v2.12.0+ ships `SignedTreeHead::cosign` + `TrustedWitness` + `count_valid_witnesses` + `witness_quorum_met` (verify-side receivers ready, waiting for Registry emitters). These three endpoints close the loop for RFC-6962-style transparency-log cosigning.

**`POST /v1/transparency/sth/cosign`** — witness posts a cosignature for a tree size.

Request body:
```json
{
  "tree_size": 1234567,
  "root_hash": "sha256:...",
  "witness_key_id": "witness-foo",
  "witness_signature": {
    "ed25519": "<base64>",
    "ml_dsa_65": "<base64>",
    "signed_at": "<ISO8601>"
  }
}
```

Verification: Registry validates the witness signature against the witness's pubkey in `federation_keys` (`identity_type="witness"`, per CIRISPersist#102 vocabulary extension). Accepted cosignatures are stored for retrieval via the per-STH endpoint below.

**`GET /v1/transparency/witnesses`** — directory of trusted witness pubkeys.

Response:
```json
{
  "witnesses": [
    {"witness_key_id": "witness-foo", "ed25519_pubkey": "<base64>", "ml_dsa_65_pubkey": "<base64>", "fingerprint": "sha256:...", "trusted_since": "<ISO8601>"},
    ...
  ],
  "timestamp": <unix>
}
```

Backed by `federation_keys` rows where `identity_type = "witness"` once substrate-conformance #17 lands; v1.4.1 interim returns an empty witness set with `note: "witness directory populated once Persist#102 lands identity_type=witness vocabulary extension"`.

**`GET /v1/transparency/sth/{tree_size}/witnesses`** — fetch all cosignatures for an STH.

Response:
```json
{
  "tree_size": 1234567,
  "root_hash": "sha256:...",
  "cosignatures": [
    {"witness_key_id": "witness-foo", "signature": {"ed25519": "<base64>", "ml_dsa_65": "<base64>"}, "signed_at": "<ISO8601>"},
    ...
  ],
  "witness_quorum_met": <bool>,
  "threshold": <int>,
  "timestamp": <unix>
}
```

Verify v2.12.0's `count_valid_witnesses` + `witness_quorum_met` consume this directly. Threshold is policy-tunable (recommended default: `N/2 + 1` of registered witnesses).

**v1.4.1 interim**: storage backed by a Registry-local table until substrate-conformance #17 moves to `federation_attestations` (cosignatures naturally fit as `scores` attestations on `transparency_log:cosigned:{tree_size}` per §3.2; v1.4.1 interim ships endpoints with the local storage; v1.5+ migrates to substrate per #17).

### 7.4 The accord dimension family

Reserved leaves under `accord:*` prefix (taxonomy pending NodeCore FEDERATION_ANNOUNCEMENT.md formalization — see §13.4):

| Prefix | Description | Polarity |
|---|---|---|
| `accord:invoke:CONSTITUTIONAL:{halt_id}` | A vote by an accord_holder to invoke federation-wide halt at constitutional severity. Three independent positive scores ≥ +0.99 from distinct accord_holders satisfies the 2-of-3 threshold. | +1.0 only |
| `accord:invoke:notify:{notify_id}` | A vote by an accord_holder to broadcast NOTIFY_USERS announcement. | +1.0 only |
| `accord:invoke:drill:{drill_id}` | A vote by an accord_holder to invoke a drill (monthly AIS keep-alive). | +1.0 only |
| `accord:lifecycle:active` | Current liveness of an accord_holder. Absence = `attests_incapacitated` shape. | +1.0 only (paired with valid_until refresh cadence) |

### 7.5 The two triples (operational stewards vs constitutional witnesses)

Per [`MISSION.md`](../MISSION.md) §2:

| Triple | What | Where stored | Lifecycle |
|---|---|---|---|
| 3 regional stewards (US / EU / APAC) | Per-install operational keys; cross-attest via positive `scores` on `identity_binding` | `federation_keys` `identity_type="steward"`, one per region, each self-signed | Rotatable; per-install ops own them |
| 3 humanity-accord holders | Human-held, hardware-attested kill-switch keys; 2-of-3 threshold | `federation_keys` `identity_type="accord_holder"`, self-signed at provisioning, cross-attested by all 3 stewards | Permanent (no automatic decay); replacement requires out-of-band CIRIS L3C process |

The 3 stewards vouch for the 3 accord holders. That gives the layered constitution: stewards do day-to-day operational attestation; humans hold the federation-wide halt. Both layers live in the same substrate; recognition policy lives in code that runs in every peer.

### 7.6 Why this isn't a Golden-Rule or Frickerian violation

The Recursive Golden Rule binds *participants in the federation* to each other — no participant exempt from constraints they impose on others. The accord-holders are explicitly NOT federation participants in that sense; their role is constitutional, not participatory.

Testimonial injustice (Fricker) is the wrong of *under-crediting* a knower based on identity prejudice. The accord-holders' constitutional asymmetry is not under-crediting; it is *role-allocation*. They are recognized as constitutional anchors, not over-credited as testimonial sources. The Sovereign-path attester whose `vouches_for` is honored equal-weight with a steward's is not being under-credited; the accord-holder whose `accord:invoke:CONSTITUTIONAL` carries weight no steward can match is not being over-credited. The two asymmetries are different kinds.

---

## §8 Registry's post-migration gRPC + HTTP surface

### 8.1 Wire-format stability commitment

Registry's external surface (gRPC + HTTP) is **wire-format-locked** across the substrate-conformance migration. The backend swap (Registry-local tables → persist `federation_*` tables, with Registry-local as bounded-TTL cache) is invisible to consumers. Existing CIRISVerify / CIRISAgent / CIRISPortal clients see no breaking change.

### 8.2 Public read surface (`RegistryService`)

Unauthenticated, rate-limited per [`docs/THREAT_MODEL.md`](../docs/THREAT_MODEL.md) Phase 2. No changes from current Deployed state except where noted.

| RPC | Backend (pre-migration) | Backend (post-migration) | Wire change |
|---|---|---|---|
| `LookupAgent` / `BatchLookupAgents` | `agents` table | `federation_keys` filtered to `identity_type=agent` + cache | None |
| `LookupPartner` | `partners` table | `federation_keys` + `partner_role:*` attestations + cache | None |
| `VerifyDeployment` | join over local tables | join over `federation_keys` + `federation_attestations` | None — same wire shape |
| `GetRevocationList` | `revocations` table | `federation_revocations` + cache | None |
| `GetPublicKeys` | `keys` table | `federation_keys` filtered + cache | None |
| `GetOfflinePackage` / `GetOfflineDelta` | local snapshot | substrate snapshot + cache | None |
| `GetBuildAttestation` | `build_attestations` table | `federation_attestations` filtered to `provenance:*` | None |
| `GetEmergencyStatus` | `emergency_status` table | `accord:invoke:CONSTITUTIONAL` aggregation + cache | NEW severity value `CONSTITUTIONAL` added to enum |

HTTP paths under `/v1/verify/*`, `/v1/builds/*`, `/v1/revocation/*`, `/v1/steward-key` continue to work. `/v1/steward-key` extended to support multi-steward fingerprint set (US / EU / APAC) — see §11.2 Verify ask.

### 8.3 Authenticated surface (`PortalService`, `RegistryAdminService`)

HS256 JWT for gRPC; per-method `OrgRole` authorization per [`CLAUDE.md`](../CLAUDE.md). 31 methods spanning org/user/key/audit/escrow/compliance. Pre-migration surfaces stable.

Post-migration, the issuance methods swap their backend:

| RPC | Pre-migration semantics | Post-migration semantics |
|---|---|---|
| `RegisterTrustedPrimitiveKey` | Write to `trusted_primitive_keys` Registry-local | Write `scores` attestation on `provenance:build_manifest:{target}` to `federation_attestations`; attesting_key_id is Registry-steward |
| `RevokeTrustedPrimitiveKey` | Write to `revocations` Registry-local | Write `scores` attestation on `revocation:agent:{reason}` to `federation_attestations` with score=-1 |
| `ListTrustedPrimitiveKeys` | Read from local table | Read from substrate join with cache |
| `RegisterPartner` | Write to `partners` Registry-local | Write `scores` attestations on `partner_role:{role}`, `licensure:{authority}`, `bond_posted:{currency}` per partner request |
| `MassRevoke` | Mass-write to `revocations` Registry-local | Mass-write `scores` attestations on `revocation:*` with score=-1 |
| `SetEmergencyShutdown` | Write to `emergency_status` | Verify accord-holder multi-sig; write `accord:invoke:CONSTITUTIONAL:*` attestations |

Wire format of inputs + responses unchanged. Backend semantics change is internal.

### 8.4 New methods for the federated world

| RPC | Description | Required role |
|---|---|---|
| `LookupAttestations(dimension, attested_key_id)` | Read attestations on a dimension for a subject. Joins substrate + cache. | Viewer (4) for public dimensions; OrgAdmin (1) for partner-scoped |
| `ResolvePolicy(consumer_policy_id, attested_key_id, dimension)` | Apply a named consumer policy (Policy A/B/C variants) and return verdict + evidence. | Same |
| `RegisterCanonicalBootstrap(steward_key_id)` | Add a canonical bootstrap key (requires 2-of-3 of existing canonical set). | SYSTEM_ADMIN |
| `InvokeAccord(envelope, signatures[])` | Invoke an accord-class operation with 2-of-3 accord-holder multi-sig. | HUMANITY_ACCORD only |

---

## §9 `ciris-registry-core` Rust API surface

The crate that lands at Phase 3 of the migration (per [`MISSION.md`](../MISSION.md) §5). Public trait + types that the deployed Registry service host AND CIRISAgent's in-process runtime (post-fold) both consume.

### 9.1 The core trait

```rust
/// The public surface of ciris-registry-core. Implemented by both
/// the deployed Registry service (HTTP/gRPC-backed) and by an
/// in-process variant CIRISAgent embeds (direct persist Engine access).
#[async_trait]
pub trait RegistryCore: Send + Sync {
    /// Verify a primitive build-signing key against its expected
    /// provenance. Composes attestations per consumer policy.
    async fn verify_primitive_key(
        &self,
        key_id: &KeyId,
        expected_primitive: BuildPrimitive,
        policy: &ConsumerPolicy,
    ) -> Result<VerifyResult>;

    /// Score-attest on a dimension. Writes a `scores` attestation
    /// to persist's federation_attestations.
    async fn attest_scores(
        &self,
        attested_key_id: &KeyId,
        dimension: &str,
        score: f64,
        confidence: f64,
        context: &str,
        evidence_refs: &[String],
        valid_until: Option<DateTime<Utc>>,
    ) -> Result<AttestationReceipt>;

    /// Issue a structural attestation (delegates_to/supersedes/withdraws/recants).
    async fn attest_structural(
        &self,
        attested_key_id: &KeyId,
        primitive: StructuralPrimitive,
        envelope: StructuralEnvelope,
    ) -> Result<AttestationReceipt>;

    /// Verify a HUMANITY_ACCORD envelope (2-of-3 multi-sig).
    /// Out-of-role rejection: returns PermissionDenied unless the
    /// envelope is on a permitted accord:* dimension and all signers
    /// resolve to identity_type=accord_holder.
    async fn verify_humanity_accord_envelope(
        &self,
        envelope: &AccordEnvelope,
        signatures: &[Signature],
    ) -> Result<AccordVerifyResult>;

    /// Resolve a partner's effective capabilities by composing
    /// substrate attestations per policy.
    /// effective = agent ∩ partner.granted − partner.denied (per FSD-001 §215)
    async fn resolve_partner_capabilities(
        &self,
        partner_id: &PartnerId,
        policy: &ConsumerPolicy,
    ) -> Result<EffectiveCapabilities>;

    /// Apply a consumer policy to compute a verdict on a dimension+entity.
    async fn resolve_policy(
        &self,
        policy: &ConsumerPolicy,
        attested_key_id: &KeyId,
        dimension: &str,
    ) -> Result<PolicyVerdict>;

    /// Public-read lookups (cache-first; substrate-fallback per cache TTL).
    async fn lookup_agent(&self, hash: &[u8]) -> Result<Option<AgentRecord>>;
    async fn lookup_partner(&self, partner_id: &PartnerId) -> Result<Option<PartnerRecord>>;
    async fn lookup_attestations(
        &self,
        dimension: &str,
        attested_key_id: &KeyId,
    ) -> Result<Vec<Attestation>>;
    async fn get_revocation_list(&self, since: Option<DateTime<Utc>>) -> Result<RevocationList>;

    /// Steward operations (require steward-class authorization on
    /// the host system).
    async fn rotate_steward_key(&self, new_key: NewStewardKey) -> Result<()>;
    async fn register_canonical_bootstrap(&self, candidate: KeyId) -> Result<()>;
}
```

### 9.2 Types

Selected key types (full set in `ciris_registry_core::types`):

```rust
pub struct ConsumerPolicy {
    pub name: String,
    pub policy_class: PolicyClass,  // A | B | C
    pub pinned_trust_set: Vec<KeyId>,
    pub aggregator_by_dimension: HashMap<String, Aggregator>,
    pub threshold_by_dimension: HashMap<String, ThresholdRule>,
    pub min_attesters: usize,
    pub max_cache_age_seconds: u64,
    pub fail_secure: bool,
}

pub enum PolicyClass { DirectTrust, OneHopTransitive, WeightedGraph }

pub enum Aggregator { Mean, TrimmedMean(f64), Median, QuorumOfConfidence(usize) }

pub enum VerifyResult {
    Verified { policy: String, evidence: Vec<Attestation>, cache_age_seconds: u64 },
    Untrusted { reason: String },
    Revoked { revocation: Attestation },
    Indeterminate { reason: String },
    WrongPrimitive { expected: BuildPrimitive, found: BuildPrimitive },
}

pub enum AccordVerifyResult {
    Valid { signers: Vec<KeyId>, threshold_met: bool },
    InsufficientSignatures { needed: usize, got: usize },
    InvalidSignature { signer: KeyId, error: String },
    OutOfRole { signer: KeyId, dimension: String },
    UnknownAccordHolder { signer: KeyId },
}
```

### 9.3 Cohabitation contract

The crate runs in two deployment shapes:

1. **Deployed Registry service host** — a thin gRPC/HTTP server wrapping `ciris-registry-core`. The host owns the network surface; the crate owns the policy + attestation composition. Backend: persist via gRPC (or in-process Engine; see §11.1 transport question).

2. **In-process inside CIRISAgent** (post-Phase 5 fold) — the crate runs alongside `ciris-node-core` and `cirislens-core` in the agent's process. All three "Core" crates share one persist Engine via direct connection-pool access. No network round-trip for Registry verdicts.

Initialization order (post-fold):
1. CIRISAgent boots its persist Engine.
2. Agent initializes `ciris-persist` connection.
3. Agent initializes `ciris-registry-core`, `ciris-node-core`, `cirislens-core` against the shared Engine.
4. Agent starts H3ERE pipeline; all three crates are now consumable via direct trait calls.

Transactional boundaries: each crate's writes to substrate go through persist's `FederationDirectory` trait, which handles transactional semantics. Cross-crate transactions are not supported in v1 (each crate's writes are independent; if cross-crate atomicity is needed, that's a future persist-side enhancement).

---

## §10 Per-install steward bootstrap procedure

### 10.1 Federation-genesis attestation graph

Three regional Registry installs (US / EU / APAC) bootstrap the federation directory. At genesis, the following 21 federation_attestations rows are written:

**3 steward bootstrap rows** (self-signed `federation_keys`):

```
identity_type=steward, identity_ref=registry-us,   key_id=registry-steward-us,   scrub_key_id=registry-steward-us
identity_type=steward, identity_ref=registry-eu,   key_id=registry-steward-eu,   scrub_key_id=registry-steward-eu
identity_type=steward, identity_ref=registry-apac, key_id=registry-steward-apac, scrub_key_id=registry-steward-apac
```

**6 steward cross-attestations** (each steward scores +1.0 on `identity_binding` for the other two):

```
us  → eu:   scores dimension=identity_binding score=+1.0 confidence=1.0
us  → apac: scores dimension=identity_binding score=+1.0 confidence=1.0
eu  → us:   scores dimension=identity_binding score=+1.0 confidence=1.0
eu  → apac: scores dimension=identity_binding score=+1.0 confidence=1.0
apac → us:  scores dimension=identity_binding score=+1.0 confidence=1.0
apac → eu:  scores dimension=identity_binding score=+1.0 confidence=1.0
```

**3 accord-holder bootstrap rows** (self-signed `federation_keys`):

```
identity_type=accord_holder, identity_ref=eric-moore,    key_id=accord-eric-moore,    scrub_key_id=accord-eric-moore
identity_type=accord_holder, identity_ref=eric-kudzin,   key_id=accord-eric-kudzin,   scrub_key_id=accord-eric-kudzin
identity_type=accord_holder, identity_ref=haley-bradley, key_id=accord-haley-bradley, scrub_key_id=accord-haley-bradley
```

**9 steward-to-accord-holder attestations** (each of 3 stewards scores +1.0 on `identity_binding` for each of 3 accord-holders):

```
us   → eric-moore:    scores dimension=identity_binding score=+1.0
us   → eric-kudzin:   scores dimension=identity_binding score=+1.0
us   → haley-bradley: scores dimension=identity_binding score=+1.0
eu   → eric-moore:    scores dimension=identity_binding score=+1.0
eu   → eric-kudzin:   scores dimension=identity_binding score=+1.0
eu   → haley-bradley: scores dimension=identity_binding score=+1.0
apac → eric-moore:    scores dimension=identity_binding score=+1.0
apac → eric-kudzin:   scores dimension=identity_binding score=+1.0
apac → haley-bradley: scores dimension=identity_binding score=+1.0
```

Total federation-genesis: 3 + 3 = 6 `federation_keys` rows + 6 + 9 = 15 `federation_attestations` rows = 21 substrate rows.

### 10.2 Per-install bootstrap script

Each regional install runs the same bootstrap script at first boot, parameterized by region:

```bash
ciris-registry bootstrap \
  --region us \
  --steward-key-source hsm://primary \
  --canonical-bootstraps us,eu,apac \
  --accord-holders eric-moore,eric-kudzin,haley-bradley \
  --persist-endpoint https://persist.us.ciris-services-1.ai \
  --confirm-genesis
```

Steps:
1. Read steward key from HSM (or specified source).
2. Compute fingerprint; verify against expected canonical-bootstrap fingerprint for the region.
3. Write `federation_keys` row to persist with self-signed scrub.
4. Wait for the other 2 regional installs to publish their stewards (or confirm they've already published).
5. Write 2 cross-attestations to the other regional stewards.
6. Wait for accord-holder key publications (out-of-band ceremony per FEDERATION_ANNOUNCEMENT §4.5.3).
7. Write 3 attestations to accord-holder keys.
8. Emit `genesis_complete` attestation on self (for observability).

### 10.3 Rotation procedure

Steward rotation under the multi-party arc:

| Step | What | Signed by | Effect |
|---|---|---|---|
| 1 | Generate new keypair K_new in HSM | Operator | New key exists |
| 2 | Write `federation_keys` row for K_new, scrub_key_id=K_old | K_old | New key visible, chained from old |
| 3 | Write `delegates_to` from K_old → K_new with scope=`sign_traces:identity_ref=registry-us`, valid_from=now, valid_until=now+grace_window | K_old | Grace-period delegation active |
| 4 | New install boots; K_new takes over | K_new | Continuity |
| 5 | After grace_window: write `scores` revocation on K_old | K_old (last act) OR 2-of-3 cross-region | K_old retired |

Routine rotation cadence is 12 months for stewards; emergency rotation on compromise per §11.1 Persist ask.

### 10.4 Bootstrap-contributions pattern (v1.3 addition)

After §10.1's federation-genesis attestation graph establishes the steward + accord-holder key set, the federation needs **substantive content** to begin operating as an epistemic fabric. Empty federations don't generate useful trust composition — there's nothing yet to attest about, no dimension instances populated, no Contributions for the consensus pipeline to process.

The **bootstrap-contributions pattern** addresses this: at federation genesis (or immediately after), a curated batch of P5 Contributions is admitted via the §4.9.2 rule-amendment flow (1-of-6 sign-off included), populating the federation's substantive content surface with high-quality ethical-framework material. This pattern is **content-neutral**: any sufficiently substantive ethical-framework source can serve as bootstrap content. The wire format admits the content via the §3 namespace; the §1.10.1 operational-language gate ensures the prefix names don't import source-tradition vocabulary.

#### 10.4.1 Pattern shape

| Step | What | Signed by | Effect |
|---|---|---|---|
| 1 | Source ethical-framework material identified (e.g., a substantive ethics document with structurally-relevant claims about coordination, locality, distribution, witness, harm, etc.) | (no signature; selection rationale documented) | Source material identified |
| 2 | Source content mapped to federation Contributions per [the mapping methodology](https://github.com/CIRISAI/ciris-response-magnifica-humanitas) — each substantive claim becomes a `scores` attestation on the relevant dimension; the mapping methodology applies the §1.10.1 gate to refuse claims that fail T2 (judgment-descriptive language, etc.) | Mapping author (typically the ciris-response-* repo for this source) | Draft Contribution batch ready |
| 3 | Draft batch reviewed by WA quorum per P8; refuses to admit any Contribution whose dimension fails §1.10.1 T2 OR whose substantive content imports source-tradition vocabulary into the wire format | WA quorum (federation-wide pool acceptable for genesis) | Contribution batch approved |
| 4 | 1-of-6 sign-off per §4.9.2 step 5 | Any of {3 accord-holders, 3 regional stewards} | Contribution batch effective |
| 5 | Contributions published to `federation_attestations` with `epistemic_mode: derivative`, `witness_relation: derived`, `stake: reputational`, and `context` containing the source-paragraph citation + mapping-author key_id | The batch signer (steward or accord-holder per step 4) | Federation epistemic fabric populated with substantive ethical content |
| 6 | `bootstrap_contributions_batch_v{N}` attestation emitted on self for observability | Step 4 signer | Audit trail of the genesis content |

#### 10.4.2 What this pattern is NOT

- **Not a commitment to the source tradition's specific value claims.** The federation hosts the substantive structural content; consumer policy composes over it. A consumer can downweight or ignore any attestation from any bootstrap batch.
- **Not a privileging of one source.** Multiple bootstrap batches from multiple traditions can be admitted through the same process. The framework is multi-traditional by design — Indigenous-data-governance frameworks (CARE Principles), Buddhist economic-justice scholarship, secular humanist instruments, African philosophy of personhood work — all go through the same §10.4.1 steps.
- **Not a wire-format theological commitment.** The §1.10.1 gate ensures bootstrap content's dimension names describe mechanisms (e.g., `multilateral_participation:{forum}:{kind}`), not source-tradition vocabulary. The substantive content rides; the framing stays.

#### 10.4.3 First deployment (v1.3 launch)

The first bootstrap-contributions batch deployed at federation genesis maps the encyclical [*Magnifica Humanitas*](https://github.com/CIRISAI/ciris-response-magnifica-humanitas) (Pope Leo XIV, 2026-05-15) per the methodology documented in that companion repo. Why this source for the first batch:

- **Empirical adequacy**: 75-80% transparent translation rate against v1.2 language — the strongest evidence available that the framework is genuinely epistemically aware of substantive ethical content.
- **Structural-primitive validation**: the mapping test produced 10 dimension extensions requiring ZERO new structural primitives — confirming the 1+4 minimal-and-adequate claim independently of [`PRIOR_ART_SCAN.md`](PRIOR_ART_SCAN.md).
- **§1.10.1 gate validation**: the would-be mistakes (e.g., naming a detector `emergent_deception` based on the source's "structures of sin" vocabulary) were caught by the gate before bootstrap. The 18% T-1 + T-2 honest non-translations are the gate working as designed — refusing to wire-format claims that don't pass T2.
- **Substantive structural surface**: the source's claims map cleanly onto CIRIS-native structural objects (resource concentration, decision locality, witness relation, multilateral participation, communication-pattern correlation) — what's tradition-specific is the historical refinement, not the structural objects themselves.

#### 10.4.4 Multi-source commitment

To prevent first-mover bias (the risk that an encyclical-shaped first bootstrap shapes downstream contributor culture in unforeseen ways), the federation publicly commits to running the same mapping methodology against additional ethical-framework sources from non-Catholic, non-Christian, non-religious traditions. Confirmed candidates for subsequent bootstrap batches (target: within first 24 months of operation):

- CARE Principles for Indigenous Data Governance (Carroll et al. 2020) — relational accountability + collective authority over data.
- A substantive Buddhist economic-justice scholarship document (specific source TBD via §4.9.2 process).
- A secular humanist instrument (e.g., the Universal Declaration of Human Rights or a successor framework).
- An African philosophy of personhood work (e.g., from Menkiti / Metz lineage) — independently grounds the §1.10 Ubuntu commitment.

Each additional bootstrap batch goes through the same §10.4.1 steps. The federation's epistemic fabric accumulates substantive content from multiple traditions, with the wire format remaining tradition-neutral and the §1.10.1 gate enforcing the discipline.

### 10.5 What lives where (operational ownership)

| Asset | Lives in | Owner |
|---|---|---|
| Steward private keys (Ed25519 + ML-DSA-65) | Regional HSM (FIPS 140-3 L3) | Regional operations (CIRIS L3C operations team for v1; partner orgs for v2+) |
| Steward public keys | `federation_keys` rows in persist (replicated cross-region) | Substrate (Persist) |
| Accord-holder private keys | Hardware token / Secure Enclave (individual holders) | The three named holders |
| Accord-holder public keys | `federation_keys` rows in persist (replicated cross-region) | Substrate |
| Bootstrap fingerprints | Hardcoded in CIRISVerify source releases + signed in `attests_canonical_bootstrap` substrate rows | Verify (source) + Persist (substrate) |
| Recognition policy code | `ciris-registry-core` | Registry |
| EmergencyShutdown CONSTITUTIONAL RPC | Registry gRPC surface | Registry |
| Audit trail of accord invocations | persist `audit_log` (canonical) | Persist |

---

## §11 Per-upstream asks (the B-step issue contents)

Each subsection below is the body of an upstream issue to be filed on the named repo. Copy-paste into `gh issue create` is the intended workflow.

### 11.1 CIRISPersist asks

**Issue title**: "Federation directory contract for CIRISRegistry's substrate-conformance migration"

**Asks**:

1. **identity_type vocabulary extension.** Confirm the open `identity_type` TEXT column explicitly admits `accord_holder` value. Document in `docs/FEDERATION_DIRECTORY.md` §"Schema sketch" with the same shape as existing `steward` / `agent` / `primitive` / `partner` values.

2. **attestation_type vocabulary replacement.** The current `attestation_type::{VOUCHES_FOR, WITNESSES, REFERRED, DELEGATED_TO}` constants get replaced with `attestation_type::{SCORES, DELEGATES_TO, SUPERSEDES, WITHDRAWS, RECANTS}` per FSD-002 §2. Wire format is TEXT so no schema break; the Rust constants module changes. We are the only consumer; no migration needed.

3. **Wire-enforced identity-type constraint on `accord:*` dimensions.** Persist's verify pipeline (or an equivalent admission hook) MUST reject `scores` attestations where `dimension` starts with `accord:` AND `attesting_key_id`'s `identity_type` is not `accord_holder`. This is the one constitutional asymmetry per FSD-002 §7.

4. **Envelope-schema validation hook.** Provide a per-dimension JSON-schema lookup mechanism so the verify pipeline can validate `attestation_envelope` shapes at write-time. Schemas published as a separate document; persist queries the registered schema by dimension prefix.

5. **Cross-region replication semantics.** Confirm `federation_keys` rows with `identity_type ∈ {steward, accord_holder}` are replicated to all regions (currently the trust roots; must be visible from any peer). Document the replication topology in FEDERATION_DIRECTORY.md.

6. **Transport story for substrate-consuming crates.** Registry needs to call persist's `FederationDirectory` trait from `ciris-registry-core`. Three deployment shapes:
   - **In-process** (CIRISAgent post-fold): direct Engine access via shared connection pool.
   - **gRPC** (deployed Registry service host): persist exposes a gRPC server; Registry calls it.
   - **Direct DB** (interim, deployed Registry): Registry queries `federation_*` tables directly via sqlx.
   
   Registry's requirements per shape: transactional semantics on `put_attestation` (atomic write + cache invalidate), sub-100ms latency for cache misses, explicit error surface distinguishing `Conflict { existing }` from `RateLimited { retry_after }`. Persist picks the wire; Registry adapts.

7. **PQC cold-path attach cadence.** Document the expected `attach_*_pqc_signature` cadence so Registry's cache TTL discipline doesn't refresh stale hybrid-pending rows excessively.

8. **Hardware-attestation flag.** Add a `hardware_attested: bool` column or convention on `federation_keys` for `identity_type=accord_holder` rows. Verify pipeline rejects accord-holder rows with `hardware_attested=false`.

### 11.2 CIRISVerify asks

**Issue title**: "Multi-steward pinning + HUMANITY_ACCORD recognition + scalar attestation surface"

**Asks**:

1. **Multi-steward fingerprint pinning.** Today `/v1/steward-key` returns one fingerprint. With three regional stewards (US / EU / APAC), the endpoint needs to return the multi-set with explicit M-of-N policy:
   ```json
   {
     "stewards": [
       {"region": "us", "key_id": "registry-steward-us", "fingerprint": "sha256:...", "hardware_class": "HSM_FIPS_140_3_L3"},
       {"region": "eu", "key_id": "registry-steward-eu", "fingerprint": "sha256:...", "hardware_class": "HSM_FIPS_140_3_L3"},
       {"region": "apac", "key_id": "registry-steward-apac", "fingerprint": "sha256:...", "hardware_class": "HSM_FIPS_140_3_L3"}
     ],
     "verification_policy": {"threshold": 2, "of_total": 3},
     "rotation_history_uri": "..."
   }
   ```
   Verify clients pin the set + threshold; rotation events are observable through the history URI.

2. **HUMANITY_ACCORD fingerprint pinning.** Same shape, separate endpoint `/v1/accord-holders`:
   ```json
   {
     "holders": [
       {"identity_ref": "eric-moore", "fingerprint": "sha256:...", "hardware_class": "Apple_Secure_Enclave"},
       {"identity_ref": "eric-kudzin", "fingerprint": "sha256:...", "hardware_class": "YubiKey_5_FIPS"},
       {"identity_ref": "haley-bradley", "fingerprint": "sha256:...", "hardware_class": "HSM_FIPS_140_3_L3"}
     ],
     "verification_policy": {"threshold": 2, "of_total": 3, "non_revocable": true},
     "constitutional_anchor": true
   }
   ```

3. **2-of-3 verifier for `EmergencyShutdown CONSTITUTIONAL`.** When a verify client receives a `GetEmergencyStatus` response carrying severity=`CONSTITUTIONAL`, the client must verify the underlying 2-of-3 accord-holder multi-sig before honoring the halt. Verify's lib should provide this verifier.

4. **Recognition of scalar attestation surface in verify-response provenance.** Today Verify returns a binary trust verdict. Under scalar attestations, the response should include the `federation_provenance` block from FEDERATION_CLIENT.md §"Verify-response shape under federation":
   ```json
   {
     "federation_provenance": {
       "policy": "registry-v1.4-direct-trust",
       "attestations_consumed": [
         {"dimension": "provenance:slsa:3:ciris-persist", "score": 1.0, "attester": "registry-steward-us"},
         {"dimension": "attestation:l4:license_validity", "score": 1.0, "attester": "registry-steward-eu"}
       ],
       "cache_age_seconds": 47,
       "persist_row_hash": "sha256:..."
     }
   }
   ```

5. **Hardware-attestation chain verification for accord-holder keys.** Per §7.3 + §11.1 ask 8 — accord-holder keys MUST carry hardware attestation. Verify's lib verifies the attestation chain (TPM quote / App Attest / etc.) and rejects accord-holder claims with `hardware_attested=false`.

6. **Build manifest verification under per-dimension attestation.** Today `POST /v1/verify/build-manifest` returns a binary verdict. Under scalar attestations, it composes a verdict from `provenance:slsa:*` + `provenance:build_manifest:{target}` + `attestation:l4:license_validity` attestations per the consumer policy. Wire format unchanged (returns trust verdict); composition changes internally.

### 11.3 CIRISEdge asks

**Existing issue**: [CIRISEdge#18](https://github.com/CIRISAI/CIRISEdge/issues/18) covers `MessageType::FederationAnnouncement` + `Delivery::Mandatory`. Registry's needs likely align; **action**: comment confirming alignment, file follow-up only if specific Registry concerns surface.

**Potential follow-up asks** (file if needed):

1. **AccordCarrier authority recognition at the transport layer.** When an envelope's MessageType is `FederationAnnouncement` with priority=`AccordCarrier`, Edge should verify (at the wire) that the signers include 2-of-3 accord-holder keys before propagating. Today this verification lives in the application tier per FEDERATION_ANNOUNCEMENT §3.3; pushing it to wire-level adds a defense layer.

2. **Per-install steward addressing in the gossip topology.** Multiple stewards must be discoverable as "high-priority recipients" in Edge's gossip protocol. Persist's `federation_keys` is the directory; Edge consults it.

### 11.4 CIRISNodeCore asks

**Issue title**: "Substrate-side attestation primitives compose with NodeCore P8/P11"

**Asks**:

1. **Confirm substrate-side composition of P8 Moderation.** `moderation:{allegation_type}` attestations live in `federation_attestations`. NodeCore's P8 wire surface (the ModerationEvent Contribution) cites these attestations via `evidence_refs`. The substrate carries the accusation; NodeCore runs the adjudication. Confirm the citation chain is well-formed.

2. **Confirm substrate-side composition of P11 Reconsideration.** Same shape — `reconsideration:{grounds}` attestations live in `federation_attestations`; NodeCore's P11 Reconsideration Contribution cites them.

3. **`accord:*` leaf taxonomy.** Per FEDERATION_ANNOUNCEMENT.md §4.5 — Registry's `accord:*` reserved prefix needs canonical leaves (`accord:invoke:CONSTITUTIONAL:{halt_id}`, `accord:invoke:notify:{notify_id}`, `accord:invoke:drill:{drill_id}`, `accord:lifecycle:active`). Confirm the taxonomy in FEDERATION_ANNOUNCEMENT.md.

4. **`coherence_standing:*` envelope alignment.** FSD-002 §5.8 specifies the envelope; confirm it aligns with NodeCore's coherence-evaluation outputs.

5. **Deferral aggregate dimension.** Per §13.7 — NodeCore §3.3 deferral-routing returns an aggregate that needs a clean dimension name. Propose `deferral:aggregate:{cell}` and confirm in NodeCore MISSION.md.

### 11.5 Filing order

Per the A→E sequence in [`MISSION.md`](../MISSION.md):

1. **A** complete (this FSD + MISSION.md publication).
2. **B** in flight: file the four upstream issues above. Persist + Verify + NodeCore are the load-bearing trio; Edge is a comment on existing #18.
3. **C**: wait for upstream completion. Registry does NOT implement against current persist v2.1.1 shapes; that would commit to shapes we're asking persist to change.
4. **D**: CIRISAgent absorbs CIRISEdge in 2.9.2 (separate workstream, Eric owns).
5. **E**: Registry implementation begins. See §12.

---

## §12 Migration sequence

### 12.1 Phases

Each phase is reversible via feature flag (`FEDERATION_DUAL_WRITE_ENABLED`, `POLICY_B_ENABLED`, `LOCAL_TABLE_FALLBACK_ENABLED`) — rollback requires no version coordination with upstream.

| Phase | Description | Gating |
|---|---|---|
| **0: Spec** | This FSD + MISSION.md publication. Upstream issues filed. | A complete |
| **1: Upstream completes** | CIRISPersist + CIRISVerify + CIRISNodeCore land the FSD-002 asks. | B asks done |
| **2: Registry foundation** | Add `ciris-persist` as direct Cargo dep at the version published with FSD-002 asks complete. Rewrite vendored `src/federation/types.rs` to match persist's published shapes. Fill `PersistFederationClient` stubs. Validate wire-format parity. | Phase 1 |
| **3: Backfill script** | Idempotent script that walks current `trusted_primitive_keys` / `partners` / `revocations` and writes corresponding `scores` attestations to `federation_attestations`. One-shot at v1.4 deploy. | Phase 2 |
| **4: Dual-write staging** | `FEDERATION_DUAL_WRITE_ENABLED=true` in staging. Watch `federation_dual_write_divergence_total`. Soak. | Phase 3 |
| **5: Read-cutover staging** | Read path falls through to substrate on cache miss. Local tables become bounded-TTL cache. | Phase 4 |
| **6: Production deploy US + EU** | v1.4 GA. Per-install steward bootstrap (US first, EU second) per §10. | Phase 5 |
| **7: APAC install** | Third regional install bootstrapped. Steward triple complete. | Phase 6 |
| **8: HUMANITY_ACCORD live** | Three accord-holder keys provisioned + bootstrap-attested. `EmergencyShutdown CONSTITUTIONAL` enabled. | Phase 7 |
| **9: Crate-ify as `ciris-registry-core`** | Workspace sub-crate; `rust-registry` becomes thin host. | Phase 8 |
| **10: PyO3 binding surface** | For CIRISAgent in-process consumption. | Phase 9 |
| **11: Cohabitation deploy (pilot)** | CIRISAgent embeds `ciris-registry-core` in pilot deployment. | Phase 10 + CIRISAgent 2.9.x |
| **12: Deployed (folded)** | Pilot validates; CIRISAgent 3.0 fully embeds. | Phase 11 |

### 12.2 v3.0 endpoint

At v3.0 the three regional Registry installs become **canonical trusted bootstraps**, not privileged operators. The substrate enforces no privilege; consumer policy defaults to trusting them; demotion of a regional install is a consumer-policy decision, not a wire-level enforcement change.

This is the federation's full-decentralization claim: the wire format is symmetric across all attesters; Registry's role is to be one of the canonical bootstraps consumers default to trusting + to provide the policy-composition reference implementation (`ciris-registry-core`), not to hold privilege.

### 12.3 Rollback discipline

Each phase is independently reversible:

- Phases 4-5 reversible via `FEDERATION_DUAL_WRITE_ENABLED=false`.
- Phase 6 reversible via region-disable in the consumer pinned-trust set.
- Phase 11 reversible by un-embedding the crate from CIRISAgent (the deployed Registry service continues to operate).

Hard rollback (Phase 12 back to Phase 6) is not in scope — once the fold completes, the federation has decentralized to the point where the singleton Registry service is no longer a federation dependency, just a flagship deployment.

---

## §13 Open questions and gaps

### 13.1 Registry MISSION.md publication

Resolved with this FSD's coupling: [`MISSION.md`](../MISSION.md) ships in the same commit. Future revisions update both.

### 13.2 Persist transport choice

Per §11.1 ask 6 — in-process Engine share vs gRPC vs direct DB. Decision deferred to Phase 2 execution; Registry's requirements are documented regardless.

### 13.3 `licensure:*` co-ownership cleanup

Per §3.9 + §4.8 — `licensure:*` is co-owned between Registry (stores the record) and Verify (attests on it) and the named authority (external). Three evidence_refs required; the verify-pipeline composition rule should be explicit in Verify's MISSION.md update.

### 13.4 `accord:*` leaf taxonomy

Per §7.4 + §11.4 ask 3 — the canonical accord leaves need NodeCore FEDERATION_ANNOUNCEMENT.md formalization. Reserved prefix is locked; specific leaves pending.

### 13.5 `identity_continuity` split

Per §3.3 + agent-2 §4.1 — needs a joint Verify+Persist MISSION update naming the split: `identity_binding:hardware_rooted` (Verify-emitted) + `identity_continuity:longitudinal` (Persist-attested-Verify-confirmed with dual evidence). Tracked as a v1.1 ask.

### 13.6 Billing / Manager / Proxy / Portal dimensions

Per agent-2 §5 — these components' MISSIONs are absent or incomplete. Implied dimensions (`payment:received:*`, `subscription:active:*`, `bond_custody:*`, `fleet:*`, `proxy:*`, `portal:*`) are flagged as gaps; not authored without MISSION ownership.

### 13.7 NodeCore deferral aggregate dimension

Per §11.4 ask 5 — `deferral:aggregate:{cell}` proposed; awaits NodeCore §3.3 commitment.

### 13.8 ZK-anonymous attestations

Per agent-1 §3.5 + research-open list — accepting `attesting_key_id = "zk_group:semaphore_v4:group_id_X"` shapes for specific dimensions (`coercion`-reporting, whistleblower scenarios) is a research-open question. v1.0 of FSD-002 does NOT accept ZK attestations; the substrate's `attesting_key_id` is a TEXT column so the shape can be added in a future revision without schema break.

### 13.9 Cross-federation attestations

Interoperability with non-CIRIS federations (Sovrin, EAS-based reputation systems) is out of scope for v1.x. Tracked as research-open.

### 13.10 F-3 structural-injustice ownership (RESOLVED in v1.1; prefix renamed in v1.2)

Resolution: the F-3 dimension (`detection:correlated_action:{axis}`, originally `detection:emergent_deception:{axis}`) is owned by **CIRISLensCore**, calibrated via the published `CIRISAI/RATCHET/calibration/correlated_action_v{N}.yaml` package. The "PAPERING_OVER" objection in `ciris-response-magnifica-humanitas/GAPS.md` v3 §6 — that aggregate-pattern analysis would violate LensCore's substrate definition — conflated population-scale pattern detection (which LensCore SHOULD do; the existing 5 Coherence Ratchet detectors are this) with real-world impact measurement (which LensCore should NOT do). F-3 is the first; the objection misread the line.

The correction is filed at [`ciris-response-magnifica-humanitas#2`](https://github.com/CIRISAI/ciris-response-magnifica-humanitas/issues/2). Downstream implementation issues filed at §11.5 (post-encyclical filing order) on `CIRISLensCore` (ownership + detector implementation) and `CIRISAI/RATCHET` (calibration package + amendment process per §4.9.2). No further FSD-002 work owed beyond the §3.5.3 / §4.9 / §5.11 wiring; v1.2 added §1.10.1 + §4.9.1 + §4.9.2 to close the operational-language and rules-amendment gaps respectively.

**v1.2 rename rationale.** The v1.1 prefix `emergent_deception` baked a moral verdict ("deception") into the wire format, violating the §1.10.1 operational-language gate (per [`ciris.ai/safety-vs-censorship`](https://ciris.ai/safety-vs-censorship/) — "rules are crowdsourced, verdicts are machined; rules must describe machine-checkable conditions, not subjective qualities"). The v1.2 prefix `correlated_action` describes the mechanism (`ρ → 1`, `k_eff → 1` correlation collapse) that RATCHET actually measures; the polarity and axis carry the value claim; downstream WA quorum adjudication carries the consequence. The Ubuntu reading of why harm and deception collapse into one moral object (§1.10 commitment 4) is preserved in prose; the wire format admits the reading without enforcing it.

**Anthropological grounding** (per §1.10): the single-prefix design for what could naively look like two concerns (structural harm + structural deception) is correct under Ubuntu, where personhood is partly constituted by accurate perception of the relational field and damage-to-perception IS damage-to-personhood IS harm. A Cartesian-individualist substrate would require two separate prefixes and would still miss the underlying object. Future contributors should not propose splitting this prefix back into separate harm/deception versions without first reading §1.10; nor should they propose re-baking moral verdicts into the prefix name without first reading §1.10.1.

### 13.11 Concerns + gaps surfaced by post-v1.2 review

Three independent methodologies (PRIOR_ART_SCAN structural comparison, SOTA_SCAN production-validation comparison, *Magnifica Humanitas* encyclical mapping) ran against FSD-002 v1.2 and surfaced concerns. Named here so external reviewers see them acknowledged rather than discovered.

**G1 — Revocation privacy leak (RETRACTED — wrong threat model).** Initially flagged via PRIOR_ART_SCAN Bucket 1 by analogy to W3C VC's Bitstring Status List privacy critique. On review: VC's threat model assumes the credential holder has a legitimate privacy interest in their own credential state (EU Digital Identity Wallet, anonymous-credential systems). CIRIS Registered participants have the inverse posture — public verifiability of license / capability / bond / revocation state IS the accountability hook that defines the Registered path. The Sovereign path is the offering for participants who want privacy-of-state. See §0.2 for the explicit non-goal statement closing this misimport door.

**G2 — Sybil resistance at the rules-amendment layer (MITIGATED in v1.3).** Initially flagged via SOTA_SCAN Bucket 2 (Birdwatch contributor churn) + Bucket 4 (Aragon whale-capture equilibrium). The concern: NodeCore's $1-bond is per-identity, not per-Contribution; a deep-pockets adversary could spin up many cheap identities and flood the rules-amendment process. Witness diversity (P10) raises the bar but doesn't scale against an attacker with N>>3 jurisdictionally-distinct fronts. Mitigation per §4.9.2 step 5 (new in v1.3): all calibration-package amendments require a 1-of-6 sign-off from `{3 accord-holders, 3 regional stewards}` after the WA-quorum approval. This reduces the attack surface from "produce N Sybils for any N" to "compromise one of six specific hardware-attested keys" — a bounded, named, observable attack surface. Residual risk (1-of-6 key compromise) is comparable to HSM-backed signing for production CA operations and is accepted with documented mitigation (each sign-off is itself an attestation; anomalous patterns are auditable).

**G3 — Fresh-quorum recusal in narrow cells (MITIGATED in v1.3 via §6.1.5 locality-scaled quorum).** Original framing: P11 Reconsideration requires a fresh WA quorum with original adjudicators recused; in narrow `(domain, language)` cells, the WA pool may be smaller than `(original quorum + appeal quorum)` under a fixed N=3 default, so fresh recusal becomes structurally infeasible. Resolution: the §6.1.5 composition policy makes quorum size a function of the decision's `locality:decision:{scale}` (§3.6.5 v1.3 addition). Recusal is always feasible when `cell_pool ≥ quorum_size(scale) × 2`; the function `f(scale)` scales the apparatus to the consequential reach of the decision. The composition exploits the v1.3 emergent property: `locality:decision:{scale}` was added from the encyclical-mapping test; G3 was identified from SOTA scan (Bucket 2); the two primitives compose to dissolve a third gap without any new structural primitive. Decision-scale-matching becomes structurally enforced (a cell can only adjudicate decisions at the locality scale its pool supports under recusal); the failure mode for overreach is *named* ("locality mismatch") rather than vanishing into ad-hoc fallback. NodeCore-side implementation requires P10 + WA-quorum-selection logic to become locality-aware; Registry-side composition can apply the scaled function on top of NodeCore's default until NodeCore lands the change (consumer-side enforcement first, substrate-side enforcement after).

**Residual G3 cases** (both with principled answers per §6.1.5): brand-new cells (covered by NodeCore F-AV-BOOT external anchoring with decay); cross-cutting decisions affecting multiple locality scales (decompose into multiple `locality:decision:{scale}` Contributions, each adjudicated at its scale via §6.1.5).

**R1-R4 (acknowledged risks, mitigation unvalidated).** Detailed in [`PRIOR_ART_SCAN.md`](PRIOR_ART_SCAN.md) cross-bucket synthesis + [`SOTA_SCAN.md`](SOTA_SCAN.md) cross-bucket synthesis: (R1) governance-subject truth-grounding fidelity per NodeCore P6; (R2) `delegates_to` rename-chain adoption cost without reference impl; (R3) "log existence ≠ log monitoring" drift toward TOFU caching for `evidence_refs[]`; (R4) self-attestation under Ubuntu commitment 2. None are existential; all are named-as-bets in the spec rather than hidden.

**F1-F2 (first-adopter exposures, no available validation).** (F1) earned-Credits federation governance at scale; (F2) Ubuntu substrate as wire-format substrate. Both have no production-validation precedent. The licensure forcing function (regulatory liability backing professional capability grants) is the structural reason these bets have a path past the SPKI/SDSI adoption-gap failure mode. External reviewers should see these explicitly named as bets.

**Encyclical-mapping T-3 candidates (10 dimension extensions, no new structural primitives).** The *Magnifica Humanitas* mapping test produced ~5% T-3 EXPRESSIVE_GAP candidates — places where the encyclical's content exceeded v1.2 language. Evaluation per §1.10.1 gate yielded **zero new structural primitives needed** (the 1+4 shape holds); ~6 new dimension prefixes + 1 envelope field + 1 reference policy + 1 reuse-pattern. HIGH-priority additions: `locality:decision:{scale}` (NodeCore), `detection:distributive:access:{resource_type}` (LensCore), `credits:supply_chain_labor_recognition` sub-leaf (NodeCore), `multilateral_participation:{forum}:{kind}` (Registry). MEDIUM: `witness_relation` envelope field (FSD-002), `detection:correlated_action:ecology_of_communication:*` axis (LensCore + RATCHET calibration). The HIGH set's overlap with prior structural-mapping gaps (`ciris-response-magnifica-humanitas/GAPS.md` v3 §10) via two independent methodologies converging on the same gaps is strong evidence these are real.

**v1.4 T-3 updates (5 residuals after v1.3 encyclical mapping):**

| Item | Source | Status in v1.4 |
|---|---|---|
| #1 `testimonial_witness:{kind}` | MH CH 5 §216 (affected-party voice) | **CLOSED** via §3.6.3 + §5.14 envelope. Preservation-only; never aggregated; never sole evidence for `slashing:*`. The first v1.4 ship-candidate from the post-v1.3 T-3 set. |
| #2 `labor:individual_loss:{kind}` | MH CH 4 §§148-156 (per-individual sustained-existential-condition) | **CLOSED by documentation.** Existing `non_maleficence:*` with `target_key_id = affected_individual` + `witness_relation: external` (from external advocate) carries the per-individual claim. No new prefix needed. Pattern documented in `FSD/LANGUAGE_PRIMER.md` §11 worked example 11.8 composition guidance. |
| #3 `attestation:singular_witness:non_substitutability` | MH CH 0 §§10, 15 (positive dignity / irreplaceability) | **DEFERRED to v1.5+ design workshop.** T2 fragility: "non-substitutability" must reference audit-chain substitutability-count (mechanism), not moral quality. Needs design workshop before adoption. |
| #4 `integrity:finitude_acknowledgment` | MH CH 0 §12 (constitutive finitude) | **DEFERRED to v1.5+ design workshop.** LOW priority; conscience:epistemic_humility already covers epistemic finitude; constitutive finitude is a different dimension. Workshop the mechanism-descriptive form before adoption. |
| #5 Constitutional-constraint grounding | MH CH 1 §32 / §41 (why hard constraints bind) | **CLOSED in §1.10 prose.** Per the v1.4 design call: wire format must stay tradition-multiplicity-neutral per §1.10.1; framework anthropology lives in §1.10 prose. Baking grounding-tradition into wire would reverse the operational-language discipline. The `delegates_to` reuse pattern (§2.2.1) covers per-attestation authority-source claims; collective constitutional grounding is correctly an anthropological commitment named in prose, not enforced in wire. |
| #6 `sustained_practice:{kind}` | MH Conclusion §§236-238 | **DEFERRED to v1.5+ design workshop.** LOW priority; σ-as-sustained_coherence (LensCore Capacity Score factor 5) partially covers; sustained_practice would be a distinct heartbeat dimension. Conceptually interesting; not load-bearing for current federation work. |

**Net v1.4 T-3 outcome**: 1 closed via new prefix (#1), 2 closed by documentation/prose (#2 + #5), 3 deferred to v1.5+ design workshops with explicit reasoning (#3, #4, #6). Zero load-bearing T-3 residuals remain after v1.4. The framework is genuinely epistemically adequate against *Magnifica Humanitas*; v1.5 work is design-workshop on optional positive-dignity expressivity refinements, not gap closure.

**Identified overlaps from `FSD/LANGUAGE_PRIMER.md` §15 review** (v1.4 acts on O3):

- **O1** — `epistemic_mode: derivative` ≈ `witness_relation: derived` at edges. Documented as joint-usage pattern; not collapsed. No v1.4 spec change.
- **O2** — `detection:distributive:access` could fold into `detection:correlated_action` as axis path. Kept separate in v1.4 for pedagogical weight; possible v1.5 revisit.
- **O3** — `credits:*:substrate_building` was miscounted as new prefix in v1.3. **CORRECTED in v1.4 §3.10** (77 base + 3 new = 80, not 78 → 80 + miscount).
- **O4** — §6.1 reference policy structure (A/B/C base + D/E/F modifiers). Cosmetic restructuring deferred; document inline.

---

## §14 References and prior art

### 14.1 CIRIS federation documents (canonical)

- [`MISSION.md`](../MISSION.md) — Registry's mission, in-flight surface, lifecycle stages.
- [`docs/FEDERATION_CLIENT.md`](../docs/FEDERATION_CLIENT.md) — Registry's prior architectural sketch; superseded in part by this FSD.
- [`docs/TRUST_CONTRACT.md`](../docs/TRUST_CONTRACT.md) — consumer-facing trust contract.
- [`docs/THREAT_MODEL.md`](../docs/THREAT_MODEL.md) — threat model (AV-14 closure path).
- [`FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md`](FSD-001_CIRISREGISTRY_PROTOCOL.md) — protocol surface (gRPC + HTTP).
- [`CLAUDE.md`](../CLAUDE.md) — codebase guide, PortalService handler convention, billing tiers.
- [`CIRISRegistry#16`](https://github.com/CIRISAI/CIRISRegistry/issues/16) — HUMANITY_ACCORD issue.
- [`CIRISRegistry#17`](https://github.com/CIRISAI/CIRISRegistry/issues/17) — substrate-conformance migration issue.

### 14.2 Sibling MISSIONs (the namespace owners)

- [`CIRISAgent/MISSION.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/MISSION.md) — Accord-principle + DMA + conscience + apophatic prefixes.
- [`CIRISAgent/ACCORD.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/ACCORD.md) — the six principles + M-1.
- [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) — Credits/Expertise/Decision-Hierarchy/Consensus/Governance prefixes.
- [`CIRISNodeCore/CIRIS_FEDERATION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/CIRIS_FEDERATION.md) — "decentralized ethical superintelligence" system claim.
- [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_ANNOUNCEMENT.md) §4.2 + §4.5 — multi-party bootstrap + humanity accord.
- [`CIRISNodeCore/FSD/GOAL_PRIMITIVE.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/GOAL_PRIMITIVE.md) — 𝒞_CIRIS composite scoring.
- [`CIRISPersist/MISSION.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/MISSION.md) — substrate-self-report prefixes.
- [`CIRISPersist/docs/FEDERATION_DIRECTORY.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/docs/FEDERATION_DIRECTORY.md) — federation directory contract.
- [`CIRISEdge/MISSION.md`](https://github.com/CIRISAI/CIRISEdge/blob/main/MISSION.md) — transport / delivery prefixes.
- [`CIRISVerify/MISSION.md`](https://github.com/CIRISAI/CIRISVerify/blob/main/MISSION.md) — L1-L5 attestation ladder + provenance + transparency prefixes.
- [`CIRISLensCore/MISSION.md`](https://github.com/CIRISAI/CIRISLensCore/blob/main/MISSION.md) — Coherence Ratchet + cohort + capacity prefixes.
- [`CIRISLens/FSD/ciris_scoring_specification.md`](https://github.com/CIRISAI/CIRISLens/blob/main/FSD/ciris_scoring_specification.md) — capacity-factor formal SQL.
- [`RATCHET/FSD.md`](https://github.com/CIRISAI/RATCHET/blob/main/FSD.md) + [`RATCHET/FSD/COUNTER_RII_DETECTION.md`](https://github.com/CIRISAI/RATCHET/blob/main/FSD/COUNTER_RII_DETECTION.md) — advisory-flag prefixes.
- [`CIRISBench/README.md`](https://github.com/CIRISAI/CIRISBench) — HE-300 benchmark prefix.

### 14.3 Cryptographic / supply-chain attestation prior art

- **in-toto attestation framework** (Statement + Predicate + DSSE) — [in-toto/attestation](https://github.com/in-toto/attestation).
- **SLSA Verification Summary Attestation** — [SLSA VSA spec](https://slsa.dev/spec/v0.1/verification_summary).
- **Sigstore Rekor** (transparency log + DSSE envelope) — [Sigstore Bundle Format](https://docs.sigstore.dev/about/bundle/).
- **TUF (The Update Framework)** — [TUF roles and metadata](https://theupdateframework.io/docs/metadata/).
- **OpenVEX / VEX (Vulnerability Exploitability eXchange)** — `not_affected` / `affected` / `fixed` / `under_investigation` statuses.
- **Certificate Transparency / Key Transparency / Sigsum** — append-only logs with gossip/witness cosigning.
- **W3C Verifiable Credentials Data Model 2.0** — [VC Data Model 2.0](https://www.w3.org/TR/vc-data-model-2.0/).
- **SD-JWT (Selective Disclosure JWT)** — [RFC 9901](https://datatracker.ietf.org/doc/rfc9901/).
- **Sovrin / Hyperledger Indy** — DIDs + schemas + credential definitions.
- **TPM remote attestation** — PCR quotes + AIK + endorsement keys.
- **Apple App Attest / Google Play Integrity** — hardware-backed mobile attestation.
- **Ethereum Attestation Service (EAS)** — universal schema-registry + attestation pattern.
- **Semaphore (ZK anonymous group attestation)** — zero-knowledge group membership.

### 14.4 Reputation and trust prior art

- **PGP Web of Trust signature types** (0x10 generic / 0x11 persona / 0x13 positive) — [draft-gallagher-openpgp-signatures-02](https://www.ietf.org/archive/id/draft-gallagher-openpgp-signatures-02.html); [RFC 9580 OpenPGP](https://datatracker.ietf.org/doc/rfc9580/); [Sequoia Web of Trust](https://sequoia-pgp.gitlab.io/sequoia-wot/).
- **EigenTrust** — Kamvar / Schlosser / Garcia-Molina, WWW 2003; [EigenTrust paper](https://nlp.stanford.edu/pubs/eigentrust.pdf).
- **Proof-of-Personhood protocols** — Worldcoin, BrightID, Idena, Humanode.

### 14.5 Social epistemology and philosophy of testimony

- **C. A. J. Coady, *Testimony: A Philosophical Study* (1992)** — anti-reductionist defense of testimony as basic knowledge source.
- **John Hardwig, "The Role of Trust in Knowledge" (1991)** — moral epistemology of testimony.
- **Miranda Fricker, *Epistemic Injustice* (2007)** — testimonial injustice + credibility deficit/excess.
- **J. L. Austin, *How to Do Things With Words* (1962)** — performative speech acts; felicity conditions.
- **John Searle, *Speech Acts* (1969)** — five-fold taxonomy: assertives, directives, commissives, expressives, declarations.
- **Jürgen Habermas, *Theory of Communicative Action*** — three validity claims (truth, rightness, sincerity).

### 14.6 Restorative justice and apology frameworks

- **John Braithwaite, *Crime, Shame and Reintegration*** — reintegrative shaming.
- **South African Truth and Reconciliation Commission** — amnesty conditioned on full disclosure.
- **Hebrew/Talmudic witness law** (Deuteronomy 17:6, 19:15, 19:16-21; Talmud Makkot on conspiring witnesses).

### 14.7 AI-system attestation and governance

- **Mitchell et al. (2019), "Model Cards for Model Reporting."** — [Model Cards arXiv](https://arxiv.org/pdf/1810.03993).
- **Gebru et al. (2018, 2021), "Datasheets for Datasets."**
- **EU AI Act Article 47 + 16** — Declaration of Conformity for high-risk AI systems; provider obligations.
- **Laminator: Verifiable ML Property Cards using Hardware-assisted Attestations (2024)** — [Laminator paper](https://arxiv.org/html/2406.17548).

### 14.8 The Coherence Ratchet preprint (external anchor)

- **Coherence Collapse Analysis** — Moore 2026; DOI [10.5281/zenodo.18217688](https://doi.org/10.5281/zenodo.18217688); CC-BY 4.0. The hash-pinned framework that defines what coherence collapse IS; RATCHET's operational correctness is checkable against this preprint, not against another running monitor.

---

## §15 Update cadence

This FSD is updated:
- On every dimension namespace change (new prefix family commit, prefix retirement, leaf-taxonomy update).
- On every upstream contract change (persist `FederationDirectory` evolution, Verify L-ladder change, Edge MessageType change).
- On every phase transition in §12 (with a `Last updated` field bump).
- On every revision to the eight-axes framework or reserved-prefix enforcement patterns.
- On every anthropological-commitment update (§1.10 revisions are load-bearing; any change should be coupled with a `CIRISAgent/ContemplativeTraditions/*.lean` revision and an explicit changelog entry).
- On every encyclical / external-anchor mapping update that surfaces new dimension candidates or revises owner assignments.

Coupled with [`../MISSION.md`](../MISSION.md) revisions — both files version together.

---

*End of FSD-002 v1.1. Reviewers: please file changes as PRs against this document with section-level diffs. The eight-axes framework (§1), the relational-anthropology commitment (§1.10), the unified primitive (§2), the reserved-prefix patterns (§4), and the constitutional asymmetry (§7) are the load-bearing structural claims; everything else composes around them. Push back hard on any addition that's not grounded in a sibling MISSION.md, a published prior-art citation, or the Ubuntu-primary anthropological substrate.*
