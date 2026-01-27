# CIRISRegistry

The authoritative trust registry for the CIRIS AI ecosystem.

## Overview

CIRISRegistry is a gRPC-based registry service that provides:

- **Agent Verification** - Lookup and validate registered agent builds
- **Partner Authorization** - License and capability management
- **Revocation Tracking** - Real-time revocation list distribution
- **Key Custody** - Hybrid Ed25519 + ML-DSA-65 key management
- **Audit Compliance** - SOC2/HIPAA/GDPR reporting

## Quick Start

### Prerequisites

- Rust 1.75+ (for building)
- PostgreSQL 15+ (database)
- Docker & Docker Compose (recommended)

### Run with Docker

```bash
docker compose up -d
```

Services:
- gRPC API: `localhost:50052`
- HTTP Health/Metrics: `localhost:8082`
- PostgreSQL: `localhost:5434`

### Run Locally

```bash
# Start PostgreSQL
docker compose up -d postgres

# Build and run
cd rust-registry
cargo run
```

### Verify Installation

```bash
# Health check
curl http://localhost:8082/health

# gRPC health (requires grpcurl)
grpcurl -plaintext localhost:50052 ciris.registry.v1.RegistryService/HealthCheck
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Clients                                  │
│                                                                  │
│   CIRISVerify (Read)          CIRISPortal (Read+Write)          │
│   • Agent lookups             • Organization management          │
│   • Partner lookups           • User/key management             │
│   • Revocation checks         • License management              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    CIRISRegistry API                             │
│                                                                  │
│   gRPC Services (port 50052):                                   │
│   ├── RegistryService       - Public read-only (13 methods)    │
│   ├── PortalService         - Authenticated ops (23 methods)   │
│   └── RegistryAdminService  - Admin operations (18 methods)    │
│                                                                  │
│   HTTP Endpoints (port 8082):                                   │
│   ├── GET /health           - Health check                     │
│   └── GET /metrics          - Prometheus metrics               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│   PostgreSQL Database        Hybrid Cryptography                │
│   • Organizations            • Ed25519 (classical)              │
│   • Users                    • ML-DSA-65 (post-quantum)         │
│   • Agents                   • Dual signatures required         │
│   • Partners                                                    │
│   • Keys                                                        │
│   • Audit logs                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## API Reference

### RegistryService (Public)

| Method | Description |
|--------|-------------|
| `HealthCheck` | System health status |
| `GetCapabilities` | API feature discovery |
| `GetMetrics` | Performance metrics |
| `LookupAgent` | Lookup agent by SHA-256 hash |
| `BatchLookupAgents` | Batch lookup (max 100) |
| `LookupPartner` | Lookup partner by ID |
| `VerifyDeployment` | Combined agent+partner verification |
| `GetRevocationList` | Full or delta revocation list |
| `GetPublicKeys` | Organization public keys |
| `GetOfflinePackage` | 72-hour offline verification bundle |
| `GetOfflineDelta` | Incremental snapshot updates |
| `GetBuildAttestation` | SLSA build provenance |
| `GetEmergencyStatus` | Emergency shutdown status |

### PortalService (Authenticated)

| Method | Description |
|--------|-------------|
| `CreateOrganization` | Create new organization |
| `GetOrganization` | Get organization details |
| `UpdateOrganization` | Update organization |
| `ListOrganizations` | List all organizations (paginated) |
| `BatchCreateOrganizations` | Batch create (max 100) |
| `CreateOrgUser` | Invite user to organization |
| `GetOrgUser` | Get user by ID |
| `GetOrgUserByEmail` | Get user by email |
| `UpdateOrgUser` | Update user details/role |
| `ListOrgUsers` | List organization users |
| `BatchCreateOrgUsers` | Batch create users |
| `GenerateKeyPair` | Generate Ed25519+ML-DSA-65 keypair |
| `ListKeys` | List organization keys |
| `ActivateKey` | Activate pending key |
| `RotateKey` | Zero-downtime key rotation |
| `RevokeKey` | Revoke compromised key |
| `RequestKeyEscrow` | Create key backup |
| `RequestKeyRecovery` | Recover escrowed key |
| `ListKeyEscrows` | List key escrows |
| `RequestSignature` | Sign with custodied key |
| `GetAuditLog` | Query audit events |
| `ExportAuditLog` | Export JSON/CSV/JSONL/Splunk |
| `GenerateComplianceReport` | SOC2/HIPAA/GDPR reports |

### RegistryAdminService (Admin Only)

| Method | Description |
|--------|-------------|
| `RegisterAgent` | Register new agent build |
| `BatchRegisterAgents` | Batch register (max 1000) |
| `RegisterPartner` | Register licensed partner |
| `RevokeEntity` | Revoke agent/partner/license |
| `MassRevoke` | Incident response mass revocation |
| `SetEmergencyShutdown` | Enable emergency lockdown |
| `ClearEmergencyShutdown` | Clear emergency status |
| `RotateSigningKey` | Rotate registry signing key |
| `GetActiveSigningKey` | Get current signing key |
| `ListSigningKeys` | List all signing keys |
| `TestHSMConnection` | Test HSM/Vault connectivity |
| `RegisterBuildAttestation` | Register SLSA attestation |
| `RegisterWebhook` | Configure event webhook |
| `ListWebhooks` | List webhooks |
| `DeleteWebhook` | Remove webhook |
| `ListExpiringLicenses` | Track license expirations |
| `GetPartnerActivity` | Partner health assessment |
| `CleanupTestRecords` | Remove test data |

## Security

### Hybrid Cryptography

All signatures use both classical and post-quantum algorithms:

- **Ed25519** - 64-byte classical signatures
- **ML-DSA-65** - ~3300-byte post-quantum signatures (FIPS 204)

Both signatures are required for verification. This ensures:
- Current security via Ed25519
- Future-proof protection against quantum computers

### Fail-Secure Design

- Unknown agents default to community tier (no professional capabilities)
- Unknown partners receive no capability grants
- Network failures trigger graceful degradation, never escalation
- Any revocation signal from any source triggers immediate enforcement

### Multi-Source Validation

Critical deployments can verify against multiple sources:
- DNS US (registry-us.ciris.ai)
- DNS EU (registry-eu.ciris.ai)
- HTTPS API (api.registry.ciris.ai)

2-of-3 agreement required for positive verification.

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://ciris:ciris_dev@localhost:5434/ciris_registry` | PostgreSQL connection |
| `GRPC_PORT` | `50052` | gRPC server port |
| `HTTP_PORT` | `8082` | HTTP health/metrics port |
| `RUST_LOG` | `info` | Log level (trace/debug/info/warn/error) |

