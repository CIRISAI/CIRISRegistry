# FSD-001: CIRISRegistry Protocol Specification

**Version:** 1.1.0
**Status:** Draft
**Authors:** CIRIS L3C
**Date:** 2025-01-26

## Changelog

### v1.1.0 (2025-01-26)
Major update based on feedback from Partner, Admin, and Systems Integrator perspectives:

| Category | Addition | Rationale |
|----------|----------|-----------|
| **Error Handling** | Standardized RegistryErrorCode enum | Partners reported inability to programmatically handle errors; enables machine-parseable error handling across all languages/SDKs |
| **Tracing** | RequestContext/ResponseContext messages | Systems integrators need to trace requests across microservices for debugging and observability |
| **Monitoring** | HealthCheck, Readiness, Metrics endpoints | Kubernetes integration requires health/readiness probes; operators need visibility into registry performance |
| **Discovery** | GetCapabilities endpoint | Enables graceful feature detection, deprecation, and client version compatibility |
| **Batch Ops** | BatchLookupAgents, BatchCreateOrganizations, BatchCreateOrgUsers | Partners onboarding 50+ locations need batch APIs; polling individual records is O(N) |
| **Incident Response** | MassRevoke, EmergencyShutdown | Admins need rapid response to security incidents; circuit breaker pattern for emergencies |
| **Offline** | OfflineVerificationPackage with chain of custody | Medical devices and network-limited environments need full 72+ hour offline verification capability |
| **Key Rotation** | KeyRotationMode (IMMEDIATE, STAGED, DUAL_SIGN) | Partners need zero-downtime key rotation without failing in-flight transactions |
| **Key Recovery** | KeyEscrow, KeyRecovery endpoints | Healthcare/financial partners must have backup recovery procedures for compliance |
| **HSM Support** | RegistrySigningKey, KeyStorageMode enum | Admins need HSM integration (Vault, CloudKMS, Thales) for FIPS compliance |
| **CI/CD** | BuildAttestation, BuildProvenance | Systems integrators need verifiable build provenance (SLSA-compatible) |
| **Webhooks** | WebhookConfig, webhook events | CI/CD needs real-time notifications without polling; event-driven deployments |
| **Compliance** | ComplianceReport, ExportAuditLog | Partners need SOC2/HIPAA/GDPR compliance automation |
| **Testing** | is_test_record, test_tag, CleanupTestRecords | Staging environments need isolated test records that don't affect SLA calculations |

### v1.0.0 (2025-01-25)
- Initial specification

---

## Mission (WHY)

### Problem Statement

The CIRIS ecosystem requires a trustworthy source of truth for:

1. **Agent Identity**: Which agent builds are legitimate and what capabilities do they possess?
2. **Partner Authorization**: Which organizations are licensed to deploy agents with professional capabilities?
3. **Revocation State**: Which agents or licenses have been compromised or revoked?

Without a registry, CIRISVerify cannot distinguish between:
- A legitimate medical AI deployment at a hospital
- A counterfeit agent claiming professional capabilities
- A valid license that has been revoked

### Mission Alignment

CIRISRegistry serves the CIRIS covenant by:

- **Protecting humans** from unauthorized AI systems claiming professional authority
- **Enabling legitimate operators** to prove their authorization status
- **Maintaining accountability** through traceable agent provenance
- **Supporting transparency** through public verification endpoints

### Design Principles

1. **Fail Secure**: Unknown agents default to community tier; unknown partners have no grants
2. **Multi-Source Validation**: Critical queries validated across geographically distributed sources
3. **Offline Resilience**: Signed registry snapshots enable degraded offline operation
4. **Audit Trail**: All registry changes logged with cryptographic proof
5. **Minimal Data**: Only data required for verification; no behavioral telemetry

---

## Schemas (WHAT)

### Agent Registry

The Agent Registry maps cryptographic hashes of agent builds to their verified properties.

