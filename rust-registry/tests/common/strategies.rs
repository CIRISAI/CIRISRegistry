//! Proptest strategies for property-based testing
//!
//! These strategies generate arbitrary test data, similar to Hypothesis strategies.

use super::fixtures::capabilities;
use proptest::prelude::*;

/// Strategy for generating valid capability strings
pub fn capability_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(
        capabilities::all()
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>(),
    )
}

/// Strategy for generating a set of capabilities (0-10 items)
pub fn capability_set_strategy() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(capability_strategy(), 0..10).prop_map(|mut v| {
        v.sort();
        v.dedup();
        v
    })
}

/// Strategy for generating agent hashes (32 bytes)
pub fn agent_hash_strategy() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(any::<u8>(), 32)
}

/// Strategy for generating partner IDs
pub fn partner_id_strategy() -> impl Strategy<Value = String> {
    "[a-z]{3,8}-[0-9]{4}".prop_map(|s| format!("partner-{}", s))
}

/// Strategy for valid agent types
pub fn agent_type_strategy() -> impl Strategy<Value = i32> {
    prop::sample::select(vec![1, 2, 3, 4]) // CIRISCARE through CIRISFINANCIAL
}

/// Strategy for valid agent statuses
pub fn agent_status_strategy() -> impl Strategy<Value = i32> {
    prop::sample::select(vec![1, 2, 3]) // ACTIVE, DEPRECATED, REVOKED
}

/// Strategy for valid autonomy tiers
pub fn autonomy_tier_strategy() -> impl Strategy<Value = i32> {
    prop::sample::select(vec![1, 2, 3, 4, 5]) // A0 through A4
}

/// Strategy for valid license types
pub fn license_type_strategy() -> impl Strategy<Value = i32> {
    prop::sample::select(vec![1, 2, 3, 4, 5, 6]) // COMMUNITY through PROFESSIONAL_FULL
}

/// Strategy for semantic versions
pub fn version_strategy() -> impl Strategy<Value = (u32, u32, u32)> {
    (0u32..100, 0u32..100, 0u32..1000)
}

/// Strategy for test data that should be cleaned up
pub fn test_tag_strategy() -> impl Strategy<Value = String> {
    "[a-z]{5,10}".prop_map(|s| format!("proptest-{}", s))
}

/// Strategy for arbitrary binary data (for signature testing)
pub fn binary_data_strategy(min: usize, max: usize) -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(any::<u8>(), min..max)
}

/// Strategy for nonce generation (32+ bytes)
pub fn nonce_strategy() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(any::<u8>(), 32..64)
}

/// Strategy for timestamps (reasonable range)
pub fn timestamp_strategy() -> impl Strategy<Value = i64> {
    // 2020-01-01 to 2030-01-01
    1577836800i64..1893456000i64
}

/// Strategy for page sizes
pub fn page_size_strategy() -> impl Strategy<Value = i32> {
    prop::sample::select(vec![10, 25, 50, 100])
}

// Strategy tests are run via proptest! macro in property test files