### Key Storage Options

1. **File-based** (development) - Keys stored in local files
2. **HashiCorp Vault** (production) - Secure secret management
3. **HSM** (high-security) - Hardware security module integration

## Development

### Project Structure

```
CIRISRegistry/
├── rust-registry/           # Rust implementation
│   ├── src/
│   │   ├── services/       # gRPC service implementations
│   │   ├── db/             # Database layer (13 modules)
│   │   ├── crypto/         # Hybrid cryptography
│   │   ├── middleware/     # Auth, metrics, tracing
│   │   └── api/            # HTTP endpoints
│   └── migrations/         # SQL schema
├── protocol/               # Protobuf definitions
│   └── ciris_registry.proto
├── FSD/                    # Functional specifications
├── docs/                   # Documentation
└── scripts/                # Utility scripts
```

### Build from Source

```bash
cd rust-registry
cargo build --release
```

### Run Tests

```bash
cd rust-registry
cargo test
```

### Generate Proto Code

Proto code is auto-generated at build time via `build.rs`. To regenerate:

```bash
cd rust-registry
cargo build
```

## Documentation

- [CLAUDE.md](./CLAUDE.md) - Development guide and architecture
- [FSD-001](./FSD/FSD-001_CIRISREGISTRY_PROTOCOL.md) - Protocol specification
- [UIUX-001](./FSD/UIUX-001_PORTAL_SCREENS.md) - Portal UI/UX guide
- [QA Plan](./docs/QA_INTEGRATION_PLAN.md) - Testing strategy

## CIRIS Ecosystem

CIRISRegistry is part of the CIRIS AI governance ecosystem:

| Component | Purpose |
|-----------|---------|
| **CIRISRegistry** | Trust registry (this repo) |
| **CIRISPortal** | Admin web interface |
| **CIRISVerify** | Hardware-rooted verification |
| **CIRISAgent** | Ethical AI framework |
| **CIRISLens** | Observability platform |

## License

Copyright 2025 CIRIS L3C. All rights reserved.

## Contact

- Technical: registry@ciris.ai
- Security: security@ciris.ai
- Licensing: licensing@ciris.ai
