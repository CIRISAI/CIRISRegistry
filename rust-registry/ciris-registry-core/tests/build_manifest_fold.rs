//! CEG-native build manifests on the fold router, against a real SQLite Engine.
//!
//! What this file can and cannot show. It shows every refusal, and it shows the
//! two reasons verify v18's own producer cannot be used as the wire shape. It
//! does NOT show a blessed pipeline being admitted: blessing needs a trust root
//! this node accepts, and minting one takes the accord ceremony. That path is
//! exercised end to end by CIRISServer's `harness/mesh-repro` `manifest`
//! scenario, on a synthetic trust root.
//!
//!   cargo test -p ciris-registry-core --test build_manifest_fold --no-default-features

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use base64::Engine as _;
use ciris_persist::engine::Engine;
use ciris_persist::federation::envelope::EnvelopeCore;
use ciris_persist::federation::types::SignedAttestation;
use ciris_persist::federation::{attestation_emit, EmitAttestationInput, SignedKeyRecord};
use ciris_registry_core::crypto::HybridCrypto;
use ciris_registry_core::fold_builds::{
    build_manifest_dimension, contribution_envelope, BuildFacts, SignedContribution,
};
use ciris_verify_core::federation_self_record::{produce_scrubbed_key_record, ScrubTarget};
use ciris_verify_core::self_at_login::{HybridSigningIdentity, SelfSigner};
use sha2::{Digest, Sha256};
use tower::ServiceExt;

const NODE: &str = "fold-test-node";
const MANIFEST: &[u8] = br#"{"version":"9.9.9","files":{"ciris_engine/__init__.py":"aa"}}"#;

async fn engine() -> Arc<Engine> {
    let crypto = HybridCrypto::generate_ephemeral().expect("ephemeral keys");
    let signer = crypto.build_persist_local_signer().expect("persist signer");
    Arc::new(Engine::with_signer(signer, "sqlite::memory:").await.expect("sqlite engine"))
}

fn facts(manifest: &[u8]) -> BuildFacts {
    BuildFacts {
        target: "python-source-tree".to_string(),
        build_id: "build-1".to_string(),
        binary_hash: "aa".repeat(32),
        binary_version: "9.9.9".to_string(),
        manifest_hash: hex::encode(Sha256::digest(manifest)),
    }
}

/// A self-signed `node` record for `id`: a real key, blessed by nobody.
async fn self_record(id: &HybridSigningIdentity) -> SignedKeyRecord {
    let m = id.directory_member().expect("member");
    let rec = produce_scrubbed_key_record(
        id,
        ScrubTarget {
            key_id: id.key_id().to_string(),
            pubkey_ed25519_base64: m.ed25519_public_key_base64.clone(),
            pubkey_ml_dsa_65_base64: m.mldsa65_public_key_base64.clone().expect("hybrid"),
            identity_type: "node".to_string(),
            roles: vec![],
        },
        &chrono::Utc::now().to_rfc3339(),
        None,
        &[],
    )
    .await
    .expect("self record");
    serde_json::from_value(serde_json::to_value(&rec).unwrap()).expect("persist wire shape")
}

/// Stamp `envelope` through persist's emit chokepoint and sign it as `pipeline`.
async fn sign(pipeline: &HybridSigningIdentity, envelope: serde_json::Value) -> SignedContribution {
    let core = EnvelopeCore::from_value(envelope).expect("envelope core");
    let mut input = EmitAttestationInput::with_envelope("scores", core, "federation");
    input.attested_key_id = Some(pipeline.key_id().to_string());
    let canonical =
        attestation_emit::stamp_and_canonicalize(&mut input, pipeline.key_id(), chrono::Utc::now())
            .expect("stamp");
    let (ed, pqc) = pipeline.sign_bound(&canonical).await.expect("sign");
    SignedContribution {
        signed_envelope: input.attestation_envelope.to_value(),
        ed25519_signature_base64: ed,
        mldsa65_signature_base64: pqc,
    }
}

fn body(c: &SignedContribution, manifest: &[u8], record: Option<&SignedKeyRecord>) -> String {
    serde_json::json!({
        "contribution": c,
        "manifest_base64": base64::engine::general_purpose::STANDARD.encode(manifest),
        "pipeline_record": record,
    })
    .to_string()
}

async fn post(engine: &Arc<Engine>, body: String) -> (StatusCode, serde_json::Value) {
    let app = ciris_registry_core::fold::router(Arc::clone(engine), NODE.to_string());
    let req = Request::post("/v1/builds")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 22).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null))
}

async fn get(engine: &Arc<Engine>, uri: &str) -> StatusCode {
    let app = ciris_registry_core::fold::router(Arc::clone(engine), NODE.to_string());
    app.oneshot(Request::get(uri).body(Body::empty()).unwrap()).await.unwrap().status()
}

