//! Property-based tests for capability logic
//!
//! Tests mathematical invariants of capability intersection and effective
//! capability calculation using proptest.

mod common;

use proptest::prelude::*;
use common::strategies::*;
use std::collections::HashSet;

// ============================================================================
// Capability Logic (duplicated from src/capabilities.rs for testing isolation)
// ============================================================================

/// Calculate the intersection of two capability sets
fn intersect(a: &[String], b: &[String]) -> Vec<String> {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();

    let mut result: Vec<String> = set_a
        .intersection(&set_b)
        .map(|s| (*s).clone())
        .collect();

    result.sort();
    result
}

/// Calculate effective capabilities using the formula:
/// `effective = agent ∩ partner.granted - partner.denied`
fn calculate_effective(
    agent_caps: &[String],
    granted: &[String],
    denied: &[String],
) -> Vec<String> {
    let intersection = intersect(agent_caps, granted);
    let denied_set: HashSet<_> = denied.iter().collect();

    let mut result: Vec<String> = intersection
        .into_iter()
        .filter(|cap| !denied_set.contains(cap))
        .collect();

    result.sort();
    result
}

/// Calculate effective autonomy tier as the minimum
fn effective_autonomy(agent_tier: i32, partner_tier: i32) -> i32 {
    std::cmp::min(agent_tier, partner_tier)
}

/// Check if a capability string is valid
fn is_valid_capability(cap: &str) -> bool {
    let parts: Vec<&str> = cap.split(':').collect();
    if parts.len() < 3 {
        return false;
    }
    matches!(parts[0], "domain" | "modality" | "autonomy")
}

// ============================================================================
// Property-Based Tests
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 512,
        max_shrink_iters: 2000,
        ..ProptestConfig::default()
    })]

    // =========================================================================
    // Intersection Properties
    // =========================================================================

    /// Property: Intersection is commutative
    ///
    /// ∀ A, B: A ∩ B = B ∩ A
    #[test]
    fn intersection_is_commutative(
        a in capability_set_strategy(),
        b in capability_set_strategy()
    ) {
        let ab = intersect(&a, &b);
        let ba = intersect(&b, &a);
        prop_assert_eq!(ab, ba, "Intersection should be commutative");
    }

    /// Property: Intersection is associative
    ///
    /// ∀ A, B, C: (A ∩ B) ∩ C = A ∩ (B ∩ C)
    #[test]
    fn intersection_is_associative(
        a in capability_set_strategy(),
        b in capability_set_strategy(),
        c in capability_set_strategy()
    ) {
        let ab_c = intersect(&intersect(&a, &b), &c);
        let a_bc = intersect(&a, &intersect(&b, &c));
        prop_assert_eq!(ab_c, a_bc, "Intersection should be associative");
    }

    /// Property: Intersection is idempotent
    ///
    /// ∀ A: A ∩ A = A
    #[test]
    fn intersection_is_idempotent(a in capability_set_strategy()) {
        let aa = intersect(&a, &a);
        let mut expected = a.clone();
        expected.sort();
        expected.dedup();
        prop_assert_eq!(aa, expected, "Intersection with self should return self");
    }

    /// Property: Empty set is identity element for intersection
    ///
    /// ∀ A: A ∩ ∅ = ∅
    #[test]
    fn empty_intersection_is_empty(a in capability_set_strategy()) {
        let empty: Vec<String> = vec![];
        let result = intersect(&a, &empty);
        prop_assert!(result.is_empty(), "Intersection with empty set should be empty");
    }

    /// Property: Intersection result is subset of both inputs
    ///
    /// ∀ A, B: (A ∩ B) ⊆ A ∧ (A ∩ B) ⊆ B
    #[test]
    fn intersection_is_subset_of_both(
        a in capability_set_strategy(),
        b in capability_set_strategy()
    ) {
        let result = intersect(&a, &b);

        for cap in &result {
            prop_assert!(
                a.contains(cap),
                "Intersection element {} should be in first set", cap
            );
            prop_assert!(
                b.contains(cap),
                "Intersection element {} should be in second set", cap
            );
        }
    }

    /// Property: Intersection result contains only common elements
    #[test]
    fn intersection_contains_only_common(
        a in capability_set_strategy(),
        b in capability_set_strategy()
    ) {
        let result = intersect(&a, &b);

        for cap in &result {
            prop_assert!(a.contains(cap) && b.contains(cap));
        }

        for cap in &a {
            if b.contains(cap) {
                prop_assert!(result.contains(cap),
                    "Common element {} should be in intersection", cap);
            }
        }
    }

    // =========================================================================
    // Effective Capability Properties
    // =========================================================================

    /// Property: Denied capabilities are never in effective set
    #[test]
    fn denied_never_in_effective(
        agent in capability_set_strategy(),
        granted in capability_set_strategy(),
        denied in capability_set_strategy()
    ) {
        let effective = calculate_effective(&agent, &granted, &denied);

        for cap in &denied {
            prop_assert!(
                !effective.contains(cap),
                "Denied capability {} should not be in effective set", cap
            );
        }
    }

    /// Property: Effective set is subset of agent capabilities
    #[test]
    fn effective_subset_of_agent(
        agent in capability_set_strategy(),
        granted in capability_set_strategy(),
        denied in capability_set_strategy()
    ) {
        let effective = calculate_effective(&agent, &granted, &denied);

        for cap in &effective {
            prop_assert!(
                agent.contains(cap),
                "Effective capability {} must be in agent capabilities", cap
            );
        }
    }

    /// Property: Effective set is subset of granted capabilities
    #[test]
    fn effective_subset_of_granted(
        agent in capability_set_strategy(),
        granted in capability_set_strategy(),
        denied in capability_set_strategy()
    ) {
        let effective = calculate_effective(&agent, &granted, &denied);

        for cap in &effective {
            prop_assert!(
                granted.contains(cap),
                "Effective capability {} must be in granted capabilities", cap
            );
        }
    }

    /// Property: Empty denied set means effective = agent ∩ granted
    #[test]
    fn empty_denied_is_pure_intersection(
        agent in capability_set_strategy(),
        granted in capability_set_strategy()
    ) {
        let empty_denied: Vec<String> = vec![];
        let effective = calculate_effective(&agent, &granted, &empty_denied);
        let pure_intersection = intersect(&agent, &granted);

        prop_assert_eq!(effective, pure_intersection);
    }

    /// Property: Adding to denied set can only shrink effective set
    #[test]
    fn more_denied_means_fewer_effective(
        agent in capability_set_strategy(),
        granted in capability_set_strategy(),
        denied1 in capability_set_strategy(),
        extra_denied in capability_set_strategy()
    ) {
        let mut denied2 = denied1.clone();
        denied2.extend(extra_denied);
        denied2.sort();
        denied2.dedup();

        let effective1 = calculate_effective(&agent, &granted, &denied1);
        let effective2 = calculate_effective(&agent, &granted, &denied2);

        for cap in &effective2 {
            prop_assert!(
                effective1.contains(cap),
                "With more denied, {} should not be added to effective", cap
            );
        }
    }

    // =========================================================================
    // Autonomy Tier Properties
    // =========================================================================

    /// Property: Effective autonomy is commutative
    #[test]
    fn autonomy_is_commutative(
        agent_tier in autonomy_tier_strategy(),
        partner_tier in autonomy_tier_strategy()
    ) {
        let ab = effective_autonomy(agent_tier, partner_tier);
        let ba = effective_autonomy(partner_tier, agent_tier);
        prop_assert_eq!(ab, ba);
    }

    /// Property: Effective autonomy is always ≤ agent tier
    #[test]
    fn autonomy_bounded_by_agent(
        agent_tier in autonomy_tier_strategy(),
        partner_tier in autonomy_tier_strategy()
    ) {
        let effective = effective_autonomy(agent_tier, partner_tier);
        prop_assert!(effective <= agent_tier);
    }

    /// Property: Effective autonomy is always ≤ partner tier
    #[test]
    fn autonomy_bounded_by_partner(
        agent_tier in autonomy_tier_strategy(),
        partner_tier in autonomy_tier_strategy()
    ) {
        let effective = effective_autonomy(agent_tier, partner_tier);
        prop_assert!(effective <= partner_tier);
    }

    /// Property: Effective autonomy equals the minimum
    #[test]
    fn autonomy_equals_minimum(
        agent_tier in autonomy_tier_strategy(),
        partner_tier in autonomy_tier_strategy()
    ) {
        let effective = effective_autonomy(agent_tier, partner_tier);
        let expected = std::cmp::min(agent_tier, partner_tier);
        prop_assert_eq!(effective, expected);
    }

    // =========================================================================
    // Capability Validation Properties
    // =========================================================================

    /// Property: All capabilities from strategy are valid
    #[test]
    fn strategy_capabilities_are_valid(cap in capability_strategy()) {
        prop_assert!(
            is_valid_capability(&cap),
            "Generated capability '{}' should be valid", cap
        );
    }
}

