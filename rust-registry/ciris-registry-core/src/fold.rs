//! The registry slice's fold surface — the routes CIRISServer mounts.
//!
//! FSD-004 §5: after the fold, registry-core hands the server routers built on
//! the server's shared persist `Engine`, the way `ciris-lens-core` hands over
//! `read_api*` (CIRISServer `src/compose.rs`). This module is that surface. It
//! compiles without the `standalone` feature: no sqlx pool, no tonic, no JWT,
//! no Edge of its own. Every route here is FSD-004 §4.1 tier P (public) and
//! serves self-authenticating objects, so it carries no caller auth.
//!
//! The standalone binary serves the same handlers through `api::http`, which
//! pulls a [`FoldState`] out of its own state, so the two deployments cannot
//! drift apart.
//!
//! Routes (FSD-004 disposition KEEP):
//! - `GET /v1/steward-key`, `GET /v1/trust-root/bundle` — the baked genesis
//!   bundle (#133).
//! - `GET /v1/agent_files/{kind}` — three-layer trust composition over the
//!   federation directory.
//! - `POST /v1/builds`, `GET /v1/builds/…` — CEG-native build manifests
//!   ([`crate::fold_builds`]); mounted by [`router`] only.
//!
//! Not here, because the server already serves them: `/v1/identity`,
//! `/v1/accord-holders`, health and metrics.

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

use crate::federation::FederationDirectory;

/// What the fold routes need, and nothing more.
#[derive(Clone)]
pub struct FoldState {
    /// The shared persist Engine. `None` only in a standalone boot that did
    /// not construct one; routes that need it degrade honestly.
    pub engine: Option<Arc<ciris_persist::engine::Engine>>,
    /// The federation directory the reads go through.
    pub federation: Arc<dyn FederationDirectory>,
    /// The key this node operates as (reported, unsigned, in `served_by`).
    pub node_key_id: String,
}

/// The registry slice's router, for CIRISServer's `compose_registry`.
///
/// `engine` is the server's shared Engine; `node_key_id` is the key the node
/// operates as (the same one `/v1/federation/conformance` reads). Reads go to
/// the federation directory over that Engine.
pub fn router(engine: Arc<ciris_persist::engine::Engine>, node_key_id: String) -> Router {
    let federation: Arc<dyn FederationDirectory> = Arc::new(
        crate::federation::persist_client::PersistFederationClient::new(Arc::clone(&engine)),
    );
    routes()
        .with_state(FoldState {
            engine: Some(Arc::clone(&engine)),
            federation,
            node_key_id: node_key_id.clone(),
        })
        // CEG-native builds. Server-only: the standalone binary still serves
        // `/v1/builds` from its Postgres build table.
        .merge(crate::fold_builds::router(engine, node_key_id))
}

/// The fold routes, unbound. `api::http` merges these into the standalone
/// router; [`router`] binds them to the server's Engine.
pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    FoldState: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/v1/steward-key", get(steward_key))
        .route("/v1/trust-root/bundle", get(steward_key))
        .route("/v1/agent_files/{kind}", get(agent_files_for_kind))
}

/// Response from /v1/agent_files/{kind} (CIRISRegistry#23 Surface 2 + #18).
/// Three-layer trust composition per FSD-002 §6.1.6.
#[derive(Serialize)]
struct AgentFilesResponse {
    kind: String,
    platform_or_target: Option<String>,
    canonical_attester: Option<AgentFileAttesterEntry>,
    open_attesters: Vec<AgentFileAttesterEntry>,
    vote_then_trust: Vec<AgentFileAttesterEntry>,
    anti_trick_guarantee: String,
    timestamp: i64,
}

#[derive(Serialize)]
struct AgentFileAttesterEntry {
    attester_key_id: String,
    file_sha256: String,
    attestation_score: f64,
    confidence: f64,
    trust_layer: String,
    note: Option<String>,
}

