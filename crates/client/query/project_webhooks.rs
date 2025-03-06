//! Data layer for managing project webhooks.
//!
//! # Tables
//!
//! - project_webhooks

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::project_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectWebhookCreateInput {
    pub project_id: Uuid,
    pub metadata: Value,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::project_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectWebhookCreateOutput {
    pub id: Uuid,
    pub project_id: Uuid,
    pub metadata: Value,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::project_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectWebhookUpdateInput {
    pub metadata: Option<Value>,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ProjectWebhooksRepository;

impl ProjectWebhooksRepository {
    /// Creates a new project webhook.
    pub async fn create_project_webhook(
        conn: &mut AsyncPgConnection,
        form: &ProjectWebhookCreateInput,
    ) -> DatabaseResult<ProjectWebhookCreateOutput> {
        use schema::project_webhooks::dsl::*;

        let query = insert_into(project_webhooks)
            .values(form)
            .returning((id, project_id, metadata, created_at, updated_at, deleted_at))
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Retrieves a project webhook by its ID.
    pub async fn view_project_webhook(
        conn: &mut AsyncPgConnection,
        webhook_id: Uuid,
    ) -> DatabaseResult<ProjectWebhookCreateOutput> {
        use schema::project_webhooks::dsl::*;

        let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
        let query = project_webhooks
            .filter(filter_cond)
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Updates a project webhook.
    pub async fn update_project_webhook(
        conn: &mut AsyncPgConnection,
        webhook_id: Uuid,
        form: ProjectWebhookUpdateInput,
    ) -> DatabaseResult<()> {
        use schema::project_webhooks::dsl::*;

        let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
        update(project_webhooks.filter(filter_cond))
            .set(form)
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Flags the project webhook as deleted.
    pub async fn delete_project_webhook(
        conn: &mut AsyncPgConnection,
        webhook_id: Uuid,
    ) -> DatabaseResult<()> {
        use schema::project_webhooks::dsl::*;

        let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
        update(project_webhooks.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
