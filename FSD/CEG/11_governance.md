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

---

[← §10 Endpoints](10_endpoints.md) | **§11 Governance** | [Next: §12 Translation →](12_translation.md)
