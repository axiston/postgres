//! Includes all callbacks and hooks for [`DatabaseExt`].
//!
//! [`DatabaseExt`]: crate::DatabaseExt

use diesel_async::pooled_connection::PoolableConnection;
use diesel_async::AsyncPgConnection;

use crate::DatabaseResult;

/// Custom hook called before a connection has been used to run migrations.
///
/// See [`DatabaseExt`] for more details.
///
/// [`DatabaseExt`]: crate::DatabaseExt
pub async fn pre_migrate(conn: &mut AsyncPgConnection) -> DatabaseResult<()> {
    tracing::trace!(
        target: "database",
        is_broken = conn.is_broken(),
        "pre migrate hook is running"
    );

    Ok(())
}

/// Custom hook called after a connection has been used to run migrations.
///
/// See [`DatabaseExt`] for more details.
///
/// [`DatabaseExt`]: crate::DatabaseExt
pub async fn post_migrate(conn: &mut AsyncPgConnection) -> DatabaseResult<()> {
    tracing::trace!(
        target: "database",
        is_broken = conn.is_broken(),
        "post migrate hook is running"
    );

    Ok(())
}
