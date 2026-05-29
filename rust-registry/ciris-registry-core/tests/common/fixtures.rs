//! Test fixtures and data generators
//!
//! Provides reusable test data similar to pytest fixtures.

use sha2::{Digest, Sha256};

/// Generate a deterministic agent hash from a string identifier
pub fn agent_hash(id: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(format!("test-agent-{}", id).as_bytes());
    hasher.finalize().to_vec()
}

/// Generate a deterministic partner ID
pub fn partner_id(id: &str) -> String {
    format!("test-partner-{}", id)
}

/// Standard test capabilities for different agent types
pub mod capabilities {
    pub const MEDICAL_TRIAGE: &str = "domain:medical:triage";
    pub const MEDICAL_DIAGNOSIS: &str = "domain:medical:diagnosis";
    pub const MEDICAL_TREATMENT: &str = "domain:medical:treatment";
    pub const LEGAL_RESEARCH: &str = "domain:legal:research";
    pub const LEGAL_DOCUMENT: &str = "domain:legal:document_review";
    pub const FINANCIAL_ANALYSIS: &str = "domain:financial:analysis";
    pub const COMMUNITY_WELLNESS: &str = "domain:community:wellness";
    pub const TEXT_GENERATION: &str = "modality:text:generation";
    pub const AUTONOMY_A0: &str = "autonomy:A0:advisory";
    pub const AUTONOMY_A1: &str = "autonomy:A1:limited";
    pub const AUTONOMY_A2: &str = "autonomy:A2:moderate";
    pub const AUTONOMY_A3: &str = "autonomy:A3:high";

    /// All valid capability strings for testing
    pub fn all() -> Vec<&'static str> {
        vec![
            MEDICAL_TRIAGE,
            MEDICAL_DIAGNOSIS,
            MEDICAL_TREATMENT,
            LEGAL_RESEARCH,
            LEGAL_DOCUMENT,
            FINANCIAL_ANALYSIS,
            COMMUNITY_WELLNESS,
            TEXT_GENERATION,
            AUTONOMY_A0,
            AUTONOMY_A1,
            AUTONOMY_A2,
            AUTONOMY_A3,
        ]
    }

    /// Medical capabilities subset
    pub fn medical() -> Vec<&'static str> {
        vec![MEDICAL_TRIAGE, MEDICAL_DIAGNOSIS, MEDICAL_TREATMENT]
    }

    /// Community/basic capabilities
    pub fn community() -> Vec<&'static str> {
        vec![COMMUNITY_WELLNESS, TEXT_GENERATION, AUTONOMY_A0]
    }
}

/// Test tag generator for cleanup
pub fn test_tag(suite: &str) -> String {
    format!(
        "test-{}-{}",
        suite,
        uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
    )
}

/// Agent status constants (matching proto enums)
pub mod status {
    pub const UNSPECIFIED: i32 = 0;
    pub const ACTIVE: i32 = 1;
    pub const DEPRECATED: i32 = 2;
    pub const REVOKED: i32 = 3;
}

/// Agent type constants (matching proto enums)
pub mod agent_type {
    pub const UNSPECIFIED: i32 = 0;
    pub const CIRISCARE: i32 = 1;
    pub const CIRISMEDICAL: i32 = 2;
    pub const CIRISLEGAL: i32 = 3;
    pub const CIRISFINANCIAL: i32 = 4;
}

/// Autonomy tier constants (matching proto enums)
pub mod autonomy {
    pub const UNSPECIFIED: i32 = 0;
    pub const A0_ADVISORY: i32 = 1;
    pub const A1_LIMITED: i32 = 2;
    pub const A2_MODERATE: i32 = 3;
    pub const A3_HIGH: i32 = 4;
    pub const A4_CRITICAL: i32 = 5;
}

/// License type constants (matching proto enums)
pub mod license_type {
    pub const UNSPECIFIED: i32 = 0;
    pub const COMMUNITY: i32 = 1;
    pub const COMMUNITY_PLUS: i32 = 2;
    pub const PROFESSIONAL_MEDICAL: i32 = 3;
    pub const PROFESSIONAL_LEGAL: i32 = 4;
    pub const PROFESSIONAL_FINANCIAL: i32 = 5;
    pub const PROFESSIONAL_FULL: i32 = 6;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_hash_deterministic() {
        let hash1 = agent_hash("foo");
        let hash2 = agent_hash("foo");
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32); // SHA-256 = 32 bytes
    }

    #[test]
    fn test_agent_hash_unique() {
        let hash1 = agent_hash("foo");
        let hash2 = agent_hash("bar");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_capabilities_all() {
        let all = capabilities::all();
        assert!(all.len() >= 10);
        assert!(all.contains(&capabilities::MEDICAL_TRIAGE));
    }
}
