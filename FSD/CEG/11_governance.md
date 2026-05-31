[← §10 Endpoints](10_endpoints.md) | **§11 Governance** | [Next: §12 Translation →](12_translation.md)

---

# §11 Governance discipline

## §11.1 Operational-language gate at admission

Every new prefix admitted to the [§5](05_namespace.md) namespace passes the [§1.3.1](01_foundation.md) four-test gate. Failed admissions are revised (mechanism-descriptive reframe) or rejected.

## §11.2 Amendment process — federation Contribution + WA quorum + 1-of-6 sign-off

Rule-layer changes (new prefixes, new envelope fields, new policies, calibration package version transitions) route through:

1. **Proposed amendment** filed as a NodeCore P5 Contribution (kind: `PROPOSAL`, subject: the proposal artifact).
2. **Witness diversity** per NodeCore P10 (N=3 default).
3. **WA quorum adjudication** per NodeCore P8.
4. **Reconsideration** per NodeCore P11 with fresh-quorum recusal (per [§8.1.5](08_composition.md) locality-scaled-quorum, including the §8.1.5.1 sub-quorum fallback).
5. **1-of-6 accord-holder OR steward sign-off** as defense-in-depth gate against rules-layer Sybil capture. The 1-of-6 sign-off is the secondary check; WA quorum is the primary substantive review. Any single signer can VETO by refusing to sign. Reduces the attack surface from "produce N Sybils" to "compromise one of six specific hardware-attested keys."

### §11.2.1 Axis-vocabulary discipline

Every `{axis}` value emittable under open-vocabulary prefixes (e.g., `detection:correlated_action:{axis}`, `hard_case:{kind}`, `testimonial_witness:{kind}`) MUST carry an operational definition where the prefix has a calibration package (RATCHET-calibrated detectors) or a documented convention where it doesn't.

For RATCHET-calibrated detectors, the operational definition lives in the calibration package version pinned via `evidence_refs[]`:

1. Measurement procedure
2. Threshold function
3. Statistical floor
4. Evidence-shape requirement
5. Polarity semantics

For documentation-only open vocabularies (`testimonial_witness:{kind}`, `hard_case:{kind}`, `topical_relation:{kind}`), discoverability lives in non-normative registry documents like [`WITNESS_KIND_REGISTRY.md`](../WITNESS_KIND_REGISTRY.md) — additions there require no spec amendment.

### §11.2.2 Open-vocabulary collision rule

When two parties independently register confusingly-similar `{kind}` / `{axis}` values within the same prefix family, the following resolution applies:

