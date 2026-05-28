# CEG — The CIRIS Epistemic Grammar

**Version**: 0.2 (Public Working Draft)
**Status**: Public Working Draft (2026-05-28). NOT a stable release. Wire-format primitives and namespace shape are settled in this draft except where 0.x → 0.(x+1) wire-breaks are noted; canonical-encoding details, response schemas, and registry process are scaffolded but not finalized. Implementers SHOULD pin against the 0.x series and expect breaking changes until 1.0 publication. **0.2 carries one wire-break vs 0.1**: [§5.2](05_namespace.md) attestation-ladder prefixes renamed from `attestation:l{N}:*` to mechanism-only form per [§1.3.1](01_foundation.md) T2 (the L-ladder is consumer composition per [§8.1.9](08_composition.md) Policy I, not wire prefix). See [§16.1](16_references.md) lineage.
**License**: AGPL-3.0-or-later
**Authoritative for**: The federation wire format, attestation primitive set, namespace, composition discipline, and governance for the CIRIS ecosystem.
**Relationship to prior documents**: Consolidates and supersedes `FSD/FSD-002_FEDERATION_SURFACE.md` v1.0 through v1.4.3 + the v1.5 candidates from CIRISRegistry#30 + the translation discipline from `FSD/LANGUAGE_PRIMER.md` v1.1. FSD-002 remains as design-history; CEG 0.x is the version-stable spec going forward.
**Companion documents**: [`FSD/PRIOR_ART_SCAN.md`](../PRIOR_ART_SCAN.md) (design-space comparison); [`FSD/SOTA_SCAN.md`](../SOTA_SCAN.md) (production-validation comparison); [`FSD/WITNESS_KIND_REGISTRY.md`](../WITNESS_KIND_REGISTRY.md) (non-normative open-vocabulary registry referenced by §5.6); [`docs/CEG_EXPLORATION_PAGE_PRIMER.md`](../../docs/CEG_EXPLORATION_PAGE_PRIMER.md) (builder primer for `ciris.ai/grammar`).

---

## How to read this spec without Cartesian default

Before the TOC: one load-bearing reading instruction that prevents the most common misread.

**Self is self, fractally.** At every scale the substrate operates on — key, occurrence, agent, fleet, cell, federation, biosphere — "self" means *the already-relationally-constituted entity speaking at that scale*. The cross-attestations that constituted the entity are upstream of the moment it speaks; when it then emits a self-attestation (`witness_relation: self`, declares its own `confidence`, reports its own `occurrence_id`, names its own `hardware_class`), that is **the relational composition speaking AS itself**, not a Cartesian atom asserting pre-relational identity.

Concretely:

