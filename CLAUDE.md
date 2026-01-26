# CIRISRegistry - Claude Development Guide

## Project Overview

CIRISRegistry is the authoritative source of truth for the CIRIS AI ecosystem, providing:

- **Agent Identity** - Verification of legitimate agent builds and their capabilities
- **Partner Authorization** - License management for organizations deploying CIRIS agents
- **Revocation State** - Real-time status of compromised or revoked agents/licenses

The registry is the **trust backbone** that distinguishes licensed professional deployments from community deployments. It enables the ecosystem's core promise: *"You're not paying for capability. You're paying for accountability."*

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
│  │                    CIRISRegistry API                               │  │
│  │                  api.registry.ciris.ai                             │  │
│  │                                                                    │  │
│  │  Endpoints:                                                        │  │
│  │  • GET  /v1/agents/{hash}           - Agent lookup                 │  │
│  │  • GET  /v1/partners/{id}           - Partner lookup               │  │
│  │  • POST /v1/partners                - Create partner               │  │
│  │  • POST /v1/partners/{id}/keys      - Register public key          │  │
│  │  • GET  /v1/revocations             - Revocation list              │  │
│  │  • GET  /v1/organizations           - List organizations           │  │
│  │  • POST /v1/organizations           - Create organization          │  │
│  │  • POST /v1/organizations/{id}/users - Invite user                 │  │
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

## CIRISPortal Integration

**CIRISPortal** (portal.ciris.ai) is the administrative interface that writes to this registry.

### Portal → Registry Data Flow

```
1. Admin creates Organization in Portal
   └─→ POST /v1/organizations

2. Admin onboards Partner (assigns license)
   └─→ POST /v1/partners

3. Partner generates custodied keys in Portal
   └─→ Portal generates Ed25519 + ML-DSA-65 key pair
   └─→ POST /v1/partners/{id}/keys (registers public key)

4. Partner rotates keys
   └─→ Portal generates new key pair
   └─→ POST /v1/partners/{id}/keys (new public key)
   └─→ Old key marked as rotated (still valid for verification)
```

### API Endpoints Needed by Portal

| Endpoint | Method | Purpose | Status |
|----------|--------|---------|--------|
| `/v1/organizations` | GET | List orgs (admin) | TODO |
| `/v1/organizations` | POST | Create org | TODO |
| `/v1/organizations/{id}` | GET | Get org details | TODO |
| `/v1/organizations/{id}` | PATCH | Update org | TODO |
| `/v1/organizations/{id}/users` | GET | List org users | TODO |
| `/v1/organizations/{id}/users` | POST | Invite user | TODO |
| `/v1/partners` | POST | Create partner | TODO |
| `/v1/partners/{id}` | GET | Get partner | Spec'd |
| `/v1/partners/{id}` | PATCH | Update partner | TODO |
| `/v1/partners/{id}/keys` | GET | List partner keys | TODO |
| `/v1/partners/{id}/keys` | POST | Register public key | TODO |
| `/v1/partners/{id}/keys/{keyId}` | DELETE | Revoke key | TODO |

### Proto Additions Needed

```protobuf
// Organization management (add to ciris_registry.proto)
message Organization {
  string org_id = 1;
  string name = 2;
  string tax_id = 3;
  repeated string admin_emails = 10;
  int64 created_at = 20;
  int64 updated_at = 21;
}

message OrgUser {
  string user_id = 1;
  string email = 2;
  OrgRole role = 3;
  string org_id = 4;
  int64 invited_at = 10;
  int64 joined_at = 11;
}

enum OrgRole {
  ORG_ROLE_UNSPECIFIED = 0;
  ORG_ADMIN = 1;      // Can manage keys, invite users
  ORG_USER = 2;       // Read-only access
}

// Partner key registration
message PartnerKeyRecord {
  string key_id = 1;
  string partner_id = 2;
  KeyStatus status = 3;
  PublicKeys public_keys = 10;
  int64 created_at = 20;
  int64 rotated_at = 21;
  string rotated_by_key_id = 22;  // If rotated, which key replaced this
}

enum KeyStatus {
  KEY_STATUS_UNSPECIFIED = 0;
  KEY_ACTIVE = 1;
  KEY_ROTATED = 2;    // Still valid for verification, not signing
  KEY_REVOKED = 3;
}

message PublicKeys {
  bytes ed25519_public = 1;       // 32 bytes
  bytes mldsa65_public = 2;       // ~1952 bytes
}
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

## Project Structure

```
CIRISRegistry/
├── FSD/                        # Functional Specification Documents
│   └── FSD-001_*.md            # Protocol specification
├── protocol/
│   └── ciris_registry.proto    # Protocol buffer definitions
├── docs/                       # Additional documentation
├── CLAUDE.md                   # This file
└── README.md
```

## Security Requirements

### Hybrid Cryptography

All records use Ed25519 (classical) + ML-DSA-65 (post-quantum) signatures:

```protobuf
message HybridSignature {
  bytes classical_signature = 1;      // Ed25519
  bytes post_quantum_signature = 2;   // ML-DSA-65 (FIPS 204)
  int64 timestamp = 3;
  string key_id = 4;
}
```

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

## Testing Considerations

- Verify multi-source validation logic handles disagreements correctly
- Test offline/degraded mode behavior (72-hour grace period default)
- Ensure revocation propagation is immediate
- Validate hybrid signature verification (both classical AND post-quantum)
- Test capability intersection logic: `effective = agent ∩ partner.granted - partner.denied`
- **Test Portal integration** - verify API endpoints work correctly with Portal

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
