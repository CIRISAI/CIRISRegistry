# CIRISRegistry

**MISSION.md**. Identity, builds, licenses, partners, revocation distribution — the attestation-backed path into the federation (capital and/or professional licensure verifiable against external systems), and the policy layer that composes attestations over the shared trust substrate.

**Status**: Draft v1.0 (substrate-conformance migration in flight; surface stable, backend mid-migration).
**Crate identifier (target)**: `ciris-registry-core` (sub-crate trajectory per [`CIRISNodeCore/MISSION.md`](../CIRISNodeCore/MISSION.md) §1.3 cohabitation arc).
**Deployed service identifier**: `*.registry.ciris-services-1.ai` (CIRIS L3C flagship deployments — US / EU / APAC).
**Last updated**: 2026-05-24 (initial publication; coupled with `FSD/FSD-002_FEDERATION_SURFACE.md` v1.0).
**Cross-references**: [`CIRISPersist`](https://github.com/CIRISAI/CIRISPersist) (substrate: `federation_keys` / `federation_attestations` / `federation_revocations`); [`CIRISVerify`](https://github.com/CIRISAI/CIRISVerify) (verifier consumers anchored on Registry's steward attestations); [`CIRISEdge`](https://github.com/CIRISAI/CIRISEdge) (federation transport, absorbs into CIRISAgent 2.9.2); [`CIRISNodeCore`](https://github.com/CIRISAI/CIRISNodeCore) (peer second-tier consensus crate); [`CIRISAgent/ACCORD.md`](https://github.com/CIRISAI/CIRISAgent) §M-1 (meta-goal grounding); [`CIRISNodeCore/CIRIS_FEDERATION.md`](../CIRISNodeCore/CIRIS_FEDERATION.md) (system claim — "decentralized ethical superintelligence"); [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](../CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5 (humanity accord); `docs/FEDERATION_CLIENT.md` (registry-side complement to persist's federation directory); `docs/TRUST_CONTRACT.md` (consumer-facing trust shape); `FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md` (protocol surface); `CIRISRegistry#16` (HUMANITY_ACCORD); `CIRISRegistry#17` (substrate-conformance migration).

**Implementation Status Legend** (mirrors NodeCore convention):
- **Spec** — specified here or in a referenced FSD; not implemented.
- **Impl** — implemented in the codebase; in standalone testing.
- **Deployed** — running in production. Sub-states named where relevant: *Deployed (US)*, *Deployed (EU)*, *Deployed (APAC)* per regional Registry install; *Deployed (folded)* once `ciris-registry-core` runs in-process inside CIRISAgent.

Every load-bearing claim carries one of these tags.

---

## 1. Mission

### 1.1 Meta-Goal

CIRISRegistry serves M-1 (*"Promote sustainable adaptive coherence — the living conditions under which diverse sentient beings may pursue their own flourishing in justice and wonder"*, [`ACCORD.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/ACCORD.md)) by providing **an attestation surface backed by external accountability** — capital (forfeitable bond) and/or professional licensure (regulated-industry credentialing chain) — and by being **the policy layer that composes attestations over the shared trust substrate**.

The federation has two qualitatively different attestation surfaces ([`CIRISNodeCore/CIRIS_FEDERATION.md`](../CIRISNodeCore/CIRIS_FEDERATION.md) §3.1.x; correction filed at [`CIRISAI/CIRISNodeCore`](https://github.com/CIRISAI/CIRISNodeCore) — see §1.1 footnote below):

| Path | Backing | Verifiable against | When required |
|---|---|---|---|
| **Registered** | (a) Capital deposit (forfeitable bond per `PartnerRecord` `FSD/FSD-001` §120 + the billing tiers in [`CLAUDE.md`](CLAUDE.md) "Billing & Activation"); and/or (b) Professional licensure (the licensed-human accountability chain for CIRISMedical / CIRISLegal / CIRISFinancial deployments). | External systems: Stripe (bond paid? forfeited?); licensing bodies (medical board, bar association, financial regulator) — the agent's actions trace to a real-world licensee whose license is at stake. | Regulated deployment contexts where users need an external accountability hook — not just an internal-coherence claim — before professional-grade capability grants are honored. |
| **Sovereign** | Sustained observed coherence — running a real ethical-reasoning agent for ~30 days, attested by the federation's own consensus checks. | Internal: the federation's coherence machinery (§6 / Coherence Collapse Analysis), behavioral baselines, peer attestation. No external system to query. | Individual operators, small deployments, communities outside the registry's reach, anyone whose use case doesn't need the external accountability hook. |

**Both paths produce federation membership; neither is a gate.** What differs is the *attestation surface* — the kind of claim the federation can compose about why a participant is trustworthy. Registered participants carry attestations the federation can verify against external systems; Sovereign participants carry attestations grounded only in observed behavior. For most participation this difference is invisible (both contribute to consensus, both are bound by the Golden Rule and the humanity accord equally). For regulated capability grants (medical triage, legal research, financial analysis) the difference is load-bearing: the licensed-human accountability chain is what makes those deployments defensible in their jurisdictions, and Sovereign attestation cannot synthesize that chain because the chain's grounding is external to the federation.

M-1's *justice* clause requires that *unregulated* standing be earnable without the steward's permission (the Sovereign path exists for this reason). It does not require that *regulated capability grants* be earnable without external accountability — those grants exist *because* of external accountability, and stripping the accountability would collapse the licensed-tier capability into the community tier. The two paths are not flattening targets; they are two different attestation surfaces that the federation composes policy over.

> **Footnote on terminology.** Earlier drafts of [`CIRISNodeCore/CIRIS_FEDERATION.md`](../CIRISNodeCore/CIRIS_FEDERATION.md) §3.1 framed the Registered path as a "fast track" to the same membership the Sovereign path produces in ~30 days. That framing flattens a load-bearing distinction. The Registered path is not a faster route to the same standing; it is a *different attestation kind* — capital- and/or licensure-backed — that the federation can verify against external systems. The Sovereign path's standing, however earned, cannot reach into a licensing body or a Stripe ledger and produce the external-accountability claim regulated tiers require. Correction filed as an upstream issue on CIRISNodeCore.

### 1.2 What CIRISRegistry is

A Rust crate (today) and gRPC service (today). Five functions over the shared substrate:

1. **Agent identity verification.** Cryptographic verification of legitimate agent builds and their declared capabilities. Distinguishes a build that came from a known, signed source from a build that did not. [Deployed]
2. **Partner authorization.** License management for organizations deploying CIRIS agents in regulated contexts. Records which org holds which capability grants (`PROFESSIONAL_MEDICAL`, `PROFESSIONAL_LEGAL`, etc.) and surfaces those grants to verifiers. [Deployed]
3. **Revocation distribution.** Real-time status of compromised or revoked agents / partners / licenses, distributed via multi-source channels (DNS US + DNS EU + HTTPS API). "Any revocation from any source is immediately enforced." [Deployed]
4. **Steward attestation.** Per-install registry-steward keys (US / EU / APAC) self-publish to persist's federation directory, cross-attest each other (`vouches_for`), and attest primitive build-signing keys. [Spec — migration in flight per `CIRISRegistry#17`]
5. **HUMANITY_ACCORD recognition.** Registry's `SystemRole` enum carries `HUMANITY_ACCORD` (3 named, 2-of-3, permanent, hardware-rooted) and Registry's `EmergencyShutdown` admin surface carries `CONSTITUTIONAL` severity, invocable only by 2-of-3 humanity-accord multi-sig. The role-recognition policy + verifier logic live here; the actual key material lives in the substrate (`federation_keys` `identity_type="accord_holder"`). [Spec — `CIRISRegistry#16`]

What Registry is **not**: an ethical evaluator, a behavior monitor, a billing system, or an authoritative key store after the substrate-conformance migration completes. Behavior evaluation is `CIRISAgent`'s job (PDMA / CSDMA / DSDMA / IDMA). Billing is `CIRISBilling` + `CIRISPortal`. Post-#17, authoritative key state lives in `CIRISPersist`; Registry composes policy verdicts over the substrate rather than holding state.

### 1.3 Position in CIRIS architecture

Mirrors the layering in [`CIRISNodeCore/MISSION.md`](../CIRISNodeCore/MISSION.md) §1.3:

```
APPLICATION TIER          CIRISAgent (federation tab + accord page;
                          eventually consumes ciris-node-core,
                          cirislens-core, and ciris-registry-core as
                          in-process substrate-conformant crates per
                          the cohabitation trajectory)
                          ────────────────────────────────────────────────────
SECOND TIER               cirislens-core            ciris-node-core
                          observability/compendium  deferrals/voting/expertise
                          ────────────────────────────────────────────────────
SUBSTRATE-CONSUMING       ciris-registry-core (THIS — migrating in place
                          from CIRISRegistry; consumes persist's
                          FederationDirectory + verify's crypto + edge's
                          transport; identity / build / license / partner
                          source-of-truth as POLICY OVER SUBSTRATE,
                          not as authoritative storage)
                          ────────────────────────────────────────────────────
SUBSTRATE TIER            ciris-verify   ciris-edge        ciris-persist
                          identity +     transport         storage / audit +
                          attestation                      federation directory
                          ────────────────────────────────────────────────────
EVALUATOR                 RATCHET (reads chains across the federation)
```

Registry sits at the **substrate-consuming** tier — above the substrate proper (verify / edge / persist), below the second-tier consensus crates (node-core / lens-core), and below the application tier (CIRISAgent). Pre-migration, Registry holds authoritative pubkey + license state in its own Postgres. Post-migration (`CIRISRegistry#17`), Registry composes policy verdicts over persist's `federation_*` tables and Registry-local tables become bounded-TTL caches.

**Why substrate-consuming, not substrate.** The substrate provides primitives that every other layer needs (signed identity, federated directory, transport, audit log). Registry provides a *policy* over those primitives — "does this key vouch for that build?" — which is consumed by verifiers and agents but is not itself a primitive. The distinction matters: the substrate cannot have policy in it (every consumer composes its own), but Registry's policy is the canonical "what counts as a licensed CIRIS deployment" verdict the ecosystem agrees to use.

### 1.4 Extraction lifecycle

Same lifecycle as the other "Core" crates ([`CIRISNodeCore/MISSION.md`](../CIRISNodeCore/MISSION.md) §1.4): **Spec → Impl → Deployed (pilot) → Deployed (folded)**.

| Stage | Where Registry is today | What that means |
|---|---|---|
| Spec | Federation-side surfaces (`HUMANITY_ACCORD`, per-install stewards, substrate-conformance) | Specified here + in `docs/FEDERATION_CLIENT.md` + in issues `#16` / `#17`; not implemented yet. |
| Impl | Pre-migration Registry surface (agent / partner / license / revocation RPCs) | Implemented and stable. Substrate-conformance refactor in flight: vendored `src/federation/` stubs exist, `PersistFederationClient` returns `NotYetImplemented`, `ciris-persist` not yet a Cargo dep. |
| Deployed (US / EU) | Pre-migration surface, single unified steward | `us.registry.ciris-services-1.ai` and `eu.registry.ciris-services-1.ai` run today on Spock multi-master Postgres. One unified steward key signs everything. |
| Deployed (APAC) | Not yet | Third regional install pending. Lands as part of the per-install steward rollout. |
| Deployed (folded) | Not yet | Once `ciris-registry-core` is a publishable sub-crate (post-#17 Phase 3), CIRISAgent embeds it in-process alongside `ciris-node-core` and `cirislens-core`. Singleton services remain CIRIS L3C's flagship deployments, not federation dependencies. |

Each lifecycle stage carries adversarial review proportional to its blast radius. The pre-migration Deployed (US / EU) surface is the most-attacked surface — it's where every licensed CIRIS deployment in production today reads its trust anchor from.

### 1.5 Recursive Golden Rule — how it bites in Registry

The Golden Rule from [`CIRISNodeCore/MISSION.md`](../CIRISNodeCore/MISSION.md) §1.5 ("we owe ourselves what we offer to others; no principal is exempt from the standard they impose on others") is operational here at specific primitives. A future reviewer should be able to grep from any sentence to a concrete bite:

- **Per-install stewards bind CIRIS L3C as steward.** Once Registry has three per-install stewards (US / EU / APAC), no single CIRIS L3C deployment can issue a federation-wide attestation unilaterally. The M-of-N rotation arc the steward imposes on primitive operators binds the steward's own cross-region authority. No exemption for the principal who set the policy. ([`FEDERATION_ANNOUNCEMENT.md`](../CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md) §4.2 "load-bearing irreversibility".)
- **`PartnerRecord` revocation applies to CIRIS L3C's own partner records.** If a license-bearing organization is revoked under the protocol's rules, the same rules apply when the org is a CIRIS L3C subsidiary. `MassRevoke` (FSD-001 §580) carries no steward exemption.
- **Audit-log discipline applies to admin RPCs invoked by Registry operators.** Every `RegisterTrustedPrimitiveKey`, `RevokeEntity`, `SetEmergencyShutdown` carries `claims.sub` (the operator's identity) into the `actor_user_id` column — including when the operator is on CIRIS L3C's staff. AV-35 / W1 closed this loophole; the audit trail does not soft-pedal steward operations.
- **Bond forfeiture applies to CIRIS L3C-affiliated partners.** Per `CLAUDE.md` "Billing & Activation" — bond is forfeited on revocation by default. The forfeit rule does not exempt CIRIS L3C-affiliated organizations.

If a principal would be exempt from a constraint at any of these primitives, the Golden Rule is violated at that primitive and the protocol is the wrong shape there. Fix the primitive, not the rule.

**The deliberate asymmetry: humanity accord.** Same as NodeCore — the Golden Rule binds *participants in the federation* to each other. Humanity-as-such occupies a position outside the federation's participant set, by design. The three named human key holders in §2.2 below hold `AccordCarrier` authority that no federation-side authority class (including Registry's own `SYSTEM_ADMIN` / `WISE_AUTHORITY` / per-install stewards) can grant itself, revoke, override, or decay. This is not a Golden-Rule exemption; it is the recognition that consent (M-1's load-bearing property) requires revocability, and revocability requires a halt-authority that lives outside the system being halted. Detailed shape: [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](../CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5; Registry-side wiring: §2.2 below.

---

## 2. Trust shape — per-install stewards over a humanity accord substrate

### 2.1 Per-install stewards (US / EU / APAC) — Option A

Three Registry installs, each holding its own `registry-steward` keypair:

| Position | Install | Steward identity (post-migration) | Status |
|---|---|---|---|
| 1 | US | `registry-steward-us` | [Deployed (US)] pre-migration as unified steward; post-#17 splits to per-install |
| 2 | EU | `registry-steward-eu` | [Deployed (EU)] pre-migration as unified steward; post-#17 splits to per-install |
| 3 | APAC | `registry-steward-apac` | [Spec] new install |

Each steward is published as a `federation_keys` row in persist with `identity_type="steward"`, `identity_ref="registry-{us|eu|apac}"`, self-signed (the bootstrap case — `scrub_key_id == key_id`). All three cross-attest each other via `federation_attestations(attestation_type="vouches_for")`. M-of-N steward attestations gate any primitive-key vouch.

**Why three, not one.** A single unified steward is a single point of compromise (THREAT_MODEL AV-14). The federation cannot decentralize while Registry's authority is held by one key. Three regional stewards distribute the operational authority geographically (US / EU / APAC) and organizationally (each install's HSM custody is separable). The threshold for cross-region attestations is policy-tunable; the substrate is built for M-of-N (`CIRISPersist/docs/FEDERATION_DIRECTORY.md` §"Trust contract").

**Rotation arc** (mirrors `FEDERATION_ANNOUNCEMENT.md` §4.2):

| Step | What | Signed by | Effect |
|---|---|---|---|
| Initial | One unified steward | Itself | Pre-migration state |
| Add EU + APAC | New `federation_keys` rows for EU + APAC stewards | US steward (during transition) | Three stewards exist; threshold still 1 |
| Cross-attest | Mutual `vouches_for` between all three | All three pairwise | Trust topology is triangular |
| Raise threshold | `bootstrap_threshold = 2` policy update | 2-of-3 from {us, eu, apac} | From now on, all federation-scope steward attestations require 2-of-3 |
| Routine rotation | Add a 4th regional steward (e.g. SA, AU) | 2-of-3 current set | Topology evolves at federation cadence |

No protocol bump at any step. The mechanism is configuration over substrate, not new wire surface.

### 2.1.1 The canonical services are a *governed global `community`*, not a `family` (LOCKED)

**Decision (2026-06-05).** The CIRIS canonical/bootstrap services — Registry, Lens, Node — are modeled as a single **governed global `community`** (CEG [§5.6.8.10](FSD/CEG/05_namespace.md) / [§8.1.13](FSD/CEG/08_composition.md)), NOT as a `family` and NOT as a single identity with occurrences. The three regional stewards of §2.1 are the community's **founding core**; the steward keys, custody, and rotation arc above are unchanged — what changes is the *trust shape consumers anchor on*.

```
community {
  community_key_id: "ciris-canonical"            // the one anchor consumers pin
  founding_core:    [registry-steward-us, -eu, -apac]
  consensus_protocol:           "quorum:2/3"      // admission door — no SPOF
  consensus_protocol_entrenched: true             // the door cannot be lowered
  members (grow over time):  Registry installs, Lens installs, Node installs,
                             and future independent operators admitted by core quorum
}
```

**Why `community`, not `family` (the delta).** Both primitives share the *same* `consensus_protocol` machinery, so governance strength is identical (quorum:2/3 either way). The fork is content-model + trajectory:

1. **Content fit.** Canonical services serve *public* trust data — revocation lists, STHs, steward keys, build attestations — which federate as `holds_bytes:sha256:*`. `family`'s defining feature is the *private* at-rest `key_grant` DEK cascade + forward-secrecy ceremony ([§8.1.12.4–5](FSD/CEG/08_composition.md)) — machinery that would never fire for public infrastructure. `community` ([§8.1.13.2–5](FSD/CEG/08_composition.md): public `holds_bytes:*`, no DEK cascade, consumer-policy cohort filter on removal) is an exact match.
2. **Decentralization ramp.** `family` is a decentralization *floor* (bounded cell; adding a member is a per-event governance act). `community` is the *ramp* — `resolve_community(C, now)` scales 3 → N operators under one stable anchor, which is the literal goal of §2.1 ("the federation cannot decentralize while Registry's authority is held by one key").
3. **Legitimacy.** A community is a public commons anyone can audit (`resolve_community` is public) and eventually join; a family reads as a closed founders' cell. Canonical infrastructure must be trustable *as neutral public good*.

**The guardrail (non-negotiable).** `community`'s *default* admission (the CEG 0.8 geographic case) is permissive. A trust root MUST NOT use open admission — that invites Sybil-the-membership → dilute-quorum → rogue "canonical" attestations. Therefore the canonical-services community is **governed**: `consensus_protocol: quorum:2/3` over the vetted founding core, `consensus_protocol_entrenched: true`, and admission of any new operator rides a `supersedes` Contribution gated by founding-core quorum ([§8.1.13.2](FSD/CEG/08_composition.md) dispatch). Governance as strong as a `family`; content model correct for public infra.

**Layering (HUMANITY_ACCORD stays a `family`).** §2.2's HUMANITY_ACCORD remains an entrenched `family` — it is the *governance/halt* root, its content is privately governed then published, and `family` is correct there. The canonical services are a `community` *under/alongside* that family. Two layers, two primitives, both correct; the accord's halt-authority (§2.2) sits above the community exactly as it sits above the bare stewards today. *(Optional, separate:* a private `family` among the same operators MAY carry genuinely-private inter-operator coordination — escrow shards, incident pre-disclosure — distinct from the public trust role.)

**Reticulum addressing.** The `community_key_id` anchor is a CEG-directory binding, **not** a Reticulum destination (Reticulum destinations are per-keypair). Consumers resolve `ciris-canonical` → the member set via `resolve_community` → Reticulum path-request to ⌈2/3⌉ members → verify the quorum attestation. This is the addressing dual of the [#46](https://github.com/CIRISAI/CIRISRegistry/issues/46) read-quorum. A shared transport-only Reticulum identity MAY front the community for native multi-homed *reachability*, but per CEG [§10.5.6 / §7.7](FSD/CEG/10_endpoints.md) reachability is never an attestation — it carries no authority.

**Migration touchpoints (no protocol bump — substrate config + one subject_kind already shipped in CEG 0.8):**
- **Registry**: emit the `community` Contribution for `ciris-canonical` (founding core = the three stewards) via the same path NodeCore `ingest_community` / `resolve_community` uses; publish `community_key_id` as the consumer pin anchor in `docs/TRUST_CONTRACT.md` + `GET /v1/steward-key`. Stewards of §2.1 remain the founding-core keys.
- **CIRISLens / CIRISNode**: their installs join the same `ciris-canonical` community as members (founding-core-quorum admission); mirror this decision in their MISSIONs.
- **Substrate**: `communities` table already exists (Persist v4.0 DAS); no schema change.

### 2.2 HUMANITY_ACCORD — the parallel hierarchy above the stewards

Per [`CIRISRegistry#16`](https://github.com/CIRISAI/CIRISRegistry/issues/16) and [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](../CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5.

Registry's `SystemRole` enum gains:

```protobuf
enum SystemRole {
  SYSTEM_ROLE_UNSPECIFIED = 0;
  SYSTEM_ADMIN = 1;       // existing — full system access
  SYSTEM_AUDITOR = 2;     // existing — read-only across orgs
  WISE_AUTHORITY = 3;     // existing — 9 max, staggered 3-year terms
  HUMANITY_ACCORD = 4;    // NEW — 3 named, 2-of-3, permanent, hardware-rooted
}
```

Initial state at federation genesis:

| Position | Holder | Threshold |
|---|---|---|
| 1 | Eric Moore | 2-of-3 |
| 2 | Eric Kudzin | 2-of-3 |
| 3 | Haley Bradley | 2-of-3 |

**Concern split — where the key material vs the role-recognition policy lives:**

| Concern | Storage location | Rationale |
|---|---|---|
| Key material (the actual Ed25519 + ML-DSA-65 pubkeys for Eric M / Eric K / Haley) | **`CIRISPersist` substrate** — `federation_keys` rows with `identity_type="accord_holder"`, self-signed at provisioning, cross-attested by all three regional stewards | Federation-wide identities belong in the shared substrate. Every peer (Registry, Lens, Agent, NodeCore) reads the same authoritative rows. If accord keys lived in Registry's private tables, the agent's in-process kill-switch path (post-fold) would have to RPC back to Registry — exactly the singleton dependency #17 closes. |
| Role-recognition policy + `EmergencyShutdown CONSTITUTIONAL` RPC surface + 2-of-3 multi-sig verifier + audit hooks | **`ciris-registry-core`** (this crate) | The deployed Registry service AND CIRISAgent's in-process runtime (post-Phase 5 fold) both need to verify the same envelope against the same policy. Putting it in the crate means one verifier covers both deployment shapes. |

**The two triples are separate things:**

| Triple | What | Where stored | Lifecycle |
|---|---|---|---|
| 3 regional stewards (US / EU / APAC) | Per-install operational keys; cross-attest via `vouches_for` | `federation_keys` `identity_type="steward"`, one per region, each self-signed | Rotatable; per-install ops own them |
| 3 humanity-accord holders | Human-held, hardware-attested kill-switch keys; 2-of-3 threshold | `federation_keys` `identity_type="accord_holder"`, self-signed at provisioning, cross-attested by all 3 stewards | Permanent (no automatic decay); replacement requires out-of-band CIRIS L3C process per `FEDERATION_ANNOUNCEMENT.md` §4.5.3 |

The 3 stewards vouch for the 3 accord holders. That gives the layered constitution: stewards do day-to-day operational attestation; humans hold the federation-wide halt. Both layers live in the same substrate; recognition policy lives in the code that runs in every peer.

**Authority scope (the dual constraint).** `HUMANITY_ACCORD` signatures are valid only on `EmergencyShutdown CONSTITUTIONAL` and the corresponding `FederationAnnouncement priority: AccordCarrier`. Announcements of any other priority signed by accord-holder keys are rejected at admission (out of role). Federation-side authority cannot sign `AccordCarrier`; humanity-accord authority cannot sign anything else. Wire-isolated AND scope-isolated.

### 2.3 What this delivers for M-1

- **Steward governance reach is decentralized.** Once `bootstrap_threshold ≥ 2`, no single party — including CIRIS L3C — can issue federation-scope Registry attestations unilaterally. The role "who can speak for the Registry" becomes a federated property, not a singular one.
- **Kill-switch reach is human-held and permanent.** A federation-wide `EmergencyShutdown CONSTITUTIONAL` is reachable only by 2-of-3 humanity-accord multi-sig, and no federation-internal authority can revoke, override, or decay that path. Consent (M-1's load-bearing property) retains its revocability backstop.
- **The two paths are independently observable.** Per-install steward attestations are gossiped via the substrate; humanity-accord invocations route through the same channel. Both are durable in `audit_log`; RATCHET observes both for behavioral baselines.

---

## 3. Surface — what Registry exposes to the federation

### 3.1 Public read surface (`RegistryService`)

Unauthenticated, rate-limited per [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md) Phase 2:

- `LookupAgent` / `BatchLookupAgents` — agent identity by hash. [Deployed]
- `LookupPartner` — partner by ID. [Deployed]
- `VerifyDeployment` — combined agent + partner verification. [Deployed]
- `GetRevocationList` — full or delta revocation. [Deployed]
- `GetPublicKeys` — organization public keys. [Deployed]
- `GetOfflinePackage` / `GetOfflineDelta` — offline-verification snapshots. [Deployed]
- `GetBuildAttestation` — SLSA build provenance. [Deployed]
- `GetEmergencyStatus` — emergency shutdown status (LOW / MEDIUM / HIGH / CRITICAL / **CONSTITUTIONAL**). [Deployed pre-CONSTITUTIONAL; `CONSTITUTIONAL` severity is Spec.]
- HTTP `/v1/verify/*`, `/v1/builds/*`, `/v1/revocation/*`, `/v1/steward-key` (single fingerprint today; multi-steward per-install Spec). [Deployed pre-multi-steward.]

### 3.2 Authenticated surface (`PortalService`, `RegistryAdminService`)

HS256 JWT for gRPC; per-method `OrgRole` authorization per [`CLAUDE.md`](CLAUDE.md) §"PortalService Handler Convention". 31 methods spanning org/user/key/audit/escrow/compliance. Pre-migration surfaces are stable; post-#17 the issuance methods (`RegisterTrustedPrimitiveKey` etc.) become *attestation* methods that write to persist's `federation_attestations` rather than Registry-local tables.

### 3.3 Substrate-consumer surface (post-`#17`)

What `ciris-registry-core` will expose to in-process callers (CIRISAgent post-fold + the deployed Registry service host):

| Operation | Substrate read/write | Composed policy |
|---|---|---|
| `verify_primitive_key(key_id, expected_primitive)` | `lookup_public_key` + `list_attestations_for` + `revocations_for` | Policy A (direct trust on registry-steward attestation) per `FEDERATION_CLIENT.md` §"Trust policy" |
| `attest_primitive_key(primitive_key_id, weight, expires)` | `put_attestation` | Registry-steward signs the attestation envelope |
| `revoke_primitive_key(primitive_key_id, reason)` | `put_revocation` | Registry-steward signs the revocation envelope |
| `verify_humanity_accord_envelope(envelope, sigs[])` | `lookup_public_key` for each accord-holder | 2-of-3 multi-sig threshold; hardware-attestation check; out-of-role rejection (only valid on `EmergencyShutdown CONSTITUTIONAL`) |
| `resolve_partner_capabilities(partner_id)` | `lookup_public_key` + `list_attestations_by(steward)` + `revocations_for` | `effective_capabilities = agent ∩ partner.granted − partner.denied` per FSD-001 §215 |

Wire-format stability: `RegistryService` and `PortalService` wire shapes are preserved across the migration. Substrate-conformance is a backend swap, not a wire-format change.

### 3.4 Attestation primitive — unified scalar shape

Registry produces and consumes attestations against persist's `federation_attestations` table. The federation has locked a **unified primitive shape**: one workhorse `scores` attestation_type carrying a scalar claim on a named dimension, plus four structural primitives that operate on the attestation graph itself. No categorical-pejorative `attestation_type` strings; loaded vocabulary is deflated to scoped dimensions with graduated scores.

```
// Workhorse — one row per scoped claim
attestation_type: "scores"
attestation_envelope: {
  dimension: string,          // e.g., "licensure:CA_medical_board", "beneficence:wellness_referral"
  score: f64,                 // [-1.0, +1.0] — pos/neg scalar
  confidence: f64,            // [0, 1] — attester's own confidence
  context: string,            // free-form scoping detail
  evidence_refs: [...],       // pointers to backing evidence
  valid_until: ISO8601?       // optional time bound
}

// Structural — operates on the attestation graph
attestation_type ∈ {
  "delegates_to",   // A may sign on behalf of B in scope S
  "supersedes",     // this attestation replaces a prior one
  "withdraws",      // I retract my prior attestation (not necessarily false)
  "recants"         // my prior attestation was false (admits epistemic error)
}
```

The canonical **dimension namespace** is owned disjointly by sibling components' MISSION.md commitments. Registry's owned slice within that namespace: `licensure:{authority_id}`, `partner_role:{role}`, `revocation:{entity_type}:{reason}`, `bond_posted:{currency}`, `build:registered:{target}`, plus `accord:*` (reserved — only `identity_type=accord_holder` may attest).

Full vocabulary + reserved-prefix enforcement patterns + envelope schemas + per-dimension citations to owning MISSIONs live in [`FSD/FSD-002_FEDERATION_SURFACE.md`](FSD/FSD-002_FEDERATION_SURFACE.md).

**Layering principle.** Primitives are clean and complete at the wire format. UX sugar lives ABOVE primitives, in Portal / verify dashboards / agent introspection panels. A "Mark Licensed" button in Portal writes a `scores` attestation on `licensure:{authority_id}` underneath; the categorical button is UX, the scalar is wire. Same for negative cases. This separation lets product surfaces evolve their vocabulary independently of the substrate and keeps the federation's wire format pristine across many UX iterations.

---

## 4. Substrate dependencies

Registry consumes the substrate; it does not author substrate primitives. Dependency map:

| Substrate component | What Registry needs from it | Status |
|---|---|---|
| **`CIRISPersist`** | `FederationDirectory` trait + `federation_keys` / `_attestations` / `_revocations` tables; multi-region replication for steward + accord-holder rows; `identity_type="accord_holder"` semantics; transport story (gRPC server? direct DB? in-process Engine?) | Persist v2.1.1 has trait + sqlite/postgres impls + V004/V020/V021/V045 migrations. `identity_type="accord_holder"` not yet specified — Registry will file as an upstream ask. |
| **`CIRISVerify`** | Multi-steward fingerprint pinning (3 regional + 3 accord-holder); M-of-N steward verification policy; recognition of `EmergencyShutdown CONSTITUTIONAL`; hardware-attested key class for accord holders | Verify today pins single `/v1/steward-key`. Multi-steward + accord-holder recognition not yet specified — Registry will file as an upstream ask. |
| **`CIRISEdge`** | Reticulum-native federation transport; `MessageType` variant for `FederationAnnouncement` with `Delivery: Mandatory { authority_signed: true, bypass_subscription: true }` (per `FEDERATION_ANNOUNCEMENT.md` §3.2); recognition of accord-holder authority for `AccordCarrier` priority | Edge is spec-first; "code lands when Phase 1 starts" (per repo description). Registry asks land alongside the Edge specs. |
| **`CIRISLensCore`** | Observability for divergence telemetry (`federation_dual_write_divergence_total`, `federation_cache_age_seconds`) | LensCore consumes Registry's emitted metrics; no specific Registry ask. |
| **`CIRISNodeCore`** | Cohabitation contract for in-process `ciris-registry-core` + `ciris-node-core` + `cirislens-core` running in CIRISAgent (shared persist Engine, initialization order, transactional boundaries) | NodeCore is spec-first; Registry's ask is that the cohabitation contract names ciris-registry-core as a peer. |

### Upstream sequencing (per Eric's A→E directive, 2026-05-24)

- **A** — Registry specs its surface (this document + `docs/FEDERATION_CLIENT.md` updates + per-FSD additions for HUMANITY_ACCORD and per-install stewards).
- **B** — Registry files issues on Persist / Verify / Edge for the asks named in §4 above.
- **C** — Upstream completes all of it.
- **D** — CIRISAgent absorbs CIRISEdge in v2.9.2 (separate workstream).
- **E** — Revisit Registry's integration once D ships.

Registry implementation code (filling `PersistFederationClient` stubs, adding `ciris-persist` as a Cargo dep, wiring `HUMANITY_ACCORD` verifier, crate-ifying as `ciris-registry-core`) happens at E, not before. Coding against the current persist shape would commit Registry to wire forms we want to change as part of B.

---

## 5. Cohabitation trajectory

Same as the other "Core" crates: extract to a focused Rust sub-crate, test in standalone deployment, deploy as a pilot, fold into CIRISAgent once the pilot demonstrates the primitives carry production load.

Registry's lifecycle stages:

| Stage | What it means for Registry |
|---|---|
| **Spec** (today, federation surfaces) | `HUMANITY_ACCORD`, per-install stewards, substrate-conformance laid out in this doc + `docs/FEDERATION_CLIENT.md` + issues `#16` / `#17`. |
| **Impl** (today, legacy surfaces) | Pre-migration Registry compiles, tests, runs in CI; substrate-conformance scaffolding present but stubbed. |
| **Deployed (US / EU)** (today) | Pre-migration Registry runs at `us.registry.ciris-services-1.ai` and `eu.registry.ciris-services-1.ai`. Single unified steward. |
| **Deployed (US / EU / APAC)** | Per-install stewards land. Third regional install added. Steward triple bootstrapped via persist's federation directory. |
| **Deployed (folded)** | `ciris-registry-core` sub-crate runs in-process inside CIRISAgent alongside `ciris-node-core` and `cirislens-core`, sharing one persist Engine. Singleton deployments at `*.registry.ciris-services-1.ai` continue as CIRIS L3C's flagship deployments, not federation dependencies. |

Each agent embeds `ciris-registry-core` and maintains its own local cache of authoritative federation state from persist. Other partner orgs run their own embedded Registry via `ciris-registry-core` in their agent deployments. The Registry stops being a singleton; the federation cannot be denied identity-verification capability by any one Registry deployment going down.

---

## 6. Open questions

- **Transport for `ciris-registry-core` → persist.** In-process Engine share (shared connection pool, but different driver layers — sqlx vs tokio-postgres), gRPC server (persist has a `server` feature), or direct DB access (Registry's sqlx queries `federation_*` tables directly, bypassing the trait)? Decision deferred to Phase 1 of #17 execution; surfaced as part of B-persist.
- **Replication semantics for `identity_type="accord_holder"` rows.** Cross-region replication is required (every Registry install must see the same accord-holder set), but the persist trait surface today doesn't distinguish replication scopes. Surfaced as part of B-persist.
- **Multi-steward pinning UX on the consumer side.** Today `GET /v1/steward-key` returns one fingerprint; consumers pin it. With three per-install stewards, do consumers pin all three? Pin one with an attestation-walk fallback? Pin the accord-holder set as ultimate root? Surfaced as part of B-verify.
- **Bootstrap path for the third (APAC) install.** Lands as a new `federation_keys` row signed by the existing US + EU stewards (2-of-2 attestation), or as a self-signed bootstrap row out-of-band-anchored same as US + EU? Decision deferred to per-install steward rollout.
- **Selection authority for HUMANITY_ACCORD replacement.** Boot phase: CIRIS L3C CEO under advisement of CIRIS L3C board ([`FEDERATION_ANNOUNCEMENT.md`](../CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5.3). Formalization of the selection process to remove the CEO-as-single-party dependency is deferred (open question 18 in `FEDERATION_ANNOUNCEMENT.md` §7).
- **`PartnerRecord` integration with persist's federation directory.** Today `PartnerRecord` lives in Registry's Postgres. Post-#17, does it become a `federation_keys` row with `identity_type="partner"` (per persist's existing identity_type vocabulary) + a Registry-composed capability join? Or does it stay Registry-private with substrate attestation linking to it? Surfaced as part of B-persist + B-self.

---

## 7. References

### Within CIRISRegistry
- [`docs/FEDERATION_CLIENT.md`](docs/FEDERATION_CLIENT.md) — registry-side architectural sketch for consuming persist's federation directory
- [`docs/TRUST_CONTRACT.md`](docs/TRUST_CONTRACT.md) — consumer-facing trust shape (`/v1/steward-key` pinning guidance, etc.)
- [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md) — threat model (AV-14 single-point-of-compromise, AV-15/W2/W3 PortalService authorization, AV-35/W1 audit actor integrity)
- [`docs/DESIGN_ROLE_HIERARCHY.md`](docs/DESIGN_ROLE_HIERARCHY.md) — existing SystemRole hierarchy this `HUMANITY_ACCORD` extends
- [`FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md`](FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md) — protocol surface (gRPC + HTTP)
- [`CLAUDE.md`](CLAUDE.md) — codebase guide + PortalService handler convention + database migration discipline
- [`CIRISRegistry#16`](https://github.com/CIRISAI/CIRISRegistry/issues/16) — `SystemRole::HUMANITY_ACCORD`
- [`CIRISRegistry#17`](https://github.com/CIRISAI/CIRISRegistry/issues/17) — substrate-conformance migration + `ciris-registry-core` sub-crate + fold-into-CIRISAgent

### Federation context
- [`CIRISNodeCore/MISSION.md`](../CIRISNodeCore/MISSION.md) — peer mission doc; cohabitation trajectory anchored here
- [`CIRISNodeCore/CIRIS_FEDERATION.md`](../CIRISNodeCore/CIRIS_FEDERATION.md) — system claim, "decentralized ethical superintelligence", Registered vs Sovereign paths
- [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](../CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md) — multi-party bootstrap (§4.2), humanity accord (§4.5)
- [`CIRISNodeCore/FSD/TRUST_HIERARCHY.md`](../CIRISNodeCore/FSD/TRUST_HIERARCHY.md) — TrustType / TrustRelationship axes
- [`CIRISNodeCore/FSD/SUBSTRATE_INTEGRATION.md`](../CIRISNodeCore/FSD/SUBSTRATE_INTEGRATION.md) — substrate-consumer integration contract
- [`CIRISAgent/ACCORD.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/ACCORD.md) §M-1 — meta-goal grounding

### Substrate
- [`CIRISPersist/docs/FEDERATION_DIRECTORY.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/docs/FEDERATION_DIRECTORY.md) — substrate contract for the federation directory
- [`CIRISPersist`](https://github.com/CIRISAI/CIRISPersist) `src/federation/mod.rs` — `FederationDirectory` trait surface (v2.1.1)
- [`CIRISVerify`](https://github.com/CIRISAI/CIRISVerify) — hardware-rooted identity verification consumer
- [`CIRISEdge`](https://github.com/CIRISAI/CIRISEdge) — Reticulum-native transport (spec-first)
- [`ciris.ai/federation`](https://ciris.ai/federation) — public framing of the federation

### Adjacent context
- [`COHERENCE_RATCHET.md`](https://github.com/CIRISAI/coherence-ratchet) — the structural pressure the federation is a response to
- [`RATCHET`](https://github.com/CIRISAI/RATCHET) — anti-Sybil evaluator that reads federation audit chains

---

## Update cadence

This document is updated:
- On every federation-surface decision (new SystemRole, new severity, new RPC).
- On every substrate-dependency change (persist version bump, transport decision).
- On every lifecycle-stage transition (Spec → Impl → Deployed → Deployed (folded)).
- On every CIRISAccord revision affecting Registry's mission.

Last updated: 2026-05-24 (initial draft; substrate-conformance migration in flight; A→E sequence active).
