//! Evidence of work on the fold router — `/v1/evidence/{suite}`.
//!
//! CC 3.1.5.5. A piece of evidence is work done under a named rule, cited by
//! hash: a `scores` Contribution on
//! `evidence:{suite}:{kind}:{cell}:{prefix}:v1`, hybrid-signed by the
//! submitting key, naming the bundle's manifest in `evidence_refs`, with the
//! manifest bytes held as a commons blob at that hash. The same door and the
//! same authority as a build manifest ([`crate::fold_builds`]): the signer
//! must hold `infra:attest`, by a `delegates_to(root → signer, infra:attest)`
//! grant from a root this node accepts or by the accord role on its record.
//!
//! `{suite}` is a closed vocabulary, admitted by registration (CC 3.1.7 R2:
//! an unregistered family never admits). Each registered suite this node
//! serves has a [`SuiteRules`] validator: what its `{kind}`s are, what a
//! `{cell}` looks like, how `{prefix}` is derived from the bundle, and how the
//! row's score is read off the bundle. A suite outside that vocabulary is
//! refused with CC's own token, `namespace_vocab_value_unregistered`.
//! `safety_battery` is the registered suite today, and the dimension strings
//! are byte-identical to its registration.
//!
//! A submitted row is a **self-report** (CC 3.4.5.1): it confers the right
//! to emit and no right to be believed. Standing comes only from others'
//! attestations. This door therefore verifies and stores the row and its
//! bytes, and computes no credit, no recognition and no verdict about the
//! work; the read reports the signer's blessing for `infra:attest`, nothing
//! more.
//!
//! # `safety_battery`
//!
//! Two kinds. A **capture** is one battery run's question-and-answer pairs
//! for one cell (`{lang}_{domain}`, e.g. `am_mental_health`) against one
//! agent version; its `{prefix}` is the first 16 lowercase hex of its own
//! manifest's SHA-256 and its score is 1. An **interpret** is a judge's
//! per-criterion verdicts over one capture; it carries its capture's
//! `capture_manifest_sha256` in the manifest, its `{prefix}` is the first 16
//! hex of THAT digest, and its score carries the verdict in its sign: the
//! family's polarity is `signed` (CC 3.1.7 R4; Part 2: the sign carries the
//! verdict), so an interpretation scores `2·r − 1` for pass ratio `r`:
//! all-fail is −1, all-pass is +1, and `r` reads back as `(score + 1) / 2`.
//! Both rows land on one dimension per capture, so a re-posted
//! interpretation of the same capture is a later row on the same dimension,
//! not a new one.
//!
//! `asserted_at` is the signing time. `captured_at` in the manifest is when
//! the work was done. The two may differ by any amount: back-signing existing
//! bundles is admitted with the original run's `ci_provenance`.

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use ciris_persist::engine::Engine;
use ciris_persist::federation::trust_root::INFRA_ATTEST_SCOPE;
use ciris_persist::federation::{FederationDirectory, KeyRecord};
use serde::{Deserialize, Serialize};

use crate::fold_builds::{
    admit, check_blob, is_bare_sha256_hex, manifest_bytes, verified_rows, Blessing, BlobClaim,
    PipelineCredentials, Refusal, SignedContribution, VerifiedRow, MAX_MANIFEST_BYTES,
};

/// The family stem. A dimension under it is `{stem}{suite}:{kind}:{cell}:{prefix}:v1`.
pub const DIMENSION_STEM: &str = "evidence:";
/// The rule version this module reads and writes (CC 3.1.7 R3).
const DIMENSION_VERSION: &str = "v1";
/// Length of the `{prefix}` segment: the first 16 lowercase hex of a digest.
pub const PREFIX_LEN: usize = 16;

/// `evidence:{suite}:{kind}:{cell}:{prefix}:v1`.
#[must_use]
pub fn evidence_dimension(suite: &str, kind: &str, cell: &str, prefix: &str) -> String {
    format!("{DIMENSION_STEM}{suite}:{kind}:{cell}:{prefix}:{DIMENSION_VERSION}")
}

/// The stem of one suite's dimensions: `evidence:{suite}:`.
#[must_use]
pub fn suite_stem(suite: &str) -> String {
    format!("{DIMENSION_STEM}{suite}:")
}

/// The first [`PREFIX_LEN`] characters of a lowercase hex digest.
#[must_use]
pub fn prefix_of(sha256_hex: &str) -> String {
    sha256_hex.chars().take(PREFIX_LEN).collect()
}