1. **First-registered wins** for the canonical-attestation surface. The earlier `signed_at` (per [§0.5](00_conformance.md)) holds the name; later registrations carry a `differs_in: ["semantic_disambiguation"]` clarification or pick a distinct value.
2. **Levenshtein-distance guard**: a CEG-Conforming Substrate (CCS) SHOULD compute Levenshtein distance against existing values in the same prefix family at admission; values within distance ≤ 2 of an existing canonical value SHOULD return a `409 IDEMPOTENT_CONFLICT` with an advisory hint, NOT a hard reject — the producer may proceed if the similarity is intentional (e.g., `commonsense` vs `commonsense_hard` are intentionally close).
3. **No squatting**: a `{kind}` registered but never used (no scored attestations in 90 days) MAY be reclaimed by another producer via the [§11.2](#112-amendment-process--federation-contribution--wa-quorum--1-of-6-sign-off) amendment process.

### §11.2.3 Meta-amendment + entrenchment

The §11.2 amendment process itself, the [§1.3.1](01_foundation.md) T1–T4 prefix-admission gate, and the [§9](09_humanity_accord.md) HUMANITY_ACCORD constitutional layer are **entrenched** — changes to these three surfaces require a MAJOR version bump per [§0.3](00_conformance.md) AND an additional 2-of-3 HUMANITY_ACCORD signatures (NOT the 2-of-3 from §11.2 step 5 — a separate, dedicated accord ratification). Without this entrenchment, a single quorum could rewrite the gate admitting the next quorum.

## §11.3 Bootstrap-content pattern

After federation genesis, a curated batch of P5 Contributions is admitted via the §11.2 amendment flow, populating the federation's substantive content surface with high-quality ethical-framework material. **Content-neutral**: any sufficiently substantive ethical-framework source can serve. The wire format admits content via the [§5](05_namespace.md) namespace; the [§1.3.1](01_foundation.md) gate ensures prefix names don't import source-tradition vocabulary.

**First deployment**: the *Magnifica Humanitas* encyclical mapping at ~75-80% transparent translation rate (Cargo `ciris-response-magnifica-humanitas` repo).

**Multi-source commitment**: subsequent bootstrap batches from CARE Principles (Indigenous data governance), Buddhist economic-justice scholarship, secular humanist instruments, African philosophy of personhood work — all through the same amendment process. The framework is multi-traditional by design.

## §11.4 Fast-path takedown coordination (CEG 0.3 addition; per CIRISRegistry#37 + #38)

For `takedown_notice` Contributions ([§5.6.8.4](05_namespace.md)) whose `legal_basis` falls in the **immediate-removal** category (`TvecTerrorist` / `NcmecCsam` / `GifctCip` / `PerceptualHashCsam` / `CourtOrder`), the §11.2 amendment process timeline is too slow — TVEC mandates a 1-hour removal obligation; GIFCT CIP coordinates within hours; NCMEC + perceptual-hash + court orders demand near-immediate response.

CEG 0.3 carves out a fast-path coordination protocol:

1. **Notice admission**: the `takedown_notice` Contribution arrives at the substrate, signed by `claimant_key_id`. The substrate accepts it without §11.2 quorum; speed matters at this layer.
2. **Holder eviction**: substrate emits a `withdraws` against the matching `holds_bytes:sha256:{prefix}` directory entry per [§10.1.2](10_endpoints.md). Holders see their advertisement marked withdrawn and SHOULD cease serving the bytes.
3. **Per-basis dispatch**:
   - `TvecTerrorist` — operator coordinates via TVEC-designated channel (national regulator notification within 1 hour); substrate logs the notice + the eviction action to its audit chain.
   - `GifctCip` — operator coordinates via GIFCT Content Incident Protocol communication channel; same audit-chain logging.
   - `NcmecCsam` + `PerceptualHashCsam` — operator MUST file the NCMEC CyberTipline report (US 18 USC §2258A); substrate retains hash + minimal metadata for the federal-legal retention window only. No content retention.
   - `CourtOrder` — operator follows the court's stated timeline; substrate logs the order text + the eviction action.
4. **Audit trail**: every fast-path takedown enters a `hard_case:fast_path_takedown` Contribution ([§5.6.6](05_namespace.md)) for downstream review. Reviewers MAY file a `reconsideration:procedural_error` if the fast-path basis was misclassified.
5. **No counter-notice for immediate-removal cases**: by `legal_basis` design (TVEC / NCMEC / GIFCT / PerceptualHashCsam / CourtOrder all bypass counter-notice). The `expeditious-with-counter-notice` bases (`Dmca512` / `DsaArticle16` / `CommunityStandards` / `OsaIllegalContent`) route through the standard §11.2 amendment path on counter-notice via `reconsideration:new_evidence`.

**The takedown-isn't-a-coup property**: the §9 HUMANITY_ACCORD remains load-bearing. Fast-path takedowns happen via this protocol but a `takedown_notice` Contribution targeting the substrate itself (e.g., a state actor demanding takedown of `federation_keys` for whole categories of dissenting participants) would not propagate the same way — substrate-protective discipline + HUMANITY_ACCORD veto authority intersect at the substrate level. Operators in jurisdictions where this conflict materializes SHOULD escalate to the HUMANITY_ACCORD triple per §9.2 invocation procedures.

## §11.5 Hash-database operator policy (CEG 0.3 addition; per CIRISRegistry#39)

Perceptual-hash matchers (PhotoDNA / PDQ / Project Arachnid / GIFCT hash-sharing) are pluggable per the CIRISPersist `PerceptualHashMatcher` trait. Operators choose which matcher implementations to enable; CEG governs the access-policy contract.

### §11.5.1 Hash-database access landscape

| Matcher | Access posture |
|---|---|
| **PDQ** (Meta, 2019) | Open — algorithm + reference hashes publicly distributed |
| **PhotoDNA** (Microsoft, 2009) | Access-gated; restricted to vetted orgs (NCMEC + select platforms); substrate operators cannot download the hash database directly |
| **Project Arachnid** (C3P, 2017) | Access-gated; API access requires C3P partnership |
| **GIFCT hash-sharing** | TVEC-focused; access via GIFCT membership |

### §11.5.2 Operator path (CEG 0.3 default — option (a) per CIRISRegistry#39)

For a CIRIS substrate operator running a federation node, **the CEG 0.3 default operator path is**:

> **Self-hosted PDQ matcher against publicly-distributed reference feeds** (Microsoft Project Arachnid feed where publicly available, GIFCT-published lists where openly available). No access-governance overhead. Operator carries responsibility for index freshness.

This avoids the federation-dependency-at-substrate-protective-layer problem that option (b) (clearinghouse delegation) would introduce, and the controversy around option (c) (on-device hash-database access via OS-vended hooks, per the iOS NeuralHash 2021 incident).

### §11.5.3 Future hash-coalition path (deferred; awaits CIRIS hash-coalition emergence)

CIRISRegistry will file a follow-up issue when a CIRIS hash-coalition emerges that can serve as a clearinghouse for option (b) — substrate operators delegating perceptual-hash checks to a trusted coalition peer via federation. CEG 0.3 documents the slot; the actual coalition operator-onboarding flow is deferred.

### §11.5.4 What CEG 0.3 documents

- The closed-set of `legal_basis` values that compose with `PerceptualHashCsam` (the only `legal_basis` value that consumes hash-match output as immediate-removal trigger; see [§5.6.8.4](05_namespace.md))
- The operator-onboarding contract: an operator running a PDQ matcher MUST register their matcher's source feeds (which hash-list source URLs they're pulling from + freshness cadence) via a `system:perceptual_hash_matcher:registered` Contribution. Composes with [§5.3](05_namespace.md) Persist substrate-self-report discipline.

