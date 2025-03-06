//! Asynchronous `postgres` connection pool and its configuration.

mod custom_hooks;
mod pool_configs;

use std::fmt;

use deadpool::Runtime;
use diesel_async::pooled_connection::deadpool::{Hook, Object, Pool};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::AsyncPgConnection;

use crate::config::custom_hooks::{post_create, post_recycle, pre_recycle, setup_callback};
pub use crate::config::pool_configs::DatabaseConfig;
use crate::DatabaseResult;

/// Asynchronous `postgres` connection pool.
///
/// - Implemented with [`diesel`] and [`deadpool`].
/// - Includes predefined create/recycle hooks.
/// - Emits traces on lifecycle events.
/// - Uses [`DatabaseConfig`] for configuration.
#[derive(Clone)]
pub struct Database {
    conn: Pool<AsyncPgConnection>,
}

impl Database {
    /// Returns a new [`Database`] connection pool.
    pub fn new(addr: impl Into<String>, pool_config: DatabaseConfig) -> Self {
        let mut manager_config = ManagerConfig::default();
        manager_config.custom_setup = Box::new(setup_callback);
        manager_config.recycling_method = pool_config.recycling_method.unwrap_or_default();

        let conn = AsyncDieselConnectionManager::new_with_config(addr, manager_config);
        let pool = Pool::builder(conn)
            .max_size(pool_config.max_conn.unwrap_or(8))
            .create_timeout(pool_config.create_timeout)
            .wait_timeout(pool_config.wait_timeout)
            .recycle_timeout(pool_config.recycle_timeout)
            .post_create(Hook::sync_fn(post_create))
            .pre_recycle(Hook::sync_fn(pre_recycle))
            .post_recycle(Hook::sync_fn(post_recycle))
            .runtime(Runtime::Tokio1);

        let pool = pool.build().expect("should not require runtime");
        Self { conn: pool }
    }

    /// Returns a new [`Database`] connection pool for a single gateway.
    pub fn new_single_gateway(addr: impl Into<String>) -> Self {
        Self::new(addr, DatabaseConfig::new_single_gateway())
    }

    /// Returns a new [`Database`] connection pool for multiple gateways.
    pub fn new_multiple_gateways(addr: impl Into<String>) -> Self {
        Self::new(addr, DatabaseConfig::new_multiple_gateways())
    }

    /// Retrieves a connection from this pool or waits for one to become available.
    pub async fn get_connection(&self) -> DatabaseResult<Object<AsyncPgConnection>> {
        self.conn.get().await.map_err(Into::into)
    }
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = self.conn.status();
        let is_closed = self.conn.is_closed();
        f.debug_struct("Database")
            .field("size", &status.size)
            .field("max_size", &status.max_size)
            .field("available", &status.available)
            .field("waiting", &status.waiting)
            .field("is_closed", &is_closed)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use crate::{Database, DatabaseResult};

    #[tokio::test]
    async fn connect_single_gateway() -> DatabaseResult<()> {
        let addr = "postgresql://postgres:postgres@localhost:5432/postgres";
        let database = Database::new_single_gateway(addr);
        let _conn = database.get_connection().await?;
        Ok(())
    }

    #[tokio::test]
    async fn connect_multiple_gateways() -> DatabaseResult<()> {
        let addr = "postgresql://postgres:postgres@localhost:5432/postgres";
        let database = Database::new_multiple_gateways(addr);
        let _conn = database.get_connection().await?;
        Ok(())
    }
}
