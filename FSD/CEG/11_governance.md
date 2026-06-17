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

**Option B** (rotate-DEK on member departure; re-wrap all extant content to remaining members) is deferred. CEG 0.7 documents the slot for a future `subject_kind: family_rotation` ceremony that operators can opt into per family; the per-`(family_id, epoch)` rotation axis would parallel CEG 0.10's per-`(stream_id, epoch)` axis ([§10.5.3](10_endpoints.md)) — a distinct axis from CEG 0.3's `key_grant.rotation_chain` (which is content-addressed grant-supersession lineage per [§5.6.8.4](05_namespace.md), not key rotation). The ceremony envelope is downstream-demand-driven; the wire-format primitives needed are the `key_grant` wrap + Option-A re-grant on existing members (which already work today).

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

## §11.8 Geographic-community privacy invariant (CEG 0.8 addition; per CIRISRegistry#48)

Per [§5.6.8.10](05_namespace.md) `community` with `cohort_subkind: geographic` + [§5.6.8.11](05_namespace.md) `location_proof` + [§0.8](00_conformance.md) H3 cell canonicalization + [§8.1.13](08_composition.md) Policy M.

CEG 0.8 codifies a load-bearing privacy invariant: **joining a geographic community is a one-way disclosure**. Three sub-properties make this wire-format-level, not policy-level.

### §11.8.1 Rough-only is wire-format-enforced

Per [§0.8.1](00_conformance.md): `location_proof.cell_resolution ≤ 7`. H3 resolution 7 hexagons average ~5 km² edge-length — sufficient for city/borough disclosure without block/building precision. Producers attempting finer resolution have admission rejected at the substrate gate.

**This is the closure of an entire class of accidental over-share**: a malformed UI cannot publish precise location even if the client-side gating fails. The wire-format-level enforcement is the privacy primitive; UI is the second line.

Substrate emits `hard_case:location_proof_resolution_violation` ([§7.8](07_reserved.md)) on rejection so operators can observe malformed-client patterns. This is observability for operator debugging, NOT a slashing trigger — malformed producers are usually buggy clients, not attackers.

### §11.8.2 Leaving is forward-only — the audit chain preserves the historical claim

