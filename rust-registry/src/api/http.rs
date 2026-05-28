//! HTTP/REST gateway for health, metrics, and public verification endpoints

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use tower_http::limit::RequestBodyLimitLayer;
use base64::Engine;
use metrics_exporter_prometheus::PrometheusHandle;
use serde::Serialize;
use tracing::{info, warn};

use crate::crypto::HybridCrypto;
use crate::db::{self, BuildRow, Database, get_build};
use crate::app_attest::{
    AppAttestAssertRequest, AppAttestConfig, AppAttestService, AppAttestVerifyRequest,
};
use crate::play_integrity::{
    IntegrityAuthRequest, IntegrityAuthResponse, IntegrityVerifyRequest, PlayIntegrityConfig,
    PlayIntegrityService,
};
use crate::rate_limiter::{
    check_nonce_rate_limit, check_verify_rate_limit, check_assertion_cache,
    cache_assertion_result, is_already_attested, AssertionCacheResult,
};

/// Global start time for uptime tracking
static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

/// Initialize start time (call once at startup)
pub fn init_start_time() {
    START_TIME.get_or_init(Instant::now);
}

/// Get uptime in seconds
pub fn get_uptime_seconds() -> u64 {
    START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0)
}

#[derive(Clone)]
struct AppState {
    db: Database,
    crypto: Arc<HybridCrypto>,
    metrics_handle: Arc<PrometheusHandle>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    database_healthy: bool,
    uptime_seconds: u64,
}

#[derive(Serialize)]
struct ReadinessResponse {
    ready: bool,
}

/// Response from the /v1/steward-key endpoint (v1.4 multi-steward shape per
/// FSD-002 §7.7). CIRISVerify v3.1.0+ consumer wiring at
/// `ciris-verify-core::ThresholdMember` + `verify_threshold_signatures`
/// per CIRISRegistry#21.
#[derive(Serialize)]
struct StewardKeyResponse {
    stewards: Vec<StewardEntry>,
    verification_policy: VerificationPolicy,
    rotation_history_uri: String,
    signature_mode: String,
    revision: u64,
    timestamp: i64,
}

/// One steward entry in the multi-steward response. Non-deployed stewards
/// (e.g., APAC during the rollout window) carry `deployed: false` +
/// `hardware_class: placeholder_pending_provisioning` + null pubkeys.
/// Verify-side `ThresholdMember` construction filters by `deployed=true`.
#[derive(Serialize)]
struct StewardEntry {
    region: String,
    key_id: String,
    /// Base64 Ed25519 pubkey, null for non-deployed stewards.
    classical_pubkey: Option<String>,
    /// Base64 ML-DSA-65 pubkey, null for non-deployed stewards.
    pqc_pubkey: Option<String>,
    /// sha256:... fingerprint of ML-DSA-65 pubkey, null for non-deployed stewards.
    fingerprint: Option<String>,
    hardware_class: String,
    deployed: bool,
}

#[derive(Serialize)]
struct VerificationPolicy {
    threshold: u32,
    of_total: u32,
    scheme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_revocable: Option<bool>,
}

#[derive(Serialize)]
struct StewardKeyError {
    error: String,
}

/// Response from the /v1/accord-holders endpoint (FSD-002 §7.7).
/// v1.4 interim: placeholder fingerprints; provisioned=false signals
/// to consumers that CONSTITUTIONAL invocations MUST NOT be honored
/// until real hardware-attested keys land.
#[derive(Serialize)]
struct AccordHoldersResponse {
    holders: Vec<AccordHolderEntry>,
    verification_policy: VerificationPolicy,
    constitutional_anchor: bool,
    rotation_history_uri: String,
    timestamp: i64,
}

#[derive(Serialize, Clone)]
struct AccordHolderEntry {
    identity_ref: String,
    classical_pubkey: String,
    pqc_pubkey: String,
    fingerprint: String,
    hardware_class: String,
    provisioned: bool,
}

/// Response from the /v1/accord/holders UI wrapper (CIRISRegistry#23 Surface 1).
/// Adds per-holder accord_emissions[] joined from accord:* attestations.
/// v1.4 interim: accord_emissions empty for all holders until substrate-
/// conformance migration lands and accord-holders start emitting.
#[derive(Serialize)]
struct AccordHoldersUiResponse {
    holders: Vec<AccordHolderUiEntry>,
    timestamp: i64,
}

#[derive(Serialize)]
struct AccordHolderUiEntry {
    key_id: String,
    identity_ref: String,
    fingerprint: String,
    hardware_class: String,
    provisioned: bool,
    registered_at: Option<i64>,
    accord_emissions: Vec<serde_json::Value>,
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

/// Response from /v1/partner/{key_id} (CIRISRegistry#23 Surface 3).
/// Composes from partners + revocations + builds tables for the
/// CIRISAgent 2.10.0 ProfileScorecard.
#[derive(Serialize)]
struct PartnerCompositionResponse {
    key_id: String,
    partner_role: Option<String>,
    bond_posted: Option<BondInfo>,
    licensure: Vec<LicensureEntry>,
    revocation_active: bool,
    revocation_reason: Option<String>,
    timestamp: i64,
}

#[derive(Serialize)]
struct BondInfo {
    currency: String,
    amount: String,
    forfeited: bool,
}

#[derive(Serialize)]
struct LicensureEntry {
    authority_id: String,
    status: String,
    expires_at: Option<i64>,
}

/// Response from /v1/rotation-history (FSD-002 §7.7 audit endpoint).
/// v1.4 interim: empty events; pre-migration history available via
/// /v1/audit-log queries on registry_signing_keys table.
#[derive(Serialize)]
struct RotationHistoryResponse {
    events: Vec<serde_json::Value>,
    note: String,
    timestamp: i64,
}

/// FSD-002 §7.3.1 hardware_class taxonomy — placeholder value for v1.4 interim.
const HARDWARE_CLASS_PLACEHOLDER: &str = "placeholder_pending_provisioning";

/// FSD-002 §10.5 — production stewards run on FIPS 140-3 L3 HSMs.
const HARDWARE_CLASS_HSM_PROD: &str = "HSM_FIPS_140_3_L3";

/// FSD-002 §10.4 + MISSION §2.1 — the three named accord-holders.
const ACCORD_HOLDERS: &[(&str, &str)] = &[
    ("accord_holder_eric_moore",    "eric-moore"),
    ("accord_holder_eric_kudzin",   "eric-kudzin"),
    ("accord_holder_haley_bradley", "haley-bradley"),
];

/// FSD-002 §2.1 — three regional stewards per the multi-party arc.
/// "us" + "eu" deployed today; "apac" is Spec per MISSION §2.1.
const STEWARD_REGIONS: &[(&str, &str)] = &[
    ("us",   "registry-steward-us"),
    ("eu",   "registry-steward-eu"),
    ("apac", "registry-steward-apac"),
];

/// Deterministic placeholder fingerprint for non-provisioned accord-holders
/// and non-deployed stewards. Lets the endpoint shape be structurally live
/// for downstream consumer wiring while clearly signaling that the underlying
/// key material is not yet hardware-attested. Format `sha256:placeholder_<sha>`
/// with the "placeholder_" prefix encoded in-band so consumers can pattern-match.
fn placeholder_fingerprint(identity_ref: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"placeholder:");
    hasher.update(identity_ref.as_bytes());
    format!("sha256:placeholder_{}", hex::encode(hasher.finalize()))
}

/// Deterministic placeholder Ed25519 pubkey (32 bytes, base64) for v1.4 interim.
fn placeholder_ed25519_pubkey(identity_ref: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"placeholder_ed25519:");
    hasher.update(identity_ref.as_bytes());
    let b64 = base64::engine::general_purpose::STANDARD;
    b64.encode(hasher.finalize())
}

/// Deterministic placeholder ML-DSA-65 pubkey (1952 bytes; here just a base64
/// scalar marker because real ML-DSA-65 pubkeys are too large for trivial
/// placeholders — consumers MUST treat any pubkey from a non-provisioned
/// holder as non-verifiable regardless).
fn placeholder_mldsa_pubkey(identity_ref: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"placeholder_mldsa65:");
    hasher.update(identity_ref.as_bytes());
    let b64 = base64::engine::general_purpose::STANDARD;
    b64.encode(hasher.finalize())
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let db_healthy = state.db.health_check().await.unwrap_or(false);

    Json(HealthResponse {
        status: if db_healthy { "healthy" } else { "unhealthy" }.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database_healthy: db_healthy,
        uptime_seconds: get_uptime_seconds(),
    })
}

async fn readiness(State(state): State<AppState>) -> (StatusCode, Json<ReadinessResponse>) {
    let db_healthy = state.db.health_check().await.unwrap_or(false);

    if db_healthy {
        (StatusCode::OK, Json(ReadinessResponse { ready: true }))
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ReadinessResponse { ready: false }),
        )
    }
}

async fn liveness() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

async fn metrics(State(state): State<AppState>) -> String {
    // Export Prometheus metrics from the global recorder
    let mut output = state.metrics_handle.render();

    // Add version info metric
    output.push_str("# HELP ciris_registry_info Registry version info\n");
    output.push_str("# TYPE ciris_registry_info gauge\n");
    output.push_str(&format!(
        "ciris_registry_info{{version=\"{}\"}} 1\n",
        env!("CARGO_PKG_VERSION")
    ));

    // Add uptime metric
    output.push_str("# HELP ciris_registry_uptime_seconds Time since registry started\n");
    output.push_str("# TYPE ciris_registry_uptime_seconds gauge\n");
    output.push_str(&format!(
        "ciris_registry_uptime_seconds {}\n",
        get_uptime_seconds()
    ));

    output
}

