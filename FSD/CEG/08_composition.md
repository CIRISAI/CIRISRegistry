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

### §8.1.8 Policy H — Tiered-Scope Composition (LIVE)

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

### §8.1.9 Policy I — Attestation-Ladder Composition

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

### §8.1.10 Policy J — Trusted-Publisher composition

Composition pattern for multimedia content discovery per CIRISRegistry#37 + CIRISNodeCore FSD/MEDIA_SHARING.md. Reads as: "this `external_content` Contribution comes from a publisher whose attestation chain is trusted at the cohort level, with content-class + content-rating + age-assurance composed into the gate."

The composition has three layers (analogous to [§8.1.6](#816-policy-f--agent_files-trust-composition) Policy F for agent_files but specialized for multimedia content):

**Layer 1 — Distributor attestation chain**: an `external_content` Contribution with `sub_kind: film` (or any media sub_kind) carries a distributor attestation that chains to a federation_key with `identity_type: distributor`. Distributor identity is established via [§5.9](05_namespace.md) Registry partner_role machinery + an out-of-band distributor-onboarding flow (operator's choice — CIRIS L3C maintains a default trust set; community-run substrates maintain their own).

**Layer 2 — Content-class + content-rating composition**: consumer gates by combining `content_class:{class}` + `content_rating:{scheme}:{rating}` per §5.6.8.3:

```
gate_decision(content) = match (content.content_class, content.content_rating, consumer.preferences):
    # Producer-declared content_class is consultable but not authoritative — UI may show
    # the producer claim alongside cohort cw_class declarations and let the consumer choose.
    (class, _, prefs) if class in prefs.blocked_classes => Block
    (_, rating, prefs) if rating.exceeds(prefs.max_rating) => Block
    (_, _, _) => Allow
```

Layered with [§8.3](#83-frickerian-discipline--consumer-policy-norms) — `cw_class:*` declarations from low-attestation-density cohorts MUST NOT be downweighted; they ride alongside the gate decision as informational.

**Layer 3 — Age-assurance gating**: for content where `content_rating:*` rises above an age threshold (e.g. PEGI 18, MPAA NC-17), consumer gates via `age_assurance:{level}` per §5.6.8.3:

```
age_gate(content, consumer):
    required_level = age_required_for(content.content_rating)
    consumer_level = consumer.highest_age_assurance_level()
    return consumer_level >= required_level
```

Where the `age_assurance:{level}` ordering is: `self < provider:{verifier_key}:adult < government:{credential_class}:adult`. Consumer SHOULD accept the strongest assurance the user has provided; substrate MUST NOT issue `slashing:*` on age-assurance misdeclaration alone — `moderation:age_assurance_misdeclaration` is the adjudication path per [§5.6.4](05_namespace.md).

**Anti-tricking guarantee parallel to §8.1.6**: the canonical-distributor Layer 1 rule MUST apply regardless of vote accumulation. No amount of NodeCore P4 vote weight elevates an unverified distributor into Layer 1; the only path is the operator-set trust list. Binds CIRIS L3C: cannot exempt itself from this rule for its own content distribution.

### §8.1.11 Policy K — CEM composition

Per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842). Composition pattern for dual-authority Contributions where the subject is named via [§4.2](04_envelope.md) `subject_key_ids`, with consent state composed from the [§5.6.8.6](05_namespace.md) `consent:*` namespace family.

Reads as: "this Contribution names a subject whose consent state evolves over time; consumer policy resolves the effective consent verdict by walking the subject's latest non-superseded `consent:state:*` emission, gated by `valid_until`, and tracks producer deletion-SLA obligations on revocation."

#### §8.1.11.1 Effective consent resolution (read path)

For a target Contribution `T` carrying `subject_key_ids` of length ≥ 1, the effective consent state per subject `s ∈ T.subject_key_ids` is computed as:

```
resolve_consent(T, s, now):
    candidates = federation_attestations
        .where(target == T)
        .where(attesting_key_id == s OR
               attesting_key_id ∈ delegates_to(s).proxies)
        .where(dimension matches "consent:state:*")
        .where(supersedes_id IS NULL OR replaced by latest in supersedes chain)
        .where(valid_until IS NULL OR valid_until > now)
        .order_by(asserted_at DESC)

    latest = first(candidates)
    return:
        granted   if latest.dimension == "consent:state:granted"
        revoked   if latest.dimension == "consent:state:revoked"
        expired   if latest.dimension == "consent:state:expired" OR
                                   latest.valid_until passed without renewal
        unspecified  if no candidates (subject named but never declared)
```

Substrate MAY cache the resolution per `(T, s)` keyed on the latest `asserted_at`; invalidate on any new `consent:*` write from `s` or `s`'s proxy chain.

#### §8.1.11.2 Multi-subject revocation (any-subject-binding)

When `len(T.subject_key_ids) > 1`, each subject is an **independent** revocation authority. A `withdraws` admitted under [§3.2.3 rule 2 or 3](03_primitives.md) from ANY single subject in `T.subject_key_ids` evicts the Contribution. Consumer policy MUST treat `T` as revoked from the perspective of all subjects (no "majority-rules" or "all-subjects-must-agree" softening) — this is the subject-as-individual principle from MISSION.md §1.5 applied at the subject-authority layer.

Concrete cases:
- Group photo with three subjects: any one subject revokes → the photo is evicted from federation propagation.
- Group chat export with N participants: any one participant revokes → the export is evicted.
- Multi-party contract: any one signatory revokes → the contract Contribution is evicted (separate from the legal-validity question, which is consumer-side; the substrate just removes the wire artifact).

Producers MAY mitigate by partitioning content into per-subject Contributions (e.g., one chat-message Contribution per author, linked via `topical_relation:replies_to`) so that one subject's revocation doesn't evict another's content.

> **No distinct multi-subject evict path — admission + precedence is sufficient (normative confirmation, 1.0-RC5; resolves [CIRISPersist#146](https://github.com/CIRISAI/CIRISPersist/issues/146) Ask 4).** Multi-subject eviction is fully expressed by two mechanisms already in the spec and needs **no new primitive, dimension, or `hard_case`**: (1) **admission** — the per-subject `withdraws` is admitted under [§3.2.3 rule 2/3](03_primitives.md) exactly as a single-subject `withdraws` is (each subject in `subject_key_ids` is independently a valid revoker; the rule does not change for `len > 1`); (2) **precedence** — the latest-wins / revoke-is-terminal precedence at the consumer read path ([§8.1.11.1](#81111-effective-consent-resolution-read-path)) applies per-subject, and any one subject's terminal `withdraws` evicts `T` for all (the any-subject-binding above). There is no quorum to compute and no "evict event" to materialize separately from the admitted `withdraws` — the withdrawal IS the evict. Substrates implement this as the OR over per-subject revocation state, not as a new code path.

#### §8.1.11.3 Deletion-SLA watcher (substrate emission)

When subject `s` emits `consent:state:revoked` (or an admitted `withdraws`) against target `T`, substrate watches for producer compliance:

```
watch_sla(T, s, revocation_at):
    sla = T.attestations
        .where(attesting_key_id == T.attesting_key_id)
        .where(dimension == "consent:deletion_sla:*")
        .latest()
        .extract_days()

    if sla is None:
        return  # no SLA commitment; no watcher

    deadline = revocation_at + sla.days
    completion = T.attestations
        .where(attesting_key_id == T.attesting_key_id)
        .where(dimension == "consent:deletion_complete")
        .where(asserted_at > revocation_at)
        .first()

    if now > deadline and completion is None:
        emit hard_case:consent_sla_breach against T
```

The `hard_case:*` emission is the **primitive observability signal**; per [§11.6](11_governance.md) governance, LensCore composes derived detectors on top (`detection:consent:repeat_sla_breach`, etc.).

#### §8.1.11.4 Bilateral pair composition (PARTNERED ceremony)

For the bilateral partnership shape per [§5.6.8.7](05_namespace.md) `consent_record`:

```
ratified_pair(pair_id):
    subject_half = federation_attestations
        .where(subject_kind == "consent_record")
        .where(envelope.bilateral_pair_id == pair_id)
        .where(envelope.stance == "granted")
        .where(envelope.subject_key_id == attesting_key_id)  # subject signing for self
        .first()

    producer_half = federation_attestations
        .where(subject_kind == "consent_record")
        .where(envelope.bilateral_pair_id == pair_id)
        .where(envelope.stance == "granted")
        .where(envelope.target_key_id == subject_half.subject_key_id)
        .where(attesting_key_id != subject_half.subject_key_id)  # producer signing
        .first()

    return subject_half AND producer_half  # both required for ratification
```

`topical_relation:bilateral_pair` is the open-vocab edge documenting the pair linkage (recommended, not required for ratification — `bilateral_pair_id` is the binding mechanism).

#### §8.1.11.5 Decay-protocol stage composition (CIRISAgent CEM ANONYMOUS)

For consent_records carrying `decay_protocol: "ciris-agent-90day"` (or any named decay path):

```
decay_state(consent_record, now):
    elapsed = now - revocation_event(consent_record).asserted_at

    walk substrate emissions on dimension `consent:decay:*` against consent_record,
    matching the decay_protocol's stage map. CIRISAgent 90-day decay:

    elapsed < 30d  → consent:decay:identity_severed (substrate emits at elapsed=0)
    30d ≤ elapsed < 60d  → consent:decay:patterns_anonymized (substrate emits at elapsed=30d)
    60d ≤ elapsed < 90d  → (in flight; no new stage emission)
    elapsed ≥ 90d  → consent:decay:complete (substrate emits at elapsed=90d)
```

Per [§5.6.8.6](05_namespace.md) `consent:decay:{stage}` is open vocab. Other decay protocols MAY name other stage sequences; substrate honors the producer's published `decay_protocol` string and emits stages per the protocol's stage map.

#### §8.1.11.6 What Policy K composes

| CIRISAgent CEM stream | Policy K composition |
|---|---|
| **TEMPORARY** (14d default) | `consent:state:granted` + envelope `valid_until = asserted_at + 14d` + auto-renew dimension on interaction (consumer-policy concern) |
| **PARTNERED** | Bilateral pair per §8.1.11.4: subject `consent:partnership_grant` + producer `consent:partnership_accept` under same `bilateral_pair_id`; no `valid_until` |
| **ANONYMOUS** | Revocation + decay-protocol per §8.1.11.5: substrate emits stage milestones; agent honors stage-appropriate processing constraints |

Per CEG's [§3.4 MISSION.md](../../MISSION.md) layering: CIRISAgent's three streams are a **named bundle** at the consumer-policy layer. Other agents MAY compose other streams over the same wire primitives. CEG documents the canonical bundle for ecosystem coordination; CEG does not lock the bundle.

### §8.1.12 Policy L — Self/family membership composition (CEG 0.7 addition)

Per [CIRISRegistry#47](https://github.com/CIRISAI/CIRISRegistry/issues/47) + [§5.6.8.8](05_namespace.md) `identity_occurrence` + [§5.6.8.9](05_namespace.md) `family` + [§10.1.4](10_endpoints.md) structural-invisibility. Composition pattern for resolving the **current membership set** of an identity's self-collective OR a family, gating the at-rest encryption flow (CIRISPersist#152) that wraps DEKs to admitted members.

Reads as: "for any `cohort_scope: self | family` Contribution, the substrate computes the current member set by walking the latest `identity_occurrence` / `family` Contributions and resolves which keys MUST receive a `key_grant` wrap of the content DEK."

#### §8.1.12.1 Self-collective resolution

For identity `I`, the current self-collective is computed as:

```
resolve_self(I, now):
    candidates = federation_attestations
        .where(subject_kind == "identity_occurrence")
        .where(envelope.identity_key_id == I)
        .where(supersedes chain → latest non-superseded)
        .where(NOT withdrawn by I or by current occurrence)
        .where(envelope.valid_until IS NULL OR > now)
    return {c.envelope.occurrence_key_id for c in candidates} ∪ {I}
```

The root `I` itself is implicitly a member (the identity_key is always an admissible signer for its own content). Single-vouch admission: any current occurrence (including `I` itself) may admit a new occurrence via `attesting_key_id` on a fresh `identity_occurrence` Contribution.

**Concrete**: Alice has admitted `alice_phone`, `alice_laptop`, `alice_agent`. Her self-collective is `{alice_root, alice_phone, alice_laptop, alice_agent}`. When Alice's phone publishes a `cohort_scope: self` Twitter scroll, the substrate wraps the content DEK under all four keys; the content reaches her laptop and agent via the at-rest encryption flow without emitting `holds_bytes:sha256:*`.

#### §8.1.12.2 Family membership resolution

For family `F`, the current member set is computed as:

```
resolve_family(F, now):
    latest = federation_attestations
        .where(subject_kind == "family")
        .where(envelope.family_key_id == F)
        .where(supersedes chain → latest non-superseded)
        .where(admission rule satisfied per CURRENT consensus_protocol;
               see §8.1.12.3)
    return {m.key_id for m in latest.envelope.members}
```

Each `member.key_id` is itself an identity — the substrate does NOT walk into each member's `identity_occurrence` set at family-resolution time. When DEK wrapping happens, each member's identity_key receives a `key_grant`; the member's own substrate then composes Policy L on its own self-collective to propagate to that member's devices/agents (recursive composition).

**Concrete**: The Acme Household family has members `{alice_root, bob_root, roku_living_room, kitchen_tablet, nest_thermostat}`. When Alice's phone publishes a `cohort_scope: family` dinner photo with `family_id: acme-household`, the substrate wraps the DEK under each of those 5 identity keys. Alice's `alice_root` then re-wraps to her self-collective (phone, laptop, agent); Bob's `bob_root` re-wraps to his self-collective; the Roku and kitchen tablet receive directly (single-key identities — they don't have multi-occurrence sets); the Nest thermostat same.

#### §8.1.12.3 Membership-change admission per consensus_protocol

A proposed membership change (addition OR removal) rides a `supersedes` Contribution on the family's latest admitted `family` Contribution. Substrate admission gate evaluates the CURRENT family's `consensus_protocol` against the signatures on the proposal:

```
admit_family_change(F, proposed: family_record):
    current = resolve_family(F, now)
    protocol = current_family_record(F).consensus_protocol
    signatures = collect_signatures_on(proposed)

    match protocol:
        "founder_only":
            return any sig from m where m.role == "founder" in current
        "unanimous":
            return every m.key_id in current has signed
        "majority":
            return count(sig from m in current) > len(current) / 2
        "quorum:M/N":
            return count(sig from m in current) >= M  // M is ABSOLUTE — see §8.1.12.3.1
        "weighted:{rubric}":
            return sum(weight(m, rubric) for m in current who signed) >= threshold
        "custom:{family_id}":
            return operator-defined predicate evaluates to true

    if admit:
        emit hard_case:family_membership_change:{F}
        trigger §8.1.12.4 key_grant cascade
    else:
        hold in pending state (per operator-policy window) OR
        emit hard_case:family_consensus_protocol_violation:{F} and reject

##### §8.1.12.3.1 `quorum:M/N` is absolute-M (normative; per CIRISRegistry#52 + NodeCore#30)

The `M` in `quorum:M/N` is an **absolute signature count**, NOT a fraction that rebases with roster size. A `quorum:2/3` collective that grows to 5 members still admits at **2** signatures; the `N` is documentary (records the roster size at protocol-adoption time) and is NOT recomputed against the live roster. This was an ambiguity in the pre-pin §8.1.12.3 pseudocode ("operator policy resolves rebasing"); CIRISRegistry#52's ceremony gate ([§52 design](https://github.com/CIRISAI/CIRISRegistry/issues/52)) requires a deterministic, operator-independent reading, so absolute-`M` is pinned.

**Rationale**: absolute-`M` is the simpler invariant (the gate is a pure `count >= M` with no live-roster division), it matches NodeCore's shipped `evaluate_consensus_protocol` ([NodeCore#30](https://github.com/CIRISAI/CIRISNodeCore/issues/30) commit 7469dcc) so the single shared predicate needs no change, and proportional/rebasing quorum is still expressible for operators who want it via `weighted:{rubric}` (assign each member weight 1, set threshold = `ceil(roster_size / 2)` recomputed by the rubric resolver). Collectives that want "M grows with the roster" therefore use `weighted:`, not `quorum:` — keeping `quorum:M/N` unambiguous.

This rule applies identically to `community` admission ([§8.1.13.2](#81132-community-admission-per-consensus_protocol-cohort_subkind-dispatch)) — the `quorum:M/N` arm there is the same absolute-`M` reading.
```

**Consensus-protocol amendment** (changing `consensus_protocol` itself on a non-entrenched family) rides the SAME admission rule on the proposed amendment Contribution — meta-amendment shape parallel to [§11.2.3](11_governance.md). Entrenched families (`consensus_protocol_entrenched == true`) reject amendments at the substrate gate; replacement requires the out-of-band ceremony documented per family (for HUMANITY_ACCORD see [§9.2](09_humanity_accord.md)).

#### §8.1.12.4 Key-grant cascade (the at-rest encryption flow)

On admission of a new `identity_occurrence` OR a `family` membership-add, substrate emits retroactive `key_grant`s wrapping all extant `cohort_scope: self|family` content DEKs to the new member's key:

```
on_member_added(scope_target, new_member_key):
    for each Contribution C in federation_attestations where:
        C.cohort_scope == "self" AND C.attesting_key_id ∈ resolve_self(scope_target)
        OR
        C.cohort_scope == "family" AND C.family_id == scope_target
    AND C is still substrate-admitted (not withdrawn / not expired):
        emit key_grant {
            wrap_algorithm:     X25519MlKem768Aes256GcmHkdfSha256,  // v2 — see PQC note
            recipient_key_id:   new_member_key,
            content_sha256:     C.evidence_refs[0].sha256,
            scope:              GroupMember,
            wrapped_dek:        wrap(C.dek, new_member_key.pubkey),
            ...
        }
```

**PQC at rest — `wrap_algorithm: v2` MANDATORY (normative).** The self/family at-rest DEK MUST be wrapped with **`wrap_algorithm: v2` = `x25519_mlkem768_aes256_gcm_hkdf_sha256`** (hybrid X25519+ML-KEM-768; the [§5.6.8.4](05_namespace.md) variant `X25519MlKem768Aes256GcmHkdfSha256`), **never v1** (X25519-only). Self/family content is the user's most private and longest-lived data (memory, photos, identity, the [§8.1.12.7](#81127-the-self-at-login--app--agent-co-self--partnered-delegation-ceg-015-normative-composition) Self DEK) — a classical-only KEM is a harvest-now-decrypt-later exposure, the identical mandate as the streaming epoch DEK ([§10.5.3](10_endpoints.md)). The AES-256-GCM content seal is symmetric (PQC-safe); the wrap signature is hybrid Ed25519+ML-DSA-65. A Consumer MUST reject a self/family at-rest grant carrying `wrap_algorithm: v1`.

The cascade is the wire-format primitive for the "I got a new phone and want my Twitter history" + "I added Carol to the household for a week" flows. Operator policy MAY bound the cascade depth (e.g., last 90 days of self-content; opt-in for the full historical wrap).

#### §8.1.12.5 Forward secrecy on member removal (Option A, recommended for v1)

Per [CIRISRegistry#47 Decision 1](https://github.com/CIRISAI/CIRISRegistry/issues/47) recommendation, CEG 0.7 adopts **Option A** — once shared, always shared at the wire layer:

```
on_member_removed(scope_target, removed_member_key):
    // The removed member retains existing key_grants — cannot retroactively un-share.
    // Substrate STOPS wrapping NEW key_grants to them on subsequent
    // cohort_scope: self|family Contributions for scope_target.
    // No DEK rotation; no re-encryption of historical content.
```

This is consistent with [§11.4](11_governance.md) "takedown isn't a coup" + [§3.2](03_primitives.md) `withdraws-isn't-retroactive` semantics — historical state isn't retroactively re-keyed. Option B (rotate-DEK on removal) is deferred to a future `subject_kind: family_rotation` ceremony; CEG 0.7 documents the slot, leaves the rotation primitive for a downstream-demand-driven release.

**Why Option A**: aligns with the substrate's existing forward-secrecy posture (consent revocations don't retroactively un-emit per CEG 0.6 §8.1.11; takedowns don't retroactively un-emit per §11.4); matches user-intuition that "leaving the family" governs future content, not historical; bounded substrate cost (no re-wrap-all-content storm on member removal).

#### §8.1.12.6 Composition with CEG 0.6 subject_key_ids[]

`identity_occurrence` + `family` (membership / visibility scoping) compose cleanly with CEG 0.6 `subject_key_ids[]` (revocation authority):

- A `cohort_scope: family` Contribution naming `subject_key_ids: [user_canonical_hash]` is admitted at family visibility AND the named subject retains independent revocation authority per CEG 0.6 §8.1.11.
- A `cohort_scope: self` Contribution that Alice writes about Bob (Bob in `subject_key_ids`) stays in Alice's self-collective (Bob does NOT receive a key_grant unless Bob's identity is in Alice's self — which it isn't). Bob's subject-side revocation authority still composes: Bob CAN issue `withdraws` against Alice's Contribution (admitted per CEG 0.6 §3.2.3 rule 2 even though Bob can't access the bytes); admission emits `hard_case:consent_sla_breach` clock-start if Alice committed `consent:deletion_sla`.

The orthogonality holds: **`cohort_scope` is producer-side visibility scoping**; **`identity_occurrence` + `family` are substrate-side membership primitives that gate at-rest DEK wrapping**; **`subject_key_ids` is subject-side revocation authority**. Three independent envelope-level concerns that compose without overlap.

#### §8.1.12.7 The "Self at login" — app + agent co-self + partnered delegation (CEG 0.15, normative composition)

Per [CIRISRegistry#65](https://github.com/CIRISAI/CIRISRegistry/issues/65). The canonical user-identity composition: a person's **app** (the KMP client key) and their **agent** (CIRISAgent's local key) are two occurrences of one user identity that share one **Self DEK**, and at login the agent is **partnered + delegated** to act as the user on the network. **No new structural primitive** — this composes `identity_occurrence` + Policy L + `consent:partnered` + `delegates_to` + `identity_type`-set + `transport_destination`. The four implementations MUST follow this shape so a "Self" is identical everywhere.

**The Self (membership).** One **user `identity_key`**: hybrid Ed25519+ML-DSA-65 ([§10.3](10_endpoints.md)), hardware-rooted ([§9.4](09_humanity_accord.md); WebAuthn/passkey is the *presence/unlock factor*, not the key), with `identity_type ⊇ {user}` — and `⊇ {user, wise_authority}` when the user is also a WA ([§7.0.1](07_reserved.md) set-membership; one key, two roles). Its occurrences ([§5.6.8.8](05_namespace.md)): the **app** (`device_class: phone|laptop`) and the **agent** (`device_class: agent`), co-admitted at login by single-vouch (the user key admits the agent occurrence). Both receive the **Self DEK** via the [§8.1.12.4](#81124-key-grant-cascade-the-at-rest-encryption-flow) Policy-L cascade — every `cohort_scope: self` Contribution's DEK wraps to both, so **the app and the agent decrypt the same Self content** (memory / config / consent / identity). That shared cascade *is* the "single Self key."

**Two layers — and they are independently revocable (the load-bearing distinction):**

| Layer | Mechanism | Buys | Revoked by |
|---|---|---|---|
| **Co-self** (visibility) | occurrence + Policy-L Self DEK | the agent can *read/manage the user's Self* (decrypts self-content) | `withdraws` the occurrence → Option-A re-key ([§8.1.12.5](#81125-forward-secrecy-on-member-removal-option-a-recommended-for-v1)) |
| **Agency** (act-on-behalf) | `consent:partnered` + scoped `delegates_to` | the app may *act AS the user on the network* (send/receive, presence, sub-delegate) | `withdraws` the delegation → agency ends, **co-self unaffected** |

So a user MAY grant a device co-self (it manages their data locally) while revoking its network agency — or vice-versa — without disturbing the other.

**Agency at login (the partnering + delegation).**
1. **Partnering** — the user emits `consent:partnership_grant` and the agent occurrence emits `consent:partnership_accept` under one `bilateral_pair_id` ([§8.1.11.4](#81114-bilateral-partnered-pair) PARTNERED); the bilateral pair is the persistent, auditable relationship.
2. **Delegation** — the user identity emits `delegates_to` against the **agent occurrence key**, with `delegated_scope` drawn from the canonical act-on-behalf kinds below, `delegation_purpose: "act_as_user"`, bounded `delegation_valid_until`. Sub-delegation works because `delegates_to` chains (depth-capped at 5 per [§13.3](13_anti_patterns.md)).
3. **This grant is FEDERATION-tier ([§10.1.5](10_endpoints.md)), not local** — *other peers must verify the agent's authority before honoring its messages/presence*, so the partnering+delegation is signed + promoted at login. **Promotion is the "app shows up on the network" moment.** (The agent's own self-content stays local-tier; only the act-on-behalf authorization federates.)

**Canonical `delegated_scope` kinds for act-on-behalf** (recommended; open-vocab per [§11.2.1](11_governance.md), named for ecosystem coordination):

| scope | grants the agent |
|---|---|
| `act_on_behalf` | umbrella: emit Contributions AS the user (`attesting_key_id = agent occurrence`, speaking as the user identity per [§5.6.8.8](05_namespace.md)) |
| `message_io` | send + receive directed messages on the user's behalf |
| `network_presence` | announce/resolve the user's `transport_destination` ([§5.6.8.8.1](05_namespace.md)) — be reachable AS the user |
| `sub_delegation` | issue further `delegates_to` within the granted scope (depth-capped) |

**Moderation duties — `moderate` / `takedown` / `review` (canonical, ENFORCED admission; 1.0-RC19, resolves [CIRISRegistry#90](https://github.com/CIRISAI/CIRISRegistry/issues/90)).** Moderation is a **delegable *duty*, not a platform/fabric-assigned role**: a participant exercises it *as themselves*, or delegates it — to **their agent** (AI on-behalf-of) or to **any trusted party** (a human, a community moderator) — via `delegates_to`. These three kinds carry **enforced admission** (unlike the *recommended* act-on-behalf kinds above), mirroring `consent_revocation` ([§3.2.3 rule 3](03_primitives.md)):

| scope | authorizes the delegate to emit, on the delegator's behalf | shipped primitive |
|---|---|---|
| `moderate` | a `moderation:{allegation_type}` ModerationEvent + the report→`scores` path + `age_assurance:*`/`content_class:*` gates | [§5.6.4](05_namespace.md) / [§8.1.10](08_composition.md) |
| `takedown` | a `takedown_notice` (incl. the §11.4 immediate-removal fast-path) | [§5.6.8.4](05_namespace.md) / [§11.4](11_governance.md) |
| `review` | a `reconsideration:{grounds}` appeal / review | [§5.6.4](05_namespace.md) |

**Enforced-admission rule (normative):** a moderation action above is admitted **iff** its `attesting_key_id` is the delegator itself **or** sits on a live `delegates_to` chain bearing the matching scope from the delegator (the entity holding the duty over the target content/scope) — exactly the §3.2.3 rule-(3) proxy shape (`scope ⊇ {moderate|takedown|review}`), depth-capped per [§13.3](13_anti_patterns.md), revocable by `withdraws` against the `delegates_to`. **Reject otherwise.** Every action is therefore delegate-signed, delegator-traceable up the chain, and revocable — the [§11.4](11_governance.md) **"takedown-isn't-a-coup"** property made *structural* (coordinated + attributable + revocable, never a unilateral seizure). See [§11.10](11_governance.md). **1+4 preserved** — a `delegated_scope` vocabulary + enforcement addition over the existing `delegates_to`; the action primitives already ship; no new structural primitive.

**Partnership WITHOUT agency — the infrastructure delegation profile (normative, 1.0-RC7 — resolves [CIRISRegistry#83](https://github.com/CIRISAI/CIRISRegistry/issues/83) §3).** The flow above binds an **agent** (a key with a brain) to a user as partnership **+ agency** — the scope includes `act_on_behalf` / `message_io`, so the agent reasons and acts AS the user. A **fabric/infrastructure node** ([§7.0.1](07_reserved.md); CIRISServer) needs the *partnership* (identity + the [§5.6.8.10](05_namespace.md) owner-binding that lets it hold non-infra membership standing under the user's authority) but MUST NOT receive agency — [§1.3](01_foundation.md) "infrastructure must not have agency." CEG pins a **reserved two-prefix scope split** so a verifier can enforce this cryptographically:

| prefix | class | scopes |
|---|---|---|
| `infra:*` | server-class (allowed for a `node`-role delegate) | `infra:network_presence`, `infra:join_communities`, `infra:serve`, `infra:store`, `infra:attest`, `infra:transport` |
| `agency:*` | brain-only (forbidden for a pure `node`-role delegate) | `agency:act_on_behalf`, `agency:message_io`, `agency:reason`, `agency:decide` |

**Conformance:** a `delegates_to` whose `attested_key_id` resolves to an identity whose `identity_type` ([§7.0.1](07_reserved.md)) is `node`-only (no `agent`/brain) MUST carry **only** `infra:*` scopes; a verifier MUST **reject** (treat as non-conformant, never grant) an `infra`-only key presenting any `agency:*` scope. This makes §1.3 a wire-checkable invariant: a user-owned fabric node can serve + hold group-membership *standing* under the user's authority, but the delegation **literally cannot carry agency**. The legacy unprefixed kinds above remain valid for `agent`-role delegates (the Self-at-login agency profile) and are the `agency:*` / `infra:network_presence` equivalents; new **infra** delegations SHOULD use the explicit `infra:*` prefixes.

**Cohabitation (`agent = node + brain`):** when both compose in one process, the node holds **partnership-without-agency** (`infra:*` — identity + membership standing) and the brain layers **Self-at-login partnership-with-agency** (`agency:*` — reasoning) as a *separate* `delegates_to`. Two delegations, two scope classes, independently revocable — the user can strip the brain's agency while the fabric node keeps serving.

**Transport (network presence) — AV-17.** Each occurrence binds a `transport_destination` ([§5.6.8.8.1](05_namespace.md)); the app is reachable on RET *as the user occurrence*. The Reticulum destination is a **separate dual-key transport identity** that the user's signing key *authorizes by signing the binding* — the federation signing seed MUST NOT enter the transport layer (AV-17 / [CIRISEdge#15](https://github.com/CIRISAI/CIRISEdge/issues/15)). "User key used as a transport key" means *roots/authorizes* the transport identity, not a shared keypair.

**Worked login flow.** (1) unlock the hardware-rooted user key (WebAuthn presence) → (2) admit the agent occurrence (single-vouch) → Policy-L Self DEK now wraps to both → (3) optionally add the `wise_authority` role to the user's `identity_type` set → (4) bind each occurrence's `transport_destination` → (5) `consent:partnership_grant`/`accept` under a `bilateral_pair_id` → (6) `delegates_to(user → agent occurrence, scope: [act_on_behalf, message_io, network_presence, sub_delegation])`, **promoted to federation-tier**. The app now reads the user's Self locally AND acts as the user on the network; the user can revoke either layer independently.

##### §8.1.12.7.1 Signing member sets (normative — the JCS contract Verify hybrid-signs; resolves CIRISVerify#63)

Each of the three Self-at-login Contributions is hybrid-signed over `JCS(envelope)` ([§0.9](00_conformance.md) / RFC 8785), and at login promoted to federation-tier (the [§10.1.5.3](10_endpoints.md) promotion canonicalizes the **exact committed member set** — omit-vs-materialize ([§0.9.2](00_conformance.md)) is load-bearing; the signer MUST NOT re-default). **The [§0.9.2.1](00_conformance.md) determinism rules apply** (raised in CIRISVerify#63 review): `subject_key_ids[]` and `delegated_scope[]` are **lexicographically sorted** (set-semantics); `aspects[]` retains RNS order (sequence-semantics); all key/hash/pubkey byte fields (`*_key_id`, `subject_key_ids[]`, the two reticulum pubkeys, `destination_hash`) are **lowercase hex per [§0.6](00_conformance.md)**; all timestamps are **[§0.5](00_conformance.md)-canonical**. The member sets the producer commits (and which `JCS` therefore covers) are pinned below. Optional [§4](04_envelope.md) envelope fields not listed ride the §0.9.2 omit rule (absent unless the producer sets them).

**(a) `consent:partnership_grant:v1` (user side) / `consent:partnership_accept:v1` (agent side)** — bare `scores` ([§5.6.8.6](05_namespace.md)) bound by `bilateral_pair_id`:
```
{ attestation_type: "scores",
  attesting_key_id: <user identity_key_id (grant) | agent occurrence_key_id (accept)>,
  dimension:        "consent:partnership_grant:v1" | "consent:partnership_accept:v1",
  score:            <positive>,
  subject_key_ids:  [<the partner key: agent occurrence (grant) | user identity (accept)>],
  bilateral_pair_id:<shared pair id>,                       // §8.1.11.4 binding mechanism
  signed_at:        <rfc3339_canonical> }
```
> **Version segment pinned (normative, 1.0-RC5 — resolves [CIRISRegistry#78](https://github.com/CIRISAI/CIRISRegistry/issues/78) caveat 2).** The dimension carries the `:v1` version segment — `consent:partnership_grant:v1` / `consent:partnership_accept:v1` — to satisfy the [§13.1](13_anti_patterns.md) `scores` version-segment gate. This is the canonical form persist v6.5.0 shipped; confirmed, not changed. The `:v1` is the partnership-ceremony schema version (bump to `:v2` only if the bilateral shape changes); the shared `bilateral_pair_id` remains the §8.1.11.4 binding mechanism.

**Canonical signed member set for the two `:v1` envelopes (normative, 1.0-RC7 — resolves [CIRISRegistry#81](https://github.com/CIRISAI/CIRISRegistry/issues/81) / [CIRISVerify#63](https://github.com/CIRISAI/CIRISVerify/issues/63)).** Both impls MUST canonicalize (and thus hybrid-sign) **exactly** this member set, or the JCS bytes — and the signatures — diverge. The set is the [§5.6.8.6](05_namespace.md) bare-`scores` shape; these seven members are **REQUIRED** (present in the JCS for both `grant` and `accept`):

| Member | Value | Verify#63 name |
|---|---|---|
| `attestation_type` | literal `"scores"` | — |
| `attesting_key_id` | **the signer** = `granter_key_id` (grant) / `accepter_key_id` (accept); the bound sig binds because signer ≡ `attesting_key_id` | `granter_key_id` / `accepter_key_id` |
| `dimension` | literal `"consent:partnership_grant:v1"` \| `"consent:partnership_accept:v1"` | `envelope_type` |
| `score` | positive (the affirmation) | — |
| `subject_key_ids` | **exactly `[partner_key_id]`** — the OTHER party (agent occurrence for `grant`, user identity for `accept`); single-element, [§0.9.2.1](00_conformance.md) set-sort is trivial | `partner_key_id` |
| `bilateral_pair_id` | the shared join ([§8.1.11.4](#81114-bilateral-partnered-pair)) — identical string on both halves | `bilateral_pair_id` |
| `signed_at` | [§0.5](00_conformance.md)-canonical RFC 3339 | `timestamp` |

**Mapping (so the two impls agree on naming):** `granter`/`accepter` ≡ `attesting_key_id` (the signer of that half); `partner` ≡ `subject_key_ids[0]`. **No `valid_until`** — a PARTNERED pair has no expiry ([§8.1.11.4](#81114-bilateral-partnered-pair)); the field is **omitted** (NOT materialized as `null`), per the [§0.9.2](00_conformance.md) omit rule. **All other [§4](04_envelope.md) envelope fields ride the omit rule** — absent from the JCS unless the producer explicitly sets them; the signer MUST NOT re-default. Additional canonical members are a `:v2` bump, never a silent `:v1` addition. Byte-field members (`attesting_key_id`, `subject_key_ids[]`) are lowercase-hex per [§0.6](00_conformance.md).

**(b) `delegates_to` (user → agent occurrence)** — the act-on-behalf grant ([§3.2](03_primitives.md) envelope shape):
```
{ attestation_type:     "delegates_to",
  attesting_key_id:     <user identity_key_id>,
  attested_key_id:      <agent occurrence_key_id>,          // the delegate
  delegated_scope:      ["act_on_behalf", ...],             // §8.1.12.7 canonical kinds
  delegation_purpose:   "act_as_user",
  delegation_valid_from:<rfc3339_canonical>,
  delegation_valid_until:<rfc3339_canonical>,
  signed_at:            <rfc3339_canonical> }
```

**(c) `transport_destination` binding** — an `identity_occurrence` ([§5.6.8.8](05_namespace.md) / [§5.6.8.8.1](05_namespace.md)) carrying the binding; signed by `identity_key_id` (or a current occurrence), AV-17:
```
{ attestation_type: "scores",
  subject_kind:     "identity_occurrence",                  // payload discriminator §4.2.2.3
  attesting_key_id: <user identity_key_id | a current occurrence>,
  identity_key_id:  <user identity_key_id>,
  occurrence_key_id:<the occurrence being bound>,
  device_class:     "phone" | "laptop" | "agent" | ...,
  transport_destination: {
    reticulum_x25519_pubkey:  <[u8;32]>,
    reticulum_ed25519_pubkey: <[u8;32]>,
    destination_hash:         <[u8;16]>,                    // MUST derive per §5.6.8.8.1
    app_name:                 <string>,
    aspects:                  [<string>, ...] },            // ordered
  asserted_at:      <rfc3339_canonical>,
  signed_at:        <rfc3339_canonical> }
```

Registry owns these member sets (this section); Verify computes `JCS(...)` + the hybrid Ed25519+ML-DSA-65 signature over each via `jcs::canonicalize` ([CIRISVerify#59](https://github.com/CIRISAI/CIRISVerify/issues/59)); the promotion signature ([§10.1.5.3](10_endpoints.md) OQ-4) is the identical JCS bytes — confirmed.

**`encryption_pubkeys` joins member set (c) (1.0-RC1 — [CIRISVerify#64](https://github.com/CIRISAI/CIRISVerify/issues/64)).** When the occurrence carries the [§5.6.8.8.2](05_namespace.md) `encryption_pubkeys` field-set, both halves are **inside the signed JCS bytes** as opaque base64 strings (RFC 4648 STANDARD, padded — the [§0.9.2.1](00_conformance.md) rule-2 pin). Optional presence rides the §0.9.2 omit rule (absent unless the producer sets it; the signer MUST NOT re-default). They are payload, never verification material — neither half may be fed to a signature-verify path (the §5.6.8.8.2 key-separation rule, type-enforced in Verify).

### §8.1.13 Policy M — Community membership composition

Per [CIRISRegistry#48](https://github.com/CIRISAI/CIRISRegistry/issues/48) + [§5.6.8.10](05_namespace.md) `community` + [§5.6.8.11](05_namespace.md) `location_proof`. Composition pattern for resolving the **current membership set** of a community, gating cohort-filtered visibility for `cohort_scope: community` content.

Sibling to [§8.1.12 Policy L](#8112-policy-l--selffamily-membership-composition-ceg-07-addition) (self/family) but with different defaults — community content is encrypted under a per-community DEK + emits `holds_bytes:sha256:*` with cleartext provenance (CEG 0.17 — see §8.1.13.3); the privacy property is byte-confidential-to-members, not cohort-filtered-visibility.

#### §8.1.13.1 Community membership resolution

For community `C`, the current member set is computed as:

```
resolve_community(C, now):
    latest = federation_attestations
        .where(subject_kind == "community")
        .where(envelope.community_key_id == C)
        .where(supersedes chain → latest non-superseded)
        .where(admission rule satisfied per CURRENT consensus_protocol;
               see §8.1.13.2)
    return {m.key_id for m in latest.envelope.members}
```

Same shape as `resolve_family` ([§8.1.12.2](#81122-family-membership-resolution)). Each `member.key_id` is an identity (which may itself have a multi-occurrence set via CEG 0.7 `identity_occurrence`).

##### §8.1.13.1.1 Deterministic resolution + member→address resolution (NORMATIVE)

In a CEG/RET stack member resolution replaces DNS+IP: it is a chain of *signed* bindings, and every implementation MUST resolve **identically** (a 1.0 interop requirement). Two normative pins:

**(a) Determinism of `resolve_community`.** Where the §8.1.13.1 computation has choices, they are fixed:
- **`now` semantics** — the resolving Consumer's own clock; membership is evaluated as of `now` (a member is included iff joined ≤ `now` and not removed ≤ `now`). No global clock assumed.
- **"latest non-superseded"** — the `community` Contribution with the highest `signed_at`; on equal `signed_at`, the higher `canonical_bytes_hash` ([§0.9](00_conformance.md)) wins (total order, no ambiguity). The same comparator the [CIRISVerify#49](https://github.com/CIRISAI/CIRISVerify/issues/49) R1/Q1 merge uses (quorum_weight DESC → signed_timestamp DESC → canonical_bytes_hash).
- **member ordering** — the returned set is canonically ordered by `key_id` (lowercase-hex byte order) for any downstream hashing/iteration.
- **founder-subset eval** — for `cohort_subkind: infrastructure` ([§5.6.8.10](05_namespace.md)) admission is `evaluate_consensus_protocol` over `{m : m.role == founder}`, NOT all members.

**(b) Member → reachable address (the DNS-free resolution).** Resolving a member identity to a Reticulum destination:

```
resolve_member_transport(C, now):
    members = resolve_community(C, now)            // (a) above; founder-quorum verified
    out = []
    for M in members (canonical key_id order):
        occ = latest non-superseded identity_occurrence of M
                 with transport_destination present, valid at now,
                 hybrid-sig verified against M (or an occurrence of M)   // §5.6.8.8.1
        if occ is null: continue                   // no authenticated binding → skip (fail-secure)
        td = occ.transport_destination
        REQUIRE td.destination_hash == rns_destination_hash(    // §5.6.8.8.1.1 pinned two-stage
                 td.reticulum_x25519_pubkey, td.reticulum_ed25519_pubkey,
                 td.app_name, td.aspects)                       // NOT a flat-concat SHA-256
        out.push((M, td.destination_hash))
    return out                                     // → Reticulum announce/path-request per dest
```

The three resolutions and their trust roots: **WHO** = `resolve_community` (signed Contribution + founder-quorum) · **BINDING** = `transport_destination` (federation-key-signed `identity_occurrence`, §5.6.8.8.1) · **WHERE** = Reticulum announce/path-request (mesh, no CEG). Reachability is never trust ([§10.5.6](10_endpoints.md)): a path that answers is not an authorization; only the signed binding + quorum are.

**Cold-start (bootstrap).** A node that knows only `community_key_id` needs two out-of-band pins: (1) the `community_key_id` itself (trust anchor) and (2) ≥1 **seed `destination_hash`** (reachability). It reaches any one seed, pulls the signed `community` Contribution, runs `resolve_member_transport`, verifies founder-quorum against the pinned `community_key_id`, then floats on the live set thereafter. A malicious seed cannot forge membership (quorum signature check) — it can only deny service (try another seed). **Bootstrap reachability ≠ bootstrap trust.** `ciris-canonical` ([§5.6.8.10](05_namespace.md)) is the root community whose seeds clients pin.

#### §8.1.13.2 Community admission per `consensus_protocol` + `cohort_subkind`

A proposed membership change rides a `supersedes` Contribution on the community's latest admitted `community` Contribution. Substrate evaluates the CURRENT community's `consensus_protocol` (same six canonical kinds as family) AND any subkind-specific admission requirements:

```
admit_community_change(C, proposed: community_record):
    current = resolve_community(C, now)
    protocol = current_community_record(C).consensus_protocol
    subkind = current_community_record(C).cohort_subkind

    signatures_ok = evaluate_consensus_protocol(protocol, current, proposed.signatures)
    if not signatures_ok:
        emit hard_case:community_consensus_protocol_violation:{C}
        reject

    subkind_ok = evaluate_subkind_admission(subkind, current, proposed)
    if not subkind_ok:
        // For geographic: subkind admission failed (e.g., new member's location_proof
        // not contained in geographic_constraint, or expired location_proof)
        emit hard_case:community_consensus_protocol_violation:{C}  // same observability prefix
        reject

    admit
    emit hard_case:community_membership_change:{C}
```

The `evaluate_subkind_admission` predicate dispatches per `cohort_subkind`:

```
evaluate_subkind_admission(subkind, current, proposed):
    match subkind:
        "geographic":
            # Per §5.6.8.10 geographic admission requirement
            for each NEW member in proposed.members (not in current):
                their_proof = latest valid location_proof from new_member.key_id
                if their_proof IS NULL:
                    return false  // no location_proof on file
                if their_proof.cell_id NOT contained in current.geographic_constraint.cell_id:
                    return false  // location outside community's geographic bound
                if their_proof.valid_until passed:
                    return false  // expired
            return true
        _:
            return true  // unknown subkinds admit on consensus_protocol alone
```

Operator vocabularies extending `cohort_subkind` provide their own `evaluate_subkind_admission` predicates per the `custom:{id}` consensus_protocol hook pattern from CEG 0.7 [§8.1.12.3](#81123-membership-change-admission-per-consensus_protocol).

#### §8.1.13.3 The three crypto tiers + the Community DEK cascade (normative — supersedes the 0.8 "no cascade" reasoning)

CEG ≤ 0.16 drew a binary cut (self/family encrypt; everything else plaintext), which collapsed a bounded **Community** and the unbounded **Commons** into one bucket and left **no cryptographic home for a persecuted community**. The 0.8 "no at-rest cascade" reasoning ("per-member DEK wrap on every emission would be infeasible") was a **wrong premise** — corrected here. CEG 0.17 (per [CIRISRegistry#67](https://github.com/CIRISAI/CIRISRegistry/issues/67)) draws the line at **"does it have a bounded membership roster?"** — yes → encrypt, no → plaintext:

| Tier | `cohort_scope` | At-rest | Wire discovery | Reader |
|---|---|---|---|---|
| **self / family** | `self`, `family` | encrypted, per-write DEK | **none** ([§10.1.4](10_endpoints.md) structural invisibility) | occurrences / family members |
| **Community** | `community`, `affiliations` | **encrypted under the community DEK** | `holds_bytes:*` **+ cleartext provenance** | community members (DEK cascade) |
| **Commons** | `species`, `biosphere`, `federation` | **plaintext** | `holds_bytes:*` | anyone |

**Community DEK cascade (MANDATORY).** Community content (`cohort_scope: community | affiliations`) is encrypted at rest under a **per-community DEK** and emits `holds_bytes:sha256:*` carrying **cleartext provenance** (`attesting_key_id`, `community_id`, reason/dimension) so non-member holders can make an informed keep/evict decision without reading content. The community DEK **is the [§10.5.3](10_endpoints.md) epoch-DEK cascade applied to `cohort_scope: community`** — *a community is a stream its members subscribe to, cryptographically*: **one** DEK shared across emissions (per-emission cost O(1), not O(members)), wrapped to each member on admission, re-wrapped on membership change (§8.1.13.4), **`wrap_algorithm: v2` (hybrid PQC) MANDATORY** (same harvest-now-decrypt-later reasoning as [§8.1.12.4](#81124-key-grant-cascade-the-at-rest-encryption-flow) / [§10.5.3](10_endpoints.md)). The infeasibility objection is refuted by already-merged code (`list_key_grants_for_stream_epoch`, persist v4.4.0). This is **mandatory, not opt-in** — the tier name *is* the guarantee; a persecuted community is protected by *being a community*, not by remembering a flag. (Deliberately stronger than the [§8.1.12.4](#81124-key-grant-cascade-the-at-rest-encryption-flow) self/family opt-in: self/family has structural invisibility so at-rest crypto is defense-in-depth; community *federates*, so the DEK is its **sole** confidentiality boundary.)

**Holder-inspectability principle (normative rationale).** Any data a host holds above local tier MUST support an informed keep/evict decision: either the holder **inspects the bytes** (Commons — plaintext, maximally inspectable, hence the *preferred* social-distribution mechanism) **or** it **inspects the provenance** (Community — the cleartext `attesting_key_id` + `community_id` + reason on an otherwise-encrypted blob) and chooses to hold opaque ciphertext for a community it trusts. **Nothing above local tier is ever a forced, unattributable opaque blob.** This is *why* the split is shaped this way and what the eviction rules (persist `EvictionSweeper` / `evict_actor`) enforce.

**The `infrastructure` exception (Open-Q1 ruling, normative).** A `community` with `cohort_subkind: infrastructure` ([§5.6.8.10](05_namespace.md)) — `ciris-canonical` and any governance/trust root — **opts OUT of the mandatory DEK cascade and is Commons-tier (plaintext, `holds_bytes:*`, no DEK)**. The trust root cannot be an opaque blob; its entire purpose is public auditability — transparency-seeking, not privacy-seeking. **Node→canonical traces** (conformance / `registry_consensus` emissions to a governance community) are therefore `cohort_scope: federation` (Commons/plaintext, world-readable) — a node enrolling in governance is thereby told its conformance traces are public (Open-Q2 ruling).

#### §8.1.13.4 Forward secrecy on community member removal (Option A — now applies)

With a community DEK, community **does** have the removed-member-can-still-decrypt concern (the 0.8 "no DEK to retain" reasoning is superseded by §8.1.13.3). The [§8.1.12.5](#81125-forward-secrecy-on-member-removal-option-a-recommended-for-v1) **Option A** discipline applies identically: on member removal the substrate **rotates the community DEK** ([§11.7.1](11_governance.md)); subsequent emissions are sealed under a DEK the removed member doesn't have. "Once shared, always shared" forward-only — content the removed member already received during membership stays in their cache (no PCS); they receive no NEW community content post-removal.

#### §8.1.13.5 Geographic-community admission flow (worked example)

```
1. Alice publishes a location_proof:
   subject_kind: location_proof
   subject_key_id: alice_root_key
   cell_id: "872830828ffffff"      // H3 res 7, ~5 km², downtown Austin
   cell_resolution: 7
   asserted_at: now
   valid_until: now + 30 days
   cohort_scope: federation         // public; the disclosure IS the opt-in

   Substrate admits (resolution 7 ≤ 7 OK). Emission federates.

2. Alice proposes joining Austin community:
   subject_kind: community
   community_key_id: austin-community
   supersedes: prior_austin_community_record_id
   members: [...existing..., {key_id: alice_root_key, joined_at: now, role: member}]
   signatures: [...current_member_sigs]   // satisfies majority rule

3. Substrate admission gate (per §8.1.13.2):
   - signatures_ok: majority rule satisfied ✓
   - subkind: "geographic" → evaluate_subkind_admission:
     - alice_root_key's latest valid location_proof exists
     - cell_id "872830828ffffff" CONTAINED in "85283473fffffff" (austin metro, res 5)
       via §0.8.2 containment (res 7 ≥ res 5; parent-walk reaches the constraint)
     - valid_until in the future
     - subkind_ok ✓
   - admit + emit hard_case:community_membership_change:austin-community

4. Alice now receives cohort-filtered visibility for cohort_scope:community content
   with community_id: austin-community. No key_grant cascade (community is unencrypted);
   no at-rest wrap; substrate emits holds_bytes:* for community content per status quo.
```

#### §8.1.13.6 Composition with CEG 0.6 + 0.7

Communities compose with consent (CEG 0.6) and self-collectives (CEG 0.7) cleanly:

- A community member who is also a CIRISAgent-using individual has an `identity_occurrence` set (CEG 0.7); community content arriving at the member's identity_key is then propagated to their occurrences via Policy L's at-rest cascade (if the member's local substrate chose to re-emit the community content at `cohort_scope: self` for cross-device sync; otherwise community content stays at the cohort_scope: community visibility on each device the member uses to query)
- A community member who is also a subject in `subject_key_ids` of a community-scoped Contribution retains revocation authority per CEG 0.6 [§3.2.3](03_primitives.md) rule 2; the orthogonality between cohort_scope (visibility) and subject_key_ids (revocability) holds at community scope same as at family scope
- A geographic community whose `geographic_constraint` covers a region that overlaps a family's at-home location does NOT cross-contaminate: families are not auto-admitted to communities; communities are not auto-admitted to families. Each membership is explicit, ceremony-shaped, and independent.

#### §8.1.13.7 Delivery extension — `delivery_mode` × Policy M

Per [CIRISRegistry#44 absorbed](https://github.com/CIRISAI/CIRISRegistry/issues/44) + [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857). Policy M's community-membership composition extends to govern the **delivery axis** (`delivery_mode` envelope field per [§4](04_envelope.md)). The subscriber-set for any push-delivery flow IS a `community` Contribution; "subscribe = join the community"; inherits revocation, consensus, and structural-invisibility from Policy M unchanged.

**Subscriber-set composition**:

```
For a Contribution C with delivery_mode = push AND cohort_scope = community:
    subscriber_set(C) = resolve_community(C.community_id, now)
                        per §8.1.13.1 community membership resolution

The community is admitted under the standard Policy M consensus_protocol
options (founder_only / unanimous / majority / quorum:M/N / weighted /
custom) per §8.1.13.2. Subscription-specific admission semantics —
producer_gated (publisher approves each member) vs open (anyone can
join) — ride the consensus_protocol choice:

    producer_gated → consensus_protocol = "founder_only" with the
                     publisher as sole founder; the publisher authorizes
                     each new subscriber
    open           → consensus_protocol = "open:self_admit" (open-vocab
                     extension hook) where any subject signs their own
                     admission Contribution
```

**Cardinality unified across observer-share and multicast**:

| Cardinality | Per-subscriber crypto | Epoch handling |
|---|---|---|
| **N=1 (observer-share)** | Single `key_grant` ([§5.6.8.4](05_namespace.md)); no epoch needed (one recipient, one DEK) | No epoch — single grant, single DEK; revocation = `withdraws` against the grant |
| **N>1 (multicast)** | Flat per-epoch `key_grant` cascade — one grant per (subscriber, epoch); the stream-epoch DEK seals content O(1), the cascade distributes the 32-byte epoch key O(N)/epoch per [§10.5.3](10_endpoints.md) | Per-`(stream_id, epoch)` axis; epoch rolls on member removal (mandatory) + time/bytes (optional) per [§10.5.3](10_endpoints.md) D3 |

The same Policy M machinery handles both cardinalities — the difference is purely the cascade fan-out factor.

**Composition with `delivery_mode: pull` (RC1 default)**:

For `delivery_mode: pull`, subscribers discover via the standard `holds_bytes:sha256:*` directory per [§10.1](10_endpoints.md). Policy M still resolves the community for visibility-filtering (consumer reads `community_id` envelope field, walks the membership, filters out non-member peers from the discovery surface). No fan-out push; no `delivery_receipt` emission required (best-effort).

**Composition with `delivery_mode: push` (RC1 substrate-pending; live in 1.x)**:

For `delivery_mode: push`, the substrate fans out to `entitled ∧ reachable` per [§10.5.6](10_endpoints.md) D6 liveness invariant. Entitlement = Policy M membership resolution (durable, signed CEG, replicated, logged); reachability = Edge `reachability.rs` ([CIRISEdge#29](https://github.com/CIRISAI/CIRISEdge/issues/29)) node-local presence tracker (TTL sec/min, NEVER an attestation, never replicated, never logged). Missed (entitled-but-unreachable) members fall back to pull on reconnect per [§10.5.6](10_endpoints.md).

**`history_on_join` × Policy M membership additions**:

On a new-member admission via Policy M, the new member's `history_on_join` envelope value (per [§4](04_envelope.md)) determines retroactive content delivery:

- `from_join` (default) — new member receives current-epoch content forward only; no retroactive `key_grant` cascade
- `full` — new member receives `key_grant`s for prior epochs subject to the [§10.5.3](10_endpoints.md) P4 catch-up bound (`min(operator depth cap, chunk-eviction horizon)`); evicted-epoch grants return `ContentMiss` per the [`MISSION.md`](../../MISSION.md) fail-honest invariant

This composes with [§8.1.12.5](#81125-forward-secrecy-on-member-removal-option-a-recommended-for-v1) Option A forward-secrecy: removed members retain extant `key_grant`s for content they were entitled to during membership; new members may or may not get retroactive grants per `history_on_join`. The substrate's forward-secrecy posture is uniform across consent, takedown, membership-departure, AND delivery-onboarding surfaces.

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
