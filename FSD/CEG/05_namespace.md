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

> **0.1 SCAFFOLD NOTE**: The contracts below use newline-delimited `key=value` encoding for readability. CEG 0.2+ will replace this with TupleHash128 ([FIPS-202]) using explicit domain-separation labels, fixing newline-injection and field-confusion attack surfaces identified in 0.1 cryptographic review. The 0.2 canonical-bytes redesign lands under Phase B per CIRISRegistry#30 review. Producers building against 0.1 SHOULD treat these contracts as unstable.

#### `SkillImportManifest` canonical bytes (0.1 interim)

```
canonical_bytes = sha256(
    "ciris.skill_import.v1\n" ||
    "source=" || source_string || "\n" ||
    "skill_manifest_sha256=" || sha256_hex_lowercase || "\n" ||   // per §0.6
    "signer_identity=" || signer_key_id || "\n" ||
    "import_timestamp=" || rfc3339_canonical || "\n" ||           // per §0.5
    "capability_declaration=" || rfc8785_jcs_json || "\n" ||      // RFC 8785 JCS
    "valid_until=" || optional_rfc3339_canonical_or_empty         // per §0.5
)
```

Hybrid signature: Ed25519 over `canonical_bytes`; ML-DSA-65 over `canonical_bytes || ed25519_signature_bytes` (bound payload).

#### Per-locale Merkle composition (0.1 interim)

```
leaf_hash[lang_code] = sha256(
    0x00 ||                                  // RFC 6962 leaf-domain prefix
    "ciris.locale_manifest.v1\n" ||
    "target=" || target_string || "\n" ||
    "locale=" || lang_code || "\n" ||
    "files_root=" || files_merkle_root_hex_lowercase || "\n" ||   // per §0.6
    "build_id=" || build_id || "\n" ||
    "signer_identity=" || signer_key_id
)

parent_hash(left, right) = sha256(
    0x01 ||                                  // RFC 6962 parent-domain prefix
    left || right
)
```

Locale ordering: lexicographic by ISO 639-1 / BCP 47 byte representation; `"polyglot"` sorts last. RFC 6962 padding: duplicate last leaf to next power of 2.

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

### §5.6.5 Decision-locality prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `locality:decision:{scale}` | Names the scale at which a decision is being made. `{scale}` ∈ `local` \| `regional` \| `national` \| `federation`. Composes with [§8.1.5](08_composition.md) locality-scaled quorum (closes G3 — fresh-quorum-recusal in narrow cells). | enumerated |

### §5.6.6 Hard-case + transparency + judge-model prefixes

