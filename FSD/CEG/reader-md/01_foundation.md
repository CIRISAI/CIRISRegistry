
---

# §1 Foundation

## §1.1 Mental model — federated structured-claim emission

The federation is a network of peers emitting structured claims about each other and about reality. A claim travels as a **Contribution** (the universal envelope) carrying a typed **Attestation** (the actual content of the claim).

**What CEG is, stripped of framing.** Independent of the CIRIS application, the AI vocabulary, or the [§1.2](#12-the-ubuntu-commitment--relational-anthropology-substrate-informative) anthropology, CEG is a **signed, compositional graph language for expressing claims, relationships, authority, membership, consent, governance, addressing, and settlement across a decentralized network** — a general-purpose *attestation calculus*. Structurally it is closer to a composition of Certificate Transparency + MLS + ActivityPub + DID/VC + reputation systems + governance protocols than to a conventional AI architecture. The AI/agent use cases are the *first consumer* of that calculus, not its definition. Read this way, the rest of the spec is: one workhorse claim primitive, four graph-composers, and a namespace.

Every Attestation answers four questions in machine-readable form:

1. **WHO emits** — issuer key_id, signature, witness_relation, optional accord/steward sign-off
2. **WHAT KIND of claim** — a prefix from the canonical namespace ([§5](05_namespace.md))
3. **HOW STRONG** — polarity (+/−), score magnitude, cohort scope
4. **WHAT IT'S BASED ON** — evidence_refs, schema_ref (calibration version), validity window

Consumers walk attestation graphs and compose verdicts. The substrate stores; the wire transports; CEG describes the shape of the claim. None of the three prescribes outcomes; consumer policy does.

## §1.2 The Ubuntu commitment — relational-anthropology substrate *(informative)*


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

The federation has exactly **one workhorse attestation primitive + four structural composers** at the **structural layer**. That is a genuine, narrow invariant — the *graph-operation* set is closed at five (`scores` + `delegates_to` / `supersedes` / `withdraws` / `recants`). It is **not** a claim that the whole grammar is five things.

**Scope of the claim (read this before citing "1+4").** What follows is an **inductive adequacy result, not a closure theorem.** The sixteen paths below show the structural set is *expressive across the surfaces tested*; they do **not** prove it generates *every* expressible structured claim. We have not defined the class of structured claims and proven 1+4 generates exactly it — until someone does, "1+4 is adequate" means "adequate across the sixteen surfaces examined," nothing stronger. The refutation bar ("exhibit a claim that cannot be composed") is, honestly, near-unfalsifiable while *composition itself* is unbounded — so absence of a counterexample is weak evidence, and these paths should be read as accumulating confidence, not as proof.

**"Minimal" is partly an accounting choice.** The structural set is five, but every path moved complexity *into* the namespace and envelope axes rather than removing it. The **full normative conformance surface** a second implementer must get exactly right is much larger than five: ~12 `subject_kind`s ([§5.6.8](05_namespace.md)) plus the open `external_content` sub_kind set, ~21 optional envelope fields ([§4](04_envelope.md)), 13 composition policies (A–M, [§8](08_composition.md)), 5 canonicalization families ([§0.5–0.9](00_conformance.md)), 6 `consensus_protocol` kinds, the [§7](07_reserved.md) reserved-prefix taxonomy, and dozens of dimension prefixes ([§5](05_namespace.md)). "1+4" is the elegant *structural* invariant; it is **not** the conformance surface, and citing it as "the grammar is five things" understates what interop requires. Always report the surface beside the invariant.


*Sixteen independent design exercises each composed without a new structural primitive; the enumeration lives in the canonical working draft.*


**Future extensions are dimension prefixes or envelope fields, not new structural primitives.** Proposals to expand the 1+4 set face a high evidentiary bar and route through the [§11.2](11_governance.md) amendment process. A successful refutation requires either: (a) demonstrating an operational claim that cannot be expressed via the existing 1+4 set plus envelope composition, OR (b) demonstrating a structural-primitive consolidation that reduces below 1+4 without loss.

**The standing falsification target (named, so the claim is a real bet).** The strongest current candidate for "a genuinely important domain not naturally expressible in 1+4" is **atomic fair exchange / bilateral simultaneity** — atomic content-for-payment, atomic swaps, simultaneous mutual commitment. CEG attestations are *unilateral, monotonic* graph claims; fair exchange is classically impossible without a trusted third party or a totally-ordered ledger ([Even–Goldreich–Lempel](https://en.wikipedia.org/wiki/Optimistic_fair_exchange)). CEG does **not** express it in-grammar — it **bridges** atomicity to an external settlement rail ([§5.6.8.12](05_namespace.md) `settlement` over a chain) and records only the after-the-fact trust claim. That is the honest boundary: the first domain where 1+4 reaches for something outside itself. The claim "1+4 is adequate for the federation's claims" survives *because* fair exchange is treated as out-of-grammar (a bridge, not a primitive); it would be **refuted** by either (i) a natural in-grammar expression of fair exchange, or (ii) a federation-critical domain that resists even bridging. Adversarial reviewers: this is the test to push on.

## §1.5 The Recursive Golden Rule (structural, not exhortatory)

No principal — including CIRIS L3C as steward — is exempt from constraints the protocol imposes on others. Operational bites in CEG-shape:

- **Per-install stewards bind CIRIS L3C as steward.** Once `bootstrap_threshold ≥ 2`, no single Registry install can issue federation-scope attestations unilaterally.
- **Partner-revocation rules apply to CIRIS L3C subsidiaries.** `revocation:*` carries no steward exemption.
- **Audit discipline applies to steward operations.** Every admin RPC carries the operator's identity into `actor_user_id`, including for CIRIS L3C staff.
- **Bond forfeiture applies to CIRIS L3C-affiliated partners.** No exemption.
- **The HUMANITY_ACCORD asymmetry ([§9](09_humanity_accord.md)) is the ONE constitutional asymmetry.** Three named human holders carry kill-switch authority no federation-internal authority can grant / revoke / override / decay. This is not a Golden-Rule exemption; it is the recognition that consent requires revocability, and revocability requires a halt-authority outside the system being halted.

If a principal would be exempt from a constraint at any of these primitives, the Golden Rule is violated at that primitive and the protocol is the wrong shape there. Fix the primitive, not the rule.

## §1.6 Adversary model & privacy non-goals (normative)

CEG makes confidentiality and integrity claims; this section bounds them. **The word "privacy" in this spec means exactly two things and no more: (1) content-holding confidentiality and (2) cohort-scoped visibility.** It does **not** mean metadata privacy, communication-graph privacy, or unobservability. Implementers and operators MUST NOT represent CEG as providing the stronger properties.

### §1.6.1 What the structural-invisibility primitive ([§10.1.4](10_endpoints.md)) buys
Suppressing `holds_bytes:sha256:*` for `cohort_scope: self | family` content gives **content-holding confidentiality**: a non-member cannot *discover that the bytes exist via the substrate* and cannot *fetch* them (no holder is advertised; the bytes are delivered only to admitted members via the at-rest key cascade). End-to-end content confidentiality is additionally provided by the per-epoch DEK (hybrid X25519+ML-KEM-768) and AES-256-GCM. That is the whole of what omission buys.

### §1.6.2 Non-goals — what omission does NOT buy
The following are **explicitly out of scope** at the base CEG/RET layer; treating them as provided is an error:

- **Relationship-existence privacy.** The *existence* of a self-collective / family / community and its membership-change events are observable: `family_id` / `community_id` ride the envelope, and admission / removal / consensus-protocol changes emit `hard_case:*` reserved-prefix events ([§7.7–7.8](07_reserved.md)) into the log. An observer learns that a group exists, roughly how big it is, and when its membership churns.
- **Communication-graph / metadata privacy.** DNS-free member resolution ([§8.1.13.1.1](08_composition.md)) plus Reticulum announce / path-request expose *who is reachable where*; the federation directory + `transport_destination` bindings name endpoints. A passive network observer or an honest-but-curious member can reconstruct a substantial portion of the **who-talks-to-whom** graph. Cohort scope hides *content*, not *contact*.
- **Traffic-analysis resistance.** Encrypted streams still leak via side channels the wire format does not pad or cover: the §10.5.1 STH cadence (default T=2 s), the churn-driven key-cascade volume/timing ([§10.5.3](10_endpoints.md)), and per-chunk size/rate. An observer can infer stream existence, approximate group size, churn rate, activity bursts, and often media bitrate class — without decrypting a byte.
- **Unobservability / anonymity.** Base CEG/RET provides neither sender/receiver anonymity nor cover traffic. Self-certifying cryptographic identities are *pseudonymous*, and the transport reveals path endpoints. Anonymity is a **separate, opt-in** mechanism (the CIRISNodeCore Anonymous Tier — Sphinx onion routing), NOT a property of base CEG.
- **Post-compromise security (PCS) for streams.** The CEG 0.7 [§11.7.1](11_governance.md) Option-A choice is forward-only: a member removed at epoch *e* cannot read epoch *e+1*, but a *compromised current member's* key is not self-healed by a key-update the way MLS PCS provides (revisit tracked for CEG 0.15).

### §1.6.3 Adversary classes (and where each is / is not addressed)
| Adversary | Addressed | NOT addressed |
|---|---|---|
| Passive network observer | content confidentiality (AEAD + DEK); equivocation (STH) | comm-graph, traffic analysis, group size/churn inference |
| Honest-but-curious member | — (members see in-scope content by design) | can enumerate co-members + reconstruct local comm-graph |
| Malicious member | cannot forge others' attestations; removal is forward-secret | can leak content they were entitled to; metadata as above |
| Compromised substrate node | cannot decrypt self/family content (no DEK); CEG-native replication carries signed provenance | can observe directory metadata + traffic patterns it routes |
| Equivocating producer | **mitigated** — per-stream STH ([§10.5.1](10_endpoints.md)) + consistency proofs ([§10.3.1](10_endpoints.md)): cannot show different chunk-K to different viewers nor rewrite mid-stream | — |

**Operator guidance:** if a deployment requires metadata privacy or unobservability (e.g., under a totalitarian-threat model), it MUST layer the Anonymous Tier; base CEG/RET is not sufficient. State this in any user-facing privacy representation.