/// `GET /v1/steward-key` and `GET /v1/trust-root/bundle` — serve the
/// **portable trust root persist bakes**, not this node's own key.
///
/// This is the #133 resolution. The old body published registry's own steward
/// key as a trust root: unsigned on the wire while declaring
/// `signature_mode: "HYBRID_REQUIRED"`, and asserting `hardware_class: HSM_PROD`
/// under `self_attested: true`. Three mutually incompatible client schemas for
/// it exist across the fleet and none of them could parse it. There was no
/// working contract to preserve, so it is replaced rather than repaired.
///
/// What is served instead is the `GenesisBundle` — the `humanity-accord`
/// charter, its A1/B1/C1 holder roster, the `infra:*` scopes the charter
/// confers, and the serve-node grants issued under it. It is
/// **self-authenticating**: its `authorizations` are hybrid Ed25519 + ML-DSA-65
/// signatures from accord holders over the charter, and a consumer re-derives
/// authority from its OWN records, never from anything the bundle says about
/// itself.
///
/// **The authority is inside `bundle` and nowhere else.** Everything outside it
/// — `bundle_fingerprint`, `charter_root_key_id`, `served_by` — is unsigned
/// convenience metadata: this node's unverified claim about bytes it is
/// relaying. There is deliberately NO `response_signature`: signing the wrapper
/// would prove only that the relaying node said it, which is exactly what the
/// old steward-key proved and exactly what was worthless. The same schema is
/// served by CIRISServer at `/v1/trust-root/bundle`, so a consumer sees one
/// shape on both sides of the fold.
///
/// The old path `/v1/steward-key` is KEPT and serves this; any consumer still
/// pinned to it now receives the portable root instead of a self-assertion.
async fn steward_key(
    State(state): State<FoldState>,
) -> Result<Json<TrustRootBundleResponse>, crate::api::error::ApiError> {
    let bundle = ciris_persist::federation::genesis::canonical_genesis_bundle();

    let bundle_json = serde_json::to_value(bundle).map_err(|e| {
        crate::api::error::ApiError::from_status(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("the baked genesis bundle could not be serialized: {e}"),
        )
    })?;

    // Fingerprint over the JCS-canonical bytes so it is stable across
    // serializers. Best-effort: a bundle we cannot fingerprint is still worth
    // serving — the consumer verifies the artifact, not this field.
    let bundle_fingerprint = ciris_verify_core::jcs::canonicalize(&bundle_json)
        .ok()
        .map(|bytes| {
            use sha2::{Digest, Sha256};
            format!("sha256:{}", hex::encode(Sha256::digest(bytes)))
        });

    let charter_root_key_id = charter_root_of(bundle);

    // Does THIS node's own `trust:accepts` edge reach the charter root? Only
    // answerable when the persist Engine is wired (FEDERATION_DUAL_WRITE);
    // otherwise honestly `false` — a node can relay a root it has not accepted.
    let accepts_this_root = match (&state.engine, &charter_root_key_id) {
        (Some(engine), Some(root)) => {
            ciris_persist::federation::trust_root::trust_root_valid(
                engine.federation_directory().as_ref(),
                &state.node_key_id,
                root,
            )
            .await
            .ok()
            .and_then(|v| serde_json::to_value(&v).ok())
            .and_then(|j| j.get("user_accepts").and_then(serde_json::Value::as_bool))
            .unwrap_or(false)
        }
        _ => false,
    };

    Ok(Json(TrustRootBundleResponse {
        bundle: bundle_json,
        bundle_fingerprint,
        charter_root_key_id,
        served_by: TrustRootServedBy {
            node_key_id: state.node_key_id.clone(),
            accepts_this_root,
        },
    }))
}

/// The charter root a bundle declares — read off the `genesis-charter`
/// attestation's `attested_key_id` (`humanity-accord`). For discoverability
/// only; a consumer verifies it, it does not trust this field.
fn charter_root_of(bundle: &ciris_persist::federation::genesis::GenesisBundle) -> Option<String> {
    bundle
        .attestations
        .iter()
        .find(|a| a.attestation.attestation_id == "genesis-charter")
        .map(|a| a.attestation.attested_key_id.clone())
}

/// The accord holder roster the bundle carries (A1/B1/C1). Used as the
/// "who may bless a CI pipeline" set by the #138 manifest consumer.
///
/// Persist is explicit that bundle-carried holders are cross-check input and
/// never the verification authority on their own (the CIRISPersist#377
/// lesson) — but this is the BAKED bundle, compiled into the persist crate,
/// not one received over the wire, so it is the same authority persist's own
/// admission gate re-derives from.
#[cfg_attr(not(feature = "standalone"), allow(dead_code))]
pub(crate) fn accord_holder_roster() -> Vec<String> {
    ciris_persist::federation::genesis::canonical_genesis_bundle()
        .holders
        .iter()
        .map(|h| h.record.key_id.clone())
        .collect()
}