| Prefix | Description | Polarity |
|---|---|---|
| `hard_case:{kind}` | **Open vocabulary**. Surfaces flag conditions for federation-health observability + downstream review. Canonical kinds: `vote_variance` (vote variance exceeded threshold at truth-grounding resolution), `resolution_time` (truth-grounding took > P75 of cell's distribution), `moderation_filed` (substantive ModerationEvent filed), `novel_context` (no precedent in attestation graph), `sla_breach_unattested` (per `fidelity:explainability_sla:{tier}` composition), `unresolved_consent` (consent boundary unclear). New `{kind}` values land via the [§11.2](11_governance.md) amendment process. | positive-only |
| `seed_holder_voting_alignment:{cell}` | Pairwise cosine of seed-holder vote vectors per voting window. Transparency signal only — not a slashing trigger. | signed |
| `judge_model:verdict:{model_id}` | Independent foundation-model judge verdict (PASS/FAIL/UNDETERMINED). Default model: Claude Opus 4.7. | boolean-via-score |

### §5.6.7 Files-as-Contributions joint claim

| Prefix | Description | Polarity |
|---|---|---|
| `agent_files:{kind}:{platform_or_target}` | **Joint claim with [§5.9](#59-cirisregistry--identity--build--license--partner) CIRISRegistry.** Files a CIRIS agent (or installer fetching one) may load. `{kind}` open vocabulary; canonical: `installer:{platform}`, `adapter:{name}`, `config:{kind}`, `build:{target}`, `source:{language}:{module}`, `state:{component}`. Bytes are SHA-256-addressed and resolved via [§10.1](10_endpoints.md) transport substrate (Edge `MessageType::ContentFetch`). NodeCore-side rule: node-mode peers serve bytes; client/relay modes don't. | signed |
| `holds_bytes:sha256:{prefix}` | Substrate auto-emission per CIRISPersist#103 `federation_blobs.put_blob`. `{prefix}` is a short SHA prefix for index efficiency; full SHA lives in `evidence_refs[]`. Consumed by Edge's `PeerResolver::resolve_holders` to route `ContentFetch` requests. **Consumer MUST verify the full SHA in `evidence_refs[]` matches the received blob before consumption** (see [§10.1](10_endpoints.md)). | boolean-via-score |

### §5.6.8 Content-ingestion prefixes

Per CIRISNodeCore commit b1582cb (three-tier interface model). NodeCore ships an open set of `external_content` sub_kinds with three feed surfaces (local / community / global) composed against `cohort_scope`. See [§8.1.8](08_composition.md) Tiered-Scope Composition pattern.

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

#### §5.6.8.3 Multimedia dimension families (CEG 0.3 addition)

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

#### §5.6.8.4 Governance subject_kinds (CEG 0.3 addition per CIRISRegistry#37 + #38)

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
    wrapped_dek:              base64url               // the DEK encrypted under recipient's pubkey
    key_validity_window: {
        start:                rfc3339_canonical       // per §0.5
        end:                  Option<rfc3339_canonical>
    }
    ratchet_version:          u32                     // monotonic ratchet for rotation
    rotation_chain:           [key_grant_id, ...]     // prior key_grant ids in the rotation lineage
    asserted_at:              rfc3339_canonical
}
```

Where:

| `wrap_algorithm` | Algorithm |
|---|---|
| `X25519AesGcmHkdfSha256` | HPKE RFC 9180 base-mode shape; X25519 KEM + HKDF-SHA-256 KDF + AES-256-GCM AEAD. **v1 default.** |

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

#### §5.6.8.7 `consent_record` subject_kind (CEG 0.6 addition)

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

Rides existing `scores` attestation_type with `subject_kind=consent_record` discriminator. No new attestation_type. 1+4 preserved.

**Bilateral pair pattern** (per [CIRISAgent CEM](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/CIRIS_CONSENT_SERVICE.md) PARTNERED stream):

```
1. Subject emits consent_record(subject_key_id, stance: granted,
                                 bilateral_pair_id: <fresh-uuid>) +
                  scores on `consent:partnership_grant`
2. Producer emits consent_record(subject_key_id, target_key_id: subject_key_id,
                                 stance: granted, bilateral_pair_id: <same-uuid>) +
                  scores on `consent:partnership_accept`
3. topical_relation:bilateral_pair links the two Contributions
4. Consumer policy treats the partnership as ratified iff both halves present
   under the same bilateral_pair_id with stance: granted
```

The structural primitives close the bilateral shape — no new attestation_type, no new envelope field beyond `subject_key_ids` itself.

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
- **CEG 0.6** (additive at the envelope layer; per [CIRISRegistry#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842)): **subject-side consent authority — the missing half of consent at the wire format.** Universal across medical records / photos / interviews / training data / group chat / financial / surveillance / FERPA / multi-party contracts. CEG ≤ 0.5 encoded only producer authority (`attesting_key_id`); CEG 0.6 adds subject authority via **one new optional envelope field** ([§4.2](04_envelope.md)): `subject_key_ids: Vec<KeyId>` — accepts both federation_keys identities AND canonical-hash identifiers (resolves [CIRISAgent#840 OQ3](https://github.com/CIRISAI/CIRISAgent/issues/840)). **Semantic broadening of `withdraws`** ([§3.2.3](03_primitives.md)) to admit subject revocation + delegated proxy chain for canonical-hash subjects; the primitive's wire shape is unchanged. **One new dimension family** ([§5.6.8.6](#5686-consent-namespace-family-ceg-06-addition)): `consent:*` (8 prefixes — `state:*`, `stream:*`, `deletion_sla:*`, `deletion_complete`, `decay:*`, `partnership_grant`, `partnership_accept`, `scope:*`). **One new subject_kind** ([§5.6.8.7](#5687-consent_record-subject_kind-ceg-06-addition)): `consent_record` (ceremony envelope parallel to `key_grant` / `takedown_notice`; both bare-`scores` and ceremony shapes admitted at the same gate). **New composition policy** ([§8.1.11](08_composition.md)) Policy K — CEM composition. **New governance section** ([§11.6](11_governance.md)) vertical compliance mapping (HIPAA / GDPR Art 9 / FERPA / CCPA / AI training right-to-be-forgotten) + dimension-pattern-implies-`subject_key_ids` requirement. **1+4 wire-format lockdown preserved** — zero new structural primitives; one envelope field + one namespace family + one optional subject_kind + one semantic broadening. **CIRISAgent's CEM** (TEMPORARY / PARTNERED / ANONYMOUS streams) becomes a **consumer-policy bundle over the wire primitive**, not a wire-format lockdown; other agents MAY compose other streams over the same primitives.

Zero new structural primitives across the entire lineage. 1+4 minimal-and-adequate claim examined across **7 independent paths** ([§1.4](01_foundation.md)) — CEG 0.6 dual-authority composition is the seventh, demonstrating the lockdown holds when subject authority is added orthogonally to producer authority. The wire format's structural set is stable at 1+4 across content (multimedia + time-bound) AND across consent (dual-authority + decay-protocol + bilateral-pair).

---

[← §4 Envelope](04_envelope.md) | **§5 Namespace** | [Next: §6 Relations →](06_relations.md)
