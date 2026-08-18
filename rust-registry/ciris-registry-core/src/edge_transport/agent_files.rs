//! Three-layer `agent_files:*` trust composition per CEG 0.2 §8.1.6.
//!
//! Per CEG 0.2 §8.1.6 Policy F:
//!
//! - **Layer 1 — Canonical**: an `agent_files:*` attestation with
//!   `score >= 0.7` from a `registry-steward-triple` key constitutes the
//!   CIRIS canonical default-trust state. The install endpoint at
//!   `registry.ciris-services-1.ai/install` resolves canonical files via
//!   this rule.
//! - **Layer 2 — Open Contribution**: any federation-key holder may emit
//!   `agent_files:*` attestations. The wire format admits them; consumer
//!   policy decides whether to surface them.
//! - **Layer 3 — Vote-then-trust**: a non-canonical attestation
//!   accumulates NodeCore P4 votes. Consumer policy MAY elevate trust
//!   once an accumulated-weight threshold is reached.
//!
//! **Anti-tricking guarantee**: the canonical-default Layer 1 rule applies
//! at the install endpoint **regardless of attester or vote accumulation**.
//! Third-party `agent_files:*` are reachable only via the explicit "Browse
//! alternatives" path. This binds CIRIS L3C: the federation cannot exempt
//! itself from the rule that newcomers' default trust path is the
//! steward-attested canonical one.
//!
//! This module ships the pure composition function. Input: a list of
//! `agent_files:*` attestations + the set of registry-steward-triple
//! `key_id`s. Output: a typed three-layer result the endpoint serializes
//! into the `AgentFilesResponse` JSON shape.

use std::collections::HashSet;

use crate::federation::Attestation;

/// Minimum score for an attestation to be admitted as canonical Layer 1.
/// Per CEG §8.1.6: "score >= 0.7".
pub const CANONICAL_LAYER_MIN_SCORE: f64 = 0.7;

/// Three-layer composition result for `/v1/agent_files/{kind}`.
///
/// The canonical layer is optional — if no steward-triple attester has
/// scored an `agent_files:*` attestation at or above
/// [`CANONICAL_LAYER_MIN_SCORE`], the layer is absent and the install
/// endpoint MUST fail-secure rather than promote a Layer-2 attester to
/// canonical (the §8.1.6 anti-tricking guarantee).
#[derive(Debug, Clone, PartialEq)]
pub struct AgentFilesTrustComposition {
    /// Layer 1 — the canonical attester's `key_id` if any qualified.
    /// `None` means no canonical default; the install endpoint should
    /// refuse to ship a default and require explicit "Browse alternatives"
    /// selection.
    pub canonical_attester: Option<String>,

    /// Layer 2 — every federation-key holder that emitted an
    /// `agent_files:*` attestation at any score (canonical or not).
    /// Includes the canonical attester if any. Surfaced behind the
    /// explicit "Browse alternatives" disclosure in consumer UI.
    pub open_attesters: Vec<OpenAttester>,

    /// Layer 3 — non-canonical attesters that have accumulated
    /// NodeCore P4 vote weight above a consumer-defined threshold.
    /// Vote accumulation lives outside CEG's wire format; this list is
    /// supplied by the caller (Registry's vote-tally service or
    /// NodeCore's read API), not derived here.
    pub vote_then_trust: Vec<VoteThenTrustAttester>,
}

/// A single Layer-2 attester (any federation-key holder).
#[derive(Debug, Clone, PartialEq)]
pub struct OpenAttester {
    /// The attester's `federation_keys.key_id`.
    pub key_id: String,
    /// The attester's score on the `agent_files:*` attestation. Signed
    /// per the dimension's polarity column ([`crate::federation::types`]).
    pub score: f64,
    /// The attester's own confidence in their score (0.0-1.0).
    pub confidence: f64,
}

/// A single Layer-3 attester (non-canonical with accumulated P4 votes).
#[derive(Debug, Clone, PartialEq)]
pub struct VoteThenTrustAttester {
    /// The attester's `federation_keys.key_id`.
    pub key_id: String,
    /// Accumulated NodeCore P4 vote weight on this attester's
    /// `agent_files:*` attestation.
    pub vote_weight: f64,
}