/// Public endpoint: GET /v1/steward-key
///
/// Returns the multi-steward set per FSD-002 §7.7. This instance's region
/// (derived from REGISTRY_REGION env var; defaults to "us") has its actual
/// crypto pubkeys; other regions return placeholder pubkeys + `deployed:
/// false` until their respective Registry instances ship. CIRISVerify v3.1.0+
/// `ThresholdMember` consumer filters by `deployed=true`.
///
/// Closes CIRISRegistry#21 Ask 1 (v1.4 multi-steward shape change).
async fn steward_key(
    State(state): State<AppState>,
) -> Result<Json<StewardKeyResponse>, (StatusCode, Json<StewardKeyError>)> {
    let ed25519_pubkey = state.crypto.ed25519_public_key();
    let mldsa_pubkey = state.crypto.mldsa_public_key();
    let key_id = state.crypto.key_id().to_string();

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&mldsa_pubkey);
    let mldsa_fingerprint = format!("sha256:{}", hex::encode(hasher.finalize()));

    let revision: u64 = sqlx::query_scalar::<_, Option<i32>>("SELECT MAX(id) FROM revocations")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(Some(0))
        .unwrap_or(0) as u64;

    let b64 = base64::engine::general_purpose::STANDARD;
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    let this_region = std::env::var("REGISTRY_REGION").unwrap_or_else(|_| "us".to_string());
    let mut stewards: Vec<StewardEntry> = Vec::with_capacity(STEWARD_REGIONS.len());
    for (region, region_key_id) in STEWARD_REGIONS {
        if *region == this_region {
            stewards.push(StewardEntry {
                region: region.to_string(),
                key_id: key_id.clone(),
                classical_pubkey: Some(b64.encode(&ed25519_pubkey)),
                pqc_pubkey: Some(b64.encode(&mldsa_pubkey)),
                fingerprint: Some(mldsa_fingerprint.clone()),
                hardware_class: HARDWARE_CLASS_HSM_PROD.to_string(),
                deployed: true,
            });
        } else {
            // Other regions: placeholder until their Registry instance ships.
            // Per FSD-002 §11.2 v1.4 interim — endpoint shape live so Verify-side
            // ThresholdMember construction works end-to-end; downstream filters
            // by deployed=true. APAC is Spec per MISSION §2.1; EU+US are
            // Deployed but each instance only knows its own crypto today
            // (pre-substrate-conformance #17). Placeholder for the non-this
            // entries lets v1.4 ship before #17 lands.
            stewards.push(StewardEntry {
                region: region.to_string(),
                key_id: region_key_id.to_string(),
                classical_pubkey: None,
                pqc_pubkey: None,
                fingerprint: None,
                hardware_class: HARDWARE_CLASS_PLACEHOLDER.to_string(),
                deployed: false,
            });
        }
    }

    Ok(Json(StewardKeyResponse {
        stewards,
        verification_policy: VerificationPolicy {
            threshold: 2,
            of_total: 3,
            scheme: "M-of-N hybrid Ed25519 + ML-DSA-65".to_string(),
            non_revocable: None,
        },
        rotation_history_uri: "/v1/rotation-history".to_string(),
        signature_mode: "HYBRID_REQUIRED".to_string(),
        revision,
        timestamp: now,
    }))
}

/// Public endpoint: GET /v1/accord-holders (FSD-002 §7.7)
///
/// Returns the three named accord-holders with their key material per
/// §10.4 + MISSION §2.1. v1.4 interim ships placeholder fingerprints +
/// `provisioned: false` — consumers MUST NOT honor CONSTITUTIONAL invocations
/// against these placeholders. Endpoint shape is live so Verify-side
/// `ThresholdMember` wiring works end-to-end before hardware-attestation
/// provisioning completes.
///
/// Closes CIRISRegistry#21 Ask 2.
async fn accord_holders(
    State(_state): State<AppState>,
) -> Result<Json<AccordHoldersResponse>, (StatusCode, Json<StewardKeyError>)> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    let holders: Vec<AccordHolderEntry> = ACCORD_HOLDERS
        .iter()
        .map(|(_, identity_ref)| AccordHolderEntry {
            identity_ref: identity_ref.to_string(),
            classical_pubkey: placeholder_ed25519_pubkey(identity_ref),
            pqc_pubkey: placeholder_mldsa_pubkey(identity_ref),
            fingerprint: placeholder_fingerprint(identity_ref),
            hardware_class: HARDWARE_CLASS_PLACEHOLDER.to_string(),
            provisioned: false,
        })
        .collect();

    Ok(Json(AccordHoldersResponse {
        holders,
        verification_policy: VerificationPolicy {
            threshold: 2,
            of_total: 3,
            scheme: "M-of-N hybrid Ed25519 + ML-DSA-65".to_string(),
            non_revocable: Some(true),
        },
        constitutional_anchor: true,
        rotation_history_uri: "/v1/rotation-history".to_string(),
        timestamp: now,
    }))
}

/// Public endpoint: GET /v1/accord/holders (UI wrapper per CIRISRegistry#23 Surface 1)
///
/// UI-shaped wrapper around /v1/accord-holders. Adds per-holder
/// `accord_emissions[]` — accord:* attestations the holder has signed.
/// v1.4 interim: emissions list is empty until substrate-conformance
/// migration (#17) lands and accord-holders start emitting.
async fn accord_holders_ui(
    State(_state): State<AppState>,
) -> Result<Json<AccordHoldersUiResponse>, (StatusCode, Json<StewardKeyError>)> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    let holders: Vec<AccordHolderUiEntry> = ACCORD_HOLDERS
        .iter()
        .map(|(key_id, identity_ref)| AccordHolderUiEntry {
            key_id: key_id.to_string(),
            identity_ref: identity_ref.to_string(),
            fingerprint: placeholder_fingerprint(identity_ref),
            hardware_class: HARDWARE_CLASS_PLACEHOLDER.to_string(),
            provisioned: false,
            registered_at: None,
            accord_emissions: Vec::new(),
        })
        .collect();

    Ok(Json(AccordHoldersUiResponse {
        holders,
        timestamp: now,
    }))
}

/// Public endpoint: GET /v1/agent_files/{kind}?platform_or_target=...
/// (CIRISRegistry#23 Surface 2 + #18)
///
/// Three-layer trust composition per FSD-002 §6.1.6:
/// - Layer 1 Canonical: registry-steward-triple attestations on `agent_files:*`
/// - Layer 2 Open: any federation-key holder may emit
/// - Layer 3 Vote-then-trust: NodeCore P4 vote accumulation
///
/// v1.4 interim: returns empty lists until the substrate trio
/// (Edge#21 + Persist#103 + NodeCore#11) ships and federation_attestations
/// table starts carrying agent_files:* claims. The endpoint shape is live
/// so CIRISAgent 2.10.0 UI wiring works end-to-end.
async fn agent_files_for_kind(
    State(_state): State<AppState>,
    axum::extract::Path(kind): axum::extract::Path<String>,
    axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<AgentFilesResponse>, (StatusCode, Json<StewardKeyError>)> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let platform_or_target = q.get("platform_or_target").cloned();

    Ok(Json(AgentFilesResponse {
        kind,
        platform_or_target,
        canonical_attester: None,
        open_attesters: Vec::new(),
        vote_then_trust: Vec::new(),
        anti_trick_guarantee: "Canonical attester (registry-steward-triple, score >= 0.7) determines /install endpoint default. Third-party agent_files reachable only via explicit 'Browse alternatives' informed-consent path. Anti-tricking per CIRISRegistry#18 + FSD-002 §6.1.6.".to_string(),
        timestamp: now,
    }))
}

/// Public endpoint: GET /v1/partner/{key_id} (CIRISRegistry#23 Surface 3)
///
/// Per-agent partner status badge composed from four prefix families:
/// partner_role, bond_posted, licensure, revocation_active. Backs the
/// CIRISAgent 2.10.0 ProfileScorecard UI surface.
async fn partner_composition(
    State(state): State<AppState>,
    axum::extract::Path(key_id): axum::extract::Path<String>,
) -> Result<Json<PartnerCompositionResponse>, (StatusCode, Json<StewardKeyError>)> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    let partner_row = db::lookup_partner(state.db.pool(), &key_id).await.ok().flatten();

    let (partner_role, bond_posted, revocation_active, revocation_reason) = match &partner_row {
        Some(row) => {
            // Map license_type integer to LICENSE_TYPE string per proto.
            // Status integer is enum (1=ACTIVE, 2=SUSPENDED, 3=REVOKED per common shape).
            let role = match row.license_type {
                1 => Some("COMMUNITY".to_string()),
                2 => Some("COMMUNITY_PLUS".to_string()),
                3 => Some("PROFESSIONAL_MEDICAL".to_string()),
                4 => Some("PROFESSIONAL_LEGAL".to_string()),
                5 => Some("PROFESSIONAL_FINANCIAL".to_string()),
                6 => Some("PROFESSIONAL_FULL".to_string()),
                _ => None,
            };
            let revoked = row.status == 3 || row.revocation_reason.is_some();
            (
                role,
                None,
                revoked,
                row.revocation_reason.clone(),
            )
        }
        None => (None, None, false, None),
    };

    let licensure: Vec<LicensureEntry> = match &partner_row {
        Some(row) => vec![LicensureEntry {
            authority_id: row.organization_id.clone(),
            status: if row.status == 1 { "active".to_string() } else { "inactive".to_string() },
            expires_at: Some(row.expires_at.unix_timestamp()),
        }],
        None => Vec::new(),
    };

    Ok(Json(PartnerCompositionResponse {
        key_id,
        partner_role,
        bond_posted,
        licensure,
        revocation_active,
        revocation_reason,
        timestamp: now,
    }))
}

/// Public endpoint: GET /v1/rotation-history (FSD-002 §7.7 audit endpoint)
///
/// Chronological rotation events for stewards and accord-holders.
/// v1.4 interim: empty events; pre-migration history is queryable via
/// /v1/audit-log against registry_signing_keys table.
async fn rotation_history(
    State(_state): State<AppState>,
) -> Result<Json<RotationHistoryResponse>, (StatusCode, Json<StewardKeyError>)> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    Ok(Json(RotationHistoryResponse {
        events: Vec::new(),
        note: "rotation_history seeded at substrate-conformance migration; pre-migration history available via /v1/audit-log queries on registry_signing_keys table".to_string(),
        timestamp: now,
    }))
}

/// Public endpoint: GET /v1/revocation/{license_id}
///
/// Check if a specific license/entity has been revoked.
/// This endpoint is unauthenticated.
async fn check_revocation(
    State(state): State<AppState>,
    axum::extract::Path(target_id): axum::extract::Path<String>,
) -> Json<RevocationCheckResponse> {
    let revoked = sqlx::query_as::<_, RevocationHit>(
        "SELECT id, revoked_at, reason_detail FROM revocations WHERE target_id = $1 ORDER BY id DESC LIMIT 1",
    )
    .bind(&target_id)
    .fetch_optional(state.db.pool())
    .await
    .ok()
    .flatten();

    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    match revoked {
        Some(hit) => Json(RevocationCheckResponse {
            license_id: target_id,
            revoked: true,
            revoked_at: Some(hit.revoked_at.unix_timestamp()),
            reason: hit.reason_detail,
            checked_at: now,
        }),
        None => Json(RevocationCheckResponse {
            license_id: target_id,
            revoked: false,
            revoked_at: None,
            reason: None,
            checked_at: now,
        }),
    }
}

#[derive(Serialize)]
struct RevocationCheckResponse {
    license_id: String,
    revoked: bool,
    revoked_at: Option<i64>,
    reason: Option<String>,
    checked_at: i64,
}

#[derive(sqlx::FromRow)]
struct RevocationHit {
    #[allow(dead_code)]
    id: i32,
    revoked_at: time::OffsetDateTime,
    reason_detail: Option<String>,
}

/// Response for binary manifest not found
#[derive(Serialize)]
struct BinaryManifestNotFound {
    error: String,
    message: String,
}

/// Request to register a binary manifest (from CI)
#[derive(serde::Deserialize)]
struct RegisterBinaryManifestRequest {
    version: String,
    binaries: std::collections::HashMap<String, String>,
    generated_at: String,
    #[serde(default)]
    notes: Option<String>,
    /// CIRIS primitive name (kebab-case). Empty/missing → "ciris-agent".
    /// Added v1.4.0 for federation peer support.
    #[serde(default)]
    project: String,
}

