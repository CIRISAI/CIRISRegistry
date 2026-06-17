
---

# §7 Reserved-prefix enforcement

Most of the namespace is open-vocabulary. A small number of prefixes are reserved — only specific identity types may emit them. **Enforcement is normative at the substrate verify-pipeline AND at every CEG-Conforming Consumer per [§0.2](00_conformance.md)**.

## §7.0 The enforcement rule (normative)

A CEG-Conforming Substrate (CCS) MUST reject any incoming `scores` attestation whose `dimension` matches a reserved-prefix pattern below AND whose `attesting_key_id` does not satisfy the prefix's emitter rule. Rejection is at admission to `federation_attestations`; rejected rows are not stored.

A CEG-Conforming Consumer (CCC) MUST independently re-check the reserved-prefix rule on every received attestation regardless of whether it was previously admitted by another peer's substrate. Trust does not propagate: the substrate's admission check is the FIRST line of defense; the consumer's re-check is the second. Both checks MUST agree.

A CEG-Conforming Producer (CCP) MUST NOT emit an attestation under a reserved prefix unless its `attesting_key_id` satisfies the emitter rule. Violation is a producer-side conformance violation regardless of whether any downstream substrate accepts the violation.

## §7.0.1 `identity_type` is a set — single-key role cohabitation (CEG 0.9 addition)

