[← §15 Gaps](15_gaps.md) | **§16 References** | [Next: §17 Cadence →](17_cadence.md)

---

# §16 References + lineage

## §16.1 CEG specification lineage

| Version | Date | Change |
|---|---|---|
| **CEG 1.0-RC12** | 2026-06-16 | Unified retirement / forever-memory model — revocation/retirement/eviction/expiry/aging as one pressure-driven monotonic descent toward the noise floor ([§19.7](19_holonomic.md)); [#85](https://github.com/CIRISAI/CIRISRegistry/issues/85). No 1+4 change. |
| **CEG 1.0-RC11** | 2026-06-16 | Holonomic substrate absorbed with normative guardrails (new [§19](19_holonomic.md): WholenessWitness / recursive bootstrap / fountain-swarm / ALM), subordinated to [§10.1.6](10_endpoints.md) quorum-merge; [#85](https://github.com/CIRISAI/CIRISRegistry/issues/85). No 1+4 change. |
| **CEG 1.0-RC10** | 2026-06-15 | Realtime A/V chunk wire absorbed byte-exact from Edge v4.0.0 — `SealedAvChunk` / `ChunkLayer` / double-seal nonce derivation ([§10.5.8.3–.5](10_endpoints.md)); [#85](https://github.com/CIRISAI/CIRISRegistry/issues/85). No 1+4 change. |
| **CEG 1.0-RC9** | 2026-06-14 | `codec_id` + `ChunkLayer` transport-chunk namespace ratified ([§10.5.8.2](10_endpoints.md)); [#84](https://github.com/CIRISAI/CIRISRegistry/issues/84). No 1+4 change. |
| **CEG 1.0-RC8** | 2026-06-13 | Store-path PQC clarification ([§10.1.5.1.1](10_endpoints.md)) + informative interoperability-profiles section (new [§18](18_interop.md), C2PA import/emit); [#72](https://github.com/CIRISAI/CIRISRegistry/issues/72) / [CIRISPersist#225](https://github.com/CIRISAI/CIRISPersist/issues/225). No wire change. |
| **CEG 1.0-RC7** | 2026-06-13 | PQC mandatory at admission ([§10.1.5.1.1](10_endpoints.md)) + bilateral partnership member-set ([§8.1.12.7.1](08_composition.md)) + trust≠membership / owner-binding / infra-no-agency refinements; [#81](https://github.com/CIRISAI/CIRISRegistry/issues/81)/[#82](https://github.com/CIRISAI/CIRISRegistry/issues/82)/[#83](https://github.com/CIRISAI/CIRISRegistry/issues/83). No wire change. |
| **CEG 1.0-RC6** | 2026-06-12 | RNS destination-hash two-stage algorithm pinned in-spec ([§5.6.8.8.1.1](05_namespace.md)); [#80](https://github.com/CIRISAI/CIRISRegistry/issues/80) / [CIRISVerify#28](https://github.com/CIRISAI/CIRISVerify/issues/28). Correctness pin on existing field; no wire change. |
| **CEG 1.0-RC5** | 2026-06-11 | All open CEG asks resolved — `consent_record` admission rules ([§5.6.8.7](05_namespace.md)), `identity:canonical_binding` ([§5.6.8.14](05_namespace.md)), removal-path emission shape ([§7.7](07_reserved.md)), core-terms glossary ([§14.0](14_glossaries.md)); [#77](https://github.com/CIRISAI/CIRISRegistry/issues/77)/[#78](https://github.com/CIRISAI/CIRISRegistry/issues/78)/[#79](https://github.com/CIRISAI/CIRISRegistry/issues/79). No wire change. |
| **CEG 1.0-RC4** | 2026-06-10 | §RC `consent_role` Counter-RII gate ratified (new [§7.0.2](07_reserved.md), OQ-1/2/3 at `ConsentGate.lean` defaults); [CIRISAgent#760](https://github.com/CIRISAI/CIRISAgent/issues/760). `consent_role` is a `federation_keys` field, not an envelope primitive; no wire change. |
| **CEG 1.0-RC3** | 2026-06-09 | Clarity cut — fabric-node separation-of-powers ([§7.0.1](07_reserved.md)), `ciris-canonical` re-rootable default ([§5.6.8.10](05_namespace.md)), NodeCode `key_id` shorthand ([§0.10](00_conformance.md)); [#75](https://github.com/CIRISAI/CIRISRegistry/issues/75). No wire change. |
| **CEG 1.0-RC2** | 2026-06-08 | Operational-data envelopes (scheduled additive cut) — `organization` / `org_membership` / `partner_record` subject_kinds ([§5.6.8.13](05_namespace.md)) + per-subject_kind merge intents ([§10.1.6](10_endpoints.md)); [#70](https://github.com/CIRISAI/CIRISRegistry/issues/70) under [#58](https://github.com/CIRISAI/CIRISRegistry/issues/58). Sixteenth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 1.0-RC1** | 2026-06-07 | Freeze cut — five honesty/safety patches + base64/ML-KEM pins + JCS provenance contracts ([§5.2.1](05_namespace.md), TupleHash128 retired); [#71](https://github.com/CIRISAI/CIRISRegistry/issues/71) / [CIRISVerify#64](https://github.com/CIRISAI/CIRISVerify/issues/64) / [#57](https://github.com/CIRISAI/CIRISRegistry/issues/57) blocker A. 1+4 surface frozen. |
| **CEG 0.18** | 2026-06-06 | Recipient encryption-key registration — optional `encryption_pubkeys` field-set on `identity_occurrence` ([§5.6.8.8.2](05_namespace.md)) feeding `wrap_algorithm: v2`; [#69](https://github.com/CIRISAI/CIRISRegistry/issues/69) / [CIRISPersist#192](https://github.com/CIRISAI/CIRISPersist/issues/192). Fifteenth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.17** | 2026-06-05 | Three crypto tiers (self/family · Community per-community DEK · Commons plaintext) + holder-inspectability principle ([§8.1.13.3](08_composition.md)); [#67](https://github.com/CIRISAI/CIRISRegistry/issues/67). No 1+4 change (community DEK = existing epoch cascade). |
| **CEG 0.16** | 2026-06-04 | Agent-identity hardening + cross-impl byte-determinism — attestation tier model ([§10.1.5](10_endpoints.md)), self-at-login ([§8.1.12.7](08_composition.md)), canonical-bytes determinism ([§0.9.2.1](00_conformance.md)), PQC-at-rest for self/family; [#63](https://github.com/CIRISAI/CIRISRegistry/issues/63)/[#65](https://github.com/CIRISAI/CIRISRegistry/issues/65). No wire change. |
| **CEG 0.15** | 2026-06-03 | Streaming standards folded in — [§10.5](10_endpoints.md) conforms to SFrame + MLS TreeKEM (RFC 9420); [§0.4](00_conformance.md) normative refs added; [§1.6](01_foundation.md) adversary model; [#61](https://github.com/CIRISAI/CIRISRegistry/issues/61). No wire change. |
| **CEG 0.14** | 2026-06-03 | CEG↔value-transfer linkage — `settlement` subject_kind ([§5.6.8.12](05_namespace.md), x402 receipt via `evidence_refs[]`) + §0.8 H3 prose fix; [#59](https://github.com/CIRISAI/CIRISRegistry/issues/59) / [#55](https://github.com/CIRISAI/CIRISRegistry/issues/55). Fourteenth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.13** | 2026-06-02 | Realtime group communication as composition — call/voice/screen-share/chat/channel transport profiles ([§10.5.8](10_endpoints.md)); [#56](https://github.com/CIRISAI/CIRISRegistry/issues/56). Thirteenth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.12** | 2026-06-02 | DNS-free addressing layer — optional `transport_destination` on `identity_occurrence` ([§5.6.8.8.1](05_namespace.md)) + `resolve_member_transport` ([§8.1.13.1.1](08_composition.md)), closes AV-42; [#56](https://github.com/CIRISAI/CIRISRegistry/issues/56) / [CIRISEdge#15](https://github.com/CIRISAI/CIRISEdge/issues/15). Twelfth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.11** | 2026-06-01 | `cohort_subkind: infrastructure` trust-root community subkind — founder-subset admission basis ([§5.6.8.10](05_namespace.md)); [#56](https://github.com/CIRISAI/CIRISRegistry/issues/56). Eleventh [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.10** | 2026-06-03 | Delivery axis — third orthogonal envelope concern (visibility + revocability + delivery): three optional envelope fields ([§4](04_envelope.md) `delivery_mode`/`listed`/`history_on_join`) + streaming endpoint section ([§10.5](10_endpoints.md)) + `delivery_receipt:{stream_id}` ([§7.9](07_reserved.md)); [#44 absorbed](https://github.com/CIRISAI/CIRISRegistry/issues/44) + [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857) + [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142). Tenth [§1.4](01_foundation.md) path; open caveats RC1-1c / RC1-7. |
| **CEG 0.9** | 2026-06-01 | Wire-break (representation-only, semantically null for legacy single-role keys) — `federation_keys.identity_type` generalized to a set of roles, set-membership emitter gating ([§7.0.1](07_reserved.md) + [§11.9](11_governance.md)); [#49](https://github.com/CIRISAI/CIRISRegistry/issues/49) + [CIRISAgent#856](https://github.com/CIRISAI/CIRISAgent/issues/856). NOT a [§1.4](01_foundation.md) path (§7-layer enforcement generalization). |
| **CEG 0.8** | 2026-05-31 | `community` + `location_proof` subject_kinds + `cohort_subkind: geographic` + H3 rough-only canonicalization (`cell_resolution ≤ 7`, [§0.8](00_conformance.md)); [#48](https://github.com/CIRISAI/CIRISRegistry/issues/48). Ninth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.7** | 2026-05-31 | Self/family membership — `identity_occurrence` + `family` subject_kinds + `family_id` envelope field + wire-format structural invisibility for self/family content ([§10.1.4](10_endpoints.md)); [#47](https://github.com/CIRISAI/CIRISRegistry/issues/47) + [CIRISPersist#152](https://github.com/CIRISAI/CIRISPersist/issues/152). Eighth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.6** | 2026-05-31 | Subject-side consent authority — optional `subject_key_ids` envelope field ([§4.2](04_envelope.md)) + `consent:*` dimension family + `consent_record` subject_kind + `withdraws` semantic broadening; [#45](https://github.com/CIRISAI/CIRISRegistry/issues/45) + [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842). Seventh [§1.4](01_foundation.md) path; no 1+4 change. |
| (CEG 0.5) | *in flight* | `live_stream` promotion + chunk-DAG (later absorbed by CEG 0.10); [#44](https://github.com/CIRISAI/CIRISRegistry/issues/44) + [CIRISNodeCore#26](https://github.com/CIRISAI/CIRISNodeCore/issues/26). |
| **CEG 0.4** | 2026-05-30 | `event_listing` external_content sub_kind + `event:lifecycle:{state}` dimension family ([§5.6.8.5](05_namespace.md)); state machine composes from `withdraws`/`supersedes`/`delegates_to`; [#40](https://github.com/CIRISAI/CIRISRegistry/issues/40) + [CIRISNodeCore#25](https://github.com/CIRISAI/CIRISNodeCore/issues/25). Sixth [§1.4](01_foundation.md) path; no 1+4 change. |
| **CEG 0.3** | 2026-05-29 | Multimedia tier — `takedown_notice` + `key_grant` subject_kinds, five `external_content` media sub_kinds, four media dimension families, Policy J ([§8.1.10](08_composition.md)), takedown + hash-DB governance ([§11.4](11_governance.md)/[§11.5](11_governance.md)); CIRISRegistry#37 + #38 + #39. No 1+4 change. |
| CEG 0.2 | 2026-05-28 | Wire-break — attestation-ladder prefixes renamed `attestation:l{N}:*` → mechanism-only ([§5.2](05_namespace.md)) per [§1.3.1](01_foundation.md) T2; ladder moves to [§8.1.9](08_composition.md) Policy I; deprecation in [§13.1](13_anti_patterns.md). |
| FSD-002 v1.0 | 2026-05-24 | Initial federation surface spec; 73 prefix families |
| FSD-002 v1.1 | 2026-05-26 | §1.10 anthropological commitment; F-3 detector under wrong (Cartesian) name `emergent_deception` |
| FSD-002 v1.2 | 2026-05-27 | Renamed `emergent_deception` → `correlated_action` per §1.10.1 operational-language gate; added §1.10.1 + §4.9.1 + §4.9.2 disciplines |
| FSD-002 v1.3 | 2026-05-27 | §4.9.2.5 1-of-6 sign-off (closed G2); §6.1.5 locality-scaled quorum (closed G3); §13.11 concerns + gaps; v1.3 dimension additions (multilateral_participation, locality:decision, distributive:access) + envelope (witness_relation) |
| FSD-002 v1.4 | 2026-05-27 | Files-as-Contributions (agent_files + holds_bytes); testimonial_witness; need:* (v1.5-loadbearing absorption); goal:planet enum; §7.7 endpoint shapes |
| FSD-002 v1.4.1 | 2026-05-28 | 3.0 / Compliance spec batch (fidelity:explainability_sla, oversight_mode, skill_import, per-locale build_manifest, etc.); §7.8 STH cosigning |
| FSD-002 v1.4.2 | 2026-05-28 | Occurrence_id/count/role envelope fields (Persist#110) |
| FSD-002 v1.4.3 | 2026-05-28 | §3.2.1 canonical-bytes contracts (SkillImportManifest + per-locale Merkle); §3.6.2 Goal substrate cross-ref |
| (CEG 1.0 attempted) | 2026-05-28 | Consolidated spec under CIRIS Epistemic Grammar name; renumbered to **0.1** after critical-review pass surfaced spec-discipline gaps. The 1.0 attempt is preserved in git history at commit prior to the rename. |
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

### CEG 0.10 additions

- [CIRISLensCore#857](https://github.com/CIRISAI/CIRISLensCore/issues/857) — observer-share driver (prod-lens-via-transit-key); the N=1 cardinality demonstrator that motivated the delivery-axis primitive
- [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142) — streaming substrate prerequisite; sequence `get_range` (v3.7) → `BlobBody::ChunkDag` + manifest (v3.8) → `put_blob_chunk` / `seal_stream` + `federation_stream_chunks` (v3.9, births `stream_id`); unowned/unscheduled at 0.10 cut
- [CIRISRegistry#34](https://github.com/CIRISAI/CIRISRegistry/issues/34) — STH consistency-proof enforcement at cosign admission; load-bearing for the accountable-stream tier per [§10.5.1](10_endpoints.md)
- [CIRISRegistry#44 absorbed](https://github.com/CIRISAI/CIRISRegistry/issues/44) — pre-0.10 codification of `live_stream` sub_kind + `topical_relation:has_chunk`; CEG 0.10 absorbs and supersedes
- [RFC 8785 — JSON Canonicalization Scheme (JCS)](https://www.rfc-editor.org/rfc/rfc8785) — envelope canonicalization per [§0.9](00_conformance.md); composes under §10.5's per-stream `SignedTreeHead.signing_bytes` discipline
- [RFC 6962 — Certificate Transparency](https://www.rfc-editor.org/rfc/rfc6962) — the per-stream transparency-log shape that §10.5.1 instantiates per-`stream_id`; the §10.3 cosign + consistency-proof abstractions reused verbatim
- [`KEY_GRANT_V1_INFO`](https://github.com/CIRISAI/CIRISVerify/blob/main/src/ciris-crypto/src/key_grant.rs) — CEG 0.3 HKDF versioned-context pattern (`b"cewp-key-grant/v1"`); §10.5.2 V2 STREAM nonce-prefix derivation reuses the pattern at the streaming-nonce boundary
- Hoang, Reyhanitabar, Rogaway, Vizár — *Online Authenticated-Encryption and its Nonce-Reuse Misuse-Resistance*, CRYPTO 2015 — STREAM nonce layout citation for §10.5.2 V2
- [NIST FIPS 197](https://csrc.nist.gov/pubs/fips/197/final) + [SP 800-38D](https://csrc.nist.gov/pubs/sp/800/38/d/final) — AES + AES-GCM for chunk sealing per §10.5.2
- [`CIRISEdge/src/reachability.rs`](https://github.com/CIRISAI/CIRISEdge) — node-local reachability tracker (CIRISEdge#29); the Edge side of the §10.5.6 D6 entitled-∧-reachable invariant; never-an-attestation discipline

### CEG 0.8 additions

- [H3 hierarchical hexagonal indexing](https://h3geo.org/) — Uber-originated, MIT-licensed; the canonical cell-identifier encoding for `cell_id` per [§0.8](00_conformance.md). Resolution table: 0 = ~4.3 M km²/cell; 5 = ~250 km²; 7 = ~5 km² (the rough-only bound per §0.8.1); 10 = ~15,000 m²; 15 = ~0.9 m²
- [CIRISPersist#153](https://github.com/CIRISAI/CIRISPersist/issues/153) — substrate implementation tracker (CEG 0.7 sister); CEG 0.8 community + location_proof admission gates extend the same substrate spec
- [`ciris-keyring`](https://github.com/CIRISAI/CIRISVerify) optional `attestation_evidence` payload for `location_proof.attestation_evidence` — hardware-attested location claims from TPM / Secure Enclave / StrongBox for higher-assurance communities
- Civic-engagement composition patterns ([§5.6.8.10](05_namespace.md) worked example): municipal communities, voting districts, town halls, ballot initiatives, public comments, petitions, FOIA, citizen journalism, whistleblower disclosure, mutual-aid networks
- Emergency-messaging composition patterns ([§5.6.8.10](05_namespace.md) worked example): severe weather warnings, AMBER alerts, shelter-in-place, evacuation orders, disease outbreak alerts, mass casualty incident coordination, infrastructure failure notices, disaster recovery activation
- Comparison with alternative geospatial systems considered + rejected: [Google S2](https://s2geometry.io/) (similar hierarchical cell index; rejected because non-hexagonal cells have non-uniform neighbor-distance properties that complicate containment math for arbitrarily-shaped geographic_constraints); [Geohash](https://en.wikipedia.org/wiki/Geohash) (lexicographic prefix-based; rejected because the prefix encoding leaks precision in the string itself, fighting the rough-only enforcement goal); [Plus Codes / Open Location Code](https://maps.google.com/pluscodes/) (similar to Geohash; same prefix-leakage issue)

### CEG 0.7 additions

- [ciris.ai/cewp](https://ciris.ai/cewp) — CIRIS Epistemic Web Platform identity FSD; the **structural-invisibility privacy claim** ("Self and family content never emits the attestation that would tell the rest of the network it exists... the wire format can't carry them in the first place") is the load-bearing source for [§10.1.4](10_endpoints.md)
- [CIRISPersist#152](https://github.com/CIRISAI/CIRISPersist/issues/152) — at-rest encryption substrate spec; the wire-format counterpart of CEG 0.7's membership primitives
- [CIRISAgent docs/MULTI_OCCURRENCE_CONSENT_ANALYSIS.md](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/MULTI_OCCURRENCE_CONSENT_ANALYSIS.md) — multi-occurrence design (`identity_occurrence` is the wire-format primitive making this composable across CEG-native agents)
- Signal Protocol — "trust any device I've already onboarded" single-vouch admission pattern that CEG 0.7 [§11.7.4](11_governance.md) self-occurrence admission adopts
- HPKE RFC 9180 — the wrap algorithm that powers the at-rest DEK cascade per [§8.1.12.4](08_composition.md) (reused from CEG 0.3 `key_grant` lock)

### CEG 0.6 additions

- [CIRISAgent ConsentService](https://github.com/CIRISAI/CIRISAgent/blob/main/ciris_engine/schemas/consent/core.py) — `ConsentStream` (TEMPORARY / PARTNERED / ANONYMOUS) + `ConsentStatus` + `ConsentAuditEntry` + `DSARExportPackage` + `PartnershipRequestHandler` — the in-graph CEM that becomes a consumer-policy bundle over CEG 0.6 primitives
- [CIRISAgent docs/CIRIS_CONSENT_SERVICE.md](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/CIRIS_CONSENT_SERVICE.md) — three-stream model + 90-day decay protocol + bilateral partnership protocol + DSAR integration
- [CIRISAgent docs/MULTI_OCCURRENCE_CONSENT_ANALYSIS.md](https://github.com/CIRISAI/CIRISAgent/blob/main/docs/MULTI_OCCURRENCE_CONSENT_ANALYSIS.md) — per-occurrence lifecycle consent shape (orthogonal to subject-side consent per [§4.2.4](04_envelope.md))
- [CIRISAgent#840](https://github.com/CIRISAI/CIRISAgent/issues/840) — "Design: CEG-native agent — graph_nodes ARE self-level CEG attestations" (the downstream consumer whose envelope shape CEG 0.6 unblocks)
- [CIRISAgent#842](https://github.com/CIRISAI/CIRISAgent/issues/842) — upstream-blocker for #840 documenting 7 consent-shape gaps CEG 0.6 closes
- [CIRISRegistry MISSION.md §1.5](../../MISSION.md) — the philosophical anchor: "consent (M-1's load-bearing property) requires revocability, and revocability requires a halt-authority that lives outside the system being halted"
- GDPR Articles 7 (consent), 9 (special category), 17 (right to erasure), 20 (data portability)
- HIPAA 45 CFR 164.502 (uses + disclosures), 164.524 (patient right of access)
- FERPA 34 CFR Part 99 (educational records)
- CCPA §1798.105 (right to delete)
- EU AI Act Article 50 (training data transparency + opt-out)
- CIRIS Accord §M-1 — meta-goal anchor for consent revocability

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
