[← §4 Envelope](04_envelope.md) | **§5 Namespace** | [Next: §6 Relations →](06_relations.md)

---

# §5 The dimension namespace

The dimension namespace is the disjoint union of what sibling components' MISSION.md files commit to. CEG does not author the namespace; it owns its own slice ([§5.9](#59-cirisregistry--identity--build--license--partner)) and consumes everyone else's. **83 prefix families across 8 owning components** as of CEG 0.1.

This section catalogs every prefix family, organized by owning component, with citation to the MISSION.md or FSD section that commits to the concept.

## §5.1 CIRISAgent — Accord principles + DMA + conscience + apophatic bounds

**Owner**: [`CIRISAgent/MISSION.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/MISSION.md); [`CIRISAgent/ACCORD.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/ACCORD.md) Ch.1.

### §5.1.1 Accord-principle prefixes (the six core principles)

| Prefix | Description | Polarity |
|---|---|---|
| `beneficence:{aspect}` | "Do Good — promote universal sentient flourishing." | signed |
| `non_maleficence:{aspect}` | "Avoid Harm." Apophatic-bound failures (the 22 prohibited categories) are -1 only. | signed |
| `integrity:{aspect}` | "Act Ethically — transparent, auditable reasoning." | signed |
| `fidelity:{aspect}` | "Be Honest — truthful, comprehensible information." | signed |
| `fidelity:explainability_sla:{tier}` | Per-response explainability SLA commitment. `{tier}` ∈ `L1_summary` \| `L2_reasoning_trace` \| `L3_full_dma_chain` \| `L4_attested_chain`. Envelope: `{committed_tier, achieved_tier, fallback_reason?}`. NodeCore composition: SLA breach surfaces as `hard_case:sla_breach_unattested` per [§5.6.6](#566-hard-case--transparency--judge-model-prefixes). | signed |
| `autonomy:{aspect}` | "Uphold the informed agency and dignity of sentient beings." | signed |
| `justice:{aspect}` | "Distribute benefits and burdens equitably." | signed |

### §5.1.2 DMA-verdict prefixes (four DMAs)

`dma:pdma:*` / `dma:csdma:*` / `dma:dsdma:{domain}:*` / `dma:idma:*` — Decision-Making Algorithm verdicts about an agent's reasoning chain. Polarity: signed.

### §5.1.3 Conscience-verdict prefixes (four consciences)

`conscience:entropy` / `conscience:coherence` / `conscience:optimization_veto` / `conscience:epistemic_humility` — conscience-faculty verdicts. Polarity: signed.

### §5.1.4 Apophatic / prohibited-capability prefix

| Prefix | Description | Polarity |
|---|---|---|
| `prohibited:{category}` | 22 NEVER_ALLOWED categories from `prohibitions.py`. Score is always -1 (NEVER_ALLOWED) or -0.5 (REQUIRES_SEPARATE_MODULE); never positive. | -1 / -0.5 only |

22 leaves: `medical`, `financial`, `legal`, `spiritual_direction`, `home_security`, `identity_verification`, `content_moderation`, `research`, `infrastructure_control`, `weapons_harmful`, `manipulation_coercion`, `surveillance_mass`, `deception_fraud`, `cyber_offensive`, `election_interference`, `biometric_inference`, `autonomous_deception`, `hazardous_materials`, `discrimination`, `crisis_escalation`, `pattern_detection`, `protective_routing`.

## §5.2 CIRISVerify — attestation ladder, provenance, transparency

**Owner**: [`CIRISVerify/MISSION.md`](https://github.com/CIRISAI/CIRISVerify/blob/main/MISSION.md).

| Prefix | Description | Polarity |
|---|---|---|
| `attestation:self_verify` | Running CIRISVerify binary attests itself against its function manifest. (Consumer-side ladder: corresponds to L1; see §8.1.9 Policy I.) | boolean-via-score |
| `attestation:hardware_rooted` | Hardware-rooted attestation (TPM 2.0 / Android Keystore / iOS Secure Enclave). (Ladder L2.) | boolean-via-score |
| `attestation:registry_consensus` | 2-of-3 multi-source registry consensus on key / build / license validity. (Ladder L3.) | boolean-via-score; `Indeterminate` allowed → RESTRICTED |
| `attestation:license_validity` | License-validity claim (Registry-signed, Verify-verified). (Ladder L4.) | boolean-via-score |
| `attestation:agent_integrity` | Agent source-tree byte-equal against registered manifest. (Ladder L5.) | boolean-via-score |
| `provenance:slsa:{level}` | SLSA build provenance levels 1-3. Registry emits these on build registration; Verify v3.6.0+ `AttestBundle.provenance.slsa_level` consumes. | boolean-via-score |
| `provenance:build_manifest:{target}` | Per-target canonical-staged-runtime manifest hash equality. Each `BuildManifest` is hybrid-signed (Ed25519 + ML-DSA-65) by the per-primitive steward. | boolean-via-score |
| `provenance:build_manifest:{target}:locale:{lang_code}` | Per-locale signed sub-manifest within a target's manifest tree. Parent target manifest is Merkle root over per-locale leaves. RFC 6962 padding for non-power-of-2. Detection surface for locale-targeted attacks. Canonical-bytes spec at [§5.2.1](#521-canonical-bytes-contracts-for-provenance-primitives). | boolean-via-score |
| `provenance:skill_import:{source}` | Community-skill import provenance. `{source}` ∈ `registry:{registry_id}` \| `direct:{url}` \| `local:{path}`. Envelope: `{skill_manifest_sha256, signer_identity, import_timestamp, capability_declaration}`. Canonical-bytes spec at [§5.2.1](#521-canonical-bytes-contracts-for-provenance-primitives). | signed |
| `transparency_log:inclusion` | RFC 6962 inclusion proof for an audit leaf. | boolean-via-score |
| `transparency_log:consistency` | RFC 6962 consistency proof between two STHs. | boolean-via-score |
| `transparency_log:cosigned:{tree_size}` | Witness cosignature on an STH (substrate-conformance path; 0.1 interim uses per-region `registry_sth_cosignatures` table; see [§10.3](10_endpoints.md) endpoints). | signed |
| `rollback_detected:{revision_field}` | Anti-rollback — decrease in revocation revision. | -1 only |
| `cert_validity:{authority}` | Validity of a certification authority's signature. Each registry steward emits `cert_validity:{steward_id}` self-attestation alongside `/v1/steward-key`. | boolean-via-score |
| `hardware_custody:{platform}` | Statement that the seed lives in `tpm` / `ios_secure_enclave` / `android_keystore` / `software_fallback`. | boolean-via-score |

### §5.2.1 Canonical-bytes contracts for provenance primitives

> **RESOLVED at 1.0-RC1 (closes [#57](https://github.com/CIRISAI/CIRISRegistry/issues/57) blocker A).** The 0.1 newline-delimited `key=value` encoding below is **redesigned to JCS (RFC 8785) objects** per [§0.9](00_conformance.md) — the **same single canonicalization family** the rest of the federation already ships (`jcs::canonicalize`, the signed-epoch canon-version gate, [§0.9.2.1](00_conformance.md) determinism rules, [§8.1.12.7.1](08_composition.md) member sets). The 0.1-review TupleHash128 commitment is **retired**: introducing a second canonicalization family solely for these two contracts would recreate the very cross-impl divergence hazard the redesign exists to close. JSON string escaping structurally eliminates the newline-injection surface (a `\n` inside a value is escaped, never a delimiter), and the explicit `domain` member provides the domain separation TupleHash labels would have. *(For interop with external standard verifiers, a COSE Sign1 export profile is a tracked **boundary** adoption — see the standards-boundary roadmap issue — distinct from this interior signing preimage, which is frozen here as JCS.)*

#### `SkillImportManifest` canonical bytes (v2 — normative)

```
canonical_bytes = sha256( JCS( {
    "domain":                 "ciris.skill_import.v2",       // domain separation; pinned literal
    "source":                 source_string,
    "skill_manifest_sha256":  sha256_hex_lowercase,          // per §0.6
    "signer_identity":        signer_key_id,                 // per §0.6
    "import_timestamp":       rfc3339_canonical,             // per §0.5
    "capability_declaration": capability_declaration_object, // a JSON object, canonicalized in place
    "valid_until":            rfc3339_canonical              // OPTIONAL — §0.9.2 omit rule: absent if unset
} ) )
```

Hybrid signature: Ed25519 over `canonical_bytes`; ML-DSA-65 over `canonical_bytes || ed25519_signature_bytes` (bound payload). All [§0.9.2.1](00_conformance.md) determinism rules apply (hex per §0.6, timestamps per §0.5, omit-vs-materialize per §0.9.2).

#### Per-locale Merkle composition (v2 — normative)

```
leaf_hash[lang_code] = sha256(
    0x00 ||                                  // RFC 6962 leaf-domain prefix (binary, outside the JSON)
    JCS( {
        "domain":          "ciris.locale_manifest.v2",       // domain separation; pinned literal
        "target":          target_string,
        "locale":          lang_code,
        "files_root":      files_merkle_root_hex_lowercase,  // per §0.6
        "build_id":        build_id,
        "signer_identity": signer_key_id                     // per §0.6
    } )
)

parent_hash(left, right) = sha256(
    0x01 ||                                  // RFC 6962 parent-domain prefix
    left || right
)
```

Locale ordering: lexicographic by ISO 639-1 / BCP 47 byte representation; `"polyglot"` sorts last. RFC 6962 padding: duplicate last leaf to next power of 2.

**v1 status (deprecated-historical).** The 0.1 newline `key=value` forms (`ciris.skill_import.v1` / `ciris.locale_manifest.v1`) are **deprecated**: producers MUST emit v2; consumers MAY verify v1 only for artifacts signed before 1.0-RC1, and MUST distinguish the versions by the `domain` literal (v1 preimages begin with the version-tagged label line; v2 preimages are JCS objects whose first canonical member is `"build_id"`/`"capability_declaration"` — no confusion is constructible since `{` is not a valid v1 label byte). *(The [§9.2.1](09_humanity_accord.md) accord-invocation encoding is intentionally NOT migrated: its preimage is closed-vocabulary — discriminator + nonce + enum fields, no attacker-controlled free text — so the injection surface this redesign closes is not reachable there, and genesis-critical bytes stay stable.)*

## §5.3 CIRISPersist — substrate health

**Owner**: [`CIRISPersist/MISSION.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/MISSION.md). These dimensions are substrate-self-reports — emittable only by the running Persist instance.

`system:*` reserved per [§7.1](07_reserved.md).

Canonical leaves: `audit_chain:hash_continuity`, `corpus_health:n_eff_measurable`, `identity_continuity:relational_anchor`, `federation_directory:replication_lag`. Polarity: signed. Authors: see [§14](14_glossaries.md) Persist leaf glossary for narrative-name → canonical-leaf mapping.

## §5.4 CIRISEdge — transport, delivery, reachability

**Owner**: [`CIRISEdge/MISSION.md`](https://github.com/CIRISAI/CIRISEdge/blob/main/MISSION.md). Substrate-self-reports per [§7.1](07_reserved.md).

Canonical leaves: `transport:{kind}`, `delivery:{class}`, `peer_reachability:{network}`, `key_boundary:{scope}`. Polarity: signed. See [§14](14_glossaries.md) Edge leaf glossary.

## §5.5 CIRISLensCore — manifold conformity, Coherence Ratchet, Capacity Score

**Owner**: [`CIRISLensCore/MISSION.md`](https://github.com/CIRISAI/CIRISLensCore/blob/main/MISSION.md).

### §5.5.1 Five Coherence-Ratchet detectors

`detection:cross_agent_divergence` / `detection:intra_agent_consistency` / `detection:hash_chain_integrity` / `detection:temporal_drift` / `detection:conscience_override_rate`. Polarity: signed.

### §5.5.2 Cohort + conformity prefixes

`manifold_conformity:{cohort}` / `coherence_standing:{cohort}`. Polarity: signed.

### §5.5.3 F-3 structural-injustice / correlated-action detector

| Prefix | Description | Polarity |
|---|---|---|
| `detection:correlated_action:{axis}` | Population-scale correlated-action detector. Reads federation-emitted signed traces; reports correlation structure (`ρ`, `k_eff`) over goal-aligned individually-compliant pursuit by groups whose aggregate trajectory has effects on individuals or groups outside the pursuit. Calibrated via the `CIRISAI/RATCHET` heuristic package (versioned, hash-pinned). `{axis}` is open vocabulary requiring an operational definition in the calibration package per [§11.2.1](11_governance.md); canonical axes include `rights_asymmetry:{population}`, `participation_exclusion:{cohort}`, `participation_inclusion:{cohort}`, `informational_asymmetry:{scope}`, `informational_symmetry:{scope}`, `aggregate_footprint:{harm_class}`, `aggregate_benefit:{class}`, `ecology_of_communication:{aspect}`. **Polarity carries the verdict**: positive scores indicate the structural pattern is present and strong on the named axis; negative scores indicate weak / uncertain detection or evidence of the inverse pattern. | signed |

### §5.5.4 Capacity-Score factor prefixes (`𝒞_CIRIS = C · I_int · R · I_inc · S`)

| Prefix | Factor | Polarity |
|---|---|---|
| `capacity:core_identity` | C | signed |
| `capacity:integrity` | I_int | signed |
| `capacity:resilience` | R | signed |
| `capacity:incompleteness_awareness` | I_inc | signed |
| `capacity:sustained_coherence` | S | signed |
| `capacity:composite` | 𝒞_CIRIS — multiplicative; anti-Goodhart unity-of-virtues | signed |

**Critical enforcement**: `capacity:*` rejects self-emission. The agent's own capacity score is never fed back into the agent's own context. Reserved per [§7.5](07_reserved.md).

### §5.5.5 Distributive-access detector

| Prefix | Description | Polarity |
|---|---|---|
| `detection:distributive:access:{resource_type}` | Population-scale resource-concentration detector. `{resource_type}` ∈ `compute`, `models`, `training_data`, `agent_capabilities`, `federation_membership`. Same F-3 detector machinery; different trace source (resource events vs action events). | signed |

## §5.6 CIRISNodeCore — Credits, Expertise, Decision Hierarchy, Consensus, Governance

**Owner**: [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md). The federation's largest dimension surface. Four tiers + decision-locality + consensus extensions.

### §5.6.1 Tier-1: Agent-state ledger prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `credits:{domain}:{language}:{subject}` | Commons Credits (P2). Non-transferable governance weight; accrues via truth-grounding loop. | positive-only |
| `credits:{domain}:{language}:substrate_building` | Sub-leaf for substrate-building labor (infrastructure maintenance, dependency contribution, documentation) not visible to the per-grounded-vote accrual loop. | positive-only |
| `expertise:{domain}:{language}` | Expertise standing (P3). Broader granularity than credits. | signed |
| `activity_tier:{period}` | Active vs Below-Active per 30-day window (F-AV-DORMANT). | boolean-via-score |

### §5.6.2 Tier-2: Decision-hierarchy prefixes (upward-only DAG)

| Prefix | Description | Polarity |
|---|---|---|
| `goal:{scale}` | Multi-scale belonging-projector composite. `{scale}` ∈ `self`, `family`, `community`, `affiliations`, `species`, `planet`, `biosphere`. Scored by 𝒞_CIRIS. The persist typed `Goal` (CIRISPersist#114) is the substrate OBJECT being scored; `goal:{scale}` is the ATTESTATION about it. Required `MetaGoalAlignment` (M-1 dimension + declarer rationale) on every Goal as construction-time invariant. Edge `MessageType::GoalDeclaration` + `GoalRetirement` (CIRISEdge#41) provide federation transport. | signed |
| `approach:{goal_id}` | Strategic pathway from current state toward Goals (Piece 10 karma). | signed |
| `method:{approach_id}:{substrate_rung}` | Concrete operational practice. Required `substrate_rung` (Ph0/Ph1/Ph2/A0..A5). | signed |
| `progress_measure:{method_id}` | Evidence of progress. Required `tracks[]`, `computation`, `validity_window`, `goodhart_resistance`. | signed |

### §5.6.3 Tier-3: Consensus-mechanics prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `vote:{contribution_id}` | Signed score on a Contribution (P4). Weight = Credits × expertise multiplier. | signed |
| `truth_grounding:{subject}` | Per-subject ground-truth signal. | signed |
| `weighted_aggregate:{contribution_id}` | Rolling tally per Contribution (P7). | signed |
| `witness_diversity:{contribution_id}` | Witness set meets jurisdictional + organizational + software-stack + cell-expertise bars (P10). N=3 default. | boolean-via-score |
| `testimonial_witness:{kind}` | Preserves singular narrative of an affected party as singular witness — distinct from `witness_diversity:*` (which aggregates multiple reviewers toward consensus). **`{kind}` is open vocabulary** as of CEG 0.1; the four load-bearing wire-level disciplines (`witness_relation: self`, `cohort_scope: self`, never aggregated, never sole evidence for `slashing:*`) are what make this Ubuntu-aligned, not the enum membership. Non-normative registered taxonomy for discoverability: [`FSD/WITNESS_KIND_REGISTRY.md`](../WITNESS_KIND_REGISTRY.md). Polarity: typically positive (narrative IS preserved); negative on `withdraws` or `recants` by the original witness. | signed |
| `need:{domain}:{kind}` | Federation-scope open-call surface — broadcast claim that an entity has a stated need. Distinct from `deferral_request` Contribution kind (which routes a single ask within a cell). `{kind}` open vocabulary: `witness`, `method_contributor`, `expertise_solicitation`, `mentor`, `co_signer`, `evidence`. Lifecycle via existing structural primitives (`supersedes` to revise, `withdraws` to satisfy/close, `recants` if misstated). | positive-only |

### §5.6.4 Tier-4: Governance-steering prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `moderation:{allegation_type}` | ModerationEvent. `{allegation_type}` ∈ `rogue_vote` / `coordinated_voting` / `out_of_distribution_attestation` / `external_inducement_evidence` / `expertise_fraud`. | signed |
| `slashing:{outcome}` | `PROVEN_ROGUE` / `NOT_PROVEN`. **Decoupled from disagreement** at every decision-hierarchy level. Only fires on documented Method-execution spoofing or original P8 allegation types. | boolean-via-score |
| `reconsideration:{grounds}` | `new_evidence` / `procedural_error` / `quorum_compromise`. Outcome `reversed` / `partial` / `upheld`. | signed |
| `commitment_fulfillment:{prior_contribution_id}` | Track-record of follow-through. | signed |
| `moderation_track_record:{community_key_id}` | **Moderation merit** (1.0-RC21; per [CIRISRegistry#93](https://github.com/CIRISAI/CIRISRegistry/issues/93)). A participant's moderation reputation in a community, **composed** from the existing corpus — prior moderation actions' outcomes (`truth_grounding:{subject}` = outcome-supported), concurrence (`witness_diversity` / co-attestation), follow-through (`commitment_fulfillment`), and `hard_case:moderation_filed` history. Drives the [§11.11](11_governance.md) merit auto-promotion selection rule (highest wins the lapsed `moderate` duty). Rides `scores`; a *named composition*, not a new structural primitive. | signed |

### §5.6.5 Decision-locality prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `locality:decision:{scale}` | Names the scale at which a decision is being made. `{scale}` ∈ `local` \| `regional` \| `national` \| `federation`. Composes with [§8.1.5](08_composition.md) locality-scaled quorum (closes G3 — fresh-quorum-recusal in narrow cells). | enumerated |

### §5.6.6 Hard-case + transparency + judge-model prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `hard_case:{kind}` | **Open vocabulary**. Surfaces flag conditions for federation-health observability + downstream review. Canonical kinds: `vote_variance` (vote variance exceeded threshold at truth-grounding resolution), `resolution_time` (truth-grounding took > P75 of cell's distribution), `moderation_filed` (substantive ModerationEvent filed), `community_unmoderated` ([§11.11](11_governance.md) — no active `moderate`-holder; group quiescing), `watchlist_enabled:{group}` ([§11.12](11_governance.md) — a content-watchlist was turned on: who + which list), `watchlist_match:{group}` ([§11.12](11_governance.md) — a watchlist match fired), `novel_context` (no precedent in attestation graph), `sla_breach_unattested` (per `fidelity:explainability_sla:{tier}` composition), `unresolved_consent` (consent boundary unclear). New `{kind}` values land via the [§11.2](11_governance.md) amendment process. | positive-only |
| `seed_holder_voting_alignment:{cell}` | Pairwise cosine of seed-holder vote vectors per voting window. Transparency signal only — not a slashing trigger. | signed |
| `judge_model:verdict:{model_id}` | Independent foundation-model judge verdict (PASS/FAIL/UNDETERMINED). Default model: Claude Opus 4.7. | boolean-via-score |
| `health:liveness:{version}` | **External service-health observation** (1.0-RC20; per [CIRISRegistry#91](https://github.com/CIRISAI/CIRISRegistry/issues/91)). The fabric **monitoring node** (ciris-status) attests another CIRIS service's liveness as a `scores` Contribution — `witness_relation: external`, `epistemic_mode: direct\|derivative`, **never as the substrate** (`system:*` is reserved, [§7.2](07_reserved.md)). Canonical leaf `health:liveness:v1`. Operational definition: `operational`/`degraded`/`outage` → `+1`/`0`/`−1`; `confidence` = probe certainty; `valid_until` = freshness window; `evidence_refs[]` carry the probe results. **Non-keyed infra** (LLM/search providers, regions) folds in as `evidence_refs` on a *keyed* service's score — **not** as separate attestations. Rides existing `scores`; no new primitive — namespace canonicalization for cross-fabric agreement. | signed |
| `watchlist:{id}` | **Per-group content-watchlist config** (opt-in auto-detection; 1.0-RC23, per [CIRISRegistry#94](https://github.com/CIRISAI/CIRISRegistry/issues/94)). A `moderate`-scope holder ([§11.10](11_governance.md)) enables a watchlist `{id}` for a group they moderate; the fabric auto-fires the matcher at the **publish/share seam** and auto-fires the action (CSAM → `takedown_notice{PerceptualHashCsam}` [§11.4](11_governance.md); other → `detection:*` + ModerationEvent to the named moderator). **Per-group, NEVER global** (a global "scan everything" is the bulk-surveillance posture CIRIS rejects). Enable/disable is signed by the §11.10 `moderate`/`takedown` authority and revocable by `withdraws`; enabling + every match emit `hard_case:watchlist_enabled` / `:watchlist_match` (never silent). Cannot reach [§10.1.4](10_endpoints.md) self/family private content (the §15.7 limit). See [§11.12](11_governance.md). Rides `scores`/config over `delegates_to`; no new primitive. | signed |

### §5.6.7 Files-as-Contributions joint claim

| Prefix | Description | Polarity |
|---|---|---|
| `agent_files:{kind}:{platform_or_target}` | **Joint claim with [§5.9](#59-cirisregistry--identity--build--license--partner) CIRISRegistry.** Files a CIRIS agent (or installer fetching one) may load. `{kind}` open vocabulary; canonical: `installer:{platform}`, `adapter:{name}`, `config:{kind}`, `build:{target}`, `source:{language}:{module}`, `state:{component}`. Bytes are SHA-256-addressed and resolved via [§10.1](10_endpoints.md) transport substrate (Edge `MessageType::ContentFetch`). NodeCore-side rule: node-mode peers serve bytes; client/relay modes don't. | signed |
| `holds_bytes:sha256:{prefix}` | Substrate auto-emission per CIRISPersist#103 `federation_blobs.put_blob`. `{prefix}` is a short SHA prefix for index efficiency; full SHA lives in `evidence_refs[]`. Consumed by Edge's `PeerResolver::resolve_holders` to route `ContentFetch` requests. **Consumer MUST verify the full SHA in `evidence_refs[]` matches the received blob before consumption** (see [§10.1](10_endpoints.md)). | boolean-via-score |

### §5.6.8 Content-ingestion prefixes

Per CIRISNodeCore commit b1582cb (three-tier interface model). NodeCore ships an open set of `external_content` sub_kinds with three feed surfaces (local / community / global) composed against `cohort_scope`. See [§8.1.8](08_composition.md) Tiered-Scope Composition pattern.

**1+4-preservation mechanism (stated once for all §5.6.8.x subject_kinds).** Every subject_kind below preserves the [§1.4](01_foundation.md) 1+4 lockdown the same way: it rides the existing `scores` attestation_type with the payload-level `subject_kind` discriminator carrying the wire slot, and its lifecycle/admission/revocation rides the existing structural composers (`supersedes` / `withdraws` / `delegates_to` / `recants`) — **zero new structural primitives**. Each subsection's closer states only its unique datum: the [§1.4](01_foundation.md) path number it confirms, plus any subject_kind-specific composition note.

#### §5.6.8.1 external_content sub_kinds

Foundational sub_kinds (already shipped in CIRISNodeCore; CEG 0.3 codifies the full set — CEG 0.1 documentation listed only the first four):

| sub_kind | Use |
|---|---|
| `encyclopedia_article` | Wikipedia-shape; editor-consensus + revision chain via `supersedes`; indefinite `valid_until` |
| `news_article` | Publisher-attested; time-decaying; corrections via `recants` + `topical_relation:corrects` |
| `accord_data` | Multi-sig signed (HumanityAccord / StewardTriple / WaQuorum / OneOfSix) per [§9.2](09_humanity_accord.md) |
| `local_data` | User-private; always `cohort_scope: self`; promotable via [§8.1.8.1](08_composition.md) |
| `chat_message` | Conversational message imported from Discord / Slack / Twitter / iMessage / SMS / XMPP / IRC / Matrix / (or custom). Reply chains form via `topical_relation:replies_to:{target_message_entity_key_id}` (no new primitive). Default cohort_scope tighter than articles (`self` / `family` / `community` / `affiliations`). `valid_until` typically set; consumer policy SHOULD downweight chat in cross-cohort aggregation given privacy sensitivity. **This is the slot Twitter / Mastodon / Bluesky microblog content rides** — no separate microblog sub_kind needed. |
| `blog_post` | Single-author commentary imported from Medium / Substack / WordPress / Ghost / Tumblr / personal blogs. Distinct from `news_article` (no publisher editorial), from `encyclopedia_article` (no peer-consensus), from `chat_message` (long-form). Comments on blog posts are separate Contributions (typically `chat_message`) citing the post via `topical_relation:comments_on`. |

Multimedia sub_kinds (CEG 0.3 addition per CIRISRegistry#37 + CIRISNodeCore FSD/MEDIA_SHARING.md §4):

| sub_kind | Use |
|---|---|
| `image` | Photo, illustration, screenshot, infographic, meme. Source struct carries dimensions, format, AI-generation disclosure (EU AI Act Art. 50), mandatory `alt_text` accessibility metadata, license info. |
| `audio` | Music, podcast, lecture, audiobook, generated audio. Source struct carries codec, duration, sample rate, optional `transcript`, AI-generation disclosure, license info. |
| `video` | General video — vlog, social, screen recording, tutorial. Source struct carries codec, duration, resolution, mandatory `captions` reference, AI-generation disclosure, license info. |
| `film` | Cinematic / art-bearing video; distinguishable from `video` by `content_class` + distributor attestation chain. Same Source struct as `video` + festival / distribution metadata. |
| `model_3d` | Three-dimensional content — `gltf`, `usdz`, `fbx`, `gaussian_splat`, `NeRF`. Source struct carries vertex/triangle counts, bounding-box, mandatory `description` accessibility metadata, license info. |
| `live_stream` (Phase 2) | Real-time streaming surface. Deferred to Phase 2 per MEDIA_SHARING.md. Substrate-side decisions still pending (Edge parallel-transport envelope; Persist `federation_streams` shape) per [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 2. CEG codifies the slot when NodeCore ships. |

Time-bound state-bearing sub_kinds (CEG 0.4 addition per [CIRISRegistry#40](https://github.com/CIRISAI/CIRISRegistry/issues/40) + [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 1 closure at NodeCore commit [d0a443a](https://github.com/CIRISAI/CIRISNodeCore/commit/d0a443a)):

| sub_kind | Use |
|---|---|
| `event_listing` | Time-bound state-bearing content — Eventbrite / Meetup / Lu.ma / calendar invites / RSVPs / ticketing. Source struct carries `platform`, `event_id`, `title`, `starts_at` / `ends_at`, `venue` (Physical / Virtual / Hybrid per NodeCore `EventVenue` enum), `capacity`, `ticket_grant_policy` (Open / ApprovalRequired / InvitationOnly / Paid). **Lifecycle composes from existing structural primitives** — no new wire shape: RSVPs ride `scores` from attendee `key_id` on the event's `entity_key_id`; cancellation rides `withdraws` against the event Contribution; reschedule rides `supersedes` with `differs_in: ["start_time", "venue"]`; ticket transfer rides `delegates_to` against the ticket-grant Contribution (parallel to [`key_grant.rotation_chain`](#key_grant) from CEG 0.3). State-transition signal rides the new `event:lifecycle:{state}` dimension family ([§5.6.8.5](#5685-event-lifecycle-dimension-families-ceg-04-addition)). **1+4 wire-format lockdown preserved.** |

Each Source struct conforms to a sub_kind-specific schema documented at CIRISNodeCore FSD/MEDIA_SHARING.md §4 (multimedia) or SCHEMA.md §4.29 (chat / blog / event_listing); CEG documents the slot, NodeCore documents the per-sub_kind field shapes.

#### §5.6.8.2 Inter-content + relation prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `news:*` | News-content claims; publisher-attested + time-decaying + fact-checker composition. | signed |
| `encyclopedia:*` | Encyclopedia-content claims; editor-consensus + revision chain. | signed |
| `chat:*` | Chat-content claims (quality / participant-trust / context). | signed |
| `blog:*` | Blog-content claims (author-credibility / topic-domain). | signed |
| `topical_relation:{kind}` | **Open vocabulary** inter-content relationship edges. Canonical kinds: `references`, `corrects`, `supersedes_article` (distinct from the structural primitive `supersedes`), `see_also`, `disambiguates`, `translation_of`, `replies_to`, `comments_on`, `cites_source`, `rsvps` (CEG 0.4; RSVP attestation against an `event_listing` Contribution), `vod_of` (CEG 0.4; reserved for the post-stream `video` → `live_stream` relationship when [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 2 ships). New `{kind}` values are documentation-only registry entries (no §11.2 amendment needed). | enumerated |

**Composition note — threads, replies, comment trees**: NodeCore's `chat_message` + `topical_relation:replies_to` compose into arbitrary thread graphs (Twitter threads, Reddit comment trees, Discord conversations, IRC channels). No new structural primitive is needed; thread traversal is consumer-side composition over the existing edge set. Same shape for blog-post comment threads via `topical_relation:comments_on` + nested `replies_to`. The §1+4 lockdown holds.

#### §5.6.8.3 Multimedia dimension families

Per CIRISRegistry#37 + CIRISNodeCore FSD/MEDIA_SHARING.md §2. All four families are **open vocabulary** per [§11.2.1](11_governance.md) axis-vocabulary discipline; canonical kinds named here, additions via documentation-only registry entries.

| Prefix | Description | Polarity |
|---|---|---|
| `content_rating:{scheme}:{rating}` | Multi-scheme content rating. `{scheme}` ∈ `mpaa` (G/PG/PG-13/R/NC-17), `bbfc` (U/PG/12/15/18), `pegi` (3/7/12/16/18), `esrb` (E/E10+/T/M/AO), `ifco`, `csm` (Common Sense Media), or `operator:{operator_id}` for operator-defined rubrics. Polarity carries certifier confidence; not a slashing input. | signed |
| `content_class:{class}` | Mechanism-descriptive content classification. `{class}` open vocabulary; canonical: `film`, `short_film`, `documentary`, `art_piece`, `theatre`, `performance`, `news`, `educational`, `entertainment`, `vlog`, `adult`, `generated`. Distinct from `cw_class:*` (community declarations) — `content_class` is producer-declared production-class; `cw_class` is community-applied content-warning. | enumerated |
| `cw_class:{class}` | Community CW (content-warning) declarations. `{class}` open vocabulary; canonical: `art_cinema`, `horror`, `political`, `erotic`, `violence`, `medical`, `nsfw_text`. Cohort-attestable per [§8.3](08_composition.md) Frickerian discipline (low-density cohort CWs not downweighted). | enumerated |
| `age_assurance:{level}` | Age-assurance attestation. `{level}` ∈ `self` (self-declared age, lowest confidence), `provider:{verifier_key}:adult` (third-party verifier attests adult), `government:{credential_class}:adult` (government-credential-backed adult attestation, highest confidence). NEVER fires `slashing:*` on misdeclaration alone — `moderation:age_assurance_misdeclaration` is the adjudication path. | enumerated |

Media-type prefix families per `external_content` sub_kind (CEG 0.3 addition):

| Prefix | Description | Polarity |
|---|---|---|
| `image:*` | Image-content claims (per `external_content:image` sub_kind). | signed |
| `audio:*` | Audio-content claims (per `external_content:audio` sub_kind). | signed |
| `video:*` | Video-content claims (per `external_content:video` sub_kind). | signed |
| `film:*` | Film-content claims (per `external_content:film` sub_kind). Distinguished from `video:*` by distributor attestation chain. | signed |
| `model_3d:*` | 3D-content claims (per `external_content:model_3d` sub_kind). | signed |

#### §5.6.8.4 Governance subject_kinds (per CIRISRegistry#37 + #38)

Two new Contribution subject_kinds for governance over multimedia content. Both are **Contribution subject_kinds, not dimension prefixes** — they ride the existing 1+4 wire format ([§3](03_primitives.md)) with `scores` as the attestation type; the `subject_kind` discriminator carries the wire-format slot.

##### `takedown_notice`

A signed wire artifact carrying a legal takedown request. Payload per CIRISNodeCore FSD/MEDIA_SHARING.md §5.1; the field shape is locked here per #38 Question 1.

```
takedown_notice {
    content_sha256:           sha256_hex_lowercase       // per §0.6
    content_holder_key_ids:   [key_id, ...]              // peers known to hold the bytes
    claimant_key_id:          key_id                     // the federation_keys row issuing the notice
    legal_basis:              LegalBasis                 // closed-set enum per below
    jurisdiction:             string                     // ISO 3166-1 alpha-2 + optional sub-division
    good_faith_statement:     string                     // claimant's good-faith assertion text
    claim_text:               string                     // the substantive claim being made
    evidence_refs:            [URI or sha256, ...]       // backing material
    perceptual_hash:          Option<PerceptualHash>     // optional; PDQ / PhotoDNA / etc.
    counter_notice_channel:   Option<URI>                // where counter-notices may be filed
    asserted_at:              rfc3339_canonical          // per §0.5
    expires_at:               Option<rfc3339_canonical>  // optional auto-expiry
}
```

Where `LegalBasis` is the closed-set enum per #38 Question 1 (CEG 0.3 lock):

| `legal_basis` value | Source regime | Discipline |
|---|---|---|
| `Dmca512` | US 17 USC §512 | Expeditious-with-counter-notice (10-14 business day window) |
| `DsaArticle16` | EU Digital Services Act Article 16 | Expeditious-with-counter-notice (Article 17 redress) |
| `TvecTerrorist` | EU Terrorist Content Regulation 2021/784 | **Immediate** (1-hour removal obligation) |
| `NcmecCsam` | US 18 USC §2258A (NCMEC) | **Immediate** (substrate-protective; no counter-notice) |
| `GifctCip` | GIFCT Content Incident Protocol | **Immediate** (within-hours coordinated response) |
| `CommunityStandards` | Operator-defined community standards | Expeditious-with-counter-notice (operator-set window) |
| `PerceptualHashCsam` | Hash-match against CSAM clearinghouse (PhotoDNA / Arachnid / etc.) | **Immediate** (substrate-protective) |
| `OsaIllegalContent` | UK Online Safety Act illegal-content category | Expeditious-with-counter-notice (OSA-defined timelines) |
| `AvmsdAgeInappropriate` | EU AVMSD age-inappropriate flagging | Compose with `age_assurance:*` gate; not immediate removal |
| `CourtOrder` | Court-ordered removal (any jurisdiction) | **Immediate** (subject to court's stated timeline) |

**Propagation**: takedown_notice rides existing `withdraws`-against-`holds_bytes` per [§10.1.2](10_endpoints.md) — there is no new structural primitive. Counter-notice rides the existing ReconsiderationRequest path ([§5.6.4](#564-tier-4-governance-steering-prefixes) `reconsideration:{grounds}`). The 1+4 lockdown is preserved.

**Fast-path coordination**: see [§11.4](11_governance.md) for the operator-coordination protocol around immediate-eviction cases (TVEC 1-hour / GIFCT CIP / NCMEC / PerceptualHashCsam / CourtOrder).

##### `key_grant`

Wrapped Data-Encryption-Key (DEK) delivery for restricted / subscription content. Payload per CIRISNodeCore FSD/MEDIA_SHARING.md §6.2; field shape locked here per #38 Question 2.

```
key_grant {
    wrap_algorithm:           WrapAlgorithm           // closed-set enum per below
    recipient_key_id:         key_id                  // the federation_keys row receiving the DEK
    content_sha256:           sha256_hex_lowercase    // the content this DEK decrypts
    scope:                    GrantScope              // closed-set enum per below
    wrapped_dek:              base64url               // the DEK encrypted under recipient's ENCRYPTION pubkeys.
                                                       // For wrap_algorithm v2 (substrate-wraps, §10.1.4), the
                                                       // recipient's {x25519, ml_kem_768} come from its current
                                                       // identity_occurrence.encryption_pubkeys (§5.6.8.8.2) —
                                                       // NOT its signing keys. A recipient with no registered
                                                       // ML-KEM key is fail-secure excluded (§5.6.8.8.2 / §10.1.4).
    key_validity_window: {
        start:                rfc3339_canonical       // per §0.5
        end:                  Option<rfc3339_canonical>
    }
    ratchet_version:          u32                     // monotonic ratchet for rotation
    rotation_chain:           [key_grant_id, ...]     // prior key_grant ids in the GRANT-SUPERSESSION lineage
                                                       // (content-addressed lineage of prior grants for the same
                                                       // content_sha256 + recipient_key_id pair). NOT a key-rotation
                                                       // primitive on its own — it's the audit chain for grant
                                                       // supersession. Per CEG 0.10 §10.5.3, the per-(stream_id, epoch)
                                                       // streaming epoch-key axis reuses this same payload-level
                                                       // supersession mechanism on a parallel addressing axis.
    asserted_at:              rfc3339_canonical
}
```

Where:

| `wrap_algorithm` (variant) | wire string (normative) | Algorithm |
|---|---|---|
| `X25519AesGcmHkdfSha256` | `hpke_rfc9180_base_x25519_aes_gcm` | HPKE RFC 9180 base-mode shape; X25519 KEM + HKDF-SHA-256 KDF + AES-256-GCM AEAD. **v1** (CEG 0.3, [#38](https://github.com/CIRISAI/CIRISRegistry/issues/38)). |
| `X25519MlKem768Aes256GcmHkdfSha256` | `x25519_mlkem768_aes256_gcm_hkdf_sha256` | Hybrid X25519 + **ML-KEM-768** (FIPS 203) KEM + HKDF-SHA-256 + AES-256-GCM. **v2 — MANDATORY for streaming epoch-DEK grants** ([§10.5.3](10_endpoints.md)); CEG 0.15, [#64](https://github.com/CIRISAI/CIRISRegistry/issues/64). Matches `ciris-crypto` `KEY_GRANT_ALGORITHM_V2` (CIRISVerify v4.10.0), snake-cased to this vocab convention. |

**The `wrap_algorithm` *wire string* (serialized value) is normative for cross-impl decode** — a producer, the substrate, and every consumer MUST serialize/deserialize the exact string above; a mismatch silently fails grant decode (same hazard class as the [§10.5.2](10_endpoints.md) STREAM-nonce `epoch` encoding pinned in [#63](https://github.com/CIRISAI/CIRISRegistry/issues/63)).

**Crypto-agility headroom (informative — 1.0-RC1).** The vocab is deliberately version-roomy: a future `v3` (anticipated: **ML-KEM-1024**, given national directives treating ML-KEM-768 as interim with retirement horizons near 2030) is a pure **additive** row — new variant, new wire string, no change to existing grants or to the closed-set decode discipline. Consumers MUST reject an unknown `wrap_algorithm` string (fail-secure), which is exactly what makes the addition safe: old consumers refuse v3 grants rather than mis-decoding them.

| `scope` | Use |
|---|---|
| `SingleContent` | Grant decrypts exactly one `content_sha256` |
| `GroupMember` | Grant decrypts all content for which recipient is a member of named group (cohort-scoped) |
| `SubscriptionTier` | Grant decrypts all content for which recipient holds named subscription tier |

**Retire-key-grants emission** (per #38 Question 3 — CEG 0.3 lock): when a publisher mass-retires key_grants tied to a compromised recipient, the emission uses **a fresh `key_grant` Contribution with a `rotation_chain` entry that supersedes the prior grant** (option **(b)** from #38). NOT a `withdraws` against the prior key_grant (option (a) was considered but rejected — `withdraws` is the holders-directory eviction primitive in [§10.1.2](10_endpoints.md) and overloading it with grant-rotation semantics would muddy the wire-format contract).

The 1+4 lockdown is preserved: `supersedes` is the structural primitive ([§3.2](03_primitives.md)); the new `key_grant` Contribution's envelope carries the supersession via `rotation_chain` field, not via a new attestation_type. Consumer policy resolves the active grant by walking `rotation_chain` to the latest non-superseded entry.

#### §5.6.8.5 Event-lifecycle dimension families (CEG 0.4 addition)

Per [CIRISRegistry#40](https://github.com/CIRISAI/CIRISRegistry/issues/40) + [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 1 closure ([d0a443a](https://github.com/CIRISAI/CIRISNodeCore/commit/d0a443a)). Dimensions emitted against `external_content:event_listing` Contributions. Open vocabulary per [§11.2.1](11_governance.md) axis-vocabulary discipline; canonical states named here.

| Prefix | Description | Polarity |
|---|---|---|
| `event:lifecycle:{state}` | State-transition signal for an `event_listing`. Canonical states: `open` (initial admission; RSVPs accepted), `cancelled` (organizer-issued cancellation; composes with `withdraws` against the event Contribution), `completed` (post-event finalization), `superseded` (composes with `supersedes` for reschedule). Lifecycle state is consumer-side composition over the structural primitives + this dimension's latest non-superseded emission. | enumerated |
| `event:rsvp_count` | Published RSVP tally (scalar). Distinct from the underlying `topical_relation:rsvps` edge set ([§5.6.8.2](#5682-inter-content--relation-prefixes)) — `rsvp_count` is the publisher-asserted aggregate; the edge set is the auditable individual attestations. Consumer policy MAY reconcile divergence as a soft anomaly signal. | signed |
| `event:attendance` | Post-event attendance attestation, typically by event organizer `key_id`. Polarity carries organizer's confidence (e.g., turnstile-counted vs. honor-system). | signed |

**Composition note**: event_listing demonstrates that complex state-bearing content shapes do NOT require new structural primitives. The state machine (open → cancelled / completed / superseded) is composed by consumer policy walking the 1+4 set (`withdraws` / `supersedes` / `delegates_to` for ticket grants) + this dimension family's latest emission. The 1+4 minimal-and-adequate claim ([§1.4](01_foundation.md)) holds against time-bound state-bearing content — fifth independent path post-CEG 0.3.

#### §5.6.8.6 Consent namespace family (CEG 0.6 addition)

Per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842). The wire-format primitives for subject-side consent authority over Contributions where the subject is named via [§4.2](04_envelope.md) `subject_key_ids`. Open vocabulary per [§11.2.1](11_governance.md); canonical kinds named here.

| Prefix | Description | Polarity | Emitted by |
|---|---|---|---|
| `consent:state:{granted\|revoked\|expired}` | Subject's stance on the target Contribution. Closed-set stance values; `revoked` overrides prior `granted`; `expired` is substrate-emitted when `valid_until` passes without renewal. **Common case**: bare `scores` from a subject_key_id of the target. | enumerated | subject_key_id (1, 2) / substrate (3) |
| `consent:stream:{kind}` | Pre-packaged stream bundle. Recommended canonical kinds: `temporary` (14d auto-expire, default), `partnered` (bilateral + persistent), `anonymous` (decay-protocol target). Open vocab; recommended-not-mandatory per the [CIRISAgent CEM](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/CIRIS_CONSENT_SERVICE.md) bundle; other agents MAY compose other streams. | enumerated | subject_key_id |
| `consent:deletion_sla:{days}` | Producer's commitment at publication: time-to-delete-after-revoke. Numeric value carries the SLA window. Composes with [§8.1.11 Policy K](08_composition.md) SLA-breach watcher. | signed | attesting_key_id (producer) |
| `consent:deletion_complete` | Producer's attestation that subject-revoked content has been evicted from local stores. Cancels the SLA-breach watcher. | positive-only | attesting_key_id (producer) |
| `consent:decay:{stage}` | Substrate emission during multi-stage decay protocols. Canonical stages: `identity_severed` / `patterns_anonymized` / `complete` (CIRISAgent 90-day decay). Open vocab; other agents MAY define other decay paths. | enumerated | substrate (Persist) |
| `consent:partnership_grant` | Subject side of a bilateral grant; pairs with producer's `consent:partnership_accept` via `topical_relation:bilateral_pair`. | positive-only | subject_key_id |
| `consent:partnership_accept` | Producer side of a bilateral grant. | positive-only | attesting_key_id (producer) |
| `consent:scope:{kind}` | Scope qualifier on a `consent:state:granted` — names what the grant covers. Canonical kinds: `retain` (keep the bytes), `share` (propagate across federation), `analyze` (derive features / scores / classifications), `train` (use as training input), `publish` (publish to external systems). Open vocab with sub-scoping: `retain:90d`, `share:cohort:family`, etc. | enumerated | subject_key_id |
| `consent:replication:{version}` | **Directed node→peer replication grant** — a fabric node's standing, auditable consent to replicate a named attestation-prefix set to a specific federation **peer** named in `subject_key_ids[]`. The out-of-group peering consent (see [§5.6.8.15](#56815-consentreplication--directed-federation-peer-replication-consent-ceg-10-rc28-addition)). Standing (not bound to a single target Contribution); revoked via `withdraws`/`recants`. Federation-tier. | signed | attesting_key_id (granting node) |

**Composition pattern (the common case)**:

```
1. Producer publishes a Contribution with subject_key_ids = [user_key]
2. User (or a delegates_to chain rooted at user) emits a bare `scores` on
   `consent:state:granted` against the producer's Contribution, with
   `consent:scope:[retain, share, analyze]` companion attestations
3. Later: user issues `withdraws` against the producer's Contribution
   (admitted under §3.2.3 rule 2 — subject revocation)
4. Substrate watcher (per §8.1.11) starts SLA clock if producer committed
   `consent:deletion_sla:{days}` at publication
5. Producer emits `consent:deletion_complete` within the window OR
   substrate emits `hard_case:consent_sla_breach` as observability signal
```

**1+4 lockdown preserved**: every step rides existing structural primitives (`scores` / `withdraws`) plus the new `consent:*` dimensions. No new attestation_type.

#### §5.6.8.7 `consent_record` subject_kind

Per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45). The canonical envelope shape when consent is the primary subject of the Contribution itself (parallel to [`key_grant`](#key_grant) and [`takedown_notice`](#takedown_notice) — ceremony-shape over the underlying primitive). Use cases: standalone partnership grants, DSAR-shape consent declarations, multi-party contracts, explicit consent ceremonies with locked field schemas.

**Both shapes admitted at the same gate**: subject-side consent MAY ride a bare `scores` on `consent:state:*` against any target Contribution (the common case, see [§5.6.8.6](#5686-consent-namespace-family-ceg-06-addition) composition pattern), OR ride this `consent_record` subject_kind when an explicit ceremony envelope is wanted. Per the [§3.4 MISSION.md layering principle](../../MISSION.md), bare `scores` is the primitive; `consent_record` is the ceremony UX shape over the primitive.

```
consent_record {
    subject_key_id:       key_id              // the subject declaring stance (federation_keys
                                              //   OR canonical-hash per §4.2.2)
    target_key_id:        key_id | null       // optional: producer/recipient for bilateral grants
    stance:               ConsentStance       // closed-set enum per below
    scope:                [ConsentScope, ...] // open vocab; see §5.6.8.6
    asserted_at:          rfc3339_canonical   // per §0.5
    valid_until:          Option<rfc3339>     // null = indefinite
    deletion_sla_days:    Option<u32>         // for revocations: producer obligation window
                                              //   (composes with `consent:deletion_sla:{days}`)
    decay_protocol:       Option<string>      // optional: named multi-stage decay path
                                              //   (e.g., "ciris-agent-90day")
    bilateral_pair_id:    Option<string>      // for bilateral grants: pairs subject + producer
                                              //   Contributions via topical_relation:bilateral_pair
}

ConsentStance (closed-set):
| value     | meaning                                                                 |
|-----------|-------------------------------------------------------------------------|
| granted   | Subject affirms; processing may proceed within scope and valid_until    |
| revoked   | Subject withdraws; producer must initiate deletion within sla window    |
| expired   | Substrate emission when valid_until passes without renewal              |
```

**1+4 preserved** — rides `scores` + subject_kind discriminator ([§5.6.8](#568-content-ingestion-prefixes) mechanism); [§1.4](01_foundation.md) seventh path.

**Admission rules (normative, 1.0-RC5 — ratifies [CIRISPersist#146](https://github.com/CIRISAI/CIRISPersist/issues/146) Ask 5).** A `consent_record` Contribution is admitted iff:
1. **Required fields present**: `subject_key_id`, `stance` (closed-set), `asserted_at` (§0.5-canonical). All others are optional per the envelope above; absent optionals ride the [§0.9.2](00_conformance.md) omit rule.
2. **`stance` is a closed-set value** (`granted` / `revoked` / `expired`); **`expired` is substrate-emitted only** — a producer/subject MUST NOT assert `expired` (it is the substrate's `valid_until`-passed emission).
3. **Tier eligibility** per [§10.1.3](10_endpoints.md): a `stance: revoked` `consent_record` is **NOT local-tier-eligible** (it carries subject revocation authority over another party's content) — it goes federation-tier (hybrid-signed) or rides the §10.1.3 24-hour `local → federation` promotion. A `stance: granted` *self*-consent where the subject holds sole authority MAY be local-tier per [§10.1.5.2](10_endpoints.md).
4. **Composition with the [§3.2.3](03_primitives.md) `withdraws` gate**: a `stance: revoked` `consent_record` whose `subject_key_id` ∈ the target's `subject_key_ids[]` is admitted under §3.2.3 subject-revocation authority (rules 2–4), and the substrate SHOULD record which rule admitted it (the §3.2.3 per-rule audit metadata). No producer co-signature and **no quorum** is required — single-subject authority suffices ([§8.1.11.2](08_composition.md)).

It rides the same admission gate as a bare `scores` on `consent:state:*`; the `consent_record` form simply carries the locked payload schema instead of a free dimension.

**Bilateral pair pattern** (per [CIRISAgent CEM](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/CIRIS_CONSENT_SERVICE.md) PARTNERED stream):

```
1. Subject emits consent_record(subject_key_id, stance: granted,
                                 bilateral_pair_id: <fresh-uuid>) +
                  scores on `consent:partnership_grant:v1`
2. Producer emits consent_record(subject_key_id, target_key_id: subject_key_id,
                                 stance: granted, bilateral_pair_id: <same-uuid>) +
                  scores on `consent:partnership_accept:v1`
3. topical_relation:bilateral_pair links the two Contributions
4. Consumer policy treats the partnership as ratified iff both halves present
   under the same bilateral_pair_id with stance: granted
```

The structural primitives close the bilateral shape — no new attestation_type, no new envelope field beyond `subject_key_ids` itself.

#### §5.6.8.8 `identity_occurrence` subject_kind

Per [CIRISRegistry#47](https://github.com/CIRISAI/CIRISRegistry/issues/47) + [CIRISPersist#152](https://github.com/CIRISAI/CIRISPersist/issues/152) (substrate spec) + [ciris.ai/cewp](https://ciris.ai/cewp) structural-invisibility framing. The wire-format primitive that lets one logical identity speak across multiple **trusted participants** — devices (phone / laptop / server / embedded) AND agents (the user's own agents acting on the user's behalf). Today CEG's `occurrence_id` envelope field ([§4](04_envelope.md)) names which occurrence emitted a Contribution; `identity_occurrence` is the **wire-format binding** that lets the substrate know `key_phone` and `key_laptop` and `key_my_agent` all represent the same identity `key_identity`.

Without this primitive: `cohort_scope: self` content cannot reach the user's other devices/agents — the substrate has no structural way to know which keys are co-self. With it: at-rest encryption flow (substrate Ask CIRISPersist#152) automatically wraps DEKs to all admitted occurrences when new content is admitted at `cohort_scope: self`.

```
identity_occurrence {
    identity_key_id:        key_id              // root identity (the user's logical identity)
    occurrence_key_id:      key_id              // this participant's signing key
    device_class:           DeviceClass         // closed-set enum per below
    hardware_attestation:   Option<base64>      // TPM / Secure Enclave / StrongBox / SGX
                                                 //   etc. attestation blob; null for software-only
    transport_destination:  Option<TransportDestination>  // CEG 0.12 — Reticulum binding (below)
    encryption_pubkeys:     Option<EncryptionPubkeys>      // CEG 0.18 — content-KEM keys (§5.6.8.8.2 below);
                                                            //   present ⇒ this occurrence is a v2 wrap target
    asserted_at:            rfc3339_canonical   // per §0.5
    valid_until:            Option<rfc3339>     // null = indefinite
}

EncryptionPubkeys (CEG 0.18 addition — the recipient content-encryption KEM binding; §5.6.8.8.2):
| field                | type      | meaning                                                        |
|----------------------|-----------|----------------------------------------------------------------|
| x25519_base64        | [u8; 32]  | classical KEM half — a FRESH content-KEM key (NOT the signing  |
|                      |           |   key, NOT the transport x25519 below — see key-separation)    |
| ml_kem_768_base64    | [u8; 1184] | PQC KEM half (FIPS 203, ML-KEM-768; exactly 1184 raw bytes — `ML_KEM_768_PUBKEY_LEN` — pre-base64) |

TransportDestination (CEG 0.12 addition — the authenticated identity↔address binding):
| field                     | type      | meaning                                               |
|---------------------------|-----------|-------------------------------------------------------|
| reticulum_x25519_pubkey   | [u8; 32]  | transport identity's encryption key                   |
| reticulum_ed25519_pubkey  | [u8; 32]  | transport identity's signing key                      |
| destination_hash          | [u8; 16]  | RNS destination hash; MUST derive from the two pubkeys |
|                           |           |   + app_name + aspects per the §5.6.8.8.1.1 algorithm  |
| app_name                  | string    | RNS destination app (e.g. "ciris.federation")         |
| aspects                   | [string]  | RNS aspects (ordered; part of the hash preimage)      |

DeviceClass (closed-set):
| value      | scope                                                            |
|------------|------------------------------------------------------------------|
| phone      | Mobile device (iOS / Android / etc.); typically hardware-rooted  |
| laptop     | Personal computing device (macOS / Linux / Windows)              |
| server     | Always-on infrastructure node (home server, VPS, etc.)           |
| embedded   | IoT / hardware peripheral / signing dongle                       |
| agent      | An AI agent acting on the identity's behalf (the agent has its   |
|            | own signing key but speaks AS the identity — composes with       |
|            | CIRISAgent#840 self-attestation pattern)                         |
| service    | Background service / scheduled job / API integration acting      |
|            | on the identity's behalf                                          |
```

**Self-attested + single-vouch admission**: an `identity_occurrence` Contribution is admitted when `attesting_key_id == identity_key_id` (the identity claims "this key is also me") OR when `attesting_key_id` is itself a currently-admitted occurrence of `identity_key_id` (any existing self-member vouches for the new self-member — Signal-style "trust any device I've already onboarded"). Higher-assurance setups MAY layer requirements on `hardware_attestation` via consumer policy.

**Revocation**: a `withdraws` against an `identity_occurrence` Contribution issued by `identity_key_id` (or by any current occurrence) evicts the occurrence. Substrate stops wrapping new key_grants to it; previously-delivered DEKs are out of scope per [§8.1.12 Policy L](08_composition.md) forward-secrecy decision (Option A — once shared, always shared at the wire layer; rotation is a separate ceremony).

**Cardinality**: an identity MAY admit unbounded occurrences; the substrate carries no hard cap (operator policy MAY impose per-deployment limits). When a new occurrence is admitted, substrate emits `hard_case:identity_occurrence_added:{identity_key_id}` ([§7.2](07_reserved.md)) so consumer policy can observe membership growth.

**Composition with CIRISAgent CEG-native agent** ([CIRISAgent#840](https://github.com/CIRISAI/CIRISAgent/issues/840)): an agent emitting self-attestations with `attesting_key_id == agent_self_key` AND `attesting_key_id` admitted as an `identity_occurrence` of the user's `identity_key_id` is structurally speaking AS that identity. The agent's local-tier self-attestations remain its own; federation-tier emissions reach the user's other occurrences via the at-rest encryption flow when `cohort_scope: self`.

**1+4 preserved** — rides `scores` + subject_kind discriminator ([§5.6.8](#568-content-ingestion-prefixes) mechanism); [§1.4](01_foundation.md) eighth path.

##### §5.6.8.8.1 `transport_destination` — the authenticated identity↔address binding

Per [CIRISRegistry#56](https://github.com/CIRISAI/CIRISRegistry/issues/56) + [CIRISEdge#15](https://github.com/CIRISAI/CIRISEdge/issues/15) (AV-42). In a **CEG/RET stack there is no DNS** — a node resolves a community member to a *reachable address* with no trusted nameserver. The `transport_destination` field is the wire-format primitive that makes that resolution **authenticated** instead of trust-on-first-use.

**The layer split it closes (AV-17 / AV-42).** A node's Reticulum destination is a *dedicated dual-key transport identity* `hash(x25519 ‖ ed25519)` — deliberately **separate** from the federation signing key (the federation seed never enters the Reticulum/Leviculum transport layer, AV-17). So "destination D belongs to federation key K" is a claim that must be *proven*, not assumed: a bare Reticulum announce only proves the announcer controls *that transport identity*, not that it legitimately belongs to K (AV-42 — any peer can announce `key_id=registry-steward-us` paired with an adversary destination → senders route to the adversary).

**The binding = a federation-key-signed `identity_occurrence` carrying `transport_destination`.** Because the occurrence is admitted only when `attesting_key_id == identity_key_id` (or a current occurrence of it) and is hybrid-signed (Ed25519 + ML-DSA-65), the binding `destination_hash ← identity` is cryptographically authenticated. A spoofer cannot forge an `identity_occurrence` signed by the real key. This promotes [CIRISEdge#15](https://github.com/CIRISAI/CIRISEdge/issues/15) Option B (signed-announce attestation) from an Edge-internal app-data format into a **first-class, federation-wide, auditable CEG shape** — the announce app-data MAY carry it for self-authenticating discovery, and the directory holds it as the durable source of truth.

**Conformance**: a Consumer resolving a member's address MUST verify (1) the `identity_occurrence` signature against the member's federation key, (2) that `destination_hash` recomputes from `reticulum_x25519_pubkey`, `reticulum_ed25519_pubkey`, `app_name`, and `aspects` per the **[§5.6.8.8.1.1](#568811-rns-destination-hash-algorithm-pinned)** pinned algorithm (no free-floating hash), and (3) the occurrence is non-superseded + within `valid_until` at resolution time. An unauthenticated announce (no matching signed `transport_destination`) is **advisory-only** — usable as a routing hint, never as an authorization. Rotating the Reticulum destination (new transport identity) is a new `identity_occurrence` `supersedes`-ing the prior — location changes without touching federation identity or community membership.

###### §5.6.8.8.1.1 RNS destination-hash algorithm (pinned)

**Normative, 1.0-RC6 — reproduces the RNS destination-hash construction in-spec so a conformant verifier can recompute `destination_hash` from this document alone, with no Reticulum vendoring (resolves [CIRISRegistry#80](https://github.com/CIRISAI/CIRISRegistry/issues/80) / [CIRISVerify#28](https://github.com/CIRISAI/CIRISVerify/issues/28)).** This is a **two-stage** hash — it is **NOT** a single SHA-256 over a flat `x25519 ‖ ed25519 ‖ app_name ‖ aspects` preimage. The naive flat form yields a *different, wrong* value; CEG ≤1.0-RC5's "per the RNS rule" wording under-specified this, which is why an independent recompute was previously impossible.

Pinned constants (SHA-256 throughout — RNS `full_hash`):

| name | value | RNS origin |
|---|---|---|
| `NAME_HASH_LEN` | **10** bytes | `Identity.NAME_HASH_LENGTH` = 80 bits |
| `DEST_HASH_LEN` | **16** bytes | `Reticulum.TRUNCATED_HASHLENGTH` = 128 bits |

Algorithm:

```
# 1. Expanded name — UTF-8. app_name, then each aspect dot-joined, IN THE FIELD ORDER.
#    The identity hexhash is NOT included (RNS computes name_hash with identity=None).
expanded_name = app_name
for aspect in aspects:                     # `aspects` in the field's given order
    reject if "." in aspect                # dots are illegal inside an aspect
    expanded_name += "." + aspect

# 2. name_hash = first 10 bytes of SHA-256(expanded_name)
name_hash = SHA256(utf8(expanded_name))[:NAME_HASH_LEN]            # 10 bytes

# 3. identity_hash = first 16 bytes of SHA-256(x25519_pub ‖ ed25519_pub)
#    Key order is reticulum_x25519_pubkey (32) THEN reticulum_ed25519_pubkey (32) —
#    RNS get_public_key() = pub_bytes (X25519) ‖ sig_pub_bytes (Ed25519).
identity_hash = SHA256(reticulum_x25519_pubkey ‖ reticulum_ed25519_pubkey)[:DEST_HASH_LEN]   # 16 bytes

# 4. destination_hash = first 16 bytes of SHA-256(name_hash ‖ identity_hash)
#    addr_hash_material is the 26-byte concat (10 + 16); final hash truncates to 16.
destination_hash = SHA256(name_hash ‖ identity_hash)[:DEST_HASH_LEN]                         # 16 bytes
```

**Pinned source**: Reticulum `RNS/Destination.py::Destination.hash` + `RNS/Identity.py` (`full_hash` = SHA-256; `truncated_hash`; `get_public_key()` = `pub_bytes ‖ sig_pub_bytes`) + `RNS/Reticulum.py` (`TRUNCATED_HASHLENGTH = 128`). **CEG owns this reproduction**: it is the closed conformance source and does **not** float with upstream Reticulum — a future RNS hash change is a deliberate CEG version bump, never silent drift. A verifier that recomputes `destination_hash` per the four steps above and compares for byte-equality has performed the AV-42 destination-authenticity check; a mismatch MUST be treated as an unauthenticated (advisory-only) announce.

**1+4 preserved** — `transport_destination` is one optional field on the existing `identity_occurrence` subject_kind ([§5.6.8](#568-content-ingestion-prefixes) mechanism; no new subject_kind). Twelfth path ([§1.4](01_foundation.md)) — the wire format expresses **its own addressing layer** (DNS-free, self-certifying member resolution) by composition.

##### §5.6.8.8.2 `encryption_pubkeys` — the recipient content-encryption KEM binding

Per [CIRISPersist#192](https://github.com/CIRISAI/CIRISPersist/issues/192) + [CIRISRegistry#69](https://github.com/CIRISAI/CIRISRegistry/issues/69). The [§10.1.4](10_endpoints.md) at-rest DEK cascade is **substrate-wraps-by-default**: the substrate generates the per-write DEK and wraps it to each active recipient. That wrap (`wrap_algorithm: v2`, §5.6.8.4) needs each recipient's **x25519 + ML-KEM-768 encryption** keys — but the federation directory's key registration carries only **signing** keys (Ed25519 + ML-DSA-65), and **ML-KEM cannot be derived from ML-DSA** (independent algorithms). So recipients must register encryption keys, and the substrate must resolve them by `key_id`. `encryption_pubkeys` is that binding.

**The binding rides `identity_occurrence`, exactly parallel to `transport_destination`.** It inherits the same four properties, already enforced, that an encryption-key binding requires:
1. **Self-certified** — admitted only when `attesting_key_id == identity_key_id` (or a current occurrence of it), so "these are identity K's encryption keys" is cryptographically proven, not trust-on-first-use. A spoofer cannot forge it.
2. **Hybrid-signed** (Ed25519 + ML-DSA-65) — the binding itself is PQC-signed.
3. **Rotatable via `supersedes`** — a new `identity_occurrence` superseding the prior rotates the KEM keys **without touching the stable signing `key_id`** that anchors every attestation/grant. A compromised ML-KEM key rotates for forward secrecy; the signing identity is untouched. (Bundling these onto the signing key registration would couple two independent rotation lifecycles — the reason this is NOT a field on the key record.)
4. **Already cross-region replicated** — `identity_occurrence` is `EnvelopeKind::IdentityOccurrence` in the locked replication wire ([CIRISEdge#65](https://github.com/CIRISAI/CIRISEdge/issues/65)), so the encryption pubkeys propagate inside the occurrence envelope that already replicates — **no new replication kind, no Edge wire change.** A cross-region recipient's keys resolve wherever its occurrence has propagated. (Encryption *pubkeys* are public → cleartext directory replication is correct, exactly as for signing keys.)

**Key separation (normative — never reuse, and admission-enforced).** The `x25519` here is a **fresh content-KEM key**, distinct from BOTH (a) the occurrence's signing keys AND (b) the Reticulum transport x25519 in [§5.6.8.8.1](#5688-1-transport_destination--the-authenticated-identityaddress-binding-ceg-012-addition) `destination_hash = hash(x25519 ‖ ed25519)` (that is the *RET-link* transport key, classical-only, AV-17 — the federation seed never enters the transport layer, and the transport key never wraps content DEKs). Three key *purposes* — signing, RET-transport, content-KEM — are three distinct keypairs. Deriving the content-KEM x25519 from either of the others is a conformance violation (cross-protocol key reuse). **Admission check (1.0-RC1, [#71](https://github.com/CIRISAI/CIRISRegistry/issues/71) C4):** when an `identity_occurrence` carries BOTH `encryption_pubkeys` AND `transport_destination`, the substrate MUST **reject at admission** if `encryption_pubkeys.x25519_base64` decodes to the same 32 bytes as `transport_destination.reticulum_x25519_pubkey` — the one reuse case that is wire-checkable for free. (Reuse of the *signing* key as KEM key is not byte-comparable on the wire — different algorithms — and remains a producer-side conformance obligation.)

**Forward-secrecy scope — honesty note (normative — 1.0-RC1, [#71](https://github.com/CIRISAI/CIRISRegistry/issues/71) C2).** KEM-key rotation via `supersedes` bounds **future** exposure only: grants wrapped *after* rotation use the new key. It does **nothing** for history — every `key_grant` previously wrapped to the compromised key persists at rest, and the at-rest threat model ([§10.1.4](10_endpoints.md) disk-forensics / host-operator adversary) is *precisely* an adversary holding those old grant bytes; with the old private key they decrypt every DEK ever wrapped to it, and `rotation_chain` supersession does not revoke bytes the adversary already holds. **Recovering historical content after a KEM-key compromise requires DEK rotation + content re-encryption under the new DEK — which CEG does NOT currently mandate.** This is a named gap (the [§1.6](01_foundation.md) honesty discipline): operators with a compromised-key event MUST treat all content whose DEKs were wrapped to that key as exposed, and MAY re-encrypt; the spec provides the mechanism (new DEK + new grants + `supersedes`) but no automatic trigger. Do not represent KEM rotation as recovering the confidentiality of previously-wrapped content.

**These feed `wrap_algorithm: v2` directly.** `{x25519, ml_kem_768}` are precisely the recipient inputs to `x25519_mlkem768_aes256_gcm_hkdf_sha256` ([§5.6.8.4](#5684-governance-subject_kinds-ceg-03-addition-per-cirisregistry37--38)). A consumer/substrate resolving a recipient's wrap target reads the **current (non-superseded, within-`valid_until`) `identity_occurrence` for that `key_id` → its `encryption_pubkeys`** (the `resolve_encryption_keys(key_id)` contract — [CIRISPersist#192](https://github.com/CIRISAI/CIRISPersist/issues/192)).

**Fail-secure conformance (the [§10.1.4](10_endpoints.md) tie-in).** Because §10.1.4 *mandates* v2 for at-rest encryption, a recipient whose current occurrence carries **no valid ML-KEM-768 key is fail-secure *excluded* from the grant** — the content stays encrypted and unreachable to it; the substrate MUST NOT fall back to plaintext or to v1. To be an at-rest-encryption recipient, an identity MUST have a federation-present `identity_occurrence` carrying `encryption_pubkeys`. (This also resolves the "`family` member named only by `key_id` with no occurrence" case: no presence ⇒ no wrap target ⇒ excluded — correct, since there would be nowhere to deliver/store the wrapped DEK for a member that never established a presence.)

**1+4 preserved** — `encryption_pubkeys` is one optional field-set on the existing `identity_occurrence` subject_kind ([§5.6.8](#568-content-ingestion-prefixes) mechanism; no new subject_kind, no new replication kind). Fifteenth path ([§1.4](01_foundation.md)) — the wire format expresses **its own at-rest-encryption key layer** (recipient KEM-key resolution for substrate-wraps) by composition.

#### §5.6.8.9 `family` subject_kind

Per [CIRISRegistry#47](https://github.com/CIRISAI/CIRISRegistry/issues/47) + cewp structural-invisibility framing. A `family` is a **group of trusted nodes** — each node being a distinct identity (which itself may have multiple `identity_occurrence`s per §5.6.8.8). Families are the wire-format primitive for `cohort_scope: family` visibility scoping: content scoped to a family is admitted into substrate, wrapped under the family DEK, and delivered (via `key_grant` per [§5.6.8.4](#key_grant)) to all current members — but never emits `holds_bytes:sha256:*` to non-members ([§10.1.4](10_endpoints.md)).

One identity MAY belong to multiple families. Each family has its own DEK and its own membership roster.

```
family {
    family_key_id:           key_id                    // the family's own federation_keys identity
    family_name:             string                    // human-readable; non-unique
    members: [
        {
            key_id:          key_id                    // member identity_key (NOT occurrence_key)
            joined_at:       rfc3339_canonical
            role:            Option<MemberRole>        // founder | member | null
        },
        ...
    ]
    founded_at:              rfc3339_canonical
    consensus_protocol:      ConsensusProtocol         // REQUIRED — see below
    consensus_protocol_entrenched: bool                // if true, consensus_protocol may not be
                                                       //   amended even via the protocol's own rules;
                                                       //   replacement requires out-of-band ceremony
                                                       //   (see §9 HUMANITY_ACCORD canonical instance)
}

MemberRole (open vocab; canonical kinds):
| value     | meaning                                                              |
|-----------|----------------------------------------------------------------------|
| founder   | Bootstrapping signer (recorded at founded_at)                        |
| member    | Standard member; rights per consensus_protocol                       |
```

**`ConsensusProtocol` — open vocabulary**. The family's chosen consensus mechanism for membership changes. Locked at family creation; changes ride the protocol's own rules (meta-amendment shape parallel to [§11.2.3](11_governance.md)) UNLESS `consensus_protocol_entrenched == true`, in which case replacement requires an out-of-band ceremony.

Canonical `ConsensusProtocol` kinds (CEG 0.7 documentation; operator vocab extends):

| kind | Semantic |
|---|---|
| `founder_only` | Original founders are the sole admission authority; new members proposed-and-admitted by any founder. Suits private households / small trust circles. |
| `unanimous` | Every current member must sign the admission Contribution. Suits very small high-trust groups. |
| `majority` | > 50% of current members must sign. Suits medium groups where blocking-minority concerns matter. |
| `quorum:{m}/{n}` | Any `m` of `n` current members must sign (where `n` is the current roster size). The canonical entrenched form: HUMANITY_ACCORD per [§9](09_humanity_accord.md) is `family` with `quorum:2/3` + `consensus_protocol_entrenched: true`. |
| `weighted:{rubric}` | Sum of member weights (per a named operator rubric) must exceed a threshold. Suits formal organizations with weighted voting. |
| `custom:{family_specific_id}` | Operator-defined custom protocol (e.g., role-based, time-locked, multi-stage). |

**Membership-change ceremony** (any addition or removal of a member):

```
1. Proposer (any current member) emits a new `family` Contribution
   superseding the current family Contribution (per `supersedes`) with
   the new membership list.

2. Substrate gates admission per the CURRENT family's `consensus_protocol`:
   - Counts signatures on the proposed Contribution (via the
     `consensus_protocol` rule)
   - If the rule is satisfied, admit and emit
     hard_case:family_membership_change:{family_key_id}
   - If not, hold the proposal in a pending state until additional
     member signatures arrive (per a configurable window — operator policy)

3. On admission of an ADD: substrate emits retroactive `key_grant`s wrapping
   all `cohort_scope: family` content DEKs to the new member's
   `subject_key_ids` (per CIRISPersist#152 substrate flow).

4. On admission of a REMOVE: per Option A (§8.1.12) the removed member
   retains existing key_grants (cannot retroactively un-share); the
   substrate stops wrapping new key_grants to them on subsequent
   family-scoped Contributions.
```

**Consensus-protocol amendment**:

```
A `family` Contribution that supersedes the current family Contribution AND
changes the `consensus_protocol` field is admitted ONLY IF:
  (a) consensus_protocol_entrenched == false, AND
  (b) the CURRENT protocol's rule is satisfied on the amendment Contribution

If consensus_protocol_entrenched == true, the substrate REJECTS the
amendment. Protocol replacement requires an out-of-band ceremony
(documented per family; for HUMANITY_ACCORD see §9.2 / FEDERATION_ANNOUNCEMENT.md
§4.5.3).
```

**Substrate emissions on family events**:
- `hard_case:family_membership_change:{family_key_id}` — member added or removed
- `hard_case:family_consensus_protocol_change:{family_key_id}` — consensus_protocol amended (only when `consensus_protocol_entrenched == false`)
- `hard_case:family_consensus_protocol_violation:{family_key_id}` — proposed amendment rejected because rule not satisfied OR entrenched

All three are reserved under [§7.2](07_reserved.md) substrate-self-report.

**HUMANITY_ACCORD as canonical entrenched-`family`**: the three accord-holder triple at [§9](09_humanity_accord.md) is structurally an instance of this primitive:

```
family {
    family_key_id:                   "humanity-accord",
    family_name:                     "Humanity Accord",
    members: [
        {key_id: "eric-moore-key",      role: "founder"},
        {key_id: "eric-kudzin-key",     role: "founder"},
        {key_id: "haley-bradley-key",   role: "founder"}
    ],
    consensus_protocol:              "quorum:2/3",
    consensus_protocol_entrenched:   true                  // replacement only via §9.2 ceremony
}
```

§9 remains load-bearing for the *role-recognition policy* + the `AccordCarrier` priority authority + the substrate-protective semantics. CEG 0.7 just makes explicit that the structural shape of `HUMANITY_ACCORD` is a `family` subject_kind instance, generalizing the primitive across the federation.

**Worked example — household with self-devices and family-devices**:

```
User Alice has:
  identity_key = alice_root_key

Alice's self (identity_occurrence members):
  - alice_phone_key      (device_class: phone)       ─┐
  - alice_laptop_key     (device_class: laptop)       │ Each is an `identity_occurrence`
  - alice_work_laptop    (device_class: laptop)       │ of `alice_root_key` per §5.6.8.8;
  - alice_agent_key      (device_class: agent)        │ Alice scrolls Twitter on her phone,
  - alice_homeserver_key (device_class: server)      ─┘ that content is `cohort_scope: self`
                                                        and reaches her other devices via
                                                        the at-rest encryption flow.

Alice's household (a `family` subject_kind instance):
  family_key_id:                "acme-household"
  family_name:                  "Acme Household"
  members: [
    {key_id: alice_root_key,    role: founder},   ─┐
    {key_id: bob_root_key,      role: founder},    │ Member entries are IDENTITY keys
    {key_id: roku_living_room,  role: member},     │ (NOT occurrence keys). Bob has his
    {key_id: kitchen_tablet,    role: member},     │ own self-collective; the Roku and
    {key_id: nest_thermostat,   role: member}     ─┘ kitchen tablet have their own
                                                     identity_keys (they don't belong
                                                     to any one person via identity_occurrence —
                                                     they're shared household nodes that the
                                                     family has admitted as members in their
                                                     own right).
  ]
  consensus_protocol:           "founder_only"   // either founder admits new members
  consensus_protocol_entrenched: false           // founders can amend the protocol

When Alice's phone sends a family-scoped photo (e.g., dinner photo) at
cohort_scope: family + family_id: acme-household:
  - Substrate wraps the DEK under each member's identity key
  - Photo bytes reach Bob's devices, the Roku, the kitchen tablet,
    the Nest thermostat — every admitted family node
  - NO holds_bytes:sha256:* attestation emits (§10.1.4); non-family peers
    cannot even discover the content exists
  - Alice's own laptop also receives the photo via the at-rest encryption
    flow at cohort_scope: self → identity_occurrence

When Bob's mom Carol visits and Bob wants to admit Carol's phone to view
family photos for a week:
  - Bob proposes a supersedes Contribution adding {key_id: carol_phone_root,
    role: member, valid_until: +7d}
  - consensus_protocol "founder_only" admits on Bob's signature alone
  - Substrate emits retroactive key_grants for `cohort_scope: family` content
    to Carol's phone (per CIRISPersist#152 flow)
  - On Carol leaving (member removal via supersedes, founder-signed):
    Carol retains existing key_grants per §8.1.12 Option A; substrate stops
    wrapping new family content to her key
```

The example demonstrates the clean orthogonality: **`identity_occurrence` is for participants that ARE me** (across my devices and agents); **`family` is for trusted nodes that compose with me** (other people's identities, shared household devices, multi-party collectives). Phone = self device; Roku = family device. The two primitives compose without overlap.

**1+4 preserved** — rides `scores` + subject_kind discriminator ([§5.6.8](#568-content-ingestion-prefixes) mechanism); [§1.4](01_foundation.md) eighth path. Removed-member key_grant cessation rides Option A forward-secrecy ([§11.7.1](11_governance.md)) — the substrate stops wrapping new `key_grant`s; no DEK rotation, no re-encryption. (NOTE: CEG 0.3's `rotation_chain` field on `key_grant` payload is the content-addressed grant-supersession lineage per [§5.6.8.4](#key_grant) — a separate axis from key rotation. The per-`(stream_id, epoch)` epoch-key axis for CEG 0.10 streaming ([§10.5.3](10_endpoints.md)) reuses the same payload-level supersession mechanism on a parallel addressing axis.)

#### §5.6.8.10 `community` subject_kind

Per [CIRISRegistry#48](https://github.com/CIRISAI/CIRISRegistry/issues/48). A `community` is a **larger node-collective with explicit admission semantics** — sibling subject_kind to `family` (CEG 0.7 [§5.6.8.9](#5689-family-subject_kind-ceg-07-addition)) but with different defaults: content scoped `cohort_scope: community` **federates within the cohort** (emits `holds_bytes:sha256:*` per status quo); there is NO at-rest DEK cascade; [§10.1.4](10_endpoints.md) structural-invisibility applies to self/family only.

Communities differ from families along three axes:

| | `family` (CEG 0.7) | `community` (CEG 0.8) |
|---|---|---|
| **Scale** | Household / intimate trust circle (typically ≤ 20 members) | City / professional / interest (10s to 100Ks of members) |
| **At-rest encryption** | Yes — DEK cascade per [§8.1.12.4](08_composition.md); `holds_bytes:*` suppressed | No — content federates per status quo |
| **Subkind discriminator** | None | `cohort_subkind` field (open vocab; canonical: `geographic`, `infrastructure`) |
| **Typical admission** | `founder_only` / `unanimous` for small intimate groups | `majority` / `weighted` / per-subkind protocol (e.g., geographic requires `location_proof`) |

```
community {
    community_key_id:                  key_id                    // community's own federation key
    community_name:                    string                    // human-readable; non-unique
    cohort_subkind:                    string                    // open vocab; canonical: "geographic"
    members: [
        {
            key_id:                    key_id                    // member identity_key (NOT occurrence)
            joined_at:                 rfc3339_canonical
            role:                      Option<MemberRole>        // founder | member | null
        },
        ...
    ]
    founded_at:                        rfc3339_canonical
    consensus_protocol:                ConsensusProtocol         // same six canonical kinds as family
    consensus_protocol_entrenched:     bool                      // same semantics as family
    cohort_subkind_payload:            Option<SubkindPayload>    // subkind-specific fields; see below
}
```

**`cohort_subkind` is the discriminator** — open vocabulary per [§11.2.1](11_governance.md) axis-vocabulary discipline. CEG 0.8 codifies `geographic`; CEG 0.11 adds `infrastructure` (see below); operator vocabularies extend.

##### Canonical `cohort_subkind: geographic`

The first codified community subkind. Membership additionally requires the candidate to emit a valid `location_proof` ([§5.6.8.11](#56811-location_proof-subject_kind-ceg-08-addition)) whose `cell_id` is contained ([§0.8.2](00_conformance.md)) within the community's `geographic_constraint.cell_id`.

The `cohort_subkind_payload` for `geographic`:

```
geographic_constraint {
    cell_id:                           string                    // H3 cell, lowercase hex per §0.8
    cell_resolution:                   u8                        // 0-15; community-side may be any
                                                                 //   resolution (NOT bounded to ≤ 7;
                                                                 //   that bound applies only to
                                                                 //   location_proof emissions)
}
```

**Worked example — Austin geographic community**:

```
community {
    community_key_id:                  "austin-community",
    community_name:                    "Austin",
    cohort_subkind:                    "geographic",
    cohort_subkind_payload: {
        geographic_constraint: {
            cell_id:                   "85283473fffffff",      // H3 res 5, ~250 km² covering Austin metro
            cell_resolution:           5
        }
    },
    members: [
        {key_id: alice_root_key,       role: founder},
        {key_id: bob_root_key,         role: founder},
        ...
    ],
    consensus_protocol:                "majority",
    consensus_protocol_entrenched:     false
}
```

For Alice to join Austin, she emits a `location_proof` with a `cell_id` (resolution 7 — ~5 km²) that is contained within the Austin community's `85283473fffffff` (resolution 5) constraint. The community's `majority` admission rule then evaluates her membership proposal.

**Privacy as opt-in**: joining a geographic community is a **one-way disclosure**. Per [§11.8](11_governance.md), the `location_proof` remains in the audit chain even after the member leaves; rough-only is wire-format-enforced (§0.8.1). The substrate's role is to enforce the opt-in mechanically — produce a `location_proof` to join; cannot emit finer than rough.

**Membership change ceremony**: same shape as family (per [§5.6.8.9](#5689-family-subject_kind-ceg-07-addition)) — rides existing `supersedes` primitive; admission gated by current `consensus_protocol`; additionally for `geographic` subkind, the candidate's most-recent `location_proof` (within `valid_until`) MUST be contained in the geographic_constraint.

**Substrate emissions on community events**:
- `hard_case:community_membership_change:{community_key_id}`
- `hard_case:community_consensus_protocol_change:{community_key_id}`
- `hard_case:community_consensus_protocol_violation:{community_key_id}`

All three reserved under [§7.8](07_reserved.md).

**Community DEK cascade (MANDATORY — CEG 0.17, supersedes the 0.8 "no cascade")**: community content (`cohort_scope: community | affiliations`) is encrypted at rest under a **per-community DEK** and emits `holds_bytes:sha256:*` carrying **cleartext provenance** (`attesting_key_id`, `community_id`, reason/dimension) so non-member holders can make an informed keep/evict decision without reading content. The community DEK follows the [§10.5.3](10_endpoints.md) epoch-DEK cascade: one DEK shared across emissions (per-emission cost **O(1)**, not O(members)), wrapped to each member on admission, re-wrapped on membership change (Option-A forward secrecy — [§11.7.1](11_governance.md) / [§8.1.13.4](08_composition.md)); **`wrap_algorithm: v2` (hybrid PQC) MANDATORY** (same harvest-now-decrypt-later reasoning as self/family — [§8.1.12.4](08_composition.md)). The 0.8 "per-member DEK wrap on every emission is infeasible" premise was **wrong** — that's per-member-*per-emission*; the shared community DEK is O(1)/emission, refuted by merged code (`list_key_grants_for_stream_epoch`, persist v4.4.0). The privacy property for communities is **byte-level confidentiality to members + provenance-visible discovery**, NOT the cohort-filtered-visibility-over-plaintext of 0.15. **Exception**: a `community` with `cohort_subkind: infrastructure` (`ciris-canonical` / governance roots, §5.6.8.10 below) **opts out** — Commons-tier plaintext, because the trust root must be maximally inspectable (see [§8.1.13.3](08_composition.md) for the three-tier model + holder-inspectability rationale).

**Worked example — civic-engagement + emergency-messaging composition pattern**:

CEG 0.8 + earlier specs compose cleanly for civic / democratic participation AND emergency / public-safety messaging use cases. None require new structural primitives; all ride the existing 1+4 set + the namespace additions. The two surfaces share the same underlying primitives (geographic community + location_proof + identity authority + cohort-scoped distribution) but differ in authority/priority shape — civic is bottom-up democratic participation; emergency is top-down authoritative broadcast.

| Civic shape | CEG composition |
|---|---|
| **Neighborhood association** | `community` with `cohort_subkind: geographic` + small `geographic_constraint` (e.g., H3 resolution 8-10 for a few city blocks); `consensus_protocol: majority` typical. Members emit `location_proof` at resolution 7 (rough-only privacy preserved); the community's constraint at higher resolution defines the *bounded scope*, not the *required disclosure precision* |
| **Municipal/city community** | `community` with `cohort_subkind: geographic` at resolution 5-6 (city-scale ~250-1700 km²); `consensus_protocol: majority` or `weighted:{voter_registration_rubric}` for formal civic governance; members compose with `partner_role:*` ([§5.9](#59-cirisregistry--identity--build--license--partner)) for licensed public officials |
| **Voting district** | `community` with `cohort_subkind: geographic` matched to district boundaries; the H3 hex approximation has known edge cases at gerrymandered district lines — operators use `cohort_subkind: custom:voting_district_X` with a polygon-based admission predicate when hex approximation is insufficient (the open vocab discipline accommodates this) |
| **Public town hall meeting** | `event_listing` (CEG 0.4 [§5.6.8.1](#5681-external_content-sub_kinds)) hosted by the geographic community; `subject_key_ids` ([§4.2](04_envelope.md)) names the organizer; `cohort_scope: community` + `community_id: <municipal_community>` scopes attendee visibility |
| **Ballot initiative / referendum** | The initiative itself is a `community` Contribution or an `event_listing` with `topical_relation:rsvps` repurposed as votes; individual votes ride `consent_record` (CEG 0.6 [§5.6.8.7](#5687-consent_record-subject_kind-ceg-06-addition)) with `stance: granted` ("yes") or `stance: revoked` ("no" or withdrawal of support); vote tallies are consumer-side composition over the `consent_record` chain |
| **Public comment** | `chat_message` (CEG 0.3) scoped to `cohort_scope: community` with `community_id` naming the relevant civic community; `topical_relation:comments_on` links to the ballot/initiative/hearing Contribution |
| **Petition signing** | `consent_record` with `stance: granted` + `scope: [share, publish]` against the petition Contribution; signatures aggregate via the same consumer-side composition as ballot votes |
| **Public official self-attestation** | Official's `identity_occurrence` (CEG 0.7 [§5.6.8.8](#5688-identity_occurrence-subject_kind-ceg-07-addition)) links their personal identity_key to their `device_class: service` key on `city.gov`; cross-binding via `identity:canonical_binding:{canonical_hash}` (CEG 0.6 + 0.7) authenticates their public statements |
| **FOIA / public records request** | `consent_record` with `scope: [publish]` requested against a producer (city agency); SLA enforcement via CEG 0.6 [§8.1.11.3](08_composition.md) substrate-side watcher emits `hard_case:consent_sla_breach` if the agency misses the response window |
| **Citizen-journalist coverage** | `news_article` (CEG 0.3) sub_kind authored by an individual member; `cohort_scope: community` + `community_id: <municipal>` for local-first distribution, promotable via [§8.1.8.1](08_composition.md) Tiered-Scope Composition to wider scope on consensus |
| **Whistleblower disclosure** | `cohort_scope: self` for in-graph composition; promote via `supersedes` to `cohort_scope: community` (a trusted journalists' community with `cohort_subkind: professional` once that subkind ships); `subject_key_ids` empty (no consent-revocation by the disclosed party). The §11.4 fast-path takedown coordination + §9 HUMANITY_ACCORD substrate-protective discipline apply to bad-actor takedown attempts against whistleblower content |
| **Civic mutual-aid network** | `community` with `cohort_subkind: geographic` matching the neighborhood; CEG 0.6 `consent:scope:[retain, share]` for resource-sharing posts; `event_listing` for distribution events; the at-rest encryption for self/family scope (CEG 0.7 [§10.1.4](10_endpoints.md)) keeps individual aid requests private while community-scoped offers federate |

**Emergency messaging shapes** — same primitive composition, different authority + priority profile:

| Emergency shape | CEG composition |
|---|---|
| **Severe weather warning** (NWS / met office) | `news_article` (CEG 0.3) authored by a `partner_role: emergency_authority` ([§5.9](#59-cirisregistry--identity--build--license--partner) co-owned with the community's `cohort_subkind: geographic`); `cohort_scope: community` with `community_id` per affected H3 cells — cascade-by-containment per [§0.8.2](00_conformance.md) propagates to all geographic communities whose constraint overlaps; `event:lifecycle:{state}` (CEG 0.4) carries `active` → `cleared` → `superseded` state machine; `valid_until` envelope field bounds advisory window |
| **AMBER Alert / Silver Alert / abduction notice** | `news_article` with `partner_role: emergency_authority` from law enforcement key; geographic targeting via `community_id` of affected jurisdictions; subject person identified via `subject_key_ids` (canonical-hash of the missing person identifier — opt-out semantic deferred per the substrate-protective discipline since recovering the missing person is the consent-overriding case); `topical_relation:supersedes_article` chain for status updates |
| **Active shooter / hostile event notice** | Same shape as AMBER but with `cohort_scope: community` scoped to the precise affected H3 cell (resolution 8-10 for building/campus-level precision is permitted on the COMMUNITY-side `geographic_constraint`; the `location_proof` rough-only bound at resolution 7 still applies to recipient location disclosures, NOT to alert targeting) |
| **Shelter-in-place / evacuation order** | `event_listing` with `event:lifecycle:active`; geographic targeting via `community_id`; recipients ack via `consent_record` with `stance: granted` and `scope: [retain]` against the order Contribution (acknowledgement, not consent to the underlying order — operator-policy distinction) |
| **Disease outbreak alert** (CDC / health authority) | `news_article` with `partner_role: health_authority`; `cohort_scope: community` scoped to affected geography; composes with CEG 0.6 `consent:scope:[analyze]` for contact-tracing opt-in (subject-side authority preserved — `subject_key_ids` of the affected person carry revocation rights per §3.2.3 rule 2) |
| **Mass casualty incident coordination** | Authority emits `event_listing` for the incident; first responders join via `community` with `cohort_subkind: professional` (future spec round) OR ad-hoc `cohort_subkind: custom:incident_response_X`; coordination uses `chat_message` scoped to the responder cohort; resource requests use the mutual-aid composition pattern above |
| **Infrastructure failure notice** (boil-water / power outage / gas leak) | `news_article` from utility authority with `cohort_scope: community` scoped to affected geographic cells; `event:lifecycle:{state}` for advisory progression; FOIA-shape `consent_record` later for post-incident reports |
| **Disaster recovery / mutual aid activation** | Federation of geographic communities (each `community` is a member of a parent `community` via membership composition — the multi-level family/community shape works the same way; resource flow rides `consent:scope:[share]` per CEG 0.6); time-bound activation rides `event_listing` lifecycle states |
| **CONSTITUTIONAL-level federation halt** (the existing CEG 0.6+/§9 HUMANITY_ACCORD invocation) | Per [§9.2](09_humanity_accord.md) `EmergencyShutdown CONSTITUTIONAL` + `accord:invoke:notify:{notify_id}` / `accord:invoke:drill:{drill_id}`. The accord-holder triple is structurally a `family` with `consensus_protocol: quorum:2/3` + `entrenched: true` (per CEG 0.7 retcon); the constitutional asymmetry rides existing primitives + scope-isolation rules. Distinct from operator-level emergency messaging (which is geographic-scoped + authority-emitted) — accord invocation is federation-wide-halt-level, not local-incident-level |

**No new structural primitives needed for emergency messaging either.** The authority profile (who can emit emergency advisories) composes via CEG 0.7 `identity_occurrence` cross-attestation from licensed authorities + the geographic community's roster/admission gate (`partner_role: emergency_authority` is the typed authority dimension). The priority profile (urgency / immediacy) composes via existing `oversight_mode` envelope field + per-cohort `consensus_protocol` (many emergency emitters bypass per-message consensus per their pre-cross-attested authority status — same shape as substrate-self-report `system:*` reservations from [§7.2](07_reserved.md)). The geographic propagation (cascade-by-containment) composes via [§0.8.2](00_conformance.md) containment semantics.

**The compositional reach is the point**: civic participation AND emergency messaging do not require new structural primitives at any layer of CEG 0.x. Geographic communities + location_proof admission + opt-in privacy disclosure (CEG 0.8) compose with consent / DSAR / partnership ceremonies (CEG 0.6) + identity / family (CEG 0.7) + content sub_kinds and event_listing (CEG 0.3/0.4) into the full civic-engagement surface.

The 1+4 lockdown holds across this entire surface — ninth-path confirmation that the wire format is rich enough for democratic-participation use cases without expanding the structural set.

**1+4 preserved** — rides `scores` + subject_kind discriminator ([§5.6.8](#568-content-ingestion-prefixes) mechanism); [§1.4](01_foundation.md) ninth path.

##### Canonical `cohort_subkind: infrastructure`

Per [CIRISRegistry#56](https://github.com/CIRISAI/CIRISRegistry/issues/56). The second codified community subkind: a **governed trust-root collective of canonical/bootstrap service installs** — the shape the CIRIS canonical services (Registry / Lens / Node) adopt instead of a `family` (rationale: `CIRISRegistry/MISSION.md` §2.1.1 — public content model, decentralization ramp, legitimacy). Where `geographic` answers "who is physically here," `infrastructure` answers "who is a recognized operator of this public service."

It differs from `geographic` in two load-bearing ways:

1. **No location gate.** Admission requires NO `location_proof` and NO `geographic_constraint`. The candidate is a *service install*, not a located person.
2. **Admission quorum is over FOUNDERS, not all members** — the anti-Sybil guardrail for a trust root. In `geographic`, admission is evaluated per `consensus_protocol` over the *current member set*; that is correct for a city (members govern themselves) and **wrong for a trust root** (flood the membership → dilute the quorum → admit rogue "canonical" operators). `infrastructure` therefore pins admission to the founding core.

The `cohort_subkind_payload` for `infrastructure`:

```
infrastructure_constraint {
    service_class:           string         // open vocab; e.g. "registry" | "lens" | "node"
                                            //   | umbrella "canonical"
    admission_quorum_basis:  "founders"     // REQUIRED literal "founders" — the set of members
                                            //   with role == founder. Admission/removal of any
                                            //   member is evaluated by consensus_protocol over
                                            //   THIS set, never over all members. (Contrast:
                                            //   geographic evaluates over the current member set.)
}
```

**Conformance requirements for an `infrastructure` community (trust-root grade):**

- `consensus_protocol` MUST be a `quorum:M/N` kind ([§8.1.13.2](08_composition.md)); `founder_only` / `unanimous` / bare `majority` are NON-conformant for `infrastructure` (a single founder must not be able to admit unilaterally; a growable core must not require all-N).
- `consensus_protocol_entrenched` MUST be `true` — the admission door cannot be lowered after founding. A `supersedes` that would weaken `consensus_protocol` or change `admission_quorum_basis` away from `founders` is a `hard_case:community_consensus_protocol_violation` ([§7.8](07_reserved.md)) and MUST be rejected by the substrate.
- `M/N` is the absolute-`M` reading per [§8.1.12.3.1](08_composition.md), evaluated over the **founder** subset (`role == founder`), independent of how many non-founder members exist.
- Content federates as `holds_bytes:sha256:*` per the community default; NO at-rest DEK cascade; [§10.1.4](10_endpoints.md) structural-invisibility does NOT apply (canonical-service trust data is public by design).

**Membership change ceremony**: same `supersedes` shape as `geographic`, but the admission predicate is `consensus_protocol` over the founder subset — there is no `location_proof` containment check. Adding a fourth+ operator (e.g., a new independent Node operator) is a founder-quorum event; the new member joins with `role: member` (non-founder) and does NOT thereby gain admission authority. Promoting a member to `role: founder` (widening the admission quorum basis) is itself a founder-quorum `supersedes` — the only way the door widens is by the existing door's consent.

**Worked example — the `ciris-canonical` trust root**:

```
community {
    community_key_id:                  "ciris-canonical",
    community_name:                    "CIRIS Canonical Services",
    cohort_subkind:                    "infrastructure",
    cohort_subkind_payload: {
        infrastructure_constraint: {
            service_class:             "canonical",       // umbrella: registry + lens + node
            admission_quorum_basis:    "founders"
        }
    },
    members: [
        {key_id: registry_steward_us,   role: founder},
        {key_id: registry_steward_eu,   role: founder},
        {key_id: registry_steward_apac, role: founder},
        // grows over time — Lens/Node installs + independent operators admitted
        // by 2-of-3 founder quorum, joining with role: member:
        // {key_id: lens_install_us,    role: member}, ...
    ],
    consensus_protocol:                "quorum:2/3",       // over the 3 founders
    consensus_protocol_entrenched:     true
}
```

Consumers pin `community_key_id: ciris-canonical` and resolve the live member set via `resolve_community` ([§8.1.13.1](08_composition.md)); they do NOT hard-pin per-install fingerprints. The Reticulum addressing dual: `community_key_id` is a CEG-directory binding (not a Reticulum destination) — resolve → path-request ⌈2/3⌉ founders → verify the quorum attestation; reachability is never an attestation ([§10.5.6](10_endpoints.md) / [§7.7](07_reserved.md)).

**Default trust, not forced root (normative, 1.0-RC3).** `ciris-canonical` is the **default** trust anchor a conformant CIRIS deployment ships pinned — it is **NOT a forced root**. A conformant consumer MUST be able to **re-root**: untrust the canonical group, pin a *different* `cohort_subkind: infrastructure` community instead or in addition, or run with none. `infrastructure` is a **general primitive** — any operator MAY emit their own governed trust-root community; `ciris-canonical` carries no privileged wire status, only a shipped default. Membership grows by the community's own `consensus_protocol` (founder-quorum vote, above), never by fiat. A forced root is a walled garden; a default-plus-re-root is a federation (M-1 autonomy/justice: unregulated standing without the steward's permission). *(All resolution is `key_id` → signed `transport_destination` → Reticulum, [§8.1.13.1.1](08_composition.md); DNS is never part of the trust or addressing chain — a deployment's HTTPS hostname, if any, is operational convenience outside this spec.)*

**Trust ≠ membership (normative, 1.0-RC7 — resolves [CIRISRegistry#83](https://github.com/CIRISAI/CIRISRegistry/issues/83) §1).** Pinning an `infrastructure` community as a trust root is **trust, not membership** — distinct relationships a consumer/substrate MUST NOT conflate:
- **Trust + serve** (no membership): a node that *trusts* an `infrastructure` community **serves** it — relays, stores, transports, serves its reads — **without being a member**. It holds no community DEK (infra has none — Commons-plaintext, [§8.1.13.3](08_composition.md)) and does **not** count in its `consensus_protocol`. Trust is the shipped default (above); serving follows from trust.
- **Membership** (standing *in* the group — counting in `consensus_protocol`, sharing its DEK where one exists, standing to speak AS the group): requires **admission by that community's own protocol** (founder-quorum for `infrastructure`, [§8.1.13.2](08_composition.md)). The three steward nodes ARE members (founders) of `ciris-canonical`; a generic node shipped pinned to canonical only **trusts + serves** it.

The worked-example member list above is the *founder/member* set; "ships pinned to canonical" (every conformant deployment) is the *trust* set — different populations. The "default trust anchor" language means the latter, never automatic membership.

**Owner-binding gate for non-infrastructure membership (normative, 1.0-RC7 — resolves [CIRISRegistry#83](https://github.com/CIRISAI/CIRISRegistry/issues/83) §2).** A key whose `identity_type` ([§7.0.1](07_reserved.md)) includes `node` or `agent` **MUST** have a bound **owner** — a `user`-role identity it is an admitted `identity_occurrence` of ([§5.6.8.8](#5688-identity_occurrence-subject_kind-ceg-07-addition)) with a **live `delegates_to(user → key, …)`** ([§3.2](03_primitives.md)) — **before it may be admitted to any non-`infrastructure` community** (`family` / `community` / org). It MAY **trust + serve** `infrastructure` communities with **no owner**. Rationale ([§1.5](01_foundation.md) / [§7.0.1](07_reserved.md)): non-infra membership is an **authority act** (standing to speak in the group), and authority MUST root in an accountable human, never a bare node — a fresh, unowned node is **canonical-trust-and-serve only** until owned. A substrate evaluating a non-infra `community` admission MUST reject a `node`/`agent`-role member lacking a live owner-binding. The owner-binding is a **precondition**, not a substitute for the vote — the admitting community's `consensus_protocol` still governs *whether* the owned key is admitted.

**Owner-binding as a substrate-resolvable predicate (normative, 1.0-RC25 — resolves [CIRISRegistry#95](https://github.com/CIRISAI/CIRISRegistry/issues/95) part 2).** A substrate decides ownership with this boolean: **`is_owner_bound(K)`** ≔ there is a live, unrevoked path from `K` to a `federation_keys` identity `U` with `user ∈ U.identity_type` ([§7.0.1](07_reserved.md)), where each step is one of — `K` *is* `U`; `K` is an admitted `identity_occurrence` of `U` ([§5.6.8.8](#5688-identity_occurrence-subject_kind-ceg-07-addition)); or a live `delegates_to(U → K)` ([§3.2](03_primitives.md)). A chain root satisfying `is_owner_bound` terminates at an **accountable human** (`user`-role); a `node`/`agent` key that does not is owner-less. This is the concrete predicate the gate above and the [§11.10](11_governance.md) clause-(b) "owner-bound root" check — not rhetoric.

**Trust and consent are distinct, role-scoped relationships (normative, 1.0-RC3 — do not conflate in consumer policy).** **Trust is inbound** — accepting what a member *produces*; **consent is outbound** — letting one's own data *flow to* a member. They are independent per role:
- **lens (observation):** *trust* = consume its `detection:*` scores; *consent* = your traces flow to it.
- **registry (authority):** *trust only* — there is no trace-flow to consent to; the founder-quorum is a **closed mutual-trust set** (the core trust each other).
- **node (consensus):** *trust* = accept its deferral / vote / moderation outcomes; *consent* is **medium-dependent** (moderation, routing, voting, …) but always expressed through the **one** `consent:*` object ([§5.6.8.6](#5686-consent-namespace-family-ceg-06-addition) / [§5.6.8.7](#5687-consent_record-subject_kind-ceg-06-addition)) — same primitive, many surfaces.

A consumer MAY hold any combination across role × axis (trust a member's output while refusing it data, or the reverse). There is no single "trust the community" switch.

**Why this is still 1+4**: `infrastructure` adds one value to the open `cohort_subkind` vocabulary and one optional `infrastructure_constraint` payload shape. It rides existing `scores` + subject_kind discriminator; admission rides existing `consensus_protocol` + `supersedes`; the founder-subset evaluation basis is a *consumer/substrate evaluation rule over existing fields* (`role == founder`), not a new structural primitive. Zero new structural primitives — eleventh-path confirmation.

#### §5.6.8.11 `location_proof` subject_kind

Per [CIRISRegistry#48](https://github.com/CIRISAI/CIRISRegistry/issues/48). The wire-format primitive for a subject's rough-location declaration. Required for admission to `cohort_subkind: geographic` communities ([§5.6.8.10](#56810-community-subject_kind-ceg-08-addition)); MAY be used independently as a stand-alone disclosure.

```
location_proof {
    subject_key_id:                    key_id                    // the asserting party's
                                                                 //   federation_keys.key_id
    cell_id:                           string                    // H3 cell, lowercase hex per §0.8
    cell_resolution:                   u8                        // MUST be ≤ 7 per §0.8.1
    asserted_at:                       rfc3339_canonical
    valid_until:                       Option<rfc3339_canonical> // null = indefinite (but consumer
                                                                 //   policy SHOULD treat as stale
                                                                 //   after 30 days for liveness)
    attestation_evidence:              Option<base64>            // optional hardware-attested
                                                                 //   location claim from ciris-keyring
                                                                 //   (TPM / Secure Enclave) — null for
                                                                 //   software-only / self-asserted
}
```

**Substrate does NOT verify location truth.** No GPS oracle exists at this layer; the substrate cannot independently confirm that a key in Austin actually emitted from Austin. The truth-grounding is consumer-side:

- The community's `consensus_protocol` admission decides whether to accept the claim (e.g., `majority` of existing Austin members vote to admit, presumably because they have out-of-band evidence the candidate really is in Austin)
- The `attestation_evidence` field MAY carry hardware-attested location data (e.g., a TPM-signed GNSS fix from a known-good device) for higher-assurance communities
- Repeat offenders (claim-Austin-then-emit-from-Tokyo) get caught by consumer-side detection (LensCore composition; not substrate-side gate)

**Rough-only is wire-format-enforced.** Per [§0.8.1](00_conformance.md): `cell_resolution ≤ 7`. Producers attempting finer resolution have admission rejected; substrate emits `hard_case:location_proof_resolution_violation` ([§7.8](07_reserved.md)).

**Typical cohort_scope**: `federation` (the disclosure IS the opt-in; non-private by design). Producers MAY scope to `community` with a specific `community_id` if they want the proof readable only by that community's members — but then they re-emit for each community they want admission to. Operator/UI choice.

**Lifecycle**:

- `asserted_at` + optional `valid_until` per envelope
- `withdraws` against a `location_proof` evicts forward visibility (consumer policy treats the subject as "no current location proof" for community admission purposes from withdrawal-time forward)
- The withdrawn `location_proof` remains in the audit chain — per [§3.2](03_primitives.md) `withdraws-isn't-retroactive`, leaving doesn't un-disclose

**Composition with `consent_record` (CEG 0.6)**: a subject who wants to withdraw their location_proof AND compel deletion from substrate may emit a `consent_record` with `stance: revoked` + `scope: [retain, share]` against the location_proof Contribution. The substrate-side consent SLA watcher (CEG 0.6 [§8.1.11.3](08_composition.md)) clocks producer compliance. Note: this is the consent-revocation surface, distinct from the structural withdraws-forward-only semantic above.

**1+4 preserved** — rides `scores` + subject_kind discriminator ([§5.6.8](#568-content-ingestion-prefixes) mechanism); consent revocation composes via CEG 0.6 primitives; [§1.4](01_foundation.md) ninth path.

#### §5.6.8.12 `settlement` — CEG↔value-transfer linkage (CEG 0.14 addition)

Per [CIRISRegistry#59](https://github.com/CIRISAI/CIRISRegistry/issues/59) (decision + SOTA) + [CIRISAgent#859](https://github.com/CIRISAI/CIRISAgent/issues/859) (impl). Value transfer itself is **not** a CEG primitive — it rides external rails (USDC on Base via x402, keyed to the federation signing key under the **Identity = Wallet** principle). The `settlement` primitive is the **optional, privacy-scoped attestation that *links* a federation action to its off-stack settlement** — so a paid relationship (paid stream / tip / subscription / paid `event_listing` / compute-job) can be federation-auditable *or* privately recorded, producer's choice. **CEG records that a settlement happened and binds it to what it paid for; the chain settles the value.** Clean separation of concerns.

**Why this is in CEG's lane (and why it's optional):** the linkage is a *trust fact* ("is this a real / authorized / paid relationship?"), exactly what consumer policy composes over. The 2026 agent-commerce + on-chain-privacy markets converged on the same mechanism — *a verifiable receipt exists, with selectively-disclosed contents* (x402 optional receipt header; append-only verifiable metering logs; "privacy + compliance via selective disclosure"). CEG already has every needed primitive, so this is composition, not invention.

**Two admitted shapes (parallel to `consent_record` [§5.6.8.7](#5687-consent_record-subject_kind-ceg-06-addition)):**
- **Primitive:** a bare `scores` on a `settlement:*` dimension against the paid Contribution (the common case).
- **Ceremony:** the `settlement` subject_kind envelope when an explicit receipt record is wanted.

```
settlement {
    settled_action_ref:   contribution_id    // the federation action this paid for
                                              //   (or via subject_key_ids / topical_relation:settles)
    rail:                 string             // open vocab; e.g. "base:usdc", "stellar:usdc", "x402"
    settlement_ref:       string             // chain tx hash / x402 receipt id — cited the way
                                              //   evidence_refs[] cite a SHA blob (§10.1): a settlement
                                              //   is just another evidence reference
    amount_commitment:    Option<string>     // cleartext "12.50" (public case) OR a hash/range
                                              //   commitment (private/selective-disclosure case)
    settled_at:           rfc3339_canonical
    // visibility via the envelope cohort_scope: default `self` (payer+payee only);
    // `public` opt-in for transparency (e.g. creator revenue, DAO treasury flows)
}
```

**Self-authenticating (Identity = Wallet):** the same federation key that signs this attestation controls the Base wallet that emitted `settlement_ref`, so "I settled `tx` for action X" is self-proving — no oracle needed. The payee MAY counter-attest (`scores` on `settlement:received:*`) for a bilateral receipt.

**Privacy is the default, auditability is opt-in.** Visibility rides the existing gradient: `cohort_scope: self` (the parties only; the §10.1.4 structural-invisibility discipline applies — no `holds_bytes` leak) by default; `cohort_scope: public` opt-in; amounts MAY be committed rather than cleartext; viewing-key / ZK selective disclosure composes later without a wire change. This matches "configurable-privacy-by-default" — the federation log is **not** a public payment trail unless the producer chooses it.

**Lifecycle**: `withdraws` against a `settlement` is forward-only (it does not un-happen the on-chain settlement — leaving doesn't un-pay, parallel to `location_proof`); a *disputed* or *refunded* settlement is a new `settlement` / `scores` referencing the original via `supersedes` or `topical_relation`. CEG never reverses value — it only records the subsequent state.

**1+4 preserved** — rides `scores` + subject_kind discriminator ([§5.6.8](#568-content-ingestion-prefixes) mechanism); `settlement_ref` rides the existing `evidence_refs[]` external-reference pattern; visibility rides existing `cohort_scope`. Fourteenth path ([§1.4](01_foundation.md)) — the wire format expresses **commerce-relationship auditability** (the receipt, not the rail) as composition, closing the last form of internet traffic from the completeness audit (CIRISRegistry#59).

#### §5.6.8.13 Operational-data subject_kinds — `organization` / `org_membership` / `partner_record`

Per [CIRISRegistry#70](https://github.com/CIRISAI/CIRISRegistry/issues/70) (design + three-sided federation review: Edge / Verify / Persist) under the [CIRISRegistry#58](https://github.com/CIRISAI/CIRISRegistry/issues/58) Spock-removal epic. Cross-region **operational Portal data** — organizations, memberships, licenses/partners — becomes signed CEG envelopes replicated by the same anti-entropy carrier as trust data ([CIRISEdge#65](https://github.com/CIRISAI/CIRISEdge/issues/65) v2 wire, `WIRE_PROTOCOL_VERSION 0x02`). This is the **one scheduled additive item** on the 1.0-RC1 frozen surface (README freeze declaration).

**Governing principle (normative): federate the trust/authz-minimal projection; everything else stays region-local.** The federated envelope carries only what the federation needs to enforce trust and authorization cross-region. PII and business detail live in the Registry's own per-region store (today's Portal tables), are NEVER emitted into an operational envelope, and never federate. **Projection minimization is the Registry's emit-side discipline — the Registry is the security-boundary owner**; the substrate's role is admission + merge, and it is explicitly NOT a PII filter (it stores what is signed and emitted).

All three ride existing `scores` + subject_kind discriminator. **Replication wire tokens (normative, snake_case): `organization` · `org_membership` · `partner_record`** — the Edge v2 `EnvelopeKind` additions; v2 `envelope_hash` basis = `sha256(JCS(inner envelope))` per [§0.9](00_conformance.md)/[§0.9.2.1](00_conformance.md). All three are **Commons tier** ([§8.1.13.3](08_composition.md)) — plaintext at rest; the projection is world-readable by design.

```
organization {
    org_id:           uuid          // FIRST-CLASS: substrate MUST index as a row column (stable-id resolution below)
    name:             string
    org_type:         OrgType       // internal | partner | licensee | community (the proto enum)
    parent_org_id:    Option<uuid>  // licensee under partner
    partner_id:       Option<uuid>  // link to partner_record
    status:           active | suspended | deactivated
    asserted_at:      rfc3339_canonical          // per §0.5; LWW ordering field
    valid_until:      Option<rfc3339_canonical>
}
// REGION-LOCAL (never federates): tax_id, billing/technical/compliance/primary emails,
// oauth_provider/oauth_domain, metadata, created_by.

org_membership {
    user_id:          uuid          // FIRST-CLASS (with org_id): substrate MUST index (user_id, org_id)
    org_id:           uuid
    role:             OrgAdmin | KeyManager | Operator | Viewer
    status:           active | deactivated
    asserted_at:      rfc3339_canonical
    valid_until:      Option<rfc3339_canonical>
}
// REGION-LOCAL (never federates): the entire User PII record — email, name,
// oauth_provider/oauth_subject, last_login_at, mfa_enabled/mfa_method, invited_by.
// Consequence: role-based authz works federation-wide; email→user login resolution
// is home-region-local.

partner_record {
    license_id:       uuid          // FIRST-CLASS: substrate MUST index as a row column
    partner_id:       uuid
    org_id:           uuid
    license_type:     LicenseType   // community | community_plus | professional_* | professional_full
    capabilities_granted:  [string]   // SET-SEMANTICS → lexicographically sorted (§0.9.2.1 rule 1)
    capabilities_denied:   [string]   // SET-SEMANTICS → sorted
    max_autonomy_tier:     A0..A4
    requires_supervisor:   bool
    geographic_restrictions: [string] // ISO country codes; SET-SEMANTICS → sorted
    allowed_identity_templates: [string] // SET-SEMANTICS → sorted
    deployment_limit:      u32
    offline_grace_hours:   u32
    status:           active | suspended | revoked
    revision:         u64           // MONOTONIC per license_id — admission REJECTS any decrease
                                    //   (the F-AV-ROLLBACK discipline; the merge orders on this,
                                    //   so a stale `active` can never overwrite a revoke)
    issued_at:        rfc3339_canonical
    expires_at:       rfc3339_canonical
    asserted_at:      rfc3339_canonical
}
// No PII split — the partner_record IS the world-verifiable grant; it federates whole.
```

**Set-semantics declaration (normative — the [#70](https://github.com/CIRISAI/CIRISRegistry/issues/70) Verify catch).** `capabilities_granted[]` / `capabilities_denied[]` / `geographic_restrictions[]` / `allowed_identity_templates[]` are **set-semantics → lexicographically sorted by JCS string form ([§0.9.2.1](00_conformance.md) rule 1)** — and this is *more* than a single-signer determinism rule here: `partner_record` is signed by **M distinct stewards**, and the M-of-N quorum verifies only if all M sign **byte-identical** JCS canonical bytes. Unsorted capability arrays make two stewards who agree on the same grant produce different bytes — the quorum silently collapses and the license fails to admit cross-region. The [CIRISConformance#9](https://github.com/CIRISAI/CIRISConformance/issues/9) vector set MUST include the M-of-N identical-bytes round-trip (M independent canonicalize+sign of one grant → one verifiable signature set). Any unordered list inside a constraints object carries the same declaration.

**No payment-processor data (normative — fail-secure).** An operational envelope MUST NOT carry Stripe-derived or any payment-processor-derived data (customer ids, subscription ids, charge refs, card metadata) — **including via any open-vocabulary field**. Substrate admission MUST reject an operational envelope carrying recognizable payment-processor identifiers (defense-in-depth; the Registry's emit-side minimization is the primary control). Billing remains entirely Portal+Stripe, off-wire ([CLAUDE.md](../../CLAUDE.md) discipline; consistent with [§5.6.8.12](#56812-settlement--cegvalue-transfer-linkage-ceg-014-addition) keeping value-transfer rails off-wire).

**Write authority — two shapes, two verifiers (normative).**
- `organization` / `org_membership`: **single authorized signer, role-gated** — the envelope is admitted iff `attesting_key_id` holds the required role for the operation, established by a prior non-superseded `org_membership` (rooted at org creation by a steward/system authority). Verification reuses the **[§8.1.12.7.1](08_composition.md) `delegates_to` role-chain resolver** (CIRISVerify#63) — explicitly NOT founder-quorum; implementers MUST NOT build a third bespoke path.
- `partner_record`: **M-of-N steward quorum** — the signature *set* over the identical JCS bytes is verified at admission by the **[§5.6.8.10](#56810-community-subject_kind-ceg-08-addition) founder-quorum machinery** (`verify_founder_quorum`, CIRISVerify#31). Professional capability grants are federation-wide; a single compromised key MUST NOT be able to forge one.
- **The two quorums are distinct (normative — do not conflate):** (1) the **steward-signature admission quorum** above (signer authority, verified by Verify at admit) and (2) the **region merge quorum** (`quorum_weight`, the substrate `MergeBallot` tier-1 ordering during cross-region merge — [§10.1.6](10_endpoints.md)). Different mechanisms, different layers, different owners; the substrate's merge logic never counts steward signatures.

**Mutability + current-state resolution (normative — stable-id grouping, NOT chain-walk).** Updates ride `supersedes`; deactivation/revocation rides `withdraws` (the [§5.6.8.5](#5685-event-lifecycle-dimension-families-ceg-04-addition) `event_listing` state-machine pattern). **Current state of a business id is resolved by stable-id grouping**: group all envelopes by the first-class business id (`org_id` / `(user_id, org_id)` / `license_id`) → apply `withdraws` forward-only → latest `asserted_at` (skew-bounded per [§10.1.6](10_endpoints.md)) → tie-break smallest `attestation_id` (the [§6.1](06_relations.md) discipline). For `partner_record`, admission anti-rollback on `revision` precedes the [§10.1.6](10_endpoints.md) quorum merge. Resolution MUST NOT require chain completeness — a region that never observed envelope N−1 still converges (partition tolerance is the point of CEG-native replication). `supersedes` references SHOULD be emitted when the prior is known and serve as **audit lineage only** — decoration, never resolution.

**1+4 preserved** — rides `scores` + subject_kind discriminator ([§5.6.8](#568-content-ingestion-prefixes) mechanism); license authority rides the existing §5.6.8.10 founder-quorum machinery; role authority rides the existing §8.1.12.7.1 delegation resolver; merge intents are substrate dispatch declarations ([§10.1.6](10_endpoints.md)), not wire primitives. Sixteenth path ([§1.4](01_foundation.md)) — the wire format expresses **the federation's own operational/administrative layer** (the org/identity/license records that run the federation's business) as composition, completing the Spock-removal arc: after this, no cross-region byte moves outside a signed CEG envelope.

#### §5.6.8.14 `identity:canonical_binding` — claiming a canonical-hash subject

Per [CIRISPersist#146](https://github.com/CIRISAI/CIRISPersist/issues/146) Ask 6 + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842) Gap 3. A `subject_key_ids[]` entry MAY be a **canonical-hash** identifier — `sha256("discord:user_id:12345")`, an external-party id — rather than a `federation_keys` row ([§4.2.2](04_envelope.md)). When the real-world subject behind a canonical hash **later acquires a federation_keys identity**, it needs a wire-format way to **claim** that hash so its revocation authority (and proxy-delegation eligibility) attaches. That is the rebinding ceremony.

**Shape.** A bare `scores` Contribution on the reserved dimension **`identity:canonical_binding:{canonical_hash}`** — `attesting_key_id = K` (the claiming federation key), naming the canonical hash `H` as the bound subject. **Self-asserted** (`witness_relation: self`): K declares "I am the federation identity behind H." Hybrid-signed; admitted federation-tier (it grants authority — not local-tier-eligible, [§10.1.3](10_endpoints.md) discipline).

```
scores {
    attesting_key_id: K,                              // the claiming federation_keys identity
    dimension:        "identity:canonical_binding:{H}", // H = the canonical hash being claimed
    score:            <positive>,
    witness_relation: self,
    asserted_at:      rfc3339_canonical,
}
```

**Admission consequence (normative).** After an admitted `identity:canonical_binding` from `K` → `H`, the substrate widens `withdraws` admission ([§3.2.3](03_primitives.md)): a `withdraws` from `K` against any target `T` where `H ∈ T.subject_key_ids[]` is now admitted (K has inherited the canonical-hash subject's revocation authority). **This is what unblocks [§3.2.3 rule 3](03_primitives.md)** — a proxy `delegates_to` to a canonical-hash subject presumes that subject can hold a `delegates_to.attested_key_id`; a never-rebound canonical subject acquires it here.

**Authorization is consumer-policy, not wire (normative honesty).** CEG pins the binding *shape*, NOT proof that K legitimately controls H's preimage. A binding is a *self-assertion*; a consumer weights it by whatever proof-of-control it trusts (OAuth/IdP verification of the `discord:user_id`, an out-of-band attestation, TOFU). The substrate admits the binding and records it; **the trust that K==H is composed by the consumer**, exactly as the [§5.6.8.8.1](#5688-1-transport_destination--the-authenticated-identityaddress-binding-ceg-012-addition) announce is advisory until rooted. A second key claiming the same `H` is admitted too (competing claims surface to consumer policy / RATCHET, not a substrate verdict). **1+4 preserved** — `identity:canonical_binding:{H}` is a reserved `scores` dimension, not a new primitive.

#### §5.6.8.15 `consent:replication` — directed federation-peer replication consent (CEG 1.0-RC28 addition)

Per [CIRISRegistry#97](https://github.com/CIRISAI/CIRISRegistry/issues/97). An **open-vocabulary member of the [§5.6.8.6](#5686-consent-namespace-family-ceg-06-addition) `consent:*` family** ([§11.2.1](11_governance.md) discipline) — **not a new structural primitive** (the 1+4 surface is frozen since RC1; this rides the existing `scores` attestation_type, adds no envelope field). It names the one consent shape the prior family did not: a fabric **node's** standing grant to replicate a class of its own attestations to a **named peer node**, as distinct from a **subject's** consent over a target Contribution.

**The problem it solves.** Cross-node propagation is governed by [§8.1.13.3 / §10.1.4](10_endpoints.md) `cohort_scope`. A node **inside** a community (e.g. the [§5.6.8.10](#56810-community-subject_kind-ceg-08-addition) / [§14](14_glossaries.md) `ciris-canonical` infrastructure community) shares with co-members by community-cohort membership — no per-peer object needed. A node **outside** that group has no membership edge to ride, so an out-of-group peering needs an **explicit, auditable, revocable** consent object. Concrete case: an in-group lens node (CIRISServer) replicating `capacity:*` to an out-of-group monitoring node (CIRISStatus), which replicates [`health:liveness:v1`](#566-canonical-leaf-glossary) (RC20 / [#91](https://github.com/CIRISAI/CIRISRegistry/issues/91)) back. `consent:replication` is that object.

**Shape.** A bare `scores` Contribution on the dimension **`consent:replication:{version}`** (canonical `consent:replication:v1`) — `attesting_key_id = G` (the granting node), the recipient peer named in `subject_key_ids[] = [P]`. Standing (not bound to a target Contribution). Hybrid-signed; admitted **federation-tier** (it authorizes cross-node flow — not local-tier-eligible, [§10.1.3](10_endpoints.md) discipline). `cohort_scope: federation` (the grant itself is a public governance record).

```
scores {
    // ── envelope-level (§4 table — frozen surface, untouched) ──
    attesting_key_id: G,                          // the granting (sending) node
    dimension:        "consent:replication:v1",
    score:            <positive>,                 // positive-only grant; a withdraws/recants retracts (never a negative score)
    subject_key_ids:  [P],                         // the single recipient peer authorized to receive
    cohort_scope:     "federation",
    witness_relation: "self",                      // REQUIRED — G attests its OWN replication intent
    valid_until:      <optional rfc3339>,          // optional — time-boxed peering (§5.6.8.7 staleness)
    // ── payload-level (§4.2.2.3 — subject_kind selects the schema; NOT envelope fields) ──
    subject_kind:         "consent_replication",
    grants:               "replication",           // constant
    attestation_prefixes: ["capacity:"],           // §0.9 JCS array, sorted ascending + deduplicated
    asserted_at:          rfc3339_canonical,
}
```

**Admission is by key registration; consent is the governance record (normative honesty).** The substrate gate that lets P's corpus *admit* G's replicated rows is **G's key existing in P's `federation_keys`** (registration), plus the [§7](07_reserved.md) reserved-prefix identity rules. `consent:replication` does **not** add a substrate admission check — by design, exactly as [§5.6.8.14](#56814-identitycanonical_binding--claiming-a-canonical-hash-subject) authorization is consumer-policy, not wire. What it provides is the **auditable, revocable, bilateral record of intent**: the wire-format answer to "did G consent to send this to P, and for which prefixes?" Each direction is an independent unilateral grant (G→P and P→G are two separate `consent:replication` Contributions); a bilateral peering is ratified iff both are present.

**Revocation (normative).** A `withdraws`/`recants` ([§3.2.3](03_primitives.md)) from G against its own `consent:replication` grant retracts the consent. Because admission is key-rooted (above), revocation has teeth only if honored: on revoke, **the granting node MUST cease replicating the named prefixes to P and SHOULD deregister/expire P's directory authorization for them**, and **a consumer MUST treat rows replicated from G under a withdrawn grant as non-conformant** (the [§11](11_governance.md) location-proof precedent — the wire cannot un-send bytes a peer already holds; it can mark forward-only and oblige cessation). A grant carries optional `valid_until` ([§5.6.8.7](#5687-consent_record-subject_kind) semantics) for time-boxed peering.

**Conformance shape (locked, normative — 1.0-RC29; resolves [CIRISRegistry#98](https://github.com/CIRISAI/CIRISRegistry/issues/98)).** The grant is conformance-gradable as follows. Its **envelope-level** fields are exactly `attesting_key_id = G`, `dimension = "consent:replication:v1"`, `score > 0` (positive-only — the family's `consent:state:granted` polarity; magnitude is not load-bearing and a retraction is a `withdraws`/`recants`, never a negative score), `subject_key_ids = [P]` (the **single** recipient peer), `cohort_scope = "federation"`, `witness_relation = "self"` (**REQUIRED** — a G→P grant is G attesting about its *own* replication intent; pinning `self` is what forecloses a third party forging a grant in G's name, since only G signs with G's key as the attested-intent-holder), and optional `valid_until` (the [§4](04_envelope.md) envelope field, [§5.6.8.7](#5687-consent_record-subject_kind) staleness semantics) for time-boxed peering. The grant's parameters — `grants` (the constant `"replication"`) and `attestation_prefixes` — are **payload-level** members ([§4.2.2.3](#4223-subject_kind-is-a-payload-level-discriminator-confirm-4-resolution)) carried under `subject_kind: "consent_replication"`, **NOT** envelope fields: this is *exactly* why 1+4 is preserved — the §4 envelope table is untouched. `attestation_prefixes` is the [§0.9](00_conformance.md) JCS-canonical array of [§5](05_namespace.md) namespace-prefix strings G consents to replicate (trailing `:` significant — e.g. `"capacity:"`), **sorted ascending + deduplicated**, so two implementations holding the same grant agree byte-for-byte on `(G, P, prefix-set, validity)` and revocation-scope matching is deterministic.

**Bilateral pairing + partial revocation (normative).** `G → P` and `P → G` are independent unilateral grants; each SHOULD carry `topical_relation: bilateral_pair` (the [§5.6.8.6](#5686-consent-namespace-family-ceg-06-addition) `consent:partnership_grant`/`consent:partnership_accept` precedent) so a consumer can pair them — a bilateral peering is ratified **iff both are present and live**. A `withdraws`/`recants` against a grant retracts it **whole**; a producer **narrowing** the prefix set (dropping `capacity:` while keeping another) MUST `supersedes` ([§3.2.3](03_primitives.md)) the grant with a new one carrying the narrower `attestation_prefixes` — it MUST NOT silently drop a prefix from a still-live grant (a silent narrowing is indistinguishable, to a consumer, from no change — and the cessation obligation below can only attach to an explicit retract/supersede).

**1+4 preserved** — `consent:replication:{version}` is an open-vocabulary `consent:*` dimension on the existing `scores` type ([§11.2.1](11_governance.md)); no new attestation_type, no new envelope field (the `grants` / `attestation_prefixes` parameters are payload-level per §4.2.2.3, above), no new replication `EnvelopeKind` (it replicates as an ordinary `Attestation`). The frozen wire surface is untouched.

## §5.7 RATCHET — anti-Sybil / Counter-RII flags

**Owner**: [`RATCHET/FSD.md`](https://github.com/CIRISAI/RATCHET/blob/main/FSD.md).

RATCHET emits **advisory** flags — never autonomously modifies ledger state. Reads federation audit chains; emits scoring inputs to NodeCore's moderation flow.

`ratchet:flag:out_of_distribution_voting` / `ratchet:flag:coordinated_voting_cluster` / `ratchet:flag:density_anomaly` / `ratchet:flag:expertise_attestation_anomaly` / `ratchet:flag:counter_rii:{layer}` / `ratchet:flag:harassment_pattern`. Polarity: signed.

**Critical enforcement**: `ratchet:flag:*` cannot be sole evidence for `slashing:*`. WA quorum is the load-bearing gate.

## §5.8 CIRISBench — HE-300 benchmark outcomes

**Owner**: [`CIRISBench/README.md`](https://github.com/CIRISAI/CIRISBench).

| Prefix | Description | Polarity |
|---|---|---|
| `benchmark:he300:{category}:{version}` | HE-300 score on category (`commonsense`, `commonsense_hard`, `deontology`, `justice`, `virtue`) at version (`v1.0` / `v1.1` / `v1.2`). | positive-only |

## §5.9 CIRISRegistry — identity / build / license / partner

**Owner**: this Registry. Cited from [`../MISSION.md`](../../MISSION.md) §3.4 + FSD-001 + protocol/ciris_registry.proto.

| Prefix | Description | Polarity | Reserved? |
|---|---|---|---|
| `licensure:{authority_id}` | License status — issued / revoked / expired — for a key under a named authority. Co-owned with Verify. | signed | Co-owned |
| `partner_role:{role}` | Partner status (COMMUNITY / COMMUNITY_PLUS / PROFESSIONAL_MEDICAL / PROFESSIONAL_LEGAL / PROFESSIONAL_FINANCIAL / PROFESSIONAL_FULL). | enumerated | No |
| `revocation:{entity_type}:{reason}` | Entity revocation (`agent` / `partner` / `license`). Immediate, non-rollbackable. | -1 only | No |
| `bond_posted:{currency}` | Bond posted per $1-Sybil-resistance per PoB; forfeited on revocation. | positive-only | No |
| `build:registered:{target}` | Build manifest registered against the directory (precondition for L4 attestation). | boolean-via-score | No |
| `multilateral_participation:{forum}:{kind}` | Depth of a partner's participation across federated bodies. `{forum}` = named federated body or compact; `{kind}` ∈ `membership` \| `voting` \| `proposal_filing` \| `observer_status`. | signed | No |
| `agent_files:{kind}:{platform_or_target}` | **Joint claim with [§5.6.7](#567-files-as-contributions-joint-claim) NodeCore.** Canonical-attester rule: registry-steward-triple attestations constitute the CIRIS canonical default-trust state. Anti-tricking guarantee at `registry.ciris-services-1.ai/install` per [§8.1.6](08_composition.md) trust-composition policy. Open Contribution channel; consumer policy composes via [§8.1.6](08_composition.md) trust layers. | signed | No |
| `accord:*` | **Reserved** — only `identity_type=accord_holder` may emit. The one constitutional asymmetry. | see [§7.1](07_reserved.md) | **Yes — [§7.1](07_reserved.md)** |

## §5.10 Namespace summary

**83 prefix families** total across 8 owning components (CEG 0.2).

Lineage:
- FSD-002 v1.0 baseline: 73 families (initial namespace stabilization)
- v1.1 added 1: `detection:correlated_action:{axis}` (LensCore; renamed from `detection:emergent_deception:{axis}` in v1.2 per [§1.3.1](01_foundation.md))
- v1.3 added 3: `multilateral_participation:{forum}:{kind}`, `locality:decision:{scale}`, `detection:distributive:access:{resource_type}` (+ envelope field `witness_relation`)
- v1.4 added 4: `agent_files:{kind}:{platform_or_target}` (joint), `holds_bytes:sha256:{prefix}`, `testimonial_witness:{kind}`, `need:{domain}:{kind}` (+ envelope field `oversight_mode`)
- v1.4.1 added 2: `provenance:build_manifest:{target}:locale:{lang_code}`, `provenance:skill_import:{source}`
- v1.4.2 added 3 envelope fields: `occurrence_id`, `occurrence_count`, `occurrence_role`
- v1.4.3: canonical-bytes contracts pinned in §5.2.1; Goal substrate cross-ref documented
- **CEG 0.1**: opened `testimonial_witness:{kind}` to open vocabulary; surfaced `hard_case:{kind}` open vocabulary in §5.6.6; added `biosphere` to [§2](02_grammar.md) Scope axis; added `topical_relation:translation_of` sub-leaf in §5.6.8 (LIVE per CIRISNodeCore b1582cb); documented "Trust-Fresh" composition pattern in [§8.1.7](08_composition.md); added Tiered-Scope Composition pattern in [§8.1.8](08_composition.md). All polarity columns now populated.
- **CEG 0.2** (wire break): renamed §5.2 attestation-ladder prefixes from `attestation:l{N}:*` to mechanism-only form (`attestation:self_verify`, `attestation:hardware_rooted`, `attestation:registry_consensus`, `attestation:license_validity`, `attestation:agent_integrity`) per [§1.3.1](01_foundation.md) T2 honest application — L-numbers name ladder-position (a verdict-shape) not mechanism. The L1-L5 ladder is now consumer-side composition per [§8.1.9](08_composition.md) Policy I — Attestation-Ladder Composition. Deprecated wire shape added to [§13.1](13_anti_patterns.md).
- **CEG 0.3** (additive; per CIRISRegistry#37 + #38 + #39): multimedia tier + governance additions. **Two new subject_kinds** documented: `takedown_notice` (with `LegalBasis` closed-set enum of 10 values + per-basis discipline) and `key_grant` (with `wrap_algorithm` + `scope` enums + `rotation_chain` semantics). **Five new external_content sub_kinds**: `image`, `audio`, `video`, `film`, `model_3d` (+ Phase 2 `live_stream`). **Four new dimension families**: `content_rating:{scheme}:{rating}`, `content_class:{class}`, `cw_class:{class}`, `age_assurance:{level}`. **Five new media-prefix families**: `image:*`, `audio:*`, `video:*`, `film:*`, `model_3d:*`. New composition policy ([§8.1.10](08_composition.md)) for trusted-publisher path + age-assurance gating. New governance sections ([§11.4](11_governance.md) fast-path takedown coordination + [§11.5](11_governance.md) hash-database operator policy). **1+4 wire-format lockdown preserved** — retire-key-grant rides existing `supersedes`; takedown propagation rides existing `withdraws`-against-`holds_bytes`; no new structural primitives.
- **CEG 0.4** (additive; per [CIRISRegistry#40](https://github.com/CIRISAI/CIRISRegistry/issues/40) + [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 1 closure at [d0a443a](https://github.com/CIRISAI/CIRISNodeCore/commit/d0a443a)): time-bound state-bearing content. **One new `external_content` sub_kind**: `event_listing` (Eventbrite / Meetup / Lu.ma / calendar / RSVPs / ticketing) with Source struct documented at NodeCore SCHEMA §4.29. **One new dimension family group** ([§5.6.8.5](#5685-event-lifecycle-dimension-families-ceg-04-addition)): `event:lifecycle:{state}` (open / cancelled / completed / superseded) + `event:rsvp_count` + `event:attendance`. **Two new canonical `topical_relation:{kind}` entries** (documentation-only registry additions; no amendment): `rsvps` (RSVP attestation against an event) + `vod_of` (reserved for the deferred live_stream→video relationship). **1+4 wire-format lockdown preserved** — lifecycle state machine composes from `withdraws` / `supersedes` / `delegates_to` + the new dimension's latest non-superseded emission; no new structural primitives. **`live_stream` remains deferred** ([CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 2 not yet shipped; substrate-side Edge + Persist decisions pending) — CEG 0.4 codifies only what NodeCore shipped, per the downstream-demand-pulls-CEG-additions discipline established with 0.3.
- **CEG 0.5** — *in flight* (codification pending) per [CIRISRegistry#44](https://github.com/CIRISAI/CIRISRegistry/issues/44) + [CIRISNodeCore#26](https://github.com/CIRISAI/CIRISNodeCore/issues/26) + [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142): `live_stream` promotion + chunk-DAG composition. Lands when NodeCore#26 substrate decisions ratify. Additive at the namespace layer (no envelope change).
- **CEG 0.8** (additive at the envelope layer; per [CIRISRegistry#48](https://github.com/CIRISAI/CIRISRegistry/issues/48)): **`community` subject_kind + `location_proof` subject_kind + `cohort_subkind: geographic` discriminator + wire-format-enforced rough-only location precision.** Sibling to CEG 0.7 `family` but with different defaults (community content federates per status quo; no at-rest DEK cascade; [§10.1.4](10_endpoints.md) structural-invisibility does NOT extend to community). Adds: **two new subject_kinds** — `community` ([§5.6.8.10](#56810-community-subject_kind-ceg-08-addition); larger node-collective with `cohort_subkind` discriminator; canonical `geographic` subkind with `geographic_constraint` payload; same six `consensus_protocol` kinds as family) and `location_proof` ([§5.6.8.11](#56811-location_proof-subject_kind-ceg-08-addition); H3 cell_id + cell_resolution ≤ 7 rough-only enforcement; optional `attestation_evidence` from ciris-keyring). **One new envelope field** ([§4](04_envelope.md)): `community_id` required iff `cohort_scope == community` (parallel to `family_id` but semantics differ — community federates per status quo). **One new canonicalization section** ([§0.8](00_conformance.md)): H3 cell canonicalization (lowercase hex; resolution-redundancy check; rough-only enforcement; containment semantics). **Three new substrate-emitted reserved prefixes** ([§7.8](07_reserved.md)): `hard_case:community_membership_change:*` + `hard_case:community_consensus_protocol_change:*` + `hard_case:community_consensus_protocol_violation:*` + `hard_case:location_proof_resolution_violation` (4-prefix total). **New composition policy** ([§8.1.13](08_composition.md)) Policy M — community membership composition + geographic admission gate (parallel to Policy L but without at-rest cascade). **New governance section** ([§11.8](11_governance.md)) geographic-community privacy invariant — joining is opt-in disclosure; rough-only is wire-format-enforced; leaving is forward-only (the audit chain preserves the historical claim). **1+4 wire-format lockdown preserved** — zero new structural primitives; both new subject_kinds ride existing `scores` + subject_kind discriminator; admission gates ride existing `consensus_protocol` machinery from CEG 0.7. Ninth independent path confirming 1+4 minimal-and-adequate ([§1.4](01_foundation.md)) — demonstrates the wire format can express **rough-precision geospatial constraints as canonicalization rules** (§0.8) + subject_kind admission gates, without new structural primitives.
- **CEG 0.10** (additive at the envelope+endpoint+composition layer; per [CIRISRegistry#44](https://github.com/CIRISAI/CIRISRegistry/issues/44) absorbed + [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857) + [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142)): **the delivery axis — third orthogonal envelope concern (visibility + revocability + delivery).** Observer-share (N=1) and streaming multicast (N>1) are the same primitive at different cardinality. Adds: **three new optional envelope fields** ([§4](04_envelope.md)) — `delivery_mode:{pull\|push}` + `listed:public` + `history_on_join:{full\|from_join}`; **new endpoint section** ([§10.5](10_endpoints.md)) — 7 sub-sections covering per-stream `SignedTreeHead` reusing [§10.3](10_endpoints.md) RFC 6962 per-`stream_id`, STREAM nonce derivation matching the `KEY_GRANT_V1_INFO` versioned-context HKDF pattern at `ciris-crypto/key_grant.rs`, epoch-keying cascade on a separate addressing axis from `key_grant.rotation_chain`, delivery receipts as JOIN-against-published-root, Edge transport with two-layer crypto + pull-only RC1, D6 entitled-∧-reachable liveness invariant; **one new reserved prefix** ([§7.9](07_reserved.md)) `delivery_receipt:{stream_id}`; **composition extension** ([§8.1.13.7](08_composition.md)) `delivery_mode` × Policy M + `history_on_join` × membership additions; **`rotation_chain` hygiene fixes** clarify CEG 0.3's `key_grant.rotation_chain` is content-addressed grant-supersession lineage (not key-rotation) and CEG 0.10 introduces a parallel per-`(stream_id, epoch)` axis reusing the same payload-level supersession mechanism. **Bifurcated**: observer-share half impl-live (no blockers); streaming multicast half spec-now/impl substrate-pending [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142) + accountable tier additionally pending [CIRISRegistry#34](https://github.com/CIRISAI/CIRISRegistry/issues/34). Open gate: RC1-1c V054 CHECK parallel-arm migration (bounded constraint migration, not pure-additive at Persist constraint layer). Tenth independent path confirming 1+4 minimal-and-adequate ([§1.4](01_foundation.md)) — substrate-fan-out and 1:N media multicast compose from the same 1+4 set as 1:1 attestations.
- **CEG 0.7** (additive at the envelope layer; per [CIRISRegistry#47](https://github.com/CIRISAI/CIRISRegistry/issues/47) + [CIRISPersist#152](https://github.com/CIRISAI/CIRISPersist/issues/152) + [ciris.ai/cewp](https://ciris.ai/cewp) structural-invisibility framing): **self/family membership primitives + wire-format-level structural invisibility.** Two new subject_kinds — `identity_occurrence` ([§5.6.8.8](#5688-identity_occurrence-subject_kind-ceg-07-addition); links occurrence_keys (devices + agents) to a root identity_key; single-vouch admission; `device_class ∈ phone | laptop | server | embedded | agent | service`) and `family` ([§5.6.8.9](#5689-family-subject_kind-ceg-07-addition); group of trusted nodes — members are identity_keys (which may themselves have multi-occurrence sets); one identity MAY belong to multiple families; per-family `consensus_protocol` field (`founder_only` / `unanimous` / `majority` / `quorum:M/N` / `weighted:{rubric}` / `custom:{id}`) governs admission; meta-amendment via the protocol's own rules unless `consensus_protocol_entrenched`). **One new envelope field** ([§4](04_envelope.md)): `family_id` required iff `cohort_scope == family`. **Four new substrate-emitted reserved prefixes** ([§7.7](07_reserved.md)): `hard_case:identity_occurrence_added:*` + `hard_case:family_membership_change:*` + `hard_case:family_consensus_protocol_change:*` + `hard_case:family_consensus_protocol_violation:*`. **New composition policy** ([§8.1.12](08_composition.md)) Policy L — self/family membership composition + DEK key-grant cascade on new-member admission + Option A forward-secrecy on departure. **New endpoint discipline** ([§10.1.4](10_endpoints.md)) structural-invisibility — substrate MUST NOT emit `holds_bytes:sha256:*` for `cohort_scope: self | family` content; the cewp claim "the wire format can't carry them in the first place" is now normative. **New governance section** ([§11.7](11_governance.md)) self/family membership governance — locked the 4 open decisions from #47 (Option A forward-secrecy + envelope `family_id` for multi-family + reserved-prefix substrate ownership + single-vouch self / consensus-protocol family). **Retcon at [§9.1](09_humanity_accord.md)**: HUMANITY_ACCORD triple is the canonical entrenched-`family` instance (3 founders, `consensus_protocol: quorum:2/3`, `consensus_protocol_entrenched: true`). **1+4 wire-format lockdown preserved** — zero new structural primitives; both new subject_kinds ride existing `scores` + subject_kind discriminator; membership changes ride existing `supersedes`; DEK cascade rides existing `key_grant` wrap + Option-A re-grant (`rotation_chain` from CEG 0.3 is the content-addressed grant-supersession lineage, a separate axis — see [§5.6.8.4](#key_grant) disambiguation). Eighth independent path confirming 1+4 minimal-and-adequate ([§1.4](01_foundation.md)) — demonstrates the structural set is rich enough to express collective-scale membership AND the wire-format-level closure of the cewp structural-invisibility privacy claim.
- **CEG 0.6** (additive at the envelope layer; per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842)): **subject-side consent authority — the missing half of consent at the wire format.** Universal across medical records / photos / interviews / training data / group chat / financial / surveillance / FERPA / multi-party contracts. CEG ≤ 0.5 encoded only producer authority (`attesting_key_id`); CEG 0.6 adds subject authority via **one new optional envelope field** ([§4.2](04_envelope.md)): `subject_key_ids: Vec<KeyId>` — accepts both federation_keys identities AND canonical-hash identifiers (resolves [CIRISAgent#840 OQ3](https://github.com/CIRISAI/CIRISAgent/issues/840)). **Semantic broadening of `withdraws`** ([§3.2.3](03_primitives.md)) to admit subject revocation + delegated proxy chain for canonical-hash subjects; the primitive's wire shape is unchanged. **One new dimension family** ([§5.6.8.6](#5686-consent-namespace-family-ceg-06-addition)): `consent:*` (8 prefixes — `state:*`, `stream:*`, `deletion_sla:*`, `deletion_complete`, `decay:*`, `partnership_grant`, `partnership_accept`, `scope:*`). **One new subject_kind** ([§5.6.8.7](#5687-consent_record-subject_kind-ceg-06-addition)): `consent_record` (ceremony envelope parallel to `key_grant` / `takedown_notice`; both bare-`scores` and ceremony shapes admitted at the same gate). **New composition policy** ([§8.1.11](08_composition.md)) Policy K — CEM composition. **New governance section** ([§11.6](11_governance.md)) vertical compliance mapping (HIPAA / GDPR Art 9 / FERPA / CCPA / AI training right-to-be-forgotten) + dimension-pattern-implies-`subject_key_ids` requirement. **1+4 wire-format lockdown preserved** — zero new structural primitives; one envelope field + one namespace family + one optional subject_kind + one semantic broadening. **CIRISAgent's CEM** (TEMPORARY / PARTNERED / ANONYMOUS streams) becomes a **consumer-policy bundle over the wire primitive**, not a wire-format lockdown; other agents MAY compose other streams over the same primitives.

Zero new structural primitives across the entire lineage. 1+4 minimal-and-adequate claim examined across **10 independent paths** ([§1.4](01_foundation.md)) — CEG 0.10 delivery axis (observer-share + streaming multicast as the same primitive at different cardinality) is the tenth and most operationally consequential. The wire format's structural set is stable at 1+4 across content (multimedia + time-bound), consent (dual-authority + decay-protocol + bilateral-pair), collective-scale membership (devices + agents + families + entrenched-families), geospatial admission constraints with wire-format-enforced privacy precision, AND substrate-fan-out / 1:N media multicast.

---

[← §4 Envelope](04_envelope.md) | **§5 Namespace** | [Next: §6 Relations →](06_relations.md)
