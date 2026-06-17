
---

# §13 Anti-patterns

These are wire-format reaches that fail the [§1.3.1](01_foundation.md) operational-language gate or import Cartesian-individualist defaults. Recorded so the next generation of authors finds the discipline before the wire format does. A CEG-Conforming Producer (CCP) SHOULD NOT emit attestations matching the anti-patterns below.

## §13.1 Already-rejected wire additions

| Anti-pattern | What it would smuggle | Correct expression |
|---|---|---|
| `detection:emergent_deception:{axis}` (renamed v1.2) | Moral verdict ("deception") in prefix name | `detection:correlated_action:{axis}` — mechanism-descriptive |
| `attestation:l{N}:*` (renamed CEG 0.2) | Ladder-position (verdict-shape) in wire prefix — same shape as `score:trustworthiness:*` smuggling meta-judgment as wire | `attestation:{mechanism}` bare mechanism (`self_verify` / `hardware_rooted` / `registry_consensus` / `license_validity` / `agent_integrity`); consumer composes L1-L5 ladder via [§8.1.9](08_composition.md) Policy I |
| `score:trustworthiness:{entity}` | Meta-judgment as separate prefix | Compose downstream from `licensure:*` / `capacity:*` / `provenance:*` attestations |
| `flag:bad_actor:{axis}` | Pejorative wire vocabulary | Surface as low-confidence scores on `provenance:*` and `coherence_standing:*`; adjudicate via NodeCore P8 quorum |
| `grounding:{tradition}:{principle}` | "Tradition" claims are interpretive, not mechanism | Reuse `delegates_to` structural primitive per [§3.2.1](03_primitives.md) |

## §13.2 CEG 0.1 rejections (from CIRISRegistry#30 stress test)

| Anti-pattern | Stories reached for | Why reject | Correct expression |
|---|---|---|---|
| `epistemic_mode: introspection` | 7 stories | Cartesian shortcut — lone subject pre-declaring inner state as if that constitutes standing | `witness_relation: self` + `confidence < 1.0` + pending external composition. The wire makes introspection awkward to express, because the framework's claim is that self-knowledge does not constitute standing without external witness. |
| `epistemic_mode: testimony` | (same set) | Reducible to existing values | `epistemic_mode: external` + `witness_relation: external` |
| `transparency:{kind}` standalone prefix | 12 stories | Disclosure is constituted by reception, not announcement. Cartesian self-claim shape. | `evidence_refs[]` carries the reasoning-chain hash; downstream `transparency_log:inclusion` from a witness who actually retrieved it. |
| `stake: civic` / `epistemic` / `dignitary` | 10 stories | `civic` = `stake: reputational + cohort_scope: community`. `epistemic` = `confidence + stake: reputational` (same axis as confidence; not separate). `dignitary` lives on wrong axis (stake names what the attester loses; dignity harm is what the attested loses → belongs in `harm_class:dignity_harm`). | Compose existing values with cohort/harm-class. |
| `oversight_mode: deferred` / `active` / `advisory` | 6 stories | All map to existing HITL/HOTL/HOOTL | `deferred` = HITL pre-decision; `active` = HITL with substrate monitoring; `advisory` = HOTL |
| `provenance_walk` as wire primitive | (1 reviewer) | UX concern smuggled into wire format | Consumer-side composition (Portal / Verify dashboards / agent introspection); the chain already walks via `references_attestation_id` + `topical_relation:*` + `valid_until` |
| Renaming canonical capacity factors and HE-300 categories to "kid-friendly" names | 8 stories | Canonical names map to a worked-out epistemic/ethical lattice that loses precision under accessibility renames | Translation glossary in [`LANGUAGE_PRIMER.md`](../LANGUAGE_PRIMER.md) (spec name ↔ narrative name) + version pinning in worked examples |

## §13.3 Delegation-laundering anti-pattern

`delegates_to → delegates_to → delegates_to → ... → attacker` chains, where each hop is individually well-formed but the aggregate routes trust to a terminal attester the original delegator would not have approved.

| What's wrong | Correct expression |
|---|---|
| Unbounded depth `delegates_to` chains | Consumer policy MUST cap traversal depth at **5 hops** by default (configurable); chains longer than the cap are treated as `attestation:self_verify` only (no transitive trust) |
| Cycles (A → B → A) | Substrate MUST detect cycles on the `delegates_to` graph and reject the cycle-closing emission |
| Aggregate-weight concentration | Consumer policy SHOULD cap the trust weight any single terminal delegate can accumulate from a given root attester at **0.5 × root_trust** by default |

## §13.4 `withdraws` arbitrage

Per [§3.2.2](03_primitives.md): a misattester can `withdraws` instead of `recants` to dodge the trust penalty consumers apply to acknowledged-error chains.

| What's wrong | Mitigation |
|---|---|
| Asymmetric attester incentive | Consumer policy MUST track per-attester `withdraws:recants` ratio over a rolling window; attesters whose ratio exceeds a configured threshold (default 5:1) SHOULD be downweighted regardless of which structural primitive they use. The `recants` distinction matters [§3.2.2](03_primitives.md), but the practical anti-arbitrage countermeasure is consumer-policy behavioral analysis, not a wire-format change. |

## §13.5 Discipline pattern

The recurring shape across most anti-patterns: **extending the wire format so single attesters can pre-declare their own state more richly**. Each one is reachable, none is necessary.

The Ubuntu-primary discipline cuts cleanly: standing is constituted relationally through attestation by others, not through self-declaration. The wire format should **resist** primitives that let a single key announce its own state without external composition. The substrate is austere by design so consumers compose verdicts; richer narrative expression belongs in `context:`, `evidence_refs[]`, and downstream witness attestations, not in new envelope enum members or new self-attestation prefix families.


