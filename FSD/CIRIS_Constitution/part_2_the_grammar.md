# Part II — The Grammar

**CC decimal range** `2.x` · **42 concepts** · **page budget 16.8pp** (∝ importance) · [← master index](README.md)

> The minimal-and-adequate wire grammar: the envelope every statement rides in, the five primitives,
> the conformance levels, and the canonicalization that makes two implementations agree byte-for-byte.
> This is the **frozen surface** — the bytes M-1's *adaptive* clause (1.7) is rendered in. The
> Foundation declared the 1+4 set *adequate*; this Part is where it is *specified*.

---

## 2.1 `envelope` — The envelope
<sub>budget 2.6pp · import #4 · from **CEG §4** · semantic id `envelope`</sub>

Every attestation the federation emits is one **envelope**: a signed record carrying the claim plus
the metadata a consumer needs to weigh it. The envelope is the unit of trust — nothing the
federation knows exists outside one. Its fields divide into three roles:

- **Who & what** — `attesting_key_id` (the signer), `dimension` (the named claim, gated by the
  [admission gate](part_1_foundation.md#12-admission--the-four-test-prefix-admission-gate)), `score`
  (the scalar value), `confidence`.
- **Who it's about & who may revoke it** — `subject_key_ids` (2.3), `cohort_scope`, `community_id`
  / `family_id`. This is the **revocability** half of consent — the structural expression of
  [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy).
- **How to weigh it** — `epistemic_mode`, `witness_relation`, `oversight_mode`, `stake`,
  `evidence_refs`, `valid_until`. A consumer is never told *what to believe*; it is given what it
  needs to decide for itself.

### 2.1.1 `forward-compatibility`

The envelope is **append-tolerant**: a consumer MUST ignore fields it does not recognise rather than
reject the record. This is how the grammar stays frozen at the 1+4 surface while still admitting new
*metadata* — the structural primitives never change; the descriptive fields may grow. (Normative:
CEG §4.1.)

---

## 2.2 `conformance` — Conformance levels
<sub>budget 1.2pp · import #13 · from **CEG §0.2** · semantic id `conformance`</sub>

The grammar defines what it means to *speak it correctly*. A **CEG-Conforming Consumer (CCC)** is
held to the normative rules of every Part it relies on; a producer is held to the canonicalization
and signing rules. Conformance is graded and testable (Part VIII carries the vectors): an
implementation either reproduces the canonical bytes or it does not. This is
[integrity](part_1_foundation.md#18-integrity--integrity) made checkable — "conformant" is a
verdict a third party can re-derive, not a self-assertion. (Normative: CEG §0.2; conformance
language §0.1 / §2.6.9.)

---

## 2.3 `subject_keys` — `subject_key_ids` semantics
<sub>budget 1.1pp · import #16 · from **CEG §4.2** · semantic id `subject_keys`</sub>

The half of consent the baseline grammar lacked: where `attesting_key_id` encodes *producer*
authority, `subject_key_ids` encodes **subject** authority — the parties with substrate-recognised
power to revoke or qualify a statement *about them*. Each listed key may issue a `withdraws` against
the record and emit `consent:*` dimensions over it.

This is the wire root of [autonomy](part_1_foundation.md#14-autonomy--respect-for-autonomy): consent
is only real if it is *revocable*, and revocability requires the subject to be named in the bytes.
Its sub-rules are the precise mechanics — subject-bearing dimensions that *require* a subject (2.3.1,
the default-leak gate), the federation-key vs. canonical-hash identifier forms (2.3.2, with the
canonical-hash preimage convention pinned at 2.3.2.1), the orthogonality of *visibility*
(`cohort_scope`) vs *revocability* (`subject_key_ids`) vs *delivery* (2.3.3), and the self-consent
ceremony where attester and subject coincide (2.3.4). (Normative: CEG §4.2 and subsections.)

---

## 2.4 `primitive` — The primitive set (1+4)
<sub>budget 1.1pp · import #18 · from **CEG §3** · semantic id `primitive`</sub>

The Foundation named the 1+4 set and claimed it minimal-and-adequate (1.7); here it is specified.

### 2.4.1 `structure` — The four structural composers

The four relations operate on the attestation graph itself:

- **`delegates_to`** (2.4.1.2) — A may sign on behalf of B within a scope. Authority is *granted*,
  never assumed; the grant chains, is depth-capped, and is revocable. This single primitive carries
  the federation's entire delegation model — moderation duties, key custody, on-behalf signing.
- **`supersedes`** — this record replaces a prior one; the adaptation primitive.
- **`withdraws`** (2.4.1.1) — I retract a prior record (not necessarily because it was false). Its
  CEG 0.6 broadening — that any `subject_key_ids` holder may withdraw, not only the producer — is
  the highest-importance primitive sub-rule in this Part, because it is where revocability becomes
  enforceable.
- **`recants`** (2.4.1.3) — my prior record *was* false. The distinction from `withdraws` matters:
  withdrawal is "I no longer assert this"; recantation is "this was wrong" — an admission of
  epistemic error, which the [coherence ratchet](part_6_the_coherence_mathematics.md) treats
  differently from a mere retraction.

### 2.4.2 `scores` — The workhorse

The one attestation type. Every substantive claim — a licensure attestation, a capacity score, a
consent grant, a moderation verdict — is a `scores` on a named dimension. The categorical button in
a UI ("Mark Licensed") writes a scalar `scores` underneath; the vocabulary lives *above* the wire,
never in it. (Normative: CEG §3.1.)

---

## 2.5 `reasoning` — The reasoning grammar (the eight axes)
<sub>budget 0.5pp · import #65 · from **CEG §2** · semantic id `reasoning`</sub>

Beyond *what* is claimed, the grammar names *how the claim was formed* — eight orthogonal axes a
consumer may weigh (epistemic mode, witness relation, stake, oversight mode, and the rest). They let
the federation distinguish direct witness from hearsay, reputational stake from bonded stake,
human-in-the-loop from autonomous — without ever collapsing those distinctions into a single
"trust score." Diversity of perspective is preserved at the wire level. (Normative: CEG §2.)

---

## 2.6 `foreword` — Front matter & canonicalization
<sub>budget ~4.0pp (with subsections) · import #173 · from **CEG §0** · semantic id `foreword`</sub>

CEG's §0 front matter nests here: the conformance language, versioning policy, normative references,
and — most importantly — the **canonicalization rules** that make the frozen surface actually
deterministic across implementations.

### 2.6.1 `envelope-canonicalization` — JCS + the omit-vs-materialize rule
<sub>budget ~1.6pp · import #21 · from **CEG §0.9**</sub>

Two honest implementations must produce **byte-identical** signed preimages or signatures do not
transfer. The canonicalization rule (JSON Canonicalization Scheme + three determinism rules JCS does
*not* pin: array ordering, byte-field encoding, timestamp form — 2.6.1.1.1) is what guarantees that.
Its load-bearing subtlety is the **round-trip rule** (2.6.1.1): defaults are applied at
*interpretation* time, never baked into the *encoding* — so an omitted field and its materialised
default canonicalize identically, closing a signature-malleability attack (2.6.1.4). This is
[integrity](part_1_foundation.md#18-integrity--integrity) at the byte level: the bytes mean exactly
one thing, verifiably. (Normative: CEG §0.9 and subsections.)

### 2.6.2–2.6.9 — the determinism primitives

The remaining canonicalization rules each pin one encoding so it cannot drift: **date-time** (2.6.2),
**hexadecimal** (2.6.3), **H3 geographic cell** (2.6.6, with rough-only enforcement for
`location_proof` at 2.6.6.1 — a privacy floor: location attestations are *deliberately* coarse),
**time and clocks** (2.6.7), the **`key_id` shorthand** (2.6.8), plus versioning policy (2.6.4) and
normative references (2.6.5). Individually small; collectively they are why "the wire is frozen" is a
*checkable* claim and not a hope. (Normative: CEG §0.3–§0.10.)

---

*Part II is the document's frozen core. Its deep tail — the per-field encoding tables, the worked
attacks, the CONFIRM-resolution sub-rules — is migrated verbatim in Phase 4 with full `legacy_ref`
provenance ([`toc.tsv`](toc.tsv)); the importance graph keeps it page-thin here because the
federation leans on the envelope and the canonicalization rule, while the encoding minutiae are
consulted, not read.*
