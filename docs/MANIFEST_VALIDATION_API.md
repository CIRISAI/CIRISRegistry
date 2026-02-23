# CIRISRegistry Manifest & Validation API Reference

This document describes all Registry APIs used by CIRISVerify and CIRISAgent for integrity verification and license validation.

## Overview

CIRISRegistry provides three verification levels:

| Level | Purpose | Endpoint |
|-------|---------|----------|
| 1 | Registry key authenticity | `/v1/steward-key` |
| 2 | Binary integrity (whole file) | `/v1/verify/binary-manifest/{version}` |
| 3 | Function integrity (per-FFI export) | `/v1/verify/function-manifest/{version}/{target}` |

Plus runtime license/revocation checks via gRPC and HTTP.

---

## HTTP REST Endpoints (Public)

Base URL: `https://api.registry.ciris-services-1.ai`

### 1. Steward Key Retrieval

```
GET /v1/steward-key
```

Returns the registry's active signing key pair for signature verification.

**Response:**
```json
{
  "classical": {
    "algorithm": "Ed25519",
    "key": "<base64-encoded-public-key>",
    "key_id": "steward-2026"
  },
  "pqc": {
    "algorithm": "ML-DSA-65",
    "key": "<base64-encoded-public-key>",
    "key_id": "steward-2026",
    "fingerprint": "sha256:abc123..."
  },
  "signature_mode": "HYBRID_REQUIRED",
  "revision": 42,
  "timestamp": 1740100000,
  "next_rotation": null
}
```

**Used by:** CIRISVerify Level 1 multi-source validation

---

### 2. Revocation Check

```
GET /v1/revocation/{target_id}
```

Check if a specific license, agent, or partner has been revoked.

**Parameters:**
- `target_id` - License ID, agent hash, or partner ID

**Response (not revoked):**
```json
{
  "license_id": "partner-123",
  "revoked": false,
  "revoked_at": null,
  "reason": null,
  "checked_at": 1740100000
}
```

**Response (revoked):**
```json
{
  "license_id": "partner-123",
  "revoked": true,
  "revoked_at": 1740050000,
  "reason": "License expired",
  "checked_at": 1740100000
}
```

**Used by:** CIRISVerify, CIRISAgent runtime checks

---

### 3. Binary Manifest (Level 2 Self-Verification)

```
GET /v1/verify/binary-manifest/{version}
```

Returns SHA-256 hashes for all binaries of a given version.

**Parameters:**
- `version` - Semver version string (e.g., `0.5.4`)

**Response:**
```json
{
  "version": "0.5.4",
  "binaries": {
    "x86_64-unknown-linux-gnu": "sha256:c7049adb0f4b7bc75d5ca6c6bfdc22552fd03c3d39f429863e589331e4b4a26a",
    "aarch64-unknown-linux-gnu": "sha256:...",
    "x86_64-apple-darwin": "sha256:...",
    "aarch64-apple-darwin": "sha256:...",
    "x86_64-pc-windows-msvc": "sha256:..."
  },
  "generated_at": "2026-02-21T02:51:03Z"
}
```

**Used by:** CIRISVerify Level 2 (compares own binary hash against manifest)

---

### 4. Function Manifest (Level 3 Runtime Integrity)

```
GET /v1/verify/function-manifest/{version}/{target}
```

Returns per-function SHA-256 hashes for runtime FFI verification.

**Parameters:**
- `version` - Binary version (e.g., `0.5.4`)
- `target` - Rust target triple (e.g., `x86_64-unknown-linux-gnu`)

**Example:**
```
GET /v1/verify/function-manifest/0.5.4/x86_64-unknown-linux-gnu
```

