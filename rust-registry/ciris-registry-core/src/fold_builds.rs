//! CEG-native build manifests — the fold surface for `/v1/builds`.
//!
//! FSD-004 §4.1 / §4.2 / §6.2. A build is no longer a row the registry signs.
//! It is two things a CI pipeline publishes and any node can check:
//!
//! 1. a **Contribution**: a `scores` attestation on
//!    `provenance:build_manifest:{target}:v1`, hybrid-signed by the pipeline's
//!    own key, naming the manifest by hash in `evidence_refs`;
//! 2. the **manifest bytes**, a commons blob addressed by that hash.
//!
//! The Contribution is only worth anything if the pipeline key holds
//! `infra:attest` from a trust root THIS node accepts: a live
//! `delegates_to(root → pipeline, infra:attest)` grant. A role on the
//! pipeline's key record is not enough; the capability walk reads a co-scrubbed
//! record as "this key is itself a root", which a pipeline is not. That is the whole
//! authority model: no admin bearer, no registry signature, no allowlist of CI
//! keys kept here. `capability_roots_to_trusted_root` answers it, the same walk
//! the server's registry-slice gate uses for itself.
//!
//! # Why the envelope is persist's, not verify's
//!
//! `ciris_verify_core::manifest_contribution::sign_build_manifest_contribution`
//! (verify v18) emits an envelope persist v52 cannot store, for two reasons
//! measured against a live Engine (`tests/build_manifest_fold.rs`):
//!
//! - its dimension has no trailing version segment, which CC 3.1.7 R3 requires
//!   and persist's admission refuses as `missing_version_segment`;
//! - it carries no signed `row` mirror (CIRISPersist#643), so the row's columns
//!   would be chosen by whoever stored it rather than by the signer.
//!
//! So this module reads the persist shape: the Contribution's facts inside an
//! envelope stamped by `attestation_emit::stamp_and_canonicalize`, signed over
//! persist's canonical bytes (which are JCS, so verify's bound-hybrid signer
//! produces them unchanged). [`contribution_envelope`] is the one place that
//! shape is spelled, for producers and tests alike.
//!
//! # Fail-closed at both ends
//!
//! The submit door refuses an unblessed pipeline. The read re-checks anyway:
//! rows also arrive by anti-entropy, and a blessing can be withdrawn after a
//! row was admitted. A Contribution whose pipeline is not blessed *now* yields
//! no build, never a build with weaker provenance.

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
use ciris_persist::federation::envelope::EnvelopeCore;
use ciris_persist::federation::trust_root::{
    capability_roots_to_trusted_root, TrustedGrant, INFRA_ATTEST_SCOPE,
};
use ciris_persist::federation::types::{
    attestation_type, cohort_scope, identity_type, Attestation, SignedAttestation,
};
use ciris_persist::federation::{
    attestation_emit, BlobBody, EmitAttestationInput, FederationDirectory, KeyRecord,
    SignedKeyRecord,
};
use ciris_verify_core::threshold::{verify_threshold_signatures, ThresholdMember, ThresholdSignature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// The family stem. A dimension under it is `{stem}{target}:v1`.
const DIMENSION_STEM: &str = "provenance:build_manifest:";
/// The rule version this module reads and writes (CC 3.1.7 R3).
const DIMENSION_VERSION: &str = "v1";
/// Largest manifest the submit door takes. A file manifest for the agent tree
/// is a few hundred KiB; persist's inline blob cap is 1 MiB.
const MAX_MANIFEST_BYTES: usize = 1024 * 1024;

/// `provenance:build_manifest:{target}:v1`.
#[must_use]
pub fn build_manifest_dimension(target: &str) -> String {
    format!("{DIMENSION_STEM}{target}:{DIMENSION_VERSION}")
}

/// The facts a Contribution attests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildFacts {
    /// Build target (`python-source-tree`, a Rust triple, …).
    pub target: String,
    /// The build's identifier.
    pub build_id: String,
    /// SHA-256 of the built artifact, lowercase hex.
    pub binary_hash: String,
    /// The version string the artifact reports.
    pub binary_version: String,
    /// SHA-256 of the manifest bytes, 64 lowercase hex chars. This is the
    /// blob's address and the Contribution's `evidence_refs[0]`.
    pub manifest_hash: String,
    /// Length of the manifest in bytes (CC 5.3.2.5: every blob carries its
    /// size). A reader checks this before hashing, so an oversized body is
    /// refused without being read to the end.
    pub manifest_size: u64,
}

