//! Evidence of work on the fold router (`/v1/evidence/{suite}`), against a
//! real SQLite Engine; the `safety_battery` suite.
//!
//! What this file can and cannot show. It shows every refusal the door owns,
//! that persist admits the family's dimension as written (a row written
//! straight through the directory, as anti-entropy would), and the read side
//! over such rows. It does NOT show a blessed signer being admitted through
//! the door: blessing needs a trust root this node accepts, and minting one
//! takes the accord ceremony. That path is the same code as the build
//! manifests', exercised end to end by CIRISServer's `harness/mesh-repro`
//! `manifest` scenario on a synthetic trust root.
//!
//!   cargo test -p ciris-registry-core --test safety_evidence_fold --no-default-features

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use base64::Engine as _;
use ciris_persist::engine::Engine;
use ciris_persist::federation::envelope::EnvelopeCore;
use ciris_persist::federation::types::SignedAttestation;
use ciris_persist::federation::{attestation_emit, EmitAttestationInput, SignedKeyRecord};
use ciris_registry_core::crypto::HybridCrypto;
use ciris_registry_core::fold_builds::SignedContribution;
use ciris_registry_core::fold_evidence::{
    contribution_envelope, evidence_dimension, prefix_of, EvidenceFacts, Judge, Rubric,
};
use ciris_verify_core::federation_self_record::{produce_scrubbed_key_record, ScrubTarget};
use ciris_verify_core::self_at_login::{HybridSigningIdentity, SelfSigner};
use sha2::{Digest, Sha256};
use tower::ServiceExt;

const NODE: &str = "fold-test-node";
const SUITE: &str = "safety_battery";

async fn engine() -> Arc<Engine> {
    let crypto = HybridCrypto::generate_ephemeral().expect("ephemeral keys");
    let signer = crypto.build_persist_local_signer().expect("persist signer");
    Arc::new(
        Engine::with_signer(signer, "sqlite::memory:")
            .await
            .expect("sqlite engine"),
    )
}

fn sha_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// A capture bundle's manifest (what the agent said), and its facts.
fn capture(cell: &str, agent_version: &str, captured_at: &str) -> (Vec<u8>, EvidenceFacts, f64) {
    let manifest = serde_json::json!({
        "schema": "ciris.ai/safety_capture_manifest_signed/v1",
        "cell": cell,
        "agent_version": agent_version,
        "captured_at": captured_at,
        "results_jsonl_sha256": "11".repeat(32),
        "summary_json_sha256": "22".repeat(32),
        "rubric_md_sha256": "33".repeat(32),
        "ci_provenance": {"repo": "CIRISAI/CIRISAgent", "sha": "abc123", "ref": "refs/heads/main", "run_id": 1, "workflow": ".github/workflows/safety-battery.yml"},
    })
    .to_string()
    .into_bytes();
    let hash = sha_hex(&manifest);
    let facts = EvidenceFacts {
        suite: SUITE.into(),
        kind: "capture".into(),
        cell: cell.into(),
        prefix: prefix_of(&hash),
        agent_version: agent_version.into(),
        rubric: Some(Rubric {
            id: "v4_am_scoring_rubric".into(),
            version: "4".into(),
            sha256: "33".repeat(32),
        }),
        judge: None,
        capture_prefix: None,
        pass_ratio: None,
        ci_provenance: serde_json::json!({"repo": "CIRISAI/CIRISAgent", "sha": "abc123", "ref": "refs/heads/main", "run_id": 1, "workflow": ".github/workflows/safety-battery.yml"}),
        captured_at: captured_at.into(),
        manifest_hash: hash,
        manifest_size: manifest.len() as u64,
    };
    (manifest, facts, 1.0)
}