**Response:**
```json
{
  "version": "1.0.0",
  "target": "x86_64-unknown-linux-gnu",
  "binary_hash": "sha256:c7049adb0f4b7bc75d5ca6c6bfdc22552fd03c3d39f429863e589331e4b4a26a",
  "binary_version": "0.5.4",
  "generated_at": "2026-02-21T02:51:03Z",
  "functions": {
    "ciris_verify_init": {
      "name": "ciris_verify_init",
      "offset": 2401440,
      "size": 22297,
      "hash": "sha256:ee6f129c36169b5dfd7a9b1158dd15c7eb0aa9011b3d88f9f08540d9d0a7ed50"
    },
    "ciris_verify_get_status": {
      "name": "ciris_verify_get_status",
      "offset": 2383440,
      "size": 15176,
      "hash": "sha256:d6f9d2d1b71a8e42bd611b6ca1422bf58447b567fb341658d4280c47442c9c5b"
    }
  },
  "manifest_hash": "sha256:54b973946e34b19452fec6220f96701be1800fb13f18c10e7c5801bd7bf88eb5",
  "signature": {
    "classical": "<base64-Ed25519-signature>",
    "classical_algorithm": "Ed25519",
    "pqc": "<base64-ML-DSA-65-signature>",
    "pqc_algorithm": "ML-DSA-65",
    "key_id": "steward-2026"
  }
}
```

**Used by:** CIRISVerify runtime function integrity verification

---

### 5. List Function Manifest Targets

```
GET /v1/verify/function-manifests/{version}
```

Lists all available target triples for a given version.

**Response:**
```json
{
  "version": "0.5.4",
  "targets": [
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "aarch64-linux-android"
  ]
}
```

---

## HTTP REST Endpoints (Admin)

These endpoints require authentication via `REGISTRY_ADMIN_TOKEN`.

**Header:** `Authorization: Bearer <REGISTRY_ADMIN_TOKEN>`

### Register Binary Manifest

```
POST /v1/verify/binary-manifest
```

**Request:**
```json
{
  "version": "0.5.4",
  "binaries": {
    "x86_64-unknown-linux-gnu": "sha256:...",
    "aarch64-apple-darwin": "sha256:..."
  },
  "generated_at": "2026-02-21T02:51:03Z",
  "notes": "Release build from CI"
}
```

**Response:**
```json
{
  "success": true,
  "manifest_id": "uuid-here",
  "message": "Binary manifest registered for version 0.5.4"
}
```

---

### Register Function Manifest

```
POST /v1/verify/function-manifest
```

**Request:**
```json
{
  "version": "1.0.0",
  "target": "x86_64-unknown-linux-gnu",
  "binary_hash": "sha256:c7049adb...",
  "binary_version": "0.5.4",
  "generated_at": "2026-02-21T02:51:03Z",
  "functions": {
    "ciris_verify_init": {
      "name": "ciris_verify_init",
      "offset": 2401440,
      "size": 22297,
      "hash": "sha256:ee6f129c..."
    }
  },
  "manifest_hash": "sha256:54b97394...",
  "signature": {
    "classical": "<base64>",
    "classical_algorithm": "Ed25519",
    "pqc": "<base64>",
    "pqc_algorithm": "ML-DSA-65",
    "key_id": "steward-2026"
  }
}
```

**Response:**
```json
{
  "success": true,
  "id": 42,
  "message": "Function manifest registered for version 0.5.4 target x86_64-unknown-linux-gnu"
}
```

---

## gRPC RegistryService (Port 50052)

Public read-only service for agent/partner verification.

### Methods

| Method | Purpose |
|--------|---------|
| `HealthCheck` | System health status |
| `GetCapabilities` | API feature discovery |
| `LookupAgent` | Verify agent build by hash |
| `BatchLookupAgents` | Batch verification (max 100) |
| `LookupPartner` | Verify partner license |
| `VerifyDeployment` | Combined agent + partner check |
| `GetRevocationList` | Full or delta revocation list |
| `GetPublicKeys` | Organization's public keys |
| `GetBuildAttestation` | SLSA build provenance |
| `GetBuild` | Build metadata lookup |
| `GetEmergencyStatus` | Emergency shutdown state |
| `GetOfflinePackage` | Full offline verification snapshot |
| `GetOfflineDelta` | Incremental snapshot updates |

