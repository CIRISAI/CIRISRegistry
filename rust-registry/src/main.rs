//! CIRISRegistry - Agent and Partner Registry Service
//!
//! This is the Rust implementation of CIRISRegistry, providing:
//! - gRPC services for agent/partner lookup and management
//! - Hybrid cryptographic signatures (Ed25519 + ML-DSA-65)
//! - PostgreSQL storage with sqlx
//! - mTLS and JWT authentication

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod api;
pub mod app_attest;
pub mod capabilities;
mod config;
pub mod crypto;
mod db;
mod error;
mod middleware;
pub mod play_integrity;
pub mod rate_limiter;
pub mod build_manifest;
mod services;
mod validation;

use crate::config::Settings;
use crate::db::Database;
use crate::services::admin::AdminService;
use crate::services::portal::PortalService;
use crate::services::registry::RegistryService;

/// Include generated protobuf code
pub mod proto {
    tonic::include_proto!("ciris.registry.v1");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("ciris_registry_descriptor");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    init_tracing();

    // Initialize uptime tracking
    api::http::init_start_time();

    // Initialize Prometheus metrics recorder
    let metrics_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus recorder");

    info!("Starting CIRISRegistry v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let settings = Settings::from_env()?;
    info!(
        "Loaded configuration for environment: {:?}",
        settings.environment
    );

    // Initialize database connection pool
    let db = Database::connect(&settings.database).await?;
    info!("Connected to database");

    // Run migrations
    db.migrate().await?;
    info!("Database migrations complete");

    // Initialize crypto provider
    let crypto = Arc::new(crypto::HybridCrypto::new(&settings.crypto)?);
    info!("Initialized hybrid cryptography provider");

    // Boot-seed the registry's own steward pubkey as a trusted primitive
    // key (project='ciris-registry') ONLY IF NO ROW EXISTS — gives a
    // working default for fresh installs where the registry's runtime
    // steward identity also signs build manifests, while NOT overwriting
    // a SYSTEM_ADMIN- or CI-set key on every restart.
    //
    // The realistic production posture is for CI to sign build manifests
    // with a separate build-signing identity (GHA secret) distinct from
    // the runtime steward (production filesystem). In that posture the
    // operator runs RegisterTrustedPrimitiveKey once with the GHA pubkey,
    // and this boot-seed becomes a no-op on every subsequent restart.
    //
    // AV-26 mitigation foundation; see THREAT_MODEL.md.
    {
        let ed25519_pk = crypto.ed25519_public_key();
        let mldsa_pk = crypto.mldsa_public_key();
        let ed25519_fp = crypto::HybridCrypto::fingerprint(&ed25519_pk);
        let mldsa_fp = crypto::HybridCrypto::fingerprint(&mldsa_pk);
        match db::insert_trusted_primitive_key_if_absent(
            db.pool(),
            "ciris-registry",
            &ed25519_pk,
            &mldsa_pk,
            &ed25519_fp,
            &mldsa_fp,
            Some("boot-seed"),
            Some("Auto-seeded at boot from steward keypair (overwritten by RegisterTrustedPrimitiveKey)"),
        )
        .await
        {
            Ok(true) => info!(
                ed25519_fp = %ed25519_fp,
                mldsa_fp = %mldsa_fp,
                "Boot-seeded trusted primitive key for project='ciris-registry' (first install)"
            ),
            Ok(false) => info!(
                "Trusted primitive key for project='ciris-registry' already registered \
                 (boot-seed no-op — operator/CI key takes precedence)"
            ),
            Err(e) => tracing::warn!(
                "Failed to boot-seed registry's own trusted primitive key: {}. \
                 Self-verify of registry builds will not work until a SYSTEM_ADMIN \
                 calls RegisterTrustedPrimitiveKey for project='ciris-registry'.",
                e
            ),
        }
    }

    // Create gRPC services
    let registry_service = RegistryService::new(db.clone(), crypto.clone(), settings.environment);
    let admin_service = AdminService::new(db.clone(), crypto.clone(), settings.environment);
    let portal_service = PortalService::new(db.clone(), crypto.clone(), settings.environment);

    // Build gRPC server
    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    info!("Starting gRPC server on {}", grpc_addr);

    let grpc_server = tonic::transport::Server::builder()
        // Order matters: rate_limit FIRST so denied requests skip JWT decode +
        // tracing + metrics overhead. Auth runs second.
        .layer(middleware::rate_limit::RateLimitLayer::new())
        .layer(middleware::auth::AuthLayer::new(settings.auth.clone()))
        .layer(middleware::tracing::TracingLayer)
        .layer(middleware::metrics::MetricsLayer)
        .add_service(reflection_service)
        .add_service(proto::registry_service_server::RegistryServiceServer::new(
            registry_service,
        ))
        .add_service(
            proto::registry_admin_service_server::RegistryAdminServiceServer::new(admin_service),
        )
        .add_service(proto::portal_service_server::PortalServiceServer::new(
            portal_service,
        ))
        .serve_with_shutdown(grpc_addr, shutdown_signal());

    // Background cleanup ticker for the rate-limit HashMaps. Without this,
    // sustained high-cardinality traffic (botnets, IPv6 scans) lets the
    // per-bucket maps grow to MAX_RATE_LIMIT_ENTRIES (10k each across three
    // buckets) before the cap trips. The first request after the cap blocks
    // on an O(N) sweep under the global mutex while every other request
    // blocks. The 60s ticker keeps growth bounded and removes the latency
    // cliff. AV-9 hardening.
    let _cleanup_handle = tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        interval.tick().await; // skip the immediate first tick
        loop {
            interval.tick().await;
            crate::rate_limiter::cleanup_all();
        }
    });

    // Start HTTP gateway for health/metrics (optional)
    let http_handle = if settings.http_port > 0 {
        let http_addr: SocketAddr = format!("0.0.0.0:{}", settings.http_port).parse()?;
        info!("Starting HTTP gateway on {}", http_addr);
        Some(tokio::spawn(api::http::serve(
            http_addr,
            db.clone(),
            crypto.clone(),
            metrics_handle,
        )))
    } else {
        None
    };

    // Run servers
    grpc_server.await?;

    if let Some(handle) = http_handle {
        handle.abort();
    }

    info!("CIRISRegistry shutdown complete");
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().json())
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Received shutdown signal");
}