/// Response for the trust-root broadcast. Mirrors CIRISServer's
/// `BundleBroadcast` field-for-field so the fold changes nothing a consumer
/// sees. **Only `bundle` carries authority.**
#[derive(Serialize)]
struct TrustRootBundleResponse {
    /// The artifact — the only part of this response that carries authority.
    bundle: serde_json::Value,
    /// `sha256:` over the JCS-canonical bundle. Convenience; recompute it.
    bundle_fingerprint: Option<String>,
    /// The charter root the bundle declares. Read off the bundle; verify it.
    charter_root_key_id: Option<String>,
    served_by: TrustRootServedBy,
}

/// What this node says about itself while relaying. **Unsigned.**
#[derive(Serialize)]
struct TrustRootServedBy {
    node_key_id: String,
    /// `false` is a legitimate state: a node may relay a root it has not
    /// accepted. It is also the operator's un-trust lever.
    accepts_this_root: bool,
}

/// Public endpoint: GET /v1/agent_files/{kind}?platform_or_target=...
/// (CIRISRegistry#23 Surface 2 + #18)
///
/// Three-layer trust composition per CEG 0.2 §8.1.6 / FSD-002 §6.1.6:
/// - Layer 1 Canonical: registry-steward-triple attestations on `agent_files:*`
/// - Layer 2 Open: any federation-key holder may emit
/// - Layer 3 Vote-then-trust: NodeCore P4 vote accumulation
///
/// v1.3.0 (#33 Phase 3-followup) wired the federation-directory query
/// path. The endpoint now queries `state.federation` (NoOp by default;
/// PersistFederationClient when `FEDERATION_DUAL_WRITE_ENABLED=true`)
/// and composes via `edge_transport::compose_trust_layers`.
///
/// The "target" used for `list_attestations_for(...)` is the synthetic
/// key `agent_files:{kind}:{platform_or_target}` — the dimension itself,
/// treated as a denormalized attested entity. This keeps the wire shape
/// simple at the cost of requiring producers to attest against this key.
/// A richer index (target → attestations) is the open follow-up but
/// requires upstream Persist read-path work; deferred until there's
/// real data to compose over.
///
/// When the federation directory is NoOp (default) or has no
/// matching attestations, the composition returns empty layers — same
/// shape as the v1.4-interim stub the pre-1.3.0 endpoint returned.
async fn agent_files_for_kind(
    State(state): State<FoldState>,
    axum::extract::Path(kind): axum::extract::Path<String>,
    axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<AgentFilesResponse>, crate::api::error::ApiError> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let platform_or_target = q.get("platform_or_target").cloned();

    // Synthetic attested-key per the strategy above.
    let attested_key = match &platform_or_target {
        Some(p) => format!("agent_files:{}:{}", kind, p),
        None => format!("agent_files:{}", kind),
    };

    // Query the federation directory. NoOp returns empty; real client
    // returns whatever attestations the substrate has against this key.
    let attestations = state
        .federation
        .list_attestations_for(&attested_key)
        .await
        .unwrap_or_default();

    // Steward triple set per CEG §9 (placeholder — production wires
    // this from the registry-steward-triple identity rows).
    // TODO Phase 4: load from `federation_keys` rows where
    // `identity_type = 'steward_triple_member'` once that vocabulary
    // ships per CIRISPersist#102.
    let steward_triple: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Vote weights per CEG §8.1.6 Layer 3 — supplied by NodeCore's
    // read API. Empty until NodeCore ships the surface; the composition
    // function handles empty gracefully (no Layer-3 elevations).
    let vote_weights: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();

    let composition = crate::edge_transport::compose_trust_layers(
        &attestations,
        &steward_triple,
        &vote_weights,
    );

    // Lookup helper: given a key_id, find the matching OpenAttester
    // entry so we can populate score + confidence in the response.
    let lookup = |k: &str| -> Option<&crate::edge_transport::OpenAttester> {
        composition.open_attesters.iter().find(|a| a.key_id == k)
    };

    let canonical_entry =
        composition
            .canonical_attester
            .as_ref()
            .and_then(|k| lookup(k))
            .map(|a| AgentFileAttesterEntry {
                attester_key_id: a.key_id.clone(),
                file_sha256: String::new(), // attestations carry SHA in evidence_refs; index TBD
                attestation_score: a.score,
                confidence: a.confidence,
                trust_layer: "canonical".to_string(),
                note: Some(
                    "Layer 1 — registry-steward-triple (CEG §8.1.6 anti-tricking default)".to_string(),
                ),
            });

    let open_entries: Vec<AgentFileAttesterEntry> = composition
        .open_attesters
        .iter()
        .filter(|a| {
            // Exclude the canonical attester from open layer to avoid
            // double-rendering; UI distinguishes via layer.
            composition
                .canonical_attester
                .as_ref()
                .map(|c| c != &a.key_id)
                .unwrap_or(true)
        })
        .map(|a| AgentFileAttesterEntry {
            attester_key_id: a.key_id.clone(),
            file_sha256: String::new(),
            attestation_score: a.score,
            confidence: a.confidence,
            trust_layer: "open".to_string(),
            note: None,
        })
        .collect();

    let vote_entries: Vec<AgentFileAttesterEntry> = composition
        .vote_then_trust
        .iter()
        .map(|v| AgentFileAttesterEntry {
            attester_key_id: v.key_id.clone(),
            file_sha256: String::new(),
            attestation_score: 0.0,
            confidence: 0.0,
            trust_layer: "vote-then-trust".to_string(),
            note: Some(format!("Layer 3 — accumulated vote_weight={}", v.vote_weight)),
        })
        .collect();

    Ok(Json(AgentFilesResponse {
        kind,
        platform_or_target,
        canonical_attester: canonical_entry,
        open_attesters: open_entries,
        vote_then_trust: vote_entries,
        anti_trick_guarantee: "Canonical attester (registry-steward-triple, score >= 0.7) determines /install endpoint default. Third-party agent_files reachable only via explicit 'Browse alternatives' informed-consent path. Anti-tricking per CIRISRegistry#18 + CEG 0.2 §8.1.6.".to_string(),
        timestamp: now,
    }))
}