```
AgentRecord {
  // Identity
  agent_hash: bytes[32]           // SHA-256 of canonical agent build
  agent_type: AgentType           // CIRISCARE, CIRISMEDICAL, CIRISLEGAL, etc.
  version: SemanticVersion        // e.g., 2.1.0

  // Capabilities
  base_capabilities: Set<string>  // Capabilities this agent type can support
  max_autonomy_tier: AutonomyTier // Maximum autonomy this agent version allows

  // Provenance
  build_timestamp: int64          // When this build was created
  source_repo: string             // Git repository URL
  source_commit: string           // Git commit hash
  builder_attestation: bytes      // Reproducible build attestation

  // Status
  status: AgentStatus             // ACTIVE, DEPRECATED, REVOKED
  revocation_reason: string       // If revoked, why
  revocation_timestamp: int64     // When revoked

  // Signatures
  registry_signature: HybridSignature  // CIRIS L3C signature over record
}
```

#### Agent Types

| Type | Description | Base Capabilities |
|------|-------------|-------------------|
| CIRISCARE | Community health companion | General wellness, emotional support |
| CIRISMEDICAL | Licensed medical deployment | Triage, diagnosis support, prescription verify |
| CIRISLEGAL | Licensed legal deployment | Legal research, document review |
| CIRISFINANCIAL | Licensed financial deployment | Financial analysis, compliance |
| CUSTOM | Partner-specific build | Defined in partner agreement |

#### Agent Status

| Status | Meaning | Verification Result |
|--------|---------|---------------------|
| ACTIVE | Build is current and approved | Proceed with capability check |
| DEPRECATED | Build is outdated but functional | Warn user, suggest upgrade |
| REVOKED | Build is compromised or unauthorized | DENY all professional capabilities |

### Partner Registry

The Partner Registry maps licensed organizations to their authorized capabilities.

```
PartnerRecord {
  // Identity
  partner_id: string              // UUID assigned at licensing
  organization_name: string       // Legal entity name
  organization_id: string         // Tax ID / Registration number

  // License
  license_type: LicenseType       // COMMUNITY, PROFESSIONAL_MEDICAL, etc.
  license_id: string              // License certificate ID
  issued_at: int64                // License issue timestamp
  expires_at: int64               // License expiry timestamp

  // Grants
  capabilities_granted: Set<string>  // Specific capabilities allowed
  capabilities_denied: Set<string>   // Explicit denials (overrides grants)
  max_autonomy_tier: AutonomyTier    // Maximum autonomy allowed

  // Constraints
  requires_supervisor: bool          // Must have human supervisor
  geographic_restrictions: Set<string>  // ISO country codes where valid
  deployment_limit: int32            // Maximum concurrent deployments

  // Contact
  technical_contact: string          // Email for technical issues
  compliance_contact: string         // Email for compliance issues

  // Status
  status: PartnerStatus              // ACTIVE, SUSPENDED, REVOKED
  suspension_reason: string          // If suspended, why
  revocation_reason: string          // If revoked, why

  // Signatures
  license_signature: HybridSignature   // Steward signature on license
  registry_signature: HybridSignature  // CIRIS L3C registry signature
}
```

#### License Types

| Type | Professional Capabilities | Supervision Required |
|------|---------------------------|---------------------|
| COMMUNITY | None | No |
| COMMUNITY_PLUS | Limited (wellness, information) | No |
| PROFESSIONAL_MEDICAL | Medical triage, diagnosis support | Yes |
| PROFESSIONAL_LEGAL | Legal research, document review | Yes |
| PROFESSIONAL_FINANCIAL | Financial analysis | Yes |
| PROFESSIONAL_FULL | All professional capabilities | Yes |

#### Partner Status

| Status | Meaning | Effect on Deployments |
|--------|---------|----------------------|
| ACTIVE | License is valid | Normal operation |
| SUSPENDED | Temporary hold (payment, compliance) | Degrade to COMMUNITY |
| REVOKED | License terminated | LOCKDOWN mode |

### Revocation List

Time-critical revocation information distributed separately from full registry.