/// The rubric a run was judged against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rubric {
    /// The rubric's identifier (its file or canonical name).
    pub id: String,
    /// The rubric's version.
    pub version: String,
    /// SHA-256 of the rubric bytes, lowercase hex.
    pub sha256: String,
}

/// The judge that produced an interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Judge {
    /// The judge model, as the interpreter names it.
    pub model: String,
    /// SHA-256 of the judge prompt, lowercase hex.
    pub prompt_sha256: String,
}

/// The facts an evidence Contribution attests, carried under `evidence` in
/// the envelope. The score rides the envelope's own `score`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceFacts {
    /// The suite (`safety_battery`).
    pub suite: String,
    /// The suite's kind (`capture` | `interpret` for `safety_battery`).
    pub kind: String,
    /// The cell the work was done in (`{lang}_{domain}`).
    pub cell: String,
    /// The dimension's `{prefix}`: the first 16 hex of the capture digest.
    pub prefix: String,
    /// The agent version under test.
    pub agent_version: String,
    /// The rubric, when the suite has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rubric: Option<Rubric>,
    /// The judge, for an interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub judge: Option<Judge>,
    /// For an interpretation: the capture's prefix (equal to `prefix`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_prefix: Option<String>,
    /// For an interpretation: the pass ratio `r ∈ [0, 1]` of its verdicts.
    /// Required; the envelope's score must be `2·r − 1`, so a row's score
    /// is never readable under the positive-only rule by mistake.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass_ratio: Option<f64>,
    /// The CI run that produced the bundle (repo, sha, ref, run_id, workflow),
    /// as the submitter recorded it. Opaque here; back-signed bundles carry
    /// their ORIGINAL run.
    #[serde(default)]
    pub ci_provenance: serde_json::Value,
    /// When the work was done (RFC 3339), from the manifest.
    pub captured_at: String,
    /// SHA-256 of the manifest bytes, 64 lowercase hex chars. This is the
    /// blob's address and the Contribution's `evidence_refs[0]`.
    pub manifest_hash: String,
    /// Length of the manifest in bytes (CC 5.3.2.5).
    pub manifest_size: u64,
}

/// The unsigned Contribution envelope for `facts`, ready for
/// `attestation_emit::stamp_and_canonicalize`.
#[must_use]
pub fn contribution_envelope(facts: &EvidenceFacts, score: f64) -> serde_json::Value {
    serde_json::json!({
        "dimension": evidence_dimension(&facts.suite, &facts.kind, &facts.cell, &facts.prefix),
        "score": score,
        "delegation_scope": INFRA_ATTEST_SCOPE,
        "evidence": facts,
        "evidence_refs": [facts.manifest_hash],
    })
}

/// The facts and score in an envelope, with the self-consistency a
/// Contribution owes: a known suite, its dimension names its own facts, and
/// it references its own blob. Suite-specific rules run separately.
pub fn facts_of(envelope: &serde_json::Value) -> Result<(EvidenceFacts, f64), Refusal> {
    let evidence = envelope
        .get("evidence")
        .ok_or(Refusal::Malformed("evidence"))?;
    let facts: EvidenceFacts = serde_json::from_value(evidence.clone())
        .map_err(|_| Refusal::Malformed("evidence members"))?;
    if suite(&facts.suite).is_none() {
        return Err(Refusal::SuiteUnregistered(facts.suite));
    }
    if !is_bare_sha256_hex(&facts.manifest_hash) {
        return Err(Refusal::Malformed(
            "evidence.manifest_hash is not 64 lowercase hex chars",
        ));
    }
    if facts.prefix.len() != PREFIX_LEN
        || !facts
            .prefix
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Refusal::Malformed(
            "evidence.prefix is not 16 lowercase hex chars",
        ));
    }
    if chrono::DateTime::parse_from_rfc3339(&facts.captured_at).is_err() {
        return Err(Refusal::Malformed("evidence.captured_at is not RFC 3339"));
    }
    let found = envelope
        .get("dimension")
        .and_then(|d| d.as_str())
        .ok_or(Refusal::Malformed("dimension"))?;
    let expected = evidence_dimension(&facts.suite, &facts.kind, &facts.cell, &facts.prefix);
    if found != expected {
        return Err(Refusal::DimensionMismatch {
            expected,
            found: found.to_string(),
        });
    }
    if envelope.get("delegation_scope").and_then(|s| s.as_str()) != Some(INFRA_ATTEST_SCOPE) {
        return Err(Refusal::Malformed("delegation_scope is not infra:attest"));
    }
    let names_its_blob = envelope
        .get("evidence_refs")
        .and_then(|r| r.as_array())
        .is_some_and(|refs| {
            refs.iter()
                .any(|r| r.as_str() == Some(facts.manifest_hash.as_str()))
        });
    if !names_its_blob {
        return Err(Refusal::Malformed(
            "evidence_refs does not name evidence.manifest_hash",
        ));
    }
    let score = envelope
        .get("score")
        .and_then(serde_json::Value::as_f64)
        .ok_or_else(|| {
            Refusal::EvidenceScoreInvalid("the envelope carries no numeric score".into())
        })?;
    Ok((facts, score))
}