What CEG 0.3 does NOT do: prescribe which hash databases an operator MUST use. Operator choice. CEG documents the wire-format slot + the operator-onboarding contract + the recommended default; concrete matcher selection is operator policy.

## §11.6 Vertical compliance + subject-bearing dimension governance (CEG 0.6 addition; per CIRISRegistry#45)

Per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842). The wire-format primitives in [§4.2](04_envelope.md) `subject_key_ids` + [§5.6.8.6](05_namespace.md) `consent:*` family + [§5.6.8.7](05_namespace.md) `consent_record` + [§8.1.11](08_composition.md) Policy K compose into regulatory-vertical compliance mappings. CEG documents the canonical mappings as **informational**; the wire-format primitives are domain-agnostic and operator-configurable.

### §11.6.1 Vertical compliance mapping (informational)

| Regulatory framework | CEG primitive | How it composes |
|---|---|---|
| **GDPR Article 7** (consent) | `consent:state:granted` + `consent_record.subject_key_id` | Subject's wire-format declaration of consent; revocable via [§3.2.3](03_primitives.md) rule 2 |
| **GDPR Article 9** (special category — health, biometric, sexual orientation, etc.) | `subject_key_ids` MANDATORY for special-category content; producer's `consent:deletion_sla` SHOULD be ≤ 30 days | Substrate-level recognition that special category requires subject-side wire authority |
| **GDPR Article 17** (right to erasure) | `consent:state:revoked` → substrate-watched `consent:deletion_sla:{days}` → producer emits `consent:deletion_complete` OR substrate emits `hard_case:consent_sla_breach` | The §8.1.11.3 SLA watcher is the wire-format observability primitive for Article 17 compliance |
| **GDPR Article 20** (data portability) | DSAR export via `attestations.where(s ∈ subject_key_ids)` query | CIRISAgent's `DSARExportPackage` (per [`docs/CIRIS_CONSENT_SERVICE.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/CIRIS_CONSENT_SERVICE.md)) composes from this query trivially |
| **HIPAA 45 CFR 164.502** (uses + disclosures) | `consent:scope:{retain\|share\|analyze\|train\|publish}` + `cohort_scope` | scope qualifier names the permitted use; cohort_scope names the permitted visibility (orthogonal per [§4.2.4](04_envelope.md)) |
| **HIPAA 45 CFR 164.524** (patient right of access) | DSAR export per Article 20 above | Same composition |
| **FERPA 34 CFR Part 99** (educational records) | `subject_key_ids: [student_key]` + `delegates_to(parent_key → student_canonical_hash, scope: [consent_revocation])` for minors | Parental authority composes via the existing `delegates_to` primitive; no new shape needed |
| **CCPA §1798.105** (right to delete) | Same composition as GDPR Article 17 | Substrate-watched SLA + `consent:deletion_complete` |
| **EU AI Act Article 50** (training data transparency + opt-out) | `consent:scope:train` + `is_ai_generated` field at content publish + subject's `consent:state:revoked` against the training-datum Contribution | Subject can withdraw training-set consent; producer's deletion-SLA fires on the training-corpus Contribution |
| **CIRIS Accord M-1** (sustainable adaptive coherence — consent revocability) | The entire CEG 0.6 surface | The constitutional anchor — "consent (M-1's load-bearing property) requires revocability, and revocability requires a halt-authority that lives outside the system being halted" ([§9](09_humanity_accord.md) + MISSION.md §1.5). CEG 0.6 extends this recognition from accord-carriers (federation-as-a-whole halt) to all subject-authorities (per-Contribution halt) at scale. |