```
RevocationEntry {
  // Target
  target_type: RevocationType     // AGENT_HASH, PARTNER_ID, LICENSE_ID
  target_id: string               // Hash or ID being revoked

  // Details
  revoked_at: int64               // When revocation occurred
  reason_code: RevocationReason   // COMPROMISED, VIOLATION, EXPIRED, etc.
  reason_detail: string           // Human-readable explanation

  // Severity
  severity: RevocationSeverity    // SECURITY_CRITICAL, COMPLIANCE, ADMINISTRATIVE

  // Signature
  authority_signature: HybridSignature  // Authority that issued revocation
}
```

#### Revocation Reasons

| Reason | Description | Severity |
|--------|-------------|----------|
| SECURITY_COMPROMISED | Agent or key compromised | CRITICAL |
| LICENSE_VIOLATION | Terms violation | COMPLIANCE |
| PAYMENT_LAPSED | Payment not received | ADMINISTRATIVE |
| VOLUNTARY_TERMINATION | Partner requested termination | ADMINISTRATIVE |
| REGULATORY_ACTION | Regulatory body required revocation | CRITICAL |
| SAFETY_INCIDENT | Patient safety incident | CRITICAL |

### Capability Grant Calculation

The effective capabilities for a deployment are:

```
effective_capabilities =
    agent.base_capabilities
    ∩ partner.capabilities_granted
    - partner.capabilities_denied

effective_autonomy =
    min(agent.max_autonomy_tier, partner.max_autonomy_tier)
```

Both the agent AND the partner must authorize a capability for it to be active.

---

## Protocols (WHO)

### Query Protocol

#### Lookup Agent by Hash

**Request:**
```
GET /v1/agents/{hash}
Authorization: Bearer <api_key>  // Optional for basic lookup
X-Request-Nonce: <32 bytes hex>
```

**Response:**
```json
{
  "agent": { ... AgentRecord ... },
  "query_timestamp": 1737820800,
  "response_signature": "<hybrid_signature>"
}
```

**Error Responses:**
- `404`: Agent hash not found (treat as UNKNOWN)
- `410`: Agent explicitly revoked (returns revocation details)
- `429`: Rate limited
- `503`: Service unavailable

#### Lookup Partner by ID

**Request:**
```
GET /v1/partners/{partner_id}
Authorization: Bearer <api_key>  // Required
X-Request-Nonce: <32 bytes hex>
X-Hardware-Attestation: <attestation>  // Required for professional lookups
```

**Response:**
```json
{
  "partner": { ... PartnerRecord ... },
  "query_timestamp": 1737820800,
  "response_signature": "<hybrid_signature>"
}
```

#### Get Current Revocation List

**Request:**
```
GET /v1/revocations
If-None-Match: "<etag>"
```

**Response:**
```json
{
  "revocations": [ ... RevocationEntry ... ],
  "list_version": 12345,
  "generated_at": 1737820800,
  "next_update": 1737824400,
  "list_signature": "<hybrid_signature>"
}
```

### DNS Protocol

For multi-source validation, registry data is also published via DNS TXT records.

#### Agent Verification

```
<hash_prefix>._agent.registry.ciris.ai  TXT "v=1;s=ACTIVE;t=CIRISMEDICAL;sig=..."
```

#### Partner Verification

```
<partner_id>._partner.registry.ciris.ai  TXT "v=1;s=ACTIVE;l=PROF_MED;sig=..."
```

### Multi-Source Validation

CIRISVerify MUST query multiple sources and require agreement:

| Source | Endpoint | Purpose |
|--------|----------|---------|
| DNS US | registry-us.ciris.ai | Primary DNS |
| DNS EU | registry-eu.ciris.ai | Geographic redundancy |
| HTTPS | api.registry.ciris.ai | Full record access |

**Validation Requirements:**
- Minimum 2 of 3 sources must agree for ACTIVE status
- Any source reporting REVOKED triggers immediate revocation
- Sources disagreeing on critical fields triggers SECURITY_ALERT