/// Response for registering a binary manifest
#[derive(Serialize)]
struct RegisterBinaryManifestResponse {
    success: bool,
    manifest_id: String,
    message: String,
}

/// Error response for register endpoint
#[derive(Serialize)]
struct RegisterBinaryManifestError {
    error: String,
    message: String,
}

/// Optional `?project=` query string for project-aware lookups.
/// Empty/missing → "ciris-agent" (backwards compat).
#[derive(serde::Deserialize, Default)]
struct ProjectQuery {
    #[serde(default)]
    project: String,
}

impl ProjectQuery {
    fn as_opt(&self) -> Option<&str> {
        if self.project.is_empty() { None } else { Some(self.project.as_str()) }
    }

    fn validate(&self) -> Result<(), String> {
        crate::validation::validate_project_name(&self.project)
    }
}

/// `?project=&target=` for `GET /v1/builds/{version}`. Empty `target` →
/// `python-source-tree` (canonical byte-identical-across-platforms source
/// manifest). Closes CIRISRegistry#11.
#[derive(serde::Deserialize, Default)]
struct BuildVersionQuery {
    #[serde(default)]
    project: String,
    #[serde(default)]
    target: String,
}

impl BuildVersionQuery {
    fn project_opt(&self) -> Option<&str> {
        if self.project.is_empty() { None } else { Some(self.project.as_str()) }
    }
    fn target_opt(&self) -> Option<&str> {
        if self.target.is_empty() { None } else { Some(self.target.as_str()) }
    }
    fn validate(&self) -> Result<(), String> {
        crate::validation::validate_project_name(&self.project)
    }
}

/// Public endpoint: GET /v1/verify/binary-manifest/{version}?project=...
///
/// Returns SHA-256 hashes of CIRISVerify binaries for self-verification (Level 2).
/// This endpoint is unauthenticated.
async fn binary_manifest(
    State(state): State<AppState>,
    axum::extract::Path(version): axum::extract::Path<String>,
    axum::extract::Query(q): axum::extract::Query<ProjectQuery>,
) -> Result<Json<db::BinaryManifestResponse>, (StatusCode, Json<BinaryManifestNotFound>)> {
    // Validate version format (basic semver check)
    if version.is_empty() || !version.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(BinaryManifestNotFound {
                error: "bad_request".to_string(),
                message: "Invalid version format".to_string(),
            }),
        ));
    }

    if let Err(reason) = q.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(BinaryManifestNotFound {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    match db::get_binary_manifest(state.db.pool(), &version, q.as_opt()).await {
        Ok(Some(manifest)) => Ok(Json(manifest.to_response())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(BinaryManifestNotFound {
                error: "not_found".to_string(),
                message: format!("Binary manifest not found for version {}", version),
            }),
        )),
        Err(e) => {
            warn!("Error fetching binary manifest: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BinaryManifestNotFound {
                    error: "internal_error".to_string(),
                    message: "Failed to fetch binary manifest".to_string(),
                }),
            ))
        }
    }
}

/// Admin endpoint: POST /v1/verify/binary-manifest
///
/// Register a new binary manifest from CI. Requires REGISTRY_ADMIN_TOKEN.
async fn register_binary_manifest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RegisterBinaryManifestRequest>,
) -> Result<Json<RegisterBinaryManifestResponse>, (StatusCode, Json<RegisterBinaryManifestError>)> {
    // Check authorization
    let admin_token = std::env::var("REGISTRY_ADMIN_TOKEN").unwrap_or_default();
    if admin_token.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RegisterBinaryManifestError {
                error: "configuration_error".to_string(),
                message: "REGISTRY_ADMIN_TOKEN not configured".to_string(),
            }),
        ));
    }

    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let provided_token = auth_header
        .strip_prefix("Bearer ")
        .unwrap_or(auth_header);

    if provided_token != admin_token {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(RegisterBinaryManifestError {
                error: "unauthorized".to_string(),
                message: "Invalid or missing authorization token".to_string(),
            }),
        ));
    }

    // Validate project name (empty → "ciris-agent")
    if let Err(reason) = crate::validation::validate_project_name(&req.project) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(RegisterBinaryManifestError {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    // Parse generated_at timestamp
    let generated_at = time::OffsetDateTime::parse(
        &req.generated_at,
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap_or_else(|_| time::OffsetDateTime::now_utc());

    // Convert binaries map to JSON
    let binaries_json = serde_json::to_value(&req.binaries).unwrap_or(serde_json::json!({}));

    // Sign the manifest content with steward key (registry-side signing)
    // Canonical representation includes project so signatures don't collide
    // across project namespaces with the same version string.
    let project_for_sig = if req.project.is_empty() { "ciris-agent" } else { req.project.as_str() };
    let canonical_content = format!("{}/{}:{}", project_for_sig, req.version, binaries_json);
    let (sig_classical, sig_pqc, sig_key_id) = match state.crypto.sign(canonical_content.as_bytes())
    {
        Ok(sig) => {
            use base64::{engine::general_purpose::STANDARD, Engine};
            (
                Some(STANDARD.encode(&sig.classical_signature)),
                Some(STANDARD.encode(&sig.post_quantum_signature)),
                Some(state.crypto.key_id().to_string()),
            )
        }
        Err(e) => {
            warn!("Failed to sign binary manifest: {}", e);
            (None, None, None)
        }
    };

    // Register the manifest with signature
    match db::register_binary_manifest(
        state.db.pool(),
        &req.project,
        &req.version,
        &binaries_json,
        generated_at,
        Some("ci_push"),
        Some("ci_push"),
        req.notes.as_deref(),
        sig_classical.as_deref(),
        sig_pqc.as_deref(),
        sig_key_id.as_deref(),
    )
    .await
    {
        Ok(manifest_id) => {
            info!(
                "Binary manifest registered: project={}, version={}, signed={}",
                project_for_sig,
                req.version,
                sig_key_id.is_some()
            );
            Ok(Json(RegisterBinaryManifestResponse {
                success: true,
                manifest_id,
                message: format!(
                    "Binary manifest registered for version {} (signed={})",
                    req.version,
                    sig_key_id.is_some()
                ),
            }))
        }
        Err(e) => {
            warn!("Error registering binary manifest: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterBinaryManifestError {
                    error: "internal_error".to_string(),
                    message: "Failed to register binary manifest".to_string(),
                }),
            ))
        }
    }
}

// =============================================================================
// Function Manifests (Function-level integrity verification)
// =============================================================================

/// Request to register a function manifest (from CI)
#[derive(serde::Deserialize)]
struct RegisterFunctionManifestRequest {
    version: String,
    target: String,
    binary_hash: String,
    binary_version: String,
    generated_at: String,
    functions: serde_json::Value,
    manifest_hash: String,
    #[serde(default)]
    signature: Option<FunctionManifestSignatureRequest>,
    /// Metadata containing text_section_offset for address calculation
    #[serde(default)]
    metadata: Option<serde_json::Value>,
    /// CIRIS primitive name (kebab-case). Empty/missing → "ciris-agent".
    /// Added v1.4.0 for federation peer support.
    #[serde(default)]
    project: String,
}

#[derive(serde::Deserialize)]
struct FunctionManifestSignatureRequest {
    classical: String,
    #[serde(default)]
    classical_algorithm: Option<String>,
    pqc: String,
    #[serde(default)]
    pqc_algorithm: Option<String>,
    key_id: String,
}

/// Response for function manifest errors
#[derive(Serialize)]
struct FunctionManifestError {
    error: String,
    message: String,
}

/// Response for registering a function manifest
#[derive(Serialize)]
struct RegisterFunctionManifestResponse {
    success: bool,
    manifest_hash: String,
    message: String,
}

// =============================================================================
// Build Records (CIRISAgent file integrity manifests)
// =============================================================================

/// Build record response for CIRISVerify file verification.
/// Matches the BuildRecord struct expected by CIRISVerify.
#[derive(Serialize)]
struct BuildRecordResponse {
    build_id: String,
    version: String,
    /// Build target (`python-source-tree`, `ios-mobile-bundle`, …). v1.4.1+.
    /// CIRISVerify consumers should branch on this to confirm they're
    /// validating against the manifest for their platform. Closes #11.
    target: String,
    build_hash: String,
    file_manifest_hash: String,
    file_manifest_count: i32,
    file_manifest_json: serde_json::Value,
    includes_modules: Vec<String>,
    source_repo: Option<String>,
    source_commit: Option<String>,
    registered_at: i64,
    status: String,
}

impl From<BuildRow> for BuildRecordResponse {
    fn from(row: BuildRow) -> Self {
        // CIRISVerify expects: {"version": "...", "files": {"path": "hash"}}
        // Database stores flat: {"path": "hash"}
        // Wrap if needed for compatibility
        let file_manifest_json = if row.file_manifest_json.get("files").is_some() {
            // Already in correct format
            row.file_manifest_json
        } else {
            // Wrap flat manifest in expected structure
            serde_json::json!({
                "version": row.version,
                "files": row.file_manifest_json
            })
        };

        Self {
            build_id: row.build_id.to_string(),
            version: row.version.clone(),
            target: row.target,
            build_hash: row.build_hash,
            file_manifest_hash: row.file_manifest_hash,
            file_manifest_count: row.file_manifest_count,
            file_manifest_json,
            includes_modules: row.includes_modules,
            source_repo: row.source_repo,
            source_commit: row.source_commit,
            registered_at: row.registered_at.unix_timestamp(),
            status: row.status,
        }
    }
}

#[derive(Serialize)]
struct BuildNotFound {
    error: String,
    message: String,
}

/// Public endpoint: GET /v1/builds/{version}?project=...
///
/// Returns a build record by version. Used by CIRISVerify for file integrity verification.
/// `?project=` is optional; defaults to "ciris-agent" for backwards compat.
/// This endpoint is unauthenticated.
async fn get_build_by_version(
    State(state): State<AppState>,
    axum::extract::Path(version): axum::extract::Path<String>,
    axum::extract::Query(q): axum::extract::Query<BuildVersionQuery>,
) -> Result<Json<BuildRecordResponse>, (StatusCode, Json<BuildNotFound>)> {
    // Validate version format
    if version.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(BuildNotFound {
                error: "bad_request".to_string(),
                message: "Version is required".to_string(),
            }),
        ));
    }

    if let Err(reason) = q.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(BuildNotFound {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    match get_build(state.db.pool(), Some(&version), None, q.project_opt(), q.target_opt()).await {
        Ok(Some(row)) => Ok(Json(BuildRecordResponse::from(row))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(BuildNotFound {
                error: "not_found".to_string(),
                message: format!("Build not found for version {}", version),
            }),
        )),
        Err(e) => {
            warn!("Error fetching build by version: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BuildNotFound {
                    error: "internal_error".to_string(),
                    message: "Failed to fetch build".to_string(),
                }),
            ))
        }
    }
}

