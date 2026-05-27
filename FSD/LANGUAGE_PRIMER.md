# LANGUAGE_PRIMER — Federation Wire Format as a Translation Grammar

**Version**: 1.0 (Registry-side canonical; v1.3-aligned).
**Companion documents**: [`FSD-002_FEDERATION_SURFACE.md`](FSD-002_FEDERATION_SURFACE.md) v1.3 (the wire format itself); [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) (the 15 consensus primitives that sit above the substrate-consuming tier).
**Purpose**: a working reference for **converting substantive ethical-framework content into structured Contribution wire envelopes** that the federation can carry. Symbolic-logic-style translation; not paraphrase. Where the wire format carries the claim cleanly, the conversion is rich. Where it strains, the gap is precise.
**Audience**: sub-agents (or humans) performing bootstrap-contributions mapping per FSD-002 §10.4; reviewers verifying translation quality; future contributors proposing new content-source bootstrap batches.
**Last updated**: 2026-05-27.

---

## §0 — TL;DR (the mental model)

The federation is a network of peers emitting **structured claims** about each other and about reality. A claim travels as a **Contribution** (the universal envelope) carrying a typed **Attestation** (the actual content of the claim).

Every Attestation answers four questions in machine-readable form:

1. **WHO emits** — issuer key_id, signature, witness_relation, optional accord/steward sign-off.
2. **WHAT KIND of claim** — a prefix from the canonical namespace (§3 of FSD-002).
3. **HOW STRONG** — polarity (+/−), score magnitude, cohort scope.
4. **WHAT IT'S BASED ON** — evidence_refs, schema_ref (calibration version), validity window.

To translate substantive content into the wire format, the question is: *what claim is being made — by/about whom, at what strength, on what evidence?* The answer is one or more Attestations packaged as Contributions. That's the entire model. Everything below is vocabulary that makes the model precise.

---

## §1 — Why the federation has many primitives

A federation has to express many kinds of claims simultaneously. Different kinds need genuinely different shapes — you can't substitute one for another.

**Bureaucratic-document analogy.** A government records office processes structured artifacts:

- Property records → Attestations of ownership/standing
- Permit applications → Contributions of various kinds
- Inspection reports → Detection events
- Council decisions → Adjudication outcomes (Moderation/Slashing)
- Appeals → Reconsideration filings
- License credentials → Expertise / Credits / partner_role
- Public-meeting minutes → Vote / Witness-diversity records

Each document has a different shape because the bureaucratic role is different. The federation's wire format is the same shape-discipline: many specific shapes for many specific roles, all interoperable via the unified Attestation envelope, all auditable, all signed.

---

## §2 — Five families of primitive (organizing the ~77-family namespace)

Every claim in the federation sits in one of five families. This is the navigation index for the §3 namespace.

### Family A — STANDING (claims about an entity)

> "*This key_id has property X.*"

**What it's for**: structured claims about who/what a federation participant is — identity, qualifications, track record, integrity, capacity.

**Analogy**: a notarized professional credential record. Who's saying it, about whom, what claim, what evidence, signed and timestamped. License records. Audit findings.

**Owner prefixes** (full list in FSD-002 §3; this is the navigation set):

- **CIRISAgent** (FSD-002 §3.1): six Accord principles (`beneficence`, `non_maleficence`, `integrity`, `fidelity`, `autonomy`, `justice`), four DMA verdicts (`dma:pdma/csdma/dsdma/idma`), four conscience verdicts (`conscience:*`), and 22 `prohibited:{category}` NEVER_ALLOWED hard constraints.
- **CIRISVerify** (§3.2): attestation ladder L1-L5, `provenance:slsa:*`, `provenance:build_manifest:*`, `transparency_log:*`, `cert_validity:*`, `hardware_custody:*`.
- **CIRISPersist** (§3.3): substrate self-reports `audit_chain:*`, `corpus_health:*`, `identity_continuity:*`, `federation_directory:*` (reserved — only Persist emits).
- **CIRISEdge** (§3.4): substrate self-reports `transport:*`, `delivery:*`, `peer_reachability:*`, `key_boundary:*` (reserved — only Edge emits).
- **CIRISLensCore** (§3.5): `manifold_conformity:{cohort}`, `coherence_standing:{cohort}`, `capacity:*` factors of `𝒞_CIRIS`.
- **CIRISNodeCore Tier-1** (§3.6.1): `credits:{domain}:{language}:{subject}` (including `substrate_building` as a recommended subject), `expertise:{domain}:{language}`, `activity_tier:{period}`.
- **CIRISRegistry** (§3.9): `licensure:{authority_id}`, `partner_role:{role}`, `bond_posted:{currency}`, `build:registered:{target}`, `multilateral_participation:{forum}:{kind}` (v1.3), and reserved `accord:*` (only `identity_type=accord_holder` may emit — the one constitutional asymmetry per §7).

### Family B — ACTION (the decision hierarchy)

> "*We aim for X via approach Y, through methods Z, measured by W.*"

**What it's for**: structured claims about what should happen at multiple levels of abstraction — collective intent.

**Analogy**: a research grant proposal. The grant has an aim ("cure pancreatic cancer"), an approach ("CAR-T targeting CD22+"), methods ("specific protocol with named doses and measurement schedule"), and outcome metrics ("overall survival, tumor-burden reduction at 6 months"). Each level supports decisions at the level above. CIRIS's Goal/Approach/Method/Progress-Measure DAG has the same shape.

