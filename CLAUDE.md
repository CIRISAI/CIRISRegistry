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
│  │ CIRISVerify      │         │ CIRISPortal (portal.ciris.ai)        │  │
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
│  │                  api.registry.ciris.ai                             │  │
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
│  │  DNS US (registry-us.ciris.ai) + DNS EU + HTTPS API                │  │
│  │  2-of-3 agreement required for positive verification               │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
CIRISRegistry/
├── rust-registry/                 # Rust implementation
│   ├── src/
│   │   ├── main.rs               # Application entrypoint
│   │   ├── config.rs             # Configuration management
│   │   ├── error.rs              # Error types
│   │   ├── services/             # gRPC service implementations
│   │   │   ├── registry.rs       # RegistryService (public lookups)
│   │   │   ├── portal.rs         # PortalService (Portal operations)
│   │   │   └── admin.rs          # RegistryAdminService
│   │   ├── db/                   # Database layer (13 modules)
│   │   │   ├── mod.rs            # Database connection pool
│   │   │   ├── agents.rs         # Agent CRUD operations
│   │   │   ├── partners.rs       # Partner CRUD operations
│   │   │   ├── organizations.rs  # Organization management
│   │   │   ├── users.rs          # User management
│   │   │   ├── keys.rs           # Key management
│   │   │   ├── audit.rs          # Audit logging
│   │   │   ├── revocations.rs    # Revocation operations
│   │   │   ├── webhooks.rs       # Webhook configuration
│   │   │   ├── signing_keys.rs   # Registry signing keys
│   │   │   ├── build_attestations.rs  # SLSA provenance
│   │   │   ├── emergency_status.rs    # Emergency lockdown
│   │   │   ├── escrows.rs        # Key escrow tracking
│   │   │   └── snapshots.rs      # Offline verification
│   │   ├── crypto/               # Cryptography
│   │   │   └── mod.rs            # Hybrid Ed25519 + ML-DSA-65
│   │   ├── middleware/           # Request middleware
│   │   │   ├── auth.rs           # JWT/mTLS validation
│   │   │   ├── metrics.rs        # Prometheus metrics
│   │   │   └── tracing.rs        # Request tracing
│   │   └── api/                  # HTTP API
│   │       └── http.rs           # Health/metrics endpoints
│   ├── migrations/               # SQL migrations
│   ├── Cargo.toml                # Dependencies
│   ├── Dockerfile                # Container build
│   └── docker-compose.yml        # Local dev stack
│
├── protocol/                      # Protocol definitions
│   └── ciris_registry.proto      # gRPC/protobuf (1876 lines, v1.1.0)
│
├── FSD/                          # Functional Specifications
│   ├── FSD-001_CIRISREGISTRY_PROTOCOL.md  # Protocol spec
│   └── UIUX-001_PORTAL_SCREENS.md         # Portal UI guide
│
├── docs/                         # Documentation
│   └── QA_INTEGRATION_PLAN.md    # QA test plan
│
├── scripts/                      # Utility scripts
│   └── seed_test_data.sh         # Test data seeding
│
├── docker-compose.yml            # Root orchestration
├── Dockerfile                    # Root container build
├── CLAUDE.md                     # This file
└── README.md                     # Project README
```

## gRPC Services Reference

### RegistryService (Public, Read-Only)

| Method | Purpose | Proto Line |
|--------|---------|------------|
| `HealthCheck` | System health and diagnostics | 1767 |
| `GetCapabilities` | API feature discovery | 1768 |
| `GetMetrics` | Prometheus metrics | 1769 |
| `LookupAgent` | Lookup agent by hash | 1772 |
| `BatchLookupAgents` | Batch agent lookup (max 100) | 1773 |
| `LookupPartner` | Lookup partner by ID | 1776 |
| `VerifyDeployment` | Combined agent+partner verification | 1779 |
| `GetRevocationList` | Get revocation list (full or delta) | 1782 |
| `GetPublicKeys` | Get organization public keys | 1785 |
| `GetOfflinePackage` | Full offline verification package | 1788 |
| `GetOfflineDelta` | Incremental snapshot delta | 1789 |
| `GetBuildAttestation` | SLSA build provenance | 1792 |
| `GetEmergencyStatus` | Emergency shutdown status | 1795 |

### PortalService (Authenticated)

| Method | Purpose | Proto Line |
|--------|---------|------------|
| `CreateOrganization` | Create organization | 1840 |
| `GetOrganization` | Get organization details | 1841 |
| `UpdateOrganization` | Update organization | 1842 |
| `ListOrganizations` | List organizations (paginated) | 1843 |
| `BatchCreateOrganizations` | Batch create (max 100) | 1844 |
| `CreateOrgUser` | Create user | 1847 |
| `GetOrgUser` | Get user by ID | 1848 |
| `GetOrgUserByEmail` | Get user by email | 1849 |
| `UpdateOrgUser` | Update user | 1850 |
| `ListOrgUsers` | List organization users | 1851 |
| `BatchCreateOrgUsers` | Batch create users (max 100) | 1852 |
| `GenerateKeyPair` | Generate Ed25519+ML-DSA-65 keypair | 1855 |
| `ListKeys` | List organization keys | 1856 |
| `ActivateKey` | Activate pending key | 1857 |
| `RotateKey` | Rotate active key | 1858 |
| `RevokeKey` | Revoke key | 1859 |
| `RequestKeyEscrow` | Create key escrow | 1862 |
| `RequestKeyRecovery` | Request key recovery | 1863 |
| `ListKeyEscrows` | List key escrows | 1864 |
| `RequestSignature` | Sign data with custodied key | 1867 |
| `GetAuditLog` | Get audit log (paginated) | 1870 |
| `ExportAuditLog` | Export audit log (JSON/CSV/JSONL/Splunk) | 1871 |
| `GenerateComplianceReport` | SOC2/HIPAA/GDPR reports | 1874 |

### RegistryAdminService (Admin Only)

| Method | Purpose | Proto Line |
|--------|---------|------------|
| `RegisterAgent` | Register new agent build | 1801 |
| `BatchRegisterAgents` | Batch register (max 1000) | 1802 |
| `RegisterPartner` | Register new partner | 1805 |
| `RevokeEntity` | Revoke agent/partner/license | 1808 |
| `MassRevoke` | Mass revocation (incident response) | 1811 |
| `SetEmergencyShutdown` | Enable emergency lockdown | 1812 |
| `ClearEmergencyShutdown` | Clear emergency lockdown | 1813 |
| `RotateSigningKey` | Rotate registry signing key | 1816 |
| `GetActiveSigningKey` | Get active signing key | 1817 |
| `ListSigningKeys` | List all signing keys | 1818 |
| `TestHSMConnection` | Test HSM/Vault connection | 1819 |
| `RegisterBuildAttestation` | Register SLSA attestation | 1822 |
| `RegisterWebhook` | Register webhook | 1825 |
| `ListWebhooks` | List webhooks | 1826 |
| `DeleteWebhook` | Delete webhook | 1827 |
| `ListExpiringLicenses` | License expiration tracking | 1830 |
| `GetPartnerActivity` | Partner health assessment | 1831 |
| `CleanupTestRecords` | Remove test records | 1834 |

## CIRISPortal Integration

**CIRISPortal** (portal.ciris.ai) is the administrative interface that writes to this registry.

### Portal → Registry Data Flow

```
1. Admin creates Organization in Portal
   └─→ PortalService/CreateOrganization