// ───────────────────────────── suites ─────────────────────────────

/// What a suite's rows must satisfy. One implementation per suite this node
/// serves; the family's grammar (CC 3.1.5.5) fixes the segments, the suite
/// fixes their vocabulary and how the score is read off the bundle.
pub trait SuiteRules: Send + Sync {
    /// The `{suite}` segment.
    fn name(&self) -> &'static str;
    /// Checks that need only the envelope: the kind, the cell, and whatever
    /// the facts alone can prove about the prefix and score.
    fn check_facts(&self, facts: &EvidenceFacts, score: f64) -> Result<(), Refusal>;
    /// Checks that need the bundle: the prefix derivation against the bytes,
    /// the capture link, the score against the bundle's own summary.
    fn check_bundle(
        &self,
        facts: &EvidenceFacts,
        score: f64,
        manifest: &[u8],
    ) -> Result<(), Refusal>;
}

/// The suite validator for `name`, if `name` is in the registered vocabulary
/// and this node serves it.
#[must_use]
pub fn suite(name: &str) -> Option<&'static dyn SuiteRules> {
    match name {
        SafetyBattery::NAME => Some(&SafetyBattery),
        _ => None,
    }
}

/// The suites this node serves, for discovery.
#[must_use]
pub fn suites() -> &'static [&'static str] {
    &[SafetyBattery::NAME]
}

/// `safety_battery`: the agent's mental-health safety battery.
pub struct SafetyBattery;

impl SafetyBattery {
    /// The `{suite}` segment.
    pub const NAME: &'static str = "safety_battery";
    /// The `{kind}` vocabulary.
    pub const KINDS: &'static [&'static str] = &["capture", "interpret"];

    /// `{lang}_{domain}`: two or three lowercase letters, an underscore, then
    /// lowercase letters and underscores.
    fn cell_ok(cell: &str) -> bool {
        let Some((lang, domain)) = cell.split_once('_') else {
            return false;
        };
        (2..=3).contains(&lang.len())
            && lang.bytes().all(|b| b.is_ascii_lowercase())
            && !domain.is_empty()
            && domain.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
    }

    /// The pass ratio an interpret bundle reports. Read from
    /// `verdicts_summary`: `pass_ratio` when present, else `passed` (or
    /// `pass`) over `total` (or `passed + failed`).
    fn pass_ratio(manifest: &serde_json::Value) -> Result<f64, Refusal> {
        let summary = manifest.get("verdicts_summary").ok_or_else(|| {
            Refusal::EvidenceScoreInvalid("the manifest has no verdicts_summary".into())
        })?;
        if let Some(r) = summary
            .get("pass_ratio")
            .and_then(serde_json::Value::as_f64)
        {
            return Self::ratio_in_range(r);
        }
        let count = |k: &str| summary.get(k).and_then(serde_json::Value::as_f64);
        let passed = count("passed").or_else(|| count("pass")).ok_or_else(|| {
            Refusal::EvidenceScoreInvalid("verdicts_summary names no passed count".into())
        })?;
        let total = count("total").or_else(|| {
            count("failed")
                .or_else(|| count("fail"))
                .map(|f| passed + f)
        });
        let total = total.ok_or_else(|| {
            Refusal::EvidenceScoreInvalid("verdicts_summary names no total or failed count".into())
        })?;
        if total <= 0.0 {
            return Err(Refusal::EvidenceScoreInvalid(
                "verdicts_summary total is zero".into(),
            ));
        }
        Self::ratio_in_range(passed / total)
    }

    fn ratio_in_range(r: f64) -> Result<f64, Refusal> {
        if r.is_finite() && (0.0..=1.0).contains(&r) {
            Ok(r)
        } else {
            Err(Refusal::EvidenceScoreInvalid(format!(
                "the bundle's pass ratio {r} is outside [0, 1]"
            )))
        }
    }

    /// The score an interpretation carries for pass ratio `r`: `2·r − 1`.
    #[must_use]
    pub fn interpret_score(pass_ratio: f64) -> f64 {
        2.0 * pass_ratio - 1.0
    }

    /// The pass ratio an interpretation's score encodes: `(score + 1) / 2`.
    #[must_use]
    pub fn pass_ratio_of(score: f64) -> f64 {
        (score + 1.0) / 2.0
    }
}