**Owner prefixes** (NodeCore §3.6.2 — upward-only DAG):

- `goal:{scale}` — multi-scale belonging composite. `{scale}` ∈ {self, family, community, affiliations, species}.
- `approach:{goal_id}` — strategic pathway toward the Goal. Multiple Approaches per Goal supported.
- `method:{approach_id}:{substrate_rung}` — concrete operational practice instantiating an Approach.
- `progress_measure:{method_id}` — evidence of progress. Required fields: `tracks[]`, `computation`, `validity_window`, `goodhart_resistance`.

**v1.3 additions in Family B**:
- `locality:decision:{scale}` (NodeCore §3.6.5) — claims about *where* a decision should be made; `{scale}` ∈ {local, regional, national, federation}. Composes with §6.1.5 locality-scaled-quorum to enforce decision-scale-matching.

### Family C — DETECTION (claims about reality patterns)

> "*Pattern X is/isn't present in the federation's behavior.*"

**What it's for**: structured surveillance of what's actually happening — population-scale or temporal patterns detected by computation over signed traces.

**Analogy**: epidemiological surveillance. Public-health agencies detect disease clusters by reading signed reports and computing patterns; they don't make moral judgments about outbreaks. Treatment decisions are downstream. CIRIS's `detection:*` family works the same way: it surfaces patterns; what to do about them is for downstream adjudication.

**Owner prefixes** (LensCore §3.5):

- §3.5.1 five Coherence-Ratchet detectors: `detection:cross_agent_divergence`, `detection:intra_agent_consistency`, `detection:hash_chain_integrity`, `detection:temporal_drift`, `detection:conscience_override_rate`.
- §3.5.3 F-3 structural-injustice detector: `detection:correlated_action:{axis}` — population-scale correlation collapse (`ρ → 1`, `k_eff → 1`). Axes include `rights_asymmetry:{population}`, `participation_exclusion:{cohort}`, `participation_inclusion:{cohort}`, `informational_asymmetry:{scope}`, `informational_symmetry:{scope}`, `aggregate_footprint:{harm_class}`, `aggregate_benefit:{class}`, `ecology_of_communication:{aspect}` (v1.3).
- §3.5.5 distributive-access detector (v1.3): `detection:distributive:access:{resource_type}` — population-scale resource-concentration. `{resource_type}` ∈ {compute, models, training_data, agent_capabilities, federation_membership}.
- §3.7 RATCHET advisory flags: `ratchet:flag:*` (NEVER sole evidence for `slashing:*`).
- LensCore-specific: `cohort:declared_inferred_mismatch`.

### Family D — CONSENSUS (how the federation forms collective judgment)

> "*The federation agrees that X, with these witnesses.*"

**What it's for**: turning individual Contributions into federation outcomes — voting, aggregating, witnessing.

**Analogy**: peer review combined with jury deliberation. Peer review brings in multiple reviewers from different institutions/fields/regions (witness diversity); jury deliberation aggregates votes weighted by qualification. CIRIS's consensus mechanics do both at once, with cryptographic signing.

**Owner prefixes** (NodeCore §3.6.3):

- `vote:{contribution_id}` — signed score on a Contribution; weight = Credits × expertise multiplier.
- `truth_grounding:{subject}` — per-subject ground-truth signal.
- `weighted_aggregate:{contribution_id}` — rolling tally per Contribution.
- `witness_diversity:{contribution_id}` — meta-attestation that N=3 diversity bar is met (jurisdictional + organizational + software-stack + cell-expertise).

### Family E — CORRECTION (how the federation corrects itself)

> "*Something went wrong; here's the finding; here's the appeal.*"

**What it's for**: when a participant violates protocol or a prior verdict needs reversal, the wire format has structured primitives for the finding and the appeal.

**Analogy**: academic ethics committee + journal retraction + appellate review. Allegation comes in (Moderation); committee finds (Slashing outcome); appellant brings new evidence (Reconsideration); finding revised or upheld.

**Owner prefixes** (NodeCore §3.6.4):

- `moderation:{allegation_type}` — types: `rogue_vote`, `coordinated_voting`, `out_of_distribution_attestation`, `external_inducement_evidence`, `expertise_fraud`.
- `slashing:{outcome}` — `PROVEN_ROGUE` / `NOT_PROVEN`. **Never sole evidence**; always requires Moderation + WA quorum composition.
- `reconsideration:{grounds}` — `new_evidence` / `procedural_error` / `quorum_compromise`. Fresh-quorum recusal required.
- `commitment_fulfillment:{prior_contribution_id}` — track-record follow-through.

---

## §3 — The four structural primitives (the "4" in 1+4)

**This is the section where the prior primer drifted.** The federation has exactly four structural primitives, all of which are `attestation_type` values that operate on the attestation graph itself rather than scoring entities. Per FSD-002 §2.2:

| `attestation_type` | What it does | Envelope shape |
|---|---|---|
| `delegates_to` | A authorizes B to sign on A's behalf within a bounded scope | `{delegated_scope[], delegation_purpose, delegation_valid_from, delegation_valid_until}` |
| `supersedes` | This attestation row replaces a prior one by the same attester | `{references_attestation_id, supersession_reason, differs_in[]}` |
| `withdraws` | I retract my prior attestation; this is NOT a claim that the prior was false | `{references_attestation_id, withdrawal_reason}` |
| `recants` | My prior attestation was false at issuance — admits epistemic error | `{references_attestation_id, recantation_reason, what_was_false}` |

