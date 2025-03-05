//! Custom database connection pool configurations.

use std::fmt;
use std::time::Duration;

use deadpool::managed::PoolConfig;
use diesel_async::pooled_connection::RecyclingMethod;
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};

/// Configures [`Database`] for one or more gateways.
///
/// [`Database`]: crate::Database
#[derive(Default, Serialize, Deserialize)]
#[must_use = "configs do nothing unless you use them"]
pub struct DatabaseConfig {
    pub max_conn: Option<usize>,
    pub create_timeout: Option<Duration>,
    pub wait_timeout: Option<Duration>,
    pub recycle_timeout: Option<Duration>,

    #[serde(skip)]
    pub recycling_method: Option<RecyclingMethod<AsyncPgConnection>>,
}

impl DatabaseConfig {
    /// Creates a new [`DatabaseConfig`].
    pub fn new(pool_config: PoolConfig) -> Self {
        Self {
            max_conn: Some(pool_config.max_size),
            create_timeout: pool_config.timeouts.create,
            wait_timeout: pool_config.timeouts.wait,
            recycle_timeout: pool_config.timeouts.recycle,
            recycling_method: None,
        }
    }

    /// Creates a new [`DatabaseConfig`] for a single gateway.
    pub fn new_single_gateway() -> Self {
        Self::default().with_max_conn(64)
    }

    /// Creates a new [`DatabaseConfig`] for multiple gateways.
    pub fn new_multiple_gateways() -> Self {
        Self::default().with_max_conn(8)
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::max_conn`.
    pub fn with_max_conn(mut self, max_conn: usize) -> Self {
        self.max_conn = Some(max_conn);
        self
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::create_timeout`.
    pub fn with_create_timeout(mut self, create_timeout: Duration) -> Self {
        self.create_timeout = Some(create_timeout);
        self
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::wait_timeout`.
    pub fn with_wait_timeout(mut self, wait_timeout: Duration) -> Self {
        self.wait_timeout = Some(wait_timeout);
        self
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::recycle_timeout`.
    pub fn with_recycle_timeout(mut self, recycle_timeout: Duration) -> Self {
        self.recycle_timeout = Some(recycle_timeout);
        self
    }
}

impl From<PoolConfig> for DatabaseConfig {
    #[inline]
    fn from(value: PoolConfig) -> Self {
        Self::new(value)
    }
}

impl fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("max_conn", &self.max_conn)
            .field("create_timeout", &self.create_timeout)
            .field("wait_timeout", &self.wait_timeout)
            .field("recycle_timeout", &self.recycle_timeout)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use crate::{DatabaseConfig, DatabaseResult};

    #[test]
    fn single_gateway() -> DatabaseResult<()> {
        let _ = DatabaseConfig::new_single_gateway();
        Ok(())
    }

    #[test]
    fn multiple_gateways() -> DatabaseResult<()> {
        let _ = DatabaseConfig::new_multiple_gateways();
        Ok(())
    }
}