/// Public endpoint: GET /v1/builds/hash/{build_hash}
///
/// Returns a build record by build hash. Used by CIRISVerify for file integrity verification.
/// This endpoint is unauthenticated.
///
/// Multi-target releases (CIRISVerify v2.0.3+ ciris-build-sign register)
/// share `build_hash` across N target rows in the same `(project, version)` —
/// `derive_build_hash` produces one hash from all per-target binary
/// hashes combined. After migration 029, multiple rows can match a
/// single `build_hash`; this handler returns the first by
/// `registered_at DESC` ordering, sufficient for liveness checks.
/// Callers needing target-specific data should use
/// `GET /v1/builds/{version}?project=&target=` instead.
async fn get_build_by_hash(
    State(state): State<AppState>,
    axum::extract::Path(build_hash): axum::extract::Path<String>,
) -> Result<Json<BuildRecordResponse>, (StatusCode, Json<BuildNotFound>)> {
    // Validate hash format (should be hex)
    if build_hash.is_empty() || build_hash.len() != 64 || !build_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(BuildNotFound {
                error: "bad_request".to_string(),
                message: "Invalid build hash format (expected 64 hex characters)".to_string(),
            }),
        ));
    }

    // Multi-target releases share build_hash (mig 029); LIMIT 1 in
    // get_build returns the most recent row by (project, version, target)
    // ordering — sufficient for liveness checks.
    match get_build(state.db.pool(), None, Some(&build_hash), None, None).await {
        Ok(Some(row)) => Ok(Json(BuildRecordResponse::from(row))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(BuildNotFound {
                error: "not_found".to_string(),
                message: format!("Build not found for hash {}", &build_hash[..16]),
            }),
        )),
        Err(e) => {
            warn!("Error fetching build by hash: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BuildNotFound {
                    error: "internal_error".to_string(),
                    message: "Failed to fetch build".to_string(),
                }),
            ))
        }
    }
}

/// Public endpoint: GET /v1/verify/function-manifest/{version}/{target}?project=...
///
/// Returns a function manifest for runtime verification.
async fn function_manifest(
    State(state): State<AppState>,
    axum::extract::Path((version, target)): axum::extract::Path<(String, String)>,
    axum::extract::Query(q): axum::extract::Query<ProjectQuery>,
) -> Result<Json<db::FunctionManifestResponse>, (StatusCode, Json<FunctionManifestError>)> {
    if let Err(reason) = q.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(FunctionManifestError {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    match db::get_function_manifest(state.db.pool(), &version, &target, q.as_opt()).await {
        Ok(Some(manifest)) => Ok(Json(manifest.to_response())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(FunctionManifestError {
                error: "not_found".to_string(),
                message: format!(
                    "Function manifest not found for version {} target {}",
                    version, target
                ),
            }),
        )),
        Err(e) => {
            warn!("Error fetching function manifest: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FunctionManifestError {
                    error: "internal_error".to_string(),
                    message: "Failed to fetch function manifest".to_string(),
                }),
            ))
        }
    }
}

/// Public endpoint: GET /v1/verify/function-manifests/{version}?project=...
///
/// Lists available targets for a project + version.
async fn list_function_manifest_targets(
    State(state): State<AppState>,
    axum::extract::Path(version): axum::extract::Path<String>,
    axum::extract::Query(q): axum::extract::Query<ProjectQuery>,
) -> Result<Json<db::AvailableTargetsResponse>, (StatusCode, Json<FunctionManifestError>)> {
    if let Err(reason) = q.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(FunctionManifestError {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    match db::list_function_manifest_targets(state.db.pool(), &version, q.as_opt()).await {
        Ok(targets) => {
            if targets.is_empty() {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(FunctionManifestError {
                        error: "not_found".to_string(),
                        message: format!("No function manifests found for version {}", version),
                    }),
                ))
            } else {
                Ok(Json(db::AvailableTargetsResponse { version, targets }))
            }
        }
        Err(e) => {
            warn!("Error listing function manifest targets: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FunctionManifestError {
                    error: "internal_error".to_string(),
                    message: "Failed to list function manifest targets".to_string(),
                }),
            ))
        }
    }
}

/// Admin endpoint: POST /v1/verify/function-manifest
///
/// Register a new function manifest from CI. Requires REGISTRY_ADMIN_TOKEN.
async fn register_function_manifest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RegisterFunctionManifestRequest>,
) -> Result<Json<RegisterFunctionManifestResponse>, (StatusCode, Json<FunctionManifestError>)> {
    // Check authorization
    let admin_token = std::env::var("REGISTRY_ADMIN_TOKEN").unwrap_or_default();
    if admin_token.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(FunctionManifestError {
                error: "configuration_error".to_string(),
                message: "REGISTRY_ADMIN_TOKEN not configured".to_string(),
            }),
        ));
    }

    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let provided_token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);

    if provided_token != admin_token {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(FunctionManifestError {
                error: "unauthorized".to_string(),
                message: "Invalid or missing authorization token".to_string(),
            }),
        ));
    }

    // Validate project name (empty → "ciris-agent")
    if let Err(reason) = crate::validation::validate_project_name(&req.project) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(FunctionManifestError {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    // Parse generated_at timestamp
    let generated_at = time::OffsetDateTime::parse(
        &req.generated_at,
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap_or_else(|_| time::OffsetDateTime::now_utc());

    // Build manifest JSON (store the full manifest for serving)
    // Include metadata if present (contains text_section_offset for address calculation)
    let mut manifest_json = serde_json::json!({
        "project": if req.project.is_empty() { "ciris-agent" } else { req.project.as_str() },
        "version": req.version,
        "target": req.target,
        "binary_hash": req.binary_hash,
        "binary_version": req.binary_version,
        "generated_at": req.generated_at,
        "functions": req.functions,
    });
    if let Some(metadata) = &req.metadata {
        manifest_json["metadata"] = metadata.clone();
    }

    // Registry-side signing: always sign with steward key
    // CI doesn't have access to private key, so we sign server-side
    let (sig_classical, sig_pqc, sig_key_id) = match state.crypto.sign(req.manifest_hash.as_bytes()) {
        Ok(sig) => {
            use base64::{engine::general_purpose::STANDARD, Engine};
            (
                STANDARD.encode(&sig.classical_signature),
                STANDARD.encode(&sig.post_quantum_signature),
                state.crypto.key_id().to_string(),
            )
        }
        Err(e) => {
            warn!("Failed to sign function manifest: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FunctionManifestError {
                    error: "signing_error".to_string(),
                    message: "Failed to sign function manifest".to_string(),
                }),
            ));
        }
    };

    // Register the manifest with signature.
    // Legacy /v1/verify/function-manifest path: registry re-signs server-side
    // (Case (ii) in TRUST_CONTRACT.md §2.3) — no original CI body to preserve.
    // raw_manifest_body stays NULL; Path B will 404 for rows POSTed here.
    match db::register_function_manifest(
        state.db.pool(),
        &req.project,
        &req.binary_version,
        &req.target,
        &req.version,
        &req.binary_hash,
        &req.manifest_hash,
        &manifest_json,
        Some(&sig_classical),
        Some(&sig_pqc),
        Some(&sig_key_id),
        generated_at,
        None,
    )
    .await
    {
        Ok(manifest_hash) => {
            info!(
                "Function manifest registered: project={}, version={}, target={}, key_id={}",
                if req.project.is_empty() { "ciris-agent" } else { req.project.as_str() },
                req.binary_version,
                req.target,
                sig_key_id
            );
            Ok(Json(RegisterFunctionManifestResponse {
                success: true,
                manifest_hash,
                message: format!(
                    "Function manifest registered for version {} target {} (signed)",
                    req.binary_version, req.target
                ),
            }))
        }
        Err(e) => {
            warn!("Error registering function manifest: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FunctionManifestError {
                    error: "internal_error".to_string(),
                    message: "Failed to register function manifest".to_string(),
                }),
            ))
        }
    }
}

// =============================================================================
// Verified BuildManifest (v1.4.2 — AV-26 mitigation, Phase A)
// =============================================================================

/// Response for /v1/verify/build-manifest POST.
#[derive(Serialize)]
struct VerifiedBuildManifestResponse {
    success: bool,
    project: String,
    binary_version: String,
    target: String,
    manifest_hash: String,
    /// Fingerprint of the trusted Ed25519 key that verified the upload.
    /// Operators can compare against `ListTrustedPrimitiveKeys` output to
    /// confirm which key authorized the registration.
    verifying_key_fingerprint: String,
    message: String,
}

#[derive(Serialize)]
struct VerifiedBuildManifestError {
    error: String,
    message: String,
}

