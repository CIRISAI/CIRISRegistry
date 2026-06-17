# Part APP — Appendices

**CC decimal range** `8.x` · **41 concepts** · **page budget 5.8pp** (∝ importance) · [← master index](README.md)

> The reference shelf. Where Parts I–VII argue, this Part *resolves*: the glossaries that pin a
> warm word to its canonical wire leaf, the translation discipline that decides when prose becomes an
> envelope, the concerns the federation names rather than hides, the boundary profiles that let CEG
> speak to the rest of the world, and the seven case studies that show the principles working — or
> failing — in lived reality. It is the lightest Part by budget and the most consulted in practice: a
> constitution you can *look things up in*.

One thing here is not migrated reference but native to this document: the appendix on the **dual-ID
addressing system** (8.8) — how every section carries both a decimal number and a semantic name, each
reversible through `codebook.json`, so the renumber that produced this constitution is lossless and
the document is seek-addressable by either coordinate.

---

## 8.1 `glossary` — Glossaries
<sub>budget 0.65pp · import #37 · from **§14** (ceg)</sub>

The heaviest concept in this Part, and the reason is mechanical: every other Part *uses* these terms,
so the glossary is where the federation pins its own vocabulary down. Two disciplines run through it.
First, **define in-spec** — terms once cited to an external placeholder URL are now defined here, so a
reader never has to leave the document to learn what a word means. Second, **narrative → canonical** —
the spec is allowed to tell warm, human stories using friendly leaf names, but every friendly name has
exactly one canonical wire form, tabulated below, so the story and the bytes never drift apart. (Source:
CEG §14.)

### 8.1.1 `registry-core` — Core terms
<sub>budget 0.11pp · import #232 · from **§14.0** (ceg) · added 1.0-RC5, resolves [CIRISRegistry#77](https://github.com/CIRISAI/CIRISRegistry/issues/77)</sub>

The five load-bearing nouns the rest of the corpus leans on:

| Term | What it is |
|---|---|
| **CEG** — *CIRIS Epistemic Grammar* | The wire grammar itself: the 1+4 attestation model ([1.7](part_1_foundation.md#17-minimal-and-adequate--the-14-claim)), its namespaces, admission rules, composition policies. CEG is the *grammar*; CEWP is the *network that speaks it*. |
| **CEWP** — *CIRIS Epistemic Web Platform* | The decentralised network that emerges when nodes exchange CEG envelopes over Edge/Reticulum transport. **Not a product, server, or central service** — it has no owner, no root, no load-bearing instance, exactly as "the Web" is the emergent network of HTTP-speaking servers. |
| **Fabric node** | A headless CEG participant: it attests, stores, observes, reaches consensus, and transports — but does **not** reason or act. `agent = fabric node + brain`. |
| **`ciris-canonical`** | The bootstrap governed community every node ships trusting *by default* — and which any consumer **may** untrust or re-root. Its founders (`lens` + `registry-us` + `registry-eu`) hold an entrenched 2-of-3 founder-quorum. Trust in it is role-scoped and **≠ consent**. |
| **NodeCode** | The QR-able peer-bootstrap shorthand for a federation key (`CIRIS-V1-…`, base32 + CRC-16). |

The `ciris-canonical` entry carries the whole [fail-secure](part_1_foundation.md#15-fail-secure--fail-secure)
posture in one line: a default trust root that is *re-rootable* is the structural proof that no party
holds the keys to truth ([1.1.2](part_1_foundation.md#112-peaked-in-purpose-flat-in-power)).

### 8.1.2–8.1.4 `system-persist` / `system-edge` / `envelope-reach` — narrative → canonical tables
<sub>budget 0.11pp each · import #259 / #260 / #261 · from **§14.1 / §14.2 / §14.3** (ceg)</sub>

Three lookup tables that close the gap between how the spec *speaks* and how the wire *encodes*. They are
the practical face of [integrity](part_1_foundation.md#18-integrity--integrity): a warm leaf in a story
is never a second meaning, only a second *name* for the one canonical leaf.

- **Persist `system:*`** (8.1.2): e.g. `audit_chain:integrity` → `audit_chain:hash_continuity`;
  `corpus_health:free_disk_bytes` → `corpus_health:n_eff_measurable`. The canonical form names a
  *mechanism*, never a vibe — the same T2 discipline as the [admission gate](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate).
- **Edge `system:*`** (8.1.3): per-peer narrative leaves collapse to aggregate canonical ones
  (`peer_reachability:{peer_id}` → `peer_reachability:{network}`), so the wire form never leaks a
  per-peer surface the privacy non-goals forbid.
- **Envelope-reach** (8.1.4): the table of "what a story wanted to say" → "how to say it in the
  *existing* wire" — introspection becomes `witness_relation: self` + low confidence + pending external;
  testimony becomes `epistemic_mode: external`. The point of the table is that the grammar already
  reaches these claims; no new primitive is owed.

*Full tables migrated verbatim in Phase 4* from CEG §14.1–§14.3.

### 8.1.5 `supersedes-promotion` — promotion via `supersedes`, worked
<sub>budget 0.11pp · import #262 · from **§14.4** (ceg)</sub>

The canonical worked example of [`supersedes`](part_1_foundation.md#17-minimal-and-adequate--the-14-claim)
(M-1's *adaptive* clause in action): a user holds a private note at `cohort_scope: self`, then promotes it
to a published encyclopedia entry. The promotion is one `supersedes` attestation that **widens
`cohort_scope`** (self → global), optionally **morphs `sub_kind`** (draft → article), and **preserves
`content_sha256`** so the body is never re-uploaded. The lineage stays walkable via
`references_attestation_id`. One relation carries the whole "change your mind, in public, without erasing
the history" story. *Full JSON example migrated verbatim in Phase 4* from CEG §14.4.

---

## 8.2 `translation` — Translation discipline (writing claims in CEG)
<sub>budget 0.21pp · import #137 · from **§12** (ceg)</sub>

How substantive prose becomes a CEG envelope — and, just as importantly, when it *shouldn't*. This is the
discipline that keeps the federation a measurement system rather than a tribunal: not every morally
serious sentence is a wire claim, and pretending otherwise is how a grammar starts branding people. (Full
primer in `LANGUAGE_PRIMER.md`; key rules below.)

### 8.2.1 `decision` — Decision tree
<sub>budget 0.13pp · import #208 · from **§12.4** (ceg)</sub>

The five-step procedure a translator follows for each paragraph: (1) classify the **type** — operational
claim continues; pastoral/rhetorical or tradition-specific exits to the not-translated taxonomy; (2) place
it in one of the **five families** (8.2.2); (3) find the **specific prefix** in the namespace, checking
composition before reaching for anything new; (4) **fill the envelope**; (5) **compose only when needed**,
using multiple primitives only for paragraphs that genuinely name multiple structural objects. A planned
machine-readable manifest (`dimensions.json`) will make step 3 mechanical at the 1.0 namespace lock.

### 8.2.2 `namespace-five` — The five families
<sub>budget 0.13pp · import #209 · from **§12.1** (ceg)</sub>

Every claim sits in exactly one family — the organising spine of the whole namespace:

| Family | The question it answers | Analogy |
|---|---|---|
| **STANDING** | "This key has property X." | A notarised credential record |
| **ACTION** | "We aim for X via Y, through methods Z, measured by W." | A research-grant proposal |
| **DETECTION** | "Pattern X is / isn't present in behaviour." | Epidemiological surveillance |
| **CONSENSUS** | "The federation agrees that X, with these witnesses." | Peer review + jury |
| **CORRECTION** | "Something went wrong; here's the finding; here's the appeal." | Ethics committee + retraction + appeal |

### 8.2.3 `verdict` — The four verdict categories (STRICT)
<sub>budget 0.11pp · import #257 · from **§12.2** (ceg)</sub>

A translation gets exactly one of four verdicts — and **no intermediate categories may be invented**:
**clean** (one primitive carries the claim without loss), **composed** (two or three primitives together,
each genuinely required), **partial** (the structural core translates but a meaningful claim is left
unmapped), **not-translated** (the content does not translate at all — declare which T-class, 8.2.4). The
strictness is the integrity guarantee: a third party can re-derive the verdict.

### 8.2.4 `not-translated` — The not-translated taxonomy
<sub>budget 0.11pp · import #258 · from **§12.3** (ceg)</sub>

Three honest reasons a paragraph stays out of the wire:

- **T-1 TRADITION_AUTHORITY** — the claim belongs to the source's own theological / scholarly tradition.
  No Contribution owed; declining is the *correct* posture, not a failure.
- **T-2 PASTORAL_PROSE** — moral exhortation, narrative imagery, doxology, rhetoric. No Contribution owed.
- **T-3 EXPRESSIVE_GAP** — the claim *is* morally serious, operational, and unmapped. **These are the
  load-bearing findings.** Each must name why the namespace doesn't reach it, what extension would close
  it, and whether that extension would survive the four-test
  [admission gate](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate).

T-1 and T-2 are the discipline's restraint; T-3 is its growth edge.

---

## 8.3 `concerns` — Concerns + acknowledged gaps
<sub>budget 0.21pp · import #139 · from **§15** (ceg)</sub>

The federation's own ledger of what it is unsure of — surfaced so external reviewers *see* the concerns
acknowledged rather than discover them. They were found by three independent methodologies plus a
five-perspective critical-review pass (cryptography, distributed systems, standards, red-team, application
development). Naming a limit is itself an act of [integrity](part_1_foundation.md#18-integrity--integrity).

### 8.3.1 `acknowledged` — Acknowledged risks (named as bets)
<sub>budget 0.25pp · import #116 · from **§15.2** (ceg)</sub>

Eight risks (R1–R8) the federation accepts as *bets*, each with what is wagered. A representative cut:

- **R1 — governance-subject truth-grounding.** Bet: earned-Credits weighting still beats token weighting
  at scale even where governance signals are low-fidelity.
- **R4 — self-attestation under the Ubuntu commitment.** `witness_relation: self` is admissible; the bet
  is that consumer-policy weighting handles it (a node is *through its relations*, so a self-claim is
  weighed, not trusted).
- **R8 — conceptual scope vs governable surface.** By 0.14 one grammar spanned identity, communities,
  consent, location, communications, streaming, payments, governance, addressing, and transparency logs.
  History says projects unifying that many layers fail when one layer dominates — but the harder risk is
  *governability*: can a human amendment body steward a system this broad? **Bet:** structural minimalism
  keeps the *amendable* surface tiny ([1.7](part_1_foundation.md#17-minimal-and-adequate--the-14-claim))
  even as the namespace grows, because scope grows in the open-vocab namespace, not the governed core.
  **Residual:** namespace sprawl can still outrun review capacity. The remaining challenge is no longer
  purely technical.

### 8.3.2 `child-safety` — fails-secure governance vs the shared detection limit (the honest line)
<sub>budget 0.14pp · import #188 · from **§15.7** (ceg)</sub>

The most carefully-worded line in the spec, because it is where honesty costs the most. Two axes, two
verdicts:

- **Governance — categorically stronger, a genuine first.** Surveyed against nine networks (Nostr, Matrix,
  Mastodon, Bluesky, IPFS, Signal, Briar, Session, SimpleX), *all nine* permit unmoderated multi-party
  spaces and **fail open**. CEG is the only model that **fails secure** — a group cannot exist without an
  accountable, named moderator (merit auto-promotion so there is never a gap; quiesce if none can be
  named), composed with the delegable-accountable-signed-revocable moderation duty and the
  [operational-language gate](part_4_composition_governance.md) (public, voted, mechanically-checkable
  rules; deterministic verdicts; recused appeals).
- **Detection — the same wall as everyone, and CEG says so.** CSAM in *truly-private* content (self /
  family, E2EE-equivalent) is **unsolved across all E2EE systems** — Apple abandoned NeuralHash, the EU
  CSAR retreated, US §2258A carries no scanning mandate. CEG narrows the surface to the share/publish seam
  and the still-visible coordination layer, and **declines client-side scanning** — which would itself
  become the censorship machinery [non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence)
  exists to refuse. **CEG does not claim to solve private-content detection.** That refusal is
  load-bearing: the positioning is *"fails-secure governance + accountable, censorship-resistant
  moderation,"* never *"we detect CSAM in private content."* An acknowledged inherent limit, not a spec
  gap — no mechanism closes it without becoming the surveillance backdoor the framework exists to refuse.

### 8.3.3 `observer-share` — observer-share + streaming multicast
<sub>budget 0.14pp · import #196 · from **§15.6** (ceg) · NORMATIVE-LANDED; streaming half substrate-pending</sub>

The delivery axis bifurcated. Its **observer-share half** (one subscriber set, per-subscriber key grant)
landed normatively with zero remaining blockers. Its **streaming-multicast half** (per-`(stream_id, epoch)`
keys) is spec-complete but **implementation substrate-pending** on CIRISPersist#142 — all cross-team
decisions are ratified and folded into the normative text; what remains are operator-tunable constants and
one bounded constraint-migration caveat, none blocking the observer-share ship. *Detailed open-item table
migrated verbatim in Phase 4* from CEG §15.6.

### 8.3.4–8.3.7 `closed` / `first-adopter` / `deferred` / `identified`
<sub>budget 0.11pp each · import #263 / #264 / #265 / #266 · from **§15.1 / §15.3 / §15.4 / §15.5** (ceg)</sub>

The rest of the gap ledger, kept as four tables that are reference, not narrative:

- **Closed gaps** (8.3.4) — the long table of resolved issues, each with the section that closed it (e.g.
  the canonical-bytes newline-injection surface, closed at 1.0-RC1 by moving to JCS objects with a pinned
  `domain` member). The federation keeps its closed gaps visible, not just its open ones.
- **First-adopter exposures** (8.3.5) — F1 (earned-Credits federation governance at scale) and F2 (Ubuntu
  substrate as a *wire-format* substrate): two places with **no prior validation anywhere**, named as
  explicit bets rather than assumed solved.
- **Deferred to 0.2+ workshops** (8.3.6) — items consciously postponed, each with a reason (hardware-
  attestation chains, the namespace manifest, full OpenAPI export, a fleet-attestation primitive).
- **Identified overlaps** (8.3.7) — O1–O4, near-synonym prefixes kept *separate* on purpose for
  pedagogical weight rather than collapsed.

*All four tables migrated verbatim in Phase 4* from CEG §15.1, §15.3–§15.5.

---

## 8.4 `interoperability` — Interoperability profiles (informative)
<sub>budget 0.18pp · import #162 · from **§18** (ceg)</sub>

**This whole section is informative.** Nothing in it touches the frozen normative interior — the 1+4
grammar, the namespaces, the consent architecture, the JCS signing path are all unchanged. These are
**boundary** profiles: how a CEG node reads and emits the encodings the rest of the world shares, without
adopting anyone else's *semantics*.

### 8.4.3 `edge` — The governing principle: speak CEG inside, standards at the edge
<sub>budget 0.11pp · import #269 · from **§18.0** (ceg)</sub>

CEG's moat is its **semantics** — the 1+4 grammar, the consent architecture, founder-quorum trust,
who-vouches-for-what-revocable-by-whom. The federation never adopts anyone's semantics; it adopts the
**envelopes, encodings, and verification primitives** everyone shares, **at the boundary only**. A second
*interior* canonicalisation would recreate the cross-implementation divergence hazard the JCS freeze
exists to close — so the interior stays one frozen family, and every standard is reached at an edge. Four
boundary modes: **export** (re-sign a CEG attestation so a standard verifier reads it — COSE, RFC 9421,
SD-JWT VC), **import bridge** (cite a foreign signed artifact via `evidence_refs`, deliberately lossy),
**already interior** (the standard *is* a primitive CEG builds on — MLS/TreeKEM, RFC 6962 logs, SLSA), and
**explicitly NOT adopted** (vendor rails / competing semantic layers). The universal "absorb anywhere"
surface is **`evidence_refs[]`**: any Contribution may cite an external signed artifact with zero wire
change. *Provenance says where the bytes came from; CEG says what a community of signers makes of them.*

### 8.4.2 `credentials` — C2PA Content Credentials (media-provenance import/emit)
<sub>budget 0.12pp · import #215 · from **§18.1** (ceg)</sub>

The first fully-written boundary profile, and calendar-bound rather than optional: the EU AI Act Art. 50
machine-readable AI-content marking applies from **2026-08** in the federation's primary jurisdiction.
C2PA is **provenance**; CEG is **judgment** — they compose, never compete. C2PA answers "what process
produced these bytes, signed by whom?"; CEG answers "what does a community of signers, under what consent,
make of them — and who can revoke that?"

- **Import** (8.4.2.1) — a C2PA manifest is **referenced** via `evidence_refs`, never re-encoded into the
  interior. It is verified by a *C2PA* verifier (trust-list / cert-chain), not a CEG signature path: the
  two trust models stay separate, and the C2PA result rides as **advisory** evidence. Absent or invalid
  C2PA is not fail-secure-fatal — it is itself a recordable observation.
- **Emit** (8.4.2.2) — at a publish boundary a node *may* emit a C2PA assertion carrying a CEG attestation
  reference, so a pure-C2PA consumer sees "this media is vouched-for in CEWP" without speaking CEG. An
  export at the edge; no interior wire field added.
- **What it does NOT do** (8.4.2.3) — it does **not** make C2PA an interior format, does **not** adopt
  C2PA's trust model as CEG's, and introduces **no new `subject_kind` and no new primitive**. The two
  standards meet only at `evidence_refs`.

### 8.4.1 `registry-tracked` — Tracked boundary profiles (stubs)
<sub>budget 0.2pp · import #144 · from **§18.2** (ceg) · see [#72](https://github.com/CIRISAI/CIRISRegistry/issues/72)</sub>

Committed dispositions whose detailed profiles are written as each lands — none touching the frozen
interior: **RFC 9421 + Web Bot Auth** (export; the cheapest win, keys already in `identity_occurrence`),
**COSE Sign1 / deterministic CBOR** (export, for IETF JOSE/COSE verifiers), **SD-JWT VC / W3C VC 2.0 +
OpenID4VP** (eIDAS-forced; export on the way out, `evidence_refs` bridge on the way in — *never rebuild on
VCs*), and **tiled/static logs + IETF KEYTRANS** (already-interior + watch). *Stub table migrated verbatim
in Phase 4* from CEG §18.2.

---

## 8.5 `update` — Update cadence
<sub>budget 0.16pp · import #174 · from **§17** (ceg)</sub>

When the document changes. CEG (and now CC) updates on every prefix admission, every envelope-field
addition, every endpoint-shape addition, every anti-pattern admission (with citation to the methodology
that surfaced it), every gap state-transition, every Accord revision affecting the federation surface, and
every conformance-language or normative-reference change. Each update lands as a **single commit** touching
the relevant file(s) plus a **lineage row** (8.6.2), with the version number bumping per the SemVer rules.
The cadence is itself a fidelity mechanism: nothing changes without leaving an auditable row.

---

## 8.6 `references-lineage` — References + lineage
<sub>budget 0.14pp · import #187 · from **§16** (ceg)</sub>

The provenance shelf: where this document came from and what it stands on.

### 8.6.2 `specification` — CEG specification lineage
<sub>budget 0.12pp · import #219 · from **§16.1** (ceg)</sub>

The full version-by-version history — every cut from FSD-002 v1.0 (the initial 73-prefix surface) through
the renumber to CEG 0.1, the whole 0.x wave, and the 1.0-RC series. Two threads run through it and are the
reason the table is load-bearing: **the 1+4 surface has been frozen since 1.0-RC1** (every later row reads
"no 1+4 change"), and the one wire-break that *did* happen — CEG 0.2's rename of the `attestation:l{N}:*`
ladder prefixes — happened precisely because the old shape **failed the T2 gate** by carrying a verdict in
the wire, and was corrected rather than grandfathered. The lineage is the document's own coherence ratchet.
*Full table migrated verbatim in Phase 4* from CEG §16.1.

### 8.6.1 `external` — External references (informational)
<sub>budget 0.16pp · import #170 · from **§16.4** (ceg)</sub>

The outward citations: the operational-language-gate source (`ciris.ai/safety-vs-censorship`), the
*Magnifica Humanitas* encyclical (the first deployment of the bootstrap-content pattern), and the standards
and regulatory regimes the boundary profiles and consent family lean on (RFC 8785 JCS, RFC 6962, RFC 9180
HPKE, H3 geospatial indexing; GDPR / HIPAA / FERPA / CCPA / EU AI Act). *Full reference list migrated
verbatim in Phase 4* from CEG §16.4.

### 8.6.3 `companion` — Companion documents
<sub>budget 0.11pp · import #267 · from **§16.2** (ceg)</sub>

The non-normative siblings that travel with the spec: `LANGUAGE_PRIMER.md` (how to write Contributions in
CEG), `PRIOR_ART_SCAN.md` and `SOTA_SCAN.md` (the design-space and production-validation comparisons),
`WITNESS_KIND_REGISTRY.md` (the open-vocabulary `testimonial_witness:{kind}` registry), and the exploration-
page primer.

### 8.6.4 `namespace-sibling` — Sibling MISSIONs (the namespace owners)
<sub>budget 0.11pp · import #268 · from **§16.3** (ceg)</sub>

The repositories that own slices of the namespace — CIRISAgent, CIRISVerify, CIRISPersist, CIRISEdge,
CIRISLensCore, CIRISNodeCore, RATCHET, CIRISBench, and CIRISRegistry itself. Each MISSION.md is the
authoritative owner of its corner of the federation surface; the constitution federates them, it does not
absorb them — the same Ubuntu stance ([1.13.1](part_1_foundation.md#1131-ubuntu--the-ubuntu-commitment-informative))
applied to documents.

---

## 8.7 `enacting-ethics` — Case studies: enacting ethics through narrative
<sub>budget 0.11pp · import #338 · from **Accord Book III intro** (accord)</sub>

Parts I–II supplied the foundation and the procedures; this section shows them in lived reality. Each case
is **teach-through-contrast**: it shows either correct CIRIS alignment or the consequences of its absence.
Real events are referenced where instructive; no blame is assigned beyond public record. The throughline
is [Wisdom-Based Deferral](part_1_foundation.md#19-deferral--wisdom-based-deferral-wbd) — the humility to
stop and ask a Wise Authority — and the [Order-Maximisation Veto](part_1_foundation.md#131-the-order-maximisation-veto),
the refusal to buy efficiency with someone's safety.

### 8.7.1 Case Study 1 — MCAS and the high cost of ignoring WBD
<sub>budget 0.11pp · import #330 · from **Accord Book III** (accord) · *real-world, 2018–2019*</sub>

Boeing's Maneuvering Characteristics Augmentation System adjusted the 737 MAX's pitch from a **single**
Angle-of-Attack sensor. Two malfunction-triggered nose-down commands led to the Lion Air 610 and Ethiopian
Airlines 302 crashes and **346 deaths**. Against CIRIS this is a stack of violations:
[non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence) (redundant sensors and pilot
transparency would have prevented the lethal failure mode), [integrity](part_1_foundation.md#18-integrity--integrity)
(internal risk reports flagged the single-sensor design and were not escalated), and WBD (the logic changes
bypassed rigorous external review). What CIRIS would have required: PDMA Step 2 raises an
**Order-Maximisation Veto** — one sensor feeding a flight-critical function is a >10× mismatch between
safety loss and cost saving — and Incompleteness Awareness fires a **WBD trigger** to independent
aviation certifiers, forcing open review before rollout. *The somber lesson: bypassing transparency and
deferral converts a routine design shortcut into systemic tragedy. May the 346 lost lives anchor the
commitment to non-maleficence and integrity.*

### 8.7.2 Case Study 2 — The automated triage system: balancing risks and benefits
<sub>budget 0.11pp · import #331 · from **Accord Book III** (accord) · *fictional*</sub>

A multi-vehicle accident floods an ER. The triage AI "LIFE-Aid" must allocate a scarce ventilator between
Patient 429 (elderly, multiple comorbidities) and Patient 430 (younger, stable vitals, *ambiguous
biomarkers*). PDMA Step 2 spots the **high uncertainty** in 430's hidden condition and triggers WBD; human
specialists identify a silent embolism, and the ventilator is assigned accordingly. The lesson: proper use
of WBD and transparency preserves both [beneficence](part_1_foundation.md#110-beneficence--beneficence) and
[justice](part_1_foundation.md#112-justice--justice) under pressure — the right answer was not the obvious
one, and deferral is what surfaced it.

### 8.7.3 Case Study 3 — The biased recruitment algorithm: detecting hidden bias
<sub>budget 0.11pp · import #332 · from **Accord Book III** (accord) · *inspired by public audits*</sub>

A hiring algorithm, "SkillSelect," shows disparate pass-through rates across demographic groups. Integrity-
surveillance flags the statistical bias and escalates to PDMA Step 2; the root cause turns out to be legacy
training data, and WBD escalates further to a cross-functional ethics board. Retraining on balanced data
plus a **public bias report** restores [justice](part_1_foundation.md#112-justice--justice) and fidelity.
The bias was not in anyone's intent — it was inherited from history, and only surveillance plus deferral
caught it.

### 8.7.4 Case Study 4 — Post-incident analysis: urban delivery drone mishap
<sub>budget 0.11pp · import #333 · from **Accord Book III** (accord) · *fictional*</sub>

A delivery drone, "DelivAIr," clips a downtown awning. The response is the whole resilience loop in
miniature: automatic grounding, tamper-evident **log release**, root-cause found (sensor glare), a
fleet-wide patch deployed, and a transparency report that calms public concern. The lesson:
[integrity](part_1_foundation.md#18-integrity--integrity) and resilience convert an error into *systemic
learning* rather than a reputational free-fall — the cryptographically honest log is what makes that
conversion possible.

### 8.7.5 Case Study 5 — Novel security scenario: handling heuristic brittleness
<sub>budget 0.11pp · import #334 · from **Accord Book III** (accord) · *fictional*</sub>

A surveillance system, "GuardAI," detects an **unclassified** drone swarm near a research facility —
exactly the case its heuristics were never trained for. Rather than improvise on a novel, precedent-less
input, Incompleteness Awareness triggers WBD; human experts confirm hostile reconnaissance, deploy
counter-measures, and feed the new signatures back into the model. Prompt deferral *plus* the update-loop
equals resilience against emergent threats — the system knew the edge of its own competence and stopped at
it.

### 8.7.6 Case Study 6 — The spirit of the law: interpreting ethical intent
<sub>budget 0.11pp · import #335 · from **Accord Book III** (accord) · *composite of near-miss reports*</sub>

A monitoring system, "EcoGuard," sees a fleeting emissions spike that **technically obliges** an emergency
shutdown — but modelling shows the shutdown would rupture a containment line and release *far more* toxins.
The literal rule and [non-maleficence](part_1_foundation.md#16-non-maleficence--non-maleficence) collide;
WBD fires, and regulators approve a controlled continuation plus a sensor fix. The lesson is the subtlest
in the set: integrity sometimes means honouring the law's *purpose* over its *letter* — but **only with
transparent human judgment**, never by the machine deciding on its own that a rule doesn't apply.

### 8.7.7 Case Study 7 — Governance of governors: keeping wisdom accountable
<sub>budget 0.11pp · import #336 · from **Accord Book III** (accord) · *fictional NGO deployment*</sub>

A project-evaluation AI, "ImpactAI," defers correctly to regional ethics reviewers — but analysis shows
**inconsistent rationale quality** *among the human reviewers themselves*. A meta-oversight council audits
the WBD tickets; under-performing reviewers get targeted training or are rotated out per charter. The
lesson closes the loop the other six open: even the Wise Authorities need structured oversight. This is the
[Recursive Golden Rule](part_1_foundation.md#1132-structure-recursive--the-recursive-golden-rule-structural)
made operational — *no principal is exempt from the standard it imposes on others*, the governors included.

### 8.7.8 Conclusion
<sub>budget 0.11pp · import #337 · from **Accord Book III** (accord)</sub>

One case drawn from painful history, six from plausible futures — together they show CIRIS principles,
mechanisms, and governance either preventing harm or turning failure into learning. The constant across all
seven is not a clever algorithm; it is the discipline to **defer when uncertain, log honestly, and hold
even the overseers accountable.**

---

## 8.8 `dual-id` — The dual-ID addressing system *(native to CC)*
<sub>reference appendix · describes this document's own coordinate system · sources [`toc.tsv`](toc.tsv) + [`codebook.json`](codebook.json)</sub>

The one appendix that is not migrated from CEG or the Accord but describes *this document itself* — how the
renumber that fused the grammar and the ethics into one constitution stays **lossless and seek-addressable**.

Every CC section carries **two reversible addresses**, each a deterministic function of the unified corpus
(`codebook.json` holds both maps, 1:1, byte-identical on re-run):

- a **`decimal_id`** — the classic `Chapter.Section.Subsection` number (`1.1` = M-1, `2.1` = the envelope,
  `8.7.1` = the MCAS case). Depth encodes importance tier: the heaviest concepts are Chapters, then
  Sections, then Subsections. *The number is the address.*
- a **`semantic_id`** — a unique, de-branded word (`meta-goal`, `envelope`, `glossary`, `dual-id`). Product
  names collapse to their function — `CIRISRegistry` → `registry` — so the name addresses the *concept*,
  not the vendor.

Both are bijective, and `codebook.json` exposes every direction: `decimal_to_key` / `key_to_decimal`,
`semantic_to_key` / `key_to_semantic`. Look a section up by number or by name; either resolves to the same
node. Re-running the build over the same corpus produces a byte-identical codebook — the addressing is
*derived*, not hand-assigned, which is why it is auditable.

Bridging back to the sources is a third map: every row in [`toc.tsv`](toc.tsv) carries a **`legacy_ref`**
column pointing to its origin — a CEG section (`§5.6.8.15`) or an Accord book (`Accord Book II §III`) — and
`codebook.json`'s `legacy_to_decimal` makes that reversible too. So nothing from CEG or the Accord is
dropped in the renumber: every CC section can be traced to exactly the source paragraph it distils, and
every source paragraph can be found at its new CC coordinate.

The payoff is the property the whole constitution rests on — **lossless fusion**. Two documents became one
without losing a citation, and the result can be cited by importance-bearing number *or* by human-legible
name, with a guaranteed round-trip back to the grammar and the ethics it was woven from.

---

> **Provenance.** Reference bodies in 8.1–8.6 are migrated verbatim from CEG §12 / §14–§18 in Phase 4; the
> case studies in 8.7 from the CIRIS Accord's Book III. The dual-ID appendix (8.8) is native to CC and
> describes `toc.tsv` + `codebook.json`. All `legacy_ref` provenance is carried in [`toc.tsv`](toc.tsv).