/// The unsigned Contribution envelope for `facts`, ready for
/// `attestation_emit::stamp_and_canonicalize`.
///
/// One spelling of the shape. A producer stamps it, signs the canonical bytes
/// with the pipeline key, and submits the result as a [`SignedContribution`].
#[must_use]
pub fn contribution_envelope(facts: &BuildFacts) -> serde_json::Value {
    serde_json::json!({
        "dimension": build_manifest_dimension(&facts.target),
        "score": 1,
        "delegation_scope": INFRA_ATTEST_SCOPE,
        "build": facts,
        "evidence_refs": [facts.manifest_hash],
    })
}

/// A Contribution as it travels: the stamped envelope and the pipeline's
/// bound-hybrid signature over its canonical bytes. The same three members
/// verify's `SignedEnvelope` carries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedContribution {
    /// The stamped envelope, `row` mirror included.
    pub signed_envelope: serde_json::Value,
    /// Ed25519 over the canonical envelope.
    pub ed25519_signature_base64: String,
    /// ML-DSA-65 over `canonical || ed25519_signature`.
    pub mldsa65_signature_base64: String,
}

/// Why a Contribution was not accepted. Every variant is a hard refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// The envelope is not a build-manifest Contribution. Names the member.
    Malformed(&'static str),
    /// The dimension is not `provenance:build_manifest:{build.target}:v1`.
    DimensionMismatch { expected: String, found: String },
    /// The submitted bytes do not hash to `build.manifest_hash`.
    ManifestHashMismatch { expected: String, found: String },
    /// The submitted bytes are not `build.manifest_size` long.
    ManifestSizeMismatch { expected: u64, found: u64 },
    /// The manifest is larger than [`MAX_MANIFEST_BYTES`].
    ManifestTooLarge(usize),
    /// The signing key is not in this node's directory.
    UnknownPipeline(String),
    /// The hybrid signature does not verify against the directory's pubkeys.
    SignatureInvalid,
    /// The pipeline key holds no `infra:attest` from a root this node accepts.
    PipelineNotBlessed(String),
    /// The substrate refused or failed. Carries its message.
    Substrate(String),
}