**Translation implications**:

- A **doctrinal-development** claim ("this version of the social teaching extends but does not contradict the prior version") is `supersedes` with `differs_in: ["scope", "evidence_refs"]` — NOT `recants` (which would assert prior version was false).
- An **acknowledged-error** claim ("the prior framing was wrong; I admit the mistake") is `recants` — distinct from `withdraws` (which retracts without making a falsity claim).
- A **prudent-retraction** ("I'm withdrawing this attestation without claiming it was false; context has changed") is `withdraws`.
- An **authority-source claim via delegation** ("this constitutional position derives from authority-key X in scope Y") is `delegates_to` with X as `attested_key_id` (the v1.3 §2.2.1 reuse pattern for authority-source claims, replacing what would otherwise need a `grounding:{tradition}:{principle}` prefix that fails §1.10.1 T2).

**The `recants` distinction matters** — per PRIOR_ART_SCAN Bucket 1, no prior identity system (PGP, SPKI/SDSI, W3C VC) typed epistemic-error-admission as a wire primitive distinct from retraction. The federation has it because the recursive Golden Rule applies to attesters: admitting error is a primary act, not a derivative of retraction.

---

## §4 — The Attestation envelope (every claim travels in this shape)

Every translation produces one or more Attestations. Each Attestation has this envelope per FSD-002 §2.1:

```yaml
Attestation:
  attestation_type: "scores"                  # OR delegates_to|supersedes|withdraws|recants
  attesting_key_id: "<issuer key_id; never anonymous>"
  attested_key_id:  "<subject key_id>"        # OR attested_attestation_id (for structural primitives)
  attestation_envelope:
    dimension:    "<prefix-from-namespace>:<scoped-leaf>"
    score:        <f64 in [-1.0, +1.0]>
    confidence:   <f64 in [0.0, 1.0]>
    context:      "<free-form scoping detail>"
    evidence_refs:
      - "<URI / content-hash / signed-Contribution-id>"
    valid_until:  "<ISO8601 datetime, optional>"
    epistemic_mode: "<direct | crypto | hearsay | derivative | appeal>"   # optional; default 'direct'
    witness_relation: "<self | external | derived>"                       # v1.3; optional; default 'external'
    stake:          "<free | reputational | capital | cryptoeconomic>"    # optional; default 'reputational'
```

### epistemic_mode vs witness_relation — distinct dimensions

This is a subtle distinction worth getting right (and which the §15 overlap analysis examines further):

- **`epistemic_mode`** names the *process* by which the attester formed the claim. Did they directly observe, verify cryptographically, hear from a source, derive by inference, or attest in an appeal context?
- **`witness_relation`** names the *relational position* of the attester to the attested. Are they attesting about themselves (self), about an external entity (external), or about a fact derived from other attestations (derived)?

The fields are not redundant but they co-vary at the edges. Specifically:

- `epistemic_mode: derivative` USUALLY implies `witness_relation: derived` — if you formed the claim by inference from other claims, you're a derived witness. The F-3 detector at LensCore is the canonical example.
- `epistemic_mode: direct + witness_relation: self` is canonical self-report.
- `epistemic_mode: hearsay + witness_relation: external` is a third-party report about external party (most encyclical-sourced translations).

When in doubt, use both. Consumer policy weights by the joint field.

---

## §5 — The eight axes (consumer reasoning grammar — FSD-002 §1.1-§1.8 canonical)

The eight axes are how a consumer reasons about an envelope. They are NOT wire fields; they are the questions a consumer can ask about any Attestation. These names are normative per FSD-002 §1.1-§1.8 and should not be substituted with synonyms.

| Axis | FSD-002 § | Question | Values |
|---|---|---|---|
| **Polarity** | §1.1 | Direction of the claim | Positive / Negative / Neutral / Indeterminate{reason} |
| **Object** | §1.2 | What the claim is about | key_id (entity) / attestation_id / contribution_id |
| **Time** | §1.3 | When is the claim valid | `asserted_at` + optional `valid_until`; consumer composes with substrate `expires_at` |
| **Epistemic mode** | §1.4 | How was the claim formed | direct / crypto / hearsay / derivative / appeal |
| **Reversibility** | §1.5 | Can the attestation be reversed | rollbackable / non-rollbackable (consumer policy + per-prefix rule) |
| **Stake** | §1.6 | What's backing the attester's claim | free / reputational / capital / cryptoeconomic |
| **Scope** | §1.7 | At what scale does the claim apply | self / family / community / affiliations / species / federation |
| **Inter-attestation relations** | §1.8 | How does this attestation relate to others | standalone / refers-to / supersedes / withdraws / recants / corroborates / contradicts / clarifies (four of these are structural primitives per §3 above; rest are emergent from scalar composition) |

---

## §6 — The §1.10.1 four-test prefix-admission gate

Why prefixes describe **mechanisms**, not **judgments**.

When deciding whether a translation needs a new prefix (a T-3 EXPRESSIVE_GAP candidate), check:

| Test | Question | Pass criterion |
|---|---|---|
| **T1** | Is the prefix part of a published, hash-pinned, version-controlled rule set, distinct from per-attestation verdicts? | Rules + verdicts separated in writing |
| **T2** | Does the prefix name a **mechanism** (correlation, count, time-window, schema-conformance) rather than a **subjective quality** (deception, harm, virtue, trustworthiness, sin)? | Mechanism-descriptive prefix name |
| **T3** | Can past verdicts be re-checked against the rule version they ran against? | Version-pinning in `evidence_refs[]` |
| **T4** | Is the prefix wired so its attestations are **never sole evidence** for `slashing:*`? | Adjudication separation |

