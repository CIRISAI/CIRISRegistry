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
    cited_under, contribution_envelope, contribution_envelope_against, counted_by,
    evidence_dimension, prefix_of, resolve_goal_reference, EvidenceFacts, Judge, Rubric,
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
    write_row(engine, signer, "scores", envelope)
        .await
        .map(|_| ())
}

/// Write a signed row of `attestation_type` straight through persist by
/// `signer`, on its own key. Returns the row's id.
async fn write_row(
    engine: &Arc<Engine>,
    signer: &HybridSigningIdentity,
    attestation_type: &str,
    envelope: serde_json::Value,
) -> Result<String, String> {
    let directory = engine.federation_directory();
    let _ = directory.put_public_key(self_record(signer).await).await;
    let core = EnvelopeCore::from_value(envelope).unwrap();
    let mut input = EmitAttestationInput::with_envelope(attestation_type, core, "federation");
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
    let id = row.attestation_id.clone();
    directory
        .put_attestation(SignedAttestation { attestation: row })
        .await
        .map(|_| id)
        .map_err(|e| e.to_string())
}

/// A `goal:species:v1` row (CC 3.1.9.7): a signed `scores` row on the family.
async fn goal_row(engine: &Arc<Engine>, signer: &HybridSigningIdentity) -> String {
    write_row(
        engine,
        signer,
        "scores",
        serde_json::json!({
            "dimension": "goal:species:v1",
            "score": 0.5,
            "goal": "every cell of the safety battery reaches a passing interpretation",
        }),
    )
    .await
    .expect("persist admits a goal row")
}

/// A `withdraws` of `target` by its own author (CEG §6.1 rule 1).
async fn withdraw(engine: &Arc<Engine>, signer: &HybridSigningIdentity, target: &str) -> String {
    write_row(
        engine,
        signer,
        "withdraws",
        serde_json::json!({
            "references_attestation_id": target,
            "withdrawal_reason": "retired by its author",
        }),
    )
    .await
    .expect("persist admits a self-withdraws")
}

/// A `supersedes` of `target` by its own author, carrying the successor body.
async fn supersede(engine: &Arc<Engine>, signer: &HybridSigningIdentity, target: &str) -> String {
    write_row(
        engine,
        signer,
        "supersedes",
        serde_json::json!({
            "dimension": "goal:species:v1",
            "score": 0.6,
            "references_attestation_id": target,
            "goal": "every cell of the safety battery reaches a passing interpretation, in 29 languages",
        }),
    )
    .await
    .expect("persist admits a self-supersedes")
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

// ───────────────────────────── citing a Goal ─────────────────────────────

/// Offered against a held goal row: every suite rule and the reference pass;
/// the reference resolves to the goal itself; the stored row carries it.
#[tokio::test]
async fn evidence_offered_against_a_goal_row_is_admitted_up_to_the_blessing_and_resolves() {
    let engine = engine().await;
    let author = HybridSigningIdentity::generate("goal-author").unwrap();
    let goal = goal_row(&engine, &author).await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(&signer, contribution_envelope_against(&facts, score, &goal)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(
        json["error"], "pipeline_not_blessed",
        "the reference passed"
    );

    let resolved = resolve_goal_reference(engine.federation_directory().as_ref(), &goal)
        .await
        .unwrap();
    assert_eq!(resolved.attestation_id, goal);
    assert_eq!(resolved.family, "goal");
    assert_eq!(resolved.dimension, "goal:species:v1");

    write_through_persist(
        &engine,
        &signer,
        contribution_envelope_against(&facts, score, &goal),
    )
    .await
    .unwrap();
    let rows = rows_by(&engine, "agent-steward-test").await;
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].attestation_envelope["references_attestation_id"],
        goal
    );
}

#[tokio::test]
async fn evidence_offered_against_a_withdrawn_goal_is_refused() {
    let engine = engine().await;
    let author = HybridSigningIdentity::generate("goal-author").unwrap();
    let goal = goal_row(&engine, &author).await;
    withdraw(&engine, &author, &goal).await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(&signer, contribution_envelope_against(&facts, score, &goal)).await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["error"], "evidence_reference_withdrawn");
}