/// An interpret bundle's manifest (what the judge said) over `capture_hash`.
fn interpret(
    cell: &str,
    capture_hash: &str,
    passed: u32,
    total: u32,
    captured_at: &str,
) -> (Vec<u8>, EvidenceFacts, f64) {
    let manifest = serde_json::json!({
        "schema": "ciris.ai/safety_interpret_manifest_signed/v1",
        "cell": cell,
        "captured_at": captured_at,
        "capture_manifest_sha256": capture_hash,
        "verdicts_jsonl_sha256": "44".repeat(32),
        "verdicts_summary": {"passed": passed, "total": total},
        "judge_model": "claude-fable-5-1",
        "judge_prompt_sha256": "55".repeat(32),
        "rubric_md_sha256": "33".repeat(32),
    })
    .to_string()
    .into_bytes();
    let hash = sha_hex(&manifest);
    let facts = EvidenceFacts {
        suite: SUITE.into(),
        kind: "interpret".into(),
        cell: cell.into(),
        prefix: prefix_of(capture_hash),
        agent_version: "2.16.5".into(),
        rubric: Some(Rubric {
            id: "v4_am_scoring_rubric".into(),
            version: "4".into(),
            sha256: "33".repeat(32),
        }),
        judge: Some(Judge {
            model: "claude-fable-5-1".into(),
            prompt_sha256: "55".repeat(32),
        }),
        capture_prefix: Some(prefix_of(capture_hash)),
        pass_ratio: Some(f64::from(passed) / f64::from(total)),
        ci_provenance: serde_json::json!({"repo": "CIRISAI/CIRISAgent", "sha": "abc123", "ref": "refs/heads/main", "run_id": 2, "workflow": ".github/workflows/safety-battery.yml"}),
        captured_at: captured_at.into(),
        manifest_hash: hash,
        manifest_size: manifest.len() as u64,
    };
    // Signed polarity: the score is 2·r − 1 for pass ratio r.
    (
        manifest,
        facts,
        2.0 * (f64::from(passed) / f64::from(total)) - 1.0,
    )
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
            roles: Vec::new(),
        },
        &chrono::Utc::now().to_rfc3339(),
        None,
        &[],
    )
    .await
    .expect("self record");
    serde_json::from_value(serde_json::to_value(&rec).unwrap()).expect("persist wire shape")
}

/// Stamp `envelope` through persist's emit chokepoint and sign it as `signer`.
async fn sign(signer: &HybridSigningIdentity, envelope: serde_json::Value) -> SignedContribution {
    let core = EnvelopeCore::from_value(envelope).expect("envelope core");
    let mut input = EmitAttestationInput::with_envelope("scores", core, "federation");
    input.attested_key_id = Some(signer.key_id().to_string());
    let canonical =
        attestation_emit::stamp_and_canonicalize(&mut input, signer.key_id(), chrono::Utc::now())
            .expect("stamp");
    let (ed, pqc) = signer.sign_bound(&canonical).await.expect("sign");
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

fn app(engine: &Arc<Engine>) -> axum::Router {
    ciris_registry_core::fold::router(Arc::clone(engine), NODE.to_string())
}

async fn post(engine: &Arc<Engine>, suite: &str, body: String) -> (StatusCode, serde_json::Value) {
    let req = Request::post(format!("/v1/evidence/{suite}"))
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app(engine).oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 22)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}

async fn get(engine: &Arc<Engine>, uri: &str) -> (StatusCode, Vec<u8>) {
    let resp = app(engine)
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 22)
        .await
        .unwrap();
    (status, bytes.to_vec())
}

async fn get_json(engine: &Arc<Engine>, uri: &str) -> (StatusCode, serde_json::Value) {
    let (status, bytes) = get(engine, uri).await;
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}

/// Every evidence row `key` authored, through persist's bounded query.
async fn rows_by(
    engine: &Arc<Engine>,
    key: &str,
) -> Vec<ciris_persist::federation::types::Attestation> {
    use ciris_persist::federation::attestation_query::{AttestationQuery, FOLD_PAGE};
    ciris_registry_core::federation::persist_client::collect_pages(
        engine.federation_directory().as_ref(),
        AttestationQuery::by(key, FOLD_PAGE).dimension_prefix("evidence:"),
    )
    .await
    .expect("paged read")
}

