[← §3 Primitives](03_primitives.md) | **§4 Envelope** | [Next: §5 Namespace →](05_namespace.md)

---

# §4 The envelope

Every `scores` Attestation carries this envelope. Field semantics consolidated here.

| Field | Required | Description |
|---|:---:|---|
| `attesting_key_id` | (substrate field) | Attester's `federation_keys.key_id`. |
| `attested_key_id` | (substrate field) | Subject's `federation_keys.key_id`. |
| `dimension` | yes | The canonical namespace prefix + scoped leaf. Persist treats this as TEXT; consumers parse against [§5](05_namespace.md)'s namespace map. |
| `score` | yes | Pos/neg scalar in [-1, +1]. Polarity is encoded by sign; magnitude carries strength. Some dimensions are boolean-via-score (±1 only); some are positive-only; some are signed; per-dimension table in [§5](05_namespace.md) names the polarity. |
| `confidence` | yes | The attester's own confidence in their score. [0, 1]. Low confidence + high magnitude = "I believe this strongly but I might be wrong"; high confidence + low magnitude = "I am sure the truth is near-neutral." |
| `context` | no | Free-form scoping detail. Not parsed by the substrate; used by consumers + audit + RATCHET. |
| `evidence_refs` | no (often required by per-dimension policy) | List of URIs / content-hashes pointing to backing evidence (Stripe receipt, licensing-body record, observed interaction, log entry, audit-chain leaf, etc.). Some dimensions in [§5](05_namespace.md) require non-empty evidence_refs. |
| `valid_until` | no | ISO 8601 datetime per [§0.5](00_conformance.md). If set, consumer policy treats the attestation as stale after that point (independent of the substrate row's own `expires_at`). |
| `epistemic_mode` | no | Per [§2](02_grammar.md) Epistemic-mode axis; default `direct`. Consumers may weight by mode (e.g., direct witness > hearsay). |
| `witness_relation` | no | `self` \| `external` \| `derived`. Names the attester's relation to the attested fact: `self` = attester is the attested entity (self-attestation); `external` = attester observed independently; `derived` = attester inferred from other attestations or signed traces. Default `external`. Consumers weight by relation to prevent self-attestation gaming. Complements `epistemic_mode` (which names HOW the claim was formed) — `witness_relation` names WHO the attester is in relation to the attested entity. |
| `oversight_mode` | no | `HITL` \| `HOTL` \| `HOOTL`. Names the human-control gradient under which the attestation was produced. Default `null` (legacy contributions; consumer policy applies a per-cell default). Mode shifts are themselves attestable as `accountability:mode_shift:{from}:{to}` Contributions. |
| `occurrence_id` | no | Identifies which occurrence of a multi-occurrence agent deployment emitted this attestation. Format: `"occurrence-{n}"` per the agent's `AGENT_OCCURRENCE_ID` env var, or `"__shared__"` for shared-task pattern emissions. Default `null` → treated as `occurrence-0` for backward compat. **Self-asserted**: this field is NOT cryptographically bound to a fleet-attestation primitive in 0.x; an adversary running a single key can claim any occurrence_id. Acknowledged design tradeoff per [§15.2](15_gaps.md). |
| `occurrence_count` | no | Total occurrences in the deployment fleet emitting the attestation; integer ≥ 1. Default `null` → `1` (single-occurrence). Same self-assertion caveat as `occurrence_id`. |
| `occurrence_role` | no | `primary` \| `shared` \| `replica`. Names the occurrence's role within the fleet. Default `null` → `primary` for backward compat. Substrate-self-report attestations (`system:*` prefixes per [§5.3 + §5.4](05_namespace.md)) SHOULD carry occurrence_id + occurrence_count + occurrence_role so post-facto compliance reviewers can reconstruct "which occurrence agreed to which mandate." |
| `stake` | no | Per [§2](02_grammar.md) Stake axis; default `reputational`. Composes with the attester's actual stake-backed-by attestations from [§5.9](05_namespace.md). |

**`epistemic_mode` vs `witness_relation` — distinct dimensions**: these co-vary at edges but name different concerns. `epistemic_mode` names the *process* by which the claim was formed; `witness_relation` names the *relational position* of the attester to the attested. F-3 detector attestations carry both (`epistemic_mode: derivative` + `witness_relation: derived`). Most encyclical-sourced translations are `witness_relation: external` + `epistemic_mode: hearsay`. When in doubt, set both.

## §4.1 Forward-compatibility rule

A Conforming Consumer (CCC per [§0.2](00_conformance.md)) that receives an envelope carrying a field-name it does not recognize MUST:

- Preserve the unknown field on read (do not strip).
- Preserve it on re-emission if the Consumer is also acting as a Producer relaying the attestation.
- NOT use it in verdict composition.
- NOT reject the envelope on the basis of the unknown field alone.

Producers introducing a new envelope field MUST follow the [§0.3](00_conformance.md) versioning rules: a new field with a documented default is a MINOR bump; a field whose absence breaks consumer semantics is a MAJOR bump.

---

[← §3 Primitives](03_primitives.md) | **§4 Envelope** | [Next: §5 Namespace →](05_namespace.md)