impl Refusal {
    fn status(&self) -> StatusCode {
        match self {
            Self::Malformed(_)
            | Self::DimensionMismatch { .. }
            | Self::ManifestHashMismatch { .. }
            | Self::ManifestSizeMismatch { .. } => StatusCode::BAD_REQUEST,
            Self::ManifestTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            Self::UnknownPipeline(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::SignatureInvalid => StatusCode::UNAUTHORIZED,
            Self::PipelineNotBlessed(_) => StatusCode::FORBIDDEN,
            Self::Substrate(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// A stable token a caller can branch on.
    #[must_use]
    pub fn token(&self) -> &'static str {
        match self {
            Self::Malformed(_) => "contribution_malformed",
            Self::DimensionMismatch { .. } => "dimension_mismatch",
            Self::ManifestHashMismatch { .. } => "manifest_hash_mismatch",
            Self::ManifestSizeMismatch { .. } => "manifest_size_mismatch",
            Self::ManifestTooLarge(_) => "manifest_too_large",
            Self::UnknownPipeline(_) => "unknown_pipeline",
            Self::SignatureInvalid => "signature_invalid",
            Self::PipelineNotBlessed(_) => "pipeline_not_blessed",
            Self::Substrate(_) => "substrate_error",
        }
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed(m) => write!(f, "not a build-manifest Contribution: {m}"),
            Self::DimensionMismatch { expected, found } => {
                write!(f, "dimension is {found:?}, expected {expected:?} for this build's target")
            }
            Self::ManifestHashMismatch { expected, found } => write!(
                f,
                "the manifest bytes hash to {found}, but the Contribution attests {expected}"
            ),
            Self::ManifestSizeMismatch { expected, found } => write!(
                f,
                "the manifest is {found} bytes, but the Contribution attests {expected}"
            ),
            Self::ManifestTooLarge(n) => {
                write!(f, "manifest is {n} bytes; the limit is {MAX_MANIFEST_BYTES}")
            }
            Self::UnknownPipeline(k) => write!(
                f,
                "pipeline key {k:?} is not in this node's directory; submit its accord-blessed \
                 key record as `pipeline_record`"
            ),
            Self::SignatureInvalid => {
                f.write_str("the hybrid signature does not verify against the pipeline's registered keys")
            }
            Self::PipelineNotBlessed(k) => write!(
                f,
                "pipeline key {k:?} holds no {INFRA_ATTEST_SCOPE} from a trust root this node \
                 accepts; a build manifest is valid only from a blessed pipeline"
            ),
            Self::Substrate(e) => write!(f, "substrate: {e}"),
        }
    }
}

impl IntoResponse for Refusal {
    fn into_response(self) -> Response {
        let body = serde_json::json!({ "error": self.token(), "detail": self.to_string() });
        (self.status(), Json(body)).into_response()
    }
}

fn substrate(e: impl std::fmt::Display) -> Refusal {
    Refusal::Substrate(e.to_string())
}

fn is_bare_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The build facts in an envelope, with the self-consistency a Contribution
/// owes: its dimension names its own target, and it references its own blob.
fn facts_of(envelope: &serde_json::Value) -> Result<BuildFacts, Refusal> {
    let build = envelope.get("build").ok_or(Refusal::Malformed("build"))?;
    let facts: BuildFacts =
        serde_json::from_value(build.clone()).map_err(|_| Refusal::Malformed("build members"))?;
    if !is_bare_sha256_hex(&facts.manifest_hash) {
        return Err(Refusal::Malformed("build.manifest_hash is not 64 lowercase hex chars"));
    }
    let found = envelope
        .get("dimension")
        .and_then(|d| d.as_str())
        .ok_or(Refusal::Malformed("dimension"))?;
    let expected = build_manifest_dimension(&facts.target);
    if found != expected {
        return Err(Refusal::DimensionMismatch { expected, found: found.to_string() });
    }
    if envelope.get("delegation_scope").and_then(|s| s.as_str()) != Some(INFRA_ATTEST_SCOPE) {
        return Err(Refusal::Malformed("delegation_scope is not infra:attest"));
    }
    let names_its_blob = envelope
        .get("evidence_refs")
        .and_then(|r| r.as_array())
        .is_some_and(|refs| refs.iter().any(|r| r.as_str() == Some(facts.manifest_hash.as_str())));
    if !names_its_blob {
        return Err(Refusal::Malformed("evidence_refs does not name build.manifest_hash"));
    }
    Ok(facts)
}

/// Does `(ed, pqc)` verify over `canonical` against `key`'s registered pubkeys?
fn signature_verifies(key: &KeyRecord, canonical: &[u8], ed: &str, pqc: Option<&str>) -> bool {
    let member = ThresholdMember {
        member_id: key.key_id.clone(),
        ed25519_public_key_base64: key.pubkey_ed25519_base64.clone(),
        mldsa65_public_key_base64: key.pubkey_ml_dsa_65_base64.clone(),
        role: None,
    };
    let sig = ThresholdSignature {
        member_id: key.key_id.clone(),
        ed25519_signature_base64: ed.to_string(),
        mldsa65_signature_base64: pqc.map(str::to_string),
    };
    verify_threshold_signatures(canonical, std::slice::from_ref(&member), &[sig], 1) == Ok(1)
}

/// Is `pipeline` blessed to attest builds, as far as `node` is concerned?
///
/// "As far as this node is concerned" is the point. The root must be one this
/// node accepts, so an operator who withdraws that acceptance stops serving
/// builds rooted in it without touching this code.
async fn blessing(
    directory: &dyn FederationDirectory,
    node_key_id: &str,
    pipeline_key_id: &str,
) -> Result<Option<TrustedGrant>, Refusal> {
    capability_roots_to_trusted_root(directory, node_key_id, pipeline_key_id, INFRA_ATTEST_SCOPE)
        .await
        .map_err(substrate)
}

/// A stored Contribution that passed every check, now.
#[derive(Debug, Clone)]
pub struct VerifiedBuild {
    /// The attested facts.
    pub facts: BuildFacts,
    /// The row that carries them.
    pub attestation_id: String,
    /// The pipeline key that signed it.
    pub pipeline_key_id: String,
    /// The trust root the pipeline's `infra:attest` roots in.
    pub conferred_by: String,
    /// The grant the walk found, so a reader can fetch and check it.
    pub grant_attestation_id: String,
    /// The Contribution exactly as the pipeline signed it.
    pub contribution: SignedContribution,
}

/// Check a stored row end to end. `None` for anything that is not a currently
/// valid build-manifest Contribution; the reason is logged, not returned,
/// because a read that finds nothing valid has nothing to say about the rest.
async fn verify_row(
    directory: &dyn FederationDirectory,
    node_key_id: &str,
    pipeline: &KeyRecord,
    row: &Attestation,
) -> Option<VerifiedBuild> {
    if row.attestation_type != attestation_type::SCORES || row.cohort_scope != cohort_scope::FEDERATION {
        return None;
    }
    let is_family = row
        .attestation_envelope
        .get("dimension")
        .and_then(|d| d.as_str())
        .is_some_and(|d| d.starts_with(DIMENSION_STEM));
    if !is_family {
        return None;
    }
    let facts = match facts_of(&row.attestation_envelope) {
        Ok(f) => f,
        Err(e) => {
            tracing::warn!(attestation = %row.attestation_id, "build manifest row ignored: {e}");
            return None;
        }
    };
    let canonical = attestation_emit::canonicalize(&row.attestation_envelope).ok()?;
    if !signature_verifies(
        pipeline,
        &canonical,
        &row.scrub_signature_classical,
        row.scrub_signature_pqc.as_deref(),
    ) {
        tracing::warn!(
            attestation = %row.attestation_id,
            pipeline = %pipeline.key_id,
            "build manifest row ignored: its signature does not re-verify"
        );
        return None;
    }
    let grant = match blessing(directory, node_key_id, &pipeline.key_id).await {
        Ok(Some(g)) => g,
        Ok(None) => {
            tracing::info!(
                attestation = %row.attestation_id,
                pipeline = %pipeline.key_id,
                "build manifest row ignored: its pipeline is not blessed for infra:attest here"
            );
            return None;
        }
        Err(e) => {
            tracing::warn!(pipeline = %pipeline.key_id, "blessing walk failed: {e}");
            return None;
        }
    };
    Some(VerifiedBuild {
        facts,
        attestation_id: row.attestation_id.clone(),
        pipeline_key_id: pipeline.key_id.clone(),
        conferred_by: grant.root_key_id,
        grant_attestation_id: grant.grant_attestation_id,
        contribution: SignedContribution {
            signed_envelope: row.attestation_envelope.clone(),
            ed25519_signature_base64: row.scrub_signature_classical.clone(),
            mldsa65_signature_base64: row.scrub_signature_pqc.clone().unwrap_or_default(),
        },
    })
}

/// Every currently valid build this node holds a Contribution for.
///
/// The directory indexes attestations by attester and has no dimension index,
/// so the walk starts from who may attest: `node` keys. That is every peer on
/// a busy node; it is acceptable while builds are read rarely, and the place
/// to put an index when they are not.
pub async fn verified_builds(
    directory: &dyn FederationDirectory,
    node_key_id: &str,
) -> Result<Vec<VerifiedBuild>, Refusal> {
    let nodes = directory
        .list_keys_by_identity_type(identity_type::NODE)
        .await
        .map_err(substrate)?;
    let mut out = Vec::new();
    for pipeline in nodes {
        let Ok(rows) = directory.list_attestations_by(&pipeline.key_id).await else { continue };
        for row in &rows {
            if let Some(v) = verify_row(directory, node_key_id, &pipeline, row).await {
                out.push(v);
            }
        }
    }
    Ok(out)
}

/// What [`submit`] stored.
#[derive(Debug, Clone, Serialize)]
pub struct Accepted {
    /// The Contribution's row id.
    pub attestation_id: String,
    /// The manifest blob's address.
    pub manifest_sha256: String,
    /// The pipeline key that signed.
    pub pipeline_key_id: String,
    /// The trust root its `infra:attest` roots in.
    pub conferred_by: String,
    /// `false` when this exact Contribution was already held.
    pub newly_stored: bool,
}

/// The pipeline's credentials, carried beside a Contribution so a node that
/// has never heard of the pipeline can check it without a second round trip.
///
/// Both are self-authenticating and neither is trusted for arriving here: the
/// record is judged by persist's key admission, the grant by its signature
/// against the granter's key in THIS node's directory, and whether the granter
/// is a root this node accepts is the capability walk's question afterwards.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PipelineCredentials {
    /// The pipeline's key record.
    #[serde(default)]
    pub pipeline_record: Option<SignedKeyRecord>,
    /// The bless: `delegates_to(root → pipeline, infra:attest)`, signed by the
    /// root. This is what the accord's CI-key ceremony produces.
    #[serde(default)]
    pub pipeline_grant: Option<SignedContribution>,
}

/// Rebuild a signed row from the mirror its author signed, checking the
/// signature against the author's key in the directory. No re-stamp: a fresh
/// stamp would mint an id the signature does not cover.
async fn adopt_row(
    directory: &dyn FederationDirectory,
    signed: &SignedContribution,
) -> Result<Attestation, Refusal> {
    let core = EnvelopeCore::from_value(signed.signed_envelope.clone())
        .map_err(|_| Refusal::Malformed("envelope"))?;
    let mirror = core
        .row
        .clone()
        .ok_or(Refusal::Malformed("no signed `row` mirror (CIRISPersist#643)"))?;
    let author = directory
        .lookup_public_key(&mirror.attesting_key_id)
        .await
        .map_err(substrate)?
        .ok_or_else(|| Refusal::UnknownPipeline(mirror.attesting_key_id.clone()))?;
    let canonical = attestation_emit::canonicalize(&signed.signed_envelope).map_err(substrate)?;
    if !signature_verifies(
        &author,
        &canonical,
        &signed.ed25519_signature_base64,
        Some(&signed.mldsa65_signature_base64),
    ) {
        return Err(Refusal::SignatureInvalid);
    }
    let expires_at = match core.expires_at.as_deref() {
        Some(raw) => Some(
            chrono::DateTime::parse_from_rfc3339(raw)
                .map(|t| t.with_timezone(&chrono::Utc))
                .map_err(|_| Refusal::Malformed("expires_at"))?,
        ),
        None => None,
    };
    let mut input = EmitAttestationInput::with_envelope(
        mirror.attestation_type.clone(),
        core,
        mirror.cohort_scope.clone(),
    );
    input.attested_key_id = Some(mirror.attested_key_id.clone());
    input.subject_key_ids = mirror.subject_key_ids.clone();
    input.expires_at = expires_at;
    input.weight = mirror.weight.as_ref().and_then(serde_json::Number::as_f64);

    let decode = |s: &str| B64.decode(s).map_err(|_| Refusal::SignatureInvalid);
    let signature = ciris_crypto::HybridSignature {
        crypto_kind: ciris_crypto::CRYPTO_KIND_CIRIS_V1,
        classical: ciris_crypto::TaggedClassicalSignature {
            algorithm: ciris_crypto::ClassicalAlgorithm::Ed25519,
            signature: decode(&signed.ed25519_signature_base64)?,
            public_key: Vec::new(),
        },
        pqc: ciris_crypto::TaggedPqcSignature {
            algorithm: ciris_crypto::PqcAlgorithm::MlDsa65,
            signature: decode(&signed.mldsa65_signature_base64)?,
            public_key: Vec::new(),
        },
        mode: ciris_crypto::SignatureMode::HybridRequired,
    };
    let (row, _) = attestation_emit::assemble(author.key_id, &canonical, signature, input)
        .map_err(substrate)?;
    Ok(row)
}

/// Admit a Contribution and its manifest bytes.
///
/// Order is cheapest-first. The Contribution and its blob are written only
/// after every check has passed. The pipeline's credentials are written
/// before, because the checks read them: each is self-authenticating, and
/// storing a key record or a grant confers nothing that the capability walk
/// would not have refused anyway.
pub async fn submit(
    engine: &Arc<Engine>,
    node_key_id: &str,
    contribution: SignedContribution,
    manifest: &[u8],
    credentials: PipelineCredentials,
) -> Result<Accepted, Refusal> {
    if manifest.len() > MAX_MANIFEST_BYTES {
        return Err(Refusal::ManifestTooLarge(manifest.len()));
    }
    let facts = facts_of(&contribution.signed_envelope)?;
    // Size first, then the hash (CC 5.3.2.5).
    if manifest.len() as u64 != facts.manifest_size {
        return Err(Refusal::ManifestSizeMismatch {
            expected: facts.manifest_size,
            found: manifest.len() as u64,
        });
    }
    let sha: [u8; 32] = Sha256::digest(manifest).into();
    let found = hex::encode(sha);
    if found != facts.manifest_hash {
        return Err(Refusal::ManifestHashMismatch { expected: facts.manifest_hash, found });
    }

    // The columns come out of the signed envelope, never from the caller.
    let core = EnvelopeCore::from_value(contribution.signed_envelope.clone())
        .map_err(|_| Refusal::Malformed("envelope"))?;
    let mirror = core
        .row
        .clone()
        .ok_or(Refusal::Malformed("no signed `row` mirror (CIRISPersist#643)"))?;
    if mirror.attestation_type != attestation_type::SCORES {
        return Err(Refusal::Malformed("row.attestation_type is not `scores`"));
    }
    if mirror.cohort_scope != cohort_scope::FEDERATION {
        return Err(Refusal::Malformed("row.cohort_scope is not `federation`"));
    }

    let directory = engine.federation_directory();
    if let Some(record) = credentials.pipeline_record {
        if record.record.key_id != mirror.attesting_key_id {
            return Err(Refusal::Malformed("pipeline_record is for a different key"));
        }
        // Already registered is fine; anything else is the gate's verdict and
        // surfaces as an unknown or unblessed pipeline below.
        if let Err(e) = directory.put_public_key(record).await {
            tracing::debug!(pipeline = %mirror.attesting_key_id, "pipeline_record not stored: {e}");
        }
    }
    if let Some(grant) = credentials.pipeline_grant {
        let row = adopt_row(directory.as_ref(), &grant).await?;
        if row.attestation_type != attestation_type::DELEGATES_TO
            || row.attested_key_id != mirror.attesting_key_id
        {
            return Err(Refusal::Malformed("pipeline_grant is not a delegates_to naming this pipeline"));
        }
        // A grant already held is fine. One persist refuses is not fatal here
        // either: the pipeline may hold another, and the walk below decides.
        if let Err(e) = directory.put_attestation(SignedAttestation { attestation: row }).await {
            tracing::debug!(pipeline = %mirror.attesting_key_id, "pipeline_grant not stored: {e}");
        }
    }

    let row = adopt_row(directory.as_ref(), &contribution).await?;
    let pipeline_key_id = row.attesting_key_id.clone();
    let grant = blessing(directory.as_ref(), node_key_id, &pipeline_key_id)
        .await?
        .ok_or_else(|| Refusal::PipelineNotBlessed(pipeline_key_id.clone()))?;
    let attestation_id = row.attestation_id.clone();

    // Bytes first. A row whose blob is missing sends every peer that receives
    // it looking for bytes nobody holds; a blob with no row is merely unused.
    engine
        .put_blob_signing(
            &sha,
            BlobBody::Inline(manifest.to_vec()),
            Some("application/json"),
            &pipeline_key_id,
            chrono::Utc::now(),
            uuid::Uuid::new_v4(),
        )
        .await
        .map_err(substrate)?;
    let outcome = directory
        .put_attestation(SignedAttestation { attestation: row })
        .await
        .map_err(substrate)?;
    let newly_stored = format!("{outcome:?}").starts_with("Inserted");

    Ok(Accepted {
        attestation_id,
        manifest_sha256: facts.manifest_hash,
        pipeline_key_id,
        conferred_by: grant.root_key_id,
        newly_stored,
    })
}

/// The manifest bytes for `sha_hex`, if this node holds them.
pub async fn manifest_bytes(engine: &Engine, node_key_id: &str, sha_hex: &str) -> Option<Vec<u8>> {
    let sha: [u8; 32] = hex::decode(sha_hex).ok()?.try_into().ok()?;
    match engine.serve_blob_to_peer(&sha, node_key_id).await {
        Ok(BlobBody::Inline(bytes)) => Some(bytes),
        _ => None,
    }
}

// ───────────────────────────── HTTP ─────────────────────────────

/// State for the builds routes.
#[derive(Clone)]
pub struct BuildsState {
    engine: Arc<Engine>,
    node_key_id: String,
}

/// The builds router over the server's Engine.
///
/// Mounted by [`crate::fold::router`] only. The standalone binary still serves
/// `/v1/builds` from its Postgres build table, so these are not in
/// [`crate::fold::routes`], which both deployments share.
pub fn router(engine: Arc<Engine>, node_key_id: String) -> Router {
    Router::new()
        .route("/v1/builds", post(submit_build))
        .route("/v1/builds/{version}", get(build_by_version))
        .route("/v1/builds/hash/{manifest_hash}", get(build_by_hash))
        .route("/v1/builds/manifest/{manifest_hash}", get(manifest_by_hash))
        .with_state(BuildsState { engine, node_key_id })
}

#[derive(Deserialize)]
struct SubmitBody {
    contribution: SignedContribution,
    manifest_base64: String,
    #[serde(flatten)]
    credentials: PipelineCredentials,
}

async fn submit_build(State(st): State<BuildsState>, Json(body): Json<SubmitBody>) -> Response {
    let Ok(manifest) = B64.decode(body.manifest_base64.as_bytes()) else {
        return Refusal::Malformed("manifest_base64").into_response();
    };
    match submit(&st.engine, &st.node_key_id, body.contribution, &manifest, body.credentials)
        .await
    {
        Ok(accepted) => {
            tracing::info!(
                attestation = %accepted.attestation_id,
                pipeline = %accepted.pipeline_key_id,
                manifest = %accepted.manifest_sha256,
                "build manifest Contribution admitted"
            );
            (StatusCode::CREATED, Json(accepted)).into_response()
        }
        Err(refusal) => {
            tracing::warn!(reason = refusal.token(), "build manifest Contribution refused: {refusal}");
            refusal.into_response()
        }
    }
}

#[derive(Serialize)]
struct ProvenanceEntry {
    dimension: String,
    score: f64,
    confidence: f64,
    attester_key_id: String,
    evidence_summary: String,
}

#[derive(Serialize)]
struct ProvenanceBlock {
    attestations_consumed: Vec<ProvenanceEntry>,
    note: String,
}

/// A build, in the field names `GET /v1/builds/{version}` has always used.
#[derive(Serialize)]
struct BuildResponse {
    build_id: String,
    version: String,
    target: String,
    build_hash: String,
    file_manifest_hash: String,
    /// The manifest, parsed, when this node holds the bytes and they are JSON.
    file_manifest_json: Option<serde_json::Value>,
    /// Whether this node holds the manifest bytes. `false` means the
    /// Contribution arrived and the blob has not: ask again, or ask a holder.
    manifest_held: bool,
    /// The declared size of the manifest, from the signed facts.
    manifest_size: u64,
    federation_provenance: ProvenanceBlock,
    /// The Contribution as the pipeline signed it. A reader re-verifies this
    /// against the pipeline's key and does not have to take the fields above
    /// on this node's word (CC 5.3.4).
    contribution: SignedContribution,
    /// Whose walk said the pipeline has standing, and what it found. The
    /// walk is this node's; a reader that wants its own fetches the grant.
    standing: Standing,
}

#[derive(Serialize)]
struct Standing {
    scope: &'static str,
    root_key_id: String,
    grant_attestation_id: String,
    walked_by: String,
}

async fn respond(st: &BuildsState, build: VerifiedBuild) -> Response {
    let bytes = manifest_bytes(&st.engine, &st.node_key_id, &build.facts.manifest_hash).await;
    let body = BuildResponse {
        build_id: build.facts.build_id.clone(),
        version: build.facts.binary_version.clone(),
        target: build.facts.target.clone(),
        build_hash: build.facts.binary_hash.clone(),
        file_manifest_hash: build.facts.manifest_hash.clone(),
        manifest_held: bytes.is_some(),
        manifest_size: build.facts.manifest_size,
        contribution: build.contribution.clone(),
        standing: Standing {
            scope: INFRA_ATTEST_SCOPE,
            root_key_id: build.conferred_by.clone(),
            grant_attestation_id: build.grant_attestation_id.clone(),
            walked_by: st.node_key_id.clone(),
        },
        file_manifest_json: bytes.and_then(|b| serde_json::from_slice(&b).ok()),
        federation_provenance: ProvenanceBlock {
            attestations_consumed: vec![ProvenanceEntry {
                dimension: build_manifest_dimension(&build.facts.target),
                score: 1.0,
                confidence: 1.0,
                attester_key_id: build.pipeline_key_id.clone(),
                evidence_summary: format!(
                    "pipeline={} conferred_by={} manifest_hash={} attestation_id={}",
                    build.pipeline_key_id,
                    build.conferred_by,
                    build.facts.manifest_hash,
                    build.attestation_id
                ),
            }],
            note: "pipeline-signed Contribution, re-verified on this read; the pipeline holds \
                   infra:attest from a trust root this node accepts"
                .to_string(),
        },
    };
    Json(body).into_response()
}

fn not_found(what: &str) -> Response {
    let body = serde_json::json!({
        "error": "build_not_found",
        "detail": format!("no verified build-manifest Contribution for {what}"),
    });
    (StatusCode::NOT_FOUND, Json(body)).into_response()
}

/// Strip a trailing `-stable` only (#137): the publisher's own equivalence.
fn normalize_version(v: &str) -> &str {
    v.strip_suffix("-stable").unwrap_or(v)
}

#[derive(Deserialize)]
struct TargetQuery {
    target: Option<String>,
}

async fn build_by_version(
    State(st): State<BuildsState>,
    Path(version): Path<String>,
    Query(q): Query<TargetQuery>,
) -> Response {
    let directory = st.engine.federation_directory();
    let builds = match verified_builds(directory.as_ref(), &st.node_key_id).await {
        Ok(b) => b,
        Err(e) => return e.into_response(),
    };
    let want = normalize_version(&version);
    let hit = builds.into_iter().find(|b| {
        normalize_version(&b.facts.binary_version) == want
            && q.target.as_deref().is_none_or(|t| t == b.facts.target)
    });
    match hit {
        Some(build) => respond(&st, build).await,
        None => not_found(&format!("version {version}")),
    }
}

async fn build_by_hash(State(st): State<BuildsState>, Path(hash): Path<String>) -> Response {
    let directory = st.engine.federation_directory();
    let builds = match verified_builds(directory.as_ref(), &st.node_key_id).await {
        Ok(b) => b,
        Err(e) => return e.into_response(),
    };
    match builds.into_iter().find(|b| b.facts.manifest_hash == hash) {
        Some(build) => respond(&st, build).await,
        None => not_found(&format!("manifest {hash}")),
    }
}

/// The raw manifest bytes, but only for a manifest a valid Contribution names.
/// The blob store holds other commons content; this route is not a way to read
/// it by guessing hashes.
async fn manifest_by_hash(State(st): State<BuildsState>, Path(hash): Path<String>) -> Response {
    let directory = st.engine.federation_directory();
    let attested = match verified_builds(directory.as_ref(), &st.node_key_id).await {
        Ok(b) => b.into_iter().any(|b| b.facts.manifest_hash == hash),
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
