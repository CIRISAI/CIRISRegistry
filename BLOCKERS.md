# CIRISRegistry - Status & Roadmap

**Last Updated:** 2026-01-26

---

## Implementation Status

### Core Services - COMPLETE

| Component | Status | Notes |
|-----------|--------|-------|
| **Rust Implementation** | ✅ Complete | Migrated from Go, fully functional |
| **RegistryService** | ✅ 12 endpoints | Public read-only API |
| **RegistryAdminService** | ✅ 18 endpoints | Admin operations |
| **PortalService** | ✅ 17 endpoints | CIRISPortal backend |
| **Database Layer** | ✅ 13 modules | Full CRUD + batch operations |
| **Hybrid Cryptography** | ✅ Complete | Ed25519 + ML-DSA-65 signatures |
| **Property-Based Testing** | ✅ 61 tests | Proptest (Hypothesis equivalent) |

### Infrastructure - COMPLETE

| Component | Status | Notes |
|-----------|--------|-------|
| **Proto Definitions** | ✅ Complete | All messages and services defined |
| **Database Schema** | ✅ 8 migrations | Auto-runs on startup |
| **Docker Compose** | ✅ Working | Development environment |
| **Ansible Playbooks** | ✅ Complete | staging, production, canary vars |
| **Security Hardening** | ✅ Complete | Production secret validation |

---

## What Works Now

### Full Functionality (No External Dependencies)

1. **Agent Management**
   - RegisterAgent / BatchRegisterAgents
   - LookupAgent / BatchLookupAgents
   - ListRegisteredAgents (with filtering, pagination)
   - RevokeEntity / MassRevoke

2. **Partner Management**
   - RegisterPartner
   - LookupPartner
   - VerifyDeployment (agent+partner effective capabilities)

3. **Organization & User Management**
   - Full CRUD for Organizations
   - Full CRUD for Users
   - Batch operations (max 100)
   - Role-based access

4. **Key Management (Custodied)**
   - GenerateKeyPair (Ed25519 + ML-DSA-65)
   - RotateKey with grace period
   - RevokeKey
   - Key escrow workflow

5. **Cryptographic Operations**
   - Hybrid signatures on all responses
   - RequestSignature for custodied keys
   - Post-quantum ready (ML-DSA-65)

6. **Revocation & Emergency**
   - GetRevocationList (full + delta)
   - SetEmergencyShutdown / ClearEmergencyShutdown
   - MassRevoke by hash, version, or partner

7. **Offline Verification**
   - GetOfflinePackage (gzip, signed, 72h TTL)
   - GetOfflineDelta (incremental updates)

8. **Audit & Compliance**
   - Full audit trail
   - Export: JSON, CSV, JSONL, Splunk HEC
   - Compliance reports: SOC2, ISO27001, HIPAA

9. **Health & Observability**
   - HealthCheck with component diagnostics
   - Prometheus metrics
   - HTTP endpoints: /health, /ready, /live, /metrics

---

## Quick Start

```bash
# Start everything
docker compose up -d

# Verify health
curl http://localhost:8082/health

# List agents via gRPC
grpcurl -plaintext localhost:50052 \
  ciris.registry.v1.RegistryAdminService/ListRegisteredAgents

# Run tests (requires cargo)
cd rust-registry && cargo test
```

---

## Future Enhancements (Not Blockers)

### Nice to Have

| Feature | Priority | Status |
|---------|----------|--------|
| HSM Integration (PKCS#11) | P2 | Placeholder code exists |
| Vault Transit API | P2 | Config ready, needs implementation |
| Merkle Proofs | P3 | Stubs in place |
| Rate Limiting | P2 | Not yet implemented |
| Request Timeouts | P2 | Not yet implemented |

### CIRISVerify Integration

CIRISVerify is a **separate component** that consumes Registry data:
- Runs on client machines
- Hardware-rooted verification
- Multi-source validation (DNS + HTTPS)
- Local caching with offline support

**Registry does NOT depend on CIRISVerify.** Any gRPC client can use the Registry directly.

---

## Resolved Blockers (Historical)

### ML-DSA-65 Library - RESOLVED

**Solution:** Using `pqcrypto-mldsa` crate (Rust bindings to liboqs)

```toml
[dependencies]
pqcrypto-mldsa = "0.1"
```

Implemented in `rust-registry/src/crypto/mod.rs`.

### Go to Rust Migration - RESOLVED

**Decision:** Migrated entirely to Rust for:
- Better async performance (tokio)
- Stronger type safety
- Single-binary deployment
- Native gRPC support (tonic)

---

## Environment Configuration

All settings exposed via environment variables for Ansible:

```bash
# Required in Production
ENVIRONMENT=production
DB_PASSWORD=<from-vault>
JWT_SECRET=<min-32-chars>
DB_SSLMODE=require

# Key Storage
KEY_STORAGE_MODE=file|vault|hsm
ED25519_KEY_PATH=/path/to/key
MLDSA_KEY_PATH=/path/to/key

# See rust-registry/.env.example for full list
```

---

## Deployment Readiness

| Environment | Ready | Notes |
|-------------|-------|-------|
| Development | ✅ Yes | Docker Compose |
| Staging | ✅ Yes | Ansible vars complete |
| Production | ✅ Yes | Security hardening complete |

Production deployment requires:
1. Set `ENVIRONMENT=production`
2. Provide secure `DB_PASSWORD` (not "ciris_dev")
3. Provide secure `JWT_SECRET` (min 32 chars, no "development")
4. Configure `DB_SSLMODE=require` or `verify-full`
5. Generate and deploy signing keys

---

## Contact

- Technical: registry@ciris.ai
- Security: security@ciris.ai
- Licensing: licensing@ciris.ai