/// Admin endpoint: POST /v1/verify/build-manifest
///
/// Body: raw BuildManifest JSON (output of `ciris-build-sign --primitive
/// <name>`). Hybrid-signed by the per-primitive build-signing key.
///
/// Process:
/// 1. Parse the body as a `BuildManifest` (vendored from ciris-verify
///    v1.8.0 wire format).
/// 2. Look up the trusted Ed25519 + ML-DSA-65 pubkeys for
///    `manifest.primitive.project_name()`.
/// 3. Verify the hybrid signature (`build_manifest::verify_uploaded_manifest`).
/// 4. On success, store via the existing `function_manifests` table
///    (project-scoped; same shape as the legacy register_function_manifest
///    POST handler).
///
/// Auth: REGISTRY_ADMIN_TOKEN (defense in depth — the signature is the
/// primary trust anchor; the token gates *who can attempt* registration).
///
/// Closes THREAT_MODEL.md AV-26.
async fn register_verified_build_manifest(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<Json<VerifiedBuildManifestResponse>, (StatusCode, Json<VerifiedBuildManifestError>)> {
    // Auth gate (mirrors register_function_manifest pattern).
    let admin_token = std::env::var("REGISTRY_ADMIN_TOKEN").unwrap_or_default();
    if admin_token.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(VerifiedBuildManifestError {
                error: "configuration_error".to_string(),
                message: "REGISTRY_ADMIN_TOKEN not configured".to_string(),
            }),
        ));
    }
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let provided_token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);
    if provided_token != admin_token {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(VerifiedBuildManifestError {
                error: "unauthorized".to_string(),
                message: "Invalid or missing authorization token".to_string(),
            }),
        ));
    }

    // 1. Parse without verifying first, so we can extract the project
    //    name to look up the trusted key.
    let manifest_preview: crate::build_manifest::BuildManifest =
        match serde_json::from_slice(&body) {
            Ok(m) => m,
            Err(e) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(VerifiedBuildManifestError {
                        error: "invalid_manifest".to_string(),
                        message: format!("BuildManifest parse failed: {}", e),
                    }),
                ));
            }
        };
    let project = manifest_preview.primitive.project_name();
    let expected_primitive = manifest_preview.primitive.clone();

    // 2. Look up the trusted key for this primitive.
    let trusted = match db::get_trusted_primitive_key(state.db.pool(), &project).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(VerifiedBuildManifestError {
                    error: "no_trusted_key".to_string(),
                    message: format!(
                        "No trusted primitive key registered for project='{}'. \
                         A SYSTEM_ADMIN must call RegisterTrustedPrimitiveKey first.",
                        project
                    ),
                }),
            ));
        }
        Err(e) => {
            warn!("trusted_primitive_key lookup failed for project={}: {}", project, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(VerifiedBuildManifestError {
                    error: "internal_error".to_string(),
                    message: "Trusted-key lookup failed".to_string(),
                }),
            ));
        }
    };

    // 3. Verify the hybrid signature.
    let verified = crate::build_manifest::verify_uploaded_manifest(
        &body,
        expected_primitive,
        &trusted.ed25519_public_key,
        &trusted.ml_dsa_65_public_key,
    );
    let manifest = match verified {
        Ok(m) => m,
        Err(e) => {
            warn!(
                project = %project,
                "BuildManifest verification rejected: {}",
                e
            );
            return Err((
                StatusCode::BAD_REQUEST,
                Json(VerifiedBuildManifestError {
                    error: "verification_failed".to_string(),
                    message: format!("BuildManifest verification failed: {}", e),
                }),
            ));
        }
    };

    // 4. Store in function_manifests. The project-scoped (project,
    //    binary_version, target) PK from migration 021 is the natural
    //    home — function_manifests already carries hybrid signature
    //    columns from migration 019.
    let manifest_json = serde_json::to_value(&manifest).unwrap_or(serde_json::json!({}));
    let generated_at = time::OffsetDateTime::parse(
        &manifest.generated_at,
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap_or_else(|_| time::OffsetDateTime::now_utc());

    // Capture verbatim POST body so Path B can serve byte-identical
    // BuildManifest later (CIRISRegistry#5 §2). The original CI signature
    // is over these exact bytes — re-serializing through serde would
    // invalidate canonical-byte verification on the consumer side.
    if let Err(e) = db::register_function_manifest(
        state.db.pool(),
        &project,
        &manifest.binary_version,
        &manifest.target,
        &manifest.manifest_schema_version,
        &manifest.binary_hash,
        &manifest.manifest_hash,
        &manifest_json,
        Some(&manifest.signature.classical),
        Some(&manifest.signature.pqc),
        Some(&manifest.signature.key_id),
        generated_at,
        Some(body.as_ref()),
    )
    .await
    {
        warn!("Failed to persist verified BuildManifest: {}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(VerifiedBuildManifestError {
                error: "persist_failed".to_string(),
                message: "Failed to persist verified manifest".to_string(),
            }),
        ));
    }

    info!(
        project = %project,
        binary_version = %manifest.binary_version,
        target = %manifest.target,
        verifying_key_fp = %trusted.ed25519_fingerprint,
        "verified_build_manifest_stored"
    );

    Ok(Json(VerifiedBuildManifestResponse {
        success: true,
        project,
        binary_version: manifest.binary_version,
        target: manifest.target,
        manifest_hash: manifest.manifest_hash,
        verifying_key_fingerprint: trusted.ed25519_fingerprint,
        message: "BuildManifest verified and stored".to_string(),
    }))
}

/// Public endpoint: GET /v1/verify/build-manifest/{project}/{version}/{target}
///
/// Returns the verbatim BuildManifest JSON bytes that the publishing
/// primitive's CI signed — byte-identical to what was POSTed via the
/// `/v1/verify/build-manifest` endpoint. The original Ed25519 + ML-DSA-65
/// signature is embedded in the returned JSON; consumers verify it against
/// the per-primitive trusted-primitive-key (obtained out-of-band or via
/// CIRISRegistry#5 §4's discovery endpoint when it ships).
///
/// Returns 404 if:
///   - No row exists for (project, version, target), OR
///   - The row was POSTed via the legacy `/v1/verify/function-manifest`
///     endpoint (server-resigned; no original CI body captured). Use Path A
///     (`GET /v1/verify/function-manifest/{version}/{target}`) for those rows.
///
/// Path B per docs/TRUST_CONTRACT.md §5. Closes CIRISRegistry#5 §2.
async fn get_verified_build_manifest(
    State(state): State<AppState>,
    axum::extract::Path((project, version, target)): axum::extract::Path<(String, String, String)>,
) -> Result<axum::response::Response, (StatusCode, Json<VerifiedBuildManifestError>)> {
    if let Err(reason) = crate::validation::validate_project_name(&project) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(VerifiedBuildManifestError {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    let body = match db::get_function_manifest_raw_body(
        state.db.pool(),
        &project,
        &version,
        &target,
    )
    .await
    {
        Ok(Some(b)) => b,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(VerifiedBuildManifestError {
                    error: "not_found".to_string(),
                    message: format!(
                        "No verbatim BuildManifest stored for project={} version={} target={}. \
                         Either no row exists, or the row was POSTed via the legacy \
                         /v1/verify/function-manifest endpoint (no raw body captured).",
                        project, version, target
                    ),
                }),
            ));
        }
        Err(e) => {
            warn!("Path B fetch failed: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(VerifiedBuildManifestError {
                    error: "internal_error".to_string(),
                    message: "Failed to fetch verbatim BuildManifest".to_string(),
                }),
            ));
        }
    };

    use axum::response::IntoResponse;
    let mut resp = (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body,
    )
        .into_response();
    // Cache-Control is intentionally absent — manifest content is immutable
    // per (project, version, target), but operators may want to inspect
    // revocation state via a fresh GET. Consumers SHOULD treat the body as
    // immutable per the TRUST_CONTRACT §2.1 caching guidance.
    resp.headers_mut().insert(
        "X-Manifest-Path",
        axum::http::HeaderValue::from_static("path-b-verbatim"),
    );
    Ok(resp)
}

// =============================================================================
// POST /v1/builds — self-signed BuildRecord registration (issue #9)
// =============================================================================
//
// Eliminates the REGISTRY_JWT_SECRET requirement that gRPC RegisterBuild forces
// on every primitive's CI. Auth model matches the rest of the release pipeline:
// REGISTRY_ADMIN_TOKEN bearer + content-signature verification against
// trusted_primitive_keys for the named project.
//
// Wire contract — the bytes that get signed.
//
// `CanonicalBuild` field order is the inter-implementation contract (registry
// here, ciris-build-sign on the caller side). Changing it is a breaking change
// across CIRISRegistry + CIRISVerify (`ciris-build-sign register`). Order
// matches the issue body's RegisterBuildHttpRequest field declaration so callers
// can derive it from the obvious source.
//
// Hybrid signature scheme mirrors BuildManifest exactly (build_manifest.rs):
//   - Ed25519 sig: over canonical_bytes
//   - ML-DSA-65 sig: over (canonical_bytes || classical_sig)  [bound payload]

/// Request body for POST /v1/builds.
///
/// Signed fields (covered by canonical_bytes): project, version, build_hash,
/// build_id, modules, file_manifest_count.
///
/// Unsigned metadata (informational; rides along on the row but not bound to
/// the primitive's signature): file_manifest_hash, file_manifest_json,
/// source_repo, source_commit, notes. These can't change the build's identity
/// (the unique key is build_hash) — keeping them unsigned avoids forcing
/// callers to canonicalize JSONB payloads.
#[derive(serde::Deserialize)]
struct RegisterBuildHttpRequest {
    project: String,
    version: String,
    /// Build target (e.g. `python-source-tree`, `ios-mobile-bundle`,
    /// `android-mobile-bundle`). Required since v1.4.1 (CanonicalBuild v2,
    /// closes #11) — disambiguates multi-target releases so version
    /// lookups don't return the wrong target's manifest.
    target: String,
    build_hash: String,
    /// Caller-asserted build identifier (typically a git SHA). Bound by the
    /// signature; not stored as the DB build_id (DB autogenerates a UUID).
    build_id: String,
    #[serde(default)]
    modules: Vec<String>,
    #[serde(default)]
    file_manifest_count: u64,

    // Unsigned metadata — defaults preserve the NOT NULL DB constraints.
    #[serde(default)]
    file_manifest_hash: String,
    #[serde(default)]
    file_manifest_json: serde_json::Value,
    #[serde(default)]
    source_repo: String,
    #[serde(default)]
    source_commit: String,
    #[serde(default)]
    notes: String,

    // Hybrid signature triplet over canonical_bytes(self).
    signature_classical: String,
    signature_pqc: String,
    #[serde(default)]
    signature_key_id: String,
}

#[derive(Serialize)]
struct RegisterBuildHttpResponse {
    success: bool,
    build_id: String,
    /// Fingerprint of the trusted Ed25519 key that verified the registration.
    /// Operators can compare against ListTrustedPrimitiveKeys.
    verifying_key_fingerprint: String,
    message: String,
}

#[derive(Serialize)]
struct RegisterBuildHttpError {
    error: String,
    message: String,
}

/// Canonical signing form for a Build registration (v2, since v1.4.1).
/// Field order is the wire contract — see module-level comment above.
///
/// **v2 change (#11)**: `target` is inserted between `version` and
/// `build_hash`. Bumps the wire format relative to the c2dc594 v1 shipped
/// 2026-05-04; ciris-build-sign register must cut over to match
/// (CIRISVerify#8).
#[derive(Serialize)]
struct CanonicalBuild<'a> {
    project: &'a str,
    version: &'a str,
    target: &'a str,
    build_hash: &'a str,
    build_id: &'a str,
    modules: &'a [String],
    file_manifest_count: u64,
}

impl RegisterBuildHttpRequest {
    fn canonical_bytes(&self) -> Vec<u8> {
        let canonical = CanonicalBuild {
            project: &self.project,
            version: &self.version,
            target: &self.target,
            build_hash: &self.build_hash,
            build_id: &self.build_id,
            modules: &self.modules,
            file_manifest_count: self.file_manifest_count,
        };
        serde_json::to_vec(&canonical).unwrap_or_default()
    }
}

