[← §2 Grammar](02_grammar.md) | **§3 Primitives** | [Next: §4 Envelope →](04_envelope.md)

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

---

[← §2 Grammar](02_grammar.md) | **§3 Primitives** | [Next: §4 Envelope →](04_envelope.md)
