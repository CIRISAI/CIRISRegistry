[← §0 Conformance](00_conformance.md) | **§1 Foundation** | [Next: §2 Grammar →](02_grammar.md)

---

# §1 Foundation

## §1.1 Mental model — federated structured-claim emission

The federation is a network of peers emitting structured claims about each other and about reality. A claim travels as a **Contribution** (the universal envelope) carrying a typed **Attestation** (the actual content of the claim).

Every Attestation answers four questions in machine-readable form:

1. **WHO emits** — issuer key_id, signature, witness_relation, optional accord/steward sign-off
2. **WHAT KIND of claim** — a prefix from the canonical namespace ([§5](05_namespace.md))
3. **HOW STRONG** — polarity (+/−), score magnitude, cohort scope
4. **WHAT IT'S BASED ON** — evidence_refs, schema_ref (calibration version), validity window

Consumers walk attestation graphs and compose verdicts. The substrate stores; the wire transports; CEG describes the shape of the claim. None of the three prescribes outcomes; consumer policy does.

## §1.2 The Ubuntu commitment — relational-anthropology substrate

Per `CIRISAgent/ContemplativeTraditions/Ubuntu.lean::F_ubuntu_primary_tradition_commitment` and [`../MISSION.md`](../../MISSION.md) §1.5:

> *Umuntu ngumuntu ngabantu* — a person is a person through other persons. Persons are not atomic; the relation IS the person.

Five load-bearing consequences for the wire format:

1. **The attested entity is not prior to its attestations.** A `federation_keys` row is not a representation of a pre-existing entity that the federation observes; it is the locus at which an entity is partly constituted by the cross-attestations that name it. Self-signature alone is not identity; cross-attestation is.

2. **Attesting is a participatory act, not an observation of fact.** A `scores` attestation does not merely report data about the attested entity. The attester's score participates in constituting the entity's standing in the relational field that consumers compose policy over.

3. **Detection brings patterns into morally-real existence.** A correlated-action pattern does not pre-exist its detection waiting to be observed. The detection-and-attestation is what crosses the pattern from "statistical regularity" to "morally-real object the federation now bears."

4. **Harm and deception collapse at the structural level.** Under Cartesian individualism, harm (setback to interests) and deception (causing false belief) are categorically distinct because persons are atomic and beliefs are private. Under Ubuntu, where personhood is partly constituted by accurate perception of the relational field, damage-to-perception IS damage-to-personhood IS harm. CEG's `detection:correlated_action:{axis}` family carries both via one prefix.

5. **The Recursive Golden Rule is structural, not exhortatory.** No principal — including the steward triple and CIRIS L3C itself — is exempt from constraints they impose on others. This is the wire-format symmetry of [§8.4](08_composition.md) below (Sovereign-Registered equivalence) plus the [§7](07_reserved.md) reserved-prefix patterns that bind even canonical bootstraps. Adding any privileged shortcut for a federation-internal principal would violate the Ubuntu substrate at primitive level.

**Why this is named here and not bracketed.** Engineering specs tend to bracket anthropology as "out of scope." But the wire format encodes anthropological commitments whether they are named or not. Bracketing them out means defaulting to whichever commitments contributors assumed by training — the Cartesian-individualist default is pervasive in cryptographic identity work (PGP web of trust, X.509 PKI, even most decentralized-identity schemes treat the key as representing a pre-existing atomic principal). CEG is not Cartesian. Naming the substrate explicitly is the discipline that prevents the open vocabulary, the reserved-prefix patterns, and the consumer-policy norms from drifting back toward the Cartesian default through unexamined intermediate choices.

**Cross-tradition reading.** The same structural object is approached from multiple traditions — Ubuntu (relational-primary), Logos (rational-order-of-reality), Tao / Dharma / Aristotelian virtue. CEG does not encode any one tradition's vocabulary; it encodes the *structural object* the traditions converge on. Future namespace extensions should be locatable in this substrate, not in a Cartesian fallback.

## §1.3 Operational-language gate — the safety-vs-censorship discipline