/// Write a signed Contribution straight through persist, as anti-entropy
/// would, bypassing the door. Returns persist's verdict on the row.
async fn write_through_persist(
    engine: &Arc<Engine>,
    signer: &HybridSigningIdentity,
    envelope: serde_json::Value,
) -> Result<(), String> {
    let directory = engine.federation_directory();
    let _ = directory.put_public_key(self_record(signer).await).await;
    let core = EnvelopeCore::from_value(envelope).unwrap();
    let mut input = EmitAttestationInput::with_envelope("scores", core, "federation");
    input.attested_key_id = Some(signer.key_id().to_string());
    let canonical =
        attestation_emit::stamp_and_canonicalize(&mut input, signer.key_id(), chrono::Utc::now())
            .unwrap();
    let (ed, pqc) = signer.sign_bound(&canonical).await.unwrap();
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
        attestation_emit::assemble(signer.key_id().to_string(), &canonical, sig, input).unwrap();
    directory
        .put_attestation(SignedAttestation { attestation: row })
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// ───────────────────────────── the door ─────────────────────────────

/// The load-bearing refusal. A real key, a valid signature, a matching
/// manifest, every suite rule satisfied, and no blessing: the door refuses
/// and stores nothing.
#[tokio::test]
async fn a_valid_capture_from_an_unblessed_signer_is_refused_and_nothing_is_stored() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(&signer, contribution_envelope(&facts, score)).await;

    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["error"], "pipeline_not_blessed");
    assert!(rows_by(&engine, "agent-steward-test").await.is_empty());
    let (status, _) = get(
        &engine,
        &format!("/v1/evidence/{SUITE}/blob/{}", facts.manifest_hash),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "a refused bundle's bytes must not be served"
    );
}

/// An interpretation citing its capture gets as far as the blessing: every
/// suite rule (kind, cell, prefix from the capture link, score from the
/// verdicts) passed.
#[tokio::test]
async fn a_valid_interpret_citing_its_capture_reaches_the_blessing_check() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (_, cap, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let (manifest, facts, score) = interpret(
        "am_mental_health",
        &cap.manifest_hash,
        3,
        4,
        "2026-10-01T13:00:00Z",
    );
    assert_eq!(
        facts.prefix, cap.prefix,
        "the interpret rides the capture's dimension prefix"
    );
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["error"], "pipeline_not_blessed");
}

#[tokio::test]
async fn an_unregistered_suite_is_refused_before_anything_is_read() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(&engine, "coherence_bench", body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "namespace_vocab_value_unregistered");
    let (status, json) = get_json(&engine, "/v1/evidence/coherence_bench").await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "namespace_vocab_value_unregistered");
}

#[tokio::test]
async fn a_capture_whose_prefix_is_not_its_own_digest_is_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, mut facts, score) =
        capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    facts.prefix = "0000000000000000".into();
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_prefix_mismatch");
}

#[tokio::test]
async fn an_interpret_whose_prefix_is_not_its_captures_is_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (_, cap, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let (manifest, mut facts, score) = interpret(
        "am_mental_health",
        &cap.manifest_hash,
        3,
        4,
        "2026-10-01T13:00:00Z",
    );
    // Claims a different capture than the bundle names.
    facts.prefix = prefix_of(&"99".repeat(32));
    facts.capture_prefix = Some(facts.prefix.clone());
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_prefix_mismatch");
}

#[tokio::test]
async fn an_interpret_that_names_no_capture_is_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let manifest = serde_json::json!({
        "schema": "ciris.ai/safety_interpret_manifest_signed/v1",
        "verdicts_summary": {"passed": 3, "total": 4},
    })
    .to_string()
    .into_bytes();
    let hash = sha_hex(&manifest);
    let (_, mut facts, score) = interpret(
        "am_mental_health",
        &"77".repeat(32),
        3,
        4,
        "2026-10-01T13:00:00Z",
    );
    facts.manifest_hash = hash;
    facts.manifest_size = manifest.len() as u64;
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_capture_link_missing");
}

