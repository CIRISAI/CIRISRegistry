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
| `community_id` | no (REQUIRED iff `cohort_scope == community`) | **CEG 0.8 addition.** The `community_key_id` of the community this Contribution is scoped to, per [§5.6.8.10](05_namespace.md) `community` subject_kind. One identity MAY belong to multiple communities; the field disambiguates which community's roster gates visibility. Required iff `cohort_scope == community` (substrate rejects community-scoped Contributions missing the field). Parallel to `family_id` (CEG 0.7) but with different semantics: community content DOES emit `holds_bytes:sha256:*` (federates within the cohort per [§5.6.7](05_namespace.md)); [§10.1.4](10_endpoints.md) structural-invisibility applies to self/family only. |
| `family_id` | no (REQUIRED iff `cohort_scope == family`) | **CEG 0.7 addition.** The `family_key_id` of the family this Contribution is scoped to, per [§5.6.8.9](05_namespace.md) `family` subject_kind. One identity MAY belong to multiple families ([§8.1.12](08_composition.md) Policy L); the field disambiguates which family's DEK applies and which membership roster gates visibility. Required iff `cohort_scope == family` (substrate rejects family-scoped Contributions missing the field). Composes with [§10.1.4](10_endpoints.md) structural-invisibility — `cohort_scope: self \| family` content never emits `holds_bytes:sha256:*`, so the field is only consulted in-substrate, never on the wire to non-members. |
| `subject_key_ids` | no | **CEG 0.6 addition.** List of consent-holder `key_id`s for this Contribution. Each entry MAY be a `federation_keys.key_id` OR a canonical-hash identifier (per [§4.2](#42-subject_key_ids-semantics-ceg-06) below). Each listed key has substrate-recognized authority to (a) issue `withdraws` against this Contribution (per [§3.2](03_primitives.md) broadened admission rule) and (b) emit `consent:*` dimensions about this Contribution (per [§5.6.8.6](05_namespace.md)). Default `null`/empty = no subject authority (status quo; producer-only authority). Orthogonal to `cohort_scope` AND `delivery_mode` — see [§4.2.4](#424-orthogonality-with-cohort_scope-and-delivery_mode). |
| `delivery_mode` | no | **CEG 0.10 addition.** `pull \| push`. Default `pull`. `pull` = subscribers discover via the `holds_bytes:sha256:*` directory ([§5.6.7](05_namespace.md)) + fetch via [§10.1](10_endpoints.md) `ContentFetch`. `push` = substrate fans out to the live-delivery set per [§10.5.6](10_endpoints.md) `fan_out = entitled ∩ reachable`. Composes with [§8.1.13](08_composition.md) Policy M for community-scoped delivery; with [§10.5](10_endpoints.md) streaming for `live_stream` chunk-DAG delivery. Distinct from `cohort_scope` (visibility) and `subject_key_ids` (revocability) — see [§4.2.4](#424-orthogonality-with-cohort_scope-and-delivery_mode). RC1: pull-only multicast; push tree → 1.x per CIRISRegistry#46 / #43. |
| `listed` | no | **CEG 0.10 addition.** Per-membership opt-in flag — value `public`. Default absent (roster is producer- + self-queryable, NEVER globally enumerable). Public listing mirrors the [§11.8.3](11_governance.md) location opt-in discipline: opting into roster visibility is a one-way disclosure the member chooses; substrate does NOT solicit. Composes with [§8.1.13](08_composition.md) Policy M community membership and the new [§10.5](10_endpoints.md) streaming endpoint set. |
| `history_on_join` | no | **CEG 0.10 addition.** `full \| from_join`. Default `from_join`. Per-target — names what content a new community/stream member receives at admission. `full` = Option-A retroactive catch-up (trace / registry-export backlog) per [§11.7.1](11_governance.md); `from_join` = current epoch forward only (live media). For `full` on streams: catch-up is bounded by `min(operator depth cap, chunk-eviction horizon)` per [§10.5.3](10_endpoints.md) P4; an evicted-epoch grant returns `ContentMiss` — fail-honest, no silent gap. |

**`epistemic_mode` vs `witness_relation` — distinct dimensions**: these co-vary at edges but name different concerns. `epistemic_mode` names the *process* by which the claim was formed; `witness_relation` names the *relational position* of the attester to the attested. F-3 detector attestations carry both (`epistemic_mode: derivative` + `witness_relation: derived`). Most encyclical-sourced translations are `witness_relation: external` + `epistemic_mode: hearsay`. When in doubt, set both.

## §4.2 `subject_key_ids` semantics (CEG 0.6)

Per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45). The missing half of consent at the wire format: CEG 0.x baseline encoded only **producer authority** (`attesting_key_id`); CEG 0.6 adds **subject authority** for content where the subject of the data is not the producer of the data.

### §4.2.1 The shape

`subject_key_ids` is an OPTIONAL list. When present, each entry names a party with substrate-recognized authority over this Contribution's continued processing. The basic shape:

> A consent record is a signed declaration by a subject about the substrate's continued processing of content where that subject appears.

The medical record / photo / interview / training-datum / group-chat case all share the same shape — a producer publishes a Contribution; one or more subjects appear in it; the subjects retain revocation authority. The non-medical example table at [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) walks 13 content shapes uniformly.

### §4.2.2 Federation-key vs canonical-hash identifier

`subject_key_ids[i]` MAY be either:

1. **A `federation_keys.key_id`** — the subject is a federation-enrolled identity that can sign on its own behalf. Direct revocation: subject signs a `withdraws` against this Contribution; substrate admits via rule (2) of [§3.2 broadened `withdraws` admission](03_primitives.md).
2. **A canonical-hash identifier** — the subject is an external party with no federation_keys row (Discord user-id, channel-id, content-sha256-bound entity, etc.). Format: a canonical string identifier, hashed per [§0.6](00_conformance.md) hex canonicalization. The substrate cannot verify a signature from this entity directly; **revocation rides a `delegates_to` proxy chain** per rule (3) of [§3.2 broadened admission](03_primitives.md).

Resolves [CIRISAgent#840 OQ3](https://github.com/CIRISAI/CIRISAgent/issues/840) — "self-attestation about un-enrolled parties — canonical-hash or pseudonymous federation_keys?" — with the answer "both, distinguished by whether the issuer can sign on its own behalf."

### §4.2.3 Self-as-subject ceremony

When `attesting_key_id ∈ subject_key_ids`, the Contribution is a **self-consent ceremony** — the same identity is attesting AS subject AND producer. This composes naturally with the [CIRISAgent#840](https://github.com/CIRISAI/CIRISAgent/issues/840) CEG-native agent's self-attestation pattern: agent attests `identity:current` about itself with `subject_key_ids = [self.key_id]`, asserting consent-authority over its own identity claims (D08 autonomy claim).

### §4.2.4 Orthogonality with `cohort_scope` AND `delivery_mode` (3-axis per CEG 0.10)

CEG envelope encodes **three independent envelope-level concerns** that compose without overlap:

| Axis | Field | Authority | Names |
|---|---|---|---|
| **Visibility** | `cohort_scope` + (`family_id` or `community_id` when set) | Producer-side | Who can SEE the data |
| **Revocability** | `subject_key_ids` (CEG 0.6) | Subject-side | Who can REVOKE the data |
| **Delivery** | `delivery_mode` + `listed` + `history_on_join` (CEG 0.10) | Substrate / subscriber | Who actively RECEIVES the data + how the substrate fans out |

All three may coexist on the same Contribution:

```
A `cohort_scope: family` contribution
  carrying `subject_key_ids: [user_canonical_hash]`
  carrying `delivery_mode: push` + `history_on_join: from_join`
  carrying `family_id: <acme_household>`

publishes the bytes at family-cohort visibility (cohort_scope);
the user retains revocation authority (subject_key_ids);
the substrate actively fans out to currently-reachable members
  with new-members getting forward-only content (delivery_mode + history_on_join);
the named family roster gates the membership set (family_id).
```

This orthogonality is load-bearing for the multi-occurrence consent shape (per [`MULTI_OCCURRENCE_CONSENT_ANALYSIS.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/MULTI_OCCURRENCE_CONSENT_ANALYSIS.md)): subject-side revocation applies federation-wide regardless of producer's `occurrence_id`; per-occurrence lifecycle consent is a producer-side concern, separately tracked.

The delivery axis is the **third orthogonal axis** per CEG 0.10 — CEG ≤ 0.9 encoded visibility + revocability only; the substrate had no envelope-level handle on who actively *receives* + how the substrate fans out. Per [§10.5](10_endpoints.md), 0.10 closes that gap with three new optional envelope fields + the new §10.5 endpoint section + a delivery extension to [§8.1.13](08_composition.md) Policy M.

### §4.2.5 Empty/absent = status-quo

`subject_key_ids: null` or `[]` is the status-quo shape (producer-only authority; same as all CEG ≤ 0.5 Contributions). All CEG 0.x consumers that don't read the field see status-quo behavior. CEG 0.6 is additive at the envelope layer.

### §4.2.6 Subject-bearing dimensions (governance requirement)

Per [§11.6](11_governance.md), dimensions whose namespace pattern names a subject (e.g., `observed:user:{key_id}:*`, `epistemic:about:{key_id}:*`, `consent:partnered:{user_key}`, `agent_files:*:{subject_target}`) MUST carry `subject_key_ids` containing that subject. The substrate MAY reject admission of subject-naming dimensions that omit `subject_key_ids`. This closes the default-leak failure mode where subject-bearing content publishes without wire-level subject authority.

## §4.1 Forward-compatibility rule

> **Canonical-bytes contract**: the canonical-bytes encoding of this envelope for signing follows [§0.9](00_conformance.md) (JCS over the envelope object; defaults are interpretation-time, NOT encoding-time; relay MUST preserve member presence/absence exactly as the producer signed). Optional fields with documented defaults in the table above ride the §0.9.2 omit-vs-materialize rule. Conditional-required fields (`family_id` per CEG 0.7, `community_id` per CEG 0.8) are NOT optional-with-default and substrate rejects mis-shape per [§4.2.6](#426-subject-bearing-dimensions-governance-requirement) + [§11.6.2](11_governance.md) + [§11.7.2](11_governance.md).



A Conforming Consumer (CCC per [§0.2](00_conformance.md)) that receives an envelope carrying a field-name it does not recognize MUST:

- Preserve the unknown field on read (do not strip).
- Preserve it on re-emission if the Consumer is also acting as a Producer relaying the attestation.
- NOT use it in verdict composition.
- NOT reject the envelope on the basis of the unknown field alone.

Producers introducing a new envelope field MUST follow the [§0.3](00_conformance.md) versioning rules: a new field with a documented default is a MINOR bump; a field whose absence breaks consumer semantics is a MAJOR bump.

---

[← §3 Primitives](03_primitives.md) | **§4 Envelope** | [Next: §5 Namespace →](05_namespace.md)
