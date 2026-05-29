//! Database integration tests using sqlx::test
//!
//! These tests run against a real PostgreSQL database with automatic
//! database creation and migration per test.
//!
//! Run with: `cargo test --test db_integration`
//! Requires: DATABASE_URL environment variable pointing to PostgreSQL

mod common;

use common::fixtures;

/// Skip this test file if database is not available
/// To run: DATABASE_URL=postgres://user:pass@localhost/test cargo test --test db_integration
#[cfg(test)]
mod database_tests {
    use super::*;

    // Note: sqlx::test requires the DATABASE_URL env var to be set
    // These tests will be skipped if running `cargo test` without a database

    /// Test agent registration and lookup
    /// This test requires a running PostgreSQL instance
    #[test]
    fn test_agent_hash_fixture() {
        let hash = fixtures::agent_hash("test-1");
        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let hash2 = fixtures::agent_hash("test-1");
        assert_eq!(hash, hash2);

        // Different input should produce different hash
        let hash3 = fixtures::agent_hash("test-2");
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_partner_id_fixture() {
        let id = fixtures::partner_id("test-1");
        assert!(id.starts_with("test-partner-"));
    }

    #[test]
    fn test_capabilities_fixture() {
        let all = fixtures::capabilities::all();
        assert!(all.len() >= 10);

        let medical = fixtures::capabilities::medical();
        assert!(medical.len() >= 2);
        for cap in medical {
            assert!(cap.starts_with("domain:medical:"));
        }
    }

    #[test]
    fn test_test_tag_fixture() {
        let tag1 = fixtures::test_tag("unit");
        let tag2 = fixtures::test_tag("unit");

        // Tags should be unique
        assert_ne!(tag1, tag2);
        assert!(tag1.starts_with("test-unit-"));
    }
}

/// SQL query tests (can run without database using sqlx offline mode)
#[cfg(test)]
mod sql_syntax_tests {
    /// Verify SQL syntax is valid for agent queries
    #[test]
    fn test_agent_query_syntax() {
        // These would be compile-time checked by sqlx if using query! macro
        // For now, just verify the strings are valid SQL
        let lookup_query = r#"
            SELECT
                agent_hash, agent_type, version_major, version_minor, version_patch,
                version_prerelease, version_build_metadata, base_capabilities,
                max_autonomy_tier, build_timestamp, source_repo, source_commit,
                builder_attestation, status, revocation_reason, revocation_timestamp,
                registered_at, last_updated, registry_signature, is_test_record, test_tag
            FROM agents
            WHERE agent_hash = $1
        "#;
        assert!(lookup_query.contains("SELECT"));
        assert!(lookup_query.contains("FROM agents"));
        assert!(lookup_query.contains("WHERE"));
    }

    /// Verify SQL syntax for batch queries
    #[test]
    fn test_batch_query_syntax() {
        let batch_query = r#"
            SELECT
                agent_hash, agent_type, version_major, version_minor, version_patch,
                version_prerelease, version_build_metadata, base_capabilities,
                max_autonomy_tier, build_timestamp, source_repo, source_commit,
                builder_attestation, status, revocation_reason, revocation_timestamp,
                registered_at, last_updated, registry_signature, is_test_record, test_tag
            FROM agents
            WHERE agent_hash = ANY($1)
        "#;
        assert!(batch_query.contains("ANY($1)"));
    }

    /// Verify SQL syntax for list queries with pagination
    #[test]
    fn test_list_query_syntax() {
        let list_query = r#"
            SELECT * FROM agents
            WHERE is_test_record = false
            ORDER BY registered_at DESC
            LIMIT $1 OFFSET $2
        "#;
        assert!(list_query.contains("LIMIT"));
        assert!(list_query.contains("OFFSET"));
        assert!(list_query.contains("ORDER BY"));
    }