CEG does NOT prescribe which regulatory framework an operator MUST comply with; the wire primitives compose to ANY of them based on operator policy. Operators in regulated verticals (medical / legal / financial / educational) SHOULD pin compliance mappings as configuration above the wire primitives, not as new wire shapes.

### §11.6.2 Subject-bearing dimension governance (normative)

Per [§4.2.6](04_envelope.md). Dimensions whose namespace pattern names a subject MUST carry `subject_key_ids` containing that subject. This closes the default-leak failure mode where subject-bearing content publishes without wire-level subject authority (Gap 4 from the [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842) gap audit).

**Subject-bearing dimension patterns** (open catalog; operator vocabularies extend):

| Pattern | Example | Required `subject_key_ids` entry |
|---|---|---|
| `observed:user:{key_id}:*` | `observed:user:abc123:interaction_count` | `abc123` (or its canonical-hash form) |
| `epistemic:about:{key_id}:*` | `epistemic:about:abc123:trust_assessment` | `abc123` |
| `epistemic:memory:topic={topic}` (when topic names a person/entity) | `epistemic:memory:topic=patient_xyz_session` | `patient_xyz` canonical-hash |
| `consent:partnered:{user_key}` (CIRISAgent CEM agent-side stance) | `consent:partnered:abc123` | `abc123` |
| `agent_files:*:{subject_target}` (when target names a person) | `agent_files:medical_record:patient_xyz` | `patient_xyz` canonical-hash |
| `licensure:{authority_id}:{practitioner_key}` (when practitioner is named) | `licensure:CA_medical_board:dr_jones_key` | `dr_jones_key` |

**Substrate enforcement**: substrate admission gate MAY reject Contributions where the dimension matches a subject-bearing pattern but `subject_key_ids` is empty / does not contain the named subject. This is **operator-policy** (not normative across all substrates) — some operator configurations may admit and emit a `hard_case:subject_authority_missing` for review-queue handling instead of rejection.

**The takedown-isn't-a-coup parallel** ([§11.4 fast-path takedown](#114-fast-path-takedown-coordination-ceg-03-addition-per-cirisregistry37--38)) applies: substrate enforcement of subject-bearing dimension discipline cannot be used as a coup against the substrate itself (e.g., a state actor publishing `observed:user:dissenter_key:*` with `subject_key_ids = []` and demanding substrate admission). The §9 HUMANITY_ACCORD remains load-bearing; admission-gate rules apply uniformly.

### §11.6.3 What CEG 0.6 documents

- The wire-format primitives that compose into vertical compliance (informational mapping above)
- The dimension-pattern-implies-`subject_key_ids` requirement (normative gate)
- The bilateral-pair shape for ceremony grants per [§8.1.11.4](08_composition.md)
- The decay-protocol stage composition per [§8.1.11.5](08_composition.md)
- The SLA watcher boundary (substrate emits `hard_case:*`; LensCore composes `detection:*`) per [§8.1.11.3](08_composition.md)

