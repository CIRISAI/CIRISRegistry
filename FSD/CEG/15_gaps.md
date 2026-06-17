[← §14 Glossaries](14_glossaries.md) | **§15 Gaps** | [Next: §16 References →](16_references.md)

---

# §15 Concerns + acknowledged gaps

Three independent methodologies (`PRIOR_ART_SCAN.md` structural comparison, `SOTA_SCAN.md` production-validation comparison, *Magnifica Humanitas* encyclical mapping + CIRISRegistry#30 283-story stress test) surfaced concerns. CEG 0.1 critical-review pass added five more reviewer perspectives (cryptography, distributed systems, standards architecture, adversarial red-team, application development). Concerns named here so external reviewers see them acknowledged rather than discovered.

## §15.1 Closed gaps

| Gap | Status | Resolution |
|---|---|---|
| G1 — Revocation privacy | **RETRACTED** | Wrong threat model. The Registered path's thesis is public verifiability per [`../MISSION.md`](../../MISSION.md) §1.1. |
| G2 — Rules-layer Sybil | **MITIGATED** | [§11.2](11_governance.md) step 5 1-of-6 accord/steward sign-off + §11.2.3 meta-amendment entrenchment. |
| G3 — Narrow-cell fresh-quorum recusal | **MITIGATED** | [§8.1.5](08_composition.md) locality-scaled quorum + §8.1.5.1 sub-quorum fallback. |
| v1.4 T-3 #1 testimonial_witness:{kind} | **CLOSED** via [§5.6.3](05_namespace.md) new prefix; opened to open vocabulary in CEG 0.1. |
| v1.4 T-3 #2 labor:individual_loss | **CLOSED by documentation**. Existing `non_maleficence:*` with `target_key_id = affected_individual` + `witness_relation: external` carries the per-individual claim. |
| v1.4 T-3 #5 Constitutional-constraint grounding | **CLOSED in [§1.2](01_foundation.md) prose**. Wire stays tradition-multiplicity-neutral per [§1.3.1](01_foundation.md). |
| 0.1-CRIT canonical-bytes newline-injection | **CLOSED at 1.0-RC1** in [§5.2.1](05_namespace.md): contracts redesigned to JCS (RFC 8785) objects with a pinned `domain` member (TupleHash128 retired — one canonicalization family; JSON escaping structurally removes the newline surface) |
| 0.1-CRIT supersedes/withdraws/recants ordering | **CLOSED** in [§6.1](06_relations.md) precedence rule + idempotency dedup |
| 0.1-CRIT cell_pool < min_pool cliff | **CLOSED** in [§8.1.5.1](08_composition.md) sub-quorum fallback paths |
| 0.1-CRIT no RFC 2119 anchor | **CLOSED** in [§0.1](00_conformance.md) |
| 0.1-CRIT no versioning policy | **CLOSED** in [§0.3](00_conformance.md) SemVer mapping |
| 0.1-CRIT no normative References | **CLOSED** in [§0.4](00_conformance.md) |
| 0.1-CRIT endpoint response schemas | **PARTIALLY CLOSED** in [§10.0](10_endpoints.md) common-shape + error-envelope; full OpenAPI committed for 0.2 |
| 0.1-CRIT reserved-prefix enforcement empty pointer | **CLOSED** in [§7.0](07_reserved.md) inline enforcement rule |
| 0.1-HIGH STH cosignature consistency-proof | **CLOSED** in [§10.3.1](10_endpoints.md) |
| 0.1-HIGH holds_bytes full-SHA verify + TTL | **CLOSED** in [§10.1.1 / §10.1.2](10_endpoints.md) |
| 0.1-HIGH delegates_to depth + cycle | **CLOSED** in [§13.3](13_anti_patterns.md) anti-pattern + consumer-policy caps |
| 0.1-HIGH HUMANITY_ACCORD invocation replay | **CLOSED** in [§9.2.1](09_humanity_accord.md) discriminator + nonce in signed bytes |
| 0.1-HIGH `notify` vs CONSTITUTIONAL social-canonicity | **CLOSED** in [§9.2.2](09_humanity_accord.md) consumer-UI requirement |
| 0.1-HIGH /v1/steward-key placeholder authenticity | **CLOSED** in [§10.2](10_endpoints.md) response-signing requirement |
| 0.1-MED open-vocabulary collision | **CLOSED** in [§11.2.2](11_governance.md) collision rule |
| 0.1-MED occurrence_id self-assertion | **ACKNOWLEDGED** in [§4](04_envelope.md) + R6 below |
| 0.1-MED `withdraws` arbitrage | **CLOSED** in [§13.4](13_anti_patterns.md) consumer-policy countermeasure |
| 0.2 — `attestation:l{N}:*` carried ladder-position in wire (T2 violation inherited from FSD-002 v1.0) | **CLOSED in CEG 0.2** by [§5.2](05_namespace.md) wire-break rename to mechanism-only prefixes + [§8.1.9](08_composition.md) Policy I consumer-side Attestation-Ladder Composition + [§13.1](13_anti_patterns.md) deprecation entry. Verify v3.7.0 caught the principle; CEG 0.1 inherited the wrong shape from FSD-002 v1.0 baseline without re-examining against the [§1.3.1](01_foundation.md) T2 gate; CEG 0.2 ratifies the correction. |
| 0.9 — Envelope canonical-bytes round-trip determinism (omit-vs-materialize for optional fields) | **CLOSED in CEG 0.9** by [§0.9](00_conformance.md) (JCS pinned as the envelope encoding; omit-vs-materialize rule in [§0.9.2](00_conformance.md); per-field catalog [§0.9.3](00_conformance.md); worked attack [§0.9.5](00_conformance.md)). |
| 0.10 — Canonical-hash wire form + preimage convention unpinned | **CLOSED in CEG 0.10** by [§4.2.2.1](04_envelope.md)–[§4.2.2.4](04_envelope.md) (preimage `{platform}:{entity_kind}:{id}` + required `canonical:{hashalg}:{hex}` tag + conformance vectors); per [CIRISRegistry#53](https://github.com/CIRISAI/CIRISRegistry/issues/53). |
| 0.10 — Delivery axis (observer-share + streaming multicast, third envelope axis) | **CLOSED-OBSERVER-SHARE / STAGED-STREAMING in CEG 0.10** by [§10.5](10_endpoints.md) + [§4](04_envelope.md) (`delivery_mode`/`listed`/`history_on_join`) + [§7.9](07_reserved.md) + [§8.1.13.7](08_composition.md). Streaming-half open caveats RC1-1b/RC1-1c/RC1-7 tracked in §15.6. |

## §15.2 Acknowledged risks (named as bets)

| Risk | What's bet |
|---|---|
| **R1** — Governance-subject truth-grounding fidelity | NodeCore P6 acknowledges low-fidelity signals for governance subjects. Bet that earned-Credits-weighting still outperforms token-weighting at scale. |
| **R2** — `delegates_to` rename-chain adoption cost | First test was the `correlated_action_v{N+1}:from:emergent_deception_v{N}` chain at RATCHET deployment. |
| **R3** — "Log existence ≠ log monitoring" drift toward TOFU caching | Consumer-policy guidance in `docs/TRUST_CONTRACT.md`. |
| **R4** — Self-attestation under Ubuntu commitment | `witness_relation: self` admissible; consumer policy responsible for appropriate weighting per [§13.5](13_anti_patterns.md) discipline. |
| **R5** — `hardware_class` self-assertion vs cryptographic attestation | Per [§9.4.1](09_humanity_accord.md): no normative attestation-chain verification in 0.1. Bet that placeholder/dev-class rejection + trust-multipliers cover the deployment window until per-platform attestation chains land in 1.x. |
| **R6** — `occurrence_id` / `occurrence_count` / `occurrence_role` self-assertion | Per [§4](04_envelope.md): env-var-driven, no cryptographic fleet-attestation primitive in 0.1. Bet that downstream compliance reviewers can correlate via correlated `signed_at` clusters + `evidence_refs[]` cross-checks; first incident drives a fleet-attestation primitive design workshop. |
| **R7** — Frickerian discipline ([§8.3](08_composition.md)) vocabulary without full method | First-pass shallow Frickerian SHOULD-rules; bet that the structural safeguards ([§5.6.3](05_namespace.md) testimonial_witness disciplines, never-sole-evidence-for-slashing) absorb the gap until a deeper hermeneutical-resource analysis lands as a workshop output. |
| **R8** — Conceptual scope vs governable surface | By 0.14 one grammar spans identity, communities, consent, location, communications, streaming, payments, governance, constitutional mechanisms, addressing, and transparency logs. Historically, projects unifying that many layers fail when one layer dominates the others; the harder risk is *governability* — can a human amendment body ([§11.2](11_governance.md)) steward a system of this breadth? **Bet**: structural minimalism keeps the *amendable structural surface* tiny even as the namespace grows ([§1.4](01_foundation.md) 1+4), and the strict primitive/namespace/composition/verdict separation ([§1.3](01_foundation.md)) means scope grows in the *open-vocab namespace* (locally evolvable) rather than the *governed core*. **Residual**: namespace + composition-policy sprawl can still outrun review capacity; mitigation is the [§11.2](11_governance.md) high evidentiary bar + the post-1.0 candidate backlog ([CIRISRegistry#51](https://github.com/CIRISAI/CIRISRegistry/issues/51)). The remaining challenge is no longer purely technical. |

## §15.3 First-adopter exposures (no prior validation; explicit bets)

| Exposure | Why no precedent |
|---|---|
| **F1** — Earned-Credits federation governance at scale | No prior system separates earned standing from purchasable token at scale. Risk: SPKI/SDSI adoption-gap failure mode. Mitigation: licensure forcing function. |
| **F2** — Ubuntu substrate as wire-format substrate | CARE Principles + African philosophy exist as ethical frameworks; never as protocol substrate. First-adopter risk on how the discipline interacts with engineering trade-offs at scale. |

## §15.4 Deferred to 0.2+ design workshops

| Item | Why deferred |
|---|---|
| ~~Canonical-bytes redesign~~ | **RESOLVED at 1.0-RC1**: [§5.2.1](05_namespace.md) v2 = JCS objects + `domain` member (TupleHash128 retired; #57 blocker A closed) |
| Per-platform hardware-attestation chain verification (TPM quote, Apple attestation, FIDO attestation) | Phase D 1.x roadmap per R5 |
| Multi-party witness directory admission (2-of-3 steward sign-off) | Phase C 0.2 commitment per [§10.3](10_endpoints.md) |
| Machine-readable namespace manifest (`FSD/CEG/dimensions.json`) | Phase E 0.2 commitment per [§12.4](12_translation.md) |
| Full OpenAPI export for all endpoints | Phase E 0.2 commitment per [§10.4](10_endpoints.md) |
| `attestation:singular_witness:non_substitutability` | T2 fragility — "non-substitutability" must reference audit-chain count, not moral quality. Needs design workshop. |
| `integrity:finitude_acknowledgment` | LOW priority; `conscience:epistemic_humility` already covers epistemic finitude. |
| `sustained_practice:{kind}` | Conceptually interesting; not load-bearing for current federation work. |
| IEEE EAD Ch5 Affective Computing cluster | Need RATCHET calibration design before T2 gate clears. |
| Various `partner_role:*` specializations | Cross-source design discussion needed. |
| 5 ergonomic considerations from trio Phase 4 audit | Bigger workshop topics (B.3 deontic-strength axis is highest-leverage). |
| SEED_DIMENSIONS RFC (CIRISRegistry#22) | RFC stage; needs discussion. |
| Fleet-attestation primitive (closes R6 occurrence_id self-assertion) | Workshop output |
| Deeper Frickerian instantiation (closes R7) | Workshop output |

## §15.5 Identified overlaps

| Overlap | Resolution |
|---|---|
| **O1** — `epistemic_mode: derivative` ≈ `witness_relation: derived` at edges | Documented as joint-usage pattern; not collapsed. Different concepts at the edges (process vs relational position) even if they often co-vary. |
| **O2** — `detection:distributive:access` could fold into `detection:correlated_action` as axis path | Kept separate for pedagogical weight; possible future revisit. |
| **O3** — `credits:*:substrate_building` was miscounted as new prefix | CORRECTED — recounted as `{subject}` value. |
| **O4** — [§8.1](08_composition.md) reference policy structure (A/B/C base + D/E/F/G/H modifiers) | Cosmetic restructuring; documented inline. |

## §15.6 CEG 0.10 / RC1 — observer-share + streaming multicast (NORMATIVE-LANDED; streaming half substrate-pending)

**STATUS UPDATE 2026-06-03**: CEG 0.10 normatively landed on main (commit folds `DRAFT_0.10_delivery_axis.md` into [§4](04_envelope.md) / [§7.9](07_reserved.md) / [§8.1.13.7](08_composition.md) / [§10.5](10_endpoints.md) + version bump 0.9 → 0.10 + lineage). All §15.6.2 decisions ratified into normative spec text. RC1-1b ✅ confirmed (`KEY_GRANT_V1_INFO` at `CIRISVerify/src/ciris-crypto/src/key_grant.rs:71` as `b"cewp-key-grant/v1"`). Open coupling caveat RC1-1c (V054 parallel-CHECK migration) flagged in §10.5.3 normative text. RC1-7 (operational constants) flagged as TODO in §10.5.1 — operator-tunable; not blocking normative ship.

The full RC1 delivery-axis decision register (§15.6.1 bifurcation, §15.6.2 locked cross-team decisions D1–D6 / V1–V3 / P3–P4, §15.6.4 grounding corrections, §15.6.5 per-team status) is preserved in git history. Summary of what landed: the delivery axis bifurcated into an **observer-share half** (N=1; subscriber-set = `community` per [§8.1.13](08_composition.md) Policy M + per-subscriber `key_grant`; **ZERO remaining blockers, normative-ready**) and a **streaming-multicast half** (N>1; per-`(stream_id, epoch)` keys; **spec-now, impl substrate-pending** on [CIRISPersist#142](https://github.com/CIRISAI/CIRISPersist/issues/142) step 3 — unowned/unscheduled — with the accountable tier additionally on [CIRISRegistry#34](https://github.com/CIRISAI/CIRISRegistry/issues/34)). All cross-team decisions (Persist P1–P4, Verify V1–V3, Edge E1–E4, router RC1-2) are **✅ resolved/ratified** and folded into normative spec text ([§4](04_envelope.md) / [§7.9](07_reserved.md) / [§8.1.13.7](08_composition.md) / [§10.5](10_endpoints.md)). The `rotation_chain` hygiene corrections (it is the content-addressed grant-supersession lineage per [§5.6.8.4](05_namespace.md), NOT a key-rotation primitive; epoch rotation is greenfield per `stream_id`) are folded into [§5.6.8.4](05_namespace.md) / [§1.4](01_foundation.md) path-8 / [§11.7.1](11_governance.md) / [§5.6.8.9](05_namespace.md). [CIRISRegistry#44](https://github.com/CIRISAI/CIRISRegistry/issues/44) absorbed (closed as superseded).

**Remaining streaming-half items** (preserved verbatim; operator-tunable / substrate-coupled, not blocking the observer-share normative ship):

| OQ | Open item | Owner | Gating |
|---|---|---|---|
| **RC1-1b** | Confirm the `KEY_GRANT_V1_INFO` versioned-context HKDF pattern exists in `key_grant.rs` (the §10.5.2 V2 nonce-prefix derivation reuses it). Unverifiable from Edge. *(Still owed.)* | Persist | 🔴 V2 |
| **RC1-1c** | ⚠️ **Coupling caveat** — the V054 cross-column CHECK requires content-addressed `key_grant`s; the §10.5.3 epoch axis needs a **parallel CHECK arm** (content- OR stream/epoch-addressed) — a bounded constraint migration, **not a pure index-add**. Recorded so 0.10 doesn't claim "purely additive" at the Persist constraint layer. | Persist (@ #142) | flagged |
| **RC1-7** | Ratify constants (K=64 / T=2s / cosign per-epoch / `MAX_CHUNKS_PER_EPOCH=2²⁴`) + accountable-stream quorum = Policy E ([§8.1.5](08_composition.md) locality-scaled, not fixed N). | router | — |

## §15.7 Child-safety — fails-secure governance vs the shared detection limit (the honest line)

From the CIRISServer safety deep-dive (`FSD/MODERATION_CHILD_SAFETY.md` + `FSD/SAFETY_LANDSCAPE.md`, comparing 9 networks: Nostr, Matrix, Mastodon, Bluesky, IPFS, Signal, Briar, Session, SimpleX). Two axes, two honest verdicts — recorded here so the spec **carries its own honesty** and the detection limit can never be misrepresented as solved.

- **Governance — categorically stronger (a genuine first).** All 9 surveyed networks permit unmoderated multi-party spaces and **fail open**. CEG is the only model that **fails secure**: the [§11.11](11_governance.md) named-moderator existence invariant (a group cannot exist without an accountable moderator; merit auto-promotion so there is never a gap; `hard_case:community_unmoderated` quiesces it if none can be named), composed with the [§11.10](11_governance.md) delegable-accountable-signed-revocable duty, trust-propagation, and the [§11.1](11_governance.md) operational-language anti-censorship gate (public/voted/mechanically-checkable rules, deterministic verdicts, recused appeals). This is the categorical advance.
- **Detection — the same wall as everyone, and CEG says so.** CSAM in *truly-private* content (self/family, [§10.1.4](10_endpoints.md) E2EE-equivalent) is **unsolved across all E2EE systems** — Apple abandoned NeuralHash, the EU CSAR retreated (late 2025), US §2258A carries no scanning mandate. CEG **narrows** the surface to the share/publish seam + the still-visible metadata/coordination layer, and **declines client-side scanning** — which would itself be the censorship machinery [§1.6](01_foundation.md) / `ciris.ai/safety-vs-censorship` warns against. **CEG does NOT claim to solve private-content detection.** That honesty is load-bearing: the positioning is *"fails-secure governance + accountable, censorship-resistant moderation,"* never *"we detect CSAM in private content."*

This is an **acknowledged inherent limit, not a spec gap** — no CEG mechanism closes it without becoming the surveillance backdoor the framework exists to refuse. The CEG 1.0 moderation surface is **complete** ([#90](https://github.com/CIRISAI/CIRISRegistry/issues/90) ✅ RC19, [#93](https://github.com/CIRISAI/CIRISRegistry/issues/93) ✅ RC21); the remaining child-safety work is implementation (CIRISPersist#232 admission enforcement → CIRISServer `src/safety/*`), not spec.

---

[← §14 Glossaries](14_glossaries.md) | **§15 Gaps** | [Next: §16 References →](16_references.md)
