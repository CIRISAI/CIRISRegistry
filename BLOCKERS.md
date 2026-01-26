# CIRISRegistry - Blockers & Dependencies

**Last Updated:** 2026-01-26

---

## Progress Summary

| Component | Status |
|-----------|--------|
| Proto definitions | ✅ Complete (organizations, users, keys, audit, gRPC services) |
| Database schema | ✅ Complete (7 migration files) |
| Migration runner | ✅ Complete (auto-runs on startup) |
| API scaffolding | ✅ Complete (repository + service layers) |
| Terraform | ✅ Complete (Vultr US + Hetzner EU) |
| Ansible | ✅ Complete (needs app deployment testing) |
| ML-DSA-65 signing | 🔴 BLOCKER |
| Repository implementations | 🟡 Stubs only |

---

## Critical Blockers

### 1. ML-DSA-65 (Post-Quantum) Library

**Status:** 🔴 BLOCKER

No production-ready ML-DSA-65 implementation integrated:

| Platform | Options | Status |
|----------|---------|--------|
| Go (API server) | `circl` (Cloudflare) | Available, needs integration |
| Cloudflare Workers (Portal) | None native | Need WASM build or server proxy |
| Python (testing) | `liboqs-python` | Available |

**Recommended Path:**
1. Use Cloudflare's `circl` library for Go implementation
2. Portal uses API server for signing (custodied keys)

**Effort:** Medium (1-2 weeks)

---

### 2. Repository Implementations

**Status:** 🟡 Stubs Only

Repository interfaces defined, but implementations are stubs (`ErrNotImplemented`).

**Files:**
- `internal/repository/repository.go` - interfaces and types
- `internal/repository/stubs.go` - stub implementations

**Needs:**
- SQL queries for CRUD operations
- Pagination cursor encoding/decoding
- Error handling for not-found cases

**Effort:** Medium (3-5 days)

---

### 3. gRPC Server Wiring

**Status:** 🟡 Not Connected

Service implementations exist but aren't wired to gRPC server:

**Files:**
- `internal/service/registry.go` - RegistryService
- `internal/service/admin.go` - AdminService
- `internal/service/portal.go` - PortalService

**Needs:**
1. Generate Go code from proto: `make proto`
2. Wire services to gRPC server in `cmd/registry/main.go`
3. Add authentication middleware

**Effort:** Low (1-2 days)

---

## Dependencies

### CIRISPortal → CIRISRegistry

Portal needs these Registry gRPC endpoints (defined in proto):

| Service | Method | Priority | Status |
|---------|--------|----------|--------|
| PortalService | CreateOrganization | P0 | Service stub ready |
| PortalService | GetOrganization | P0 | Service stub ready |
| PortalService | ListOrganizations | P0 | Service stub ready |
| PortalService | CreateOrgUser | P0 | Service stub ready |
| PortalService | ListOrgUsers | P0 | Service stub ready |
| PortalService | GenerateKeyPair | P0 | Service stub ready |
| PortalService | RequestSignature | P0 | Service stub ready |
| RegistryService | GetPublicKeys | P1 | Service stub ready |
| RegistryService | LookupPartner | P1 | Service stub ready |

### CIRISVerify → CIRISRegistry

Verify needs read-only endpoints:

| Service | Method | Priority | Status |
|---------|--------|----------|--------|
| RegistryService | LookupAgent | P0 | Service stub ready |
| RegistryService | LookupPartner | P0 | Service stub ready |
| RegistryService | VerifyDeployment | P0 | Service stub ready |
| RegistryService | GetRevocationList | P0 | Service stub ready |
| DNS TXT records | - | P0 | Terraform ready |

---

## Completed Components

### Proto Definitions ✅

**File:** `protocol/ciris_registry.proto`

Added for CIRISPortal integration:
- `Organization` message
- `OrgUser` message
- `OrgRole` enum
- `PartnerKeyRecord` message
- `KeyStatus` enum
- `KeyCustodyModel` enum
- `PublicKeys` message
- `SignRequest` / `SignResponse` messages
- `AuditEntry` / `AuditActionType` messages
- CRUD request/response messages for all entities
- gRPC service definitions: `RegistryService`, `RegistryAdminService`, `PortalService`

### Database Schema ✅

**Directory:** `internal/database/migrations/`

| Migration | Description |
|-----------|-------------|
| 001_enums_and_extensions.sql | PostgreSQL extensions, all enum types |
| 002_agent_registry.sql | `agents` table |
| 003_partner_registry.sql | `partners`, `revocations`, `registry_snapshots` tables |
| 004_organizations.sql | `organizations`, `org_users`, `user_sessions` tables |
| 005_key_management.sql | `partner_keys`, `signing_log` tables |
| 006_audit_log.sql | `audit_log`, `schema_migrations` tables |
| 007_indexes_and_triggers.sql | Performance indexes, updated_at triggers |

### Migration Runner ✅

**Files:**
- `internal/database/migrator.go` - embeds and runs migrations on startup
- `internal/database/db.go` - connection helper with `ConnectAndMigrate()`

Features:
- Automatic migration on server startup
- Checksum verification (detects modified migrations)
- Transactional migrations
- Migration status reporting

### Infrastructure ✅

**Terraform:**
- [x] Vultr US region (New Jersey)
- [x] Hetzner EU region (Nuremberg)
- [x] Cloudflare DNS records
- [x] Cloudflare Load Balancer
- [x] SSH key provisioning

**Ansible:**
- [x] System setup (packages, firewall, user)
- [x] PostgreSQL installation and replication config
- [x] Nginx with SSL (Let's Encrypt)
- [x] Systemd service template
- [x] CIRISBridge Vault integration

---

## Vault Secrets Required

For CIRISBridge deployment, add to `secret/data/ciris/registry`:

```yaml
db_password: "<generated>"
db_replication_password: "<generated>"
registry_signing_key_ed25519: "<base64 Ed25519 private key>"
registry_signing_key_mldsa65: "<base64 ML-DSA-65 private key>"
admin_api_key: "<generated>"
cloudflare_api_token: "<from CF dashboard>"
cloudflare_zone_id: "<ciris.ai zone ID>"
```

---

## Build & Run

```bash
# Generate proto
make proto

# Build
make build

# Run locally (requires PostgreSQL)
export DB_HOST=localhost
export DB_PASSWORD=ciris_dev
./bin/registry

# Start dev database (Docker)
make db-start
```

---

## Next Steps

1. **Implement repository layer** - Replace stubs with SQL queries
2. **Integrate `circl`** - Add ML-DSA-65 signing via Cloudflare's library
3. **Wire gRPC server** - Connect services to generated proto code
4. **Add authentication** - OAuth token verification middleware
5. **Deploy to staging** - Test with CIRISPortal

---

## Resolved Questions

1. **API Framework:** Go + gRPC ✅
2. **Key Custody:** Portal uses Registry API for signing (custodied in Cloudflare KV) ✅
3. **Replication Strategy:** PostgreSQL active/active managed by bridge team ✅
4. **Database:** PostgreSQL 15+ ✅
