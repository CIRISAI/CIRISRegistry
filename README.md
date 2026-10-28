# CIRISRegistry

The attestation-policy layer of the CIRIS federation. The registry serves the federation's genesis trust root, verifies and serves build manifests as signed CEG records, exposes the public lookup API that CIRISVerify and CIRISAgent read, and hosts the organisation, user, key and audit surfaces that CIRISPortal writes to.

**License**: AGPL-3.0-or-later · **Crate**: `ciris-registry-core` · **Service**: `*.registry.ciris-services-1.ai`

---

## What it does

| Function | What |
|---|---|
| **Trust root** | `GET /v1/steward-key` and `GET /v1/trust-root/bundle` serve the genesis bundle: the humanity-accord charter, the accord-holder roster with custody evidence, the `infra:*` scopes the charter confers, the serve-node grants and the `ciris-canonical` community record. The bundle is self-authenticating; the outer envelope carries no response signature by design. |
| **Build manifests** | On the registry slice, `POST /v1/builds` admits a build manifest signed by a pipeline key that a `delegates_to(root → pipeline, infra:attest)` grant or the accord role on its key record blesses, and `GET /v1/builds/{version}`, `/v1/builds/hash/{manifest_hash}` and `/v1/builds/manifest/{manifest_hash}` serve the signed claim and the manifest bytes from the federation directory. The standalone binary serves `/v1/builds/{version}` and `/v1/builds/hash/{build_hash}` from its build table, with the provenance block composed from the directory, and the `/v1/verify/*-manifest` routes for binary, function and build manifests. |
| **Agent and partner lookup** | `LookupAgent`, `LookupPartner`, `VerifyDeployment`, `GetRevocationList` and the HTTP `/v1/verify/*`, `/v1/revocation/*` and `/v1/partner/{key_id}` routes. Unknown agents and partners are treated as community tier; any revocation signal is enforced immediately. |
| **Partner licensure** | `GET /v1/partner/{key_id}` composes the organisation's recognition and its `licensure[]` entries, keyed by the licensing authority's org UUID with the statuses `issued`, `suspended` and `revoked`. |
| **Portal surfaces** | `PortalService` (organisations, users, keys, escrow, audit log, compliance reports) and `RegistryAdminService` (agent and partner registration, revocation, emergency lockdown, signing keys, webhooks) over gRPC with JWT authentication and per-method role checks. |
| **Device integrity** | `/v1/integrity/*` nonce and verification routes for Google Play Integrity and Apple App Attest. |

**What the registry is not**: an ethical evaluator, a behaviour monitor, a billing system or an authoritative key store. Behaviour evaluation is CIRISAgent's job, billing is CIRISPortal and Stripe, and authoritative key and attestation state lives in CIRISPersist. The registry composes policy verdicts over that substrate.

---

## Relationship to CIRISServer

`ciris-registry-core` builds two ways.

- **Standalone** (the default `standalone` feature): the `ciris-registry` binary with sqlx and Postgres, the three gRPC services and the full HTTP router. This is the deployment at `*.registry.ciris-services-1.ai`.
- **Registry slice**: without default features the crate carries no sqlx, tonic or Postgres driver. CIRISServer composes it as the registry slice through `fold::router(engine, node_key_id)`, which serves the trust-root routes, `/v1/agent_files/{kind}` and the CEG-native `/v1/builds*` routes over the server's shared persist engine. The standalone binary mounts the same trust-root and `agent_files` handlers, so both deployments share one implementation of them.

The server owns health, `/v1/identity`, the accord-holder roster and the Reticulum transport. [`FSD/FSD-004_POST_FOLD_SURFACE.md`](./FSD/FSD-004_POST_FOLD_SURFACE.md) maps every registry route and RPC to its post-fold path, caller tier and node posture.

---

## Position in the stack

```
APPLICATION       CIRISAgent, CIRISClient (consume the registry's verdicts)
                  ──────────────────────────────────────────────────────────
SERVER            CIRISServer (composes ciris-registry-core, ciris-lens-core
                   and ciris-node-core over one persist engine)
                  ──────────────────────────────────────────────────────────
POLICY            ciris-registry-core (THIS: trust root, build manifests,
                   partner licensure, revocation policy, Portal surfaces)
                  ──────────────────────────────────────────────────────────
SUBSTRATE         ciris-verify          ciris-edge           ciris-persist
                   identity + crypto     Reticulum transport  storage + audit
```

The registry sits above the substrate and below the application tier. It composes verdicts over persist's `federation_*` tables and never holds authoritative key state of its own. See [`MISSION.md`](./MISSION.md) for the trust shape and the Recursive Golden Rule.

---

## Quick start

```bash
# Bring up Postgres + Registry locally
docker compose up -d

# Health check
curl http://localhost:8082/health

# gRPC health
grpcurl -plaintext localhost:50052 ciris.registry.v1.RegistryService/HealthCheck

# Trust root
curl http://localhost:8082/v1/steward-key

# Revocation status for an agent, partner or license id
curl http://localhost:8082/v1/revocation/{target_id}

# A build manifest, byte-identical to what was posted
curl http://localhost:8082/v1/verify/build-manifest/{project}/{version}/{target}
```