/// Admin endpoint: POST /v1/builds
///
/// Self-signed Build registration. Bearer auth gates *who can attempt*; the
/// content signature against `trusted_primitive_keys` is the trust anchor.
/// Closes issue #9 — eliminates the REGISTRY_JWT_SECRET requirement that the
/// gRPC RegisterBuild forces on every primitive's CI.
async fn register_build_http(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RegisterBuildHttpRequest>,
) -> Result<Json<RegisterBuildHttpResponse>, (StatusCode, Json<RegisterBuildHttpError>)> {
    // Auth gate (mirrors register_binary_manifest pattern).
    let admin_token = std::env::var("REGISTRY_ADMIN_TOKEN").unwrap_or_default();
    if admin_token.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RegisterBuildHttpError {
                error: "configuration_error".to_string(),
                message: "REGISTRY_ADMIN_TOKEN not configured".to_string(),
            }),
        ));
    }
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let provided_token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);
    if provided_token != admin_token {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(RegisterBuildHttpError {
                error: "unauthorized".to_string(),
                message: "Invalid or missing authorization token".to_string(),
            }),
        ));
    }

    if req.project.is_empty()
        || req.version.is_empty()
        || req.target.is_empty()
        || req.build_hash.is_empty()
        || req.build_id.is_empty()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(RegisterBuildHttpError {
                error: "bad_request".to_string(),
                message: "project, version, target, build_hash, and build_id are required"
                    .to_string(),
            }),
        ));
    }
    if let Err(reason) = crate::validation::validate_project_name(&req.project) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(RegisterBuildHttpError {
                error: "bad_request".to_string(),
                message: reason,
            }),
        ));
    }

    let project = req.project.clone();

    // Look up the trusted primitive key for this project.
    let trusted = match db::get_trusted_primitive_key(state.db.pool(), &project).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(RegisterBuildHttpError {
                    error: "no_trusted_key".to_string(),
                    message: format!(
                        "No trusted primitive key registered for project='{}'. \
                         A SYSTEM_ADMIN must call RegisterTrustedPrimitiveKey first.",
                        project
                    ),
                }),
            ));
        }
        Err(e) => {
            warn!("trusted_primitive_key lookup failed for project={}: {}", project, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterBuildHttpError {
                    error: "internal_error".to_string(),
                    message: "Trusted-key lookup failed".to_string(),
                }),
            ));
        }
    };

    // Verify hybrid signature — Ed25519 over canonical, ML-DSA-65 over
    // (canonical || classical_sig). Same scheme as build_manifest::verify_uploaded_manifest.
    let canonical = req.canonical_bytes();
    let b64 = base64::engine::general_purpose::STANDARD;
    let classical_sig = match b64.decode(&req.signature_classical) {
        Ok(s) => s,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(RegisterBuildHttpError {
                    error: "invalid_signature".to_string(),
                    message: format!("classical signature base64 decode: {}", e),
                }),
            ));
        }
    };
    let pqc_sig = match b64.decode(&req.signature_pqc) {
        Ok(s) => s,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(RegisterBuildHttpError {
                    error: "invalid_signature".to_string(),
                    message: format!("pqc signature base64 decode: {}", e),
                }),
            ));
        }
    };

    use ciris_crypto::{ClassicalVerifier, Ed25519Verifier, MlDsa65Verifier, PqcVerifier};
    let ed_ok = Ed25519Verifier::new()
        .verify(&trusted.ed25519_public_key, &canonical, &classical_sig)
        .unwrap_or(false);
    if !ed_ok {
        warn!(project = %project, "POST /v1/builds Ed25519 verification failed");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(RegisterBuildHttpError {
                error: "verification_failed".to_string(),
                message: "Ed25519 signature did not verify".to_string(),
            }),
        ));
    }
    let mut bound = Vec::with_capacity(canonical.len() + classical_sig.len());
    bound.extend_from_slice(&canonical);
    bound.extend_from_slice(&classical_sig);
    let pqc_ok = MlDsa65Verifier::new()
        .verify(&trusted.ml_dsa_65_public_key, &bound, &pqc_sig)
        .unwrap_or(false);
    if !pqc_ok {
        warn!(project = %project, "POST /v1/builds ML-DSA-65 verification failed");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(RegisterBuildHttpError {
                error: "verification_failed".to_string(),
                message: "ML-DSA-65 signature did not verify".to_string(),
            }),
        ));
    }

    // Synthesize the BuildRecord proto for db::register_build. file_manifest_hash
    // defaults to build_hash (NOT NULL satisfied) — the existing gRPC path
    // already accepts arbitrary callers' values here, treating it as informational.
    let file_manifest_hash = if req.file_manifest_hash.is_empty() {
        req.build_hash.clone()
    } else {
        req.file_manifest_hash.clone()
    };
    let manifest_json_bytes = if req.file_manifest_json.is_null() {
        b"{}".to_vec()
    } else {
        serde_json::to_vec(&req.file_manifest_json).unwrap_or_else(|_| b"{}".to_vec())
    };
    let build = crate::proto::BuildRecord {
        build_id: req.build_id.clone(),
        version: req.version.clone(),
        build_hash: req.build_hash.clone(),
        file_manifest_hash,
        file_manifest_count: i32::try_from(req.file_manifest_count).unwrap_or(i32::MAX),
        file_manifest_json: manifest_json_bytes.into(),
        includes_modules: req.modules.clone(),
        project: project.clone(),
        target: req.target.clone(),
        source_repo: req.source_repo.clone(),
        source_commit: req.source_commit.clone(),
        registered_at: 0,
        registered_by: format!("http_v1_builds:{}", trusted.ed25519_fingerprint),
        status: "active".to_string(),
        notes: req.notes.clone(),
    };

    let build_id = match db::register_build(state.db.pool(), &build).await {
        Ok(id) => id,
        Err(e) => {
            // Surface the underlying sqlx error in the response body. This
            // endpoint is REGISTRY_ADMIN_TOKEN-gated and the caller is by
            // definition trusted with the pre-shared admin secret; leaking
            // SQL constraint names + table names is acceptable here and the
            // alternative (opaque "Failed to register build") forced
            // CIRISRegistry#13's filer to file a ticket and wait for ops
            // to pull production logs. For UNIQUE constraint hits, return
            // 409 Conflict so callers can distinguish a duplicate-write
            // from a true server error and choose to retry differently.
            warn!(
                project = %project,
                version = %req.version,
                target = %req.target,
                build_hash = %req.build_hash,
                "register_build failed: {}",
                e
            );
            let err_str = e.to_string();
            let is_conflict = err_str.contains("duplicate key")
                || err_str.contains("unique constraint");
            let (status, code) = if is_conflict {
                (StatusCode::CONFLICT, "duplicate")
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
            };
            return Err((
                status,
                Json(RegisterBuildHttpError {
                    error: code.to_string(),
                    message: format!("register_build failed: {}", err_str),
                }),
            ));
        }
    };

    info!(
        project = %project,
        version = %req.version,
        target = %req.target,
        build_hash = %req.build_hash,
        verifying_key_fp = %trusted.ed25519_fingerprint,
        signature_key_id = %req.signature_key_id,
        "POST /v1/builds registered"
    );

    Ok(Json(RegisterBuildHttpResponse {
        success: true,
        build_id,
        verifying_key_fingerprint: trusted.ed25519_fingerprint,
        message: format!("Build registered for {} {} ({})", project, req.version, req.target),
    }))
}

// =============================================================================
// Key Verification (CIRISVerify agent signing key validation)
// =============================================================================

/// Response for key verification by fingerprint
#[derive(Serialize)]
struct KeyVerifyResponse {
    found: bool,
    key_id: Option<String>,
    org_id: Option<String>,
    status: String,
    status_code: i32,
    ed25519_fingerprint: Option<String>,
    ml_dsa_65_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ed25519_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ml_dsa_65_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activated_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revocation_reason: Option<String>,
}

/// Error response for key verification
#[derive(Serialize)]
struct KeyVerifyError {
    error: String,
    message: String,
}

/// Public endpoint: GET /v1/verify/key/{fingerprint}
///
/// Verify a signing key by its Ed25519 fingerprint (SHA-256 hex).
/// Used by CIRISVerify to validate agent signing keys.
///
/// Returns:
/// - found: true if key exists
/// - status: KEY_ACTIVE, KEY_REVOKED, KEY_ROTATED, KEY_PENDING
/// - public keys and metadata
async fn verify_key_by_fingerprint(
    State(state): State<AppState>,
    axum::extract::Path(fingerprint): axum::extract::Path<String>,
) -> Result<Json<KeyVerifyResponse>, (StatusCode, Json<KeyVerifyError>)> {
    // Validate fingerprint format (64 hex chars = SHA-256)
    if fingerprint.len() != 64 || !fingerprint.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(KeyVerifyError {
                error: "bad_request".to_string(),
                message: "Invalid fingerprint format (expected 64 hex characters)".to_string(),
            }),
        ));
    }

    match db::lookup_key_by_fingerprint(state.db.pool(), &fingerprint).await {
        Ok(Some(key)) => {
            let status_str = match key.status {
                0 => "KEY_PENDING",
                1 => "KEY_ACTIVE",
                2 => "KEY_ROTATED",
                3 => "KEY_REVOKED",
                _ => "KEY_UNKNOWN",
            };

            let b64 = base64::engine::general_purpose::STANDARD;

            Ok(Json(KeyVerifyResponse {
                found: true,
                key_id: Some(key.key_id),
                org_id: Some(key.org_id),
                status: status_str.to_string(),
                status_code: key.status,
                ed25519_fingerprint: Some(key.ed25519_fingerprint),
                ml_dsa_65_fingerprint: Some(key.ml_dsa_65_fingerprint),
                ed25519_public_key: Some(b64.encode(&key.ed25519_public_key)),
                ml_dsa_65_public_key: Some(b64.encode(&key.ml_dsa_65_public_key)),
                activated_at: key.activated_at.map(|t| t.unix_timestamp()),
                revoked_at: key.revoked_at.map(|t| t.unix_timestamp()),
                revocation_reason: key.revocation_reason,
            }))
        }
        Ok(None) => Ok(Json(KeyVerifyResponse {
            found: false,
            key_id: None,
            org_id: None,
            status: "NOT_FOUND".to_string(),
            status_code: -1,
            ed25519_fingerprint: None,
            ml_dsa_65_fingerprint: None,
            ed25519_public_key: None,
            ml_dsa_65_public_key: None,
            activated_at: None,
            revoked_at: None,
            revocation_reason: None,
        })),
        Err(e) => {
            warn!("Error looking up key by fingerprint: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(KeyVerifyError {
                    error: "internal_error".to_string(),
                    message: "Failed to lookup key".to_string(),
                }),
            ))
        }
    }
}

// =============================================================================
// Play Integrity Verification
// =============================================================================

/// Error response for Play Integrity endpoints
#[derive(Serialize)]
struct IntegrityError {
    error: String,
    message: String,
}

/// Rate limit error response
#[derive(Serialize)]
struct RateLimitError {
    error: String,
    message: String,
    retry_after_seconds: Option<u32>,
}

/// Extract client IP from request headers, only trusting proxy headers from known proxies.
///
/// Security: Only trusts X-Forwarded-For/X-Real-IP when the request appears to come
/// from a trusted proxy network (private IP ranges, loopback). This prevents
/// attackers from spoofing their IP to bypass rate limiting.
/// Local thin wrapper around `rate_limiter::extract_client_ip` for the axum
/// handler call sites that don't have a peer-addr extractor wired up. Pass
/// `None` for peer; trusted-proxy logic falls back to `X-Real-IP` membership
/// in the trusted CIDR ranges.
fn extract_client_ip(headers: &HeaderMap) -> std::net::IpAddr {
    crate::rate_limiter::extract_client_ip(headers, None)
}