---

## Integrity (HOW Protected)

### Cryptographic Guarantees

All registry records are signed using hybrid cryptography:

```
HybridSignature {
  classical: Ed25519Signature      // 64 bytes
  post_quantum: MLDSA65Signature   // ~3300 bytes
  timestamp: int64                 // Signature timestamp
  key_id: string                   // Signing key identifier
}
```

The PQC signature MUST cover the classical signature (binding):
```
pqc_message = classical_signature || record_hash || timestamp
```

### Registry Signing Keys

| Key Type | Purpose | Rotation |
|----------|---------|----------|
| Root CA | Signs intermediate keys | 10 years |
| Registry Signing | Signs agent/partner records | 1 year |
| Revocation Authority | Signs revocation entries | 1 year |
| DNS Signing | Signs DNS TXT records | 90 days |

### Snapshot Integrity

Offline snapshots include Merkle tree proof:

```
RegistrySnapshot {
  snapshot_version: int64
  generated_at: int64
  agents_root: bytes[32]        // Merkle root of agent records
  partners_root: bytes[32]      // Merkle root of partner records
  revocations_root: bytes[32]   // Merkle root of revocation list
  snapshot_signature: HybridSignature
}
```

---

## Operational Considerations

### Offline Operation

When registry is unreachable:

1. **Grace Period**: Use cached records for up to 72 hours (configurable per partner)
2. **Degradation**: After grace period, degrade to COMMUNITY capabilities
3. **Revocations**: Cached revocation list remains authoritative
4. **Logging**: All offline decisions logged for audit

### Rate Limits

| Endpoint | Rate Limit | Burst |
|----------|-----------|-------|
| Agent lookup | 100/min | 20 |
| Partner lookup | 60/min | 10 |
| Revocation list | 10/min | 5 |

### Data Retention

| Data Type | Retention | Reason |
|-----------|-----------|--------|
| Active records | Indefinite | Current state |
| Revoked records | 7 years | Audit trail |
| Query logs | 90 days | Abuse detection |
| Audit events | 7 years | Compliance |

---

## Appendix A: Capability Namespace

Capabilities follow hierarchical namespace:

```
domain:<domain>:<capability>
modality:<modality>:<feature>
autonomy:<tier>:<action>
```

**Medical Domain Examples:**
```
domain:medical:triage
domain:medical:diagnosis_support
domain:medical:prescription_verify
domain:medical:imaging_analysis
modality:medical:radiology
modality:medical:pathology
```

**Autonomy Tiers:**
```
autonomy:A0:advisory           # Information only
autonomy:A1:limited            # Low-risk actions
autonomy:A2:moderate           # Supervised actions
autonomy:A3:high               # Independent actions
autonomy:A4:critical           # Life-affecting actions
```

---

## Appendix B: Mandatory Disclosures

Registry queries return mandatory disclosure templates:

| Status | Disclosure Template |
|--------|---------------------|
| AGENT_UNKNOWN | "Unverified agent build. Operating in restricted mode." |
| PARTNER_UNKNOWN | "Unverified deployment. No professional capabilities." |
| AGENT_REVOKED | "⚠️ This agent build has been revoked: {reason}" |
| PARTNER_REVOKED | "⚠️ License {license_id} has been revoked: {reason}" |
| LICENSE_EXPIRED | "License expired. Operating in community mode." |
| VERIFICATION_FAILED | "Registry verification failed. Limited functionality." |

---

## Appendix C: Security Considerations

### Threat Model

| Threat | Mitigation |
|--------|------------|
| Registry compromise | Multi-source validation, certificate pinning |
| Replay attacks | Nonce in requests, timestamp in responses |
| Enumeration | Rate limiting, API key required for bulk access |
| Stale data attacks | Short cache TTL, revocation list priority |
| Key compromise | Hybrid crypto, key rotation, HSM storage |

### Incident Response

