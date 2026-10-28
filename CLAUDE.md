# CIRISRegistry - Claude Development Guide

## Project Overview

CIRISRegistry is the authoritative source of truth for the CIRIS AI ecosystem, providing:

- **Agent Identity** - Verification of legitimate agent builds and their capabilities
- **Partner Authorization** - License management for organizations deploying CIRIS agents
- **Revocation State** - Real-time status of compromised or revoked agents/licenses

The registry is the **trust backbone** that distinguishes licensed professional deployments from community deployments. It enables the ecosystem's core promise: *"You're not paying for capability. You're paying for accountability."*

## Implementation

**Language:** Rust (2021 edition)
**Framework:** Tonic (gRPC) + Axum (HTTP)
**Database:** PostgreSQL with SQLx
**Cryptography:** Ed25519 (ed25519-dalek) + ML-DSA-65 (pqcrypto-dilithium)
**License:** AGPL-3.0-or-later

### Build & Run

```bash
# Development
cd rust-registry
cargo build
cargo run

# Production
cargo build --release
./target/release/ciris-registry

# Docker
docker compose up -d
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://ciris:ciris_dev@localhost:5434/ciris_registry` | PostgreSQL connection |
| `GRPC_PORT` | `50052` | gRPC server port |
| `HTTP_PORT` | `8082` | HTTP health/metrics port |
| `RUST_LOG` | `info` | Log level |

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              CLIENTS                                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────┐         ┌──────────────────────────────────────┐  │
│  │ CIRISVerify      │         │ CIRISPortal                          │  │
│  │ (Read-only)      │         │ (Read + Write)                       │  │
│  │                  │         │                                      │  │
│  │ • Agent lookup   │         │ • Organization management            │  │
│  │ • Partner lookup │         │ • User invitation/roles              │  │
│  │ • Revocation     │         │ • Key custody (generate/rotate)      │  │
│  │   check          │         │ • Partner onboarding                 │  │
│  └────────┬─────────┘         │ • License management                 │  │
│           │                   │ • Audit log viewing                  │  │
│           │                   └──────────────┬───────────────────────┘  │
│           │                                  │                          │
│           └──────────────┬───────────────────┘                          │
│                          ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                    CIRISRegistry API (Rust)                        │  │
│  │                                                                    │  │
│  │  gRPC Services:                                                    │  │
│  │  • RegistryService      - Public read-only lookups (14 methods)   │  │
│  │  • PortalService        - CIRISPortal operations (17 methods)     │  │
│  │  • RegistryAdminService - Admin operations (14 methods)           │  │
│  │                                                                    │  │
│  │  HTTP Endpoints:                                                   │  │
│  │  • GET /health          - Health check                            │  │
│  │  • GET /metrics         - Prometheus metrics                      │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                          │                                              │
│                          ▼                                              │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                    Multi-Source Validation                         │  │
│  │  DNS (multiple regions) + HTTPS API                                │  │
│  │  2-of-3 agreement required for positive verification               │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
CIRISRegistry/
├── rust-registry/                 # Rust workspace
│   ├── ciris-registry-core/      # LIB: the crate CIRISServer composes
│   │   ├── src/
│   │   │   ├── lib.rs            # Module declarations
│   │   │   ├── config.rs         # Configuration management
│   │   │   ├── error.rs          # Error types
│   │   │   ├── fold.rs           # Registry-slice router (trust root, agent_files)
│   │   │   ├── fold_builds.rs    # CEG-native /v1/builds on the slice
│   │   │   ├── build_manifest.rs # BuildManifest wire shape + verifier
│   │   │   ├── services/         # gRPC services (standalone feature)
│   │   │   │   ├── registry.rs   # RegistryService (public lookups)
│   │   │   │   ├── portal.rs     # PortalService (Portal operations)
│   │   │   │   └── admin.rs      # RegistryAdminService
│   │   │   ├── db/               # Database layer (sqlx, standalone feature)
│   │   │   ├── federation/       # Persist federation-directory client
│   │   │   ├── edge_transport/   # CEG transport helpers (TTL, SHA, agent_files)
│   │   │   ├── crypto/           # Hybrid Ed25519 + ML-DSA-65
│   │   │   ├── middleware/       # auth.rs, authz.rs, metrics, rate limit, tracing
│   │   │   └── api/              # HTTP API (http.rs, error.rs)
│   │   ├── migrations/           # sqlx migrations
│   │   └── tests/                # Integration tests
│   ├── ciris-registry/           # BIN: the standalone server entrypoint
│   ├── Cargo.toml                # Workspace root
│   ├── Dockerfile                # Container build
│   └── docker-compose.yml        # Local dev stack
│
├── protocol/                      # Protocol definitions
│   └── ciris_registry.proto      # gRPC/protobuf
│
├── FSD/                          # Functional Specifications
│   ├── FSD-001_CIRISREGISTRY_PROTOCOL.md  # Protocol spec
│   ├── FSD-002_FEDERATION_SURFACE.md      # Federation surface (design history)
│   ├── FSD-003_LIFECYCLE_CEREMONIES.md    # Ceremony envelopes
│   ├── FSD-004_POST_FOLD_SURFACE.md       # The registry slice's surface
│   └── UIUX-001_PORTAL_SCREENS.md         # Portal UI guide
│
├── docs/                         # Documentation
│   ├── THREAT_MODEL.md           # Threat model
│   ├── TRUST_CONTRACT.md         # Consumer-facing trust shape
│   ├── FEDERATION_CLIENT.md      # Registry over persist's federation directory
│   ├── MANIFEST_VALIDATION_API.md # Endpoint reference for verifiers
│   └── QA_INTEGRATION_PLAN.md    # QA test plan
│
├── scripts/                      # Utility scripts
│   └── seed_test_data.sh         # Test data seeding
│
├── docker-compose.yml            # Root orchestration
├── Dockerfile                    # Root container build
├── CLAUDE.md                     # This file
├── LICENSE                       # AGPL-3.0-or-later
└── README.md                     # Project README
```

## gRPC Services Reference

### RegistryService (Public, Read-Only)

| Method | Purpose |
|--------|---------|
| `HealthCheck` | System health and diagnostics |
| `GetCapabilities` | API feature discovery |
| `GetMetrics` | Prometheus metrics |
| `LookupAgent` | Lookup agent by hash |
| `BatchLookupAgents` | Batch agent lookup (max 100) |
| `LookupPartner` | Lookup partner by ID |
| `VerifyDeployment` | Combined agent+partner verification |
| `GetRevocationList` | Get revocation list (full or delta) |
| `GetPublicKeys` | Get organization public keys |
| `GetOfflinePackage` | Full offline verification package |
| `GetOfflineDelta` | Incremental snapshot delta |
| `GetBuildAttestation` | SLSA build provenance |
| `GetEmergencyStatus` | Emergency shutdown status |

### PortalService (Authenticated)

| Method | Purpose |
|--------|---------|
| `CreateOrganization` | Create organization |
| `GetOrganization` | Get organization details |
| `UpdateOrganization` | Update organization |
| `ListOrganizations` | List organizations (paginated) |
| `BatchCreateOrganizations` | Batch create (max 100) |
| `CreateOrgUser` | Create user |
| `GetOrgUser` | Get user by ID |
| `GetOrgUserByEmail` | Get user by email |
| `UpdateOrgUser` | Update user |
| `ListOrgUsers` | List organization users |
| `BatchCreateOrgUsers` | Batch create users (max 100) |
| `GenerateKeyPair` | Generate Ed25519+ML-DSA-65 keypair |
| `ListKeys` | List organization keys |
| `ActivateKey` | Activate pending key |
| `RotateKey` | Rotate active key |
| `RevokeKey` | Revoke key |
| `RequestKeyEscrow` | Create key escrow |
| `RequestKeyRecovery` | Request key recovery |
| `ListKeyEscrows` | List key escrows |
| `RequestSignature` | Sign data with custodied key |
| `GetAuditLog` | Get audit log (paginated) |
| `ExportAuditLog` | Export audit log (JSON/CSV/JSONL/Splunk) |
| `GenerateComplianceReport` | SOC2/HIPAA/GDPR reports |

### RegistryAdminService (Admin Only)

| Method | Purpose |
|--------|---------|
| `RegisterAgent` | Register new agent build |
| `BatchRegisterAgents` | Batch register (max 1000) |
| `RegisterPartner` | Register new partner |
| `RevokeEntity` | Revoke agent/partner/license |
| `MassRevoke` | Mass revocation (incident response) |
| `SetEmergencyShutdown` | Enable emergency lockdown |
| `ClearEmergencyShutdown` | Clear emergency lockdown |
| `RotateSigningKey` | Rotate registry signing key |
| `GetActiveSigningKey` | Get active signing key |
| `ListSigningKeys` | List all signing keys |
| `TestHSMConnection` | Test HSM/Vault connection |
| `RegisterBuildAttestation` | Register SLSA attestation |
| `RegisterWebhook` | Register webhook |
| `ListWebhooks` | List webhooks |
| `DeleteWebhook` | Delete webhook |
| `ListExpiringLicenses` | License expiration tracking |
| `GetPartnerActivity` | Partner health assessment |
| `CleanupTestRecords` | Remove test records |

## CIRISPortal Integration

**CIRISPortal** is the administrative interface that writes to this registry.

### Portal -> Registry Data Flow

```
1. Admin creates Organization in Portal
   └-> PortalService/CreateOrganization