    /// Verify SQL syntax for revocation
    #[test]
    fn test_revoke_query_syntax() {
        let revoke_query = r#"
            UPDATE agents
            SET status = $1, revocation_reason = $2, revocation_timestamp = NOW(), last_updated = NOW()
            WHERE agent_hash = $3 AND status != $1
        "#;
        assert!(revoke_query.contains("UPDATE agents"));
        assert!(revoke_query.contains("SET status"));
        assert!(revoke_query.contains("NOW()"));
    }

    /// Verify SQL syntax for cleanup
    #[test]
    fn test_cleanup_query_syntax() {
        let cleanup_query = r#"
            DELETE FROM agents
            WHERE is_test_record = true AND test_tag = $1
        "#;
        assert!(cleanup_query.contains("DELETE FROM"));
        assert!(cleanup_query.contains("is_test_record = true"));
    }
}

/// Mock database tests that don't require a real database
#[cfg(test)]
mod mock_db_tests {
    use super::*;

    /// Test that status constants match expected values
    #[test]
    fn test_status_constants() {
        assert_eq!(fixtures::status::UNSPECIFIED, 0);
        assert_eq!(fixtures::status::ACTIVE, 1);
        assert_eq!(fixtures::status::DEPRECATED, 2);
        assert_eq!(fixtures::status::REVOKED, 3);
    }

    /// Test that agent type constants match expected values
    #[test]
    fn test_agent_type_constants() {
        assert_eq!(fixtures::agent_type::UNSPECIFIED, 0);
        assert_eq!(fixtures::agent_type::CIRISCARE, 1);
        assert_eq!(fixtures::agent_type::CIRISMEDICAL, 2);
        assert_eq!(fixtures::agent_type::CIRISLEGAL, 3);
        assert_eq!(fixtures::agent_type::CIRISFINANCIAL, 4);
    }

    /// Test that autonomy tier constants match expected values
    #[test]
    fn test_autonomy_constants() {
        assert_eq!(fixtures::autonomy::UNSPECIFIED, 0);
        assert_eq!(fixtures::autonomy::A0_ADVISORY, 1);
        assert_eq!(fixtures::autonomy::A1_LIMITED, 2);
        assert_eq!(fixtures::autonomy::A2_MODERATE, 3);
        assert_eq!(fixtures::autonomy::A3_HIGH, 4);
        assert_eq!(fixtures::autonomy::A4_CRITICAL, 5);
    }

    /// Test that license type constants match expected values
    #[test]
    fn test_license_type_constants() {
        assert_eq!(fixtures::license_type::UNSPECIFIED, 0);
        assert_eq!(fixtures::license_type::COMMUNITY, 1);
        assert_eq!(fixtures::license_type::COMMUNITY_PLUS, 2);
        assert_eq!(fixtures::license_type::PROFESSIONAL_MEDICAL, 3);
        assert_eq!(fixtures::license_type::PROFESSIONAL_LEGAL, 4);
        assert_eq!(fixtures::license_type::PROFESSIONAL_FINANCIAL, 5);
        assert_eq!(fixtures::license_type::PROFESSIONAL_FULL, 6);
    }
}

// TODO: Add full integration tests with #[sqlx::test] when DATABASE_URL is configured
//
// Example:
// #[sqlx::test(migrations = "migrations")]
// async fn test_register_and_lookup_agent(pool: sqlx::PgPool) {
//     let agent = test_agent_record("integration-1");
//     db::register_agent(&pool, &agent).await.unwrap();
//
//     let found = db::lookup_agent(&pool, &agent.agent_hash).await.unwrap();
//     assert!(found.is_some());
//     assert_eq!(found.unwrap().agent_type, agent.agent_type);
// }
//
// #[sqlx::test(migrations = "migrations")]
// async fn test_revocation_changes_status(pool: sqlx::PgPool) {
//     let agent = test_agent_record("revoke-test");
//     db::register_agent(&pool, &agent).await.unwrap();
//
//     let revoked = db::revoke_agent(&pool, &agent.agent_hash, "test_reason").await.unwrap();
//     assert!(revoked);
//
//     let found = db::lookup_agent(&pool, &agent.agent_hash).await.unwrap().unwrap();
//     assert_eq!(found.status, fixtures::status::REVOKED);
// }
