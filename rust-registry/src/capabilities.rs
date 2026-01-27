//! Capability management and intersection logic
//!
//! This module implements the core capability logic:
//! - `effective = agent ∩ partner.granted - partner.denied`
//!
//! Capabilities follow the hierarchical namespace:
//! - `domain:<domain>:<capability>` (e.g., `domain:medical:triage`)
//! - `modality:<modality>:<feature>` (e.g., `modality:text:generation`)
//! - `autonomy:<tier>:<action>` (e.g., `autonomy:A2:moderate`)

use std::collections::HashSet;

/// Calculate the intersection of two capability sets
///
/// Returns capabilities present in both sets (sorted, deduplicated)
pub fn intersect(a: &[String], b: &[String]) -> Vec<String> {
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
///
/// # Arguments
/// * `agent_caps` - Base capabilities the agent supports
/// * `granted` - Capabilities the partner license grants
/// * `denied` - Capabilities explicitly denied by the partner
///
/// # Returns
/// Effective capability set after applying all rules
pub fn calculate_effective(
    agent_caps: &[String],
    granted: &[String],
    denied: &[String],
) -> Vec<String> {
    // Step 1: Intersection of agent and granted
    let intersection = intersect(agent_caps, granted);

    // Step 2: Remove denied capabilities
    let denied_set: HashSet<_> = denied.iter().collect();

    let mut result: Vec<String> = intersection
        .into_iter()
        .filter(|cap| !denied_set.contains(cap))
        .collect();

    result.sort();
    result
}

/// Calculate effective autonomy tier as the minimum of agent and partner tiers
///
/// Autonomy tiers: A0 (1) < A1 (2) < A2 (3) < A3 (4) < A4 (5)
pub fn effective_autonomy(agent_tier: i32, partner_tier: i32) -> i32 {
    std::cmp::min(agent_tier, partner_tier)
}

/// Check if a capability string is valid (follows namespace convention)
pub fn is_valid_capability(cap: &str) -> bool {
    let parts: Vec<&str> = cap.split(':').collect();

    if parts.len() < 3 {
        return false;
    }

    matches!(parts[0], "domain" | "modality" | "autonomy")
}

/// Parse capability domain from a capability string
pub fn capability_domain(cap: &str) -> Option<&str> {
    let parts: Vec<&str> = cap.split(':').collect();
    if parts.len() >= 2 {
        Some(parts[1])
    } else {
        None
    }
}

/// Filter capabilities by domain
pub fn filter_by_domain(caps: &[String], domain: &str) -> Vec<String> {
    caps.iter()
        .filter(|c| capability_domain(c) == Some(domain))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_basic() {
        let a = vec!["cap1".to_string(), "cap2".to_string(), "cap3".to_string()];
        let b = vec!["cap2".to_string(), "cap3".to_string(), "cap4".to_string()];

        let result = intersect(&a, &b);
        assert_eq!(result, vec!["cap2", "cap3"]);
    }

    #[test]
    fn test_intersect_empty() {
        let a: Vec<String> = vec![];
        let b = vec!["cap1".to_string()];

        assert_eq!(intersect(&a, &b), Vec::<String>::new());
        assert_eq!(intersect(&b, &a), Vec::<String>::new());
    }

    #[test]
    fn test_intersect_disjoint() {
        let a = vec!["cap1".to_string()];
        let b = vec!["cap2".to_string()];

        assert_eq!(intersect(&a, &b), Vec::<String>::new());
    }

    #[test]
    fn test_calculate_effective() {
        let agent = vec![
            "domain:medical:triage".to_string(),
            "domain:medical:diagnosis".to_string(),
            "domain:legal:research".to_string(),
        ];
        let granted = vec![
            "domain:medical:triage".to_string(),
            "domain:medical:diagnosis".to_string(),
            "domain:financial:analysis".to_string(),
        ];
        let denied = vec!["domain:medical:diagnosis".to_string()];

        let result = calculate_effective(&agent, &granted, &denied);

        // Should include triage (in both, not denied)
        // Should NOT include diagnosis (denied)
        // Should NOT include legal:research (not in granted)
        // Should NOT include financial:analysis (not in agent)
        assert_eq!(result, vec!["domain:medical:triage"]);
    }

    #[test]
    fn test_effective_autonomy() {
        assert_eq!(effective_autonomy(3, 5), 3); // Agent lower
        assert_eq!(effective_autonomy(5, 3), 3); // Partner lower
        assert_eq!(effective_autonomy(3, 3), 3); // Equal
        assert_eq!(effective_autonomy(1, 5), 1); // Agent much lower
    }

    #[test]
    fn test_is_valid_capability() {
        assert!(is_valid_capability("domain:medical:triage"));
        assert!(is_valid_capability("modality:text:generation"));
        assert!(is_valid_capability("autonomy:A2:moderate"));

        assert!(!is_valid_capability("invalid"));
        assert!(!is_valid_capability("domain:medical")); // Too short
        assert!(!is_valid_capability("unknown:foo:bar")); // Invalid namespace
    }

    #[test]
    fn test_capability_domain() {
        assert_eq!(capability_domain("domain:medical:triage"), Some("medical"));
        assert_eq!(capability_domain("modality:text:generation"), Some("text"));
        assert_eq!(capability_domain("invalid"), None);
    }

    #[test]
    fn test_filter_by_domain() {
        let caps = vec![
            "domain:medical:triage".to_string(),
            "domain:medical:diagnosis".to_string(),
            "domain:legal:research".to_string(),
            "modality:text:generation".to_string(),
        ];

        let medical = filter_by_domain(&caps, "medical");
        assert_eq!(medical.len(), 2);
        assert!(medical.contains(&"domain:medical:triage".to_string()));
        assert!(medical.contains(&"domain:medical:diagnosis".to_string()));
    }
}