/// Edge case tests (not property-based)
mod edge_cases {
    use super::*;

    #[test]
    fn test_single_element_intersection() {
        let a = vec!["domain:medical:triage".to_string()];
        let b = vec!["domain:medical:triage".to_string()];
        let result = intersect(&a, &b);
        assert_eq!(result, vec!["domain:medical:triage"]);
    }

    #[test]
    fn test_all_denied() {
        let agent = vec![
            "domain:medical:triage".to_string(),
            "domain:medical:diagnosis".to_string(),
        ];
        let granted = agent.clone();
        let denied = agent.clone();

        let effective = calculate_effective(&agent, &granted, &denied);
        assert!(effective.is_empty(), "All capabilities denied should result in empty set");
    }

    #[test]
    fn test_no_overlap() {
        let agent = vec!["domain:medical:triage".to_string()];
        let granted = vec!["domain:legal:research".to_string()];
        let denied: Vec<String> = vec![];

        let effective = calculate_effective(&agent, &granted, &denied);
        assert!(effective.is_empty(), "No overlap should result in empty set");
    }

    #[test]
    fn test_deny_non_granted() {
        let agent = vec!["domain:medical:triage".to_string()];
        let granted = vec!["domain:medical:triage".to_string()];
        let denied = vec!["domain:legal:research".to_string()];

        let effective = calculate_effective(&agent, &granted, &denied);
        assert_eq!(effective, vec!["domain:medical:triage"]);
    }

    #[test]
    fn test_autonomy_same_tier() {
        assert_eq!(effective_autonomy(3, 3), 3);
    }

    #[test]
    fn test_autonomy_edge_tiers() {
        assert_eq!(effective_autonomy(1, 5), 1);
        assert_eq!(effective_autonomy(5, 1), 1);
        assert_eq!(effective_autonomy(5, 5), 5);
        assert_eq!(effective_autonomy(1, 1), 1);
    }

    #[test]
    fn test_valid_capability_formats() {
        assert!(is_valid_capability("domain:medical:triage"));
        assert!(is_valid_capability("modality:text:generation"));
        assert!(is_valid_capability("autonomy:A2:moderate"));

        assert!(!is_valid_capability("invalid"));
        assert!(!is_valid_capability("domain:medical"));
        assert!(!is_valid_capability("unknown:foo:bar"));
    }
}