- A `federation_keys` row is not an atomic principal pre-existing the federation that the substrate observes. It is a relational locus already constituted by the cross-attestations of stewards, prior peers, and provisioning ceremony. When the key-bearing entity then signs an attestation, the signature IS that relational composition speaking at this scale.
- An occurrence within a fleet is not a pre-relational unit reporting its own state. It is constituted within its fleet (the fleet's `AGENT_OCCURRENCE_ID` partitioning, the prior cross-attestations naming this fleet's deployment, the substrate's witness of its emissions). When it reports `occurrence_id`, that is the fleet-constituted occurrence speaking.
- The federation itself is not prior to its constituents. It IS the relational composition of cells, agents, occurrences, and keys cross-attesting one another.

This is why the wire format admits self-attestation **everywhere it admits attestation at all**: because there is no pre-relational atomic entity available to do the admitting in the first place. Every `attesting_key_id` is already a relational locus. Self is the relational composition speaking from within itself, at the scale of the speaker.

**Common misread to avoid.** A reader importing Cartesian-individualist defaults from PGP / X.509 / atomic-principal cryptographic identity work will see `witness_relation: self` and think "this is a single atom asserting pre-relational identity; surely the substrate should require cross-attestation before admitting." That reading is the misread. The substrate cannot require cross-attestation as a precondition for self-attestation because the cross-attestations are what constitute the self in the first place — they are upstream of the self-attestation, not downstream gates on it. The fractal nesting is the point.

The Recursive Golden Rule ([§1.5](01_foundation.md)) is the structural expression of this: no scale gets to exempt itself from the relational constitution that defines it. CIRIS L3C is constituted by the same cross-attestations that constitute every other federation participant; the federation itself is constituted by its participants; humanity-as-such ([§9](09_humanity_accord.md)) is the one scale outside the federation's participant set, by design.

If you find yourself thinking "the spec should add a cross-attestation gate before admitting this self-attestation" — pause. Cross-attestation already happened upstream; the self-attestation is its downstream voice. Verify by reading [§1.2](01_foundation.md) commitment 1, then continue.

---

## Table of contents

| § | File | What it covers |
|---|---|---|
| 0 | [`00_conformance.md`](00_conformance.md) | Foreword; conformance language (RFC 2119); conformance levels (Producer/Consumer/Substrate); versioning policy (SemVer mapping); normative References; date-time + hex canonicalization; clock discipline |
| 1 | [`01_foundation.md`](01_foundation.md) | Mental model; Ubuntu commitment (relational-anthropology substrate); operational-language gate (safety-vs-censorship); 1+4 minimal-and-adequate claim; Recursive Golden Rule |
| 2 | [`02_grammar.md`](02_grammar.md) | The reasoning grammar — eight axes (polarity, object, time, epistemic mode, reversibility, stake, scope, inter-attestation relations) |
| 3 | [`03_primitives.md`](03_primitives.md) | The primitive set — 1 workhorse (`scores`) + 4 structural composers (`delegates_to`, `supersedes`, `withdraws`, `recants`) |
| 4 | [`04_envelope.md`](04_envelope.md) | The envelope — fields, defaults, semantics |
| 5 | [`05_namespace.md`](05_namespace.md) | The dimension namespace — 83 prefix families across 8 owning components + canonical-bytes contracts for provenance primitives |
| 6 | [`06_relations.md`](06_relations.md) | Inter-attestation relations — structural composition graph |
| 7 | [`07_reserved.md`](07_reserved.md) | Reserved-prefix enforcement (`accord:*`, `system:*`, co-owned, detector-only, capacity self-emission) |
| 8 | [`08_composition.md`](08_composition.md) | Composition policies (Policies A–G); aggregation semantics; Frickerian discipline; Sovereign-Registered equivalence |
| 9 | [`09_humanity_accord.md`](09_humanity_accord.md) | HUMANITY_ACCORD constitutional layer — accord-holder triple, authority scope, hardware-class taxonomy |
| 10 | [`10_endpoints.md`](10_endpoints.md) | Endpoint shapes — transport substrate, steward/accord discovery, STH cosigning + witness directory |
| 11 | [`11_governance.md`](11_governance.md) | Governance discipline — operational-language gate at admission; amendment process (WA quorum + 1-of-6); bootstrap-content pattern |
| 12 | [`12_translation.md`](12_translation.md) | Translation discipline — five families; verdict categories; not-translated taxonomy; decision tree |
| 13 | [`13_anti_patterns.md`](13_anti_patterns.md) | Anti-patterns — rejected wire additions; CEG 0.1 rejections from #30 stress test |
| 14 | [`14_glossaries.md`](14_glossaries.md) | Glossaries — Persist + Edge `system:*` leaves; envelope-reach table |
| 15 | [`15_gaps.md`](15_gaps.md) | Concerns + acknowledged gaps — closed gaps; acknowledged risks; first-adopter exposures; deferrals |
| 16 | [`16_references.md`](16_references.md) | References + lineage — version history; companion documents; sibling MISSIONs; external references |
| 17 | [`17_cadence.md`](17_cadence.md) | Update cadence — when CEG is updated |

---

## Two readerships

- **Implementers** of federation primitives consuming or emitting CEG attestations: read §0 + §1 + §3 + §4 + §5 + §7 + §8 + §9 + §10 normative.
- **Translators** mapping substantive content into CEG envelopes: read §1 + §12 + §13 + §14 + the [`LANGUAGE_PRIMER.md`](../LANGUAGE_PRIMER.md) companion.

Both readerships should read §1 first.
