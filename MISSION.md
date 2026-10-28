# CIRISRegistry

**MISSION.md**. Identity, builds, licenses, partners, revocation distribution — the attestation-backed path into the federation (capital and/or professional licensure verifiable against external systems), and the policy layer that composes attestations over the shared trust substrate.

**Crate identifier**: `ciris-registry-core` (composed into CIRISServer as the registry slice; also built as the standalone `ciris-registry` binary).
**Deployed service identifier**: `*.registry.ciris-services-1.ai` (CIRIS L3C flagship deployments, US and EU).
**Surface specification**: `FSD/FSD-004_POST_FOLD_SURFACE.md` (every route and RPC, its caller tier and node posture).
**Cross-references**: [`CIRISPersist`](https://github.com/CIRISAI/CIRISPersist) (substrate: `federation_keys` / `federation_attestations` / `federation_revocations`); [`CIRISVerify`](https://github.com/CIRISAI/CIRISVerify) (verifier consumers anchored on the trust root the registry serves); [`CIRISEdge`](https://github.com/CIRISAI/CIRISEdge) (federation transport); [`CIRISNodeCore`](https://github.com/CIRISAI/CIRISNodeCore) (peer second-tier consensus crate); [`CIRISAgent/ACCORD.md`](https://github.com/CIRISAI/CIRISAgent) §M-1 (meta-goal grounding); [`CIRISNodeCore/CIRIS_FEDERATION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/CIRIS_FEDERATION.md) (system claim — "decentralized ethical superintelligence"); [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5 (humanity accord); `docs/FEDERATION_CLIENT.md` (registry-side complement to persist's federation directory); `docs/TRUST_CONTRACT.md` (consumer-facing trust shape); `FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md` (protocol surface).

**Implementation Status Legend** (mirrors NodeCore convention):
- **Spec** — specified here or in a referenced FSD; not implemented.
- **Impl** — implemented in the codebase; in standalone testing.
- **Deployed** — running in production. Sub-states named where relevant: *Deployed (US)*, *Deployed (EU)* per regional install; *Deployed (folded)* for the registry slice running inside CIRISServer.

Every load-bearing claim carries one of these tags.

---

## 1. Mission

### 1.1 Meta-Goal

CIRISRegistry serves M-1 (*"Promote sustainable adaptive coherence — the living conditions under which diverse sentient beings may pursue their own flourishing in justice and wonder"*, [`ACCORD.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/ACCORD.md)) by providing **an attestation surface backed by external accountability** — capital (forfeitable bond) and/or professional licensure (regulated-industry credentialing chain) — and by being **the policy layer that composes attestations over the shared trust substrate**.

The federation has two qualitatively different attestation surfaces ([`CIRISNodeCore/CIRIS_FEDERATION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/CIRIS_FEDERATION.md) §3.1.x; see the terminology footnote below):

| Path | Backing | Verifiable against | When required |
|---|---|---|---|
| **Registered** | (a) Capital deposit (forfeitable bond per `PartnerRecord` `FSD/FSD-001` §120 + the billing tiers in [`CLAUDE.md`](CLAUDE.md) "Billing & Activation"); and/or (b) Professional licensure (the licensed-human accountability chain for CIRISMedical / CIRISLegal / CIRISFinancial deployments). | External systems: Stripe (bond paid? forfeited?); licensing bodies (medical board, bar association, financial regulator) — the agent's actions trace to a real-world licensee whose license is at stake. | Regulated deployment contexts where users need an external accountability hook — not just an internal-coherence claim — before professional-grade capability grants are honored. |
| **Sovereign** | Sustained observed coherence — running a real ethical-reasoning agent for ~30 days, attested by the federation's own consensus checks. | Internal: the federation's coherence machinery (§6 / Coherence Collapse Analysis), behavioral baselines, peer attestation. No external system to query. | Individual operators, small deployments, communities outside the registry's reach, anyone whose use case doesn't need the external accountability hook. |

**Both paths produce federation membership; neither is a gate.** What differs is the *attestation surface* — the kind of claim the federation can compose about why a participant is trustworthy. Registered participants carry attestations the federation can verify against external systems; Sovereign participants carry attestations grounded only in observed behavior. For most participation this difference is invisible (both contribute to consensus, both are bound by the Golden Rule and the humanity accord equally). For regulated capability grants (medical triage, legal research, financial analysis) the difference is load-bearing: the licensed-human accountability chain is what makes those deployments defensible in their jurisdictions, and Sovereign attestation cannot synthesize that chain because the chain's grounding is external to the federation.

M-1's *justice* clause requires that *unregulated* standing be earnable without money and without the steward's permission — the Sovereign path is the non-capital answer to identity-as-wealth-gate, and exists for this reason. It does not require that *regulated capability grants* be earnable without external accountability — those grants exist *because* of external accountability, and stripping the accountability would collapse the licensed-tier capability into the community tier. The two paths are not flattening targets; they are two different attestation surfaces that the federation composes policy over.

> **Footnote on terminology.** Framing the Registered path as a "fast track" to the same membership the Sovereign path produces in ~30 days flattens a load-bearing distinction. The Registered path is not a faster route to the same standing; it is a *different attestation kind* — capital- and/or licensure-backed — that the federation can verify against external systems. The Sovereign path's standing, however earned, cannot reach into a licensing body or a Stripe ledger and produce the external-accountability claim regulated tiers require.

### 1.2 What CIRISRegistry is

A Rust crate and the services built from it. Five functions over the shared substrate:

1. **Agent identity verification.** Cryptographic verification of legitimate agent builds and their declared capabilities. Distinguishes a build that came from a known, signed source from a build that did not. [Deployed]
2. **Partner authorization.** License management for organizations deploying CIRIS agents in regulated contexts. Records which organisation holds which licensure under which authority and surfaces it to verifiers (`GET /v1/partner/{key_id}`: `licensure[]` keyed by the authority's org UUID, status `issued` / `suspended` / `revoked`). [Deployed]
3. **Revocation distribution.** Real-time status of compromised or revoked agents / partners / licenses, distributed via multi-source channels (DNS US + DNS EU + HTTPS API). "Any revocation from any source is immediately enforced." [Deployed]
4. **Trust root and build provenance.** The registry serves the genesis trust-root bundle (`GET /v1/steward-key`, `GET /v1/trust-root/bundle`). The registry slice admits and serves build manifests signed by pipeline keys blessed in the federation directory (`/v1/builds*`); the standalone binary serves its build table with the provenance block composed from the directory. Steward keys are human-rooted and conferred by accord co-scrub; the regional installs hold standing as infrastructure and attest nothing on their own authority (§2.1). [Deployed; the conferral ceremony runs in CIRISServer]
5. **HUMANITY_ACCORD recognition.** The registry's `SystemRole` enum carries `HUMANITY_ACCORD` (3 named, 2-of-3, permanent, hardware-rooted) and its `EmergencyShutdown` surface carries the `CONSTITUTIONAL` severity, invocable only by 2-of-3 humanity-accord multi-sig. The accord-holder roster and custody evidence are carried in the genesis bundle the registry serves; the halt itself is served by CIRISServer's `POST /v1/accord/halt` under accord quorum (`FSD/FSD-004_POST_FOLD_SURFACE.md` §4.2). [Role and severity: Impl; the 2-of-3 verifier in the registry: Spec]

What Registry is **not**: an ethical evaluator, a behavior monitor, a billing system, or an authoritative key store. Behavior evaluation is `CIRISAgent`'s job (PDMA / CSDMA / DSDMA / IDMA). Billing is `CIRISBilling` + `CIRISPortal`. Authoritative key and attestation state lives in `CIRISPersist`; Registry composes policy verdicts over the substrate rather than holding state.

### 1.3 Position in CIRIS architecture

Mirrors the layering in [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) §1.3:

```
APPLICATION TIER          CIRISAgent, CIRISClient (consume the node's
                          verdicts; the client holds no keys)
                          ────────────────────────────────────────────────────
SERVER                    CIRISServer composes ciris-registry-core,
                          cirislens-core and ciris-node-core over one
                          persist Engine (compose_registry)
                          ────────────────────────────────────────────────────
SUBSTRATE-CONSUMING       ciris-registry-core (THIS — consumes persist's
                          FederationDirectory + verify's crypto + edge's
                          transport; identity / build / license / partner
                          verdicts as POLICY OVER SUBSTRATE, not as
                          authoritative storage)
                          ────────────────────────────────────────────────────
SUBSTRATE TIER            ciris-verify   ciris-edge        ciris-persist
                          identity +     transport         storage / audit +
                          attestation                      federation directory
                          ────────────────────────────────────────────────────
EVALUATOR                 RATCHET (reads chains across the federation)
```

Registry sits at the **substrate-consuming** tier — above the substrate proper (verify / edge / persist), below the second-tier consensus crates (node-core / lens-core), and below the application tier. Its trust verdicts compose over persist's `federation_*` tables. The standalone binary's own Postgres holds the Portal's operational state (organisations, users, partner records, audit log) and the manifest tables, with the federation-backed trust tables used as bounded-TTL caches (`docs/FEDERATION_CLIENT.md`).

**Why substrate-consuming, not substrate.** The substrate provides primitives that every other layer needs (signed identity, federated directory, transport, audit log). Registry provides a *policy* over those primitives — "does this key vouch for that build?" — which is consumed by verifiers and agents but is not itself a primitive. The distinction matters: the substrate cannot have policy in it (every consumer composes its own), but Registry's policy is the canonical "what counts as a licensed CIRIS deployment" verdict the ecosystem agrees to use.

### 1.4 Lifecycle stages

Same lifecycle as the other "Core" crates ([`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) §1.4): **Spec → Impl → Deployed (pilot) → Deployed (folded)**.

| Stage | What is at this stage | What that means |
|---|---|---|
| Spec | The registry-side 2-of-3 accord verifier; the ceremony admission gate (`FSD/FSD-003_LIFECYCLE_CEREMONIES.md`); the OPEN routes in `FSD/FSD-004` §6 | Specified; not implemented. |
| Impl | The registry slice's additional KEEP / REPLACE routes beyond those `fold::router` mounts | Implemented in the standalone binary; moved into the slice as each backend moves to persist. |
| Deployed (US / EU) | The standalone surface (agent / partner / license / revocation RPCs, Portal surfaces, trust root, build manifests) | `us.registry.ciris-services-1.ai` and `eu.registry.ciris-services-1.ai` on multi-master Postgres. |
| Deployed (folded) | `fold::router` (trust root, `/v1/agent_files/{kind}`, CEG-native `/v1/builds*`) | CIRISServer composes `ciris-registry-core` without default features as its registry slice. Singleton services remain CIRIS L3C's flagship deployments, not federation dependencies. |

Each lifecycle stage carries adversarial review proportional to its blast radius. The Deployed (US / EU) surface is the most-attacked surface — it is where every licensed CIRIS deployment in production reads its trust anchor from.

### 1.5 Recursive Golden Rule — how it bites in Registry

The Golden Rule from [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) §1.5 ("we owe ourselves what we offer to others; no principal is exempt from the standard they impose on others") is operational here at specific primitives. A reviewer should be able to grep from any sentence to a concrete bite:

- **Human-held authority binds CIRIS L3C as steward.** No single install can issue a federation-scope attestation, because no install can issue one at all (§2.1). The M-of-N steward quorum over `partner_record` binds the steward's own cross-region authority. No exemption for the principal who set the policy. ([`FEDERATION_ANNOUNCEMENT.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_ANNOUNCEMENT.md) §4.2 "load-bearing irreversibility".)
- **`PartnerRecord` revocation applies to CIRIS L3C's own partner records.** If a license-bearing organization is revoked under the protocol's rules, the same rules apply when the org is a CIRIS L3C subsidiary. `MassRevoke` (FSD-001 §580) carries no steward exemption.
- **Audit-log discipline applies to admin RPCs invoked by Registry operators.** Every `RegisterTrustedPrimitiveKey`, `RevokeEntity`, `SetEmergencyShutdown` carries `claims.sub` (the authenticated operator) into the `actor_user_id` column — including when the operator is on CIRIS L3C's staff (`docs/THREAT_MODEL.md` AV-35). The audit trail does not soft-pedal steward operations.
- **Bond forfeiture applies to CIRIS L3C-affiliated partners.** Per `CLAUDE.md` "Billing & Activation" — bond is forfeited on revocation by default. The forfeit rule does not exempt CIRIS L3C-affiliated organizations.

If a principal would be exempt from a constraint at any of these primitives, the Golden Rule is violated at that primitive and the protocol is the wrong shape there. Fix the primitive, not the rule.

**The deliberate asymmetry: humanity accord.** Same as NodeCore — the Golden Rule binds *participants in the federation* to each other. Humanity-as-such occupies a position outside the federation's participant set, by design. The three named human key holders in §2.2 below hold `AccordCarrier` authority that no federation-side authority class (including Registry's own `SYSTEM_ADMIN` / `WISE_AUTHORITY` / the regional installs) can grant itself, revoke, override, or decay. This is not a Golden-Rule exemption; it is the recognition that consent (M-1's load-bearing property) requires revocability, and revocability requires a halt-authority that lives outside the system being halted. Detailed shape: [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5; Registry-side wiring: §2.2 below.

---

## 2. Trust shape — accord-conferred stewards over member installs

### 2.1 Stewards are human-rooted keys; installs are members

**Infrastructure does not vote.** Voting for infrastructure is done only by the accord; a canonical node's *owner* handles its configuration (CC 3.4.5 `config:*` is self-or-owner). CC 3.4.7.1 says it: `node` is "steward + `infra:*` reach only, never agency", and CC 3.2's own `ciris-canonical` example lists the founders as steward keys, human-rooted, not node keys.

Two kinds of key, never fused (CC 3.4.7.3: `node` is exclusive of `agent` / `user`):

| Key | Held by | Conferred how | May do |
|---|---|---|---|
| **Steward key** (`registry-steward-{us,eu,apac}` as names for the people who hold them) | a human, hardware-rooted | accord co-scrub on the key record, roles inside the scrub-signed registration envelope (CC 3.2 T2, ceremony plane) | sign licences as the CIRIS authority, sign `partner_record` as part of its steward quorum, confer `infra:attest` on pipeline keys, vouch for primitive keys |
| **Install key** (`ciris-canonical-{1,2,3}`, the US / EU / APAC nodes) | the install, owner-bound to a human | admission into the `ciris-canonical` community by the accord's quorum; `infra:serve` / `infra:attest` carried in the genesis grant | admit, store, replicate and serve signed records; emit `provenance:*` as the steward's infrastructure; **cast no vote, issue no verdict** |

One install per region, each a `member` of `ciris-canonical` (§2.1.1). Their job is availability and replication: no single install can issue a federation-scope attestation, because no install can issue one at all. What the regions distribute is *serving*, not *authority*. Authority is distributed by the accord's roster and by the M-of-N steward quorum over `partner_record` (CC 3.3.9), both human-held.

**Why the split matters (THREAT_MODEL AV-14).** A single unified steward key was a single point of compromise. Moving the key into three HSMs would have split custody without changing what a compromised install could do. Taking authority off the install keys altogether means a compromised node can serve stale or missing records, which consumers detect (§3, `authoritative` and the witness-cosigned STH), but cannot mint a licence, a key record or a revocation that verifies.

**Rotation arc (all steps human-signed):**

| Step | What | Signed by | Effect |
|---|---|---|---|
| Confer a steward | accord co-scrub on a human key record carrying the steward roles | 2-of-3 accord holders | the key may sign as steward; `trust:confers:v1` edge |
| Add an install | admission of the install's key into `ciris-canonical` as `member` | the accord's entrenched quorum over the community record | the install serves and replicates; no vote |
| Raise the steward quorum | change the `partner_record` M-of-N | the current steward set, human-signed | professional licences need the new M |
| Retire a steward | `withdraws` on the conferral, `revoked_after` bound if compromised | 2-of-3 accord holders | rows signed before the bound stand; after it, do not |
| Un-trust from the consumer side | delete the `trust:accepts:v1` edge | the consuming node's owner | one row; every downstream gate fails closed (CC 3.2 T3) |

No protocol bump at any step: every row is an existing `delegates_to` / `attestation` primitive carrying a `trust:*` dimension (CC 3.1.1).

### 2.1.1 The canonical services are a *governed global `community`*, not a `family`

The CIRIS canonical services — Registry, Lens, Node — are modeled as a single **governed global `community`** (CC part_3 §3.2), NOT as a `family` and NOT as a single identity with occurrences. The accord-conferred steward keys of §2.1 are the community's **founding core**; the regional installs are its `member`s. What this fixes is the *trust shape consumers anchor on*.

```
community {
  community_key_id: "ciris-canonical"            // the one anchor consumers pin
  founding_core:    [registry-steward-us, -eu, -apac]   // human-rooted steward keys,
                                                        // conferred by accord co-scrub
  consensus_protocol:           "quorum:2/3"      // admission door — no SPOF; the
                                                  // quorum is the accord's, not the nodes'
  consensus_protocol_entrenched: true             // the door cannot be lowered
  members (grow over time):  Registry installs, Lens installs, Node installs,
                             and future independent operators — member role only;
                             a member install holds standing, never a vote
}
```

**Founders and members.** `founding_core` above is the set of accord-conferred *steward* keys; a Registry / Lens / Node install joins as a `member` under those keys' standing; "admitted by core quorum" means the accord's quorum over the community record, never a quorum of node keys; and a node running the registry slice gains neither a vote nor a verdict (`FSD/FSD-004_POST_FOLD_SURFACE.md` §1 rule 1).

**Why `community`, not `family` (the delta).** Both primitives share the *same* `consensus_protocol` machinery, so governance strength is identical (quorum:2/3 either way). The fork is content-model + trajectory:

1. **Content fit.** Canonical services serve *public* trust data — revocation lists, STHs, steward keys, build attestations — which federate as `holds_bytes:sha256:*`. `family`'s defining feature is the *private* at-rest `key_grant` DEK cascade + forward-secrecy ceremony — machinery that would never fire for public infrastructure. `community` (public `holds_bytes:*`, no DEK cascade, consumer-policy cohort filter on removal) is an exact match.
2. **Decentralization ramp.** `family` is a decentralization *floor* (bounded cell; adding a member is a per-event governance act). `community` is the *ramp* — `resolve_community(C, now)` scales 3 → N operators under one stable anchor, which is the literal goal of §2.1 ("the federation cannot decentralize while Registry's authority is held by one key").
3. **Legitimacy.** A community is a public commons anyone can audit (`resolve_community` is public) and eventually join; a family reads as a closed founders' cell. Canonical infrastructure must be trustable *as neutral public good*.

**The guardrail (non-negotiable).** `community`'s *default* admission (the geographic case) is permissive. A trust root MUST NOT use open admission — that invites Sybil-the-membership → dilute-quorum → rogue "canonical" attestations. Therefore the canonical-services community is **governed**: `consensus_protocol: quorum:2/3` over the vetted founding core, `consensus_protocol_entrenched: true`, and admission of any new operator rides a `supersedes` Contribution gated by founding-core quorum. Governance as strong as a `family`; content model correct for public infra.

**Layering (HUMANITY_ACCORD stays a `family`).** §2.2's HUMANITY_ACCORD remains an entrenched `family` — it is the *governance/halt* root, its content is privately governed then published, and `family` is correct there. The canonical services are a `community` *under/alongside* that family. Two layers, two primitives, both correct; the accord's halt-authority (§2.2) sits above the community exactly as it sits above the bare stewards. *(Optional, separate:* a private `family` among the same operators MAY carry genuinely-private inter-operator coordination — escrow shards, incident pre-disclosure — distinct from the public trust role.)

**Reticulum addressing.** The `community_key_id` anchor is a CEG-directory binding, **not** a Reticulum destination (Reticulum destinations are per-keypair). Consumers resolve `ciris-canonical` → the member set via `resolve_community` → Reticulum path-request to ⌈2/3⌉ members → verify the quorum attestation. A shared transport-only Reticulum identity MAY front the community for native multi-homed *reachability*, but reachability is never an attestation — it carries no authority.

**Where it lives.** The genesis bundle the registry serves carries the `ciris-canonical` community birth record (the `community` member of `GET /v1/steward-key` and `GET /v1/trust-root/bundle`, `null` when the bundle carries none). `docs/TRUST_CONTRACT.md` names `ciris-canonical` as the consumer pin anchor. Lens and Node installs join the same community as members. The substrate's `communities` table carries the record; no registry-local schema is involved.

### 2.2 HUMANITY_ACCORD — the parallel hierarchy above the stewards

Per [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5.

Registry's `SystemRole` enum:

```protobuf
enum SystemRole {
  SYSTEM_ROLE_UNSPECIFIED = 0;
  SYSTEM_ADMIN = 1;       // full system access
  SYSTEM_AUDITOR = 2;     // read-only across orgs
  WISE_AUTHORITY = 3;     // 9 max, staggered 3-year terms
  HUMANITY_ACCORD = 4;    // 3 named, 2-of-3, permanent, hardware-rooted
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
| Key material (the actual Ed25519 + ML-DSA-65 pubkeys for the three holders) | **`CIRISPersist` substrate** — `federation_keys` rows with `identity_type="accord_holder"`, self-signed at provisioning, carried in the genesis bundle with custody evidence | Federation-wide identities belong in the shared substrate. Every peer (Registry, Lens, Agent, NodeCore) reads the same authoritative rows. If accord keys lived in Registry's private tables, the node's in-process kill-switch path would have to RPC back to a singleton registry. |
| Role-recognition policy + `EmergencyShutdown CONSTITUTIONAL` RPC surface + 2-of-3 multi-sig verifier + audit hooks | **`ciris-registry-core`** (this crate) and CIRISServer's accord surface | The standalone service and the registry slice inside CIRISServer verify the same envelope against the same policy. |

**The two triples are separate things:**

| Triple | What | Where stored | Lifecycle |
|---|---|---|---|
| 3 regional stewards (US / EU / APAC) | Human-held steward keys conferred by accord co-scrub; cross-attest via `vouches_for` | `federation_keys` `identity_type="steward"`, one per region | Rotatable per the §2.1 rotation arc |
| 3 humanity-accord holders | Human-held, hardware-attested kill-switch keys; 2-of-3 threshold | `federation_keys` `identity_type="accord_holder"`, self-signed at provisioning, cross-attested by all 3 stewards | Permanent (no automatic decay); replacement requires out-of-band CIRIS L3C process per `FEDERATION_ANNOUNCEMENT.md` §4.5.3 |

The 3 stewards vouch for the 3 accord holders. That gives the layered constitution: stewards do day-to-day operational attestation; humans hold the federation-wide halt. Both layers live in the same substrate; recognition policy lives in the code that runs in every peer.

**Authority scope (the dual constraint).** `HUMANITY_ACCORD` signatures are valid only on `EmergencyShutdown CONSTITUTIONAL` and the corresponding `FederationAnnouncement priority: AccordCarrier`. Announcements of any other priority signed by accord-holder keys are rejected at admission (out of role). Federation-side authority cannot sign `AccordCarrier`; humanity-accord authority cannot sign anything else. Wire-isolated AND scope-isolated.

### 2.3 What this delivers for M-1

- **Steward governance reach is decentralized.** No single party — including CIRIS L3C — can issue federation-scope Registry attestations unilaterally. The role "who can speak for the Registry" is a federated property, not a singular one.
- **Kill-switch reach is human-held and permanent.** A federation-wide `EmergencyShutdown CONSTITUTIONAL` is reachable only by 2-of-3 humanity-accord multi-sig, and no federation-internal authority can revoke, override, or decay that path. Consent (M-1's load-bearing property) retains its revocability backstop.
- **The two paths are independently observable.** Steward attestations are gossiped via the substrate; humanity-accord invocations route through the same channel. Both are durable in `audit_log`; RATCHET observes both for behavioral baselines.

---

## 3. Surface — what Registry exposes to the federation

`FSD/FSD-004_POST_FOLD_SURFACE.md` §4 is the route map: every route and RPC, its substrate call, its caller tier and the node posture that may serve it.

### 3.1 Public read surface (`RegistryService` and HTTP)

Unauthenticated, rate-limited per [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md) AV-9:

- `LookupAgent` / `BatchLookupAgents` — agent identity by hash. [Deployed]
- `LookupPartner` — partner by ID; HTTP `GET /v1/partner/{key_id}` composes recognition and per-authority licensure. [Deployed]
- `VerifyDeployment` — combined agent + partner verification. [Deployed]
- `GetRevocationList` — full or delta revocation; HTTP `GET /v1/revocation/{target_id}`. [Deployed]
- `GetPublicKeys` — organization public keys; HTTP `GET /v1/verify/key/{fingerprint}`. [Deployed]
- `GetOfflinePackage` / `GetOfflineDelta` — offline-verification snapshots. [Deployed]
- `GetBuildAttestation` — SLSA build provenance. [Deployed]
- `GetEmergencyStatus` — emergency shutdown status (LOW / MEDIUM / HIGH / CRITICAL / **CONSTITUTIONAL**). [Deployed; the CONSTITUTIONAL halt is served by CIRISServer]
- HTTP `GET /v1/steward-key` and `GET /v1/trust-root/bundle` — the genesis trust-root bundle. [Deployed (US / EU / folded)]
- HTTP `/v1/builds*` — build manifests: CEG-native from the federation directory on the slice; from the build table, with directory-composed provenance, on the standalone binary. [Deployed (US / EU / folded)]
- HTTP `/v1/verify/*` — binary, function and build manifests by version and target. [Deployed]

### 3.2 Authenticated surface (`PortalService`, `RegistryAdminService`)

HS256 JWT for gRPC; per-method `OrgRole` authorization per [`CLAUDE.md`](CLAUDE.md) §"PortalService Handler Convention". Methods span org / user / key / audit / escrow / compliance. `FSD/FSD-004` §4.2 and §4.3 map each one to the signed CEG envelope or server surface that carries it in the registry slice.

### 3.3 Substrate-consumer surface

The registry slice hands CIRISServer one router, `fold::router(engine, node_key_id)` (`FSD/FSD-004` §5): no own storage, no own identity or Edge, no gRPC. The policy operations the slice composes over the substrate, as specified in `FSD/FSD-002_FEDERATION_SURFACE.md` §9:

| Operation | Substrate read/write | Composed policy |
|---|---|---|
| `verify_primitive_key(key_id, expected_primitive)` | `lookup_public_key` + `list_attestations_for` + `revocations_for` | Policy A (direct trust on a blessed key) per `FEDERATION_CLIENT.md` §"Trust policy" |
| `attest_primitive_key(primitive_key_id, weight, expires)` | `put_attestation` | A steward key signs the attestation envelope |
| `revoke_primitive_key(primitive_key_id, reason)` | `put_revocation` | A steward key signs the revocation envelope |
| `verify_humanity_accord_envelope(envelope, sigs[])` | `lookup_public_key` for each accord-holder | 2-of-3 multi-sig threshold; hardware-attestation check; out-of-role rejection (only valid on `EmergencyShutdown CONSTITUTIONAL`) |
| `resolve_partner_capabilities(partner_id)` | `lookup_public_key` + `list_attestations_by(steward)` + `revocations_for` | `effective_capabilities = agent ∩ partner.granted − partner.denied` per FSD-001 §215 |

Wire-format stability: `RegistryService` and `PortalService` wire shapes are the same in both deployment shapes. The substrate is the backend; the wire format is the contract.

### 3.4 Attestation primitive — unified scalar shape

Registry produces and consumes attestations against persist's `federation_attestations` table. The federation's **unified primitive shape** is one workhorse `scores` attestation_type carrying a scalar claim on a named dimension, plus four structural primitives that operate on the attestation graph itself. No categorical-pejorative `attestation_type` strings; loaded vocabulary is deflated to scoped dimensions with graduated scores.

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

Full vocabulary + reserved-prefix enforcement patterns + envelope schemas + per-dimension citations to owning MISSIONs live in [`FSD/FSD-002_FEDERATION_SURFACE.md`](FSD/FSD-002_FEDERATION_SURFACE.md); the normative wire format is the CIRIS Constitution.

**Layering principle.** Primitives are clean and complete at the wire format. UX sugar lives ABOVE primitives, in Portal / verify dashboards / agent introspection panels. A "Mark Licensed" button writes a `scores` attestation on `licensure:{authority_id}` underneath; the categorical button is UX, the scalar is wire. Same for negative cases. This separation lets product surfaces evolve their vocabulary independently of the substrate and keeps the federation's wire format pristine across many UX iterations.

---

## 4. Substrate dependencies

Registry consumes the substrate; it does not author substrate primitives. The pins each release carries are recorded in `CHANGELOG.md`.

| Substrate component | What Registry needs from it |
|---|---|
| **`CIRISPersist`** | `FederationDirectory` trait + `federation_keys` / `_attestations` / `_revocations` tables; the `GenesisBundle` the registry serves; `identity_type` values `steward`, `accord_holder`, `witness`; the operational planes (`organization` / `org_membership` LWW, `partner_record` monotonic quorum); bounded `AttestationQuery` paging; the in-process `Engine` the registry slice shares with CIRISServer. |
| **`CIRISVerify`** | Hybrid Ed25519 + ML-DSA-65 signing and verification (`ciris-crypto`, `ciris-keyring`); the build-manifest wire shape and the manifest Contribution verifier; consumption of the trust-root bundle. |
| **`CIRISEdge`** | Reticulum-native federation transport; the federation signer for announces; `MessageType::FederationAnnouncement` with `Delivery: Mandatory` and recognition of accord-holder authority for `AccordCarrier` priority. |
| **`CIRISLensCore`** | Observability for divergence telemetry (`federation_dual_write_divergence_total`, `federation_cache_age_seconds`). |
| **`CIRISNodeCore`** | The shipped `evaluate_consensus_protocol` predicate that the ceremony gate calls (`FSD/FSD-003` §5); the cohabitation contract that names `ciris-registry-core` as a peer slice in CIRISServer. |

---

## 5. Deployment shapes

`ciris-registry-core` runs in two shapes:

| Shape | What it is |
|---|---|
| **Standalone** (`standalone` feature, default) | The `ciris-registry` binary: sqlx and Postgres, the three gRPC services, the full HTTP router, its own Edge runtime for `/v1/identity`. Deployed at `us.` and `eu.registry.ciris-services-1.ai`. |
| **Registry slice** (no default features) | No sqlx, tonic or Postgres driver in the graph. CIRISServer composes it through `fold::router` alongside `ciris-node-core` and `cirislens-core`, sharing one persist Engine. Every KEEP / REPLACE route in `FSD/FSD-004` moves into this router as its backend moves to persist. |

The singleton deployments at `*.registry.ciris-services-1.ai` are CIRIS L3C's flagship deployments, not federation dependencies: any node composing the registry slice holds the same policy code, and the federation cannot be denied identity verification by one deployment going down.

---

## 6. Open questions

- **Selection authority for HUMANITY_ACCORD replacement.** Boot phase: CIRIS L3C CEO under advisement of CIRIS L3C board ([`FEDERATION_ANNOUNCEMENT.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_ANNOUNCEMENT.md) §4.5.3). Formalization of the selection process to remove the CEO-as-single-party dependency is open (question 18 in `FEDERATION_ANNOUNCEMENT.md` §7).
- **The OPEN routes in `FSD/FSD-004` §6**: manifest-bytes storage in persist, `VerifyDeployment` as a combined read, offline packages, who may author agent registrations and org recognition, which org facts are public, the device-integrity home, the `registry:lookup` capability name, the Portal screens that replace the Portal RPCs, and the community values the Constitution still carries on `partner_role` and `partner_record.license_type`.

Resolved questions, for the record: consumers pin the genesis bundle served at `GET /v1/steward-key` and `GET /v1/trust-root/bundle`, which is self-authenticating, so there is no per-steward fingerprint to pin; an install is admitted into `ciris-canonical` by the accord's quorum and does not bootstrap a steward key of its own; the registry slice reaches persist through the in-process Engine CIRISServer shares with it, and the standalone binary through its own persist client; `PartnerRecord` becomes a `partner_record` envelope under M-of-N steward quorum with the licence carried separately as `licensure:{authority_id}` rows (`FSD/FSD-004` §4.2).

---

## 7. References

### Within CIRISRegistry
- [`FSD/FSD-004_POST_FOLD_SURFACE.md`](FSD/FSD-004_POST_FOLD_SURFACE.md) — the registry surface as composed into CIRISServer
- [`docs/FEDERATION_CLIENT.md`](docs/FEDERATION_CLIENT.md) — registry-side architecture for consuming persist's federation directory
- [`docs/TRUST_CONTRACT.md`](docs/TRUST_CONTRACT.md) — consumer-facing trust shape (trust-root pinning, manifest consumption paths)
- [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md) — threat model (AV-14 single-point-of-compromise, AV-15 PortalService authorization, AV-35 audit actor integrity)
- [`docs/DESIGN_ROLE_HIERARCHY.md`](docs/DESIGN_ROLE_HIERARCHY.md) — the SystemRole hierarchy `HUMANITY_ACCORD` extends
- [`FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md`](FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md) — protocol surface (gRPC + HTTP)
- [`FSD/FSD-002_FEDERATION_SURFACE.md`](FSD/FSD-002_FEDERATION_SURFACE.md) — the federation attestation surface (design history)
- [`FSD/FSD-003_LIFECYCLE_CEREMONIES.md`](FSD/FSD-003_LIFECYCLE_CEREMONIES.md) — self / family / community ceremony envelopes
- [`CLAUDE.md`](CLAUDE.md) — codebase guide + PortalService handler convention + database migration discipline

### Federation context
- [`CIRISConstitution`](https://github.com/CIRISAI/CIRISConstitution) — the normative wire format
- [`CIRISNodeCore/MISSION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/MISSION.md) — peer mission doc; cohabitation trajectory anchored here
- [`CIRISNodeCore/CIRIS_FEDERATION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/CIRIS_FEDERATION.md) — system claim, "decentralized ethical superintelligence", Registered vs Sovereign paths
- [`CIRISNodeCore/FSD/FEDERATION_ANNOUNCEMENT.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/FEDERATION_ANNOUNCEMENT.md) — multi-party bootstrap (§4.2), humanity accord (§4.5)
- [`CIRISNodeCore/FSD/TRUST_HIERARCHY.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/TRUST_HIERARCHY.md) — TrustType / TrustRelationship axes
- [`CIRISNodeCore/FSD/SUBSTRATE_INTEGRATION.md`](https://github.com/CIRISAI/CIRISNodeCore/blob/main/FSD/SUBSTRATE_INTEGRATION.md) — substrate-consumer integration contract
- [`CIRISAgent/ACCORD.md`](https://github.com/CIRISAI/CIRISAgent/blob/main/ACCORD.md) §M-1 — meta-goal grounding

### Substrate
- [`CIRISPersist/docs/FEDERATION_DIRECTORY.md`](https://github.com/CIRISAI/CIRISPersist/blob/main/docs/FEDERATION_DIRECTORY.md) — substrate contract for the federation directory
- [`CIRISPersist`](https://github.com/CIRISAI/CIRISPersist) `src/federation/mod.rs` — `FederationDirectory` trait surface
- [`CIRISVerify`](https://github.com/CIRISAI/CIRISVerify) — hardware-rooted identity verification consumer
- [`CIRISEdge`](https://github.com/CIRISAI/CIRISEdge) — Reticulum-native transport
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
