[← §7 Reserved](07_reserved.md) | **§8 Composition** | [Next: §9 HUMANITY_ACCORD →](09_humanity_accord.md)

---

# §8 Composition policies

The substrate carries edges (attestations); consumers compose traversals (verdicts). CEG specifies a library of named reference policies. A CEG-Conforming Consumer (CCC per [§0.2](00_conformance.md)) MUST implement at least Policy A; the others are RECOMMENDED for richer compositions.

## §8.1 Reference policies

### §8.1.1 Policy A — direct trust

Consumer trusts an attestation if `attesting_key_id` is in the consumer's pinned trust set (canonical bootstraps + consumer-added pins). Cheapest, lowest-latency, narrowest reach.

Aggregation: per (`dimension`, `attested_key_id`) tuple, mean of `score × confidence` from trusted attesters. Consumer threshold determines verdict.

**Recommended default**: Policy A with `pinned_trust = {us-steward, eu-steward, apac-steward, accord_holder_1, accord_holder_2, accord_holder_3}`. Cold-start bootstrap: a new consumer obtains the pinned trust set by fetching `GET /v1/steward-key` + `GET /v1/accord-holders` ([§10.2](10_endpoints.md)), verifying the responses' hybrid signatures against TLS pubkey pinning (consumer-side TOFU or out-of-band distribution), and persisting locally.

### §8.1.2 Policy B — one-hop transitive

Consumer trusts an attestation if `attesting_key_id` has been vouched for by the pinned trust set. Adds one hop of indirection.

### §8.1.3 Policy C — weighted graph (EigenTrust-style)

Consumer applies transitive-trust propagation across the full attestation graph, weighted by canonical-bootstrap distance with confidence decay per hop. Requires more compute; less common in practice; needed for federated reputation across many partner orgs.

### §8.1.4 Policy D — Lexical-vulnerability-priority

A composition tie-breaking rule layered on top of any base policy. When two otherwise-equivalent attestations conflict, defer to whichever attestation names the more-affected cohort — measured by `affected_population_estimate` in the attestation `context`, weighted inversely (smaller = more vulnerable, more weight).

Inverts the default popularity-weighted aggregation specifically for ties. Consumer policy, NOT a wire-format primitive (per [§1.3.1](01_foundation.md) — priority ordering is composition, not measurement).

### §8.1.5 Policy E — Locality-scaled quorum

Closes G3 (narrow-cell fresh-quorum-recusal infeasibility). Makes WA quorum size a function of decision locality:

```
quorum_size(scale) = f(locality:decision:{scale})

reference function (policy-tunable):
  local      → 2
  regional   → 3
  national   → 4
  federation → 6

minimum cell-pool requirement for fresh-quorum recusal at scale S:
  min_pool(S) = quorum_size(S) × 2
```

Recusal is feasible when `cell_pool ≥ min_pool(S)`. Decision-scale-matching is structurally enforced; overreach surfaces as a named "locality mismatch" failure.

#### §8.1.5.1 Sub-quorum fallback (0.1 scaffold; addresses CEG 0.1 distsys review)

When `cell_pool < min_pool(S)`, the consumer MUST take one of these explicit paths — there is no implicit fallback:

1. **Scale-down**: re-attest the decision at the next-lower `locality:decision:{scale}` (where the smaller quorum requirement is met) AND emit `hard_case:locality_scale_down` so the scaling event is observable for downstream review.
2. **Escalate**: emit `hard_case:locality_underpopulated` and route the decision to the federation-scale cell (which by definition has the largest pool).
3. **Liveness-defer**: emit `hard_case:locality_quorum_unreachable` and defer the decision until the cell pool grows. The deferred state MUST itself be reviewable via subsequent reconsideration.

Recursion safety: the [§11.2](11_governance.md) amendment process routes through `locality:decision:federation` by default; the federation cell's pool is sized to make `cell_pool ≥ min_pool(federation)` always true at federation genesis. If federation-scale pool ever falls below `min_pool(federation)`, the entire amendment surface is in a constitutional crisis state and only the HUMANITY_ACCORD CONSTITUTIONAL halt ([§9](09_humanity_accord.md)) can resolve it.

### §8.1.6 Policy F — `agent_files` trust composition

Three-layer consumer policy for composing trust over `agent_files:*` attestations ([§5.6.7](05_namespace.md) + [§5.9](05_namespace.md)).

**Layer 1 — Canonical (default trust)**: an `agent_files:*` attestation with `score ≥ 0.7` from a `registry-steward-triple` key constitutes the CIRIS canonical default-trust state. The install endpoint at `registry.ciris-services-1.ai/install` resolves canonical files via this rule.