2. Admin onboards Partner (assigns license)
   └─→ RegistryAdminService/RegisterPartner

3. Partner generates custodied keys in Portal
   └─→ Portal generates Ed25519 + ML-DSA-65 key pair
   └─→ PortalService/GenerateKeyPair

4. Partner rotates keys (zero-downtime)
   └─→ PortalService/RotateKey (STAGED mode with grace period)
   └─→ Old key marked as rotated (still valid during grace period)
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

## CIRIS Covenant Alignment

This project operates under the CIRIS Covenant v1.2-Beta. The foundational meta-goal is:

> **M-1**: "Promote sustainable adaptive coherence — the living conditions under which diverse sentient beings may pursue their own flourishing."

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

Critical queries require 2-of-3 source agreement across geographically distributed endpoints:

| Source | Endpoint | Purpose |
|--------|----------|---------|
| DNS US | registry-us.ciris.ai | Primary DNS |
| DNS EU | registry-eu.ciris.ai | Geographic redundancy |
| HTTPS | api.registry.ciris.ai | Full record access |

Any REVOKED status from any source triggers immediate action.

### Fail-Secure Defaults

- Unknown agents → Community tier only
- Unknown partners → No capability grants
- Network failures → Degradation, never escalation
- Any revocation signal → Immediate enforcement

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

