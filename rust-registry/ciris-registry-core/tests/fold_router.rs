//! The fold surface mounted the way CIRISServer will mount it: on a SQLite
//! persist Engine, with no registry Postgres and no gRPC (FSD-004 §5).
//!
//! Runs in both builds:
//!   cargo test -p ciris-registry-core --test fold_router
//!   cargo test -p ciris-registry-core --test fold_router --no-default-features

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ciris_registry_core::crypto::HybridCrypto;
use tower::ServiceExt;

async fn fold_app() -> (axum::Router, String) {
    let crypto = HybridCrypto::generate_ephemeral().expect("ephemeral keys");
    let signer = crypto.build_persist_local_signer().expect("persist signer");
    let engine = ciris_persist::engine::Engine::with_signer(signer, "sqlite::memory:")
        .await
        .expect("sqlite engine");
    let key_id = crypto.key_id().to_string();
    (
        ciris_registry_core::fold::router(Arc::new(engine), key_id.clone()),
        key_id,
    )
}

async fn get_json(app: axum::Router, uri: &str) -> (StatusCode, serde_json::Value) {
    let resp = app
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 22).await.unwrap();
    let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
    (status, json)
}

#[tokio::test]
async fn trust_root_serves_the_baked_bundle_on_both_paths() {
    let (app, key_id) = fold_app().await;
    let (s1, steward) = get_json(app.clone(), "/v1/steward-key").await;
    let (s2, bundle) = get_json(app, "/v1/trust-root/bundle").await;

    assert_eq!(s1, StatusCode::OK);
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(steward["bundle"], bundle["bundle"], "one artifact on both paths");
    assert_eq!(steward["charter_root_key_id"], "humanity-accord");
    assert!(steward["bundle"].get("authorizations").is_some());
    assert_eq!(steward["served_by"]["node_key_id"], key_id.as_str());
    // A fresh node has not accepted the root; saying so is the honest answer.
    assert_eq!(steward["served_by"]["accepts_this_root"], false);
    assert!(steward.get("response_signature").is_none(), "#133: no wrapper signature");
}

#[tokio::test]
async fn agent_files_reads_the_shared_directory() {
    let (app, _) = fold_app().await;
    let (status, body) =
        get_json(app, "/v1/agent_files/install?platform_or_target=linux-x86_64").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "install");
    assert_eq!(body["platform_or_target"], "linux-x86_64");
    // An empty directory composes to empty layers, not an error.
    assert_eq!(body["open_attesters"], serde_json::json!([]));
    assert!(body["canonical_attester"].is_null());
}

#[tokio::test]
async fn routes_the_server_already_owns_are_not_mounted() {
    // /v1/identity and /v1/accord-holders are CIRISServer's (FSD-004 §4.1);
    // mounting them here would collide with the server's router on merge.
    let (app, _) = fold_app().await;
    for uri in ["/v1/identity", "/v1/accord-holders", "/health"] {
        let (status, _) = get_json(app.clone(), uri).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri} must not be in the fold router");
    }
}
