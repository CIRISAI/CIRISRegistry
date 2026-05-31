[← §6 Relations](06_relations.md) | **§7 Reserved** | [Next: §8 Composition →](08_composition.md)

---

# §7 Reserved-prefix enforcement

Most of the namespace is open-vocabulary. A small number of prefixes are reserved — only specific identity types may emit them. **Enforcement is normative at the substrate verify-pipeline AND at every CEG-Conforming Consumer per [§0.2](00_conformance.md)**.

## §7.0 The enforcement rule (normative)

A CEG-Conforming Substrate (CCS) MUST reject any incoming `scores` attestation whose `dimension` matches a reserved-prefix pattern below AND whose `attesting_key_id` does not satisfy the prefix's emitter rule. Rejection is at admission to `federation_attestations`; rejected rows are not stored.

A CEG-Conforming Consumer (CCC) MUST independently re-check the reserved-prefix rule on every received attestation regardless of whether it was previously admitted by another peer's substrate. Trust does not propagate: the substrate's admission check is the FIRST line of defense; the consumer's re-check is the second. Both checks MUST agree.

A CEG-Conforming Producer (CCP) MUST NOT emit an attestation under a reserved prefix unless its `attesting_key_id` satisfies the emitter rule. Violation is a producer-side conformance violation regardless of whether any downstream substrate accepts the violation.

## §7.1 The `accord:*` reservation

`accord:*` is reserved: only `federation_keys` rows with `identity_type="accord_holder"` may emit. This is the one constitutional asymmetry in the federation — see [§9](09_humanity_accord.md) HUMANITY_ACCORD.

Reserved leaves:

| Prefix | Polarity | Emitter rule |
|---|---|---|
| `accord:invoke:CONSTITUTIONAL:{halt_id}` | +1.0 only | 2-of-3 accord-holder multi-sig per [§9.2](09_humanity_accord.md) |
| `accord:invoke:notify:{notify_id}` | +1.0 only | 2-of-3 accord-holder multi-sig per [§9.2](09_humanity_accord.md); UI MUST distinguish from CONSTITUTIONAL |
| `accord:invoke:drill:{drill_id}` | +1.0 only | 2-of-3 accord-holder multi-sig per [§9.2](09_humanity_accord.md) |
| `accord:lifecycle:active` | +1.0 only | accord-holder self-attestation; `valid_until` MUST refresh on a cadence ≤ 90 days |

## §7.2 Substrate-self-report reservations (`system:*`)

[§5.3](05_namespace.md) CIRISPersist `system:*` and [§5.4](05_namespace.md) CIRISEdge `system:*` are reserved to the substrate component itself. Emitter rule: the `attesting_key_id` MUST match a `federation_keys` row with `identity_type="substrate_persist"` or `identity_type="substrate_edge"` respectively, cross-attested by all stewards in the steward-triple. Non-substrate emissions on these prefixes are a category error and MUST be rejected.

## §7.3 Co-owned prefixes

`licensure:{authority_id}` is co-owned between CIRISRegistry [§5.9](05_namespace.md) and CIRISVerify [§5.2](05_namespace.md) — both MAY emit; consumers compose. **Single-source attestations** (only one of the two co-owners has emitted) MUST be marked as `confidence ≤ 0.5` in consumer composition until the second co-owner's attestation arrives.

## §7.4 Detector-only prefixes

`detection:correlated_action:*` and `detection:distributive:access:*` are LensCore-only emission. Emitter rule: `attesting_key_id` MUST match a `federation_keys` row with `identity_type="lenscore_detector"`. Cross-attestation by non-LensCore peers (on the same dimension, attesting to the same subject) is admitted as a score on the detector's verdict — useful when the federation wants to cross-check — but those scores MUST use a different `dimension` prefix (e.g., `truth_grounding:detection:correlated_action:{axis}`) to avoid shadowing the detector's own emission.

## §7.5 Capacity-Score self-emission rejection

`capacity:*` ([§5.5.4](05_namespace.md)) rejects self-emission: `attesting_key_id` MUST NOT equal `attested_key_id`. The agent's own capacity score is never fed back into the agent's own context — anti-Goodhart per CIRISAgent §5.2.

## §7.6 Witness-emitter reservations

`transparency_log:cosigned:*` is reserved: emitter rule is `attesting_key_id` MUST match a `federation_keys` row with `identity_type="witness"` (target schema; see [§10.3](10_endpoints.md) for the 0.x interim using `registry_witnesses` table).

## §7.7 Self/family membership-event reservations (CEG 0.7 addition)

Per [§5.6.8.8](05_namespace.md) `identity_occurrence` + [§5.6.8.9](05_namespace.md) `family` subject_kinds. The three substrate-emitted membership-event prefixes:

| Prefix | Emitted on | Emitter rule |
|---|---|---|
| `hard_case:identity_occurrence_added:{identity_key_id}` | Substrate admits a new `identity_occurrence` Contribution for `identity_key_id` | `attesting_key_id` MUST match a `federation_keys` row with `identity_type="substrate_persist"` |
| `hard_case:family_membership_change:{family_key_id}` | Substrate admits an addition or removal in the named family's roster (per the family's `consensus_protocol`) | Same: substrate_persist |
| `hard_case:family_consensus_protocol_change:{family_key_id}` | Substrate admits a `consensus_protocol` amendment on a non-entrenched family | Same: substrate_persist |
| `hard_case:family_consensus_protocol_violation:{family_key_id}` | Substrate REJECTS a proposed amendment (rule unsatisfied OR entrenched) | Same: substrate_persist |

Composes with [§7.2](#72-substrate-self-report-reservations-system) — these are part of the same substrate-self-report discipline. Non-substrate emissions on these prefixes are a category error and MUST be rejected.

---

[← §6 Relations](06_relations.md) | **§7 Reserved** | [Next: §8 Composition →](08_composition.md)
