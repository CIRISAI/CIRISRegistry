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
use base64::Engine;
use metrics_exporter_prometheus::PrometheusHandle;
use serde::Serialize;
use tracing::warn;

use crate::crypto::HybridCrypto;
use crate::db::{self, BuildRow, Database, get_build};
use crate::play_integrity::{
    IntegrityAuthRequest, IntegrityAuthResponse, IntegrityVerifyRequest, PlayIntegrityConfig,
    PlayIntegrityService,
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

/// Response from the /v1/steward-key endpoint.
/// This format matches what CIRISVerify expects (see ciris-verify-core/src/https.rs).
#[derive(Serialize)]
struct StewardKeyResponse {
    classical: ClassicalKeyInfo,
    pqc: PqcKeyInfo,
    signature_mode: String,
    revision: u64,
    timestamp: i64,
    next_rotation: Option<i64>,
    response_signature_classical: Option<String>,
    response_signature_pqc: Option<String>,
}

#[derive(Serialize)]
struct ClassicalKeyInfo {
    algorithm: String,
    key: String,
    key_id: String,
}

#[derive(Serialize)]
struct PqcKeyInfo {
    algorithm: String,
    key: String,
    key_id: String,
    fingerprint: String,
}

#[derive(Serialize)]
struct StewardKeyError {
    error: String,
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
/// Returns the active registry signing key for CIRISVerify multi-source validation.
/// This endpoint is unauthenticated — it serves public key material only.
async fn steward_key(
    State(state): State<AppState>,
) -> Result<Json<StewardKeyResponse>, (StatusCode, Json<StewardKeyError>)> {
    // Use the crypto provider's keys directly (works with Vault, file, or memory mode)
    let ed25519_pubkey = state.crypto.ed25519_public_key();
    let mldsa_pubkey = state.crypto.mldsa_public_key();
    let key_id = state.crypto.key_id().to_string();

    // Compute fingerprint for ML-DSA-65 public key
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&mldsa_pubkey);
    let mldsa_fingerprint = hex::encode(hasher.finalize());

    // Get revocation list revision (max id)
    let revision: u64 = sqlx::query_scalar::<_, Option<i32>>("SELECT MAX(id) FROM revocations")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(Some(0))
        .unwrap_or(0) as u64;

    let b64 = base64::engine::general_purpose::STANDARD;
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    Ok(Json(StewardKeyResponse {
        classical: ClassicalKeyInfo {
            algorithm: "Ed25519".to_string(),
            key: b64.encode(&ed25519_pubkey),
            key_id: key_id.clone(),
        },
        pqc: PqcKeyInfo {
            algorithm: "ML-DSA-65".to_string(),
            key: b64.encode(&mldsa_pubkey),
            key_id: key_id.clone(),
            fingerprint: format!("sha256:{}", mldsa_fingerprint),
        },
        signature_mode: "HYBRID_REQUIRED".to_string(),
        revision,
        timestamp: now,
        next_rotation: None,
        response_signature_classical: None,
        response_signature_pqc: None,
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

/// Public endpoint: GET /v1/verify/binary-manifest/{version}
///
/// Returns SHA-256 hashes of CIRISVerify binaries for self-verification (Level 2).
/// This endpoint is unauthenticated.
async fn binary_manifest(
    State(state): State<AppState>,
    axum::extract::Path(version): axum::extract::Path<String>,
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

    match db::get_binary_manifest(state.db.pool(), &version).await {
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

    // Parse generated_at timestamp
    let generated_at = time::OffsetDateTime::parse(
        &req.generated_at,
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap_or_else(|_| time::OffsetDateTime::now_utc());

    // Convert binaries map to JSON
    let binaries_json = serde_json::to_value(&req.binaries).unwrap_or(serde_json::json!({}));

    // Register the manifest
    match db::register_binary_manifest(
        state.db.pool(),
        &req.version,
        &binaries_json,
        generated_at,
        Some("ci_push"),
        Some("ci_push"),
        req.notes.as_deref(),
    )
    .await
    {
        Ok(manifest_id) => Ok(Json(RegisterBinaryManifestResponse {
            success: true,
            manifest_id,
            message: format!("Binary manifest registered for version {}", req.version),
        })),
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

/// Public endpoint: GET /v1/builds/{version}
///
/// Returns a build record by version. Used by CIRISVerify for file integrity verification.
/// This endpoint is unauthenticated.
async fn get_build_by_version(
    State(state): State<AppState>,
    axum::extract::Path(version): axum::extract::Path<String>,
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

    match get_build(state.db.pool(), Some(&version), None).await {
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

    match get_build(state.db.pool(), None, Some(&build_hash)).await {
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

/// Public endpoint: GET /v1/verify/function-manifest/{version}/{target}
///
/// Returns a function manifest for runtime verification.
async fn function_manifest(
    State(state): State<AppState>,
    axum::extract::Path((version, target)): axum::extract::Path<(String, String)>,
) -> Result<Json<db::FunctionManifestResponse>, (StatusCode, Json<FunctionManifestError>)> {
    match db::get_function_manifest(state.db.pool(), &version, &target).await {
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

/// Public endpoint: GET /v1/verify/function-manifests/{version}
///
/// Lists available targets for a version.
async fn list_function_manifest_targets(
    State(state): State<AppState>,
    axum::extract::Path(version): axum::extract::Path<String>,
) -> Result<Json<db::AvailableTargetsResponse>, (StatusCode, Json<FunctionManifestError>)> {
    match db::list_function_manifest_targets(state.db.pool(), &version).await {
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

    // Parse generated_at timestamp
    let generated_at = time::OffsetDateTime::parse(
        &req.generated_at,
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap_or_else(|_| time::OffsetDateTime::now_utc());

    // Build manifest JSON (store the full manifest for serving)
    let manifest_json = serde_json::json!({
        "version": req.version,
        "target": req.target,
        "binary_hash": req.binary_hash,
        "binary_version": req.binary_version,
        "generated_at": req.generated_at,
        "functions": req.functions,
    });

    // Extract signature fields
    let (sig_classical, sig_pqc, sig_key_id) = if let Some(sig) = &req.signature {
        (
            Some(sig.classical.as_str()),
            Some(sig.pqc.as_str()),
            Some(sig.key_id.as_str()),
        )
    } else {
        (None, None, None)
    };

    // Register the manifest
    match db::register_function_manifest(
        state.db.pool(),
        &req.binary_version,
        &req.target,
        &req.version,
        &req.binary_hash,
        &req.manifest_hash,
        &manifest_json,
        sig_classical,
        sig_pqc,
        sig_key_id,
        generated_at,
    )
    .await
    {
        Ok(manifest_hash) => Ok(Json(RegisterFunctionManifestResponse {
            success: true,
            manifest_hash,
            message: format!(
                "Function manifest registered for version {} target {}",
                req.binary_version, req.target
            ),
        })),
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

/// Public endpoint: GET /v1/integrity/nonce
///
/// Generate a cryptographically secure nonce for Play Integrity verification.
/// The nonce is single-use and expires in 5 minutes.
async fn integrity_nonce(
    axum::extract::Query(params): axum::extract::Query<IntegrityNonceParams>,
) -> Json<crate::play_integrity::IntegrityNonceResponse> {
    let config = PlayIntegrityConfig::default();
    let service = PlayIntegrityService::new(config);
    Json(service.generate_nonce(params.context.as_deref()))
}

#[derive(serde::Deserialize)]
struct IntegrityNonceParams {
    context: Option<String>,
}

/// Public endpoint: POST /v1/integrity/verify
///
/// Verify a Play Integrity token. Decodes the token via Google's API
/// and returns device/app/account verdicts.
async fn integrity_verify(
    Json(req): Json<IntegrityVerifyRequest>,
) -> Result<Json<crate::play_integrity::IntegrityVerifyResponse>, (StatusCode, Json<IntegrityError>)>
{
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
///
/// Note: This endpoint expects a Bearer token for JWT auth, but since
/// Registry doesn't have user auth, it just validates the integrity token
/// and returns auth status based on any provided context.
async fn integrity_auth(
    headers: HeaderMap,
    Json(req): Json<IntegrityAuthRequest>,
) -> Result<Json<IntegrityAuthResponse>, (StatusCode, Json<IntegrityError>)> {
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

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(readiness))
        .route("/live", get(liveness))
        .route("/metrics", get(metrics))
        // Public verification endpoints (consumed by CIRISVerify)
        .route("/v1/steward-key", get(steward_key))
        .route("/v1/revocation/{target_id}", get(check_revocation))
        // Binary manifests (whole-binary hashes for Level 2 self-check)
        .route("/v1/verify/binary-manifest/{version}", get(binary_manifest))
        .route("/v1/verify/binary-manifest", post(register_binary_manifest))
        // Function manifests (function-level hashes for runtime verification)
        .route(
            "/v1/verify/function-manifest/{version}/{target}",
            get(function_manifest),
        )
        .route(
            "/v1/verify/function-manifests/{version}",
            get(list_function_manifest_targets),
        )
        .route(
            "/v1/verify/function-manifest",
            post(register_function_manifest),
        )
        // Build records (CIRISAgent file integrity manifests for CIRISVerify)
        .route("/v1/builds/{version}", get(get_build_by_version))
        .route("/v1/builds/hash/{build_hash}", get(get_build_by_hash))
        // Key verification (agent signing key validation by fingerprint)
        .route("/v1/verify/key/{fingerprint}", get(verify_key_by_fingerprint))
        // Play Integrity verification (Android device/app attestation)
        .route("/v1/integrity/nonce", get(integrity_nonce))
        .route("/v1/integrity/verify", post(integrity_verify))
        .route("/v1/integrity/auth", post(integrity_auth))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
