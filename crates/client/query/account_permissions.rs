//! Data layer for account permissions management.
//!
//! # Tables
//!
//! - account_permissions

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::DatabaseResult;

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::account_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Permissions {
    pub permissions: Value,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct PermissionsRepository;

impl PermissionsRepository {
    /// Automatically creates or updates permissions.
    pub async fn update_permissions(
        conn: &mut AsyncPgConnection,
        updated_account_id: Uuid,
        form: Permissions,
    ) -> DatabaseResult<()> {
        use schema::account_permissions::dsl::*;

        let _query = insert_into(account_permissions)
            .values((
                account_id.eq(updated_account_id),
                permissions.eq(&form.permissions),
            ))
            .on_conflict(account_id)
            .do_update()
            .set(permissions.eq(&form.permissions))
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Returns the account permissions by account ID.
    pub async fn find_permissions(
        conn: &mut AsyncPgConnection,
        form_account_id: Uuid,
    ) -> DatabaseResult<Permissions> {
        use schema::account_permissions::dsl::*;

        let filter_cond = account_id.eq(form_account_id);
        let query = account_permissions
            .filter(filter_cond)
            .select(Permissions::as_select())
            .get_result(conn)
            .await?;

        Ok(query)
    }
}