/// Public endpoint: GET /v1/integrity/nonce
///
/// Generate a cryptographically secure nonce for Play Integrity verification.
/// The nonce is single-use and expires in 5 minutes.
/// Rate limited: 10/min, 100/hour per IP.
async fn integrity_nonce(
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<IntegrityNonceParams>,
) -> Result<Json<crate::play_integrity::IntegrityNonceResponse>, (StatusCode, Json<RateLimitError>)> {
    // Check rate limit
    let client_ip = extract_client_ip(&headers);
    let rate_limit_result = check_nonce_rate_limit(client_ip);

    if !rate_limit_result.is_allowed() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(RateLimitError {
                error: "rate_limit_exceeded".to_string(),
                message: rate_limit_result.error_message().unwrap_or_default(),
                retry_after_seconds: rate_limit_result.retry_after(),
            }),
        ));
    }

    let config = PlayIntegrityConfig::default();
    let service = PlayIntegrityService::new(config);
    Ok(Json(service.generate_nonce(params.context.as_deref())))
}

#[derive(serde::Deserialize)]
struct IntegrityNonceParams {
    context: Option<String>,
}

/// Public endpoint: POST /v1/integrity/verify
///
/// Verify a Play Integrity token. Decodes the token via Google's API
/// and returns device/app/account verdicts.
/// Rate limited: 5/min, 50/hour per IP (calls external Google API).
async fn integrity_verify(
    headers: HeaderMap,
    Json(req): Json<IntegrityVerifyRequest>,
) -> Result<Json<crate::play_integrity::IntegrityVerifyResponse>, (StatusCode, Json<IntegrityError>)>
{
    // Check rate limit (stricter for verify - calls external API)
    let client_ip = extract_client_ip(&headers);
    let rate_limit_result = check_verify_rate_limit(client_ip);

    if !rate_limit_result.is_allowed() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(IntegrityError {
                error: "rate_limit_exceeded".to_string(),
                message: rate_limit_result.error_message().unwrap_or_default(),
            }),
        ));
    }

    // Input validation: limit token size (Play Integrity tokens are ~2KB)
    if req.integrity_token.len() > 10_000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(IntegrityError {
                error: "payload_too_large".to_string(),
                message: "integrity_token exceeds maximum size (10KB)".to_string(),
            }),
        ));
    }

    let config = PlayIntegrityConfig::default();

    if config.service_account_json.is_none() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(IntegrityError {
                error: "not_configured".to_string(),
                message: "Play Integrity API not configured (PLAY_INTEGRITY_SERVICE_ACCOUNT missing)".to_string(),
            }),
        ));
    }

    let service = PlayIntegrityService::new(config);
    let result = service
        .verify_token(&req.integrity_token, &req.nonce, false)
        .await;

    Ok(Json(result))
}

/// Public endpoint: POST /v1/integrity/auth
///
/// Combined JWT + Play Integrity verification for high-security operations.
/// Requires both user authentication and device/app integrity.
/// Rate limited: 5/min, 50/hour per IP (calls external Google API).
///
/// Note: This endpoint expects a Bearer token for JWT auth, but since
/// Registry doesn't have user auth, it just validates the integrity token
/// and returns auth status based on any provided context.
async fn integrity_auth(
    headers: HeaderMap,
    Json(req): Json<IntegrityAuthRequest>,
) -> Result<Json<IntegrityAuthResponse>, (StatusCode, Json<IntegrityError>)> {
    // Check rate limit (same as verify - calls external API)
    let client_ip = extract_client_ip(&headers);
    let rate_limit_result = check_verify_rate_limit(client_ip);

    if !rate_limit_result.is_allowed() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(IntegrityError {
                error: "rate_limit_exceeded".to_string(),
                message: rate_limit_result.error_message().unwrap_or_default(),
            }),
        ));
    }

    // Input validation: limit token size (Play Integrity tokens are ~2KB)
    if req.integrity_token.len() > 10_000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(IntegrityError {
                error: "payload_too_large".to_string(),
                message: "integrity_token exceeds maximum size (10KB)".to_string(),
            }),
        ));
    }

    let config = PlayIntegrityConfig::default();

    if config.service_account_json.is_none() {
        return Ok(Json(IntegrityAuthResponse {
            authenticated: false,
            integrity_verified: false,
            user_id: None,
            email: None,
            device_integrity: None,
            app_integrity: None,
            authorized: false,
            reason: Some("Play Integrity API not configured".to_string()),
        }));
    }

    // Check for bearer token (optional - for logging/context only in Registry)
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let has_bearer = auth_header.starts_with("Bearer ");

    let service = PlayIntegrityService::new(config);
    let result = service
        .verify_token(&req.integrity_token, &req.nonce, false)
        .await;

    let integrity_verified = result.verified;
    let authorized = has_bearer && integrity_verified;

    Ok(Json(IntegrityAuthResponse {
        authenticated: has_bearer,
        integrity_verified,
        user_id: None,  // Registry doesn't decode user JWTs
        email: None,
        device_integrity: result.device_integrity,
        app_integrity: result.app_integrity,
        authorized,
        reason: if !authorized {
            Some(result.error.unwrap_or_else(|| {
                if !has_bearer {
                    "Missing Bearer token".to_string()
                } else {
                    "Integrity verification failed".to_string()
                }
            }))
        } else {
            None
        },
    }))
}

// =============================================================================
// iOS App Attest Verification
// =============================================================================

/// Error response for App Attest endpoints
#[derive(Serialize)]
struct AppAttestError {
    error: String,
    message: String,
}

/// Public endpoint: GET /v1/integrity/ios/nonce
///
/// Generate a cryptographically secure nonce for App Attest attestation.
/// The nonce is single-use and expires in 5 minutes.
/// Rate limited: 10/min, 100/hour per IP.
async fn ios_attest_nonce(
    headers: HeaderMap,
    axum::extract::Query(params): axum::extract::Query<IntegrityNonceParams>,
) -> Result<Json<crate::app_attest::AppAttestNonceResponse>, (StatusCode, Json<RateLimitError>)> {
    // Check rate limit
    let client_ip = extract_client_ip(&headers);
    let rate_limit_result = check_nonce_rate_limit(client_ip);

    if !rate_limit_result.is_allowed() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(RateLimitError {
                error: "rate_limit_exceeded".to_string(),
                message: rate_limit_result.error_message().unwrap_or_default(),
                retry_after_seconds: rate_limit_result.retry_after(),
            }),
        ));
    }

    let config = AppAttestConfig::default();
    let service = AppAttestService::new(config);
    Ok(Json(service.generate_nonce(params.context.as_deref())))
}

/// Public endpoint: POST /v1/integrity/ios/verify
///
/// Verify an App Attest attestation. This validates the device/app attestation
/// and stores the public key for future assertion verification.
/// Rejects re-attestation for already verified key_ids.
/// Rate limited: 5/min, 50/hour per IP (expensive verification).
async fn ios_attest_verify(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AppAttestVerifyRequest>,
) -> Result<Json<crate::app_attest::AppAttestVerifyResponse>, (StatusCode, Json<AppAttestError>)> {
    // Check rate limit (same limits as Play Integrity verify)
    let client_ip = extract_client_ip(&headers);
    let rate_limit_result = check_verify_rate_limit(client_ip);

    if !rate_limit_result.is_allowed() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(AppAttestError {
                error: "rate_limit_exceeded".to_string(),
                message: rate_limit_result.error_message().unwrap_or_default(),
            }),
        ));
    }

    // Input validation: limit attestation size (attestations are ~2-5KB)
    if req.attestation.len() > 50_000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AppAttestError {
                error: "payload_too_large".to_string(),
                message: "attestation exceeds maximum size (50KB)".to_string(),
            }),
        ));
    }

    if req.key_id.len() > 1000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AppAttestError {
                error: "payload_too_large".to_string(),
                message: "key_id exceeds maximum size".to_string(),
            }),
        ));
    }

    let config = AppAttestConfig::default();

    // Check if App Attest is configured
    if config.app_id.starts_with("TEAMID") {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(AppAttestError {
                error: "not_configured".to_string(),
                message: "iOS App Attest not configured (IOS_APP_ID, IOS_TEAM_ID missing)".to_string(),
            }),
        ));
    }

    // Check for attestation deduplication - reject if already attested
    match is_already_attested(state.db.pool(), &req.key_id).await {
        Ok(true) => {
            return Err((
                StatusCode::CONFLICT,
                Json(AppAttestError {
                    error: "already_attested".to_string(),
                    message: "This key_id has already been attested. Use /assert for subsequent requests.".to_string(),
                }),
            ));
        }
        Err(e) => {
            warn!("Database error checking attestation: {}", e);
            // Continue - don't block on DB errors
        }
        Ok(false) => {
            // Not yet attested, proceed
        }
    }

    let service = AppAttestService::new(config);
    let result = service
        .verify_attestation(&req.attestation, &req.key_id, &req.nonce)
        .await;

    // If verification succeeded, store in database for persistence
    if result.verified {
        if let Err(e) = crate::app_attest::store_attested_key(
            state.db.pool(),
            &req.key_id,
            &[], // Public key extracted by service and stored in memory
            result.counter.unwrap_or(0),
            &hex::decode(result.app_id_hash.as_deref().unwrap_or("")).unwrap_or_default(),
            result.environment.as_deref().unwrap_or("production"),
        ).await {
            warn!("Failed to persist attested key: {}", e);
            // Don't fail - in-memory storage still works
        }
    }

    Ok(Json(result))
}

/// Public endpoint: POST /v1/integrity/ios/assert
///
/// Verify an App Attest assertion. This validates that subsequent requests
/// come from the same attested device using the stored public key.
/// Results are cached for 5 minutes to reduce load.
/// Rate limited: 5/min, 50/hour per IP.
async fn ios_attest_assert(
    headers: HeaderMap,
    Json(req): Json<AppAttestAssertRequest>,
) -> Result<Json<crate::app_attest::AppAttestAssertResponse>, (StatusCode, Json<AppAttestError>)> {
    // Check rate limit
    let client_ip = extract_client_ip(&headers);
    let rate_limit_result = check_verify_rate_limit(client_ip);

    if !rate_limit_result.is_allowed() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(AppAttestError {
                error: "rate_limit_exceeded".to_string(),
                message: rate_limit_result.error_message().unwrap_or_default(),
            }),
        ));
    }

    // Input validation
    if req.assertion.len() > 50_000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AppAttestError {
                error: "payload_too_large".to_string(),
                message: "assertion exceeds maximum size (50KB)".to_string(),
            }),
        ));
    }

    if req.client_data.len() > 10_000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AppAttestError {
                error: "payload_too_large".to_string(),
                message: "client_data exceeds maximum size (10KB)".to_string(),
            }),
        ));
    }

    let config = AppAttestConfig::default();

    // Check if App Attest is configured
    if config.app_id.starts_with("TEAMID") {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(AppAttestError {
                error: "not_configured".to_string(),
                message: "iOS App Attest not configured (IOS_APP_ID, IOS_TEAM_ID missing)".to_string(),
            }),
        ));
    }

    // Compute client_data hash for cache key
    use sha2::{Sha256, Digest};
    let client_data_hash = hex::encode(Sha256::digest(req.client_data.as_bytes()));

    // Check assertion cache first
    match check_assertion_cache(&req.key_id, &client_data_hash) {
        AssertionCacheResult::Hit { verified, counter, .. } => {
            // Return cached result
            return Ok(Json(crate::app_attest::AppAttestAssertResponse {
                verified,
                key_id: Some(req.key_id.clone()),
                counter: Some(counter),
                error: None,
            }));
        }
        AssertionCacheResult::Miss | AssertionCacheResult::Expired => {
            // Need to verify
        }
    }

    let service = AppAttestService::new(config);
    let result = service
        .verify_assertion(&req.assertion, &req.key_id, &req.client_data, req.expected_counter)
        .await;

    // Cache the result if verification was attempted (success or failure)
    if let Some(counter) = result.counter {
        cache_assertion_result(&req.key_id, &client_data_hash, result.verified, counter);
    }

    Ok(Json(result))
}