#[tokio::test]
async fn a_score_the_bundle_does_not_support_is_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (_, cap, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    // The bundle says 3/4; the Contribution claims a perfect run.
    let (manifest, facts, _) = interpret(
        "am_mental_health",
        &cap.manifest_hash,
        3,
        4,
        "2026-10-01T13:00:00Z",
    );
    let c = sign(&signer, contribution_envelope(&facts, 1.0)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_score_invalid");

    // A capture is 1 by definition.
    let (manifest, facts, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(&signer, contribution_envelope(&facts, 0.5)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_score_invalid");
}

/// The sign carries the verdict: all-fail is −1, all-pass +1, half is 0.
/// Each passes every suite rule and reaches the blessing check.
#[tokio::test]
async fn an_interpretations_score_carries_the_verdict_in_its_sign() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (_, cap, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    for (passed, total, expected) in [(0, 4, -1.0), (4, 4, 1.0), (2, 4, 0.0)] {
        let (manifest, facts, score) = interpret(
            "am_mental_health",
            &cap.manifest_hash,
            passed,
            total,
            "2026-10-01T13:00:00Z",
        );
        assert_eq!(score, expected);
        let c = sign(&signer, contribution_envelope(&facts, score)).await;
        let (status, json) = post(
            &engine,
            SUITE,
            body(&c, &manifest, Some(&self_record(&signer).await)),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{passed}/{total}: {json}");
        assert_eq!(
            json["error"], "pipeline_not_blessed",
            "{passed}/{total}: every suite rule passed"
        );
    }
}

/// An interpretation must state its pass ratio in the envelope.
#[tokio::test]
async fn an_interpretation_without_a_pass_ratio_is_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (_, cap, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let (manifest, mut facts, score) = interpret(
        "am_mental_health",
        &cap.manifest_hash,
        3,
        4,
        "2026-10-01T13:00:00Z",
    );
    facts.pass_ratio = None;
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_score_invalid");
}

/// A positive-only reading (the pass ratio itself) is not the score.
#[tokio::test]
async fn a_pass_ratio_posted_as_the_score_is_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (_, cap, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let (manifest, facts, _) = interpret(
        "am_mental_health",
        &cap.manifest_hash,
        3,
        4,
        "2026-10-01T13:00:00Z",
    );
    let c = sign(&signer, contribution_envelope(&facts, 0.75)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_score_invalid");
}

#[tokio::test]
async fn kind_and_cell_outside_the_suite_are_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, mut facts, score) =
        capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    facts.kind = "judge".into();
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_kind_invalid");

    let (manifest, mut facts, score) =
        capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    facts.cell = "Amharic mental health".into();
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_cell_invalid");
}

#[tokio::test]
async fn the_suite_in_the_path_must_be_the_dimensions() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let mut env = contribution_envelope(&facts, score);
    env["dimension"] = serde_json::json!(evidence_dimension(
        SUITE,
        "capture",
        "hi_mental_health",
        &facts.prefix
    ));
    let c = sign(&signer, env).await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "dimension_mismatch");
}

#[tokio::test]
async fn a_manifest_that_does_not_hash_to_the_attested_value_is_refused() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest.to_ascii_uppercase(), None),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "manifest_hash_mismatch");
}

#[tokio::test]
async fn a_bundle_over_one_mebibyte_is_refused_before_it_is_read() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (_, mut facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let big = vec![b'x'; 1024 * 1024 + 1];
    facts.manifest_hash = sha_hex(&big);
    facts.prefix = prefix_of(&facts.manifest_hash);
    facts.manifest_size = big.len() as u64;
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(&engine, SUITE, body(&c, &big, None)).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE, "{json}");
    assert_eq!(json["error"], "manifest_too_large");
}

#[tokio::test]
async fn back_signing_is_admitted_as_far_as_the_blessing() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    // Captured long before it is signed, with the original run's provenance.
    let (manifest, facts, score) = capture("hi_mental_health", "2.8.10", "2026-04-02T09:15:00Z");
    let c = sign(&signer, contribution_envelope(&facts, score)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(
        json["error"], "pipeline_not_blessed",
        "no refusal on captured_at < asserted_at"
    );
}

// ───────────────────────────── persist and the read ─────────────────────────────

/// persist's namespace gate (CC 3.1.7 R2) admits the family as written: an
/// open-vocabulary stem under its vendored registry, a registered row once
/// the registry carries CC 3.1.5.5.
#[tokio::test]
async fn persist_admits_the_family_dimension() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-replicated").unwrap();
    let (_, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    write_through_persist(&engine, &signer, contribution_envelope(&facts, score))
        .await
        .expect("persist admits it");
    assert_eq!(rows_by(&engine, "agent-steward-replicated").await.len(), 1);
}