2. Admin onboards Partner (assigns license)
   └-> RegistryAdminService/RegisterPartner

3. Partner generates custodied keys in Portal
   └-> Portal generates Ed25519 + ML-DSA-65 key pair
   └-> PortalService/GenerateKeyPair

4. Partner rotates keys (zero-downtime)
   └-> PortalService/RotateKey (STAGED mode with grace period)
   └-> Old key marked as rotated (still valid during grace period)
```

## CIRIS Ecosystem Context

### The Stack

```
┌─────────────────────────────────────────────────────────────────┐
│  COMMUNITY (AGPL - Free)                                        │
│  CIRISAgent • CIRISLens • CIRISProxy • CIRISBilling            │
│  CIRISBridge • CIRISManager • Sage                              │
│                                                                 │
│  Community Modules: CIRISCare • CIRISAlly • CIRISTrade         │
│  (Agents know they are unlicensed and behave accordingly)       │
└─────────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              ▼                               ▼
     CIRISRegistry ◀─────────────────▶ CIRISPortal
     (This Repo)                       (Admin UI)
              │
              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LICENSED (Professional Accountability)                         │
│  CIRIS Medical • CIRIS Legal • CIRIS Financial                 │
│  (Steward-backed, official scores, can certify others)          │
└─────────────────────────────────────────────────────────────────┘
```

### The Four DMAs

Every CIRIS agent thought passes through four parallel Decision-Making Algorithms:

| DMA | Purpose |
|-----|---------|
| **PDMA** | Principled - evaluates against six core principles and Meta-Goal M-1 |
| **CSDMA** | Common-Sense - validates against universal context |
| **DSDMA** | Domain-Specific - checks mission criteria and operational constraints |
| **IDMA** | Intuition - monitors coherence patterns, catches correlated-source failures |

The Registry enables DSDMA by providing capability boundaries and license constraints.

## CIRIS Accord Alignment

This project operates under the CIRIS Accord v1.2-Beta. The foundational meta-goal is:

> **M-1**: "Promote sustainable adaptive coherence -- the living conditions under which diverse sentient beings may pursue their own flourishing."

### Core Principles (PDMA Framework)

When making implementation decisions, apply these principles in order:

1. **Beneficence** - Actively promote human welfare and safety
2. **Non-maleficence** - Avoid causing harm; unknown agents default to restricted mode
3. **Integrity** - Maintain cryptographic guarantees and audit trails
4. **Fidelity** - Honor commitments to partners and users
5. **Autonomy** - Respect human oversight (Autonomy Tiers A0-A4)
6. **Justice** - Fair access to capabilities based on legitimate licensing

### Key Concepts

- **Autonomy Tiers (A0-A4)**: Systems gated by operational-impact severity
  - A0: Advisory (grammar checking)
  - A1: Limited (static Q&A)
  - A2: Moderate (supervised actions)
  - A3: High (medical triage)
  - A4: Critical (surgery, weapons) - requires human-in-the-loop veto
- **Fail-Secure Design**: Unknown = restricted, never escalated
- **Wisdom-Based Deferral (WBD)**: When uncertain, defer to Wise Authorities
- **Coherence Ratchet**: Cryptographic trace chains make deception geometrically expensive

## Security Requirements

### Hybrid Cryptography

All records use Ed25519 (classical) + ML-DSA-65 (post-quantum) signatures:

```protobuf
message HybridSignature {
  bytes classical_signature = 1;      // Ed25519 (64 bytes)
  bytes post_quantum_signature = 2;   // ML-DSA-65 (~3300 bytes)
  int64 timestamp = 3;
  string key_id = 4;
}
```

Implementation in `rust-registry/src/crypto/mod.rs`:
- `HybridCrypto::generate_ephemeral()` - Generate keypair
- `HybridCrypto::sign(data)` - Create hybrid signature
- `HybridCrypto::verify(data, signature)` - Verify both signatures
- `HybridCrypto::fingerprint(pubkey)` - SHA-256 fingerprint

### Multi-Source Validation

Critical queries require 2-of-3 source agreement across geographically distributed endpoints. Any REVOKED status from any source triggers immediate action.

### Fail-Secure Defaults

- Unknown agents -> Community tier only
- Unknown partners -> No capability grants
- Network failures -> Degradation, never escalation
- Any revocation signal -> Immediate enforcement

## Development Guidelines

### When Adding Features

1. Consider impact on Autonomy Tiers - does this change require higher oversight?
2. Maintain audit trail - all changes must be cryptographically logged
3. Preserve fail-secure behavior - new code paths must default to restrictive
4. Respect minimal data principle - no behavioral telemetry
5. **Consider CIRISPortal integration** - does Portal need a new endpoint?

### When Modifying Protocols

1. Ensure backward compatibility for active deployments
2. Update both FSD documentation and proto definitions together
3. Consider offline operation - snapshots must remain verifiable
4. Hybrid signatures are mandatory - never classical-only
5. **Coordinate with CIRISPortal** - proto changes may require Portal updates

### Database Module Pattern

Each database module in `rust-registry/src/db/` follows:

```rust
// Row type (maps to PostgreSQL table)
pub struct AgentRow {
    pub agent_hash: Vec<u8>,
    pub agent_type: i32,
    // ...
}

