# CIRISRegistry Security and Operations Review

**Date**: 2026-01-26
**Reviewed Version**: rust-registry
**Reviewer**: Automated Security Scan + Manual Review

## Executive Summary

This document outlines security findings and operational recommendations for the CIRISRegistry Rust implementation. The review covers environment variable management, authentication configuration, cryptographic settings, and deployment considerations.

---

## Critical Findings

### 1. Hardcoded Default Credentials (CRITICAL)

**Location**: `src/config.rs:108, 129`

**Issue**: Production-sensitive defaults are hardcoded:
```rust
// Line 108
password: env::var("DB_PASSWORD").unwrap_or_else(|_| "ciris_dev".to_string()),

// Line 129
jwt_secret: env::var("JWT_SECRET")
    .unwrap_or_else(|_| "development-secret-do-not-use-in-production".to_string()),
```

**Risk**: If environment variables are not set in production, the service will run with insecure development credentials.

**Recommendation**:
- **Option A**: Fail fast in production if secrets are not provided
- **Option B**: Use distinct required vs optional env vars with runtime validation

**Fix Applied**: See `src/config.rs` modifications below.

---

### 2. SSL Mode Defaults to Disabled (HIGH)

**Location**: `src/config.rs:110`

```rust
sslmode: env::var("DB_SSLMODE").unwrap_or_else(|_| "disable".to_string()),
```

**Risk**: Database connections in production could be unencrypted.

**Recommendation**: Default to `require` or `verify-full` for non-development environments.

---

### 3. Authentication Not Enforced (HIGH)

**Location**: `src/middleware/auth.rs:107-117`

```rust
Err(e) => {
    tracing::warn!("JWT validation failed: {}", e);
    // For now, log but don't reject (to allow gradual migration)
    // In production, uncomment below to enforce auth:
```

**Risk**: Invalid/missing JWT tokens are logged but not rejected.

**Recommendation**: Enable enforcement for production environment.

---

### 4. Docker Compose Uses Development Credentials (MEDIUM)

**Location**: `docker-compose.yml:7-8, 31-32`

```yaml
POSTGRES_PASSWORD: ciris_dev
DB_PASSWORD: ciris_dev
```

**Risk**: Hardcoded credentials in version control.

**Recommendation**: Use environment variable substitution or external secrets.

---

## Environment Variables Reference

### Required for All Environments

| Variable | Description | Default | Notes |
|----------|-------------|---------|-------|
| `ENVIRONMENT` | Runtime environment | `development` | `development`, `staging`, `canary`, `production` |
| `GRPC_PORT` | gRPC server port | `50051` | |
| `HTTP_PORT` | HTTP health/metrics port | `8080` | Set to 0 to disable |

### Database Configuration

| Variable | Description | Default | Production Requirement |
|----------|-------------|---------|------------------------|
| `DB_HOST` | PostgreSQL host | `localhost` | Required |
| `DB_PORT` | PostgreSQL port | `5432` | |
| `DB_USER` | Database user | `ciris` | Required |
| `DB_PASSWORD` | Database password | `ciris_dev` | **MUST BE SET** |
| `DB_NAME` | Database name | `ciris_registry` | |
| `DB_SSLMODE` | SSL mode | `disable` | Use `require` or `verify-full` |
| `DB_MAX_CONNECTIONS` | Max pool connections | `10` | Tune for load |
| `DB_MIN_CONNECTIONS` | Min pool connections | `1` | |

### Authentication Configuration

| Variable | Description | Default | Production Requirement |
|----------|-------------|---------|------------------------|
| `JWT_SECRET` | JWT signing key | `development-secret...` | **MUST BE SET** (min 32 chars) |
| `JWT_ISSUER` | JWT issuer claim | `ciris-registry` | |
| `MTLS_ENABLED` | Enable mTLS | `false` | Recommended `true` |
| `TLS_CERT_PATH` | TLS certificate path | None | Required if mTLS enabled |
| `TLS_KEY_PATH` | TLS private key path | None | Required if mTLS enabled |
| `CA_CERT_PATH` | CA cert for client verification | None | Required if mTLS enabled |

### Cryptographic Configuration

| Variable | Description | Default | Notes |
|----------|-------------|---------|-------|
| `KEY_STORAGE_MODE` | Key storage backend | `memory` | `memory`, `file`, `vault`, `hsm` |
| `ED25519_KEY_PATH` | Ed25519 private key path | None | Required for `file` mode |
| `MLDSA_KEY_PATH` | ML-DSA-65 private key path | None | Required for `file` mode |
| `VAULT_ADDR` | HashiCorp Vault address | None | Required for `vault` mode |