/// Compose the three layers from a list of `agent_files:*` attestations
/// matching a specific `kind` + optional `platform_or_target`.
///
/// `steward_triple` is the set of `key_id`s that constitute the
/// registry-steward triple (per CEG §10.2 `/v1/steward-key`). An
/// attestation `attesting_key_id` in this set + score >=
/// `CANONICAL_LAYER_MIN_SCORE` qualifies as Layer 1 canonical.
///
/// `vote_weights` maps non-canonical `attesting_key_id` →
/// accumulated P4 vote weight (supplied by NodeCore's read API or
/// Registry's vote-tally service; Phase-3 caller's responsibility).
/// Empty map means no Layer 3 elevations.
///
/// **Ranking rule for Layer 1**: if multiple steward-triple keys have
/// scored the same `agent_files:*` claim, the canonical attester is the
/// one with the highest `score * confidence` product (ties broken by
/// lexicographic `key_id`). Documented here because the §8.1.6 spec
/// doesn't pin the rule explicitly but the install endpoint needs
/// determinism.
pub fn compose_trust_layers(
    attestations: &[Attestation],
    steward_triple: &HashSet<String>,
    vote_weights: &std::collections::HashMap<String, f64>,
) -> AgentFilesTrustComposition {
    let mut canonical_candidates: Vec<(String, f64)> = Vec::new();
    let mut open_attesters: Vec<OpenAttester> = Vec::new();
    let mut vote_then_trust: Vec<VoteThenTrustAttester> = Vec::new();

    for att in attestations {
        let weight = att.weight.unwrap_or(0.0);
        let confidence = 1.0; // CEG envelope `confidence` is in the JSON envelope;
                              // production caller extracts via attestation_envelope.
                              // For Phase-3 minimal helper, weight stands in.

        open_attesters.push(OpenAttester {
            key_id: att.attesting_key_id.clone(),
            score: weight,
            confidence,
        });

        if steward_triple.contains(&att.attesting_key_id)
            && weight >= CANONICAL_LAYER_MIN_SCORE
        {
            canonical_candidates.push((att.attesting_key_id.clone(), weight * confidence));
        } else if let Some(&vote_weight) = vote_weights.get(&att.attesting_key_id) {
            vote_then_trust.push(VoteThenTrustAttester {
                key_id: att.attesting_key_id.clone(),
                vote_weight,
            });
        }
    }

    // Layer 1 selection: highest score*confidence; lexicographic tie-break.
    canonical_candidates.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });
    let canonical_attester = canonical_candidates.into_iter().next().map(|(k, _)| k);

    AgentFilesTrustComposition {
        canonical_attester,
        open_attesters,
        vote_then_trust,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::federation::types::attestation_type;
    use chrono::Utc;

    fn make_attestation(attester: &str, weight: f64) -> Attestation {
        Attestation {
            attestation_id: format!("att-{}", attester),
            attesting_key_id: attester.into(),
            attested_key_id: "agent-files-target".into(),
            attestation_type: attestation_type::SCORES.into(),
            weight: Some(weight),
            asserted_at: Utc::now(),
            expires_at: None,
            attestation_envelope: serde_json::json!({
                "dimension": "agent_files:installer:linux-x86_64",
            }),
            original_content_hash: String::new(),
            scrub_signature_classical: String::new(),
            scrub_signature_pqc: None,
            scrub_key_id: attester.into(),
            scrub_timestamp: Utc::now(),
            additional_scrubs: Vec::new(),
            pqc_completed_at: None,
            persist_row_hash: String::new(),
            subject_key_ids: Vec::new(),
            withdraws_admission_rule: None,
            cohort_scope: "federation".into(),
            tier: "federation".into(),
            promoted_at: None,
        }
    }

    fn steward_triple() -> HashSet<String> {
        let mut s = HashSet::new();
        s.insert("us-steward".into());
        s.insert("eu-steward".into());
        s.insert("apac-steward".into());
        s
    }

    #[test]
    fn empty_attestations_yields_no_canonical() {
        let result = compose_trust_layers(
            &[],
            &steward_triple(),
            &std::collections::HashMap::new(),
        );
        assert!(result.canonical_attester.is_none());
        assert!(result.open_attesters.is_empty());
        assert!(result.vote_then_trust.is_empty());
    }

    #[test]
    fn steward_above_threshold_is_canonical() {
        let attestations = vec![make_attestation("us-steward", 0.9)];
        let result = compose_trust_layers(
            &attestations,
            &steward_triple(),
            &std::collections::HashMap::new(),
        );
        assert_eq!(result.canonical_attester.as_deref(), Some("us-steward"));
        assert_eq!(result.open_attesters.len(), 1);
    }

    #[test]
    fn steward_below_threshold_is_not_canonical() {
        let attestations = vec![make_attestation("us-steward", 0.6)];
        let result = compose_trust_layers(
            &attestations,
            &steward_triple(),
            &std::collections::HashMap::new(),
        );
        // §8.1.6 anti-tricking: no canonical if no steward at/above 0.7
        assert!(result.canonical_attester.is_none());
        // But still in Layer 2 open
        assert_eq!(result.open_attesters.len(), 1);
    }

    #[test]
    fn non_steward_never_promotes_to_canonical_via_high_score() {
        // §8.1.6 anti-tricking guarantee: Layer 2 cannot promote to Layer 1
        // by score alone — only steward-triple keys qualify.
        let attestations = vec![make_attestation("third-party-attester", 1.0)];
        let result = compose_trust_layers(
            &attestations,
            &steward_triple(),
            &std::collections::HashMap::new(),
        );
        assert!(result.canonical_attester.is_none());
        assert_eq!(result.open_attesters.len(), 1);
    }

    #[test]
    fn multiple_stewards_pick_highest_score() {
        let attestations = vec![
            make_attestation("us-steward", 0.8),
            make_attestation("eu-steward", 0.95),
            make_attestation("apac-steward", 0.9),
        ];
        let result = compose_trust_layers(
            &attestations,
            &steward_triple(),
            &std::collections::HashMap::new(),
        );
        assert_eq!(result.canonical_attester.as_deref(), Some("eu-steward"));
    }

    #[test]
    fn steward_score_ties_break_lexicographically() {
        let attestations = vec![
            make_attestation("us-steward", 0.85),
            make_attestation("eu-steward", 0.85),
        ];
        let result = compose_trust_layers(
            &attestations,
            &steward_triple(),
            &std::collections::HashMap::new(),
        );
        // Lex tie-break: "eu" < "us"
        assert_eq!(result.canonical_attester.as_deref(), Some("eu-steward"));
    }

    #[test]
    fn non_steward_with_vote_weight_goes_to_layer_3() {
        let attestations = vec![make_attestation("community-attester", 0.5)];
        let mut votes = std::collections::HashMap::new();
        votes.insert("community-attester".into(), 42.0);

        let result = compose_trust_layers(&attestations, &steward_triple(), &votes);
        assert_eq!(result.vote_then_trust.len(), 1);
        assert_eq!(result.vote_then_trust[0].key_id, "community-attester");
        assert_eq!(result.vote_then_trust[0].vote_weight, 42.0);
    }

    #[test]
    fn anti_tricking_guarantee_layer_3_never_promotes_to_canonical() {
        // Even with massive accumulated vote weight, a non-steward
        // attester does NOT become canonical. The §8.1.6 anti-tricking
        // guarantee binds CIRIS L3C: install endpoint default is ALWAYS
        // the steward-attested canonical one.
        let attestations = vec![make_attestation("community-attester", 1.0)];
        let mut votes = std::collections::HashMap::new();
        votes.insert("community-attester".into(), 999_999.0);

        let result = compose_trust_layers(&attestations, &steward_triple(), &votes);
        assert!(result.canonical_attester.is_none());
        assert_eq!(result.vote_then_trust.len(), 1);
    }
}