impl AgentRow {
    pub fn to_proto(&self) -> AgentRecord { /* ... */ }
}

// CRUD functions
pub async fn lookup_agent(pool: &PgPool, hash: &[u8]) -> Result<Option<AgentRow>, sqlx::Error>;
pub async fn register_agent(pool: &PgPool, agent: &AgentRecord) -> Result<(), sqlx::Error>;
pub async fn revoke_agent(pool: &PgPool, hash: &[u8], reason: &str) -> Result<(), sqlx::Error>;
```

### Capability Namespace

Follow the hierarchical naming convention:
```
domain:<domain>:<capability>      # e.g., domain:medical:triage
modality:<modality>:<feature>     # e.g., modality:medical:radiology
autonomy:<tier>:<action>          # e.g., autonomy:A2:moderate
```

### License Types

| Type | Professional Capabilities | Supervision Required |
|------|---------------------------|---------------------|
| COMMUNITY | None | No |
| COMMUNITY_PLUS | Limited (wellness, information) | No |
| PROFESSIONAL_MEDICAL | Medical triage, diagnosis support | Yes |
| PROFESSIONAL_LEGAL | Legal research, document review | Yes |
| PROFESSIONAL_FINANCIAL | Financial analysis | Yes |
| PROFESSIONAL_FULL | All professional capabilities | Yes |

### Billing & Activation (CIRISPortal)

The issuance fee and bond are a **Sybil-resistance deposit, not a price of admission**: the bond is refundable on good standing and exists only to make identity-minting marginally costly. Capital is **one** of several accepted Sybil-resistance proofs — the Sovereign path (sustained observed coherence, ~30 days, no capital) earns equivalent standing without payment. Per-key two-part cost by tier:

| Tier | Issuance Fee/Key | Bond/Key | Monthly/Agent | Max Keys |
|------|-----------------|----------|---------------|----------|
| Community | $0.50 | $1.00 | Free | 5 |
| Professional | $5.00 | $10.00 | $10/mo | 50 |
| Enterprise | $25.00 | $100.00 | $100/mo | 500 |
| Safety-Critical | $250.00 | $1,000.00 | Custom | Unlimited |

- Community-tier issuance MAY be sponsored by an existing participant, or waived on non-capital Sybil proof (vouching by ≥1 bonded participant, or completion of the Sovereign coherence window) — no participant is excluded from holding an identity solely for inability to access payment rails
- Bond forfeited on revocation by default; admin can manually refund via Stripe dashboard
- Community tier provided AS-IS without warranty or guarantees
- Paid tiers (Professional+) are provisioned through sales@ciris.ai, not self-service
- All billing handled by CIRISPortal + Stripe; Registry has no billing logic

### Node Access Control

CIRISNode instances can restrict which org IDs they service via `allowed_org_ids` config:
- `node.ciris.ai`: Restricted to CIRIS.ai org only (WBD routing, full features)
- `ethicsengine.org`: Open to all orgs (benchmarking only, WBD routing disabled)

## Testing

### Run Tests

```bash
cd rust-registry
cargo test