### Logging and Observability

| Variable | Description | Default | Notes |
|----------|-------------|---------|-------|
| `RUST_LOG` | Log level filter | `info` | `trace`, `debug`, `info`, `warn`, `error` |
| `LOG_FORMAT` | Log format | `json` | Currently hardcoded to JSON |

### Missing/Recommended Variables (Not Yet Implemented)

| Variable | Description | Recommended Default |
|----------|-------------|---------------------|
| `RATE_LIMIT_ENABLED` | Enable rate limiting | `true` |
| `RATE_LIMIT_REQUESTS` | Requests per window | `100` |
| `RATE_LIMIT_WINDOW_SECS` | Rate limit window | `60` |
| `CORS_ALLOWED_ORIGINS` | CORS origins | None (gRPC only) |
| `REQUEST_TIMEOUT_SECS` | Max request duration | `30` |
| `GRACEFUL_SHUTDOWN_SECS` | Shutdown timeout | `30` |
| `HEALTH_CHECK_INTERVAL_SECS` | DB health check interval | `30` |

---

## Recommendations

### Immediate Actions (Before Production)

1. **Add Production Secret Validation**
   - Fail startup if `JWT_SECRET` contains "development" or is < 32 characters
   - Fail startup if `DB_PASSWORD` is "ciris_dev" in production

2. **Enable JWT Enforcement**
   - Remove commented-out rejection code in auth middleware for production

3. **Secure Docker Compose**
   - Use `.env` file for credentials (excluded from git)
   - Or use Docker secrets / external secret management

4. **Default SSL Mode**
   - Change default `DB_SSLMODE` to `require` for non-development

### Short-Term Improvements

1. **Add Rate Limiting**
   - Implement request rate limiting per IP/token
   - Add env vars for configuration

2. **Request Timeouts**
   - Add configurable request timeout middleware

3. **mTLS Enforcement**
   - Complete mTLS implementation for service-to-service calls

4. **Audit Logging**
   - Ensure all state-changing operations are logged with user context

### Long-Term Enhancements

1. **HSM Integration**
   - Complete PKCS#11 support for hardware key storage

2. **Vault Integration**
   - Implement async Vault Transit API for key operations

3. **Key Rotation**
   - Implement automated key rotation with zero-downtime

---

## Ansible Variable Updates Required

The following variables need to be added to `deploy/ansible/vars/main.yml` and environment-specific files:

```yaml
# Rust Registry Configuration (add to vars/main.yml)
registry_grpc_port: 50051
registry_http_port: 8080
registry_db_sslmode: "require"  # or verify-full for production
registry_db_max_connections: 10
registry_db_min_connections: 1

# JWT Configuration
registry_jwt_issuer: "ciris-registry"
registry_mtls_enabled: true

# Crypto Configuration
registry_key_storage_mode: "file"  # file, vault, or hsm in production
registry_ed25519_key_path: "{{ app_dir }}/keys/ed25519.key"
registry_mldsa_key_path: "{{ app_dir }}/keys/mldsa65.key"

# Secrets (should come from vault/secrets manager)
# registry_db_password: "{{ vault_db_password }}"
# registry_jwt_secret: "{{ vault_jwt_secret }}"
```

---

## Compliance Notes

### CIRIS Covenant Alignment

This review considers the CIRIS Covenant principles:

1. **Non-maleficence**: Fail-secure defaults ensure unknown/misconfigured deployments don't escalate privileges
2. **Integrity**: Hybrid cryptographic signatures (Ed25519 + ML-DSA-65) provide post-quantum security
3. **Fidelity**: Audit trails and cryptographic guarantees maintain trust commitments

### Data Protection

- No PII is stored in logs (verified)
- Database passwords are not logged (verified)
- JWT tokens are validated but not logged in full (verified)

---

## Appendix: Test Commands

### Verify Environment Variables Are Read

```bash
# Start with explicit settings
ENVIRONMENT=production \
DB_PASSWORD=secure_password \
JWT_SECRET=32-char-minimum-production-secret \
DB_SSLMODE=require \
./ciris-registry
```

### Check for Hardcoded Secrets

```bash
# Search for potential secrets in codebase
grep -rn "password\|secret\|key" --include="*.rs" src/
```

### Verify TLS Configuration

```bash
# Test gRPC with TLS (when enabled)
grpcurl -cacert ca.crt -cert client.crt -key client.key \
  localhost:50051 grpc.health.v1.Health/Check
```