Registry security incidents follow CIRIS Incident Response Plan:
1. Immediate revocation capability via out-of-band channel
2. Emergency contact available 24/7
3. Public disclosure within 72 hours of confirmed breach
4. Coordinated notification to affected partners

---

---

## v1.1.0 Feature Specifications

### Standardized Error Handling

**Purpose:** Enable machine-parseable errors across all client languages and SDKs.

**Rationale:** Partners reported inability to programmatically handle errors. Different error types require different handling (retry vs. escalate vs. fix input). The error code enum with HTTP-aligned ranges enables consistent error handling.

**Error Code Ranges:**
| Range | Category | Examples |
|-------|----------|----------|
| 400-499 | Standard HTTP | INVALID_ARGUMENT, UNAUTHORIZED, NOT_FOUND |
| 1000-1999 | Agent errors | AGENT_NOT_REGISTERED, AGENT_REVOKED |
| 2000-2999 | Partner errors | PARTNER_EXPIRED, LICENSE_REVOKED |
| 3000-3999 | Cryptographic errors | INVALID_SIGNATURE, KEY_REVOKED |
| 4000-4999 | Organization errors | ORG_NOT_FOUND, INSUFFICIENT_ROLE |
| 5000-5999 | Infrastructure errors | DATABASE_ERROR, HSM_UNAVAILABLE |

**Retry Guidance:**
Each error includes retry guidance: `RETRY_NO`, `RETRY_IMMEDIATE`, `RETRY_BACKOFF`, or `RETRY_AFTER` with duration.

---

### Request/Response Context

**Purpose:** Enable request correlation across services and debugging.

**Rationale:** Systems integrators need to trace requests across microservices. Without correlation IDs, debugging distributed failures is extremely difficult.

**RequestContext fields:**
- `request_id`: UUID generated by client (or server if empty)
- `client_version`: e.g., "portal-v0.1.0", "agent-v2.3.1"
- `user_agent`: Client identifier
- `request_timestamp`: Client's timestamp

**ResponseContext fields:**
- `request_id`: Echo of request ID for correlation
- `server_timestamp`: Server's timestamp
- `processing_time_ms`: Request processing duration
- `server_version`: e.g., "registry-v1.1.0"
- `environment`: PRODUCTION, STAGING, CANARY, DEVELOPMENT

---

### Health & Monitoring

**Purpose:** Enable Kubernetes integration and operational monitoring.

**Rationale:** Kubernetes requires health/readiness probes for automated deployment. Operators need visibility into registry health without logging into servers.

**HealthCheckResponse includes:**
- `status`: SERVING, DEGRADED, NOT_SERVING
- `readiness`: STARTUP, LIVE, SHUTDOWN
- `components[]`: Per-component health (database, cache, hsm, replication)
- `version`, `build_commit`, `uptime_seconds`
- `database_healthy`, `replication_lag_ms`
- CPU/memory usage percentages

**MetricsResponse includes:**
- Query metrics: total, by type, latency percentiles (p50/p95/p99)
- Error metrics: total, by error code
- Signing operations count
- Database connection pool stats

---

### API Capabilities Discovery

**Purpose:** Allow clients to discover supported features.

**Rationale:** Enables graceful feature detection and deprecation. Clients can check capabilities before calling optional endpoints.

**RegistryCapabilities includes:**
- `protocol_version`: "1.1.0"
- Feature flags: `supports_merkle_proofs`, `supports_offline_mode`, `supports_batch_operations`, `supports_webhooks`, `supports_build_attestation`
- `supported_algorithms[]`: ["Ed25519", "ML-DSA-65"]
- Limits: `max_batch_size`, `revocation_list_ttl_seconds`, `offline_package_ttl_hours`
- Deprecation info: `deprecated_endpoints[]`, `migration_guide` map

---

### Batch Operations

**Purpose:** Enable efficient bulk operations for large-scale onboarding.

**Rationale:** Partners onboarding 50+ locations (healthcare networks, franchise systems) cannot call individual endpoints. Batch APIs reduce O(N) to O(1) round trips.