/// The load-bearing refusal. A real key, a valid signature, a matching
/// manifest, and no blessing: the door refuses and stores nothing.
#[tokio::test]
async fn a_valid_signature_from_an_unblessed_pipeline_is_refused_and_nothing_is_stored() {
    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-unblessed").unwrap();
    let record = self_record(&pipeline).await;
    let c = sign(&pipeline, contribution_envelope(&facts(MANIFEST))).await;

    let (status, json) = post(&engine, body(&c, MANIFEST, Some(&record))).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["error"], "pipeline_not_blessed");

    let rows = engine.federation_directory().list_attestations_by("ci-unblessed").await.unwrap();
    assert!(rows.is_empty(), "a refused Contribution must not be stored");
    assert_eq!(get(&engine, "/v1/builds/9.9.9").await, StatusCode::NOT_FOUND);
    let sha = facts(MANIFEST).manifest_hash;
    assert_eq!(
        get(&engine, &format!("/v1/builds/manifest/{sha}")).await,
        StatusCode::NOT_FOUND,
        "the manifest bytes of a refused Contribution must not be served"
    );
}

/// The read re-checks. A row that reached the directory some other way (here:
/// written straight through persist, as anti-entropy would) from an unblessed
/// pipeline is not a build.
#[tokio::test]
async fn an_unblessed_row_already_in_the_directory_is_not_served() {
    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-replicated").unwrap();
    let directory = engine.federation_directory();
    directory.put_public_key(self_record(&pipeline).await).await.expect("key");

    let core = EnvelopeCore::from_value(contribution_envelope(&facts(MANIFEST))).unwrap();
    let mut input = EmitAttestationInput::with_envelope("scores", core, "federation");
    input.attested_key_id = Some(pipeline.key_id().to_string());
    let canonical =
        attestation_emit::stamp_and_canonicalize(&mut input, pipeline.key_id(), chrono::Utc::now())
            .unwrap();
    let (ed, pqc) = pipeline.sign_bound(&canonical).await.unwrap();
    let b64 = base64::engine::general_purpose::STANDARD;
    let sig = ciris_crypto::HybridSignature {
        crypto_kind: ciris_crypto::CRYPTO_KIND_CIRIS_V1,
        classical: ciris_crypto::TaggedClassicalSignature {
            algorithm: ciris_crypto::ClassicalAlgorithm::Ed25519,
            signature: b64.decode(ed).unwrap(),
            public_key: Vec::new(),
        },
        pqc: ciris_crypto::TaggedPqcSignature {
            algorithm: ciris_crypto::PqcAlgorithm::MlDsa65,
            signature: b64.decode(pqc).unwrap(),
            public_key: Vec::new(),
        },
        mode: ciris_crypto::SignatureMode::HybridRequired,
    };
    let (row, _) =
        attestation_emit::assemble(pipeline.key_id().to_string(), &canonical, sig, input).unwrap();
    directory.put_attestation(SignedAttestation { attestation: row }).await.expect("persist admits it");
    assert_eq!(directory.list_attestations_by("ci-replicated").await.unwrap().len(), 1);

    assert_eq!(get(&engine, "/v1/builds/9.9.9").await, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_manifest_that_does_not_hash_to_the_attested_value_is_refused() {
    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-swap").unwrap();
    let c = sign(&pipeline, contribution_envelope(&facts(MANIFEST))).await;
    let (status, json) =
        post(&engine, body(&c, b"{\"files\":{}}", Some(&self_record(&pipeline).await))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"], "manifest_hash_mismatch");
}

#[tokio::test]
async fn an_envelope_altered_after_signing_is_refused() {
    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-tamper").unwrap();
    let mut c = sign(&pipeline, contribution_envelope(&facts(MANIFEST))).await;
    c.signed_envelope["build"]["binary_hash"] = serde_json::json!("00".repeat(32));
    let (status, json) =
        post(&engine, body(&c, MANIFEST, Some(&self_record(&pipeline).await))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "{json}");
    assert_eq!(json["error"], "signature_invalid");
}

#[tokio::test]
async fn a_pipeline_this_node_has_never_seen_is_refused_by_name() {
    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-stranger").unwrap();
    let c = sign(&pipeline, contribution_envelope(&facts(MANIFEST))).await;
    let (status, json) = post(&engine, body(&c, MANIFEST, None)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(json["error"], "unknown_pipeline");
}

/// A key record for someone else cannot be used to introduce a signer.
#[tokio::test]
async fn a_pipeline_record_for_a_different_key_is_refused() {
    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-a").unwrap();
    let other = HybridSigningIdentity::generate("ci-b").unwrap();
    let c = sign(&pipeline, contribution_envelope(&facts(MANIFEST))).await;
    let (status, json) = post(&engine, body(&c, MANIFEST, Some(&self_record(&other).await))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"], "contribution_malformed");
}

/// The dimension must name the build's own target, with the version segment.
#[tokio::test]
async fn a_dimension_for_another_target_is_refused() {
    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-dim").unwrap();
    let mut env = contribution_envelope(&facts(MANIFEST));
    env["dimension"] = serde_json::json!(build_manifest_dimension("ios-mobile-bundle"));
    let c = sign(&pipeline, env).await;
    let (status, json) =
        post(&engine, body(&c, MANIFEST, Some(&self_record(&pipeline).await))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"], "dimension_mismatch");
}

/// Why this module does not take verify v18's Contribution as-is. Both halves
/// are measured here so the day verify changes, this test says so.
#[tokio::test]
async fn verify_v18s_own_contribution_is_not_storable_by_persist() {
    use ciris_verify_core::manifest_contribution::{sign_build_manifest_contribution, BuildAttestation};

    let engine = engine().await;
    let pipeline = HybridSigningIdentity::generate("ci-verify-shape").unwrap();
    engine.federation_directory().put_public_key(self_record(&pipeline).await).await.unwrap();
    let f = facts(MANIFEST);
    let obj = sign_build_manifest_contribution(
        &pipeline,
        &BuildAttestation {
            target: &f.target,
            binary_hash: &f.binary_hash,
            build_id: &f.build_id,
            binary_version: &f.binary_version,
            manifest_hash: &f.manifest_hash,
        },
        "human-1",
        "grant-1",
        "2026-10-01T00:00:00Z",
    )
    .await
    .unwrap();
    let env = obj.body["signed_envelope"].clone();

    // (1) No signed `row` mirror, so the columns are not the signer's.
    assert!(
        EnvelopeCore::from_value(env.clone()).unwrap().row.is_none(),
        "verify now emits a row mirror: fold_builds can take its envelope directly"
    );
    // (2) No version segment (CC 3.1.7 R3).
    assert_eq!(env["dimension"], "provenance:build_manifest:python-source-tree");
    let mut facts_only = env;
    for member in ["attestation_type", "attesting_key_id", "subject_key_ids", "signed_at"] {
        facts_only.as_object_mut().unwrap().remove(member);
    }
    let core = EnvelopeCore::from_value(facts_only).unwrap();
    let mut input = EmitAttestationInput::with_envelope("scores", core, "federation");
    input.attested_key_id = Some(pipeline.key_id().to_string());
    let canonical =
        attestation_emit::stamp_and_canonicalize(&mut input, pipeline.key_id(), chrono::Utc::now())
            .unwrap();
    let (ed, pqc) = pipeline.sign_bound(&canonical).await.unwrap();
    let b64 = base64::engine::general_purpose::STANDARD;
    let sig = ciris_crypto::HybridSignature {
        crypto_kind: ciris_crypto::CRYPTO_KIND_CIRIS_V1,
        classical: ciris_crypto::TaggedClassicalSignature {
            algorithm: ciris_crypto::ClassicalAlgorithm::Ed25519,
            signature: b64.decode(ed).unwrap(),
            public_key: Vec::new(),
        },
        pqc: ciris_crypto::TaggedPqcSignature {
            algorithm: ciris_crypto::PqcAlgorithm::MlDsa65,
            signature: b64.decode(pqc).unwrap(),
            public_key: Vec::new(),
        },
        mode: ciris_crypto::SignatureMode::HybridRequired,
    };
    let (row, _) =
        attestation_emit::assemble(pipeline.key_id().to_string(), &canonical, sig, input).unwrap();
    let refused = engine
        .federation_directory()
        .put_attestation(SignedAttestation { attestation: row })
        .await
        .expect_err("persist refuses the unversioned dimension");
    assert!(
        format!("{refused:?}").contains("missing_version_segment"),
        "expected missing_version_segment, got {refused:?}"
    );
}

/// persist's canonical bytes are JCS, which is what lets a verify `SelfSigner`
/// sign a persist-stamped envelope without a second canonicalizer.
#[tokio::test]
async fn persists_canonical_bytes_are_jcs() {
    let pipeline = HybridSigningIdentity::generate("ci-jcs").unwrap();
    let c = sign(&pipeline, contribution_envelope(&facts(MANIFEST))).await;
    assert_eq!(
        attestation_emit::canonicalize(&c.signed_envelope).unwrap(),
        ciris_verify_core::jcs::canonicalize(&c.signed_envelope).unwrap()
    );
}
