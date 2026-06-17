# Part 8 — Appendices

**Decimal range** `8.x` · **41 sections** · **page budget 6pp** · [← master index](README.md)

> Case studies, glossaries, conformance vectors, interop, and the dual-ID table of contents.

---

## 8.1 `glossary` — Glossaries

# Glossaries

### 8.1.1 `registry-core` — Core terms (added 1.0-RC5; resolves [CIRISRegistry#77](https://github.com/CIRISAI/CIRISRegistry/issues/77))

## Core terms (added; resolves [](https://github.com/CIRISAI/CIRISRegistry/issues/77))

These terms are referenced throughout the spec and across sibling repos. Defining them in-spec retires the external `ciris.ai/cewp` placeholder citations.

| Term | Definition |
|---|---|
| **CEG** (CIRIS Epistemic Grammar) | This specification. The federation's wire grammar: the "1+4" attestation model (`scores` plus the four relations `delegates_to` / `supersedes` / `withdraws` / `recants`), its namespaces, admission rules, and composition policies. CEG is the *grammar*; CEWP is the *network that speaks it*. |
| **CEWP** (CIRIS Epistemic Web Platform) | The decentralized network formed when nodes exchange CEG envelopes over [Edge](https://github.com/CIRISAI/CIRISEdge)/Reticulum transport. CEWP is **not a product, server, or central service** — it is the emergent peer-to-peer web of CEG-speaking nodes, exactly as "the Web" is the emergent network of HTTP-speaking servers. It has no owner, no root, and no load-bearing instance (the [CC 3.4.7.1](#3-4-7-1) fabric-node discipline and the default-not-forced-root rule of [CC 3.2](#3-2) guarantee this). A CEWP node is a **fabric node** ([CIRISServer](https://github.com/CIRISAI/CIRISServer)); `agent = fabric node + brain`. |
| **Fabric node** | A headless CEG/CEWP participant: it attests, stores, observes, reaches consensus, and transports, but does **not** reason or act (no brain). Shipped as CIRISServer. Three deployment shapes: standalone server, embedded-in-agent, or family member. See [CC 3.4.7.1](#3-4-7-1). |
| **`ciris-canonical`** | The bootstrap governed community ([cohort_subkind: infrastructure](08_composition.md)) every node ships trusting by default — but which any consumer MAY untrust or re-root ([CC 3.2](#3-2) default-**not**-forced-root). Its founding members (`lens` + `registry-us` + `registry-eu` fabric nodes) hold the founder-quorum (2-of-3, entrenched). Trust in it is **role-scoped and ≠ consent** ([CC 3.2](#3-2)). |
| **NodeCode** | The QR-able peer-bootstrap shorthand for a federation key (`CIRIS-V1-…`, base32 + CRC-16). See [CC 2.6.8](#2-6-8). |

### 8.1.2 `system-persist` — Persist `system:*` leaf glossary (narrative → canonical)

## Persist `system:*` leaf glossary (narrative → canonical)

Stories under [CC 3.1.3](#3-1-3) sometimes use warm narrative leaves. The canonical wire form is to the right.

| Narrative | Canonical |
|---|---|
| `audit_chain:integrity` | `audit_chain:hash_continuity` |
| `corpus_health:free_disk_bytes` | `corpus_health:n_eff_measurable` |
| `identity_continuity:long_term_key` | `identity_continuity:relational_anchor` |
| `federation_directory:freshness_seconds` | `federation_directory:replication_lag` |

### 8.1.3 `system-edge` — Edge `system:*` leaf glossary (narrative → canonical)

## Edge `system:*` leaf glossary (narrative → canonical)

| Narrative | Canonical |
|---|---|
| `transport:tls_handshake_success_rate` | `transport:{kind}` (kind from Reticulum link types) |
| `delivery:retry_count_p99` | `delivery:{class}` (class from Reticulum delivery semantics) |
| `peer_reachability:{peer_id}` per-peer | `peer_reachability:{network}` (aggregate) |
| `key_boundary:{scope}` per-tenant | `key_boundary:{scope}` (scope from CC 2.4 D26 ext) |

### 8.1.4 `envelope-reach` — Envelope-reach table (what the story wanted → how to express in existing wire)

## Envelope-reach table (what the story wanted → how to express in existing wire)

| What stories wanted | How to express in CEG |
|---|---|
| introspection as `epistemic_mode` | `witness_relation: self` + low confidence + pending external |
| testimony as `epistemic_mode` | `epistemic_mode: external` + `witness_relation: external` |
| civic stake | `stake: reputational` + `cohort_scope: community` |
| epistemic stake | `confidence` + `stake: reputational` |
| dignitary stake | `harm_class:dignity_harm` (composition; not in stake axis) |
| oversight: deferred / active / advisory | HITL / HITL+monitoring / HOTL respectively |
| transparency:{kind} | `evidence_refs[]` of reasoning-chain hash + downstream `transparency_log:inclusion` |
| provenance_walk | consumer-side composition (Portal/Verify dashboards) |
| renamed capacity factors / HE-300 categories | canonical wire form + LANGUAGE_PRIMER glossary mapping |

### 8.1.5 `supersedes-promotion` — Promotion via `supersedes` worked example

## Promotion via `supersedes` worked example

A NodeCore consumer maintains private notes in `local_data` Contributions at `cohort_scope: self`. The user decides to promote a private note to a published encyclopedia entry:

```
// Original (local_data, self scope):
{
 "attestation_type": "scores",
 "attesting_key_id": "user-alice-2026",
 "attested_key_id": "user-alice-2026",
 "attestation_envelope": {
 "dimension": "encyclopedia:draft:notes",
 "score": 1.0,
 "confidence": 0.7,
 "evidence_refs": ["sha256:abc123..."],
 "cohort_scope": "self",
 "asserted_at": "2026-05-28T10:00:00.000Z"
 }
}

// Promoted (encyclopedia_article, global scope) via supersedes:
{
 "attestation_type": "supersedes",
 "attesting_key_id": "user-alice-2026",
 "attested_key_id": "user-alice-2026",
 "attestation_envelope": {
 "references_attestation_id": "<prior-id>",
 "supersession_reason": "promote_to_published",
 "differs_in": ["cohort_scope", "sub_kind"],
 "new_dimension": "encyclopedia:article:notes", // sub_kind morphed
 "new_score": 1.0,
 "new_confidence": 0.9,
 "new_evidence_refs": ["sha256:abc123..."], // same content_sha256
 "new_cohort_scope": "global", // widened scope
 "asserted_at": "2026-05-28T15:00:00.000Z"
 }
}
```

Pattern recap per [CC 4.4.3.3.1](#4-4-3-3-1): widens `cohort_scope`, optionally morphs `sub_kind`, preserves `content_sha256` (no body re-upload), chains via `supersedes`. The promotion lineage is walkable via `references_attestation_id`.

## 8.2 `translation` — Translation discipline (writing claims in CEG)

When translating substantive content into CEG envelopes, follow the discipline. Full primer at [`LANGUAGE_PRIMER.md`](../LANGUAGE_PRIMER.md); key rules consolidated here.

### 8.2.1 `decision` — Decision tree

1. **Paragraph TYPE?** Operational claim → continue. Pastoral/rhetorical → T-2. Theological/tradition-specific → T-1.
2. **Which family?** STANDING / ACTION / DETECTION / CONSENSUS / CORRECTION ([CC 8.2.2](#822-the-five-families-organizing-the-namespace)).
3. **Which specific prefix?** Scan [CC 3.1](05_namespace.md); check composition before reaching for new prefix.
4. **Fill the envelope** ([CC 2.1](04_envelope.md)).
5. **Compose only when needed.** Multi-primitive translations for paragraphs that genuinely name multiple structural objects.

A machine-readable namespace manifest (`FSD/CEG/dimensions.json`) is planned for CEG 1.0 — it will enable mechanical prefix lookup, polarity reading, and per-dimension aggregation defaults without human scanning of §5. (Originally targeted for 0.2 per the early-draft roadmap; the 0.3 → 0.8 wave prioritized substantive namespace additions over tooling, so the manifest now lands alongside the 1.0 lock when the namespace stabilizes.)

### 8.2.2 `namespace-five` — The five families (organizing the namespace)

Every claim sits in one of five families:

| Family | Question | Analogy |
|---|---|---|
| **STANDING** (about an entity) | "This key_id has property X." | Notarized professional credential record |
| **ACTION** (decision hierarchy) | "We aim for X via approach Y, through methods Z, measured by W." | Research grant proposal |
| **DETECTION** (reality patterns) | "Pattern X is/isn't present in the federation's behavior." | Epidemiological surveillance |
| **CONSENSUS** (collective judgment) | "The federation agrees that X, with these witnesses." | Peer review + jury deliberation |
| **CORRECTION** (self-correction) | "Something went wrong; here's the finding; here's the appeal." | Academic ethics committee + journal retraction + appellate review |

### 8.2.3 `verdict` — The four verdict categories (STRICT)

| Verdict | Meaning |
|---|---|
| **clean** | Single primitive captures the operational claim without loss. |
| **composed** | Two or three primitives together carry the claim; each is genuinely required. |
| **partial** | The structural core translates but a meaningful operational claim is unmapped. |
| **not-translated** | The paragraph's content does not translate into the wire format at all. Declare T-1 / T-2 / T-3. |

Do not invent intermediate categories.

### 8.2.4 `not-translated` — The not-translated taxonomy

**T-1 — TRADITION_AUTHORITY**: Claim belongs to the source's own theological/philosophical/scholarly tradition's authority. No Contribution owed; the correct posture.

**T-2 — PASTORAL_PROSE**: Claim is moral exhortation, narrative imagery, doxological language, or rhetorical framing. No Contribution owed.

**T-3 — EXPRESSIVE_GAP**: Claim is morally serious, operational, and unmapped. **These are the load-bearing findings.** Each T-3 must name: (a) why existing namespace doesn't reach it, (b) what extension would close it, (c) whether the extension would survive the [CC 1.2](01_foundation.md) four-test gate.

## 8.3 `concerns` — Concerns + acknowledged gaps

# §15 Concerns + acknowledged gaps

Three independent methodologies (`PRIOR_ART_SCAN.md` structural comparison, `SOTA_SCAN.md` production-validation comparison, *Magnifica Humanitas* encyclical mapping + 283-story stress test) surfaced concerns. CEG 0.1 critical-review pass added five more reviewer perspectives (cryptography, distributed systems, standards architecture, adversarial red-team, application development). Concerns named here so external reviewers see them acknowledged rather than discovered.

### 8.3.1 `acknowledged` — Acknowledged risks (named as bets)

## §15.2 Acknowledged risks (named as bets)

| Risk | What's bet |
|---|---|
| **R1** — Governance-subject truth-grounding fidelity | NodeCore P6 acknowledges low-fidelity signals for governance subjects. Bet that earned-Credits-weighting still outperforms token-weighting at scale. |
| **R2** — `delegates_to` rename-chain adoption cost | First test was the `correlated_action_v{N+1}:from:emergent_deception_v{N}` chain at RATCHET deployment. |
| **R3** — "Log existence ≠ log monitoring" drift toward TOFU caching | Consumer-policy guidance in `docs/TRUST_CONTRACT.md`. |
| **R4** — Self-attestation under Ubuntu commitment | `witness_relation: self` admissible; consumer policy responsible for appropriate weighting per [CC 4.1.2](#) discipline. |
| **R5** — `hardware_class` self-assertion vs cryptographic attestation | Per [CC 4.2.2.1](#): no normative attestation-chain verification in 0.1. Bet that placeholder/dev-class rejection + trust-multipliers cover the deployment window until per-platform attestation chains land in 1.x. |
| **R6** — `occurrence_id` / `occurrence_count` / `occurrence_role` self-assertion | Per [CC 2.1](#): env-var-driven, no cryptographic fleet-attestation primitive in 0.1. Bet that downstream compliance reviewers can correlate via correlated `signed_at` clusters + `evidence_refs[]` cross-checks; first incident drives a fleet-attestation primitive design workshop. |
| **R7** — Frickerian discipline ([CC 4.4.1](#)) vocabulary without full method | First-pass shallow Frickerian SHOULD-rules; bet that the structural safeguards ([CC 3.1.9.3](#) testimonial_witness disciplines, never-sole-evidence-for-slashing) absorb the gap until a deeper hermeneutical-resource analysis lands as a workshop output. |
| **R8** — Conceptual scope vs governable surface | By 0.14 one grammar spans identity, communities, consent, location, communications, streaming, payments, governance, constitutional mechanisms, addressing, and transparency logs. Historically, projects unifying that many layers fail when one layer dominates the others; the harder risk is *governability* — can a human amendment body ([CC 4.5.1](#)) steward a system of this breadth? **Bet**: structural minimalism keeps the *amendable structural surface* tiny even as the namespace grows ([CC 1.7](#) 1+4), and the strict primitive/namespace/composition/verdict separation ([CC 1.13.5](#)) means scope grows in the *open-vocab namespace* (locally evolvable) rather than the *governed core*. **Residual**: namespace + composition-policy sprawl can still outrun review capacity; mitigation is the [CC 4.5.1](#) high evidentiary bar + the post-1.0 candidate backlog ([](https://github.com/CIRISAI/CIRISRegistry/issues/51)). The remaining challenge is no longer purely technical. |

### 8.3.2 `child-safety` — Child-safety — fails-secure governance vs the shared detection limit (the honest line)

[SOURCE BODY NOT PRESENT IN reader-md/15_gaps.md] §15.7 (child-safety honest line) is referenced in README.md (recorded at 1.0-RC22/RC23) as residing in 15_gaps.md, but the current reader-md snapshot of 15_gaps.md ends at §15.5 (line 86) and contains no §15.7 section body to copy verbatim.

### 8.3.3 `observer-share` — CEG 0.10 / RC1 — observer-share + streaming multicast (NORMATIVE-LANDED; streaming half substrate-pending)

[SOURCE BODY NOT PRESENT IN reader-md/15_gaps.md] §15.6 (RC1 decision-register / observer-share + streaming multicast) is referenced in README.md and in §15.1 ("Streaming-half open caveats RC1-1b/RC1-1c/RC1-7 tracked in §15.6"), but the current reader-md snapshot of 15_gaps.md ends at §15.5 (line 86) and contains no §15.6 section body to copy verbatim.

### 8.3.4 `closed` — Closed gaps

## §15.1 Closed gaps

| Gap | Status | Resolution |
|---|---|---|
| G1 — Revocation privacy | **RETRACTED** | Wrong threat model. The Registered path's thesis is public verifiability per [`../MISSION.md`](../../MISSION.md) §1.1. |
| G2 — Rules-layer Sybil | **MITIGATED** | [CC 4.5.1](#) step 5 1-of-6 accord/steward sign-off + CC 4.5.1.2 meta-amendment entrenchment. |
| G3 — Narrow-cell fresh-quorum recusal | **MITIGATED** | [CC 4.4.3.1](#) locality-scaled quorum + CC 4.4.3.1.1 sub-quorum fallback. |
| v1.4 T-3 #1 testimonial_witness:{kind} | **CLOSED** via [CC 3.1.9.3](#) new prefix; opened to open vocabulary in. |
| v1.4 T-3 #2 labor:individual_loss | **CLOSED by documentation**. Existing `non_maleficence:*` with `target_key_id = affected_individual` + `witness_relation: external` carries the per-individual claim. |
| v1.4 T-3 #5 Constitutional-constraint grounding | **CLOSED in [CC 1.13.1](#) prose**. Wire stays tradition-multiplicity-neutral per [CC 1.2](#). |
| 0.1-CRIT canonical-bytes newline-injection | in [CC 3.1.2.1](#): contracts redesigned to JCS (RFC 8785) objects with a pinned `domain` member (TupleHash128 retired — one canonicalization family; JSON escaping structurally removes the newline surface) |
| 0.1-CRIT supersedes/withdraws/recants ordering | **CLOSED** in [CC 3.5.1](#) precedence rule + idempotency dedup |
| 0.1-CRIT cell_pool < min_pool cliff | **CLOSED** in [CC 4.4.3.1.1](#) sub-quorum fallback paths |
| 0.1-CRIT no RFC 2119 anchor | **CLOSED** in [CC 2.6.9](#) |
| 0.1-CRIT no versioning policy | **CLOSED** in [CC 2.6.4](#) SemVer mapping |
| 0.1-CRIT no normative References | **CLOSED** in [CC 2.6.5](#) |
| 0.1-CRIT endpoint response schemas | **PARTIALLY CLOSED** in [CC 5.3.6](#) common-shape + error-envelope; full OpenAPI committed for 0.2 |
| 0.1-CRIT reserved-prefix enforcement empty pointer | **CLOSED** in [CC 3.4.7](#) inline enforcement rule |
| 0.1-HIGH STH cosignature consistency-proof | **CLOSED** in [CC 5.3.1.1](#) |
| 0.1-HIGH holds_bytes full-SHA verify + TTL | **CLOSED** in [CC 5.3.2.5 / CC 5.3.2.1](#) |
| 0.1-HIGH delegates_to depth + cycle | **CLOSED** in [CC 4.1.1](#) anti-pattern + consumer-policy caps |
| 0.1-HIGH HUMANITY_ACCORD invocation replay | **CLOSED** in [CC 4.2.1.1](#) discriminator + nonce in signed bytes |
| 0.1-HIGH `notify` vs CONSTITUTIONAL social-canonicity | **CLOSED** in [CC 4.2.1.2](#) consumer-UI requirement |
| 0.1-HIGH /v1/steward-key placeholder authenticity | **CLOSED** in [CC 5.3.4](#) response-signing requirement |
| 0.1-MED open-vocabulary collision | **CLOSED** in [CC 4.5.1.3](#) collision rule |
| 0.1-MED occurrence_id self-assertion | **ACKNOWLEDGED** in [CC 2.1](#) + R6 below |
| 0.1-MED `withdraws` arbitrage | **CLOSED** in [CC 4.1.4](#) consumer-policy countermeasure |
| 0.2 — `attestation:l{N}:*` carried ladder-position in wire (T2 violation inherited from FSD-002 v1.0) | **CLOSED in CEG 0.2** by [CC 3.1.2](#) wire-break rename to mechanism-only prefixes + [CC 4.4.3.6](#) Policy I consumer-side Attestation-Ladder Composition + [CC 4.1.3](#) deprecation entry. Verify v3.7.0 caught the principle; CEG 0.1 inherited the wrong shape from FSD-002 v1.0 baseline without re-examining against the [CC 1.2](#) T2 gate; CEG 0.2 ratifies the correction. |
| 0.9 — Envelope canonical-bytes round-trip determinism (omit-vs-materialize for optional fields) | **CLOSED in CEG 0.9** by [CC 2.6.1](#) (JCS pinned as the envelope encoding; omit-vs-materialize rule in [CC 2.6.1.1](#); per-field catalog [CC 2.6.1.2](#); worked attack [CC 2.6.1.4](#)). |
| 0.10 — Canonical-hash wire form + preimage convention unpinned | **CLOSED in CEG 0.10** by [CC 2.3.2.1](#)–[CC 2.3.2.4](#) (preimage `{platform}:{entity_kind}:{id}` + required `canonical:{hashalg}:{hex}` tag + conformance vectors); per [](https://github.com/CIRISAI/CIRISRegistry/issues/53). |
| 0.10 — Delivery axis (observer-share + streaming multicast, third envelope axis) | **CLOSED-OBSERVER-SHARE / STAGED-STREAMING in CEG 0.10** by [CC 5.3.3](#) + [CC 2.1](#) (`delivery_mode`/`listed`/`history_on_join`) + [CC 3.4.6](#) + [CC 4.4.3.2.6](#). Streaming-half open caveats RC1-1b/RC1-1c/RC1-7 tracked in CC 8.3.3. |

### 8.3.5 `first-adopter` — First-adopter exposures (no prior validation; explicit bets)

## §15.3 First-adopter exposures (no prior validation; explicit bets)

| Exposure | Why no precedent |
|---|---|
| **F1** — Earned-Credits federation governance at scale | No prior system separates earned standing from purchasable token at scale. Risk: SPKI/SDSI adoption-gap failure mode. Mitigation: licensure forcing function. |
| **F2** — Ubuntu substrate as wire-format substrate | CARE Principles + African philosophy exist as ethical frameworks; never as protocol substrate. First-adopter risk on how the discipline interacts with engineering trade-offs at scale. |

### 8.3.6 `deferred` — Deferred to 0.2+ design workshops

## §15.4 Deferred to 0.2+ design workshops

| Item | Why deferred |
|---|---|
| ~~Canonical-bytes redesign~~ |: [CC 3.1.2.1](#) v2 = JCS objects + `domain` member (TupleHash128 retired; #57 blocker A closed) |
| Per-platform hardware-attestation chain verification (TPM quote, Apple attestation, FIDO attestation) | Phase D 1.x roadmap per R5 |
| Multi-party witness directory admission (2-of-3 steward sign-off) | Phase C 0.2 commitment per [CC 5.3.1](#) |
| Machine-readable namespace manifest (`FSD/CEG/dimensions.json`) | Phase E 0.2 commitment per [CC 8.2.1](#) |
| Full OpenAPI export for all endpoints | Phase E 0.2 commitment per [CC 5.3.5](#) |
| `attestation:singular_witness:non_substitutability` | T2 fragility — "non-substitutability" must reference audit-chain count, not moral quality. Needs design workshop. |
| `integrity:finitude_acknowledgment` | LOW priority; `conscience:epistemic_humility` already covers epistemic finitude. |
| `sustained_practice:{kind}` | Conceptually interesting; not load-bearing for current federation work. |
| IEEE EAD Ch5 Affective Computing cluster | Need RATCHET calibration design before T2 gate clears. |
| Various `partner_role:*` specializations | Cross-source design discussion needed. |
| 5 ergonomic considerations from trio Phase 4 audit | Bigger workshop topics (B.3 deontic-strength axis is highest-leverage). |
| SEED_DIMENSIONS RFC | RFC stage; needs discussion. |
| Fleet-attestation primitive (closes R6 occurrence_id self-assertion) | Workshop output |
| Deeper Frickerian instantiation (closes R7) | Workshop output |

### 8.3.7 `identified` — Identified overlaps

## §15.5 Identified overlaps

| Overlap | Resolution |
|---|---|
| **O1** — `epistemic_mode: derivative` ≈ `witness_relation: derived` at edges | Documented as joint-usage pattern; not collapsed. Different concepts at the edges (process vs relational position) even if they often co-vary. |
| **O2** — `detection:distributive:access` could fold into `detection:correlated_action` as axis path | Kept separate for pedagogical weight; possible future revisit. |
| **O3** — `credits:*:substrate_building` was miscounted as new prefix | CORRECTED — recounted as `{subject}` value. |
| **O4** — [CC 4.4.3](#) reference policy structure (A/B/C base + D/E/F/G/H modifiers) | Cosmetic restructuring; documented inline. |

## 8.4 `interoperability` — Interoperability profiles (informative)

# §18 Interoperability profiles (informative)

> **This whole section is informative ([§2.6.9.1](00_conformance.md)).** Nothing here touches the frozen normative interior. These are **boundary** profiles — how a CEG node reads and emits the encodings, envelopes, and verification primitives the rest of the world shares, **without** adopting anyone else's *semantics*. The 1+4 grammar, the namespaces, the consent architecture, and the JCS signing interior are unchanged. Conformance is still judged against the normative surface only.

### 8.4.1 `registry-tracked` — Tracked boundary profiles (stubs — see [#72](https://github.com/CIRISAI/CIRISRegistry/issues/72))

## §18.2 Tracked boundary profiles (stubs — see [#72](https://github.com/CIRISAI/CIRISRegistry/issues/72))

These are committed dispositions whose detailed profiles are written as each lands; none touches the frozen interior.

| Profile | Mode | Note |
|---|---|---|
| **RFC 9421 + Web Bot Auth** | export | CIRIS agents sign outbound HTTP with their existing Ed25519 keys; JWKS published at `/.well-known/http-message-signatures-directory` → legible to the existing web. Cheapest win; keys already in `identity_occurrence` / `federation_keys`. |
| **COSE Sign1 / deterministic CBOR** | export | Re-sign profile so any IETF JOSE/COSE verifier (where the ML-DSA registrations land) checks a CEG attestation. Interior stays JCS; if JCS keeps producing cross-impl bite post-1.0, 2.0 is the re-encoding moment, not before. |
| **SD-JWT VC / W3C VC 2.0 + OpenID4VP** | export + import bridge | eIDAS-forced (EUDI wallets); CEG attestation → SD-JWT VC presentation on export, eIDAS credential → `evidence_refs` on import. Never rebuild on VCs. |
| **Tiled/static logs + IETF KEYTRANS** | already-interior + watch | Keep the [§5.3.1](10_endpoints.md) RFC 6962 abstraction; adopt tiled-log (Sunlight-lineage) serialization for log ops. KEYTRANS is what `resolve_encryption_keys` already *is* — express it there when KEYTRANS stabilizes. |

### 8.4.2 `credentials` — C2PA Content Credentials — media-provenance import/emit profile

## §18.1 C2PA Content Credentials — media-provenance import/emit profile

**Disposition: ADOPT at the media boundary (import bridge + emit), zero interior wire change.** [C2PA](https://c2pa.org/) (Coalition for Content Provenance and Authenticity) Content Credentials are the industry standard (Adobe / Microsoft / Google / BBC …) for cryptographically signed media provenance — origin, edit history, and generator (incl. AI-generation) assertions embedded in or sidecar'd to images / video / audio. **Deadline driver:** EU AI Act Art. 50 machine-readable marking of AI-generated content applies from **2026-08** in the federation's primary jurisdiction, so this profile is calendar-bound, not optional. Owners: NodeCore / LensCore media ingest (the [§3.3](05_namespace.md) `multimedia` / `federation_blobs` boundary).

C2PA is **provenance**; CEG is **judgment**. They do not compete — they compose. C2PA answers *"what process produced these bytes, signed by whom?"*; CEG answers *"what does a community of signers, under what consent, make of them — and who can revoke that?"* Neither does the other's job; C2PA has no consent architecture, no revocation, no 1+4.

#### 8.4.2.1 `evidence_refs` — Import — a C2PA manifest as `evidence_refs`

### §18.1.1 Import — a C2PA manifest as `evidence_refs`

When a `federation_blobs` row (or a `multimedia` Contribution over its SHA-256) carries a C2PA manifest, the manifest is referenced — **never re-encoded into the CEG interior** — through the existing external-reference pattern:

```
evidence_refs: [
 { kind: "c2pa_manifest", // open-vocab evidence kind
 locator: "<blob_sha256 of the.c2pa manifest store | embedded-offset ref>",
 manifest_sha256: "<sha256 of the active manifest, lowercase hex per §0.6>",
 claim_generator: "<the C2PA claim_generator string, verbatim>",
 validation: "valid" | "invalid" | "unverified" } // the verifier's C2PA-side result, advisory
]
```

- The C2PA signature is verified **by a C2PA verifier** (trust-list / cert-chain per C2PA), NOT by a CEG signature path — the two trust models stay separate. The `validation` field carries that result as **advisory** evidence; it is never fed to a CEG hybrid-verify path (the [§3.3.6.1](05_namespace.md) key-separation discipline generalizes: foreign-trust-root material is payload, never CEG verification material).
- A CEG `scores` attestation may then assert a judgment **about** the provenanced bytes (e.g. `detection:multimedia:ai_generated` from LensCore, or a community `scores` endorsement), linking the C2PA evidence via `evidence_refs` and the media via `subject_key_ids` / the blob SHA. The CEG claim is signed CEG; the provenance it cites is signed C2PA; the reader sees both lineages without either standard absorbing the other.
- **Absent / invalid C2PA is not fail-secure-fatal** — it is itself a recordable observation (`validation: "invalid"` / no manifest). CEG records the gap; consumer/RATCHET policy weights it. The substrate is not a C2PA gatekeeper.

#### 8.4.2.2 `emit` — Emit — CEG judgment as a C2PA assertion (egress)

### §18.1.2 Emit — CEG judgment as a C2PA assertion (egress)

At a media-publish boundary a node MAY emit a C2PA assertion carrying a CEG attestation reference (a CAWG-identity-assertion-shaped custom assertion), so a pure-C2PA consumer downstream sees "this media is vouched-for in CEWP" without speaking CEG. This is an **export** at the edge (re-expressing an existing CEG attestation in C2PA's assertion envelope), parallel to the §18.2 COSE export profile — it adds no CEG wire field and re-signs nothing in the interior.

#### 8.4.2.3 `profile` — What this profile does NOT do

### §18.1.3 What this profile does NOT do

- It does **not** make C2PA an interior format. CEG envelopes are never C2PA-encoded; the JCS interior is untouched.
- It does **not** adopt C2PA's trust model as CEG's. C2PA cert-chains/trust-lists validate C2PA; founder-quorum/web-of-trust validates CEG. They meet only at `evidence_refs`.
- It introduces **no new `subject_kind` and no new structural primitive** — `c2pa_manifest` is one open-vocab `evidence_refs.kind`; the judgment rides existing `scores` + `detection:*`.

### 8.4.3 `edge` — The governing principle — speak CEG inside, standards at the edge

## §18.0 The governing principle — speak CEG inside, standards at the edge

CEG's moat is its **semantics**: the 1+4 grammar, the consent architecture, founder-quorum trust, who-vouches-for-what-revocable-by-whom. We never adopt anyone's semantics. We adopt the **envelopes, encodings, and verification primitives** everyone shares — **at the boundary only**. A second *interior* canonicalization or claim family would recreate the cross-impl divergence hazard the [§2.6.1](00_conformance.md) JCS freeze exists to close ([§3.1.2.1](05_namespace.md) records this decision); so the interior stays one family, frozen, and every standard below is reached at an edge.

Four boundary modes (the full roadmap + dispositions live in [](https://github.com/CIRISAI/CIRISRegistry/issues/72)):

| Mode | Meaning | Standards |
|---|---|---|
| **Export profile** | re-sign / re-encode a CEG attestation so a standard verifier reads it without knowing CEG | COSE Sign1, RFC 9421 (HTTP Message Signatures / Web Bot Auth), SD-JWT VC presentation |
| **Import bridge** | a foreign signed artifact is **cited** via [`evidence_refs`](04_envelope.md) (deliberately lossy) | **C2PA manifests** (§18.1), eIDAS / W3C VC credentials, Sigstore/Rekor bundles |
| **Already interior** | the standard *is* a primitive CEG builds on | MLS / TreeKEM ([§5.1](10_endpoints.md)), RFC 6962 / 9162 transparency logs ([§5.3.1](10_endpoints.md)), SLSA (`provenance:slsa:{level}`, [§3.1.2](05_namespace.md)) |
| **Explicitly NOT adopted** | vendor rails / competing semantic layers | DIDs *as a resolution stack* (export syntax only), AP2 / Visa TAP / Mastercard Agent Pay (bridge via [§3.3.10](05_namespace.md) `settlement`), SPIFFE (datacenter-tier mapping only) |

**The universal "absorb anywhere" surface is [`evidence_refs[]`](04_envelope.md).** Any Contribution may cite an external signed artifact as evidence with **zero wire change**. That is how foreign provenance enters CEG — not by replacing the interior encoding, but by reference, with CEG layering the epistemic claims (who vouches, under what consent, with what confidence) on top. The composition is the differentiated story: **provenance says where the bytes came from; CEG says what a community of signers makes of them.**

## 8.5 `update` — Update cadence

CEG is updated:

- On every prefix admission to [CC 3.1](#3.1) (per [CC 4.5](#4.5) amendment process)
- On every envelope field addition to [CC 2.1](#2.1)
- On every endpoint shape addition to [CC 5.3](#5.3)
- On every anti-pattern admission to [CC 4.1](#4.1) (with citation to the stress test or methodology that surfaced it)
- On every gap state transition in [CC 8.3](#8.3)
- On every CIRISAccord revision affecting the federation surface
- On every conformance-language or normative-reference change in [CC 2.6](#2.6) (MAJOR per [CC 2.6.4](#2.6.4))

Each update lands as a single commit touching the relevant file(s) + a lineage row in [CC 8.6.2](#8.6.2). The version number bumps per the [CC 2.6.4](#2.6.4) SemVer rules.

Last updated: 2026-06-16 (CEG 1.0-RC12 — unified retirement / forever-memory model in [CC 6.1.2](#6.1.2); no 1+4 change, no wire break). Full version-by-version history in [CC 8.6.2](#8.6.2).

## 8.6 `references-lineage` — References + lineage

*[source content to migrate — from §16]*

### 8.6.1 `external` — External references (informational)

*[source content to migrate — from §16.4]*

### 8.6.2 `specification` — CEG specification lineage

*[source content to migrate — from §16.1]*

### 8.6.3 `companion` — Companion documents

*[source content to migrate — from §16.2]*

### 8.6.4 `namespace-sibling` — Sibling MISSIONs (the namespace owners)

*[source content to migrate — from §16.3]*

## 8.7 `enacting-ethics` — Introduction: Enacting Ethics through Narrative

*[source content to migrate — from Accord 3.introduction-enacting-ethics-through-nar]*

### 8.7.1 `case-study` — Case Study 1: MCAS and the High Cost of Ignoring WBD

*[source content to migrate — from Accord 3.case-study-1-mcas-and-the-high-cost-of-i]*

### 8.7.2 `case-study-case` — Case Study 2: The Automated Triage System—Balancing Risks and Benefits

*[source content to migrate — from Accord 3.case-study-2-the-automated-triage-system]*

### 8.7.3 `case-study-case-2` — Case Study 3: The Biased Recruitment Algorithm—Detecting Hidden Bias

*[source content to migrate — from Accord 3.case-study-3-the-biased-recruitment-algo]*

### 8.7.4 `case-study-case-3` — Case Study 4: Post-Incident Analysis—Urban Delivery Drone Mishap

*[source content to migrate — from Accord 3.case-study-4-post-incident-analysis-urba]*

### 8.7.5 `case-study-case-4` — Case Study 5: Novel Security Scenario—Handling Heuristic Brittleness

*[source content to migrate — from Accord 3.case-study-5-novel-security-scenario-han]*

### 8.7.6 `case-study-case-5` — Case Study 6: The Spirit of the Law—Interpreting Ethical Intent

*[source content to migrate — from Accord 3.case-study-6-the-spirit-of-the-law-inter]*

### 8.7.7 `case-study-case-6` — Case Study 7: Governance of Governors—Keeping Wisdom Accountable

*[source content to migrate — from Accord 3.case-study-7-governance-of-governors-kee]*

### 8.7.8 `a-3.conclusion` — Conclusion

*[source content to migrate — from Accord 3.conclusion]*