#[cfg(test)]
mod tests {
    use super::*;

    // ── trust-root broadcast ─────────────────────────────────────────────

    /// The wrapper must never grow a signature or a hardware claim. Those are
    /// precisely the fields the old /v1/steward-key carried, and they are what
    /// made it worthless: they prove only that the relay said so.
    #[test]
    fn trust_root_outer_envelope_claims_no_authority() {
        let body = TrustRootBundleResponse {
            bundle: serde_json::json!({"stub": true}),
            bundle_fingerprint: Some("sha256:f".into()),
            charter_root_key_id: Some("humanity-accord".into()),
            served_by: TrustRootServedBy { node_key_id: "n".into(), accepts_this_root: false },
        };
        let json = serde_json::to_value(&body).unwrap();
        let outer: Vec<&str> = json.as_object().unwrap().keys().map(String::as_str).collect();
        for forbidden in ["response_signature", "signature_mode", "hardware_class", "stewards"] {
            assert!(!outer.contains(&forbidden), "`{forbidden}` must not appear on the outer envelope (#133)");
        }
        assert!(outer.contains(&"bundle"), "the artifact is the only part that matters");
    }

    /// We serve the bundle persist bakes, and read the charter + roster off it.
    /// If either ever comes back empty, the bless predicate has no roster and
    /// the consumer below can never confer — pin it here, where it is served.
    #[test]
    fn baked_bundle_names_its_charter_and_holders() {
        let b = ciris_persist::federation::genesis::canonical_genesis_bundle();
        assert_eq!(charter_root_of(b).as_deref(), Some("humanity-accord"));
        let roster = accord_holder_roster();
        assert!(!roster.is_empty(), "the baked bundle must carry the holder roster");
        for h in ["A1", "B1", "C1"] {
            assert!(roster.iter().any(|k| k == h), "holder {h} missing from baked roster {roster:?}");
        }
        let json = serde_json::to_value(b).unwrap();
        assert!(json.get("authorizations").is_some(), "authorizations are what make the bundle self-authenticating");
    }

}