Per [`ciris.ai/safety-vs-censorship`](https://ciris.ai/safety-vs-censorship/):

> *"Rules are crowdsourced. Verdicts are machined."*
> *"The same machinery that catches real failures can become the machinery that enforces preferences."*
> *"None of this is automatic."*

Translated to CEG wire format: **prefix names must describe machine-checkable conditions, not subjective qualities**. The drift the page warns about — rules sliding "from 'uses the wrong word for therapy' toward 'feels disrespectful'" — has a wire-format analog: prefix names sliding from mechanism-descriptive (`detection:correlated_action:*`) toward judgment-descriptive (`detection:emergent_deception:*`). Both forms admit the same downstream verdicts; only one admits them honestly.

### §1.3.1 The four-test prefix-admission gate

Every prefix admitted to the [§5](05_namespace.md) namespace MUST pass:

| Test | Question | Pass criterion |
|---|---|---|
| **T1** | Is the prefix part of a published, hash-pinned, version-controlled rule set, distinct from per-attestation verdicts? | Rules + verdicts separated in writing |
| **T2** | Does the prefix name a **mechanism** (correlation, count, time-window, schema-conformance) rather than a **subjective quality** (deception, harm, virtue, trustworthiness, sin)? | Mechanism-descriptive prefix name |
| **T3** | Can past verdicts be re-checked against the rule version they ran against? | Version-pinning in `evidence_refs[]` |
| **T4** | Is the prefix wired so its attestations are **never sole evidence** for `slashing:*`? | Adjudication separation |

Existing prefixes failing T2 (the most slip-prone gate) get renamed; the canonical example is `detection:emergent_deception:*` (failed T2 in v1.1) → `detection:correlated_action:*` (passes T2 in v1.2). Anti-pattern catalogue at [§13](13_anti_patterns.md).

## §1.4 The 1+4 minimal-and-adequate claim

The federation has exactly **one workhorse attestation primitive + four structural composers**. Total wire surface: five. This claim has been examined by eight independent paths; future paths may extend or refute it via the [§11.2](11_governance.md) amendment process:

1. **PRIOR_ART_SCAN structural comparison** — no prior system (PGP / SPKI-SDSI / W3C VC / Birdwatch / Pol.is / Kleros / Spritely / Holochain / Aragon / Conviction Voting / Sigstore / SLSA) covers the same shape with fewer primitives.
2. **G3 narrow-cell fresh-quorum closure via [§8.1.5](08_composition.md) composition** — two independently-motivated primitives (`locality:decision:{scale}` + NodeCore P10/P11) composed to close a third gap without adding a structural primitive.
3. **v1.4 files-as-Contributions extension** — arbitrary content (binaries, configs, adapters, source files) carries through the federation via `scores` attestations + SHA-256 `evidence_refs[]` resolved through the substrate transport layer, with no addition to the structural-primitive set.
4. ***Magnifica Humanitas* encyclical mapping** — ~75-80% transparent translation rate; the 5% T-3 EXPRESSIVE_GAP set produced 10 dimension extensions and ZERO new structural primitives.
5. **283-story authorial stress test** (CIRISRegistry#30) — 9 generation agents + 5 review agents validated against the spec; 30 grammar-gap observations consolidated to 5 v1.5 spec additions, 6 explicit rejections (Cartesian shortcuts the wire resists), 4 primer fixes. No new structural primitives needed.
6. **CEG 0.4 time-bound state-bearing content** (per [CIRISRegistry#40](https://github.com/CIRISAI/CIRISRegistry/issues/40) + [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 1 closure) — `event_listing` lifecycle (open / cancelled / completed / superseded + RSVPs + reschedule + ticket transfer) composes from `withdraws` + `supersedes` + `delegates_to` + a new dimension family ([§5.6.8.5](05_namespace.md)). State machines do not require new structural primitives. NodeCore shipped the ingest path composition-only end-to-end at [d0a443a](https://github.com/CIRISAI/CIRISNodeCore/commit/d0a443a).
7. **CEG 0.6 dual-authority composition** (per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842)) — subject-side consent authority for content where the subject is not the producer (medical records / photos / interviews / training data / group chat / financial / surveillance / educational records / multi-party contracts) composes from existing primitives: a single new optional envelope field `subject_key_ids` ([§4.2](04_envelope.md)) + semantic broadening of `withdraws` admission ([§3.2.3](03_primitives.md)) to admit subject revocation + delegated proxy chain for canonical-hash subjects. Zero new structural primitives. The 1+4 set is rich enough to express the bilateral-pair PARTNERED ceremony, the 90-day decay protocol, the deletion-SLA watcher, AND the multi-subject revocation shape — all by composition over existing primitives. This is the seventh path and the most consequential: it demonstrates that the structural set is rich enough to express not just producer authority over content but the **full duality of producer + subject authority** that real-world content-shapes require.
8. **CEG 0.7 self/family membership + structural-invisibility composition** (per [CIRISRegistry#47](https://github.com/CIRISAI/CIRISRegistry/issues/47) + [CIRISPersist#152](https://github.com/CIRISAI/CIRISPersist/issues/152) + [ciris.ai/cewp](https://ciris.ai/cewp)) — `identity_occurrence` (self = trusted devices + agents) + `family` (a group of trusted nodes; one identity may belong to multiple; per-family `consensus_protocol` governs admission) + at-rest encryption flow + `cohort_scope: self | family` suppresses `holds_bytes:sha256:*` (the cewp structural-invisibility primitive) — all compose from existing primitives. Two new subject_kinds + one new envelope field (`family_id`) + four new substrate-emitted `hard_case:*` reserved prefixes + one new composition policy ([§8.1.12](08_composition.md) Policy L) + retcon of [§9 HUMANITY_ACCORD](09_humanity_accord.md) as the canonical entrenched-`family` instance. Zero new structural primitives. Membership-change ceremony rides existing `supersedes`; DEK cascade rides existing `key_grant.rotation_chain` from CEG 0.3; meta-amendment of consensus_protocol parallels existing [§11.2.3](11_governance.md) entrenchment shape. The 1+4 set is rich enough to express not only individual-scale consent (CEG 0.6) but **collective-scale membership and the wire-format-level closure of the privacy claim** ("the wire format can't carry self/family content beyond its scope in the first place"). Eighth path.

**Future extensions are dimension prefixes or envelope fields, not new structural primitives.** Proposals to expand the 1+4 set face a high evidentiary bar and route through the [§11.2](11_governance.md) amendment process. A successful refutation requires either: (a) demonstrating an operational claim that cannot be expressed via the existing 1+4 set plus envelope composition, OR (b) demonstrating a structural-primitive consolidation that reduces below 1+4 without loss.

## §1.5 The Recursive Golden Rule (structural, not exhortatory)

No principal — including CIRIS L3C as steward — is exempt from constraints the protocol imposes on others. Operational bites in CEG-shape:

- **Per-install stewards bind CIRIS L3C as steward.** Once `bootstrap_threshold ≥ 2`, no single Registry install can issue federation-scope attestations unilaterally.
- **Partner-revocation rules apply to CIRIS L3C subsidiaries.** `revocation:*` carries no steward exemption.
- **Audit discipline applies to steward operations.** Every admin RPC carries the operator's identity into `actor_user_id`, including for CIRIS L3C staff.
- **Bond forfeiture applies to CIRIS L3C-affiliated partners.** No exemption.
- **The HUMANITY_ACCORD asymmetry ([§9](09_humanity_accord.md)) is the ONE constitutional asymmetry.** Three named human holders carry kill-switch authority no federation-internal authority can grant / revoke / override / decay. This is not a Golden-Rule exemption; it is the recognition that consent requires revocability, and revocability requires a halt-authority outside the system being halted.

If a principal would be exempt from a constraint at any of these primitives, the Golden Rule is violated at that primitive and the protocol is the wrong shape there. Fix the primitive, not the rule.

---

[← §0 Conformance](00_conformance.md) | **§1 Foundation** | [Next: §2 Grammar →](02_grammar.md)