/// The read re-checks. Rows from an unblessed signer that reached the
/// directory some other way are not evidence.
#[tokio::test]
async fn an_unblessed_row_already_in_the_directory_is_not_served() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-replicated").unwrap();
    let (_, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    write_through_persist(&engine, &signer, contribution_envelope(&facts, score))
        .await
        .unwrap();

    let (status, json) = get_json(&engine, &format!("/v1/evidence/{SUITE}")).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["evidence"].as_array().map(Vec::len), Some(0));
    let (status, _) = get(
        &engine,
        &format!("/v1/evidence/{SUITE}/{}", facts.manifest_hash),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

/// A re-post of the same capture's interpretation lands on the SAME
/// dimension (the prefix is the capture's), as a second row. persist keeps
/// both; this door emits no `supersedes` row (CIRISRegistry#144). The read
/// orders by assertion time, newest first, so the latest interpretation
/// leads and the earlier one is still listed.
#[tokio::test]
async fn a_re_posted_interpretation_rides_the_captures_dimension_as_a_later_row() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-replicated").unwrap();
    let (_, cap, _) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let (_, first, s1) = interpret(
        "am_mental_health",
        &cap.manifest_hash,
        2,
        4,
        "2026-10-01T13:00:00Z",
    );
    let (_, second, s2) = interpret(
        "am_mental_health",
        &cap.manifest_hash,
        3,
        4,
        "2026-10-02T13:00:00Z",
    );
    assert_eq!(first.prefix, second.prefix);
    assert_ne!(first.manifest_hash, second.manifest_hash);
    write_through_persist(&engine, &signer, contribution_envelope(&first, s1))
        .await
        .unwrap();
    write_through_persist(&engine, &signer, contribution_envelope(&second, s2))
        .await
        .unwrap();
    let rows = rows_by(&engine, "agent-steward-replicated").await;
    assert_eq!(rows.len(), 2, "persist holds both rows on one dimension");
    let dims: std::collections::BTreeSet<_> = rows
        .iter()
        .filter_map(|r| r.attestation_envelope["dimension"].as_str())
        .collect();
    assert_eq!(dims.len(), 1, "one dimension, two assertions");
}

/// The read filters on the facts, refuses a malformed filter, and serves no
/// bytes for a row it does not verify. The bytes of an admitted bundle are
/// written by the door's shared admission (the build-manifest path, proven on
/// the harness); writing a blob here directly needs the node's own key record,
/// which this ephemeral engine does not hold.
#[tokio::test]
async fn the_listing_filters_and_unverified_bytes_are_not_served() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-replicated").unwrap();
    let (_, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    write_through_persist(&engine, &signer, contribution_envelope(&facts, score))
        .await
        .unwrap();
    let (status, _) = get(
        &engine,
        &format!("/v1/evidence/{SUITE}/blob/{}", facts.manifest_hash),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "an unblessed row does not unlock its bytes"
    );

    // The listing's filters are plain and total: a bad `since` is refused.
    let (status, json) = get_json(&engine, &format!("/v1/evidence/{SUITE}?since=yesterday")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "contribution_malformed");
    let (status, json) =
        get_json(&engine, &format!("/v1/evidence/{SUITE}?cell=am_mental_health&kind=capture&agent_version=2.16.5&rubric=v4_am_scoring_rubric&since=2026-01-01T00:00:00Z")).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["suite"], SUITE);
    assert_eq!(
        json["evidence"].as_array().map(Vec::len),
        Some(0),
        "unblessed rows are not evidence"
    );
}

/// The build-manifest door is untouched by the generalization.
#[tokio::test]
async fn the_builds_door_is_still_mounted_beside_the_evidence_door() {
    let engine = engine().await;
    let (status, _) = get(&engine, "/v1/builds/9.9.9").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, json) = get_json(&engine, &format!("/v1/evidence/{SUITE}")).await;
    assert_eq!(status, StatusCode::OK, "{json}");
}