/// Maximum request body size (1MB) to prevent memory exhaustion attacks
const MAX_REQUEST_BODY_SIZE: usize = 1024 * 1024;

pub async fn serve(
    addr: SocketAddr,
    db: Database,
    crypto: Arc<HybridCrypto>,
    metrics_handle: PrometheusHandle,
) -> Result<(), std::io::Error> {
    let state = AppState {
        db,
        crypto,
        metrics_handle: Arc::new(metrics_handle),
    };

    // Public unauthenticated GET endpoints: rate-limited via the shared
    // public-tier bucket (60/min, 600/hr per IP). AV-9 mitigation.
    // POST endpoints (register_*) are REGISTRY_ADMIN_TOKEN-gated and skip
    // rate limiting at this layer — admin auth + per-deployment trust is
    // the primary control.
    // Integrity endpoints (/v1/integrity/*) keep their inline verify-tier
    // rate limit (5/min, 50/hr) — stricter because they call external
    // device-attestation APIs.
    let public_rate_limited = Router::new()
        .route("/v1/steward-key", get(steward_key))
        // v1.4 FSD-002 §7.7 — multi-steward + accord-holder discovery
        // (CIRISRegistry#21 + #23 Surface 1 + #16 spec support).
        // CIRISVerify v3.1.0+ consumer wiring at ciris-verify-core::ThresholdMember
        // + verify_threshold_signatures.
        .route("/v1/accord-holders", get(accord_holders))
        .route("/v1/accord/holders", get(accord_holders_ui))
        .route("/v1/rotation-history", get(rotation_history))
        // v1.4 CIRISRegistry#23 Surface 2 — agent_files trust composition
        // (FSD-002 §6.1.6 three-layer canonical/open/vote-then-trust).
        .route("/v1/agent_files/{kind}", get(agent_files_for_kind))
        // v1.4 CIRISRegistry#23 Surface 3 — partner ProfileScorecard composition
        // from partners + revocations + bond + licensure tables.
        .route("/v1/partner/{key_id}", get(partner_composition))
        .route("/v1/revocation/{target_id}", get(check_revocation))
        .route("/v1/verify/binary-manifest/{version}", get(binary_manifest))
        .route(
            "/v1/verify/function-manifest/{version}/{target}",
            get(function_manifest),
        )
        .route(
            "/v1/verify/function-manifests/{version}",
            get(list_function_manifest_targets),
        )
        .route("/v1/builds/{version}", get(get_build_by_version))
        .route("/v1/builds/hash/{build_hash}", get(get_build_by_hash))
        .route("/v1/verify/key/{fingerprint}", get(verify_key_by_fingerprint))
        // Path B (CIRISRegistry#5 §2): verbatim BuildManifest GET. Returns
        // the raw POST body — byte-identical to what the publishing primitive
        // signed — so consumers can verify the original CI signature against
        // canonical bytes without trusting the registry to re-canonicalize.
        .route(
            "/v1/verify/build-manifest/{project}/{version}/{target}",
            get(get_verified_build_manifest),
        )
        .layer(axum::middleware::from_fn(
            crate::middleware::rate_limit::rate_limit_public_http,
        ));

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(readiness))
        .route("/live", get(liveness))
        .route("/metrics", get(metrics))
        // Admin-gated POST endpoints — REGISTRY_ADMIN_TOKEN bearer check
        .route("/v1/verify/binary-manifest", post(register_binary_manifest))
        .route(
            "/v1/verify/function-manifest",
            post(register_function_manifest),
        )
        // Hybrid-signed BuildManifest upload (v1.4.2 — AV-26 mitigation)
        // Body: raw BuildManifest JSON; verified against trusted_primitive_keys
        .route(
            "/v1/verify/build-manifest",
            post(register_verified_build_manifest),
        )
        // Self-signed BuildRecord registration (issue #9). Eliminates the
        // REGISTRY_JWT_SECRET requirement by mirroring the build-manifest
        // auth model: bearer token + content signature against trusted key.
        .route("/v1/builds", post(register_build_http))
        // Play Integrity verification (Android device/app attestation)
        .route("/v1/integrity/nonce", get(integrity_nonce))
        .route("/v1/integrity/verify", post(integrity_verify))
        .route("/v1/integrity/auth", post(integrity_auth))
        // iOS App Attest verification (iOS device/app attestation)
        .route("/v1/integrity/ios/nonce", get(ios_attest_nonce))
        .route("/v1/integrity/ios/verify", post(ios_attest_verify))
        .route("/v1/integrity/ios/assert", post(ios_attest_assert))
        .merge(public_rate_limited)
        .with_state(state)
        // Apply request body size limit (1MB) to all routes
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_BODY_SIZE));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> RegisterBuildHttpRequest {
        RegisterBuildHttpRequest {
            project: "ciris-lens".to_string(),
            version: "1.2.0".to_string(),
            target: "python-source-tree".to_string(),
            build_hash: "sha256:deadbeef".to_string(),
            build_id: "abc123commit".to_string(),
            modules: vec!["core".to_string(), "ios".to_string()],
            file_manifest_count: 7,
            file_manifest_hash: String::new(),
            file_manifest_json: serde_json::Value::Null,
            source_repo: String::new(),
            source_commit: String::new(),
            notes: String::new(),
            signature_classical: "AAAA".to_string(),
            signature_pqc: "BBBB".to_string(),
            signature_key_id: "lens-build-v1".to_string(),
        }
    }

    #[test]
    fn canonical_bytes_excludes_signature_and_metadata() {
        let req = sample_request();
        let bytes = req.canonical_bytes();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert!(!s.contains("signature"), "canonical must not include signature: {s}");
        assert!(!s.contains("AAAA"), "canonical must not include base64 sig: {s}");
        assert!(!s.contains("file_manifest_hash"), "unsigned metadata leaked: {s}");
        assert!(!s.contains("source_repo"), "unsigned metadata leaked: {s}");
        assert!(!s.contains("notes"), "unsigned metadata leaked: {s}");
        // Signed fields must all be present.
        assert!(s.contains(r#""project":"ciris-lens""#));
        assert!(s.contains(r#""version":"1.2.0""#));
        assert!(s.contains(r#""target":"python-source-tree""#));
        assert!(s.contains(r#""build_hash":"sha256:deadbeef""#));
        assert!(s.contains(r#""build_id":"abc123commit""#));
        assert!(s.contains(r#""file_manifest_count":7"#));
    }

    #[test]
    fn canonical_bytes_field_order_is_wire_contract() {
        // Wire contract v2 (CIRISRegistry#11): project, version, target,
        // build_hash, build_id, modules, file_manifest_count. Order must
        // not change without coordinating with CIRISVerify
        // (ciris-build-sign register).
        let req = sample_request();
        let s = String::from_utf8(req.canonical_bytes()).unwrap();
        let positions = [
            ("project", s.find(r#""project""#).unwrap()),
            ("version", s.find(r#""version""#).unwrap()),
            ("target", s.find(r#""target""#).unwrap()),
            ("build_hash", s.find(r#""build_hash""#).unwrap()),
            ("build_id", s.find(r#""build_id""#).unwrap()),
            ("modules", s.find(r#""modules""#).unwrap()),
            ("file_manifest_count", s.find(r#""file_manifest_count""#).unwrap()),
        ];
        for w in positions.windows(2) {
            assert!(
                w[0].1 < w[1].1,
                "field order broken: {} (@{}) must precede {} (@{}); full: {s}",
                w[0].0, w[0].1, w[1].0, w[1].1
            );
        }
    }

    #[test]
    fn canonical_bytes_distinguishes_targets() {
        // Two builds at the same (project, version) with different targets
        // must produce different canonical bytes — that's the whole point
        // of the v2 wire format. CIRISRegistry#11 root cause.
        let mut a = sample_request();
        let mut b = sample_request();
        a.target = "python-source-tree".to_string();
        b.target = "ios-mobile-bundle".to_string();
        assert_ne!(a.canonical_bytes(), b.canonical_bytes());
    }

    #[test]
    fn canonical_bytes_is_deterministic() {
        let req = sample_request();
        assert_eq!(req.canonical_bytes(), req.canonical_bytes());
    }

    #[test]
    fn canonical_bytes_changes_when_signed_field_changes() {
        let mut a = sample_request();
        let bytes_a = a.canonical_bytes();
        a.build_hash = "sha256:cafebabe".to_string();
        let bytes_b = a.canonical_bytes();
        assert_ne!(bytes_a, bytes_b);
    }

    #[test]
    fn canonical_bytes_unchanged_by_unsigned_metadata() {
        let mut req = sample_request();
        let baseline = req.canonical_bytes();
        // Mutating any unsigned metadata field must not affect canonical bytes.
        req.file_manifest_hash = "sha256:zzz".to_string();
        req.file_manifest_json = serde_json::json!({"x": 1});
        req.source_repo = "https://example.test/repo".to_string();
        req.source_commit = "deadbeef".to_string();
        req.notes = "hello".to_string();
        req.signature_key_id = "different".to_string();
        assert_eq!(req.canonical_bytes(), baseline);
    }

    #[test]
    fn sign_then_verify_roundtrip() {
        // End-to-end proof that the canonical bytes a CI tool would sign
        // verify against the same bytes the registry handler would derive.
        // Mirrors build_manifest::tests::verify_signs_and_verifies_roundtrip.
        use ciris_crypto::{
            ClassicalSigner, ClassicalVerifier, Ed25519Signer, Ed25519Verifier, MlDsa65Signer,
            MlDsa65Verifier, PqcSigner, PqcVerifier,
        };
        let ed_signer = Ed25519Signer::random();
        let pqc_signer = MlDsa65Signer::new().unwrap();
        let ed_pk = ed_signer.public_key().unwrap();
        let pqc_pk = pqc_signer.public_key().unwrap();

        let req = sample_request();
        let canonical = req.canonical_bytes();

        let classical_sig = ed_signer.sign(&canonical).unwrap();
        let mut bound = canonical.clone();
        bound.extend_from_slice(&classical_sig);
        let pqc_sig = pqc_signer.sign(&bound).unwrap();

        assert!(Ed25519Verifier::new()
            .verify(&ed_pk, &canonical, &classical_sig)
            .unwrap());
        assert!(MlDsa65Verifier::new()
            .verify(&pqc_pk, &bound, &pqc_sig)
            .unwrap());
    }
}