Per [CIRISRegistry#49](https://github.com/CIRISAI/CIRISRegistry/issues/49) + [CIRISAgent#856](https://github.com/CIRISAI/CIRISAgent/issues/856) + [§11.9](11_governance.md). **`federation_keys.identity_type` is a SET of roles, not a single scalar role.** A single federation key MAY simultaneously hold multiple identity types — e.g., a folded CIRISAgent occurrence whose key is BOTH `agent` AND `lenscore_detector`, or a steward key that is both `substrate_persist` and `witness`.

Every emitter rule in this section — and the [§9.1](09_humanity_accord.md) `accord_holder` material — is therefore evaluated by **set membership**, not scalar equality:

> Wherever a §7 emitter rule reads "`attesting_key_id` MUST match a `federation_keys` row with `identity_type=X`" (or "`identity_type="X"`"), the normative reading is **`X ∈ attesting_key.identity_type`** — the key's role-set MUST CONTAIN `X`. A key satisfies a reserved-prefix gate iff the required role is one of its held roles.

**Backward compatibility (the wire-break is semantic-null for legacy keys).** A pre-0.9 scalar `identity_type="X"` is canonically the singleton set `{X}`. For a single-role key the membership test `X ∈ {X}` is identical to the scalar test `X == X`, so every pre-0.9 gating decision is unchanged. CEG 0.9 carries a wire-break ONLY in the field's representation (scalar → set) at the `federation_keys` row layer — the second wire-break in the 0.x series after [§0.2 attestation-ladder rename](16_references.md). Substrate implementations MUST migrate the column to a set/array representation; consumers MUST read a legacy scalar as a one-element set. No envelope field, structural primitive, subject_kind, or [§5](05_namespace.md) dimension prefix changes — the 1+4 wire-format lockdown is untouched; this is a [§7](#7-reserved-prefix-enforcement)-layer enforcement-rule generalization only.

**Canonical-bytes encoding.** Where `identity_type` enters a canonical-bytes computation (e.g., cross-attestation of a `federation_keys` row), the set MUST be encoded as its members sorted ascending by Unicode code point, deduplicated, comma-joined with no whitespace (e.g., `agent,lenscore_detector`). A single-role key encodes identically to its pre-0.9 scalar form (`agent` ≡ `{agent}` ≡ `"agent"`), preserving signatures over legacy single-role rows.

**Storage representation is an implementation choice — "set" is semantic, not a column type (normative confirmation, 1.0-RC5; resolves [CIRISRegistry#78](https://github.com/CIRISAI/CIRISRegistry/issues/78) caveat 1 / [CIRISPersist#183](https://github.com/CIRISAI/CIRISPersist/issues/183)).** §7.0.1 requires that `identity_type` be *interpreted* as a set (membership test, not scalar equality) and that its *canonical bytes* be the sorted-deduped-comma-joined string above. It does **NOT** mandate a structured/array **column**. A single free-form `TEXT` column holding the comma-joined form, decoded-to-set on read (persist's v6.5.0 shape), **is conformant** — the comma-joined string *is* the canonical representation, and the set is its interpretation. **No substrate migration to a structured column is required.** New `identity_type` values (e.g. `user`, `wise_authority`) are valid additive members of the open role vocabulary; they need no schema change, only inclusion in the membership-test set.

**Cohabitation does NOT collapse the dimension split.** Role cohabitation grants a key the *right* to emit under each held role's reserved prefixes; it does NOT merge the roles' namespaces. A key holding `{agent, lenscore_detector}` still emits its detector verdicts under `detection:*` (the lenscore-role surface) and its agent-intent attestations under the agent dimensions — the [§7.4](#74-detector-only-prefixes) shadowing rule and the [§7.5](#75-capacity-score-self-emission-rejection) self-emission rejection apply unchanged per held role. See [§7.4](#74-detector-only-prefixes) for the LensCore-fold worked example.

**Co-location is NOT consolidation — the fabric-node discipline (normative, 1.0-RC3).** A *fabric node* (the headless cohabitation runtime that composes registry-authority + lens-observation + node-consensus over one substrate; `agent = fabric node + brain`) routinely holds the **full role-set in one key/process** — `{substrate_persist, steward, lenscore_detector, witness, …}`. This is co-location of **custody**, not consolidation of **authority**, and the separation of powers is held **cryptographically, not procedurally**:

- **Authority stays quorum-bound.** A co-located `steward` role does NOT let a single node issue a federation-scope attestation. Registry-consensus is the [§8.1.13.1.1(a)](08_composition.md) **founder-quorum** over the `ciris-canonical` infrastructure community ([§5.6.8.10](05_namespace.md)), evaluated over `{m : m.role == founder}`. A co-located node gains a *vote*, never a *verdict*.
- **Observation stays non-authoritative by namespace.** `lenscore_detector` emissions live under `detection:*` and are **validated, not adjudicated** ([§1.4](01_foundation.md)) — never sole evidence for an authority action ([§7.4](#74-detector-only-prefixes) / [§1.3.1 T4](01_foundation.md)).
- **Observation can never manufacture authority.** Holding both roles cannot make a `detection:*` emission an authority verdict: the namespaces do not merge (above), and authority is quorum-gated *upstream of any single key*. The hazard the LensCore mission warns of — *the lenses becoming the gate* — is structurally unreachable inside one process.

A fabric node co-locating authority + observation + consensus is conformant **iff** these hold. An implementation that lets a co-located node convert what it *observes* into what it can *authorize* has broken the separation at that point and is **non-conformant** — fix the wiring, never weaken the rule.

## §7.0.2 `consent_role` — the Counter-RII consent gate (1.0-RC4, ratifies Accord §RC / CIRISAgent#760 OQ-1/2/3)

`federation_keys.consent_role` is the role enum that gates **Counter-RII** probe detection (RATCHET `FSD/COUNTER_RII_DETECTION.md`; Lean `ConsentGate.lean`, 8 theorems verified — F-CR-3 SelfConscience-zero-by-construction proved). Three primitive-level semantics shape persist's `federation_keys.consent_role` schema and edge's `ProbePatternObserver` gate and so **cannot be set per-consumer** — they are **ratified here** (the Accord §RC slot CEG 1.0-RC3 reserved is now filled; [CIRISAgent#760](https://github.com/CIRISAI/CIRISAgent/issues/760) OQ gate closed). All three take the `ConsentGate.lean` default — ratification carries **no predicate or proof change**.

**OQ-1 — revocation chain: `BaseRole`-only, non-recursive.** A `consent_role` is non-recursive: a subsequent revocation **overwrites** the prior revocation record (NO recursive revocation chain embedded in the role). Chain history, **if retained**, MUST live in a separate audit surface and **MUST NOT be embedded in the `consent_role` JSONB**. This locks the substrate-portable JSONB shape — flat, bounded, overwrite-on-revoke — consistent with [§5.6.8.5](05_namespace.md) stable-id grouping (not chain-walk; partition-tolerance) and [§13.5](13_anti_patterns.md) (no key pre-declaring its own state recursively), and **non-breaking against the shipped flat soft-delete substrate** (the permissive "if retained" is deliberate — mandating an audit table would contradict a deployed migration).

**OQ-2 — peer eligibility: blanket suppression.** A node holding the `Peer` `consent_role` escapes Counter-RII detection at **any** `trust_mode`. The cost — a sovereign peer may probe other peers without raising the signal — is **bounded by construction**: `ratchet:flag:counter_rii:{layer}` is **advisory only** ([§5.7](05_namespace.md)) — it can NEVER be sole evidence for `slashing:*`; the WA quorum is the load-bearing adjudication gate. The exemption suppresses an *advisory signal*, not an *enforcement path*. (The [§8.1.12.7](08_composition.md) ↔ CEG-native-agent dual-identity interaction was ruled at [CIRISRegistry#51](https://github.com/CIRISAI/CIRISRegistry/issues/51) ruling 3.)

**OQ-3 — post-window `AuthorizedReview`: strict.** An `AuthorizedReview` `consent_role` is signal-eligible **immediately** at `t > window_end` — no grace period. Matches the fail-secure / clean-state-machine grain ([§15.6](15_gaps.md)); reviewers MUST respect their windows.

With this, `consent_role` is **no longer a reserved-not-yet-written slot** — implementations MAY now build the `consent_role` substrate (the [CIRISPersist#146](https://github.com/CIRISAI/CIRISPersist/issues/146) consent-SLA watcher + schema; the CIRISEdge consent-gate). **1+4 untouched** — `consent_role` is a `federation_keys` identity field (sibling to [§7.0.1](#701-identity_type-is-a-set--single-key-role-cohabitation-ceg-09-addition) `identity_type`), not an envelope primitive.

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

`detection:correlated_action:*` and `detection:distributive:access:*` are LensCore-only emission. Emitter rule: `lenscore_detector ∈ attesting_key.identity_type` (per [§7.0.1](#701-identity_type-is-a-set--single-key-role-cohabitation-ceg-09-addition) set-membership reading; equivalently for a legacy single-role key, `identity_type="lenscore_detector"`). Cross-attestation by non-LensCore peers (on the same dimension, attesting to the same subject) is admitted as a score on the detector's verdict — useful when the federation wants to cross-check — but those scores MUST use a different `dimension` prefix (e.g., `truth_grounding:detection:correlated_action:{axis}`) to avoid shadowing the detector's own emission.

**LensCore-fold worked example (CEG 0.9; per [CIRISAgent#856](https://github.com/CIRISAI/CIRISAgent/issues/856)).** When `ciris-lens-core` is folded into the CIRISAgent workspace, a single folded occurrence's federation key holds `identity_type ⊇ {agent, lenscore_detector}`. The §7.4 gate is satisfied for that key's `detection:*` emissions by `lenscore_detector ∈ {agent, lenscore_detector}` — the cohabiting `agent` role neither grants nor blocks the detector right; only the held `lenscore_detector` role does. The split is preserved by the dimension namespace, not by the key: the same key emits its **agent-intent** attestations under the agent dimensions and its **detector verdicts** under `detection:*`; a downstream consumer cross-checking the detector still emits under the distinct `truth_grounding:detection:*` prefix above, shadowing-free. The [§7.5](#75-capacity-score-self-emission-rejection) self-emission rejection continues to bind per held role — a folded `agent`+`lenscore_detector` key still MUST NOT emit a `capacity:*` score about itself.

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
| `hard_case:recipient_excluded:{scope_key_id}` | Substrate fail-secure-skips a recipient in the [§10.1.4](10_endpoints.md) at-rest grant cascade (1.0-RC1, [#71](https://github.com/CIRISAI/CIRISRegistry/issues/71) C3). Payload: excluded recipient `key_id`, `reason ∈ {expired_occurrence, invalid_kem_key, missing_encryption_pubkeys}`, skipped Contribution's envelope ref. **Cohort-scoped: emitted INTO the affected self/family scope; MUST NOT federate beyond it** — the excluded member can audit; the federation learns nothing (§10.1.4 invisibility preserved). | Same: substrate_persist |

**Removal-path emission shape (normative, 1.0-RC5 — pins [CIRISPersist#161](https://github.com/CIRISAI/CIRISPersist/issues/161) Ask 5).** The membership-change prefix covers **both add and removal** — there is **no separate `hard_case:member_removed` kind** (it would split one event class across two prefixes for no gain). On the **removal** path the substrate emits the *same* `hard_case:family_membership_change:{family_key_id}` (and the §7.8 `community_membership_change` analog), with the payload distinguishing the direction and carrying what the forward-secrecy + audit consumers need:

```
hard_case:family_membership_change:{family_key_id}  (payload)
    change_kind:    "added" | "removed"          // the direction (NEW — pins the removal signal)
    subject_key_id: key_id                        // the member added/removed
    cohort_key_id:  key_id                        // the family (or community) key
    effective_at:   rfc3339_canonical             // the membership-change instant; on removal this is
                                                  //   the re-key epoch boundary (§8.1.12.5 / §8.1.13.4
                                                  //   Option-A) after which the removed member receives
                                                  //   no new wrapped content
```

So the removal-time substrate signal (the thing persist's `put_family_membership_revocation` path emits) is `change_kind: "removed"` on the existing prefix — consumers and the forward-secrecy re-key key on `effective_at`. Same shape for `community_membership_change` (§7.8) and the `identity_occurrence` removal (a `withdraws` against the occurrence; the substrate emits `family_membership_change` / `community_membership_change` where the occurrence was a member).

Composes with [§7.2](#72-substrate-self-report-reservations-system) — these are part of the same substrate-self-report discipline. Non-substrate emissions on these prefixes are a category error and MUST be rejected.

## §7.8 Community + location-event reservations (CEG 0.8 addition)

Per [§5.6.8.10](05_namespace.md) `community` + [§5.6.8.11](05_namespace.md) `location_proof` subject_kinds. Four substrate-emitted prefixes:

| Prefix | Emitted on | Emitter rule |
|---|---|---|
| `hard_case:community_membership_change:{community_key_id}` | Substrate admits an addition or removal in the named community's roster (per the community's `consensus_protocol`; for `cohort_subkind: geographic` admission additionally requires valid `location_proof`) | `attesting_key_id` MUST match `federation_keys` row with `identity_type="substrate_persist"` |
| `hard_case:community_consensus_protocol_change:{community_key_id}` | Substrate admits a `consensus_protocol` amendment on a non-entrenched community | Same: substrate_persist |
| `hard_case:community_consensus_protocol_violation:{community_key_id}` | Substrate REJECTS a proposed community amendment (rule unsatisfied OR entrenched) | Same: substrate_persist |
| `hard_case:location_proof_resolution_violation` | Substrate REJECTS a `location_proof` Contribution with `cell_resolution > 7` per [§0.8.1](00_conformance.md) rough-only enforcement; emitted against the producer's `key_id` so operators can observe malformed-client patterns | Same: substrate_persist |

Composes with [§7.2](#72-substrate-self-report-reservations-system) + [§7.7](#77-selffamily-membership-event-reservations-ceg-07-addition) — part of the same substrate-self-report discipline. Non-substrate emissions on these prefixes are a category error and MUST be rejected.

## §7.9 Delivery-receipt reservation (CEG 0.10 addition)

Per [§10.5.4](10_endpoints.md) delivery receipts (V3 lock). One reserved prefix for subscriber-emitted delivery acknowledgements:

| Prefix | Description | Emitter rule |
|---|---|---|
| `delivery_receipt:{stream_id}` | Subscriber's signed acknowledgement that they received chunk K under the named stream + epoch. Best-effort default; opt-in for accountable streams. Validated-not-adjudicated per [§1.4](01_foundation.md) MISSION fail-honest invariant — substrate / Verify authenticate origin + JOIN against published STH root per [§10.5.4](10_endpoints.md), but do not compose "delivered"/"owes N" verdicts (consumer policy). | `attesting_key_id` MUST be a current member of the community/stream the `{stream_id}` belongs to, per [§8.1.13](08_composition.md) Policy M membership resolution. NOT substrate-self-report (distinct from §7.2 / §7.7 / §7.8 substrate emissions). |

**Composition with CEG 0.9 [§7.0.1](#70-the-enforcement-rule-normative) identity_type-as-set**: a subscriber who is also a witness MAY emit delivery_receipt under the subscriber role; the role-set must contain a subscriber-eligible role (per the community's admission semantics from [§8.1.13](08_composition.md) Policy M).