## E2E QA Notes (2026-02-13)

### Known Issues Fixed

**BUG-001: NULL column decode on `permitted_actions` and `allowed_identity_templates`**

Migration `003_identity_template.sql` added these columns without `DEFAULT` or `NOT NULL`. Pre-v1.2.0 rows have NULL. Rust `FromRow` can't decode NULL into `Vec<String>`.

**Fix** (commit `f56b712`): Changed to `Option<Vec<String>>` with `.unwrap_or_default()` in:
- `src/db/agents.rs` — `permitted_actions`
- `src/db/partners.rs` — `allowed_identity_templates`
- `src/services/registry.rs` — `VerifyDeployment` references

**Pattern**: Any future `TEXT[]` or `BYTEA[]` column added via migration MUST use `Option<Vec<T>>` in the Rust struct, or add `DEFAULT '{}'` in the migration SQL. This avoids decode failures on pre-existing rows.

### Deployment Notes

- **Watchtower** polls every 60s but may need manual restart if it gets stuck
- Registry runs US+EU — both instances must be updated
- Image: `ghcr.io/cirisai/cirisregistry:latest`
- Tags: `latest` (rolling) + `main` + short SHA (immutable)

### Architecture Clarifications

- **Registry is single source of truth for keys**. Everything else verifies against Registry.
- **Signing keys for agents** are generated at CIRISPortal → stored in Registry. Private key downloaded once at generation via Portal UI, placed in agent's `data/agent_signing.key`.
- **CIRISNode** auto-discovers org_id via fingerprint lookup: `GetPublicKeys(ed25519_fingerprint=SHA256(pubkey))` returns org_id + verification status. No org_id config needed on agents.
- **Fingerprint lookup**: `GetPublicKeysRequest.ed25519_fingerprint` field added to proto. Handler in `registry.rs` calls `db::get_key_by_fingerprint()`. Returns active key matching the fingerprint with org_id in response.
- **Private key export**: `GenerateKeyPairResponse.ed25519_private_key` field returns raw 32-byte Ed25519 seed. Returned ONCE at generation, never stored in Registry.

## Related Projects

| Component | Purpose | Integration |
|-----------|---------|-------------|
| **[CIRISPortal](../CIRISPortal)** | Partner portal and key custody | Writes to Registry API |
| **CIRISVerify** | Hardware-rooted license verification | Reads from Registry API |
| **CIRISAgent** | Core ethical governance framework | Uses CIRISVerify |
| **CIRISLens** | Observability, trace collection | Logs Registry queries |
| **CIRISProxy** | LLM routing with Zero Data Retention | N/A |
| **CIRIS Medical** | Licensed professional healthcare module | Verified by Registry |
| **Sage** | Wise Authority interface | May query Registry |

## Contacts

- Technical: registry@ciris.ai
- Security: security@ciris.ai
- Licensing: licensing@ciris.ai
