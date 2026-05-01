//! Database layer for CIRISRegistry
//!
//! Uses sqlx with PostgreSQL for persistence.

mod agents;
mod audit;
mod binary_manifests;
mod build_attestations;
mod builds;
mod emergency_status;
mod function_manifests;
mod escrows;
mod keys;
mod memberships;
mod oauth_identities;
mod organizations;
mod partners;
mod revocations;
mod signing_keys;
mod snapshots;
mod system_users;
mod trusted_keys;
mod users;
mod webhooks;

use std::sync::Arc;

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

use crate::config::DatabaseSettings;
use crate::error::Result;

pub use agents::*;
pub use audit::*;
pub use binary_manifests::*;
pub use build_attestations::*;
pub use builds::*;
pub use emergency_status::*;
pub use function_manifests::*;
pub use escrows::*;
pub use keys::*;
pub use memberships::*;
pub use oauth_identities::*;
pub use organizations::*;
pub use partners::*;
pub use revocations::*;
pub use signing_keys::*;
pub use snapshots::*;
pub use system_users::*;
pub use trusted_keys::*;
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

    /// Run database migrations.
    ///
    /// In a Spock multi-master cluster (current production: registry-us ↔
    /// registry-eu), `_sqlx_migrations` would otherwise be replicated as DML,
    /// causing peer nodes to see migrations as already-applied and skip the
    /// DDL on their own postgres. We exclude `_sqlx_migrations` from
    /// replication BEFORE running migrations so each node authoritatively
    /// tracks what it locally executed. Single-node and non-Spock deployments
    /// are unaffected.
    ///
    /// Migrations themselves must be idempotent — see CLAUDE.md "Database
    /// Migration Notes". This is the in-repo half of CIRISRegistry#2.
    pub async fn migrate(&self) -> Result<()> {
        self.exclude_sqlx_migrations_from_spock_replication().await?;

        sqlx::migrate!("./migrations")
            .run(self.pool.as_ref())
            .await
            .map_err(|e| crate::error::RegistryError::Database(e.into()))?;

        info!("Database migrations completed successfully");
        Ok(())
    }

    /// If Spock is loaded, ensure `public._sqlx_migrations` is excluded from
    /// the default replication set so multi-master nodes each track their own
    /// migration history. No-op when Spock isn't present (single-node dev /
    /// staging / test).
    ///
    /// Wrapped in a PL/pgSQL `DO $$ ... EXCEPTION WHEN others ... $$` block
    /// so it is **truly idempotent** on container restarts: the second and
    /// subsequent calls (when the table is already not a member of the
    /// repset) raise an error inside Spock's `repset_remove_table` function
    /// — that error is caught and logged as a NOTICE, the DO block returns
    /// success, and boot proceeds.
    ///
    /// Earlier hotfix attempts matched on substrings of Spock's error
    /// message (`"not a member"`, `"does not exist"`); the actual wording
    /// varies by Spock version, which caused production crash-loops on
    /// container restarts (CIRISRegistry#2 follow-up, observed
    /// 2026-05-01 evening). Catching at the SQL exception layer removes
    /// the dependency on error-string parsing.
    async fn exclude_sqlx_migrations_from_spock_replication(&self) -> Result<()> {
        let spock_loaded: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM pg_extension WHERE extname = 'spock')",
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| crate::error::RegistryError::Database(e.into()))?;

        if !spock_loaded.0 {
            return Ok(());
        }

        sqlx::query(
            r#"
            DO $$
            BEGIN
                PERFORM spock.repset_remove_table('default', 'public._sqlx_migrations');
                RAISE NOTICE 'Spock: removed public._sqlx_migrations from default replication set';
            EXCEPTION
                WHEN others THEN
                    RAISE NOTICE 'Spock repset_remove_table benign error (already excluded?): %', SQLERRM;
            END;
            $$;
            "#,
        )
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| crate::error::RegistryError::Database(e.into()))?;

        info!(
            "Spock detected: ensured public._sqlx_migrations is excluded from default \
             replication set (idempotent — DO block swallows benign errors)."
        );
        Ok(())
    }

    /// Get reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Check database health
    pub async fn health_check(&self) -> Result<bool> {
        sqlx::query("SELECT 1").execute(self.pool.as_ref()).await?;
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