**Batch endpoints:**
| Endpoint | Max Size | Use Case |
|----------|----------|----------|
| BatchLookupAgents | 100 | Verify multiple agent builds at once |
| BatchCreateOrganizations | 100 | Onboard multiple facilities |
| BatchCreateOrgUsers | 100 | Add staff from HR import |
| BatchRegisterAgents | 1000 | CI/CD release automation |

**Error handling modes:**
- `FAIL_FAST`: Abort on first error
- `BEST_EFFORT`: Continue on errors, collect all results
- `TRANSACTIONAL`: All-or-nothing semantics

---

### Incident Response

**Purpose:** Enable rapid response to security incidents.

**Rationale:** Admins need mass revocation and emergency shutdown capabilities. A supply chain compromise affecting 1000s of agents cannot be handled one-by-one.

#### MassRevoke

Revoke multiple entities based on selection criteria:
- By explicit list: `agent_hashes[]`, `partner_ids[]`, `license_ids[]`
- By pattern: `agent_version_prefix` (e.g., "2.1.*")
- By type: `agent_type`
- By region: `geographic_regions[]`
- By time: `registered_before` timestamp

Supports `is_dry_run` to preview affected entities before committing.

#### EmergencyShutdown (Circuit Breaker)

Put registry in read-only or completely locked mode:
- `severity`: LOW, MEDIUM, HIGH, CRITICAL
- `lock_duration_seconds`: 0 = manual unlock required
- `allowed_operations[]`: Empty = read-only mode

Use cases:
- Active supply chain attack
- Database compromise detected
- Coordinated incident response

---

### Offline Verification Package

**Purpose:** Enable 72+ hour offline operation with full verification capability.

**Rationale:** Medical devices and network-limited environments (rural clinics, disaster response) need complete offline verification. Cannot depend on network connectivity.

**OfflineVerificationPackage includes:**
- Compressed registry data: agents, partners, revocations
- Merkle roots for each registry
- Package signature with signer public keys
- Metadata: snapshot timestamp, API version, expiration, compression format

**MerkleProofWithChain:**
Enhanced proof that includes chain of custody from record → merkle tree → snapshot → signature. Enables offline clients to verify without trusting any intermediate state.

**OfflineSnapshotDelta:**
Incremental updates for bandwidth-constrained environments:
- Added/modified/removed agents
- Added/modified/removed partners
- New revocations
- New merkle roots

---

### Key Rotation Modes

**Purpose:** Enable zero-downtime key rotation.

**Rationale:** Partners need to rotate keys without failing in-flight transactions. Immediate rotation causes race conditions.

**Rotation modes:**
| Mode | Behavior | Use Case |
|------|----------|----------|
| ROTATION_IMMEDIATE | New key active immediately, old valid 24h | Emergency rotation |
| ROTATION_STAGED | Both keys active for grace period | Planned rotation |
| ROTATION_DUAL_SIGN | New key signs, old validates | Anti-repudiation |

**RotateKeyRequest** includes:
- `mode`: Rotation strategy
- `grace_period_hours`: How long to keep old key valid (STAGED mode)

**RotateKeyResponse** includes:
- Old and new key records
- `grace_period_expires_at` timestamp
- `rotation_id` for audit tracking

---

### Key Escrow & Recovery

**Purpose:** Enable key recovery for compliance requirements.

**Rationale:** Healthcare and financial partners must have backup recovery procedures. Regulatory frameworks (HIPAA, SOX) require documented recovery procedures.

**Escrow types:**
| Type | Custodian | Recovery Requirement |
|------|-----------|---------------------|
| ESCROW_STEWARD | CIRIS Steward L3C | Single steward approval |
| ESCROW_ATTORNEY | Legal escrow agent | Per company requirements |
| ESCROW_DUAL_CUSTODY | Two stewards | Both required to recover |

**Endpoints:**
- `RequestKeyEscrow`: Create escrow for a key
- `RequestKeyRecovery`: Initiate recovery process
- `ListKeyEscrows`: View all escrows for organization