Per [§3.2](03_primitives.md) `withdraws-isn't-retroactive` + CEG 0.7 [§11.7.1](#1171-forward-secrecy-on-member-departure--option-a-ceg-07-default) Option A forward-secrecy. When a subject withdraws their location_proof (or leaves a geographic community):

- Forward visibility evicts (consumer policy treats the subject as "no current location proof" for new admission decisions from withdrawal-time forward)
- The withdrawn `location_proof` Contribution **remains in the audit chain** — federation peers retain the historical record
- "I was in Austin from May to August" is permanent; "I am currently in Austin" is what withdraws/expiry govern

This is the cost the subject opts into when emitting the `location_proof`. Per the [CEWP](https://ciris.ai/cewp) structural-not-policy framing, the wire format does not promise to expunge historical claims — that promise would be hollow (federation peers retain copies; the substrate can mark forward-only). What the wire format DOES promise:

- **Rough-only** (§11.8.1): the historical claim is bounded to resolution ≤ 7, never finer
- **Opt-in** (§11.8.3): the historical claim exists only because the subject signed and emitted it
- **Auditability**: the subject can prove what they did or did not claim, when (the audit chain is the receipt)

### §11.8.3 Joining is opt-in — substrate does NOT solicit location

A `location_proof` Contribution is emitted **only** by the subject themselves (`attesting_key_id == subject_key_id`) or by a `delegates_to` chain with `scope: [consent_revocation]` per CEG 0.6 — the substrate has no path to mint a location_proof on behalf of a key without an explicit signature.

Communities cannot **require** a location_proof from non-members (they can only **gate admission** on whether a member has produced one). The substrate has no mechanism for "involuntary location disclosure" via the wire format. A bad actor cannot force-publish another key's location_proof; without the subject's signature, the substrate rejects.

**Compose with [§9 HUMANITY_ACCORD](09_humanity_accord.md) substrate-protective discipline**: a state actor demanding the substrate emit location_proofs for non-consenting subjects is exactly the substrate-protective case that the HUMANITY_ACCORD halt authority exists to address. The substrate's role at this primitive is mechanical opt-in enforcement; political/legal disputes route through the §9 + §11.4 takedown coordination + the HUMANITY_ACCORD `EmergencyShutdown CONSTITUTIONAL` path if necessary.

### §11.8.4 What CEG 0.8 documents

- The rough-only enforcement primitive at [§0.8.1](00_conformance.md) (resolution ≤ 7 for location_proof)
- The forward-only leave semantics at [§3.2](03_primitives.md) (withdraws-isn't-retroactive applied to location_proof)
- The opt-in admission flow at [§5.6.8.11](05_namespace.md) (location_proof signed by subject only or delegates_to proxy chain)
- The geographic-community admission composition at [§8.1.13.2](08_composition.md) (Policy M evaluate_subkind_admission for geographic)
- The substrate-self-report observability at [§7.8](07_reserved.md) (4 hard_case prefixes)

What CEG 0.8 does NOT do:
- Mandate H3 over alternative geospatial systems for operator-internal use (operator choice; wire format uses H3 only)
- Provide a place-name registry (communities self-name; H3 cells are the substrate-level binding)
- Define specific cell-resolution conventions for community-side `geographic_constraint` (only `location_proof` is bounded to ≤ 7; communities MAY scope themselves at any resolution per operator/founder choice)
- Codify non-geographic community subkinds (`professional` / `interest` / `local-business` / `event-attendees` / etc. are downstream-demand-driven future spec rounds — same discipline that drove 0.3 → 0.4 → 0.5 → 0.6 → 0.7 → 0.8)
- Address `affiliations` (the fourth cohort_scope tier; deferred — CEG 0.9 took the [§7.0.1](#119-identity_type-as-a-set--single-key-role-cohabitation-ceg-09-addition-per-cirisregistry49--cirisagent856) identity_type-as-set cut instead; affiliations remains a later candidate round)

## §11.9 `identity_type` as a set — single-key role cohabitation (CEG 0.9 addition; per CIRISRegistry#49 + CIRISAgent#856)

Per [§7.0.1](07_reserved.md) + [CIRISRegistry#49](https://github.com/CIRISAI/CIRISRegistry/issues/49) + [CIRISAgent#856](https://github.com/CIRISAI/CIRISAgent/issues/856). CEG 0.9 generalizes `federation_keys.identity_type` from a single scalar role to a **set of roles**, so the [§7](07_reserved.md) reserved-prefix gates are evaluated by set membership (`X ∈ identity_type`). This amendment routed through the [§11.2](#112-amendment-process--federation-contribution--wa-quorum--1-of-6-sign-off) process; CIRISAgent#856 is the driver, CIRISRegistry#49 the CEG-authority mirror.

### §11.9.1 What the amendment changes — and what it deliberately does not

| Surface | Before 0.9 | At 0.9 |
|---|---|---|
| `federation_keys.identity_type` representation | scalar string | set of role strings (legacy scalar = singleton set) |
| §7 emitter-rule evaluation | `identity_type == "X"` | `X ∈ identity_type` ([§7.0.1](07_reserved.md)) |
| §5 dimension namespace | — | unchanged |
| Envelope ([§4](04_envelope.md)) | — | unchanged |
| 1+4 structural primitives ([§3](03_primitives.md)) | — | unchanged |
| subject_kinds ([§5.6.8](05_namespace.md)) | — | unchanged |

This is a **wire-break at the `federation_keys` row representation only** — the second 0.x wire-break after the [§0.2](16_references.md) attestation-ladder rename. It is **semantically null for every legacy single-role key**: `X ∈ {X}` ≡ `X == X`. It is NOT a [§1.4](01_foundation.md) "Nth path" confirmation (it adds no namespace surface); it is a [§7](07_reserved.md)-layer enforcement generalization that unblocks the CIRISAgent fold-in (one key, many roles) without expanding the wire's expressive surface.

### §11.9.2 Settled in CIRISAgent#856, carried as-is

- **Capacity self-emission ([§7.5](07_reserved.md))**: unchanged. The anti-Goodhart `attesting_key_id ≠ attested_key_id` rule binds regardless of how many roles a key holds. A folded `{agent, lenscore_detector}` key still MUST NOT score its own `capacity:*`. Role cohabitation does not create a self-attestation backdoor.
- **Reasoning-trace dimensions**: no separate reserved `identity_type` required; reasoning-trace emission rides the agent role's open-vocabulary surface. Cohabitation does not change this.
- **Agent-intent / LensCore-envelope split**: a cohabiting key emits agent-intent attestations under the agent dimensions and detector verdicts under `detection:*` ([§7.4](07_reserved.md) worked example). The namespace keeps the two surfaces distinct; cohabitation grants the right to emit on each, never merges them.

### §11.9.3 Cohabitation discipline for constitutional + substrate roles

Set membership grants the wire-level *right* to emit per held role, but two roles carry defense-in-depth guidance against cohabitation:

- **`accord_holder` ([§7.1](07_reserved.md) + [§9](09_humanity_accord.md))**: the one constitutional asymmetry. Consumer policy SHOULD treat an `accord_holder` key that also holds non-constitutional roles (e.g., `{accord_holder, agent}`) with elevated scrutiny — the HUMANITY_ACCORD triple's halt authority is strongest when its keys are single-purpose. CEG 0.9 does NOT forbid the cohabitation at the wire layer (the substrate cannot adjudicate constitutional intent), but the §9 entrenched-`family` discipline RECOMMENDS dedicated accord-holder keys.
- **`substrate_persist` / `substrate_edge` ([§7.2](07_reserved.md))**: substrate-self-report roles remain cross-attested by the full steward-triple per §7.2. A key cohabiting a substrate role with an application role (e.g., `{substrate_persist, agent}`) MUST still satisfy the steward cross-attestation requirement for the substrate-role emissions; cohabitation does not relax the cross-attestation gate.

### §11.9.4 What CEG 0.9 documents

- The set-membership reading of every §7 reserved-prefix emitter rule ([§7.0.1](07_reserved.md))
- The canonical-bytes encoding for the set (sorted-ascending, deduplicated, comma-joined; single-role keys encode identically to their pre-0.9 scalar form)
- The LensCore-fold worked example ([§7.4](07_reserved.md))
- The cohabitation discipline for constitutional + substrate roles (§11.9.3)

What CEG 0.9 does NOT do:
- Expand the §5 dimension namespace, the §4 envelope, the §3 structural-primitive set, or the §5.6.8 subject_kinds (zero new wire surface beyond the `identity_type` representation)
- Enumerate a closed set of role values — `identity_type` members remain an open vocabulary owned per [§7](07_reserved.md) reservations + sibling-component vocabulary extensions (e.g., CIRISPersist#102 `witness`)
- Forbid any cohabitation at the wire layer (substrate enforces gates by membership; constitutional/substrate cohabitation discipline is consumer/operator policy per §11.9.3)
- Address `affiliations` (the fourth `cohort_scope` tier; remains deferred to a later candidate round)

## §11.10 Moderation as a delegable duty — `moderate` / `takedown` / `review` (1.0-RC19; per [CIRISRegistry#90](https://github.com/CIRISAI/CIRISRegistry/issues/90))

Moderation is a **delegable *duty*, not a platform- or fabric-assigned role** (design: CIRISServer `FSD/MODERATION_CHILD_SAFETY.md`). A participant exercises a moderation / takedown / review duty **as themselves**, or delegates it — to **their agent** (AI on-behalf-of) or to **any trusted party** (another human, a community moderator) — via `delegates_to`. This is the wire foundation under the [CIRISServer#15](https://github.com/CIRISAI/CIRISServer/issues/15) child-safety / takedown / accord UX, and the spine of *accountability ships ahead of capability*: **no media/chat feature ships until this — plus persist enforcement + fabric wiring — is solved and working.**

**Two layers — open labeling, and authoritative action.** Moderation in CEG spans both layers of the composable/stackable model (cf. Bluesky labelers + Ozone) but unifies them on the 1+4 grammar and adds the enforced action tier:

- **Open labeling — anyone, no authority.** Anyone MAY file a `scores` Contribution against anything they see (an *opinion/observation*, not an action). It is **visible to everyone who chooses** to read it, and consumers compose **filters** over the score graph — hide / blur / annotate / down-rank — as pure consumer policy ([§8](08_composition.md)). This generalizes independent labelers: stackable, swappable, subscribed by choice. A filter MAY **escalate to an auto-finding** — e.g. a [§11.12](#1112-watchlist-auto-detection--opt-in-per-group-separation-of-powers-10-rc23-per-cirisregistry94) CSAM watchlist match auto-fires a `takedown_notice` under an *enabling authority* — turning a passive filter into an action.
- **Authoritative action — the enforced duty.** Hiding-for-yourself needs no authority; **acting on a group's behalf** (a takedown, an authoritative ModerationEvent, an appeal ruling) requires the delegated `moderate`/`takedown`/`review` duty (below). [`moderation_track_record`](05_namespace.md) ([§5.6.4](05_namespace.md)) composites a moderator's action outcomes into the **relative, positional reputation** decentralized-moderation converges on — never a single global score.

One grammar covers a group chat, a classroom (teacher = delegated `moderate`), a town hall, an art gallery, a subreddit, a Discord, a Facebook-scale community: **the labeling open + filterable, the authority delegable + attenuable + revocable.**

CEG **names** the three duties as canonical `delegated_scope` kinds and **enforces** their admission — mirroring the only previously-enforced scope, `consent_revocation` ([§3.2.3 rule 3](03_primitives.md)). The kinds + their shipped action primitives are pinned at [§8.1.12.7.1](08_composition.md):

| scope | emits, on the delegator's behalf | shipped primitive |
|---|---|---|
| `moderate` | `moderation:{allegation_type}` ModerationEvent + report→`scores` + `age_assurance`/`content_class` gates | [§5.6.4](05_namespace.md) / [§8.1.10](08_composition.md) |
| `takedown` | `takedown_notice` (incl. the §11.4 immediate-removal fast-path) | [§5.6.8.4](05_namespace.md) / [§11.4](#114-fast-path-takedown-coordination-ceg-03-addition-per-cirisregistry37--38) |
| `review` | `reconsideration:{grounds}` appeal / review | [§5.6.4](05_namespace.md) |

**Enforced-admission rule — the principal is the chain root, NOT a payload field (normative; 1.0-RC24, closes the `on_behalf_of` bypass found in the persist v8.7.0 / [#232](https://github.com/CIRISAI/CIRISPersist/issues/232) principal-model review).** The principal an action is taken *on behalf of* is **discovered by walking the `delegates_to` graph upward from `attesting_key_id`** — it is **never** carried in a payload field. There is deliberately **no `on_behalf_of` (or equivalent) envelope field**: a side-field both violates the 1+4 lockdown *and* opens a bypass — if "absent field ⇒ as-self ⇒ admit," then any emitter that simply omits the field (e.g. an AI agent or untrusted party firing a takedown) is admitted as-self with no owner-bound chain proven, and the gate becomes a no-op exactly where it is load-bearing. A moderation action (`takedown_notice`, `moderation:*`, `reconsideration:*`) is admitted **iff** one holds **positively**:

- **(a) as-self** — `attesting_key_id` *itself* holds the matching duty over the target: it is the target content's own subject, **or** the target community's [§11.11](#1111-named-moderator-existence-invariant--merit-auto-promotion-10-rc21-per-cirisregistry93) named-moderator / `moderate`-holder. A zero-hop chain rooted at itself.
- **(b) delegated** — a live `delegates_to` chain `root →* attesting_key_id` where **every edge bears the matching scope** (`scope ⊇ {moderate|takedown|review}`), the **root holds the duty over the target** and is **owner-bound** ([§5.6.8.10](05_namespace.md) — an accountable human), depth ≤ 5 ([§13.3](13_anti_patterns.md)), and **no edge is `withdraws`-revoked**.

Otherwise **REJECT**. **Absence of a principal field is NOT an admit condition** — admission requires (a) or (b) to hold positively; a verifier MUST NOT read "no field present" as "as-self." This is the faithful mirror of `consent_revocation` ([§3.2.3 rule 3](03_primitives.md)), which derives its principal from the existing `subject_key_ids` relationship + the chain, never a side-field. Substrate SHOULD record which rule + which root admitted the action (the §3.2.3 per-rule audit metadata).

**Deputization + attenuation (normative; SOTA-aligned — UCAN / macaroons / SPKI-SDSI / ZCAP-LD).** A `delegates_to` MAY permit its delegate to **deputize** (further-delegate the duty) — but **only if the delegator granted it**, by including `sub_delegation` in the granted `delegated_scope` ([§8.1.12.7.1](08_composition.md)). Every sub-delegation **attenuates, never expands**: `child.scope ⊆ parent.scope`, and constraints may be *added* but never removed — the capability-attenuation rule shared by UCAN (each delegation "restates or attenuates"), macaroon caveats, and SPKI/SDSI proof-carrying authorization. The chain is depth-capped at 5 ([§13.3](13_anti_patterns.md)) and **revocable at any link**: a `withdraws` against *any* `delegates_to` in the chain invalidates everything downstream of it (UCAN-style proof-chain revocation). So a delegator decides at grant time **whether** their deputy may appoint further deputies and **under what constraints**, and can sever the entire subtree with a single revocation — deputize-a-teacher's-aide, hand-a-shift-to-another-mod, appoint-an-agent, all with bounded, revocable, attenuating authority.

**Target → duty-holder resolution — makes the rule substrate-enforceable (normative, 1.0-RC25 — resolves [CIRISRegistry#95](https://github.com/CIRISAI/CIRISRegistry/issues/95) part 3).** "Holds the duty over the target" is resolved by mapping the action's target to its duty-holder set, then checking the two predicates against it:

- `takedown_notice{content_sha256}` → the content's **authoritative** subject set `subject_of(content_sha256)` (self — see the resolution rule below; **not** the action payload's self-declared `subject_key_ids`) ∪ `is_named_moderator(·, C, takedown)` for the content's community `C` (its `cohort_scope: community` / `community_id`).
- `moderation:{allegation_type}` against a subject → `is_named_moderator(·, C, moderate)` for the relevant community.
- `reconsideration:{grounds}` against a prior action → `is_named_moderator(·, C, review)` for that action's community (recused per [§5.6.5](05_namespace.md) fresh-quorum).

**(a) as-self** holds iff `attesting_key_id ∈ duty-holders(target)` (a subject, or `is_named_moderator`); **(b) delegated** holds iff the chain `root ∈ duty-holders(target)` ∧ `is_owner_bound(root)`. With **`is_owner_bound`** ([§5.6.8.10](05_namespace.md)) and **`is_named_moderator`** ([§11.11](#1111-named-moderator-existence-invariant--merit-auto-promotion-10-rc21-per-cirisregistry93)) both resolvable from existing rows (community record + `identity_occurrence` + `delegates_to` + `community_id` + `subject_key_ids`), §11.10 is now **fully substrate-enforceable** — community moderation is no longer rejected for lack of a resolvable shape (closing the [CIRISPersist#233](https://github.com/CIRISAI/CIRISPersist/issues/233) gate). No new structural primitive.

**Subject authority is resolved from the content's signed provenance, NOT the action payload (normative, 1.0-RC27 — resolves [CIRISRegistry#96](https://github.com/CIRISAI/CIRISRegistry/issues/96)).** The subject side of admit-(a) has the same substrate-resolvability requirement the named-mod path got in RC25: *which* `subject_key_ids` is authoritative. A `takedown_notice` / `moderation:*` action carries a `content_sha256` **and** its own payload `subject_key_ids` — and a substrate that reads the subject set from the **action's own payload** lets an actor self-declare `subject_key_ids = [self]` over content it does not own, satisfy "as-self," and take down arbitrary content **without being a named-moderator** (a narrower, attributable re-opening of the takedown-isn't-a-coup hole on the subject path; the named-mod path is unaffected). The subject claim MUST instead be verified against the content's **establishing attestation**:

- **`subject_of(content_sha256)`** ≔ the signed `subject_key_ids` of the **establishing attestation** — the `scores` Contribution whose content the `content_sha256` binds (the [§4.2](04_envelope.md) subject set is signed *inside* that attestation by its producer, not assertable by a later third party). A substrate resolves `content_sha256` → establishing attestation → its signed `subject_key_ids`. This is the same content-hash → signed-attestation resolution every [§4](04_envelope.md) verifier already performs; no new index.
- **Admit-(a) subject-self** for content targets then means `attesting_key_id ∈ subject_of(content_sha256)` — the **signed** subject, never the takedown payload's self-declared set. The action payload's `subject_key_ids` is, on the subject-authority path, **advisory only** (it MAY be used to *route* / *queue*, but MUST NOT be the set admit-(a) is checked against).
- **Fail-secure when the establishing attestation is not locally held.** If a substrate cannot resolve `content_sha256` to its establishing attestation, `subject_of(content_sha256)` is **undetermined** and the subject-self clause **fails** (it does not admit) — the named-moderator path (b) is the only remaining route, exactly as for any other unprovable-authority case. Absence of provenance is never an admit condition (the same discipline as the `on_behalf_of` absence rule above).

With this, **both** clauses of admit-(a) — subject-self and named-moderator — and clause (b) resolve against **signed** state, never self-declared payload. The §11.10 gate has no remaining self-declaration spoof. Composes over the existing `scores` attestation + `subject_key_ids`; no new structural primitive.

**The "takedown-isn't-a-coup" property, made structural.** Because every action is delegate-signed, delegator-traceable up the `delegates_to` chain, owner-bound at the root ([§5.6.8.10](05_namespace.md) — authority roots in an accountable human), and revocable, a takedown is **coordinated + attributable + revocable** — never a unilateral seizure. A no-authority actor, or a state actor demanding removal of `federation_keys` for whole classes of dissenters, **fails the enforced-admission gate** and escalates to the [§9](09_humanity_accord.md) HUMANITY_ACCORD per [§11.4](#114-fast-path-takedown-coordination-ceg-03-addition-per-cirisregistry37--38). The §11.4 immediate-removal timeline is unchanged — speed at the action layer; authority checked at the delegation layer.

**1+4 preserved.** A `delegated_scope` vocabulary + enforced-admission addition over the existing `delegates_to`; the action primitives (`moderation:*`, `takedown_notice`, `reconsideration:*`) already ship. **No new structural primitive.** Chain to solved-and-working: this (CEG, RC19) → persist `admission.rs` enforcement → CIRISServer `src/safety/*` wiring.

## §11.11 Named-moderator existence invariant + merit auto-promotion (1.0-RC21; per [CIRISRegistry#93](https://github.com/CIRISAI/CIRISRegistry/issues/93))

**No unmoderated multi-party space, ever.** A `community` ([§5.6.8.10](05_namespace.md)) operates / federates **only while it has ≥1 active holder of its `moderate` duty** ([§11.10](#1110-moderation-as-a-delegable-duty--moderate--takedown--review-10-rc19-per-cirisregistry90)) — the moderation analogue of the [§5.6.8.10](05_namespace.md) owner-binding gate (#83). This closes the unmoderated-space-for-predators gap of relay-level (Nostr) / immutable-store (IPFS) / lax-instance (fediverse) models. Design: CIRISServer `FSD/MODERATION_CHILD_SAFETY.md` + `FSD/SAFETY_LANDSCAPE.md`.

Three normative rules:

1. **Existence gate.** A `community` is admitted, and continues to federate, **only while ≥1 member holds the live `moderate` duty** (held or delegated, owner-bound per [§5.6.8.10](05_namespace.md)). The creator **names one at creation** (founder responsibility). A community with no active `moderate`-holder is non-conformant.
2. **Merit auto-promotion (no moderator-less window).** When the named moderator lapses (`withdraws` against the `moderate` `delegates_to`, or inactivity past the community's freshness window), the member with the **highest [`moderation_track_record`](05_namespace.md)** ([§5.6.4](05_namespace.md)) is **automatically granted** the `moderate` duty — emergent, meritocratic authority (the moderation analogue of #83's owner-binding: authority emerges from an accountable, *merited* member, never a vacuum). **Deterministic selection:** highest `moderation_track_record`; tiebreak by earliest membership, then lexicographic `key_id` (so every peer auto-promotes the *same* member).
3. **Fail-secure.** If no eligible member can be named (none with sufficient track record, none consenting, none owner-bound), the community **fails-secure** — it MUST NOT federate / operate at moderated capability. **Better no group than an unmoderated one.** (Degrade, never escalate — the fail-secure default.)

**Named-moderator binding — the substrate-resolvable shape (normative, 1.0-RC25 — resolves [CIRISRegistry#95](https://github.com/CIRISAI/CIRISRegistry/issues/95) part 1).** "K is a named-moderator over community C" is an **appointment**: a `delegates_to(authority → K, scope ⊇ {moderate|takedown|review}, community_id: C)` whose root `authority` is in **C's authority set** — a founder, or a key the community's `consensus_protocol` authorizes, per the [§5.6.8.10](05_namespace.md) community record — and is owner-bound. It rides the **existing** `community_id` envelope field ([§4](04_envelope.md), CEG 0.8) + `delegates_to`; **no new shape.** A substrate resolves: **`is_named_moderator(K, C, duty)`** ≔ ∃ live `delegates_to` chain `root →* K` with every edge `scope ⊇ {duty}`, `community_id == C`, `root ∈ authority_set(C)` (the §5.6.8.10 founders / consensus signers), and `is_owner_bound(root)` ([§5.6.8.10](05_namespace.md)). **Merit auto-promotion (rule 2) emits exactly this appointment shape** — the community's authority auto-grants the `moderate` `delegates_to` to the highest-`moderation_track_record` member — so an auto-promoted moderator is resolvable **identically** to a hand-named one (one code path, no special case).

**Merit grants the duty, NOT fiat (anti-censorship).** The auto-promoted moderator holds the [§11.10](#1110-moderation-as-a-delegable-duty--moderate--takedown--review-10-rc19-per-cirisregistry90) `moderate`/`takedown`/`review` duty — but every **action** is constrained, so this is not arbitrary power: (a) the [§11.1](#111-operational-language-gate-at-admission) operational-language gate at admission (mechanically-checkable, publicly proposed/voted rules — per [ciris.ai/safety-vs-censorship](https://ciris.ai)), and (b) deterministic verdicts + the [§11.10](#1110-moderation-as-a-delegable-duty--moderate--takedown--review-10-rc19-per-cirisregistry90) Reconsideration appeals (recused reviewers). **Merit grants the seat; the gate + appeals constrain the action.** The duty is itself revocable and re-auto-promotes on lapse — so capture is bounded.

**1+4 preserved.** `moderation_track_record` rides `scores` ([§5.6.4](05_namespace.md)); the existence invariant + auto-promotion are admission/composition rules over the existing `delegates_to` (#90 `moderate` scope) + the reputation corpus. **No new structural primitive.** Chain: #90 (scopes, ✅ RC19) + this (RC21) → persist `admission.rs` enforcement (#232) → CIRISServer `src/safety/*` wiring (#15). Further safety asks from the CIRISServer `wt/safety` deep-dive complete CEG 1.0's moderation surface.

## §11.12 Watchlist auto-detection — opt-in, per-group, separation-of-powers (1.0-RC23; per [CIRISRegistry#94](https://github.com/CIRISAI/CIRISRegistry/issues/94))

Content-watchlist auto-detection (design: CIRISServer `FSD/WATCHLIST_DETECTION.md`): a [§11.10](#1110-moderation-as-a-delegable-duty--moderate--takedown--review-10-rc19-per-cirisregistry90) `moderate`-scope holder **optionally** enables a watchlist (`watchlist:{id}`, [§5.6.6](05_namespace.md)) for a group they moderate; the fabric auto-fires the matcher at the **publish/share seam** and auto-fires the action — CSAM → `takedown_notice{PerceptualHashCsam}` ([§11.4](#114-fast-path-takedown-coordination-ceg-03-addition-per-cirisregistry37--38)); other → `detection:*` + a `moderation:*` ModerationEvent to the named moderator. Rides shipped primitives — **no new structural primitive.**

**Opt-in, per-group, NEVER global (normative).** A watchlist is enabled per-group by its `moderate`/`takedown` authority; a global "scan everything" config is **non-conformant** — that is the bulk-surveillance posture the framework refuses. Enable/disable is **signed by the authority and revocable** (`withdraws`).

**Separation of powers (the responsible-design invariant).** No single party does all three:

| Party | Holds | Cannot |
|---|---|---|
| **Fabric** (the node) | the *mechanism* — the matcher at the publish/share seam | provision the hash-DB; choose to enable |
| **Operator** | the *licensed hash-DB* (IWF/NCMEC/PDQ, [§11.5.1](#1151-hash-database-access-landscape), operator-provisioned + unshippable) + the NCMEC report obligation | turn it on for a group; act without the authority's opt-in |
| **Authority** (`moderate`-holder) | the *opt-in* (per-group enable) | run the match itself; access the licensed list |

**Audit — never silent (normative).** Enabling a watchlist emits `hard_case:watchlist_enabled:{group}` ([§5.6.6](05_namespace.md)) — who turned it on + which list; **every match** emits `hard_case:watchlist_match:{group}`. Enablement and matches are on the record, always.

**CSAM-disable non-silent floor (normative).** Disabling a **CSAM** watchlist MUST be an audited act — a `withdraws` signed by the authority that **emits `hard_case:watchlist_enabled` (disable variant)**; **silent removal of a CSAM list is barred** (a predator-operator cannot turn off CSAM detection without leaving a trace). Ordinary (non-CSAM) lists may be freely toggled. This is the floor that keeps the opt-in honest.

**Honest scope (per [§15.7](15_gaps.md)).** Detection runs **only at the publish/share seam of enabled groups** — it **cannot** reach [§10.1.4](10_endpoints.md) self/family private content (the universal E2EE limit; not claimed solved), and CEG does **not** mandate client-side scanning. **1+4 preserved** — `watchlist:{id}` rides `scores`/config over `delegates_to`; the audit reasons ride the existing `hard_case:*` prefix; the actions ride `takedown_notice` / `detection:*` / `moderation:*`. Chain to the media/chat gate: this (CEG) → persist `admission.rs` (#232) → CIRISServer `src/safety/*` + the watchlist Phase 1.5 + the operator-provisioned PDQ adapter (#15).

---

[← §10 Endpoints](10_endpoints.md) | **§11 Governance** | [Next: §12 Translation →](12_translation.md)
