//! HTTP/REST gateway for health, metrics, and public verification endpoints

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use base64::Engine;
use metrics_exporter_prometheus::PrometheusHandle;
use serde::Serialize;
use tracing::warn;

use crate::db::{self, Database};

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
    // Get active signing key from database
    let key = db::get_active_signing_key(state.db.pool())
        .await
        .map_err(|e| {
            warn!("Failed to query active signing key: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(StewardKeyError {
                    error: "Failed to retrieve signing key".to_string(),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(StewardKeyError {
                    error: "No active signing key configured".to_string(),
                }),
            )
        })?;

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
            key: b64.encode(&key.ed25519_public_key),
            key_id: key.key_id.clone(),
        },
        pqc: PqcKeyInfo {
            algorithm: "ML-DSA-65".to_string(),
            key: b64.encode(&key.mldsa65_public_key),
            key_id: key.key_id.clone(),
            fingerprint: format!("sha256:{}", key.mldsa65_fingerprint),
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

pub async fn serve(
    addr: SocketAddr,
    db: Database,
    metrics_handle: PrometheusHandle,
) -> Result<(), std::io::Error> {
    let state = AppState {
        db,
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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