#[tokio::test]
async fn evidence_offered_against_a_row_outside_the_goal_families_is_refused() {
    let engine = engine().await;
    let author = HybridSigningIdentity::generate("goal-author").unwrap();
    let (_, other, s) = capture("hi_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let not_a_goal = write_row(&engine, &author, "scores", contribution_envelope(&other, s))
        .await
        .unwrap();
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(
        &signer,
        contribution_envelope_against(&facts, score, &not_a_goal),
    )
    .await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "evidence_reference_not_a_goal");

    let unknown = uuid::Uuid::new_v4().to_string();
    let c = sign(
        &signer,
        contribution_envelope_against(&facts, score, &unknown),
    )
    .await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["error"], "evidence_reference_not_found");

    let c = sign(
        &signer,
        contribution_envelope_against(&facts, score, "not-an-id"),
    )
    .await;
    let (status, json) = post(&engine, SUITE, body(&c, &manifest, None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{json}");
    assert_eq!(json["error"], "contribution_malformed");
}

#[tokio::test]
async fn evidence_offered_against_a_superseded_goal_resolves_to_the_successor() {
    let engine = engine().await;
    let author = HybridSigningIdentity::generate("goal-author").unwrap();
    let goal = goal_row(&engine, &author).await;
    let successor = supersede(&engine, &author, &goal).await;
    let resolved = resolve_goal_reference(engine.federation_directory().as_ref(), &goal)
        .await
        .unwrap();
    assert_eq!(
        resolved.attestation_id, successor,
        "the stated goal resolves to its successor"
    );
    assert_eq!(resolved.as_stated, goal);
    assert_eq!(resolved.family, "goal");

    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let c = sign(&signer, contribution_envelope_against(&facts, score, &goal)).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(
        json["error"], "pipeline_not_blessed",
        "a superseded reference is admissible"
    );
}

/// No reference is admissible: the first test in this file is that case.
/// Here the same, with the member present but null.
#[tokio::test]
async fn evidence_with_no_reference_is_admissible() {
    let engine = engine().await;
    let signer = HybridSigningIdentity::generate("agent-steward-test").unwrap();
    let (manifest, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let mut env = contribution_envelope(&facts, score);
    env["references_attestation_id"] = serde_json::Value::Null;
    let c = sign(&signer, env).await;
    let (status, json) = post(
        &engine,
        SUITE,
        body(&c, &manifest, Some(&self_record(&signer).await)),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{json}");
    assert_eq!(json["error"], "pipeline_not_blessed");
}

/// The Goal side's count: a `progress_measure:{method_id}` row citing the
/// evidence (by manifest hash) is reported as `counted_by`, and the walk
/// goal → approach → method → progress_measure finds the evidence under the
/// goal.
#[tokio::test]
async fn a_progress_measure_citing_the_row_counts_it_and_the_goal_read_finds_it() {
    let engine = engine().await;
    let author = HybridSigningIdentity::generate("goal-author").unwrap();
    let goal = goal_row(&engine, &author).await;
    let approach = write_row(
        &engine,
        &author,
        "scores",
        serde_json::json!({
            "dimension": format!("approach:{goal}:v1"), "score": 0.5,
            "approach": "run the battery on every release candidate",
        }),
    )
    .await
    .unwrap();
    let method = write_row(
        &engine,
        &author,
        "scores",
        serde_json::json!({
            "dimension": format!("method:{approach}:a1:v1"), "score": 0.5,
            "method": "safety-battery.yml on main",
        }),
    )
    .await
    .unwrap();
    let signer = HybridSigningIdentity::generate("agent-steward-replicated").unwrap();
    let (_, facts, score) = capture("am_mental_health", "2.16.5", "2026-10-01T12:00:00Z");
    let evidence_id = write_row(
        &engine,
        &signer,
        "scores",
        contribution_envelope_against(&facts, score, &goal),
    )
    .await
    .unwrap();
    let measure = write_row(&engine, &author, "scores", serde_json::json!({
        "dimension": format!("progress_measure:{method}:v1"), "score": 0.5,
        "tracks": ["pass_ratio"], "computation": "mean", "validity_window": "P30D", "goodhart_resistance": "two judges",
        "evidence_refs": [facts.manifest_hash],
    })).await.unwrap();

    let directory = engine.federation_directory();
    let counted = counted_by(directory.as_ref(), &facts.manifest_hash, &evidence_id)
        .await
        .unwrap();
    assert_eq!(counted, vec![measure.clone()]);
    let unrelated = counted_by(directory.as_ref(), &"00".repeat(32), "nobody")
        .await
        .unwrap();
    assert!(unrelated.is_empty());

    let target = resolve_goal_reference(directory.as_ref(), &goal)
        .await
        .unwrap();
    let cited = cited_under(directory.as_ref(), &target).await.unwrap();
    assert!(
        cited.contains(&facts.manifest_hash),
        "the walk goal → approach → method → measure cites the row"
    );
    let under_method = cited_under(
        directory.as_ref(),
        &resolve_goal_reference(directory.as_ref(), &method)
            .await
            .unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(under_method, cited);

    // The route answers for the goal; the rows it would list are unblessed
    // here, so the list is empty while the citation set is not.
    let (status, json) =
        get_json(&engine, &format!("/v1/goals/{goal}/evidence?suite={SUITE}")).await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["goal"]["attestation_id"], goal);
    assert_eq!(json["cited"].as_array().map(Vec::len), Some(1));
    assert_eq!(json["evidence"].as_array().map(Vec::len), Some(0));
    let (status, json) = get_json(
        &engine,
        &format!("/v1/goals/{}/evidence", uuid::Uuid::new_v4()),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{json}");
    assert_eq!(json["error"], "evidence_reference_not_found");
    let (status, json) = get_json(
        &engine,
        &format!("/v1/evidence/{SUITE}?offered_against={goal}"),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{json}");
    assert_eq!(json["evidence"].as_array().map(Vec::len), Some(0));
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