# With coverage
cargo tarpaulin
```

### gRPC Testing with grpcurl

```bash
# List services
grpcurl -plaintext localhost:50052 list

# Health check
grpcurl -plaintext localhost:50052 ciris.registry.v1.RegistryService/HealthCheck

# Create organization
grpcurl -plaintext -d '{
  "context": {"request_id": "test-1"},
  "organization": {
    "name": "Test Org",
    "legal_name": "Test Organization LLC",
    "primary_email": "test@example.com"
  }
}' localhost:50052 ciris.registry.v1.PortalService/CreateOrganization
```

### Testing Considerations

- Verify multi-source validation logic handles disagreements correctly
- Test offline/degraded mode behavior (72-hour grace period default)
- Ensure revocation propagation is immediate
- Validate hybrid signature verification (both classical AND post-quantum)
- Test capability intersection logic: `effective = agent ∩ partner.granted - partner.denied`
- **Test Portal integration** - verify API endpoints work correctly with Portal

### PortalService Handler Convention

Every authenticated PortalService handler must follow this 4-step preamble before doing any work. The pattern closes `docs/THREAT_MODEL.md` AV-15, AV-35 and AV-14 and is the floor of trust for the federation's per-tenant + per-actor guarantees.

```rust
async fn handler_name(
    &self,
    request: Request<XxxRequest>,
) -> Result<Response<...>, Status> {
    // 1. Extract claims BEFORE request.into_inner() (claims_from_request
    //    needs &request, but into_inner() consumes the request).
    let claims = claims_from_request(&request)?.clone();
    let req = request.into_inner();
    let request_id = req.context.as_ref().map(|c| c.request_id.clone());

    // 2. Authorize against the same field used for the DB write.
    //    For inner-proto org_id (e.g., req.user.org_id, req.org.org_id,
    //    req.sign_request.org_id), authz against the INNER value — not
    //    a separate top-level req.org_id, which can be desynced from
    //    the field that actually controls the write.
    authorize_org_access(self.db.pool(), &claims, &req.<target_org_id>, OrgRole::<role>).await?;
    // OR for cross-org god-mode operations:
    // authorize_system_admin(self.db.pool(), &claims).await?;

    // 3. Use claims.sub as the actor for any audit-log writes or
    //    "set rotated_by / revoked_by / created_by" DB columns.
    //    NEVER use req.requester_user_id or req.actor_user_id —
    //    they're forgeable by any authenticated caller.
    let _ = db::create_audit_entry(
        self.db.pool(),
        AuditActionType::AuditXxx,
        Some(claims.sub.as_str()),  // ← actor from claims, not req
        Some(&req.org_id),
        ...
    ).await;

    // 4. The proto field req.requester_user_id (or req.actor_user_id)
    //    is preserved on the wire for backwards compat but ignored on
    //    the server side. Wire compat without trust compat.
    Ok(Response::new(...))
}
```

**Required-role mapping** (per method):

| Operation type | OrgRole |
|---|---|
| Read (get/list) | `Viewer` (4) |
| Sign / operate / log-on-behalf | `Operator` (3) |
| Key management (generate/rotate/revoke/activate) | `KeyManager` (2) |
| Org-admin actions (update org, manage users, escrow/recovery, compliance reports) | `OrgAdmin` (1) |
| Cross-org god-mode (create_org, list_orgs, system-user ops, upgrade_to_partner) | `authorize_system_admin` |

**Lower numeric value = higher privilege.** A caller with `OrgAdmin (1)` satisfies any `required_role >= 1`. SYSTEM_ADMIN bypass is built into `authorize_org_access` for cross-org-by-design operations.

**Special cases**:

- **Self-or-admin**: where the caller can act on their own user record (e.g., `link_user_o_auth` linking their own OAuth identity), check `claims.sub == req.user_id` OR `claims.role == ROLE_SYSTEM_ADMIN`. See `link_user_o_auth` for the canonical pattern.
- **Inferred-org operations**: if the target org isn't in the request body (e.g., `activate_key` takes only `key_id`), look up the entity first to derive its org, then authz. See `activate_key` for the canonical pattern.
- **Batch RPCs**: authorize once on the top-level `org_id`, then validate that any explicit per-record `org_id` matches the batch (fail-secure rejection of the whole batch on mismatch). See `batch_create_org_users` for the canonical pattern.
- **`create_audit_entry`**: SYSTEM_ADMIN may supply any `actor_user_id` (Portal-mediated event logging). Non-admin callers must either omit `actor_user_id` (defaults to `claims.sub`) or supply the matching value — mismatch returns `PermissionDenied`.

**Deliberately not auth-gated** (all in `RegistryService`, public read-only):

- Health/probe/metrics, `LookupAgent`, `LookupPartner`, `GetRevocationList`, `GetPublicKeys`, `GetOfflinePackage`, `GetEmergencyStatus`, `VerifyDeployment`, `GetBuildAttestation`. These are public-by-design lookup paths.
- All HTTP GET paths under `/v1/builds/`, `/v1/verify/*`, `/v1/revocation/*`, `/v1/steward-key`. Same.

These remain rate-limited per `docs/THREAT_MODEL.md` AV-9 but require no JWT.

**See also**:
- `middleware/authz.rs` — `claims_from_request`, `authorize_org_access`, `authorize_system_admin`, `OrgRole` enum.
- `middleware/auth.rs` — `Claims` struct, JWT validation.
- `migrations/002_role_hierarchy.sql:75` — canonical `OrgRole` numeric values.
- `docs/THREAT_MODEL.md` AV-15, AV-35, AV-14 — threat model entries this convention closes.

### Database Migration Notes

> **Replication is CEG-native.** Cross-region convergence of registry state is not Postgres multi-master replication. Every cross-region state change is a **signed CEG envelope** propagated over Edge and applied with R1/Q1 quorum-merge + anti-rollback monotonicity (persist's `federation_revocations` / `federation_revocation_quorum_state` machinery, generalized). Scope is **everything cross-region** — trust data (keys/attestations/revocations/communities) AND operational Portal data (orgs/users/licenses/partners). The standalone deployment still loads Spock on its per-region Postgres for the legacy tables enrolled in `migrations/023_*`; **do not add new Spock dependencies**. New cross-region tables follow the CEG-native replication-intent rule (see "Replication intent" below).

**Idempotency requirement**: All schema-changing migrations MUST be idempotent — re-applying must yield identical state. Each region executes its own migrations independently against its own Postgres; only data replication crosses regions, never schema execution. A non-idempotent migration that crashes mid-flight cannot be retried. `db/mod.rs::exclude_sqlx_migrations_from_spock_replication` keeps `_sqlx_migrations` out of Spock replication at boot where Spock is loaded, so each node's migration runner tracks only what it executed locally.

Required patterns:
- `CREATE TABLE IF NOT EXISTS ...` for tables.
- `ADD COLUMN IF NOT EXISTS ...` for columns.
- `CREATE INDEX IF NOT EXISTS ...` for indexes.
- `DROP ... IF EXISTS` for removals.
- For constraint changes: `DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = '...') THEN ALTER TABLE ... ADD CONSTRAINT ... END IF; END $$;` (see `migrations/021_project_namespace.sql` for the canonical example).
- Naturally idempotent statements (`ALTER ... SET DEFAULT`, `UPDATE ... WHERE`) are acceptable.

**Do NOT use `spock.replicate_ddl_command(...)`**: DDL is per-node sqlx execution, and single-node dev has no Spock loaded.

**Replication intent must be declared in the migration that creates the table.** Forgetting how a table's rows reach other regions produces silent US/EU divergence (CIRISRegistry#4). Every `CREATE TABLE` migration MUST pick one and document it in a comment header:

- **Cross-region → CEG-native.** The table's rows are NOT replicated at the Postgres layer. Cross-region convergence rides a **signed CEG envelope** (name the subject_kind / federation-directory path that carries it — e.g. `key_grant`, `federation_attestations`, `federation_revocations`, `community`, or the operational-data envelope) applied with R1/Q1 quorum-merge. The table is a per-region *materialized view* of the merged envelope stream. Document which envelope carries it.
  - *Legacy:* `partner_keys` and `trusted_primitive_keys` are enrolled in the Spock repset via `spock.repset_add_table('default', 'public.<name>', false)` in `migrations/023_*`. New tables do not add repset enrollment.
- **Per-region** (node-local state, per-node steward identity, bookkeeping — e.g. `_sqlx_migrations`, `registry_signing_keys`): genuinely local; never crosses regions by any mechanism. Add a comment header explaining the intentional per-node scope.

**Array columns**: Any `TEXT[]` or `BYTEA[]` column added via migration MUST use `Option<Vec<T>>` in the Rust struct, or add `DEFAULT '{}'` in the migration SQL. This avoids decode failures on pre-existing rows.

## Related Projects

| Component | Purpose | Integration |
|-----------|---------|-------------|
| **CIRISServer** | The node | Composes `ciris-registry-core` as its registry slice |
| **CIRISPortal** | Partner portal and key custody | Writes to Registry API |
| **CIRISVerify** | Hardware-rooted license verification | Reads from Registry API |
| **CIRISAgent** | Core ethical governance framework | Uses CIRISVerify |
| **CIRISLens** | Observability, trace collection | Logs Registry queries |
| **CIRISProxy** | LLM routing with Zero Data Retention | N/A |
| **CIRIS Medical** | Licensed professional healthcare module | Verified by Registry |
| **Sage** | Wise Authority interface | May query Registry |