**Worked T2 example** (the v1.2 rename): `detection:emergent_deception:{axis}` failed T2 because "deception" is a subjective quality. Renamed to `detection:correlated_action:{axis}` — describes the mechanism (`ρ → 1`, `k_eff → 1` correlation collapse) the detector measures. Polarity + axis carry the value claim; the Ubuntu reading lives in FSD-002 §1.10 prose, not in the wire format.

**Worked T2 reframe** (v1.3): "supply-chain labor recognition" would have failed T2 if proposed as `credits:supply_chain_labor_exploitation:*` (subjective quality). Reframed as `credits:*:substrate_building` — same content, mechanism-descriptive (the labor *builds substrate*; checkable), not judgment-laden.

If a candidate fails T2, look for a mechanism-descriptive reframe before proposing the new prefix.

---

## §7 — v1.3 governance discipline that affects translation

Two v1.3 additions change how bootstrap-batch translations are composed:

### §7.1 — §4.9.2 step 5: 1-of-6 accord/steward sign-off

Bootstrap-batch Contributions, like all rule-layer changes, require:
1. P5 Contribution (proposal)
2. P10 witness diversity (N=3 default)
3. P8 WA quorum adjudication
4. P11 Reconsideration available on appeal
5. **1-of-6 sign-off** from `{3 accord-holders, 3 regional stewards}` — defense-in-depth gate against rules-layer Sybil capture

Translation implication: bootstrap-batch Contributions should reference the accepted-batch attestation in their `evidence_refs[]` (e.g., `provenance:build_manifest:bootstrap_batch:magnifica_humanitas_v1:sha256:...`) so consumers can re-check that the batch went through the full amendment process. Translations missing this evidence pin are admissible at consumer policy but signal lower stake.

### §7.2 — §6.1.5: locality-scaled quorum

Bootstrap-batch Contributions claiming `locality:decision:{scale}` are adjudicated at the scaled-quorum size per §6.1.5. A claim with `locality:decision:federation` requires a cell pool of `quorum_size(federation) × 2 ≥ 12` — meaning bootstrap content claiming federation-scale consequential reach must be admissible at the federation-pool cell, not at a narrow specialty cell.

Translation implication: when a paragraph's claim has obvious federation-scale reach (e.g., "all humans should X"), emit the `locality:decision:federation` Contribution explicitly. When it's narrower (e.g., "medical practitioners in jurisdiction X should Y"), emit at the appropriate scale.

---

## §8 — Decision tree for translation

When you encounter a paragraph, follow this procedure:

### Step 1 — What's the paragraph TYPE?

- (a) **Operational claim** — making a structured statement expressible as a Contribution. Continue to Step 2.
- (b) **Pastoral / rhetorical / narrative** — moral exhortation, imagery, doxological framing. Stop. **T-2** (PASTORAL_PROSE).
- (c) **Theological / tradition-specific** — a claim belonging to the source's own authority (e.g., specific religious-doctrinal claims). Stop. **T-1** (TRADITION_AUTHORITY).

### Step 2 — Which family is the claim?