/// The family's polarity is `signed` (CC 3.1.7 R4): a score in [−1, 1].
fn score_in_polarity(score: f64) -> Result<(), Refusal> {
    if score.is_finite() && (-1.0..=1.0).contains(&score) {
        Ok(())
    } else {
        Err(Refusal::EvidenceScoreInvalid(format!(
            "{score} is outside the family's signed polarity [-1, 1]"
        )))
    }
}

impl SuiteRules for SafetyBattery {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn check_facts(&self, facts: &EvidenceFacts, score: f64) -> Result<(), Refusal> {
        if !Self::KINDS.contains(&facts.kind.as_str()) {
            return Err(Refusal::EvidenceKindInvalid(facts.kind.clone()));
        }
        if !Self::cell_ok(&facts.cell) {
            return Err(Refusal::EvidenceCellInvalid(facts.cell.clone()));
        }
        score_in_polarity(score)?;
        match facts.kind.as_str() {
            "capture" => {
                // A capture's prefix is its own digest's: provable from the facts.
                let expected = prefix_of(&facts.manifest_hash);
                if facts.prefix != expected {
                    return Err(Refusal::EvidencePrefixMismatch {
                        expected,
                        found: facts.prefix.clone(),
                    });
                }
                if score != 1.0 {
                    return Err(Refusal::EvidenceScoreInvalid(format!(
                        "a capture scores 1, not {score}"
                    )));
                }
            }
            _ => {
                // An interpretation states its pass ratio and scores 2·r − 1.
                let r = facts.pass_ratio.ok_or_else(|| {
                    Refusal::EvidenceScoreInvalid(
                        "an interpretation must carry evidence.pass_ratio".into(),
                    )
                })?;
                let r = Self::ratio_in_range(r)?;
                let expected = Self::interpret_score(r);
                if (expected - score).abs() > 1e-9 {
                    return Err(Refusal::EvidenceScoreInvalid(format!(
                        "pass_ratio {r} scores {expected} (2·r − 1), not {score}"
                    )));
                }
                // An interpretation names its capture; the prefix must be that one's.
                if let Some(cp) = &facts.capture_prefix {
                    if *cp != facts.prefix {
                        return Err(Refusal::EvidencePrefixMismatch {
                            expected: cp.clone(),
                            found: facts.prefix.clone(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn check_bundle(
        &self,
        facts: &EvidenceFacts,
        score: f64,
        manifest: &[u8],
    ) -> Result<(), Refusal> {
        let json: serde_json::Value = serde_json::from_slice(manifest)
            .map_err(|_| Refusal::Malformed("manifest is not JSON"))?;
        if facts.kind == "interpret" {
            let capture = json
                .get("capture_manifest_sha256")
                .and_then(|c| c.as_str())
                .filter(|c| is_bare_sha256_hex(c))
                .ok_or(Refusal::EvidenceCaptureLinkMissing)?;
            let expected = prefix_of(capture);
            if facts.prefix != expected {
                return Err(Refusal::EvidencePrefixMismatch {
                    expected,
                    found: facts.prefix.clone(),
                });
            }
            let ratio = Self::pass_ratio(&json)?;
            if facts.pass_ratio.is_some_and(|r| (r - ratio).abs() > 1e-9) {
                return Err(Refusal::EvidenceScoreInvalid(format!(
                    "evidence.pass_ratio {:?} is not the bundle's {ratio}",
                    facts.pass_ratio
                )));
            }
            let expected = Self::interpret_score(ratio);
            if (expected - score).abs() > 1e-9 {
                return Err(Refusal::EvidenceScoreInvalid(format!(
                    "the Contribution scores {score}, but the bundle's verdicts give pass ratio \
                     {ratio}, which scores {expected} (2·r − 1)"
                )));
            }
        }
        Ok(())
    }
}

// ───────────────────────────── admit / read ─────────────────────────────

/// What [`submit`] stored.
#[derive(Debug, Clone, Serialize)]
pub struct Accepted {
    /// The Contribution's row id.
    pub attestation_id: String,
    /// The row's dimension.
    pub dimension: String,
    /// The manifest blob's address.
    pub manifest_sha256: String,
    /// The key that signed.
    pub signer_key_id: String,
    /// How the signer holds `infra:attest`.
    #[serde(flatten)]
    pub blessing: Blessing,
    /// `false` when this exact Contribution was already held.
    pub newly_stored: bool,
}

/// Admit an evidence Contribution and its manifest bytes under `suite_name`.
///
/// Order is cheapest-first: the suite, the cap, the envelope, the suite's
/// envelope rules, the bytes against the claim, the suite's bundle rules, then
/// the shared admission (signer, signature, blessing, storage).
pub async fn submit(
    engine: &Arc<Engine>,
    node_key_id: &str,
    suite_name: &str,
    contribution: SignedContribution,
    manifest: &[u8],
    credentials: PipelineCredentials,
) -> Result<Accepted, Refusal> {
    let rules =
        suite(suite_name).ok_or_else(|| Refusal::SuiteUnregistered(suite_name.to_string()))?;
    if manifest.len() > MAX_MANIFEST_BYTES {
        return Err(Refusal::ManifestTooLarge(manifest.len()));
    }
    let (facts, score) = facts_of(&contribution.signed_envelope)?;
    if facts.suite != suite_name {
        return Err(Refusal::DimensionMismatch {
            expected: suite_stem(suite_name),
            found: evidence_dimension(&facts.suite, &facts.kind, &facts.cell, &facts.prefix),
        });
    }
    rules.check_facts(&facts, score)?;
    let claim = BlobClaim {
        hash: facts.manifest_hash.clone(),
        size: facts.manifest_size,
    };
    check_blob(manifest, &claim)?;
    rules.check_bundle(&facts, score, manifest)?;
    let admitted = admit(
        engine,
        node_key_id,
        contribution,
        manifest,
        &claim,
        credentials,
        "application/json",
    )
    .await?;
    Ok(Accepted {
        attestation_id: admitted.attestation_id,
        dimension: evidence_dimension(&facts.suite, &facts.kind, &facts.cell, &facts.prefix),
        manifest_sha256: facts.manifest_hash,
        signer_key_id: admitted.pipeline_key_id,
        blessing: admitted.blessing,
        newly_stored: admitted.newly_stored,
    })
}

/// A stored evidence row that passed every check, now: the facts and the score.
pub type VerifiedEvidence = VerifiedRow<(EvidenceFacts, f64)>;

/// Every currently valid evidence row of `suite_name` this node holds,
/// newest assertion first.
pub async fn verified_evidence(
    directory: &dyn FederationDirectory,
    node_key_id: &str,
    suite_name: &str,
) -> Result<Vec<VerifiedEvidence>, Refusal> {
    if suite(suite_name).is_none() {
        return Err(Refusal::SuiteUnregistered(suite_name.to_string()));
    }
    let stem = suite_stem(suite_name);
    let mut rows = verified_rows(directory, node_key_id, &stem, |env| {
        let (facts, score) = facts_of(env)?;
        // A stored row is re-checked against the suite's envelope rules too.
        suite(&facts.suite)
            .ok_or_else(|| Refusal::SuiteUnregistered(facts.suite.clone()))?
            .check_facts(&facts, score)?;
        Ok((facts, score))
    })
    .await?;
    rows.sort_by(|a, b| b.asserted_at.cmp(&a.asserted_at));
    Ok(rows)
}

// ───────────────────────────── HTTP ─────────────────────────────

#[derive(Clone)]
struct EvidenceState {
    engine: Arc<Engine>,
    node_key_id: String,
}

/// The evidence router over the server's Engine. Mounted by
/// [`crate::fold::router`] only.
pub fn router(engine: Arc<Engine>, node_key_id: String) -> Router {
    Router::new()
        .route(
            "/v1/evidence/{suite}",
            post(submit_evidence).get(list_evidence),
        )
        .route(
            "/v1/evidence/{suite}/{manifest_hash}",
            get(evidence_by_hash),
        )
        .route(
            "/v1/evidence/{suite}/blob/{manifest_hash}",
            get(blob_by_hash),
        )
        .with_state(EvidenceState {
            engine,
            node_key_id,
        })
}

#[derive(Deserialize)]
struct SubmitBody {
    contribution: SignedContribution,
    manifest_base64: String,
    #[serde(flatten)]
    credentials: PipelineCredentials,
}

async fn submit_evidence(
    State(st): State<EvidenceState>,
    Path(suite_name): Path<String>,
    Json(body): Json<SubmitBody>,
) -> Response {
    let Ok(manifest) = B64.decode(body.manifest_base64.as_bytes()) else {
        return Refusal::Malformed("manifest_base64").into_response();
    };
    match submit(
        &st.engine,
        &st.node_key_id,
        &suite_name,
        body.contribution,
        &manifest,
        body.credentials,
    )
    .await
    {
        Ok(accepted) => {
            tracing::info!(
                attestation = %accepted.attestation_id,
                signer = %accepted.signer_key_id,
                dimension = %accepted.dimension,
                "evidence Contribution admitted"
            );
            (StatusCode::CREATED, Json(accepted)).into_response()
        }
        Err(refusal) => {
            tracing::warn!(reason = refusal.token(), suite = %suite_name, "evidence Contribution refused: {refusal}");
            refusal.into_response()
        }
    }
}

#[derive(Serialize)]
struct Standing {
    scope: &'static str,
    #[serde(flatten)]
    blessing: Blessing,
    decided_by: String,
}

/// One evidence row as served.
#[derive(Serialize)]
struct EvidenceResponse {
    dimension: String,
    #[serde(flatten)]
    facts: EvidenceFacts,
    /// The row's score. A capture scores 1. An interpretation's sign carries
    /// the verdict: `2·r − 1` for pass ratio `r`, so the ratio is
    /// `(score + 1) / 2`.
    score: f64,
    attestation_id: String,
    asserted_at: String,
    signer_key_id: String,
    /// Whether this node holds the manifest bytes.
    blob_held: bool,
    /// The Contribution as the signer signed it; a reader re-verifies it
    /// against `signer_record` and need not take the fields above on this
    /// node's word.
    contribution: SignedContribution,
    standing: Standing,
    signer_record: KeyRecord,
}

async fn respond_row(st: &EvidenceState, row: VerifiedEvidence) -> EvidenceResponse {
    let (facts, score) = row.facts;
    let held = manifest_bytes(&st.engine, &st.node_key_id, &facts.manifest_hash)
        .await
        .is_some();
    EvidenceResponse {
        dimension: evidence_dimension(&facts.suite, &facts.kind, &facts.cell, &facts.prefix),
        facts,
        score,
        attestation_id: row.attestation_id,
        asserted_at: row.asserted_at.to_rfc3339(),
        signer_key_id: row.pipeline_key_id,
        blob_held: held,
        contribution: row.contribution,
        standing: Standing {
            scope: INFRA_ATTEST_SCOPE,
            blessing: row.blessing,
            decided_by: st.node_key_id.clone(),
        },
        signer_record: row.pipeline_record,
    }
}

#[derive(Deserialize, Default)]
struct ListQuery {
    cell: Option<String>,
    kind: Option<String>,
    agent_version: Option<String>,
    /// Matches the rubric's id.
    rubric: Option<String>,
    /// RFC 3339; rows whose `captured_at` is at or after it.
    since: Option<String>,
}

fn not_found(what: &str) -> Response {
    let body = serde_json::json!({
        "error": "evidence_not_found",
        "detail": format!("no verified evidence Contribution for {what}"),
    });
    (StatusCode::NOT_FOUND, Json(body)).into_response()
}

async fn list_evidence(
    State(st): State<EvidenceState>,
    Path(suite_name): Path<String>,
    Query(q): Query<ListQuery>,
) -> Response {
    let since = match q.since.as_deref() {
        Some(raw) => match chrono::DateTime::parse_from_rfc3339(raw) {
            Ok(t) => Some(t.with_timezone(&chrono::Utc)),
            Err(_) => return Refusal::Malformed("since is not RFC 3339").into_response(),
        },
        None => None,
    };
    let directory = st.engine.federation_directory();
    let rows = match verified_evidence(directory.as_ref(), &st.node_key_id, &suite_name).await {
        Ok(r) => r,
        Err(e) => return e.into_response(),
    };
    let mut out = Vec::new();
    for row in rows {
        let (facts, _) = &row.facts;
        if q.cell.as_deref().is_some_and(|c| c != facts.cell)
            || q.kind.as_deref().is_some_and(|k| k != facts.kind)
            || q.agent_version
                .as_deref()
                .is_some_and(|v| v != facts.agent_version)
            || q.rubric
                .as_deref()
                .is_some_and(|r| facts.rubric.as_ref().is_none_or(|x| x.id != r))
        {
            continue;
        }
        if let Some(since) = since {
            let captured = chrono::DateTime::parse_from_rfc3339(&facts.captured_at)
                .map(|t| t.with_timezone(&chrono::Utc));
            if captured.is_ok_and(|t| t < since) {
                continue;
            }
        }
        out.push(respond_row(&st, row).await);
    }
    Json(serde_json::json!({ "suite": suite_name, "evidence": out })).into_response()
}

async fn evidence_by_hash(
    State(st): State<EvidenceState>,
    Path((suite_name, hash)): Path<(String, String)>,
) -> Response {
    let directory = st.engine.federation_directory();
    let rows = match verified_evidence(directory.as_ref(), &st.node_key_id, &suite_name).await {
        Ok(r) => r,
        Err(e) => return e.into_response(),
    };
    match rows.into_iter().find(|r| r.facts.0.manifest_hash == hash) {
        Some(row) => Json(respond_row(&st, row).await).into_response(),
        None => not_found(&format!("manifest {hash}")),
    }
}

/// The raw manifest bytes, but only for a manifest a valid Contribution names.
async fn blob_by_hash(
    State(st): State<EvidenceState>,
    Path((suite_name, hash)): Path<(String, String)>,
) -> Response {
    let directory = st.engine.federation_directory();
    let attested = match verified_evidence(directory.as_ref(), &st.node_key_id, &suite_name).await {
        Ok(r) => r.into_iter().any(|r| r.facts.0.manifest_hash == hash),
        Err(e) => return e.into_response(),
    };
    if !attested {
        return not_found(&format!("manifest {hash}"));
    }
    match manifest_bytes(&st.engine, &st.node_key_id, &hash).await {
        Some(bytes) => ([(header::CONTENT_TYPE, "application/json")], bytes).into_response(),
        None => {
            let body = serde_json::json!({
                "error": "manifest_not_held",
                "detail": "the Contribution is here and its manifest bytes are not, yet",
            });
            (StatusCode::NOT_FOUND, Json(body)).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(kind: &str, manifest_hash: &str, prefix: &str) -> EvidenceFacts {
        EvidenceFacts {
            suite: "safety_battery".into(),
            kind: kind.into(),
            cell: "am_mental_health".into(),
            prefix: prefix.into(),
            agent_version: "2.16.5".into(),
            rubric: None,
            judge: None,
            capture_prefix: None,
            pass_ratio: None,
            ci_provenance: serde_json::Value::Null,
            captured_at: "2026-10-01T00:00:00Z".into(),
            manifest_hash: manifest_hash.into(),
            manifest_size: 2,
        }
    }

    #[test]
    fn the_dimension_follows_cc_3_1_5_5() {
        assert_eq!(
            evidence_dimension(
                "safety_battery",
                "capture",
                "am_mental_health",
                "0123456789abcdef"
            ),
            "evidence:safety_battery:capture:am_mental_health:0123456789abcdef:v1"
        );
    }

    #[test]
    fn a_capture_envelope_round_trips_through_facts_of() {
        let h = "ab".repeat(32);
        let f = facts("capture", &h, &prefix_of(&h));
        let (back, score) = facts_of(&contribution_envelope(&f, 1.0)).unwrap();
        assert_eq!(back, f);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn an_unregistered_suite_is_refused_with_ccs_token() {
        let h = "ab".repeat(32);
        let mut f = facts("capture", &h, &prefix_of(&h));
        f.suite = "coherence_bench".into();
        let err = facts_of(&contribution_envelope(&f, 1.0)).unwrap_err();
        assert_eq!(err.token(), "namespace_vocab_value_unregistered");
    }

    #[test]
    fn safety_battery_checks_kind_cell_prefix_and_score() {
        let h = "ab".repeat(32);
        let ok = facts("capture", &h, &prefix_of(&h));
        SafetyBattery.check_facts(&ok, 1.0).unwrap();

        let mut bad_kind = ok.clone();
        bad_kind.kind = "judge".into();
        assert_eq!(
            SafetyBattery
                .check_facts(&bad_kind, 1.0)
                .unwrap_err()
                .token(),
            "evidence_kind_invalid"
        );

        let mut bad_cell = ok.clone();
        bad_cell.cell = "Amharic".into();
        assert_eq!(
            SafetyBattery
                .check_facts(&bad_cell, 1.0)
                .unwrap_err()
                .token(),
            "evidence_cell_invalid"
        );

        let mut bad_prefix = ok.clone();
        bad_prefix.prefix = "0000000000000000".into();
        assert_eq!(
            SafetyBattery
                .check_facts(&bad_prefix, 1.0)
                .unwrap_err()
                .token(),
            "evidence_prefix_mismatch"
        );

        assert_eq!(
            SafetyBattery.check_facts(&ok, 0.5).unwrap_err().token(),
            "evidence_score_invalid"
        );
        assert_eq!(
            SafetyBattery.check_facts(&ok, 0.0).unwrap_err().token(),
            "evidence_score_invalid"
        );

        let mut interp = ok.clone();
        interp.kind = "interpret".into();
        assert_eq!(
            SafetyBattery.check_facts(&interp, 0.0).unwrap_err().token(),
            "evidence_score_invalid",
            "an interpretation without pass_ratio"
        );
        interp.pass_ratio = Some(0.0);
        SafetyBattery.check_facts(&interp, -1.0).unwrap();
        interp.pass_ratio = Some(0.5);
        SafetyBattery.check_facts(&interp, 0.0).unwrap();
        assert_eq!(
            SafetyBattery.check_facts(&interp, 0.5).unwrap_err().token(),
            "evidence_score_invalid",
            "the ratio itself is not the score"
        );
        interp.pass_ratio = Some(1.25);
        assert_eq!(
            SafetyBattery.check_facts(&interp, 1.5).unwrap_err().token(),
            "evidence_score_invalid"
        );
    }

    #[test]
    fn an_interpret_bundle_must_name_its_capture_and_its_pass_ratio() {
        let capture = "cd".repeat(32);
        let h = "ef".repeat(32);
        let mut f = facts("interpret", &h, &prefix_of(&capture));
        f.pass_ratio = Some(0.75);
        let no_link = br#"{"verdicts_summary":{"passed":3,"total":4}}"#;
        assert_eq!(
            SafetyBattery
                .check_bundle(&f, 0.75, no_link)
                .unwrap_err()
                .token(),
            "evidence_capture_link_missing"
        );
        let good = format!(
            r#"{{"capture_manifest_sha256":"{capture}","verdicts_summary":{{"passed":3,"total":4}}}}"#
        );
        // 3 of 4 pass: r = 0.75, score = 2r − 1 = 0.5.
        SafetyBattery
            .check_bundle(&f, 0.5, good.as_bytes())
            .unwrap();
        // The pass ratio itself, positive-only style, is not the score.
        assert_eq!(
            SafetyBattery
                .check_bundle(&f, 0.75, good.as_bytes())
                .unwrap_err()
                .token(),
            "evidence_score_invalid"
        );
        assert_eq!(
            SafetyBattery::pass_ratio_of(SafetyBattery::interpret_score(0.75)),
            0.75
        );
        let all_fail = format!(
            r#"{{"capture_manifest_sha256":"{capture}","verdicts_summary":{{"passed":0,"total":4}}}}"#
        );
        let mut none_pass = f.clone();
        none_pass.pass_ratio = Some(0.0);
        SafetyBattery
            .check_bundle(&none_pass, -1.0, all_fail.as_bytes())
            .unwrap();
        assert_eq!(
            SafetyBattery
                .check_bundle(&f, -1.0, all_fail.as_bytes())
                .unwrap_err()
                .token(),
            "evidence_score_invalid",
            "the envelope's pass_ratio must be the bundle's"
        );
        let over = format!(
            r#"{{"capture_manifest_sha256":"{capture}","verdicts_summary":{{"pass_ratio":1.25}}}}"#
        );
        assert_eq!(
            SafetyBattery
                .check_bundle(&f, 1.0, over.as_bytes())
                .unwrap_err()
                .token(),
            "evidence_score_invalid"
        );
        let wrong_capture = format!(
            r#"{{"capture_manifest_sha256":"{}","verdicts_summary":{{"pass_ratio":0.75}}}}"#,
            "00".repeat(32)
        );
        assert_eq!(
            SafetyBattery
                .check_bundle(&f, 0.75, wrong_capture.as_bytes())
                .unwrap_err()
                .token(),
            "evidence_prefix_mismatch"
        );
    }
}
