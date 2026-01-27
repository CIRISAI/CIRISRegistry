# CIRISRegistry v1.1.0 Comprehensive QA Plan

**Version:** 2.0.0
**Last Updated:** 2026-01-26
**Registry Version:** 1.1.0
**Total Endpoints:** 54 gRPC methods + 2 HTTP endpoints

---

## Table of Contents

1. [Environment Setup](#environment-setup)
2. [Phase 1: Infrastructure & Smoke Tests](#phase-1-infrastructure--smoke-tests)
3. [Phase 2: RegistryService (Public API)](#phase-2-registryservice-public-api)
4. [Phase 3: PortalService (Organization Management)](#phase-3-portalservice-organization-management)
5. [Phase 4: PortalService (User Management)](#phase-4-portalservice-user-management)
6. [Phase 5: PortalService (Key Management)](#phase-5-portalservice-key-management)
7. [Phase 6: PortalService (Audit & Compliance)](#phase-6-portalservice-audit--compliance)
8. [Phase 7: RegistryAdminService (Agent Management)](#phase-7-registryadminservice-agent-management)
9. [Phase 8: RegistryAdminService (Partner Management)](#phase-8-registryadminservice-partner-management)
10. [Phase 9: RegistryAdminService (Incident Response)](#phase-9-registryadminservice-incident-response)
11. [Phase 10: RegistryAdminService (Infrastructure)](#phase-10-registryadminservice-infrastructure)
12. [Phase 11: Integration & E2E Tests](#phase-11-integration--e2e-tests)
13. [Phase 12: Performance & Load Tests](#phase-12-performance--load-tests)
14. [Appendix: Test Data & Scripts](#appendix-test-data--scripts)

---

## Environment Setup

### Test Server Endpoints

| Service | Protocol | Endpoint | Purpose |
|---------|----------|----------|---------|
| gRPC API | HTTP/2 | `localhost:50052` | Primary API |
| HTTP Health | HTTP/1.1 | `localhost:8082/health` | Health checks |
| HTTP Metrics | HTTP/1.1 | `localhost:8082/metrics` | Prometheus metrics |
| PostgreSQL | TCP | `localhost:5434` | Database |

### Database Credentials (Development)

```
Host: localhost
Port: 5434
Database: ciris_registry
User: ciris
Password: ciris_dev
```

### gRPC Reflection

```bash
# List all services
grpcurl -plaintext localhost:50052 list

# Describe a service
grpcurl -plaintext localhost:50052 describe ciris.registry.v1.PortalService

# Describe a message type
grpcurl -plaintext localhost:50052 describe ciris.registry.v1.Organization
```

### Required Tools

- `grpcurl` - gRPC command-line client
- `curl` - HTTP client
- `jq` - JSON processor
- `ghz` - gRPC load testing (optional)

---

## Phase 1: Infrastructure & Smoke Tests

### 1.1 HTTP Endpoints

| Test ID | Endpoint | Method | Expected | Priority |
|---------|----------|--------|----------|----------|
| HTTP-001 | `/health` | GET | `{"status":"serving"}` | P0 |
| HTTP-002 | `/metrics` | GET | Prometheus format metrics | P0 |
| HTTP-003 | `/health` with DB down | GET | `{"status":"not_serving"}` | P1 |

### 1.2 gRPC Connectivity

| Test ID | Test Case | gRPC Method | Expected | Priority |
|---------|-----------|-------------|----------|----------|
| SM-001 | Health check | RegistryService/HealthCheck | status: HEALTH_SERVING | P0 |
| SM-002 | Health with diagnostics | HealthCheck(include_diagnostics=true) | components array populated | P1 |
| SM-003 | Get capabilities | RegistryService/GetCapabilities | protocol_version: "1.1.0" | P0 |
| SM-004 | Get metrics | RegistryService/GetMetrics | Metrics response | P1 |
| SM-005 | Service reflection | grpc.reflection | 3 services listed | P0 |

**Sample Health Check:**
```bash
grpcurl -plaintext -d '{"include_diagnostics": true}' \
  localhost:50052 ciris.registry.v1.RegistryService/HealthCheck
```

---

## Phase 2: RegistryService (Public API)

### 2.1 Agent Lookups

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| AGT-001 | Lookup non-existent agent | LookupAgent | found: false | P0 |
| AGT-002 | Lookup existing agent | LookupAgent | found: true, agent populated | P0 |
| AGT-003 | Lookup with request nonce | LookupAgent | response_signature populated | P1 |
| AGT-004 | Batch lookup empty | BatchLookupAgents([]) | Empty results | P1 |
| AGT-005 | Batch lookup 1 agent | BatchLookupAgents([hash]) | Single result | P0 |
| AGT-006 | Batch lookup max (100) | BatchLookupAgents | All 100 processed | P1 |
| AGT-007 | Batch lookup > 100 | BatchLookupAgents(101) | INVALID_ARGUMENT | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "agent_hash": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
  "request_nonce": "dGVzdC1ub25jZQ=="
}' localhost:50052 ciris.registry.v1.RegistryService/LookupAgent
```

### 2.2 Partner Lookups

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| PTR-001 | Lookup non-existent partner | LookupPartner | found: false | P0 |
| PTR-002 | Lookup existing partner | LookupPartner | found: true, partner populated | P0 |
| PTR-003 | Lookup with empty ID | LookupPartner("") | found: false | P1 |

### 2.3 Deployment Verification

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| VER-001 | Verify with unknown agent | VerifyDeployment | agent_found: false | P0 |
| VER-002 | Verify with unknown partner | VerifyDeployment | partner_found: false | P0 |
| VER-003 | Verify valid deployment | VerifyDeployment | effective_capabilities populated | P0 |
| VER-004 | Verify capability intersection | VerifyDeployment | agent ∩ partner.granted - partner.denied | P1 |

### 2.4 Revocation List

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| REV-001 | Get full revocation list | GetRevocationList | is_delta: false | P0 |
| REV-002 | Get delta (since_version) | GetRevocationList(since_version=X) | is_delta: true | P1 |
| REV-003 | Delta with no changes | GetRevocationList | Empty entries if no changes | P1 |

### 2.5 Public Keys

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| PUB-001 | Get active key by org | GetPublicKeys(org_id) | found: true, both keys | P0 |
| PUB-002 | Get key by ID | GetPublicKeys(key_id) | found: true | P1 |
| PUB-003 | Get non-existent key | GetPublicKeys | found: false | P1 |

### 2.6 Offline Verification

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| OFF-001 | Get offline package | GetOfflinePackage | package with all data | P1 |
| OFF-002 | Package signature valid | GetOfflinePackage | package_signature verifiable | P2 |
| OFF-003 | Get offline delta | GetOfflineDelta(since_timestamp) | delta populated | P1 |
| OFF-004 | Package expires correctly | GetOfflinePackage | expires_at = now + 72h | P2 |

### 2.7 Build Attestation

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| ATT-001 | Get non-existent attestation | GetBuildAttestation | found: false | P1 |
| ATT-002 | Get existing attestation | GetBuildAttestation | SLSA provenance data | P1 |

### 2.8 Emergency Status

| Test ID | Test Case | Method | Expected | Priority |
|---------|-----------|--------|----------|----------|
| EMR-001 | Get status (normal) | GetEmergencyStatus | is_locked: false | P0 |
| EMR-002 | Get status (locked) | GetEmergencyStatus | is_locked: true, details | P0 |

---

## Phase 3: PortalService (Organization Management)

### 3.1 Create Organization

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ORG-001 | Create with required fields | success: true, org_id returned | P0 |
| ORG-002 | Create with all fields | success: true | P0 |
| ORG-003 | Missing name | INVALID_ARGUMENT | P1 |
| ORG-004 | Missing email | INVALID_ARGUMENT | P1 |
| ORG-005 | Invalid email format | INVALID_ARGUMENT | P2 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-org-001"},
  "organization": {
    "name": "Test Organization",
    "legal_name": "Test Organization LLC",
    "tax_id": "12-3456789",
    "primary_email": "admin@test.org",
    "billing_email": "billing@test.org",
    "technical_contact_email": "tech@test.org",
    "compliance_contact_email": "compliance@test.org",
    "oauth_provider": "google",
    "oauth_domain": "test.org"
  }
}' localhost:50052 ciris.registry.v1.PortalService/CreateOrganization
```

### 3.2 Get Organization

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ORG-010 | Get existing org | found: true, organization populated | P0 |
| ORG-011 | Get non-existent org | found: false | P0 |
| ORG-012 | Get with empty ID | INVALID_ARGUMENT | P1 |

### 3.3 Update Organization

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ORG-020 | Update existing org | success: true | P0 |
| ORG-021 | Update non-existent org | success: false | P1 |
| ORG-022 | Partial update (name only) | success: true, other fields unchanged | P1 |

### 3.4 List Organizations

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ORG-030 | List default page | Up to 50 orgs | P0 |
| ORG-031 | List with page_size=10 | 10 orgs max | P0 |
| ORG-032 | Pagination with page_token | Next page returned | P1 |
| ORG-033 | include_inactive=false | Only active orgs | P1 |
| ORG-034 | include_inactive=true | All orgs | P1 |

### 3.5 Batch Create Organizations

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ORG-040 | Batch create 5 orgs | successful_count: 5 | P1 |
| ORG-041 | Batch create max (100) | All processed | P2 |
| ORG-042 | Batch create > 100 | INVALID_ARGUMENT | P2 |
| ORG-043 | Batch with partial failures | failed_count > 0, results detail | P2 |

---

## Phase 4: PortalService (User Management)

### 4.1 Create User

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| USR-001 | Create with required fields | success: true, user_id | P0 |
| USR-002 | Create with all fields | success: true | P0 |
| USR-003 | Invalid org_id | INTERNAL (FK violation) | P1 |
| USR-004 | Duplicate email | INTERNAL (unique violation) | P1 |
| USR-005 | Missing email | INVALID_ARGUMENT | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-user-001"},
  "user": {
    "org_id": "<ORG_ID>",
    "email": "user@test.org",
    "name": "Test User",
    "role": 100,
    "oauth_provider": "google",
    "oauth_subject": "google-oauth2|12345"
  }
}' localhost:50052 ciris.registry.v1.PortalService/CreateOrgUser
```

### 4.2 Get User

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| USR-010 | Get by ID | found: true | P0 |
| USR-011 | Get by email | found: true | P0 |
| USR-012 | Get non-existent | found: false | P0 |

### 4.3 Update User

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| USR-020 | Update name | success: true | P0 |
| USR-021 | Update role | success: true | P0 |
| USR-022 | Update non-existent | success: false | P1 |
| USR-023 | Update mfa_enabled | success: true | P1 |

### 4.4 List Users

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| USR-030 | List by org | Users in that org only | P0 |
| USR-031 | List empty org | Empty list | P1 |
| USR-032 | Pagination | Correct page navigation | P1 |
| USR-033 | include_inactive | Active + inactive users | P1 |

### 4.5 Batch Create Users

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| USR-040 | Batch create 5 users | successful_count: 5 | P1 |
| USR-041 | Batch create max (100) | All processed | P2 |
| USR-042 | Batch > 100 | INVALID_ARGUMENT | P2 |
| USR-043 | Batch with partial failures | Detailed results | P2 |

---

## Phase 5: PortalService (Key Management)

### 5.1 Generate Key Pair

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| KEY-001 | Generate for org | key_record with fingerprints | P0 |
| KEY-002 | activate_immediately=true | status: KEY_ACTIVE | P0 |
| KEY-003 | activate_immediately=false | status: KEY_PENDING | P0 |
| KEY-004 | Invalid org_id | Error | P1 |
| KEY-005 | Verify fingerprint format | 64-char hex SHA-256 | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-key-001"},
  "org_id": "<ORG_ID>",
  "requester_user_id": "<USER_ID>",
  "activate_immediately": true
}' localhost:50052 ciris.registry.v1.PortalService/GenerateKeyPair
```

### 5.2 List Keys

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| KEY-010 | List for org | All active keys | P0 |
| KEY-011 | include_revoked=true | Active + revoked | P1 |
| KEY-012 | Empty org | Empty list | P1 |

### 5.3 Activate Key

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| KEY-020 | Activate pending | success: true | P0 |
| KEY-021 | Activate already active | success: false | P1 |
| KEY-022 | Activate non-existent | success: false | P1 |

### 5.4 Rotate Key

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| KEY-030 | Rotate with IMMEDIATE mode | New key active, old rotated | P0 |
| KEY-031 | Rotate with STAGED mode | Grace period set | P0 |
| KEY-032 | Rotate no active key | FAILED_PRECONDITION | P1 |
| KEY-033 | Custom grace_period_hours | Correct expiration | P1 |
| KEY-034 | Verify audit entry created | Audit log has KEY_ROTATED | P2 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-rotate-001"},
  "org_id": "<ORG_ID>",
  "requester_user_id": "<USER_ID>",
  "mode": 1,
  "grace_period_hours": 24,
  "reason": "Scheduled rotation"
}' localhost:50052 ciris.registry.v1.PortalService/RotateKey
```

### 5.5 Revoke Key

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| KEY-040 | Revoke active key | success: true | P0 |
| KEY-041 | Revoke already revoked | success: false | P1 |
| KEY-042 | Revoke wrong org | PERMISSION_DENIED | P1 |
| KEY-043 | Verify audit entry | Audit log has KEY_REVOKED | P2 |

### 5.6 Key Escrow

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ESC-001 | Request escrow | escrow record created | P1 |
| ESC-002 | Escrow wrong org | PERMISSION_DENIED | P2 |
| ESC-003 | List escrows | All escrows for org | P1 |
| ESC-004 | Request recovery | recovery_request_id returned | P1 |
| ESC-005 | Recovery wrong org | PERMISSION_DENIED | P2 |

### 5.7 Request Signature

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| SIG-001 | Sign data with active key | Hybrid signature | P0 |
| SIG-002 | Sign with no active key | FAILED_PRECONDITION | P1 |
| SIG-003 | Sign empty data | Valid signature | P2 |
| SIG-004 | Sign 1MB data | Valid signature | P2 |
| SIG-005 | Verify signature structure | classical + post_quantum | P1 |

**Expected Signature:**
```json
{
  "signature": {
    "classical_signature": "base64 (64 bytes)",
    "post_quantum_signature": "base64 (~2420 bytes)",
    "timestamp": 1706313600,
    "key_id": "key-fingerprint"
  }
}
```

---

## Phase 6: PortalService (Audit & Compliance)

### 6.1 Get Audit Log

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| AUD-001 | Get for org | Entries for that org | P0 |
| AUD-002 | Filter by time range | Only entries in range | P0 |
| AUD-003 | Filter by action type | Only matching actions | P1 |
| AUD-004 | Pagination | Correct page navigation | P1 |
| AUD-005 | Empty result | Empty entries, total_count: 0 | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-audit-001"},
  "org_id": "<ORG_ID>",
  "start_time": 1704067200,
  "end_time": 1735689600,
  "action_types": [1, 5, 10],
  "page_size": 50
}' localhost:50052 ciris.registry.v1.PortalService/GetAuditLog
```

### 6.2 Export Audit Log

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| AUD-010 | Export as JSON | Valid JSON, content-type | P0 |
| AUD-011 | Export as CSV | Valid CSV format | P1 |
| AUD-012 | Export as JSONL | Newline-delimited JSON | P1 |
| AUD-013 | Export as Splunk HEC | HEC format events | P2 |
| AUD-014 | Export checksum | SHA-256 checksum valid | P2 |
| AUD-015 | Invalid format | INVALID_ARGUMENT | P2 |

### 6.3 Generate Compliance Report

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| CMP-001 | SOC2 report | key_management, access_control, audit | P1 |
| CMP-002 | HIPAA report | Report with PHI focus | P2 |
| CMP-003 | ISO27001 report | ISMS assessment | P2 |
| CMP-004 | Report signature | report_signature valid | P2 |
| CMP-005 | Custom time range | Data from specified period | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-compliance-001"},
  "org_id": "<ORG_ID>",
  "framework": 1,
  "start_time": 1704067200,
  "end_time": 1735689600,
  "include_sections": ["key_management", "access_control", "audit"]
}' localhost:50052 ciris.registry.v1.PortalService/GenerateComplianceReport
```

---

## Phase 7: RegistryAdminService (Agent Management)

### 7.1 Register Agent

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ADM-AGT-001 | Register new agent | success: true | P0 |
| ADM-AGT-002 | Register with all fields | success: true | P0 |
| ADM-AGT-003 | Duplicate agent hash | Error (unique) | P1 |
| ADM-AGT-004 | Missing agent_hash | INVALID_ARGUMENT | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-agent-001"},
  "agent": {
    "agent_hash": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
    "agent_type": 1,
    "version": {"major": 1, "minor": 0, "patch": 0},
    "base_capabilities": ["domain:general:chat"],
    "max_autonomy_tier": 2,
    "build_timestamp": 1706313600,
    "status": 1
  }
}' localhost:50052 ciris.registry.v1.RegistryAdminService/RegisterAgent
```

### 7.2 Batch Register Agents

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ADM-AGT-010 | Batch 10 agents | succeeded: 10 | P1 |
| ADM-AGT-011 | Batch max (1000) | All processed | P2 |
| ADM-AGT-012 | Batch > 1000 | INVALID_ARGUMENT | P2 |
| ADM-AGT-013 | Partial failures | errors array populated | P2 |

### 7.3 Build Attestation

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ADM-ATT-001 | Register attestation | success: true | P1 |
| ADM-ATT-002 | SLSA provenance data | All SLSA fields stored | P2 |

---

## Phase 8: RegistryAdminService (Partner Management)

### 8.1 Register Partner

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ADM-PTR-001 | Register new partner | success: true | P0 |
| ADM-PTR-002 | Register with capabilities | success: true | P0 |
| ADM-PTR-003 | Duplicate partner_id | Error | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "test-partner-001"},
  "partner": {
    "partner_id": "partner-001",
    "organization_name": "Test Partner Inc",
    "license_type": 3,
    "capabilities_granted": ["domain:medical:triage"],
    "capabilities_denied": [],
    "max_autonomy_tier": 2,
    "issued_at": 1706313600,
    "expires_at": 1737849600,
    "status": 1
  }
}' localhost:50052 ciris.registry.v1.RegistryAdminService/RegisterPartner
```

### 8.2 Revoke Entity

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ADM-REV-001 | Revoke agent | success: true | P0 |
| ADM-REV-002 | Revoke partner | success: true | P0 |
| ADM-REV-003 | Revoke non-existent | Error | P1 |
| ADM-REV-004 | Invalid target_type | UNIMPLEMENTED | P1 |

### 8.3 Partner Activity

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ADM-ACT-001 | Get activity for partner | Activity response | P1 |
| ADM-ACT-002 | Health status HEALTHY | Active partner | P2 |
| ADM-ACT-003 | Health status INACTIVE | No activity 30+ days | P2 |
| ADM-ACT-004 | Recommendations | Actionable suggestions | P2 |

### 8.4 Expiring Licenses

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| ADM-EXP-001 | List expiring in 30 days | Matching licenses | P1 |
| ADM-EXP-002 | Include expired | Expired + expiring | P2 |
| ADM-EXP-003 | Days remaining calc | Correct countdown | P2 |

---

## Phase 9: RegistryAdminService (Incident Response)

### 9.1 Mass Revocation

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| INC-001 | Dry run by agent hashes | dry_run_count > 0, no changes | P0 |
| INC-002 | Execute by agent hashes | revoked_count > 0 | P0 |
| INC-003 | Revoke by version prefix | Matching agents revoked | P1 |
| INC-004 | Revoke by partner IDs | Partners revoked | P1 |
| INC-005 | Mixed criteria | All criteria applied | P2 |
| INC-006 | Audit log created | Entry with incident_id | P1 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "incident-001"},
  "agent_hashes": [],
  "partner_ids": ["partner-001"],
  "agent_version_prefix": "2.1",
  "severity": 3,
  "incident_id": "INC-2026-001",
  "incident_summary": "Security vulnerability",
  "incident_details": "CVE-2026-XXXX",
  "is_dry_run": true
}' localhost:50052 ciris.registry.v1.RegistryAdminService/MassRevoke
```

### 9.2 Emergency Shutdown

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| INC-010 | Enable shutdown | enabled: true | P0 |
| INC-011 | Set duration | locked_until = now + duration | P0 |
| INC-012 | Set allowed operations | Operations in list only | P1 |
| INC-013 | Clear shutdown | success: true | P0 |
| INC-014 | Verify via GetEmergencyStatus | Status matches | P0 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "emergency-001"},
  "reason": "Security incident investigation",
  "severity": 3,
  "lock_duration_seconds": 3600,
  "allowed_operations": ["health_check", "get_emergency_status"]
}' localhost:50052 ciris.registry.v1.RegistryAdminService/SetEmergencyShutdown
```

---

## Phase 10: RegistryAdminService (Infrastructure)

### 10.1 Signing Key Management

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| INF-001 | Get active signing key | key returned | P0 |
| INF-002 | List all signing keys | All keys returned | P1 |
| INF-003 | Rotate signing key | new_key_id, old_key_id | P0 |
| INF-004 | Test HSM connection | Status response | P2 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "rotate-001"},
  "new_key_id": "key-2026-01",
  "target_storage": 1
}' localhost:50052 ciris.registry.v1.RegistryAdminService/RotateSigningKey
```

### 10.2 Webhook Management

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| WH-001 | Register webhook | success: true | P1 |
| WH-002 | HTTPS required | Validate URL scheme | P2 |
| WH-003 | List webhooks | All org webhooks | P1 |
| WH-004 | Delete webhook | success: true | P1 |
| WH-005 | Delete non-existent | NOT_FOUND | P2 |

**Sample:**
```bash
grpcurl -plaintext -d '{
  "context": {"request_id": "webhook-001", "client_version": "org-001"},
  "config": {
    "url": "https://example.com/webhook",
    "subscribed_events": ["agent.registered", "key.rotated"],
    "active": true
  }
}' localhost:50052 ciris.registry.v1.RegistryAdminService/RegisterWebhook
```

### 10.3 Test Data Cleanup

| Test ID | Test Case | Expected | Priority |
|---------|-----------|----------|----------|
| CLN-001 | Cleanup by test tag | records_removed count | P2 |
| CLN-002 | No matching records | records_removed: 0 | P2 |

---

## Phase 11: Integration & E2E Tests

### 11.1 Organization Lifecycle

| Test ID | Flow | Expected | Priority |
|---------|------|----------|----------|
| E2E-001 | Create org → Create user → Generate key → Sign data | All succeed | P0 |
| E2E-002 | Create org → Get → Update → Get | Changes persisted | P0 |
| E2E-003 | Create org → List → Pagination | Consistent results | P1 |

### 11.2 Key Lifecycle

| Test ID | Flow | Expected | Priority |
|---------|------|----------|----------|
| E2E-010 | Generate key (pending) → Activate → Sign | Success | P0 |
| E2E-011 | Generate → Rotate → Verify old rotated | Old key status: ROTATED | P0 |
| E2E-012 | Generate → Revoke → Try sign | FAILED_PRECONDITION | P0 |
| E2E-013 | Generate → Escrow → Recovery request | Recovery pending | P1 |

### 11.3 Agent/Partner Lifecycle

| Test ID | Flow | Expected | Priority |
|---------|------|----------|----------|
| E2E-020 | Register agent → Lookup → Verify deployment | All succeed | P0 |
| E2E-021 | Register partner → Lookup → Verify deployment | All succeed | P0 |
| E2E-022 | Register → Revoke → Lookup | Status: REVOKED | P0 |
| E2E-023 | Mass revoke → Lookup all | All revoked | P1 |

### 11.4 Incident Response

| Test ID | Flow | Expected | Priority |
|---------|------|----------|----------|
| E2E-030 | Set emergency → All writes blocked → Clear | Writes blocked then restored | P0 |
| E2E-031 | Mass revoke dry run → Execute → Verify | Counts match | P0 |

---

## Phase 12: Performance & Load Tests

### 12.1 Latency Targets

| Test ID | Operation | Target (p99) | Priority |
|---------|-----------|--------------|----------|
| PERF-001 | HealthCheck | < 10ms | P0 |
| PERF-002 | LookupAgent | < 50ms | P0 |
| PERF-003 | CreateOrganization | < 100ms | P1 |
| PERF-004 | ListOrganizations (100) | < 200ms | P1 |
| PERF-005 | RequestSignature | < 500ms | P1 |
| PERF-006 | GenerateKeyPair | < 2000ms | P1 |
| PERF-007 | GetOfflinePackage | < 5000ms | P2 |

### 12.2 Load Tests

| Test ID | Scenario | Target | Priority |
|---------|----------|--------|----------|
| LOAD-001 | 50 concurrent health checks | 0 errors | P0 |
| LOAD-002 | 50 concurrent lookups | 0 errors | P1 |
| LOAD-003 | 10 concurrent sign operations | 0 errors | P1 |
| LOAD-004 | Mixed workload (100 req/s) | < 1% error | P2 |

### 12.3 Load Testing Commands

```bash
# Health check load test with ghz
ghz --insecure --proto protocol/ciris_registry.proto \
    --call ciris.registry.v1.RegistryService.HealthCheck \
    -d '{}' -n 1000 -c 50 localhost:50052

# Mixed workload
ghz --insecure --proto protocol/ciris_registry.proto \
    --call ciris.registry.v1.RegistryService.LookupAgent \
    -d '{"agent_hash":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="}' \
    -n 500 -c 25 localhost:50052
```

---

## Appendix: Test Data & Scripts

### A.1 Quick Setup Script

```bash
#!/bin/bash
# scripts/seed_test_data.sh

# Create test organization
ORG_RESPONSE=$(grpcurl -plaintext -d '{
  "context": {"request_id": "seed-org"},
  "organization": {
    "name": "QA Test Organization",
    "legal_name": "QA Test Org LLC",
    "primary_email": "qa@test.org"
  }
}' localhost:50052 ciris.registry.v1.PortalService/CreateOrganization)

echo "Organization created"
ORG_ID=$(echo $ORG_RESPONSE | jq -r '.message' | grep -oP 'ID: \K[a-f0-9-]+')
echo "Org ID: $ORG_ID"

# Create test user
USER_RESPONSE=$(grpcurl -plaintext -d "{
  \"context\": {\"request_id\": \"seed-user\"},
  \"user\": {
    \"org_id\": \"$ORG_ID\",
    \"email\": \"admin@test.org\",
    \"name\": \"QA Admin\",
    \"role\": 100
  }
}" localhost:50052 ciris.registry.v1.PortalService/CreateOrgUser)

echo "User created"
USER_ID=$(echo $USER_RESPONSE | jq -r '.message' | grep -oP 'ID: \K[a-f0-9-]+')
echo "User ID: $USER_ID"

# Generate and activate key
KEY_RESPONSE=$(grpcurl -plaintext -d "{
  \"context\": {\"request_id\": \"seed-key\"},
  \"org_id\": \"$ORG_ID\",
  \"requester_user_id\": \"$USER_ID\",
  \"activate_immediately\": true
}" localhost:50052 ciris.registry.v1.PortalService/GenerateKeyPair)

echo "Key generated"
echo $KEY_RESPONSE | jq '.key_record'

echo ""
echo "Test data seeded successfully!"
echo "ORG_ID=$ORG_ID"
echo "USER_ID=$USER_ID"
```

### A.2 Response Context Handling

All responses include context for debugging:

```json
{
  "context": {
    "request_id": "echoed-or-generated",
    "server_timestamp": 1706313600,
    "processing_time_ms": 15,
    "server_version": "registry-v1.1.0",
    "environment": "ENV_DEVELOPMENT"
  }
}
```

### A.3 Error Response Format

```json
{
  "error": {
    "code": 3,
    "message": "organization required",
    "retry_status": 0,
    "retry_after_seconds": 0,
    "metadata": {},
    "cause": null
  },
  "context": { ... }
}
```

### A.4 Bug Report Template

```markdown
**Test ID**: [e.g., ORG-003]
**Environment**: Development (localhost:50052)
**Date**: YYYY-MM-DD
**Tester**: Name

**Steps to Reproduce**:
1.
2.
3.

**Request**:
```json
{ ... }
```

**Expected Result**:

**Actual Result**:

**Request ID**: (from context)
**Server Version**: (from context)
**Logs/Screenshots**:
```

---

## Sign-Off Matrix

| Phase | Tests | Pass Rate | Sign-Off |
|-------|-------|-----------|----------|
| Phase 1: Infrastructure | 5 | 100% | [ ] |
| Phase 2: RegistryService | 23 | 95% | [ ] |
| Phase 3: Org Management | 15 | 95% | [ ] |
| Phase 4: User Management | 17 | 95% | [ ] |
| Phase 5: Key Management | 25 | 95% | [ ] |
| Phase 6: Audit & Compliance | 14 | 90% | [ ] |
| Phase 7: Agent Admin | 8 | 95% | [ ] |
| Phase 8: Partner Admin | 12 | 95% | [ ] |
| Phase 9: Incident Response | 14 | 100% | [ ] |
| Phase 10: Infrastructure Admin | 10 | 90% | [ ] |
| Phase 11: E2E Tests | 12 | 95% | [ ] |
| Phase 12: Performance | 11 | 90% | [ ] |

**Total Test Cases: ~166**

---

## Contacts

| Role | Contact |
|------|---------|
| Backend Lead | registry@ciris.ai |
| QA Lead | TBD |
| Security | security@ciris.ai |

---

*Document Version: 2.0.0*
*Last Updated: 2026-01-26*
*Registry Version: 1.1.0*
