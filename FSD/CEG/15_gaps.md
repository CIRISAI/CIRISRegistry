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
| 0.1-CRIT canonical-bytes newline-injection | **SCAFFOLDED** in [§5.2.1](05_namespace.md) with 0.2 redesign committed (TupleHash128 + domain-separation labels) |
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

## §15.3 First-adopter exposures (no prior validation; explicit bets)

| Exposure | Why no precedent |
|---|---|
| **F1** — Earned-Credits federation governance at scale | No prior system separates earned standing from purchasable token at scale. Risk: SPKI/SDSI adoption-gap failure mode. Mitigation: licensure forcing function. |
| **F2** — Ubuntu substrate as wire-format substrate | CARE Principles + African philosophy exist as ethical frameworks; never as protocol substrate. First-adopter risk on how the discipline interacts with engineering trade-offs at scale. |

## §15.4 Deferred to 0.2+ design workshops

| Item | Why deferred |
|---|---|
| Canonical-bytes redesign (TupleHash128 + domain-separation labels) | Phase B 0.2 commitment per CEG 0.1 review |
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

---

[← §14 Glossaries](14_glossaries.md) | **§15 Gaps** | [Next: §16 References →](16_references.md)
