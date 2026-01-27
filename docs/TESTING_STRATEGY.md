# CIRISRegistry Testing Strategy

**Version:** 1.0.0
**Date:** 2026-01-26

This document proposes a comprehensive testing strategy for CIRISRegistry, inspired by the Hypothesis-based testing used in other CIRIS Python projects.

---

## Executive Summary

| Testing Type | Tool | Hypothesis Equivalent | Purpose |
|--------------|------|----------------------|---------|
| Property-based | **proptest** | Hypothesis | Generate arbitrary inputs, find edge cases |
| Unit | Built-in `#[test]` | pytest | Test individual functions |
| Integration | **testcontainers** | pytest-docker | Real database testing |
| Database | **sqlx::test** | pytest fixtures | Isolated DB per test |
| Mocking | **mockall** | unittest.mock | Dependency isolation |
| Fuzzing | **cargo-fuzz** | hypothesis-fuzz | Security testing |

---

## 1. Property-Based Testing with Proptest

[Proptest](https://github.com/proptest-rs/proptest) is the Rust equivalent of Python's Hypothesis. It generates arbitrary inputs and automatically shrinks failing cases to minimal examples.

### Why Proptest over QuickCheck?

Per [Luca Palmieri's guide](https://lpalmieri.com/posts/an-introduction-to-property-based-testing-in-rust/):

- **Per-value strategies** (not per-type) - more flexible composition
- **Built-in constraints** - generate `0..100` integers directly without newtypes
- **Better shrinking** - aware of constraints, doesn't generate invalid values
- **Failure persistence** - saves failing seeds for regression testing

### Add to Cargo.toml

```toml
[dev-dependencies]
proptest = "1.5"
proptest-derive = "0.5"  # For deriving Arbitrary on structs
```

### Example: Testing Capability Intersection

```rust
use proptest::prelude::*;

// Strategy for generating valid capability strings
fn capability_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("domain:medical:triage".to_string()),
        Just("domain:medical:diagnosis".to_string()),
        Just("domain:legal:research".to_string()),
        Just("domain:financial:analysis".to_string()),
        Just("modality:text:generation".to_string()),
        Just("autonomy:A2:moderate".to_string()),
    ]
}

fn capability_set_strategy() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(capability_strategy(), 0..10)
}

proptest! {
    /// Property: Capability intersection is commutative
    #[test]
    fn intersection_is_commutative(
        a in capability_set_strategy(),
        b in capability_set_strategy()
    ) {
        let ab = intersect_capabilities(&a, &b);
        let ba = intersect_capabilities(&b, &a);
        prop_assert_eq!(ab, ba);
    }

    /// Property: Intersection is subset of both inputs
    #[test]
    fn intersection_is_subset(
        agent_caps in capability_set_strategy(),
        partner_granted in capability_set_strategy()
    ) {
        let effective = intersect_capabilities(&agent_caps, &partner_granted);
        for cap in &effective {
            prop_assert!(agent_caps.contains(cap) || partner_granted.contains(cap));
        }
    }

    /// Property: Denied capabilities are never in result
    #[test]
    fn denied_never_in_result(
        agent_caps in capability_set_strategy(),
        granted in capability_set_strategy(),
        denied in capability_set_strategy()
    ) {
        let effective = calculate_effective_capabilities(&agent_caps, &granted, &denied);
        for cap in &denied {
            prop_assert!(!effective.contains(cap));
        }
    }
}
```

### Example: Testing Cryptographic Invariants

```rust
proptest! {
    /// Property: Sign then verify always succeeds for valid data
    #[test]
    fn sign_verify_roundtrip(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let crypto = HybridCrypto::generate_ephemeral().unwrap();
        let sig = crypto.sign(&data).unwrap();
        let verified = crypto.verify(
            &data,
            &sig,
            &crypto.ed25519_public_key(),
            &crypto.mldsa_public_key(),
        ).unwrap();
        prop_assert!(verified);
    }

    /// Property: Modified data fails verification
    #[test]
    fn tampered_data_fails(
        data in prop::collection::vec(any::<u8>(), 1..1000),
        flip_idx in any::<prop::sample::Index>()
    ) {
        let crypto = HybridCrypto::generate_ephemeral().unwrap();
        let sig = crypto.sign(&data).unwrap();

        // Flip one bit
        let mut tampered = data.clone();
        let idx = flip_idx.index(tampered.len());
        tampered[idx] ^= 0x01;

        let verified = crypto.verify(
            &tampered,
            &sig,
            &crypto.ed25519_public_key(),
            &crypto.mldsa_public_key(),
        ).unwrap();
        prop_assert!(!verified);
    }

    /// Property: Nonces are always unique (probabilistic)
    #[test]
    fn nonces_are_unique(count in 1usize..100) {
        let crypto = HybridCrypto::generate_ephemeral().unwrap();
        let nonces: Vec<Vec<u8>> = (0..count)
            .map(|_| crypto.generate_nonce())
            .collect();

        let unique: std::collections::HashSet<_> = nonces.iter().collect();
        prop_assert_eq!(unique.len(), nonces.len());
    }
}
```

### Failure Persistence

Proptest saves failing cases to `proptest-regressions/` files. **Commit these to git** for regression testing:

```bash
# Add to .gitignore exceptions
!**/proptest-regressions/
```

---

## 2. Database Testing with sqlx::test

[SQLx's test macro](https://docs.rs/sqlx/latest/sqlx/attr.test.html) creates isolated databases per test, similar to pytest fixtures.

### Setup

```rust
// tests/db_tests.rs
use sqlx::PgPool;

#[sqlx::test(migrations = "migrations")]
async fn test_register_agent(pool: PgPool) {
    let agent = AgentRecord {
        agent_hash: vec![0u8; 32],
        agent_type: AgentType::CirisMedical as i32,
        // ...
    };

    db::register_agent(&pool, &agent).await.unwrap();

    let found = db::lookup_agent(&pool, &agent.agent_hash).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().agent_type, AgentType::CirisMedical as i32);
}

#[sqlx::test(migrations = "migrations")]
async fn test_revocation_cascade(pool: PgPool) {
    // Register agent
    let agent = create_test_agent();
    db::register_agent(&pool, &agent).await.unwrap();

    // Revoke it
    let revoked = db::revoke_agent(&pool, &agent.agent_hash, "security_incident").await.unwrap();
    assert!(revoked);

    // Verify status changed
    let found = db::lookup_agent(&pool, &agent.agent_hash).await.unwrap().unwrap();
    assert_eq!(found.status, AgentStatus::AgentRevoked as i32);
}
```

### Database Test Fixtures

Create reusable test data generators:

```rust
// tests/fixtures.rs
pub fn test_agent(suffix: &str) -> AgentRecord {
    AgentRecord {
        agent_hash: sha256(format!("test-agent-{}", suffix).as_bytes()).to_vec(),
        agent_type: AgentType::CirisCare as i32,
        version: Some(SemanticVersion { major: 1, minor: 0, patch: 0, ..Default::default() }),
        base_capabilities: vec!["domain:community:wellness".to_string()],
        max_autonomy_tier: AutonomyTier::A1Limited as i32,
        status: AgentStatus::AgentActive as i32,
        is_test_record: true,
        test_tag: format!("test-{}", suffix),
        ..Default::default()
    }
}

pub fn test_partner(suffix: &str) -> PartnerRecord {
    PartnerRecord {
        partner_id: format!("test-partner-{}", suffix),
        organization_name: format!("Test Org {}", suffix),
        license_type: LicenseType::ProfessionalMedical as i32,
        // ...
    }
}
```

---

## 3. Integration Testing with Testcontainers

[Testcontainers](https://oneuptime.com/blog/post/2026-01-07-rust-testcontainers/view) spins up real Docker containers for true integration tests.

### Setup

```rust
// tests/integration/mod.rs
use testcontainers::{clients::Cli, images::postgres::Postgres, Container};

struct TestContext {
    _container: Container<'static, Postgres>,
    pool: PgPool,
}

impl TestContext {
    async fn new() -> Self {
        let docker = Cli::default();
        let container = docker.run(Postgres::default());

        let connection_string = format!(
            "postgres://postgres:postgres@localhost:{}/postgres",
            container.get_host_port_ipv4(5432)
        );

        let pool = PgPool::connect(&connection_string).await.unwrap();

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        Self {
            _container: container,
            pool,
        }
    }
}

#[tokio::test]
async fn test_full_verification_flow() {
    let ctx = TestContext::new().await;

    // 1. Register agent
    let agent = test_agent("integration-1");
    db::register_agent(&ctx.pool, &agent).await.unwrap();

    // 2. Register partner
    let partner = test_partner("integration-1");
    db::register_partner(&ctx.pool, &partner).await.unwrap();

    // 3. Verify deployment
    let result = verify_deployment(&ctx.pool, &agent.agent_hash, &partner.partner_id).await;
    assert!(result.is_ok());

    // 4. Revoke and verify fails
    db::revoke_agent(&ctx.pool, &agent.agent_hash, "test").await.unwrap();
    let result = verify_deployment(&ctx.pool, &agent.agent_hash, &partner.partner_id).await;
    assert!(matches!(result, Err(VerifyError::AgentRevoked)));
}
```

---

## 4. gRPC Service Testing

Test the full gRPC stack with tonic's test utilities:

```rust
// tests/grpc_tests.rs
use tonic::Request;

#[tokio::test]
async fn test_register_and_lookup_agent() {
    let (client, _server) = setup_test_server().await;

    // Register
    let req = RegisterAgentRequest {
        agent: Some(test_agent("grpc-1")),
        ..Default::default()
    };
    let resp = client.register_agent(Request::new(req)).await.unwrap();
    assert!(resp.into_inner().success);

    // Lookup
    let req = LookupAgentRequest {
        agent_hash: test_agent("grpc-1").agent_hash,
        ..Default::default()
    };
    let resp = client.lookup_agent(Request::new(req)).await.unwrap();
    let inner = resp.into_inner();
    assert_eq!(inner.status, LookupStatus::Found as i32);
}
```

---

## 5. Mocking with Mockall

Use [mockall](https://github.com/asomers/mockall) for unit testing with dependency isolation:

```rust
// src/services/registry.rs
#[cfg_attr(test, mockall::automock)]
pub trait AgentLookup {
    async fn lookup(&self, hash: &[u8]) -> Result<Option<AgentRow>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_verify_with_mock_db() {
        let mut mock = MockAgentLookup::new();

        mock.expect_lookup()
            .with(eq(vec![0u8; 32]))
            .returning(|_| Ok(Some(AgentRow {
                status: AgentStatus::AgentActive as i32,
                // ...
            })));

        let service = RegistryService::new(mock);
        let result = service.verify_agent(&[0u8; 32]).await;
        assert!(result.is_ok());
    }
}
```

---

## 6. Test Organization

### Directory Structure

```
rust-registry/
├── src/
│   ├── crypto/
│   │   └── mod.rs          # Unit tests inline with #[cfg(test)]
│   ├── db/
│   │   └── agents.rs       # Unit tests inline
│   └── services/
│       └── registry.rs     # Unit tests inline
├── tests/
│   ├── common/
│   │   ├── mod.rs          # Shared test utilities
│   │   └── fixtures.rs     # Test data generators
│   ├── db_tests.rs         # Database integration tests
│   ├── grpc_tests.rs       # gRPC integration tests
│   └── property_tests.rs   # Property-based tests
└── proptest-regressions/   # Committed failure cases
```

### Test Categories

```rust
// Run fast unit tests
cargo test --lib

// Run database tests (requires postgres)
cargo test --test db_tests

// Run property tests (slower, more thorough)
cargo test --test property_tests

// Run all tests
cargo test

// Run with verbose output
cargo test -- --nocapture
```

---

## 7. CI/CD Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_USER: ciris
          POSTGRES_PASSWORD: test
          POSTGRES_DB: ciris_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Run unit tests
        run: cargo test --lib

      - name: Run integration tests
        run: cargo test --test '*'
        env:
          DATABASE_URL: postgres://ciris:test@localhost:5432/ciris_test

      - name: Run property tests (extended)
        run: cargo test --test property_tests -- --test-threads=1
        env:
          PROPTEST_CASES: 1000  # More cases in CI
```

---

## 8. Coverage

Use `cargo-llvm-cov` for coverage reports:

```bash
# Install
cargo install cargo-llvm-cov

# Generate coverage
cargo llvm-cov --html --open

# Enforce minimum coverage
cargo llvm-cov --fail-under-lines 70
```

---

## 9. Recommended Test Priorities

### Phase 1: Critical Path (Week 1)
1. **Cryptographic operations** - sign/verify roundtrips
2. **Capability intersection** - core business logic
3. **Agent/Partner lookup** - database CRUD

### Phase 2: Security (Week 2)
4. **Revocation propagation** - fail-secure behavior
5. **JWT validation** - authentication edge cases
6. **Rate limiting** - abuse prevention

### Phase 3: Edge Cases (Week 3)
7. **Pagination** - boundary conditions
8. **Concurrent access** - race conditions
9. **Error handling** - graceful degradation

### Phase 4: Property Tests (Ongoing)
10. **Invariant testing** - mathematical properties
11. **Fuzzing** - unexpected inputs
12. **Regression tests** - persisted failures

---

## 10. Migration from Hypothesis

| Hypothesis Pattern | Proptest Equivalent |
|-------------------|---------------------|
| `@given(st.integers())` | `proptest! { fn test(x in any::<i32>()) }` |
| `@given(st.text())` | `proptest! { fn test(s in ".*") }` |
| `st.lists(st.integers())` | `prop::collection::vec(any::<i32>(), 0..100)` |
| `@settings(max_examples=1000)` | `#![proptest_config(ProptestConfig { cases: 1000, .. })]` |
| `@example(42)` | Add to `proptest-regressions/` file |
| `assume(x > 0)` | `prop_assume!(x > 0);` |

---

## Sources

- [Proptest - Hypothesis-like testing for Rust](https://github.com/proptest-rs/proptest)
- [An Introduction to Property-Based Testing in Rust](https://lpalmieri.com/posts/an-introduction-to-property-based-testing-in-rust/)
- [Property-Based Testing in Rust with Proptest](https://blog.logrocket.com/property-based-testing-in-rust-with-proptest/)
- [SQLx Test Macro Documentation](https://docs.rs/sqlx/latest/sqlx/attr.test.html)
- [Rust Integration Tests with Testcontainers](https://oneuptime.com/blog/post/2026-01-07-rust-testcontainers/view)
- [Mockall - Mocking Library for Rust](https://github.com/asomers/mockall)
- [Everything You Need to Know About Testing in Rust](https://www.shuttle.dev/blog/2024/03/21/testing-in-rust)
- [Rust Project Primer - Property Testing](https://rustprojectprimer.com/testing/property.html)