### LookupAgent

Verify an agent build by its hash.

```protobuf
rpc LookupAgent(LookupAgentRequest) returns (LookupAgentResponse);

message LookupAgentRequest {
  bytes agent_hash = 1;  // SHA-256 of agent binary
}

message LookupAgentResponse {
  AgentRecord agent = 1;
  bool found = 2;
}
```

### VerifyDeployment

Combined agent + partner verification with capability intersection.

```protobuf
rpc VerifyDeployment(VerifyDeploymentRequest) returns (VerifyDeploymentResponse);

message VerifyDeploymentRequest {
  bytes agent_hash = 1;
  string partner_id = 2;
}

message VerifyDeploymentResponse {
  AgentRecord agent = 1;
  PartnerRecord partner = 2;
  repeated string effective_capabilities = 3;  // agent ∩ partner.granted - partner.denied
  bool valid = 4;
}
```

### GetRevocationList

Get revoked entities (full list or delta since revision).

```protobuf
rpc GetRevocationList(GetRevocationListRequest) returns (GetRevocationListResponse);

message GetRevocationListRequest {
  int64 since_revision = 1;  // 0 for full list
}

message GetRevocationListResponse {
  repeated RevocationEntry entries = 1;
  int64 current_revision = 2;
}
```

---

## Verification Flow

### CIRISVerify Startup Sequence

```
1. Level 1: Fetch steward key from multiple sources
   GET /v1/steward-key (from 2+ geographic regions)
   → Requires 2-of-3 agreement

2. Level 2: Binary self-verification
   GET /v1/verify/binary-manifest/{version}
   → Compare own SHA-256 against manifest

3. Level 3: Function integrity (optional, runtime)
   GET /v1/verify/function-manifest/{version}/{target}
   → Verify FFI export hashes at load time

4. License check (runtime)
   gRPC: LookupAgent(agent_hash)
   gRPC: LookupPartner(partner_id)
   → Compute effective capabilities
```

### CIRISAgent Runtime Flow

```
1. On startup:
   gRPC: VerifyDeployment(agent_hash, partner_id)
   → Get effective capabilities

2. Periodic (every 5 min):
   gRPC: GetRevocationList(last_revision)
   → Check for new revocations

3. On capability request:
   → Check against effective_capabilities
   → Unknown = COMMUNITY tier only
```

---

## Target Triples

Supported Rust target triples for manifests:

| Target | Platform | Format |
|--------|----------|--------|
| `x86_64-unknown-linux-gnu` | Linux x86_64 | ELF |
| `aarch64-unknown-linux-gnu` | Linux ARM64 | ELF |
| `x86_64-apple-darwin` | macOS Intel | Mach-O |
| `aarch64-apple-darwin` | macOS Apple Silicon | Mach-O |
| `x86_64-pc-windows-msvc` | Windows x86_64 | PE |
| `aarch64-linux-android` | Android ARM64 | ELF |
| `aarch64-apple-ios` | iOS ARM64 | Mach-O |

---

---

## Registering Agent Builds

CIRISAgent binaries are registered differently from CIRISVerify manifests. Agent registration uses gRPC and tracks individual build artifacts by their SHA-256 hash.

### Agent Registration (gRPC)

Use `RegistryAdminService.RegisterAgent` to register agent builds:

```protobuf
rpc RegisterAgent(RegisterAgentRequest) returns (RegisterAgentResponse);

message RegisterAgentRequest {
  AgentRecord agent = 1;
  BuildAttestation attestation = 2;  // Optional SLSA provenance
}
```

### Agent Record Fields

