//! Database layer for CIRISRegistry
//!
//! Uses sqlx with PostgreSQL for persistence.

mod agents;
mod audit;
mod build_attestations;
mod emergency_status;
mod escrows;
mod keys;
mod memberships;
mod organizations;
mod partners;
mod revocations;
mod signing_keys;
mod snapshots;
mod system_users;
mod users;
mod webhooks;

use std::sync::Arc;

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

use crate::config::DatabaseSettings;
use crate::error::Result;

pub use agents::*;
pub use audit::*;
pub use build_attestations::*;
pub use emergency_status::*;
pub use escrows::*;
pub use keys::*;
pub use memberships::*;
pub use organizations::*;
pub use partners::*;
pub use revocations::*;
pub use signing_keys::*;
pub use snapshots::*;
pub use system_users::*;
pub use users::*;
pub use webhooks::*;

/// Database connection pool wrapper
#[derive(Clone)]
pub struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    /// Connect to the database
    pub async fn connect(settings: &DatabaseSettings) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(settings.max_connections)
            .min_connections(settings.min_connections)
            .connect(&settings.connection_string())
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(self.pool.as_ref())
            .await
            .map_err(|e| crate::error::RegistryError::Database(e.into()))?;

        info!("Database migrations completed successfully");
        Ok(())
    }

    /// Get reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Check database health
    pub async fn health_check(&self) -> Result<bool> {
        sqlx::query("SELECT 1")
            .execute(self.pool.as_ref())
            .await?;
        Ok(true)
    }

    /// Get connection pool statistics
    pub fn pool_stats(&self) -> PoolStats {
        PoolStats {
            active: self.pool.size(),
            idle: self.pool.num_idle(),
            max: self.pool.options().get_max_connections(),
        }
    }
}

#[derive(Debug)]
pub struct PoolStats {
    pub active: u32,
    pub idle: usize,
    pub max: u32,
}