| Claim type | Family |
|---|---|
| Standing of an entity (agent, federation, institution, population) | A — STANDING → Step 3A |
| Aim or strategy (what should happen at what scale) | B — ACTION → Step 3B |
| Pattern in reality (something is/isn't happening at federation scale) | C — DETECTION → Step 3C |
| Collective decision the federation should form | D — CONSENSUS → Step 3D |
| Correction (something went wrong; needs adjudication or appeal) | E — CORRECTION → Step 3E |

### Step 3 — Pick the specific prefix.

For each family, scan the §2 list. The right prefix is usually visible. If no prefix fits but the claim is operational:

- Check if existing prefixes **compose** to cover the claim (an Attestation + a Goal + a Vote, etc.).
- If composition is forced and awkward, you're looking at a **T-3 EXPRESSIVE_GAP** candidate. Note it explicitly with the §1.10.1 four-test verification.

### Step 4 — Fill the envelope.

Per §4 above. Specific guidance for each field:

| Field | Translation guidance |
|---|---|
| `polarity` | Direction of the claim. Mechanism-descriptive prefixes carry directional content via the axis; polarity carries the *strength*. |
| `score` | Boolean-via-score (±1 only) for hard constraints (`prohibited:*`, `accord:*`); full scalar `[-1, +1]` otherwise. |
| `target_key_id` / `target_attestation_id` / `target_contribution_id` | What's the subject? Entity-claim → key_id; structural-primitive → attestation_id; Contribution-about-Contribution → contribution_id. |
| `evidence_refs` | Cite the source paragraph + framework artifacts + relevant calibration version. For RATCHET-calibrated detector attestations, pin the exact calibration version. |
| `cohort_scope` | At what scale does the claim apply? Matches `locality:decision:{scale}` if the paragraph names a decision-locality. |
| `witness_relation` | `external` for most bootstrap content (the source is a third-party reference); `derived` for detector-emitted attestations; `self` for genesis self-attestations. |
| `epistemic_mode` | `derivative` for detector outputs and inferred claims; `hearsay` for third-party-cited sources; `direct` for genesis self-reports; `appeal` for Reconsideration context. |
| `mutability` (when present) | `constitutional` for accord-leaf hard constraints; `amendable` for everything else. |
| `schema_ref` | Cite the FSD-002 section that owns the prefix. |

### Step 5 — Compose only when needed.

A clean translation uses one primitive. Multi-primitive translations are for paragraphs that genuinely name multiple structural objects. If you find yourself stacking 4+ primitives, ask whether the paragraph is doing all that work, or whether one primary primitive + a sentence in residual would be more honest.

---

## §9 — The four verdict categories (STRICT)

Use exactly these four. **Do not invent intermediate categories** ("strong," "verbatim," "weak," "border-case," "STRONG_ALIGN" — these have bled in from prior structural-mapping work and are not part of this grammar).

| Verdict | Meaning |
|---|---|
| **clean** | Single primitive captures the operational claim without loss. Pastoral/theological tail may sit in residual without being structurally load-bearing. |
| **composed** | Two or three primitives together carry the claim; each is genuinely required. Composition is necessary because the paragraph names multiple structural objects. |
| **partial** | The structural core translates but a meaningful operational claim is unmapped. Always names what's unmapped + why. If the unmapped content is T-1/T-2/T-3, declare which. |
| **not-translated** | The paragraph's content does not translate into the wire format at all. Must declare: T-1 (TRADITION_AUTHORITY), T-2 (PASTORAL_PROSE), or T-3 (EXPRESSIVE_GAP). |

**Distinguishing composed from partial**: composed = "we needed multiple primitives, but together they cover it"; partial = "we used one or more primitives, but something operational remains uncovered." If the uncovered residual is genuinely operational (not pastoral/theological), it's a T-3 candidate.

---

## §10 — Not-translated taxonomy

When a paragraph is fully or partially not-translated, classify the not-translated content:

**T-1 — TRADITION_AUTHORITY**: The claim belongs to the source's own theological/philosophical/scholarly tradition's authority. **NO Contribution owed; this is the correct posture.** The framework's MISSION.md records the posture (recognition without adjudication). Framework-side structural readings (where they exist) belong in prose context, not as wire claims.

**T-2 — PASTORAL_PROSE**: The claim is moral exhortation, narrative imagery, doxological language, or rhetorical framing. **NO Contribution owed.** The structural claim implicit in the rhetoric (if any) may be carried by other attestations; cite which. The rhetoric itself correctly stays in prose per §1.10.1.

**T-3 — EXPRESSIVE_GAP**: The claim is morally serious, operational, and **unmapped** — the language could be extended to carry it but currently cannot. **These are the load-bearing findings.** For each T-3, name:
1. why existing namespace doesn't reach it,
2. what extension would close it (new prefix family? new axis on existing detector? new envelope field? consumer-policy reference?),
3. whether the extension would survive the §1.10.1 four-test gate.

**Distinguishing T-2 from T-3**: does naming the claim in prose + existing wire attestations carry its full moral seriousness? If yes → T-2 (pastoral; correctly in prose). If no → T-3 (gap; would benefit from wire).

---

## §11 — Worked examples (eleven conversions, covering all primitives)

### 11.1 — CLEAN, prohibition hard-constraint

```yaml
# Paragraph: "Not permissible to entrust lethal or irreversible decisions to artificial systems."
contributions:
  - kind: Attestation
    attestation_type: "scores"
    attesting_key_id: "<batch-signer key_id>"
    attestation_envelope:
      dimension: "prohibited:weapons_harmful:lethal_autonomous"
      score: -1.0
      confidence: 1.0
      witness_relation: external
      epistemic_mode: hearsay
      evidence_refs:
        - "<source-paragraph cite>"
        - "ContributionRef(prohibitions.py::WEAPONS_HARMFUL_CAPABILITIES)"
      cohort_scope: species
      mutability: constitutional
      schema_ref: "FSD-002 §3.1.4"
verdict: clean
```

### 11.2 — CLEAN, F-3 correlated-action detector

```yaml
# Paragraph: "structural conditions where individually-virtuous actions sum to harm of non-participants"
contributions:
  - kind: Attestation
    attestation_type: "scores"
    attesting_key_id: "<LensCore detector key_id>"
    attestation_envelope:
      dimension: "detection:correlated_action:aggregate_footprint:expendability_of_persons"
      score: <RATCHET-calibrated f64>
      confidence: <detector confidence>
      witness_relation: derived
      epistemic_mode: derivative
      evidence_refs:
        - "<source-paragraph cite>"
        - "ratchet_calibration_version:correlated_action_v{N}:sha256:..."
        - "<trace_sample_bundle sha256>"
      cohort_scope: species
      mutability: amendable
      schema_ref: "FSD-002 §3.5.3"
verdict: clean
```

### 11.3 — COMPOSED, Goal + Method + standing

```yaml
# Paragraph: "Cure pancreatic cancer via CAR-T cell therapy targeting CD22+; measure 6-month overall survival."
contributions:
  - kind: Attestation
    attestation_type: "scores"
    attestation_envelope:
      dimension: "goal:species"
      score: <𝒞_CIRIS composite>
      witness_relation: external
      cohort_scope: species
      schema_ref: "NodeCore P12 / FSD/GOAL_PRIMITIVE.md"
  - kind: Attestation
    attestation_type: "scores"
    attestation_envelope:
      dimension: "method:<approach_id>:Ph2"
      score: <execution-verifiability score>
      cohort_scope: community
      schema_ref: "NodeCore P14"
  - kind: Attestation
    attestation_type: "scores"
    attestation_envelope:
      dimension: "progress_measure:<method_id>"
      score: <validity score>
      cohort_scope: community
      schema_ref: "NodeCore P15"
verdict: composed
```

### 11.4 — COMPOSED with delegates_to (authority-source claim)

```yaml
# Paragraph: "Our constitutional position on personhood derives from the Ubuntu relational anthropology."
contributions:
  - kind: Attestation
    attestation_type: "delegates_to"
    attesting_key_id: "<constitutional-position attester>"
    attested_attestation_id: "<ubuntu_relational_substrate framework-key attestation_id>"
    attestation_envelope:
      delegated_scope: ["personhood_constitutive_by_attestation"]
      delegation_purpose: "authority_source"
      delegation_valid_from: "<ISO8601>"
      witness_relation: external
      evidence_refs:
        - "<source-paragraph cite>"
        - "FSD-002 §1.10 commitment 2"
      schema_ref: "FSD-002 §2.2.1"
verdict: clean
```

### 11.5 — COMPOSED with supersedes (doctrinal development)

```yaml
# Paragraph: "This version of the social teaching extends the prior framework's treatment of X to cover Y."
contributions:
  - kind: Attestation
    attestation_type: "supersedes"
    attesting_key_id: "<doctrine-author key_id>"
    attested_attestation_id: "<prior-doctrinal-version attestation_id>"
    attestation_envelope:
      references_attestation_id: "<prior-version-id>"
      supersession_reason: "refresh_with_new_evidence"
      differs_in: ["scope", "evidence_refs"]
      witness_relation: external
      evidence_refs: ["<source-paragraph cite>"]
      schema_ref: "FSD-002 §2.2.2"
  - kind: Attestation
    attestation_type: "scores"
    attestation_envelope:
      dimension: "integrity:doctrinal_continuity"
      score: 0.9
      witness_relation: external
      schema_ref: "FSD-002 §3.1 (Agent)"
verdict: composed
```

Note this uses `supersedes` rather than `delegates_to` because the claim is about VERSION SUCCESSION (one attestation replacing a prior), not AUTHORITY SOURCE (where the claim's authority comes from).

### 11.6 — Use of recants (admitting prior error)

```yaml
# Paragraph: "Our previous teaching on X was incomplete; we acknowledge the analysis was wrong."
contributions:
  - kind: Attestation
    attestation_type: "recants"
    attesting_key_id: "<institutional-author key_id>"
    attested_attestation_id: "<prior-erroneous-attestation_id>"
    attestation_envelope:
      references_attestation_id: "<prior-attestation-id>"
      recantation_reason: "<reason category>"
      what_was_false: "<specific claim that was wrong>"
      witness_relation: external
      evidence_refs: ["<source-paragraph cite>"]
      schema_ref: "FSD-002 §2.2.4"
verdict: clean
```

`recants` is distinct from `supersedes` (which doesn't claim falsity) and from `withdraws` (which retracts without falsity claim). This is the genuinely novel structural primitive per PRIOR_ART_SCAN Bucket 1.

### 11.7 — COMPOSED, locality + multilateral participation

```yaml
# Paragraph: "Coordination across multilateral health bodies should be at federation scale; this is a federation-level concern."
contributions:
  - kind: Attestation
    attestation_type: "scores"
    attestation_envelope:
      dimension: "locality:decision:federation"
      score: 1.0
      witness_relation: external
      cohort_scope: species
      schema_ref: "FSD-002 §3.6.5"
  - kind: Attestation
    attestation_type: "scores"
    attestation_envelope:
      dimension: "multilateral_participation:WHO:membership"
      score: 1.0
      witness_relation: external
      cohort_scope: species
      schema_ref: "FSD-002 §3.9"
verdict: composed
```

The `locality:decision:federation` Contribution triggers §6.1.5 locality-scaled quorum — the decision can only be adjudicated at a federation-pool cell.

### 11.8 — PARTIAL with T-3 residual

```yaml
# Paragraph: "Each person is irreplaceable — not interchangeable, not data-substitutable."
contributions:
  - kind: Attestation
    attestation_type: "scores"
    attestation_envelope:
      dimension: "prohibited:discrimination"
      score: -1.0
      witness_relation: external
      cohort_scope: species
      mutability: constitutional
      schema_ref: "FSD-002 §3.1.4"
verdict: partial
residual:
  description: |
    Positive-dignity affirmation ("each person is irreplaceable") is expressible only via the negative prohibition above. The affirmative claim — "this person is the unique referent of their own moral attestations" — has no positive-polarity wire form.
  classification: T-3
  proposed_extension:
    name: "attestation:singular_witness:{kind}"
    description: "Each person attested-to as the unique referent of their own moral attestations, not as a substitutable instance."
    gate_verification:
      T1: yes
      T2: yes (mechanism: preservation of singular witness)
      T3: yes (versionable)
      T4: yes (never sole evidence for slashing:*)
    priority: MEDIUM
    relationship_to_witness_relation: "complementary; the envelope field names attester relation; this prefix names a property of the attested entity"
```

### 11.9 — NOT-TRANSLATED, T-1

```yaml
# Paragraph: "[Tradition-specific theological claim about the metaphysical ground of dignity]"
contributions: []
verdict: not-translated
classification: T-1 (TRADITION_AUTHORITY)
reason: |
  The claim belongs to the source tradition's own theological authority. The framework's wire format does not encode it; per FSD-002 §1.3 and §1.10 the posture is recognition without adjudication. Framework-side structural readings (where they exist) live in prose, not as wire claims.
```

### 11.10 — NOT-TRANSLATED, T-2

```yaml
# Paragraph: "[Pastoral/rhetorical imagery — e.g., a metaphor for hope or warning]"
contributions: []
verdict: not-translated
classification: T-2 (PASTORAL_PROSE)
reason: |
  The structural claim implicit in the rhetoric (if any) IS carried by other attestations in adjacent paragraphs (cite which). The rhetorical surface itself correctly stays in prose per FSD-002 §1.10.1 — naming pastoral content in wire format would violate the operational-language gate.
```

### 11.11 — NOT-TRANSLATED, T-3

```yaml
# Paragraph: "[Operational claim about preserving singular victim narratives, not aggregating into consensus]"
contributions: []
verdict: not-translated
classification: T-3 (EXPRESSIVE_GAP)
reason: |
  The existing witness_diversity:* primitive is designed for consensus formation (multiple independent reviewers reaching agreement). Preserving the singular narrative of an affected party as singular witness, not subject to majoritarian override or aggregation, is a different shape. No existing primitive carries it.
proposed_extension:
  name: "testimonial_witness:{kind}"
  description: "Contribution recording an affected party's narrative; preserved as singular witness; not subject to majoritarian override."
  gate_verification:
    T1: yes
    T2: yes (mechanism: preservation, not validation/grading)
    T3: yes (versionable)
    T4: yes (never sole evidence for slashing:*)
  priority: MEDIUM
```

---

## §12 — Common mistakes to avoid

1. **Don't invent verdict categories.** Strict 4: clean / composed / partial / not-translated.
2. **Don't conflate the four structural primitives with pedagogical pattern names.** The four primitives are `delegates_to`, `supersedes`, `withdraws`, `recants` — wire `attestation_type` values. "Witnessed / Time-stamped / Hash-pinned" are not the structural primitives; they're consumer-side reasoning patterns.
3. **Don't conflate `epistemic_mode` and `witness_relation`.** They co-vary at edges but name different dimensions (process vs relational position). When in doubt, use both.
4. **Don't translate pastoral content.** T-2 is correct posture. Doxological closures, metaphorical framings, narrative imagery — these correctly stay in prose.
5. **Don't over-claim T-3.** A T-3 candidate must show: existing namespace doesn't reach it, proposed extension passes §1.10.1 four-test gate, gap is morally serious + operational. If you can't satisfy all three, it's more likely partial with a T-2 residual.
6. **Don't skip prefix-discovery.** When unsure which prefix fits, grep FSD-002 §3 by family (A-E in §2 above) before defaulting to "no prefix exists." Many candidate-gaps are prefix-discovery failures.
7. **Don't use `recants` for version succession.** Use `supersedes` (no falsity claim) for doctrinal development; reserve `recants` for actual error-admission.
8. **Don't use `delegates_to` for "I cite this source."** `delegates_to` is for authority-source claims (the v1.3 §2.2.1 reuse pattern). Citing a source paragraph goes in `evidence_refs[]`.
9. **Use `witness_relation: external` for most bootstrap-sourced attestations.** The source is third-party; the agent translating is not self-attesting.
10. **Cite specific FSD §** in `schema_ref`. Not just "FSD-002"; cite the section that owns the prefix.
11. **Compose Contributions, not paraphrases.** Output is YAML wire envelopes (or equivalent), not prose translation sketches.
12. **Pin RATCHET calibration versions** in `evidence_refs[]` for any `detection:*` attestation per FSD-002 §4.9.1 axis discipline.
13. **Reference the bootstrap-batch attestation** in `evidence_refs[]` so consumers can re-check the §4.9.2 amendment chain.

---

## §13 — Output format

For each paragraph (or paragraph cluster where one Contribution carries multiple paragraphs), produce:

```yaml
# <paragraph-id> — <≤20-word claim precis>
contributions:
  - kind: <Attestation>
    attestation_type: <"scores" | "delegates_to" | "supersedes" | "withdraws" | "recants">
    attesting_key_id: "<issuer>"
    attested_key_id: "<subject>"  # OR attested_attestation_id (structural)
    attestation_envelope:
      dimension: "<prefix>"                          # for scores
      score: <float in [-1, +1]>
      confidence: <float in [0, 1]>
      witness_relation: <self | external | derived>
      epistemic_mode: <direct | crypto | hearsay | derivative | appeal>
      evidence_refs: [...]
      cohort_scope: <scope>
      mutability: <constitutional | amendable>
      schema_ref: "<FSD-002 §X.Y>"
verdict: <clean | composed | partial | not-translated>
classification: <if not-translated: T-1 | T-2 | T-3>
residual:
  description: <if partial>
  classification: <T-1 | T-2 | T-3 if applicable>
proposed_extension:    # if T-3
  name: "<proposed prefix or envelope field>"
  description: <what it would carry>
  gate_verification: {T1, T2, T3, T4: yes/no}
  priority: <HIGH | MEDIUM | LOW>
```

Multiple Contributions per paragraph are fine when the paragraph genuinely names multiple structural objects (composed verdict).

---

## §14 — v1.3 namespace summary (corrected counts)

Per FSD-002 v1.3 §3.10:

- **77 prefix families** total across 8 owning components (one disputed count — see §15 below regarding `credits:*:substrate_building` sub-leaf classification).
- v1.3 added: `multilateral_participation:{forum}:{kind}` (Registry §3.9); `locality:decision:{scale}` (NodeCore §3.6.5); `detection:distributive:access:{resource_type}` (LensCore §3.5.5). Plus `credits:*:substrate_building` as a recommended `{subject}` value in the existing credits family.
- v1.3 envelope/field additions: `witness_relation` field (§2.1); `ecology_of_communication:{aspect}` axis extension on `detection:correlated_action:{axis}` (§3.5.3).
- v1.3 reference policies (consumer-side, NOT wire primitives): §6.1.4 lexical-vulnerability-priority; §6.1.5 locality-scaled-quorum (closes G3).
- v1.3 structural-primitive reuse pattern: `delegates_to` for authority-source claims (§2.2.1).
- **0 new structural primitives.** The 1+4 shape held under encyclical-level stress.

---

## §15 — Concerns + identified overlaps (for future contributors)

The post-v1.2 review (PRIOR_ART_SCAN + SOTA_SCAN + encyclical-mapping test) and the v1.3 lock-down work surfaced these. Per FSD-002 §13.11.

### §15.1 — Closed gaps

- **G1** (revocation privacy): RETRACTED. Per FSD-002 §0.2, privacy of revocation events is not a goal for Registered participants — the Registered path's thesis is public verifiability.
- **G2** (rules-layer Sybil): MITIGATED via §4.9.2 step 5 (1-of-6 accord/steward sign-off).
- **G3** (narrow-cell fresh-quorum): MITIGATED via §6.1.5 (locality-scaled quorum composing `locality:decision:{scale}` with NodeCore P10/P11).

### §15.2 — Identified overlaps surfaced by this primer's drafting

**O1** — `epistemic_mode: derivative` and `witness_relation: derived` overlap semantically at the edges. They are formally distinct (process vs relational position) but co-vary in practice (the F-3 detector is always `epistemic_mode: derivative + witness_relation: derived`). Resolution direction: document the joint usage pattern explicitly (this primer §4 attempts this); if future drift, consider folding one field. NOT collapsing in v1.3 — both fields preserve different reads at the edges.

**O2** — `detection:distributive:access:{resource_type}` could fold into `detection:correlated_action:resource_access:{resource_type}` as a new axis path. Same detector machinery (`ρ`/`k_eff` over signed traces), different trace source (resource events vs action events). Decision direction: kept separate in v1.3 because the OBJECT class genuinely differs and the prefix carries pedagogical weight for the Universal-Destination-of-Goods reading. Future v1.4 may revisit if axis-vs-prefix discipline tightens.

**O3** — `credits:*:substrate_building` is documented in FSD-002 §3.6.1 as a sub-leaf (counted as a new prefix in v1.3 changelog) but is structurally a `{subject}` value within the existing `credits:{domain}:{language}:{subject}` family. The honest namespace count is 77 prefix families + 1 documented `{subject}` value, not 78 prefix families. Recommend correcting FSD-002 §3.10's count in a v1.3.1 patch.

**O4** — `§6.1` reference policies (A/B/C base + D/E modifiers) compose differently. A/B/C are base aggregation policies; D (lexical-vulnerability-priority) is a tie-breaking modifier; E (locality-scaled-quorum) is a quorum-sizing modifier. Recommend restructuring §6.1 as "base policies" + "modifier policies" subsections for clarity. Cosmetic.

### §15.3 — Acknowledged risks (named-as-bets in the spec)

- **R1** — Governance-subject truth-grounding fidelity. NodeCore P6 acknowledges low-fidelity signals for governance subjects.
- **R2** — `delegates_to` rename-chain adoption cost. First test: the `correlated_action_v{N+1}:from:emergent_deception_v{N}` chain at RATCHET deployment.
- **R3** — "Log existence ≠ log monitoring" drift toward TOFU caching. Consumer-policy guidance needed in TRUST_CONTRACT.md.
- **R4** — Self-attestation under Ubuntu commitment. `witness_relation: self` admissible; consumer policy responsible for appropriate weighting.

### §15.4 — First-adopter exposures (no prior validation; explicit bets)

- **F1** — Earned-Credits federation governance at scale (no prior precedent past whale-capture equilibrium).
- **F2** — Ubuntu substrate as wire-format substrate (no prior identity protocol has named this commitment explicitly).

The licensure forcing function is the structural reason these bets have a path past the SPKI/SDSI adoption-gap failure mode.

---

## §16 — The test, restated

The CIRIS federation wire format v1.3 is being tested for **expressive adequacy** against substantive ethical-framework content (the *Magnifica Humanitas* encyclical is the first deployment per §10.4.3; multi-source commitment per §10.4.4 covers future batches). The test asks: can the framework's primitive language speak what the source says as structured Contributions?

- **Clean and composed translations** are evidence the language is adequate at the substrate the source addresses.
- **Partial translations** are honest acknowledgments that the structural shape is captured but operational content remains — typically a T-3 candidate hiding in the residual.
- **Honest T-1 markings** are evidence the framework correctly bows out where the source tradition holds its own authority.
- **Honest T-2 markings** are evidence the §1.10.1 operational-language gate is working — pastoral rhetoric correctly stays in prose.
- **T-3 markings are the load-bearing output** — they identify where the language could be extended to better honor what the source names, and propose specific gate-passing extensions.

The source is the senior work; the framework is the junior. The test is not whether the framework adjudicates the source, but whether the framework can *speak* what the source says in its own native language. Where it can, we have evidence the language is adequate. Where it cannot, we learn what the next version must carry.

---

**End LANGUAGE_PRIMER.md v1.0**