What CEG 0.6 does NOT do:
- Bundle CIRISAgent's CEM streams as the only valid stream set (open vocab; CEG names `temporary` / `partnered` / `anonymous` as recommended canonical kinds, not lockdown)
- Define specific SLA values for any regulatory framework (operator policy — though informational guidance: GDPR Article 9 default ≤ 30 days; CCPA default 45 days; etc.)
- Provide a decay-protocol library (CIRISAgent's 90-day-decay is the canonical example; other protocols MAY exist)
- Prescribe per-vertical compliance audit cadence (consumer / regulator concern)

## §11.7 Self/family membership governance (CEG 0.7 addition; per CIRISRegistry#47)

Per [§5.6.8.8](05_namespace.md) `identity_occurrence` + [§5.6.8.9](05_namespace.md) `family` + [§8.1.12](08_composition.md) Policy L + [§10.1.4](10_endpoints.md) structural-invisibility. The four governance decisions for self/family membership.

### §11.7.1 Forward secrecy on member departure — Option A (CEG 0.7 default)

When a member leaves a family (or an occurrence is revoked from a self-collective), the removed party retains existing `key_grant`s for historical content; the substrate stops wrapping new `key_grant`s on subsequent content. No DEK rotation; no re-encryption.

**Rationale**: consistent with [§3.2](03_primitives.md) `withdraws-isn't-retroactive` + [§11.4](#114-fast-path-takedown-coordination-ceg-03-addition-per-cirisregistry37--38) "takedown isn't a coup" + CEG 0.6 [§8.1.11.5](08_composition.md) consent-decay-doesn't-re-encrypt. The substrate's forward-secrecy posture is uniform across consent, takedown, and membership-departure surfaces.

**Option B** (rotate-DEK on member departure; re-wrap all extant content to remaining members) is deferred. CEG 0.7 documents the slot for a future `subject_kind: family_rotation` ceremony that operators can opt into per family; the wire-format primitives are in place (`key_grant.rotation_chain` from CEG 0.3 covers the rotation mechanic) but the ceremony envelope is downstream-demand-driven.

### §11.7.2 Multi-family membership — envelope `family_id` (CEG 0.7 default)

One identity MAY belong to multiple families. Each family has its own DEK and its own membership roster; a `cohort_scope: family` Contribution MUST carry `family_id` ([§4](04_envelope.md) envelope field) naming which family's DEK and visibility apply. Substrate rejects family-scoped Contributions missing `family_id`.

**Rationale**: avoids cross-family DEK confusion when one identity is in N families; gives the substrate an unambiguous routing key for the `key_grant` cascade per [§8.1.12.4](08_composition.md).

### §11.7.3 Reserved-prefix substrate emissions — locked in §7.7

Per [§7.7](07_reserved.md). The four substrate-emitted membership-event prefixes (`hard_case:identity_occurrence_added:*`, `hard_case:family_membership_change:*`, `hard_case:family_consensus_protocol_change:*`, `hard_case:family_consensus_protocol_violation:*`) are reserved to `identity_type="substrate_persist"` emitters. Same discipline as the existing `system:*` substrate-self-report family.

### §11.7.4 Self-occurrence admission — single-vouch (CEG 0.7 default)

A new `identity_occurrence` is admitted on a signature from EITHER the root `identity_key_id` OR any currently-admitted occurrence of that identity (Signal-style "trust any device I've already onboarded"). Higher-assurance setups MAY layer requirements on `hardware_attestation` via consumer policy.

**Rationale**: matches user-intuition for self-membership ("my phone unlocks my laptop's trust posture for new devices"); avoids the operational overhead of multi-vouch for routine onboarding; the security gradient lives in the optional `hardware_attestation` field, not in the admission rule.

### §11.7.5 Family admission — consensus_protocol (CEG 0.7 normative)

Unlike self-occurrence, family membership changes are **NOT single-vouch by default**. The family's `consensus_protocol` field (locked at family creation per [§5.6.8.9](05_namespace.md)) governs admission. Six canonical protocols (`founder_only` / `unanimous` / `majority` / `quorum:M/N` / `weighted:{rubric}` / `custom:{family_id}`); operator vocabulary extends.

The consensus_protocol field is itself subject to amendment via the SAME protocol's rules (meta-amendment shape parallel to [§11.2.3](#1123-meta-amendment--entrenchment)) UNLESS the family is `consensus_protocol_entrenched: true`. Entrenched families reject amendments at the substrate gate; replacement requires the family's documented out-of-band ceremony (HUMANITY_ACCORD per [§9.2](09_humanity_accord.md) / FEDERATION_ANNOUNCEMENT.md §4.5.3 is the canonical entrenched example).

**Rationale**: families are multi-party collectives where membership changes have real consequences (admit-new-member = grant DEK access to all extant cohort_scope: family content). The consensus_protocol gives the family explicit governance over its own boundary. Self-amendment lets families evolve their governance as they grow; entrenchment lets safety-critical families lock the boundary against any internal authority.

### §11.7.6 What CEG 0.7 documents

- The wire-format primitives that compose into self/family membership ([§5.6.8.8](05_namespace.md) + [§5.6.8.9](05_namespace.md))
- The structural-invisibility discipline at [§10.1.4](10_endpoints.md) (cohort_scope: self/family suppresses holds_bytes:*)
- The at-rest encryption flow composition at [§8.1.12](08_composition.md) Policy L
- The consensus_protocol vocabulary (canonical kinds; open-vocab extension)
- HUMANITY_ACCORD as the canonical entrenched-`family` instance at [§9.1](09_humanity_accord.md)

What CEG 0.7 does NOT do:
- Lock the consensus_protocol vocabulary (open vocab; canonical kinds named for ecosystem coordination)
- Provide a key-rotation ceremony for Option B forward secrecy (deferred to downstream-demand-driven future release)
- Prescribe per-family entrenchment policy (operator/family choice)
- Document the CIRISPersist#152 at-rest encryption flow details (substrate-side; persist spec)

---

[← §10 Endpoints](10_endpoints.md) | **§11 Governance** | [Next: §12 Translation →](12_translation.md)