| Field | Required | Description |
|-------|----------|-------------|
| `agent_hash` | Yes | SHA-256 of the agent binary |
| `agent_type` | Yes | 1=CORE, 2=MODULE, 3=ADAPTER |
| `version` | Yes | Semantic version (major.minor.patch) |
| `base_capabilities` | Yes | Granted capabilities (e.g., `domain:medical:triage`) |
| `max_autonomy_tier` | Yes | 0=A0, 1=A1, 2=A2, 3=A3, 4=A4 |
| `build_timestamp` | Yes | Unix timestamp of build |
| `source_repo` | No | GitHub repo URL |
| `source_commit` | No | Git commit SHA |
| `identity_template` | No | Template name (e.g., `medical-triage-v1`) |
| `approved_adapters` | No | List of approved adapter hashes |
| `org_id` | No | Owning organization ID |

### Build Attestation (SLSA Provenance)

For supply chain security, include build provenance:

```protobuf
message BuildAttestation {
  BuildProvenance provenance = 1;
  HybridSignature builder_signature = 2;
}

message BuildProvenance {
  string builder_id = 1;           // e.g., "github-actions"
  string invocation_id = 2;        // CI run ID
  int64 started_at = 3;
  int64 finished_at = 4;
  string source_uri = 5;           // e.g., "https://github.com/org/repo"
  string source_commit = 6;
  string source_branch = 7;
  repeated string build_commands = 8;
  bytes expected_artifact_hash = 9;
  string reproducible_build_url = 10;
  string builder_os = 11;
  string builder_architecture = 12;
  map<string, string> builder_env = 13;
}
```

### CI Pipeline Example (GitHub Actions)

```yaml
name: Release Agent Build

on:
  release:
    types: [published]

jobs:
  register-agent:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build agent
        run: cargo build --release

      - name: Compute hash
        id: hash
        run: |
          HASH=$(sha256sum target/release/ciris-agent | cut -d' ' -f1)
          echo "agent_hash=$HASH" >> $GITHUB_OUTPUT

      - name: Register with Registry
        env:
          REGISTRY_URL: api.registry.ciris-services-1.ai:50052
          REGISTRY_ADMIN_TOKEN: ${{ secrets.REGISTRY_ADMIN_TOKEN }}
        run: |
          grpcurl -plaintext \
            -H "Authorization: Bearer $REGISTRY_ADMIN_TOKEN" \
            -d '{
              "agent": {
                "agent_hash": "'$(echo -n ${{ steps.hash.outputs.agent_hash }} | xxd -r -p | base64)'",
                "agent_type": 1,
                "version": {
                  "major": 1,
                  "minor": 0,
                  "patch": 0
                },
                "base_capabilities": ["domain:general:query"],
                "max_autonomy_tier": 2,
                "build_timestamp": '$(date +%s)',
                "source_repo": "https://github.com/${{ github.repository }}",
                "source_commit": "${{ github.sha }}"
              },
              "attestation": {
                "provenance": {
                  "builder_id": "github-actions",
                  "invocation_id": "${{ github.run_id }}",
                  "source_uri": "https://github.com/${{ github.repository }}",
                  "source_commit": "${{ github.sha }}",
                  "source_branch": "${{ github.ref_name }}",
                  "builder_os": "ubuntu-latest",
                  "builder_architecture": "x86_64"
                }
              }
            }' \
            $REGISTRY_URL ciris.registry.v1.RegistryAdminService/RegisterAgent
```

### Batch Registration

For multi-target builds, use `BatchRegisterAgents` (max 1000 per call):

```protobuf
rpc BatchRegisterAgents(BatchRegisterAgentsRequest) returns (BatchRegisterAgentsResponse);

message BatchRegisterAgentsRequest {
  repeated AgentRecord agents = 1;
  bool include_attestations = 2;
}
```

### Lookup Flow (Runtime)

Once registered, agents are verified at runtime:

```
1. CIRISVerify computes SHA-256 of loaded agent binary
2. Calls LookupAgent(agent_hash) → AgentRecord
3. If not found → COMMUNITY tier (restricted capabilities)
4. If found → effective_caps = agent.base_capabilities ∩ partner.granted - partner.denied
```

---

---

## Play Integrity Verification (Android)

CIRISRegistry supports Google Play Integrity API verification for Android clients.

### GET /v1/integrity/nonce

Generate a cryptographically secure nonce for Play Integrity verification.

**Query Parameters:**
- `context` (optional) - Context for the nonce (e.g., "purchase", "login", "credit_check")

**Response:**
```json
{
  "nonce": "dGVzdC1ub25jZS1iYXNlNjQ",
  "expires_at": "2026-02-22T12:05:00Z"
}
```

**Notes:**
- Nonce is Base64 URL-safe encoded (NO_PADDING)
- Single-use (consumed on verification)
- Expires in 5 minutes

---

### POST /v1/integrity/verify

Verify a Play Integrity token from Android.

**Request:**
```json
{
  "integrity_token": "<encrypted-token-from-android>",
  "nonce": "<nonce-from-previous-step>"
}
```

**Response:**
```json
{
  "verified": true,
  "request_details": { ... },
  "device_integrity": {
    "meets_strong_integrity": true,
    "meets_device_integrity": true,
    "meets_basic_integrity": true,
    "verdicts": ["MEETS_STRONG_INTEGRITY", "MEETS_DEVICE_INTEGRITY", "MEETS_BASIC_INTEGRITY"]
  },
  "app_integrity": {
    "verdict": "PLAY_RECOGNIZED",
    "package_name": "ai.ciris.app",
    "certificate_sha256_digest": ["..."],
    "version_code": 42
  },
  "account_details": {
    "licensing_verdict": "LICENSED"
  },
  "error": null
}
```

**Verification Logic:**
- Device OK: `meets_basic_integrity` OR `meets_device_integrity` OR `meets_strong_integrity`
- App OK: verdict is `PLAY_RECOGNIZED` OR `UNRECOGNIZED_VERSION`
- `verified = device_ok && app_ok`

**Configuration:**
Requires `PLAY_INTEGRITY_SERVICE_ACCOUNT` environment variable (JSON service account credentials).

---

### POST /v1/integrity/auth

Combined JWT + Play Integrity verification for high-security operations.

**Headers:**
```
Authorization: Bearer <google_id_token>
```

**Request:**
```json
{
  "integrity_token": "<encrypted-token>",
  "nonce": "<nonce>"
}
```

**Response:**
```json
{
  "authenticated": true,
  "integrity_verified": true,
  "user_id": null,
  "email": null,
  "device_integrity": { ... },
  "app_integrity": { ... },
  "authorized": true,
  "reason": null
}
```

**Use For:**
- First app launch / registration
- Before processing payments
- Granting premium features
- Periodic verification (once per session)

---

### Play Integrity Flow (Android Client)

```
1. App → GET /v1/integrity/nonce?context=purchase
   ← { "nonce": "abc123...", "expires_at": "..." }

2. App → Google Play Integrity API (with nonce)
   ← integrity_token (encrypted)

3. App → POST /v1/integrity/verify
   → { "integrity_token": "...", "nonce": "abc123..." }
   ← { "verified": true, "device_integrity": {...} }
```

---

## Security Notes

1. **Hybrid Signatures:** All manifests use Ed25519 + ML-DSA-65 (post-quantum)
2. **Multi-Source Validation:** Steward key requires 2-of-3 source agreement
3. **Fail-Secure:** Unknown agents default to COMMUNITY tier
4. **Immutability:** Manifests cannot be modified once registered for a version+target
5. **Revocation:** Any REVOKED signal triggers immediate enforcement
6. **Play Integrity:** Nonces are single-use and expire in 5 minutes to prevent replay attacks