**Layer 2 — Open Contribution**: any federation-key holder may emit `agent_files:*` attestations. The wire format admits them; consumer policy decides whether to surface them. The "Browse alternatives" view shows third-party agent_files with explicit provenance disclosure.

**Layer 3 — Vote-then-trust**: a non-canonical `agent_files:*` attestation accumulates NodeCore P4 votes. Consumer policy may elevate trust once an accumulated-weight threshold is reached.

**Anti-tricking guarantee**: the canonical-default Layer 1 rule applies at the install endpoint **regardless of attester or vote accumulation**. Third-party agent_files are reachable only via the explicit "Browse alternatives" path. This binds CIRIS L3C: the federation cannot exempt itself from the rule that newcomers' default trust path is the steward-attested canonical one.

### §8.1.7 Policy G — Trust-Fresh / Lighthouse

Composition pattern recognized in CIRISRegistry#30 stories — `cert_validity:{authority} + transparency_log:inclusion + (attestation:registry_consensus OR attestation:license_validity)` recurred organically across ~20 substrate stories as the "freshness + attested + verified" idiom.

Reads as: the consumer wants confirmation that (a) the cert chain is currently valid; (b) the attestation appears in a transparency log; (c) either `attestation:registry_consensus` (the ladder's L3 position per §8.1.9 Policy I) or `attestation:license_validity` (L4) is satisfied. The combination is the consumer-side recipe for "this attestation is fresh AND verified AND multi-source-consensus-backed."

Not a wire primitive; a recognized composition pattern that consumer libraries SHOULD expose as a named one-call helper.

### §8.1.8 Policy H — Tiered-Scope Composition (CEG 0.1 addition; LIVE)

Per CIRISNodeCore commit b1582cb three-tier interface model. Three feed-shape composition idioms that read attestations by `cohort_scope`:

| Feed | `cohort_scope` filter | Trust composition |
|---|---|---|
| **local_feed** | `self` only; owner-filtered | self-attested only; no peer weighting; consumer's own attestation graph subset |
| **community_feed** | `{family, community, affiliations}` | cohort-weighted; expertise WITHIN cohort matters; cross-cohort attestations downweighted unless explicitly invited |
| **global_feed** | `{species, biosphere, federation}` | full federation expertise weighting; fact-checkers (`encyclopedia:*` editors, `news:*` fact-check attestations) carry weight; [§8.3](#83-frickerian-discipline--consumer-policy-norms) Frickerian discipline applied |

**Composition with [§5.6.8](05_namespace.md) sub_kinds**: each NodeCore `external_content` sub_kind (`encyclopedia_article`, `news_article`, `accord_data`, `local_data`) composes naturally across these tiers because all four use the same envelope shape. A `local_data` Contribution starts at `cohort_scope: self` and SHOULD only appear in `local_feed`; promotion to community/global widens `cohort_scope` via the `supersedes` structural primitive (see §8.1.8.1 below).

#### §8.1.8.1 Promotion via `supersedes` (worked pattern)

A Contribution's `cohort_scope` MAY be widened (promoted) by emitting a `supersedes` against the prior attestation with:

- `references_attestation_id` = the prior attestation's id
- `differs_in: ["cohort_scope", "sub_kind?"]` — naming what changed
- new attestation envelope reuses the prior `content_sha256` (no body re-upload)
- new `cohort_scope` is wider (e.g., `self` → `community` or `community` → `global`)
- optionally morph `sub_kind` (e.g., `local_data` → `encyclopedia_article` for "promote my private note to a published encyclopedia entry")

This pattern is wire-format-clean: re-uses the structural primitive `supersedes` rather than introducing a `promote` primitive. The chain is walkable via `references_attestation_id` so the promotion lineage is preserved.

### §8.1.9 Policy I — Attestation-Ladder Composition (CEG 0.2 addition)

The familiar L1-L5 verification "ladder" (self_verify → hardware_rooted → registry_consensus → license_validity → agent_integrity) is **consumer-side composition over the mechanism prefixes** in [§5.2](05_namespace.md), not a wire-level taxonomy.

Per [§1.3.1](01_foundation.md) T2 honestly applied: the L-number names a *ladder position* (a verdict-shape consumers compute), not a *mechanism* (which is what the wire MUST carry). Prefixes like `attestation:l3:registry_consensus` smuggled the verdict-shape into the prefix name, conflating the mechanism (registry consensus check) with the ladder slot (third rung). The CEG 0.2 wire-break separates them.

**Wire prefixes (mechanism)** [§5.2](05_namespace.md):

| Mechanism prefix | Ladder position (consumer-rendered) |
|---|---|
| `attestation:self_verify` | L1 |
| `attestation:hardware_rooted` | L2 |
| `attestation:registry_consensus` | L3 |
| `attestation:license_validity` | L4 |
| `attestation:agent_integrity` | L5 |

**Composition function** (reference implementation):

```
ladder_verdict(attestations) =
    let levels = []
    for prefix in [self_verify, hardware_rooted, registry_consensus,
                   license_validity, agent_integrity]:
        if any positive attestation on attestation:{prefix}:
            levels.push(prefix_to_ladder_position(prefix))
    return {
        achieved:   max(levels) if levels else None,
        ladder:     sorted(levels),
        rendering:  format_as_l1_l5_for_ui(levels)
    }
```

Consumers MAY render the ladder as `L1` / `L2` / `L3` / `L4` / `L5` for UI / dashboards / audit trails. The rendering is composition output, not wire emission.

**Why this matters**: a Verify implementation emitting `attestation:registry_consensus +1.0` is the mechanism claim. Whether that's "L3" in any particular consumer's ladder ordering is a composition concern — different consumers may order or weight the rungs differently (e.g., some safety-critical applications may require L4 *and* L5, others may treat L3 as sufficient for advisory work). The wire stays neutral; the ladder is consumer policy.

**Migration from CEG 0.1**: prior emissions of `attestation:l{N}:*` MUST be re-emitted as `attestation:{mechanism}` per the table above. Substrate-conformance migration (CIRISRegistry#17) reads-side compatibility: consumers SHOULD accept the deprecated `attestation:l{N}:*` form during the 0.1 → 0.2 transition window but MUST emit only the mechanism form going forward. The deprecated form is rejected at admission once §11.2 amendment formally retires it (target: CEG 0.3).

## §8.2 Aggregation semantics — opinionated defaults

Per dimension+attested_key_id, the verdict is computed by composing attestations under the chosen policy. Default aggregation by polarity column ([§5](05_namespace.md)):

| Polarity column value | Default aggregation |
|---|---|
| `signed` | **Mean** of `score × confidence` across attesters |
| `boolean-via-score` | **Min** (any negative trumps positive — fail-secure for hard constraints like `prohibited:*`, `attestation:l*`) |
| `+1.0 only` / `positive-only` | **Max** across attesters (any positive is conclusive) |
| `-1.0 only` | **Min** across attesters (any negative is conclusive) |
| `enumerated` | **Most-recent** by `signed_at` from the attester(s) authorized to emit per [§7](07_reserved.md) |
| Detector dimensions (`detection:correlated_action:*`, `detection:distributive:access:*`, `ratchet:flag:*`) | **Median** across attesters (resists adversarial mean-pulling by a single captured detector) |

Specific dimensions override via consumer policy; the defaults above are the [§0.2](00_conformance.md) CEG-Conforming Consumer (CCC) minimum.

## §8.3 Frickerian discipline — consumer-policy norms

Per Miranda Fricker's *epistemic injustice*: consumers SHOULD apply identity-prejudice-resistant weighting. Concretely:

- Don't downweight `testimonial_witness:*` from cohorts with low overall attestation density (testimonial preservation is precisely what corrects for that low density).
- Don't downweight `non_maleficence:*` claims about a partner just because the partner has a long `partner_role:*` track record (the long track record may be the harm).
- Apply [§8.1.4](#814-policy-d--lexical-vulnerability-priority) lexical-vulnerability-priority in tie-breaks involving small cohorts.

**Adversarial caveat**: the discipline above is consumer-policy-only; an adversary can emit `testimonial_witness:victim_of_my_competitor` exploiting the Frickerian non-downweighting rule. Per [§5.6.3](05_namespace.md), `testimonial_witness:*` is never sole evidence for `slashing:*`; per [§7.0](07_reserved.md) the consumer MUST also weight `witness_relation: self` claims against the attester's other-emission track record. The Frickerian rule applies AFTER these structural safeguards, not before them.

## §8.4 Sovereign-Registered equivalence (wire-symmetric, policy-differentiated)

A Sovereign agent scoring `licensure:CA_medical_board: +1.0` is wire-format identical to a Registry-steward scoring the same. Consumer policy weights by attester source; the substrate is source-neutral. M-1's symmetry is structural, not bolted on.

Per [`../MISSION.md`](../../MISSION.md) §1.1: both paths produce federation membership; neither is a gate. What differs is the *attestation surface* — the kind of claim the federation can compose about why a participant is trustworthy.

---

[← §7 Reserved](07_reserved.md) | **§8 Composition** | [Next: §9 HUMANITY_ACCORD →](09_humanity_accord.md)