Recovery is asynchronous with 24-hour expiration and requires steward approval.

---

### Registry Signing Key Management

**Purpose:** Enable HSM integration and key rotation for registry operators.

**Rationale:** Admins need to rotate registry signing keys without downtime. FIPS 140-2 compliance requires HSM-backed keys.

**Supported storage backends:**
| Backend | Use Case |
|---------|----------|
| IN_MEMORY | Development only |
| VAULT | HashiCorp Vault |
| CLOUDKMS | Google Cloud KMS |
| AWSKMS | AWS KMS |
| AZUREKV | Azure Key Vault |
| HSM_THALES | Thales Luna HSM |
| HSM_YUBIHSM | YubiHSM |
| HSM_CLOUDHSM | AWS CloudHSM |

**RegistrySigningKey includes:**
- Public keys and fingerprints for both Ed25519 and ML-DSA-65
- Lifecycle timestamps: created, activated, rotated, retired
- Usage count and last used timestamp
- Status: PENDING, ACTIVE, STANDBY, RETIRED
- HSM slot ID and label

---

### Build Attestation

**Purpose:** Prove reproducible builds from CI/CD pipelines.

**Rationale:** Systems integrators need verifiable build provenance. SLSA framework requires attestation for supply chain security.

**BuildProvenance** (SLSA-compatible):
- Builder identity: workflow URL, invocation ID
- Timestamps: started, finished
- Git source: URI, commit, branch
- Build commands executed
- Expected artifact hash
- Builder environment: OS, architecture, env vars

**BuildAttestation** combines provenance with hybrid signature from builder.

**Endpoints:**
- `RegisterBuildAttestation`: Record attestation for an agent hash
- `GetBuildAttestation`: Retrieve attestation and verification count

---

### Webhooks

**Purpose:** Enable event-driven deployments.

**Rationale:** CI/CD needs real-time notifications without polling. Polling is wasteful and introduces latency.

**WebhookConfig:**
- `url`: HTTPS endpoint
- `subscribed_events[]`: ["agent.registered", "agent.revoked", "partner.suspended", ...]
- `signing_secret`: For HMAC-SHA256 verification
- `active`: Enable/disable
- `consecutive_failures`: For automatic disabling

**WebhookEvent:**
- `event_type`, `timestamp`
- `entity_type`, `entity_id`
- `metadata` map
- `event_signature`: Hybrid signature for verification

---

### Compliance & Audit

**Purpose:** Generate compliance reports and export audit logs.

**Rationale:** Partners need SOC2/HIPAA/GDPR compliance automation. Manual compliance reporting is error-prone and time-consuming.

**ComplianceReport** includes:
- Period covered
- Key management summary: keys generated/rotated/revoked, rotation policy compliance
- Access control summary: total users, admin users, MFA adoption, failed login attempts
- Audit summary: total events, continuous audit trail verification
- Attestation statement
- Hybrid signature

**ExportAuditLog** supports:
- Filters: time range, action types, actor user IDs, target types
- Formats: JSON, CSV, JSONL, SPLUNK_HEC
- Includes optional signatures

**Additional admin endpoints:**
- `ListExpiringLicenses`: 30/60/90 day expiration warnings
- `GetPartnerActivity`: Health assessment (HEALTHY, IDLE, INACTIVE)

---

### Test Record Support

**Purpose:** Enable isolated testing in staging environments.

**Rationale:** E2E tests need real registry data without affecting SLA calculations or polluting production reports.

**Features:**
- `is_test_record` flag on AgentRecord
- `test_tag` for batching test records (e.g., "e2e-pr-1234")
- `CleanupTestRecords` endpoint to remove all records with a given tag

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.1.0 | 2025-01-26 | Added error handling, tracing, batch ops, incident response, offline verification, key rotation/escrow, HSM support, build attestation, webhooks, compliance, test records |
| 1.0.0 | 2025-01-25 | Initial specification |
