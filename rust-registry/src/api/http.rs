//! HTTP/REST gateway for health and metrics endpoints

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::Serialize;

use crate::db::Database;

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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
