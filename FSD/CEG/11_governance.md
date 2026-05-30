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

---

[← §10 Endpoints](10_endpoints.md) | **§11 Governance** | [Next: §12 Translation →](12_translation.md)