| Env var | Default | Purpose |
|---|---|---|
| `DATABASE_URL` | `postgres://ciris:ciris_dev@localhost:5434/ciris_registry` | PostgreSQL connection for the registry's own tables |
| `GRPC_PORT` | `50052` | gRPC server port |
| `HTTP_PORT` | `8082` | HTTP port for health, metrics and `/v1/*` |
| `REGISTRY_ADMIN_TOKEN` | (required for admin writes) | Bearer token for `POST /v1/builds` and the `POST /v1/verify/*-manifest` routes |
| `JWT_SECRET` | (required for gRPC auth) | HS256 secret for `PortalService` and `RegistryAdminService` tokens |
| `ED25519_KEY_PATH` / `MLDSA_KEY_PATH` | (recommended) | Persistent steward seeds; unset means ephemeral keys per restart |
| `FEDERATION_PERSIST_DSN` | `sqlite::memory:` | Persist engine for the federation directory; production points at Postgres |
| `FEDERATION_DUAL_WRITE_ENABLED` | `false` | Use the real persist client for federation reads and writes |
| `CIRIS_REGISTRY_EDGE_IDENTITY_PATH` | (optional) | Persistent Reticulum transport identity |
| `RUST_LOG` | `info` | Log level |

Build and test:

```bash
cd rust-registry
cargo build --workspace
cargo test --workspace --lib

# The build CIRISServer depends on: no sqlx, no tonic, no Postgres driver
cargo test -p ciris-registry-core --no-default-features
```

The development guide, handler conventions and migration rules are in [`CLAUDE.md`](./CLAUDE.md).

---

## Recursive Golden Rule

No principal, including CIRIS L3C as steward, is exempt from constraints the protocol imposes on others ([`MISSION.md`](./MISSION.md) §1.5):

- **Authority is human-held.** Every registry decision (register, license, revoke, halt) is a signature by a human-held key. A node, including a canonical one, admits, stores and serves records; it issues no verdict and casts no vote.
- **Partner-revocation rules apply to CIRIS L3C subsidiaries.** `MassRevoke` carries no steward exemption.
- **Audit discipline applies to steward operations.** Every admin RPC records the authenticated caller as the actor, including CIRIS L3C staff.
- **Bond forfeiture applies to CIRIS L3C-affiliated partners.** No exemption.
- **The HUMANITY_ACCORD asymmetry is the one constitutional asymmetry.** Three named human holders carry halt authority that no federation-internal principal can grant, revoke, override or decay, because consent requires revocability and revocability requires a halt authority outside the system being halted.

---

## Documentation

**Start here:**
- [`MISSION.md`](./MISSION.md) — mission, trust shape, position in the CIRIS architecture
- [`FSD/FSD-004_POST_FOLD_SURFACE.md`](./FSD/FSD-004_POST_FOLD_SURFACE.md) — the registry surface as composed into CIRISServer: every route, its caller tier and node posture
- [`FSD/FSD-002_FEDERATION_SURFACE.md`](./FSD/FSD-002_FEDERATION_SURFACE.md) — the federation attestation surface (design history; the CIRIS Constitution is normative)
- [`FSD/PRIOR_ART_SCAN.md`](./FSD/PRIOR_ART_SCAN.md) and [`FSD/SOTA_SCAN.md`](./FSD/SOTA_SCAN.md) — design-space and production-validation comparisons

**Protocol and operations:**
- [`FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md`](./FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md) — gRPC and HTTP protocol surface
- [`FSD/FSD-003_LIFECYCLE_CEREMONIES.md`](./FSD/FSD-003_LIFECYCLE_CEREMONIES.md) — self, family and community lifecycle ceremony envelopes
- [`docs/TRUST_CONTRACT.md`](./docs/TRUST_CONTRACT.md) — consumer-facing trust shape: trust-root pinning, manifest consumption paths
- [`docs/THREAT_MODEL.md`](./docs/THREAT_MODEL.md) — threat model with named attack vectors
- [`docs/FEDERATION_CLIENT.md`](./docs/FEDERATION_CLIENT.md) — the registry as a policy layer over persist's federation directory
- [`docs/MANIFEST_VALIDATION_API.md`](./docs/MANIFEST_VALIDATION_API.md) — endpoint and wire-format reference for verification clients
- [`CHANGELOG.md`](./CHANGELOG.md) and [`RELEASE.md`](./RELEASE.md) — release history and release process

**Development:**
- [`CLAUDE.md`](./CLAUDE.md) — development guide, PortalService handler convention, migration discipline

---

## CIRIS ecosystem

| Component | Role | Repo |
|---|---|---|
| **CIRISRegistry** | Attestation policy over the substrate (this) | (here) |
| **CIRISServer** | The node: composes the registry, lens and node-core slices | [CIRISServer](https://github.com/CIRISAI/CIRISServer) |
| **CIRISConstitution** | The normative wire format (CEG) | [CIRISConstitution](https://github.com/CIRISAI/CIRISConstitution) |
| **CIRISPersist** | Storage, audit and federation directory (substrate) | [CIRISPersist](https://github.com/CIRISAI/CIRISPersist) |
| **CIRISVerify** | Hardware-rooted identity and hybrid crypto (substrate) | [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) |
| **CIRISEdge** | Reticulum-native federation transport (substrate) | [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) |
| **CIRISNodeCore** | Consensus primitives | [CIRISNodeCore](https://github.com/CIRISAI/CIRISNodeCore) |
| **CIRISLensCore** | Observability and the correlated-action detector | [CIRISLensCore](https://github.com/CIRISAI/CIRISLensCore) |
| **CIRISAgent** | Ethical-AI agent framework (application) | [CIRISAgent](https://github.com/CIRISAI/CIRISAgent) |
| **CIRISPortal** | Admin web interface | [CIRISPortal](https://github.com/CIRISAI/CIRISPortal) |
| **RATCHET** | Anti-Sybil evaluator over federation audit chains | [RATCHET](https://github.com/CIRISAI/RATCHET) |

---

## Contact

- Technical: `registry@ciris.ai`
- Security: `security@ciris.ai`
- Licensing: `licensing@ciris.ai`

Copyright 2025-2026 CIRIS L3C. Source code licensed AGPL-3.0-or-later (see [`LICENSE`](./LICENSE)).
