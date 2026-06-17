
---

# §3 The primitive set — 1+4

## §3.1 The workhorse: `scores`

The federation has exactly **one** workhorse attestation primitive. Every claim about an entity — positive or negative, identity or capability or behavior or state or commitment, by any attester source — is expressed as a `scores` attestation on a named dimension.

```
// Wire shape (Persist's federation_attestations row):
attestation_type: "scores"
attesting_key_id: <attester's key_id>
attested_key_id:  <subject's key_id>
attestation_envelope: {
  "dimension":      "<canonical-namespace-prefix>:<scoped-leaf>",
  "score":          <f64 ∈ [-1.0, +1.0]>,
  "confidence":     <f64 ∈ [0.0, 1.0]>,
  "context":        "<free-form scoping detail>",
  "evidence_refs":  ["<URI or hash referencing backing evidence>", ...],
  "valid_until":    "<ISO8601 datetime, optional>",
  "epistemic_mode": "<direct | crypto | hearsay | derivative | appeal>",   // optional; default 'direct'
  "witness_relation": "<self | external | derived>",                       // optional; default 'external'
  "oversight_mode": "<HITL | HOTL | HOOTL | null>",                        // optional; default null
  "occurrence_id":  "<occurrence-N | __shared__ | null>",                  // optional; default null
  "occurrence_count": <int ≥ 1 | null>,                                    // optional; default null
  "occurrence_role": "<primary | shared | replica | null>",                // optional; default null
  "stake":          "<free | reputational | capital | cryptoeconomic>"     // optional; default 'reputational'
}
```

Full field semantics in [§4](04_envelope.md).

## §3.2 The four structural composers

Operations on the attestation graph itself, not score-claims on entities:

| `attestation_type` | What it does | Envelope shape |
|---|---|---|
| `delegates_to` | A authorizes B to sign on A's behalf within a bounded scope | `{delegated_scope[], delegation_purpose, delegation_valid_from, delegation_valid_until}` |
| `supersedes` | This attestation row replaces a prior one by the same attester | `{references_attestation_id, supersession_reason, differs_in[]}` |
| `withdraws` | I retract my prior attestation (does NOT claim it was false) | `{references_attestation_id, withdrawal_reason}` |
| `recants` | My prior attestation was false at issuance — admits epistemic error | `{references_attestation_id, recantation_reason, what_was_false}` |

**Translation implications**:

- A **doctrinal-development** claim ("this version extends but does not contradict the prior version") is `supersedes` with `differs_in: ["scope", "evidence_refs"]` — NOT `recants` (which would assert prior was false).
- An **acknowledged-error** claim ("the prior framing was wrong; I admit the mistake") is `recants` — distinct from `withdraws` (which retracts without making a falsity claim).
- A **prudent-retraction** ("I'm withdrawing without claiming it was false; context has changed") is `withdraws`.
- An **authority-source claim via delegation** ("this constitutional position derives from authority-key X in scope Y") is `delegates_to` with X as `attested_key_id` (the §3.2.1 reuse pattern for authority-source claims, replacing what would otherwise need a `grounding:{tradition}:{principle}` prefix that fails [§1.3.1](01_foundation.md) T2).

### §3.2.1 Authority-source claims via `delegates_to`

A constitutional or framework claim can name its source-of-authority by emitting `delegates_to` against an `attested_key_id` representing the framework, with `delegated_scope` naming the principle. Example: a Ubuntu-substrate commitment in [§1.2](01_foundation.md) commitment 2 can be expressed as `delegates_to` against the `ubuntu_relational_substrate` framework-key with `delegated_scope: ["personhood_constitutive_by_attestation"]`. Reuses the existing structural primitive rather than introducing a `grounding:{tradition}:{principle}` prefix (which would fail [§1.3.1](01_foundation.md) T2 — "tradition" claims are interpretive, not mechanism-descriptive).

### §3.2.2 The `recants` distinction matters

Per `PRIOR_ART_SCAN.md` Bucket 1: no prior identity system (PGP, SPKI/SDSI, W3C VC) typed epistemic-error-admission as a wire primitive distinct from retraction. CEG types both because the Recursive Golden Rule applies to attesters: admitting error is a primary act, not a derivative of retraction. Consumer policy can apply different trust adjustments to attesters who `recant` versus those who `withdraw`.

### §3.2.3 `withdraws` admission rule — CEG 0.6 broadening (semantic, not structural)


Substrate MUST admit a `withdraws` Contribution against target `T` when the issuer's `key_id` satisfies **ANY** of:

| # | Authority path | Description |
|---|---|---|
| 1 | `issuer.key_id == T.attesting_key_id` | Producer self-withdraw (today's shape; unchanged) |
| 2 | `issuer.key_id ∈ T.subject_key_ids` | Federation-keys subject revocation (NEW — CEG 0.6) |
| 3 | ∃ `delegates_to` chain: `issuer →* canonical_hash` where `canonical_hash ∈ T.subject_key_ids` AND `scope ⊇ {consent_revocation}` | Proxy authority for non-federation-enrolled subjects (NEW — CEG 0.6; resolves [CIRISAgent#840 OQ3](https://github.com/CIRISAI/CIRISAgent/issues/840)) |
| 4 | `issuer` holds valid `delegates_to → any of 1-3` | Delegated revocation (existing primitive, new admission path) |

**Rule (3) is the elegant answer to the un-enrolled-party case.** When a subject is a Discord user-id, a content-sha256-bound entity, or any other non-federation party, revocation authority is mediated through a `delegates_to` chain from a federation-keys signer (typically the agent that holds data on behalf of the external party) to the canonical-hash subject. The agent emits `delegates_to(canonical_hash → agent_key, scope: [consent_revocation])` at proxy-establishment time; subsequent `withdraws` from the agent against any Contribution carrying `canonical_hash` in its `subject_key_ids` is admitted under rule (3). The `canonical_hash` here is the tagged `canonical:{hashalg}:{hex}` wire form with the `{platform}:{entity_kind}:{id}` preimage convention pinned at [§4.2.2.1](04_envelope.md). **Rule (3) proxy is distinct from `canonical_binding`** (a retroactive identity claim that promotes a canonical-hash to a real key_id, enabling rule (2) direct revocation) — see [§4.2.2.2](04_envelope.md) for the distinction; `canonical_binding` is not a new admission rule.

**Per-rule audit metadata**: substrate SHOULD record which admission rule (1-4) admitted each `withdraws` Contribution in `federation_attestations` metadata so downstream consumers can compose policy (e.g., higher confidence weight for subject self-revocation rule 2 than for proxy rule 3).

**Composition with `recants`**: subject-side authority does NOT extend to `recants` (the falsity-admission primitive) — only the original attester can `recant` their own claim. A subject who believes the producer's claim about them is FACTUALLY wrong (not merely unwanted) issues `scores` with negative polarity on a contradicting dimension; that is the consumer-side rebuttal path, distinct from consent revocation. The `recants` / `withdraws` distinction matters precisely because subject authority covers the consent dimension (revocability) but not the truth dimension (falsity).


