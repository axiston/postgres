//! Asynchronous `postgres` migrator extension.

mod custom_hooks;

use std::ops::DerefMut;

use axiston_db_schema::MIGRATIONS;
use diesel::migration::MigrationSource;
use diesel::pg::Pg;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_migrations::MigrationHarness;
use tokio::task::spawn_blocking;

use crate::migrate::custom_hooks::{post_migrate, pre_migrate};
use crate::{Database, DatabaseError, DatabaseResult};

/// Asynchronous `postgres` migrator extension.
///
/// - Implemented with [`diesel`] and [`deadpool`].
pub trait DatabaseExt {
    /// Executes all pending migrations from the provided source.
    async fn apply_migrations<T>(&self, migrations: T) -> DatabaseResult<u64>
    where
        T: MigrationSource<Pg> + Send + 'static;

    /// Reverts all applied migrations from the provided source.
    async fn rollback_migrations<T>(&self, migrations: T) -> DatabaseResult<u64>
    where
        T: MigrationSource<Pg> + Send + 'static;

    /// Executes all pending migrations from [`MIGRATIONS`].
    async fn apply_default_migrations(&self) -> DatabaseResult<u64> {
        self.apply_migrations(MIGRATIONS).await
    }

    /// Reverts all applied migrations from [`MIGRATIONS`].
    async fn rollback_default_migrations(&self) -> DatabaseResult<u64> {
        self.rollback_migrations(MIGRATIONS).await
    }
}

impl DatabaseExt for Database {
    async fn apply_migrations<T>(&self, migrations: T) -> DatabaseResult<u64>
    where
        T: MigrationSource<Pg> + Send + 'static,
    {
        let conn = self.get_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<_> = conn.into();
        pre_migrate(wrapper.deref_mut()).await?;

        let (migrations, mut wrapper) = spawn_blocking(move || {
            match wrapper
                .run_pending_migrations(migrations)
                .map_err(DatabaseError::Migration)
            {
                Ok(v) => (Ok(v.len() as u64), wrapper),
                Err(x) => (Err(x), wrapper),
            }
        })
        .await
        .unwrap();

        post_migrate(wrapper.deref_mut()).await?;
        migrations
    }

    async fn rollback_migrations<T>(&self, migrations: T) -> DatabaseResult<u64>
    where
        T: MigrationSource<Pg> + Send + 'static,
    {
        let conn = self.get_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<_> = conn.into();
        pre_migrate(wrapper.deref_mut()).await?;

        let (migrations, mut wrapper) = spawn_blocking(move || {
            match wrapper
                .revert_all_migrations(migrations)
                .map_err(DatabaseError::Migration)
            {
                Ok(v) => (Ok(v.len() as u64), wrapper),
                Err(x) => (Err(x), wrapper),
            }
        })
        .await
        .unwrap();

        post_migrate(wrapper.deref_mut()).await?;
        migrations
    }
}

#[cfg(test)]
mod test {
    use crate::{Database, DatabaseExt, DatabaseResult};

    async fn create_database_client() -> DatabaseResult<Database> {
        let addr = "postgresql://postgres:postgres@localhost:5432/postgres";
        let database = Database::new_single_gateway(addr);
        let _ = database.get_connection().await?;
        Ok(database)
    }

    #[tokio::test]
    async fn apply_migrations() -> DatabaseResult<()> {
        let database = create_database_client().await?;
        let _ = database.apply_default_migrations().await?;
        Ok(())
    }

    #[tokio::test]
    async fn rollback_migrations() -> DatabaseResult<()> {
        let database = create_database_client().await?;
        let _ = database.rollback_default_migrations().await?;
        Ok(())
    }
}
