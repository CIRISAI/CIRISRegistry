# CIRISRegistry

**Agent and Partner Registry for the CIRIS Ecosystem**

CIRISRegistry provides the authoritative source of truth for:

1. **Agent Identity** - Which agent builds are legitimate and what capabilities they possess
2. **Partner Authorization** - Which organizations are licensed to deploy agents with professional capabilities
3. **Revocation State** - Which agents or licenses have been compromised or revoked

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          CIRIS Ecosystem                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌────────────────────────────┐ │
│  │ CIRISVerify  │───▶│ CIRISRegistry│◀───│ CIRISPortal                │ │
│  │ (Binary)     │    │ (This Repo)  │    │ (portal.ciris.ai)          │ │
│  └──────────────┘    └──────────────┘    │ - Org/User Management      │ │
│         │                   │            │ - Key Custody              │ │
│         │                   │            │ - Partner Onboarding       │ │
│         │                   │            └────────────────────────────┘ │
│         ▼                   ▼                                            │
│  ┌───────────────────────────────────────────────────────────┐          │
│  │                   Effective Capabilities                   │          │
│  │  = agent.capabilities ∩ partner.grants - partner.denials   │          │
│  └───────────────────────────────────────────────────────────┘          │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## How It Works

### Two-Factor Verification

CIRISVerify queries CIRISRegistry to verify **both**:

1. **Agent Hash** - Is this agent build registered and approved?
2. **Partner License** - Is this organization licensed for the requested capabilities?

The effective capabilities are the **intersection** of what the agent supports and what the partner is licensed for.

### Multi-Source Validation

Critical queries are validated across geographically distributed sources:

| Source | Endpoint | Purpose |
|--------|----------|---------|
| DNS US | registry-us.ciris.ai | Primary DNS |
| DNS EU | registry-eu.ciris.ai | Geographic redundancy |
| HTTPS | api.registry.ciris.ai | Full record access |

At least 2 of 3 sources must agree for a positive verification.

## Repository Structure

```
CIRISRegistry/
├── FSD/
│   └── FSD-001_CIRISREGISTRY_PROTOCOL.md   # Full specification
├── protocol/
│   └── ciris_registry.proto                 # Protocol buffer definitions
├── docs/
│   └── ...                                  # Additional documentation
├── CLAUDE.md                                # AI development guide
└── README.md                                # This file
```

## Key Concepts

### Agent Registry

Maps cryptographic hashes of agent builds to their properties:

```protobuf
message AgentRecord {
  bytes agent_hash = 1;           // SHA-256 of agent build
  AgentType agent_type = 2;       // CIRISCARE, CIRISMEDICAL, etc.
  SemanticVersion version = 3;
  repeated string base_capabilities = 10;
  AutonomyTier max_autonomy_tier = 11;
  AgentStatus status = 30;        // ACTIVE, DEPRECATED, REVOKED
  HybridSignature registry_signature = 50;
}
```

### Partner Registry

Maps licensed organizations to their authorized capabilities:

```protobuf
message PartnerRecord {
  string partner_id = 1;
  string organization_name = 2;
  LicenseType license_type = 10;  // COMMUNITY, PROFESSIONAL_MEDICAL, etc.
  repeated string capabilities_granted = 20;
  repeated string capabilities_denied = 21;
  AutonomyTier max_autonomy_tier = 22;
  PartnerStatus status = 50;      // ACTIVE, SUSPENDED, REVOKED
}
```

### Revocation List

Time-critical revocation information distributed separately:

```protobuf
message RevocationEntry {
  RevocationType target_type = 1; // AGENT_HASH, PARTNER_ID, LICENSE_ID
  string target_id = 2;
  RevocationReason reason_code = 11;
  RevocationSeverity severity = 13;
}
```

## Security

### Hybrid Cryptography

All records are signed using Ed25519 (classical) + ML-DSA-65 (post-quantum):

```protobuf
message HybridSignature {
  bytes classical_signature = 1;      // Ed25519
  bytes post_quantum_signature = 2;   // ML-DSA-65 (FIPS 204)
  int64 timestamp = 3;
  string key_id = 4;
}
```

### Fail-Secure Design

- Unknown agents default to community tier
- Unknown partners have no capability grants
- Any source reporting REVOKED triggers immediate action
- Network failures result in degradation, never escalation

## Integration

### CIRISVerify

Queries CIRISRegistry via:
1. **HTTPS API** - Full record lookup with hardware attestation
2. **DNS TXT** - Lightweight status verification for multi-source validation
3. **Signed Snapshots** - Offline operation support

### CIRISPortal

Administrative interface at **portal.ciris.ai** that writes to CIRISRegistry:
- Organization and user management
- Partner onboarding and license management
- Custodied key generation and registration
- Audit log viewing

See [CIRISPortal](https://github.com/CIRISAI/CIRISPortal) for the admin interface.

## API Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/v1/agents/{hash}` | GET | Lookup agent by hash |
| `/v1/partners/{id}` | GET | Lookup partner by ID |
| `/v1/partners` | POST | Create partner record |
| `/v1/partners/{id}/keys` | POST | Register partner public key |
| `/v1/revocations` | GET | Get revocation list |
| `/v1/organizations` | GET/POST | Organization management |
| `/v1/organizations/{id}/users` | GET/POST | User management |

## License

CIRIS Mission License (CML) - See LICENSE file.

## Related Projects

| Project | Purpose |
|---------|---------|
| [CIRISPortal](https://github.com/CIRISAI/CIRISPortal) | Partner portal and key custody (portal.ciris.ai) |
| [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) | Hardware-rooted license verification |
| [CIRISAgent](https://github.com/CIRISAI/CIRISAgent) | Core agent engine |

## Contact

- Technical: registry@ciris.ai
- Security: security@ciris.ai
- Licensing: licensing@ciris.ai
