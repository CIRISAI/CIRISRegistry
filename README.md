# CIRISRegistry

**The attestation-policy layer of the CIRIS federation.** Verifies agent identity, composes partner authorization grants, distributes revocations in real-time, attests per-install steward identities, and recognizes the HUMANITY_ACCORD constitutional layer — all as policy over the shared trust substrate.

**License**: AGPL-3.0-or-later · **Crate target**: `ciris-registry-core` · **Service**: `*.registry.ciris-services-1.ai`

---

## What it is

A Rust crate (today, exported as a gRPC + HTTP service; trajectory: a sub-crate that runs both standalone and in-process inside CIRISAgent). Five functions, all expressed as **policy composed over the shared federation substrate** ([CIRISPersist](https://github.com/CIRISAI/CIRISPersist) for storage, [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) for crypto, [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) for transport):

| Function | What | Status |
|---|---|---|
| **Agent identity verification** | Cryptographic verification of legitimate agent builds and their declared capabilities. | Deployed (US + EU) |
| **Partner authorization** | License management for organizations deploying CIRIS agents in regulated contexts (medical / legal / financial). | Deployed (US + EU) |
| **Revocation distribution** | Real-time multi-source revocation status (DNS US + DNS EU + HTTPS API); any revoke from any source is immediately enforced. | Deployed (US + EU) |
| **Steward attestation** | Per-install registry-steward keys (US / EU / APAC) self-publish to the federation directory and cross-attest each other; M-of-N gates federation-scope attestations. | Spec ([CIRISRegistry#17](https://github.com/CIRISAI/CIRISRegistry/issues/17)) |
| **HUMANITY_ACCORD recognition** | Constitutional kill-switch authority — 3 named human holders, 2-of-3 multi-sig, hardware-attested, no federation-internal principal can grant / revoke / override / decay. | Spec ([CIRISRegistry#16](https://github.com/CIRISAI/CIRISRegistry/issues/16)) |

**What Registry is not**: an ethical evaluator, a behavior monitor, a billing system, or — post-substrate-conformance — an authoritative key store. Behavior evaluation is [`CIRISAgent`](https://github.com/CIRISAI/CIRISAgent)'s job. Billing is `CIRISBilling` + `CIRISPortal`. Post-[#17](https://github.com/CIRISAI/CIRISRegistry/issues/17), authoritative key state lives in CIRISPersist; Registry composes policy verdicts over the substrate rather than holding state.

---

## Position in the stack

```
APPLICATION       CIRISAgent (federation client; consumes ciris-node-core,
                   cirislens-core, and ciris-registry-core as in-process
                   substrate-conformant crates per the cohabitation arc)
                  ──────────────────────────────────────────────────────────
SECOND TIER       cirislens-core             ciris-node-core
                   observability/F-3          15 consensus primitives
                  ──────────────────────────────────────────────────────────
SUBSTRATE-        ciris-registry-core (THIS — policy over substrate;
CONSUMING          attestation namespace, partner-authorization composition,
                   HUMANITY_ACCORD verifier, revocation policy)
                  ──────────────────────────────────────────────────────────
SUBSTRATE         ciris-verify          ciris-edge           ciris-persist
                   identity + crypto     Reticulum transport  storage + audit
                  ──────────────────────────────────────────────────────────
EVALUATOR         RATCHET (anti-Sybil; reads federation audit chains)
```

Registry sits at the **substrate-consuming** tier. Pre-migration it holds authoritative pubkey + license state in its own Postgres. Post-migration it composes verdicts over `CIRISPersist`'s `federation_*` tables and Registry-local tables become bounded-TTL caches. See [`MISSION.md`](./MISSION.md) §1.3 for the full architecture and §1.4 for the lifecycle stages.

---

## Quick start

```bash
# Bring up Postgres + Registry locally
docker compose up -d

# Health check
curl http://localhost:8082/health

# gRPC health
grpcurl -plaintext localhost:50052 ciris.registry.v1.RegistryService/HealthCheck

# Public read (e.g., revocation list)
curl http://localhost:8082/v1/revocation/agent-{hash}

# Verify a build manifest (Path B — verbatim signed payload, byte-identical to POST)
curl http://localhost:8082/v1/verify/build-manifest/{project}/{version}/{target}
```

| Env var | Default | Purpose |
|---|---|---|
| `DATABASE_URL` | `postgres://ciris:ciris_dev@localhost:5434/ciris_registry` | PostgreSQL connection |
| `GRPC_PORT` | `50052` | gRPC server port |
| `HTTP_PORT` | `8082` | HTTP health / metrics / `/v1/*` endpoints |
| `REGISTRY_ADMIN_TOKEN` | (required for admin writes) | Bearer token for `POST /v1/builds` + `POST /v1/verify/build-manifest` |
| `ED25519_KEY_PATH` / `MLDSA_KEY_PATH` | (recommended for prod) | Persistent steward keys (else ephemeral per restart — verify clients break) |
| `RUST_LOG` | `info` | Log level |

Build + test: `cd rust-registry && cargo build && cargo test`. Full development guide in [`CLAUDE.md`](./CLAUDE.md).

---

## The attestation surface (FSD-002)

The federation has exactly **one workhorse attestation primitive plus four structural composers** (per [`FSD/FSD-002_FEDERATION_SURFACE.md`](./FSD/FSD-002_FEDERATION_SURFACE.md) §2):

```
WORKHORSE:    scores       — scalar score + confidence on a named dimension
STRUCTURAL:   delegates_to — A may sign on behalf of B in scope S
              supersedes   — this attestation replaces a prior
              withdraws    — I retract my prior (not necessarily false)
              recants      — my prior was false (admits epistemic error)
```

Registry's owned dimension slice within the federation namespace: `licensure:{authority_id}`, `partner_role:{role}`, `revocation:{entity_type}:{reason}`, `bond_posted:{currency}`, `build:registered:{target}`, plus the reserved `accord:*` prefix (only `identity_type=accord_holder` may emit). FSD-002 v1.2 added the §1.10.1 operational-language gate — prefix names must describe machine-checkable mechanisms, not subjective qualities (the discipline behind [`ciris.ai/safety-vs-censorship`](https://ciris.ai/safety-vs-censorship/)).

---

## Recursive Golden Rule

No principal — including CIRIS L3C as steward — is exempt from constraints the protocol imposes on others ([`MISSION.md`](./MISSION.md) §1.5). Operational bites:

- **Per-install stewards bind CIRIS L3C as steward.** Once `bootstrap_threshold ≥ 2`, no single Registry install can issue federation-scope attestations unilaterally — including CIRIS L3C's own.
- **Partner-revocation rules apply to CIRIS L3C subsidiaries.** `MassRevoke` carries no steward exemption.
- **Audit discipline applies to steward operations.** Every admin RPC carries the operator's identity into `actor_user_id`, including for CIRIS L3C staff.
- **Bond forfeiture applies to CIRIS L3C-affiliated partners.** No exemption.
- **The HUMANITY_ACCORD asymmetry is the one constitutional asymmetry.** Three named human holders carry kill-switch authority no federation-internal authority can grant / revoke / override / decay — the recognition that consent requires revocability, and revocability requires a halt-authority outside the system being halted.

---

## Documentation

**Start here:**
- [`MISSION.md`](./MISSION.md) — mission, trust shape, position in CIRIS architecture, federation surface
- [`FSD/FSD-002_FEDERATION_SURFACE.md`](./FSD/FSD-002_FEDERATION_SURFACE.md) — wire-format-locked federation surface (v1.2)
- [`FSD/PRIOR_ART_SCAN.md`](./FSD/PRIOR_ART_SCAN.md) — design-space comparison vs PGP / SPKI/SDSI / VC / Birdwatch / Pol.is / Kleros / Spritely / Holochain / Aragon / Conviction Voting / Sigstore / SLSA
- [`FSD/SOTA_SCAN.md`](./FSD/SOTA_SCAN.md) — production-validation comparison against the same systems

**Protocol + ops:**
- [`FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md`](./FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md) — gRPC + HTTP protocol surface
- [`docs/TRUST_CONTRACT.md`](./docs/TRUST_CONTRACT.md) — consumer-facing trust shape (`/v1/steward-key` pinning, rotation policy, Paths A/B/C)
- [`docs/THREAT_MODEL.md`](./docs/THREAT_MODEL.md) — threat model with named AVs (AV-14 / AV-15 / AV-25 / AV-26 / AV-28 / AV-33 / AV-35)
- [`docs/FEDERATION_CLIENT.md`](./docs/FEDERATION_CLIENT.md) — substrate-consumer architectural sketch
- [`docs/MANIFEST_VALIDATION_API.md`](./docs/MANIFEST_VALIDATION_API.md) — wire-format / endpoint reference

**Development:**
- [`CLAUDE.md`](./CLAUDE.md) — development guide, PortalService handler convention, migration discipline, multi-master Spock replication rules

---

## CIRIS ecosystem

| Component | Role | Repo |
|---|---|---|
| **CIRISRegistry** | Attestation policy over substrate (this) | (here) |
| **CIRISPersist** | Storage + audit + federation directory (substrate) | [CIRISPersist](https://github.com/CIRISAI/CIRISPersist) |
| **CIRISVerify** | Hardware-rooted identity + hybrid crypto (substrate) | [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) |
| **CIRISEdge** | Reticulum-native federation transport (substrate, spec-first) | [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) |
| **CIRISNodeCore** | 15 consensus primitives (second tier) | [CIRISNodeCore](https://github.com/CIRISAI/CIRISNodeCore) |
| **CIRISLensCore** | Observability + F-3 correlated-action detector (second tier) | [CIRISLensCore](https://github.com/CIRISAI/CIRISLensCore) |
| **CIRISAgent** | Ethical-AI agent framework (application) | [CIRISAgent](https://github.com/CIRISAI/CIRISAgent) |
| **CIRISPortal** | Admin web interface | [CIRISPortal](https://github.com/CIRISAI/CIRISPortal) |
| **RATCHET** | Anti-Sybil evaluator; reads federation audit chains | [RATCHET](https://github.com/CIRISAI/RATCHET) |

---

## Contact

- Technical: `registry@ciris.ai`
- Security: `security@ciris.ai`
- Licensing: `licensing@ciris.ai`

Copyright 2025-2026 CIRIS L3C. Source code licensed AGPL-3.0-or-later (see [`LICENSE`](./LICENSE)).
