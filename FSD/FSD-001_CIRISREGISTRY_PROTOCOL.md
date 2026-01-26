# FSD-001: CIRISRegistry Protocol Specification

**Version:** 1.0.0
**Status:** Draft
**Authors:** CIRIS L3C
**Date:** 2025-01-25

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

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-01-25 | Initial specification |
