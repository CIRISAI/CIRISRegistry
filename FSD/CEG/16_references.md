[← §15 Gaps](15_gaps.md) | **§16 References** | [Next: §17 Cadence →](17_cadence.md)

---

# §16 References + lineage

## §16.1 CEG specification lineage

| Version | Date | Change |
|---|---|---|
| FSD-002 v1.0 | 2026-05-24 | Initial federation surface spec; 73 prefix families |
| FSD-002 v1.1 | 2026-05-26 | §1.10 anthropological commitment; F-3 detector under wrong (Cartesian) name `emergent_deception` |
| FSD-002 v1.2 | 2026-05-27 | Renamed `emergent_deception` → `correlated_action` per §1.10.1 operational-language gate; added §1.10.1 + §4.9.1 + §4.9.2 disciplines |
| FSD-002 v1.3 | 2026-05-27 | §4.9.2.5 1-of-6 sign-off (closed G2); §6.1.5 locality-scaled quorum (closed G3); §13.11 concerns + gaps; v1.3 dimension additions (multilateral_participation, locality:decision, distributive:access) + envelope (witness_relation) |
| FSD-002 v1.4 | 2026-05-27 | Files-as-Contributions (agent_files + holds_bytes); testimonial_witness; need:* (v1.5-loadbearing absorption); goal:planet enum; §7.7 endpoint shapes |
| FSD-002 v1.4.1 | 2026-05-28 | 3.0 / Compliance spec batch (fidelity:explainability_sla, oversight_mode, skill_import, per-locale build_manifest, etc.); §7.8 STH cosigning |
| FSD-002 v1.4.2 | 2026-05-28 | Occurrence_id/count/role envelope fields (Persist#110) |
| FSD-002 v1.4.3 | 2026-05-28 | §3.2.1 canonical-bytes contracts (SkillImportManifest + per-locale Merkle); §3.6.2 Goal substrate cross-ref |
| (CEG 1.0 attempted) | 2026-05-28 | Consolidated spec under CIRIS Epistemic Grammar name; renumbered to **0.1** after critical-review pass surfaced spec-discipline gaps. The 1.0 attempt is preserved in git history at commit prior to the rename. |
| **CEG 0.4** | **2026-05-30** | **Additive (no wire-break; 1+4 lockdown preserved). Per [CIRISRegistry#40](https://github.com/CIRISAI/CIRISRegistry/issues/40) + [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 1 closure at NodeCore commit [d0a443a](https://github.com/CIRISAI/CIRISNodeCore/commit/d0a443a).** Adds: one new `external_content` sub_kind (`event_listing` — Eventbrite / Meetup / Lu.ma / calendar / RSVPs / ticketing); one new dimension-family group at [§5.6.8.5](05_namespace.md) (`event:lifecycle:{state}` with canonical states `open` / `cancelled` / `completed` / `superseded` + `event:rsvp_count` + `event:attendance`); two new canonical `topical_relation:{kind}` entries (`rsvps` for RSVP attestations against an event, `vod_of` reserved for the deferred live_stream→video relationship). The event-lifecycle state machine composes from `withdraws` (cancellation) + `supersedes` (reschedule, with `differs_in: ["start_time", "venue"]`) + `delegates_to` (ticket transfer, parallel to `key_grant.rotation_chain`) + the new `event:lifecycle:{state}` dimension's latest non-superseded emission. **`live_stream` remains deferred** ([CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) Gap 2; substrate-side Edge + Persist decisions pending) — CEG 0.4 codifies only what NodeCore shipped, per the downstream-demand discipline established with 0.3. **Sixth independent path** confirming the 1+4 minimal-and-adequate claim ([§1.4](01_foundation.md)) against time-bound state-bearing content. |
| **CEG 0.3** | **2026-05-29** | **Additive (no wire-break; 1+4 lockdown preserved). Per CIRISRegistry#37 + #38 + #39 (multimedia tier + takedown_notice + key_grant + hash-DB governance).** Adds: two new Contribution subject_kinds (`takedown_notice` with `LegalBasis` 10-value closed-set enum + per-basis discipline; `key_grant` with `wrap_algorithm` + `scope` enums + `rotation_chain` semantics — retire-key-grant rides existing `supersedes`, NOT a new primitive); five new `external_content` sub_kinds (`image`, `audio`, `video`, `film`, `model_3d`; + Phase 2 `live_stream`); four new dimension families (`content_rating:{scheme}:{rating}`, `content_class:{class}`, `cw_class:{class}`, `age_assurance:{level}`); five new media-prefix families (`image:*`, `audio:*`, `video:*`, `film:*`, `model_3d:*`); new [§8.1.10](08_composition.md) Policy J trusted-publisher composition (three-layer: distributor attestation chain + content-class/rating gate + age-assurance gate); new [§11.4](11_governance.md) fast-path takedown coordination protocol (TVEC/NCMEC/GIFCT/PerceptualHashCsam/CourtOrder immediate-removal path + audit-trail discipline + the takedown-isn't-a-coup property); new [§11.5](11_governance.md) hash-database operator policy (default to self-hosted PDQ against open feeds per #39 recommendation; document slot for future CIRIS hash-coalition clearinghouse). Cross-refs to CIRISNodeCore FSD/MEDIA_SHARING.md throughout. |
| CEG 0.2 | 2026-05-28 | Wire-break. Renames §5.2 attestation-ladder prefixes from `attestation:l{N}:*` to mechanism-only form (`attestation:self_verify`, `attestation:hardware_rooted`, `attestation:registry_consensus`, `attestation:license_validity`, `attestation:agent_integrity`) per [§1.3.1](01_foundation.md) T2 honestly applied — L-numbers name ladder-position (verdict-shape), not mechanism. The L1-L5 ladder moves to consumer-side composition as [§8.1.9](08_composition.md) Policy I — Attestation-Ladder Composition. Deprecated wire shape entered in [§13.1](13_anti_patterns.md); 0.2 closed-gap row in [§15.1](15_gaps.md). Verify v3.7.0 had caught the principle correctly; CEG 0.1 inherited the wrong shape from FSD-002 v1.0 baseline without re-examining against the T2 gate. CIRISVerify v4.0 conforms to CEG 0.2 directly (no intermediate L-numbered re-add).** |
| CEG 0.1 | 2026-05-28 | Public Working Draft. Consolidated spec split into 18 files under `FSD/CEG/`. Adds v1.5 candidates from #30: `testimonial_witness:{kind}` open vocabulary; `hard_case:{kind}` surfaced; `biosphere` in [§2](02_grammar.md) Scope axis; `topical_relation:translation_of` sub-leaf in [§5.6.8](05_namespace.md) (LIVE per CIRISNodeCore b1582cb); [§8.1.7](08_composition.md) Trust-Fresh + [§8.1.8](08_composition.md) Tiered-Scope composition patterns. Records 6 explicit rejections from #30 stress test in [§13.2](13_anti_patterns.md). Glossaries in [§14](14_glossaries.md). Critical-review-pass scaffolding: [§0.1 RFC 2119 anchor](00_conformance.md), [§0.2 conformance levels](00_conformance.md), [§0.3 SemVer policy](00_conformance.md), [§0.4 normative references](00_conformance.md), [§0.5-§0.7 canonicalization](00_conformance.md), [§6.1 concurrent-write precedence](06_relations.md), [§7.0 reserved-prefix enforcement](07_reserved.md), [§8.1.5.1 sub-quorum fallback](08_composition.md), [§9.2.1 invocation discriminator + nonce](09_humanity_accord.md), [§9.2.2 consumer-UI requirement](09_humanity_accord.md), [§10.0 common response shape + error envelope](10_endpoints.md), [§10.1.1-§10.1.2 holds_bytes verification + TTL](10_endpoints.md), [§10.3.1 STH consistency-proof](10_endpoints.md), [§11.2.2 collision rule](11_governance.md), [§11.2.3 meta-amendment entrenchment](11_governance.md), [§13.3 delegation-laundering anti-pattern](13_anti_patterns.md), [§13.4 withdraws-arbitrage countermeasure](13_anti_patterns.md). 83 prefix families.** |

## §16.2 Companion documents

- [`FSD/LANGUAGE_PRIMER.md`](../LANGUAGE_PRIMER.md) — translation grammar (how to write Contributions in CEG)
- [`FSD/PRIOR_ART_SCAN.md`](../PRIOR_ART_SCAN.md) — design-space comparison (PGP / SPKI-SDSI / W3C VC / Birdwatch / Pol.is / Kleros / Spritely / Holochain / Aragon / Conviction Voting / Sigstore / SLSA)
- [`FSD/SOTA_SCAN.md`](../SOTA_SCAN.md) — production-validation comparison
- [`FSD/WITNESS_KIND_REGISTRY.md`](../WITNESS_KIND_REGISTRY.md) — non-normative open-vocabulary registry for `testimonial_witness:{kind}`
- [`docs/CEG_EXPLORATION_PAGE_PRIMER.md`](../../docs/CEG_EXPLORATION_PAGE_PRIMER.md) — builder primer for the public-facing exploration page

## §16.3 Sibling MISSIONs (the namespace owners)

- [`CIRISAgent/MISSION.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/MISSION.md)
- [`CIRISVerify/MISSION.md`](https://github.com/CIRISAI/CIRISVerify/blob/main/MISSION.md)
- [`CIRISPersist/MISSION.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/MISSION.md)
- [`CIRISEdge/MISSION.md`](https://github.com/CIRISAI/CIRISEdge/blob/main/MISSION.md)
- [`CIRISLensCore/MISSION.md`](https://github.com/CIRISAI/CIRISLensCore/blob/main/MISSION.md)
- [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md)
- [`RATCHET/FSD.md`](https://github.com/CIRISAI/RATCHET/blob/main/FSD.md)
- [`CIRISBench/README.md`](https://github.com/CIRISAI/CIRISBench)
- [`MISSION.md`](../../MISSION.md) — CIRISRegistry's own

## §16.4 External references (informational)

- [`ciris.ai/safety-vs-censorship`](https://ciris.ai/safety-vs-censorship/) — the operational-language gate source
- *Magnifica Humanitas* encyclical (2026-05-15) — first deployment of the bootstrap-content pattern ([§11.3](11_governance.md))
- [`ciris-response-magnifica-humanitas`](https://github.com/CIRISAI/ciris-response-magnifica-humanitas) — encyclical mapping repo

### CEG 0.4 additions

- [CIRISNodeCore SCHEMA §4.29 + commit d0a443a](https://github.com/CIRISAI/CIRISNodeCore/commit/d0a443a) — event_listing ingest path; `EventListingSource` + `EventVenue` (Physical / Virtual / Hybrid) + `TicketGrantPolicy` (Open / ApprovalRequired / InvitationOnly / Paid) types; `build_event_listing_payload` builder; `ingest_event_listing` async fn; 5 unit tests
- [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25) — primary design issue with full coverage matrix (17 content types audited; 15 structurally complete; 2 gaps named: event_listing closed, live_stream still deferred)

### CEG 0.3 additions

- [CIRISNodeCore FSD/MEDIA_SHARING.md](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/MEDIA_SHARING.md) — multimedia tier FSD; per-sub_kind Source struct shapes; takedown_notice + key_grant payload designs; PerceptualHashMatcher trait
- [CIRISNodeCore FSD/CEWP.md](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/CEWP.md) — CIRIS Epistemic Web Platform identity FSD (the platform-identity tie-in)
- [CIRISNodeCore FSD/FEDERATION_SCALING_MODEL.md](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_SCALING_MODEL.md) — scaling envelope (tiktok / youtube / netflix / adulthub scenarios)
- RFC 9180 — HPKE base mode shape (referenced by `key_grant.wrap_algorithm = X25519AesGcmHkdfSha256`)
- 17 USC §512 (DMCA), EU DSA Article 16, EU Regulation 2021/784 (TVEC), 18 USC §2258A (NCMEC), GIFCT Content Incident Protocol, UK Online Safety Act, EU AVMSD — `LegalBasis` enum source regimes per [§5.6.8.4](05_namespace.md)
- PhotoDNA / PDQ / Project Arachnid / GIFCT hash-sharing — hash-database landscape per [§11.5](11_governance.md)

For normative references see [§0.4](00_conformance.md).

---

[← §15 Gaps](15_gaps.md) | **§16 References** | [Next: §17 Cadence →](17_cadence.md)
