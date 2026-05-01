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
mod services;

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
