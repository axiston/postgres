//! Data layer for managing project schedules.
//!
//! # Tables
//!
//! - project_schedules

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Timestamptz};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::project_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectScheduleCreateInput {
    pub project_id: Uuid,
    pub metadata: Value,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::project_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectScheduleCreateOutput {
    pub id: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::project_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectScheduleViewOutput {
    pub id: Uuid,
    pub project_id: Uuid,
    pub interval: i32,
    pub metadata: Value,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::project_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectScheduleUpdateInput {
    pub metadata: Option<Value>,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ProjectSchedulesRepository;

impl ProjectSchedulesRepository {
    /// Creates a new project schedule.
    pub async fn create_project_schedule(
        conn: &mut AsyncPgConnection,
        schedule_form: &ProjectScheduleCreateInput,
    ) -> DatabaseResult<ProjectScheduleCreateOutput> {
        use schema::project_schedules::dsl::*;

        let query = insert_into(project_schedules)
            .values(schedule_form)
            .returning((id,))
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Retrieves a project schedule by its ID.
    pub async fn view_project_schedule(
        conn: &mut AsyncPgConnection,
        schedule_id: Uuid,
    ) -> DatabaseResult<ProjectScheduleViewOutput> {
        use schema::project_schedules::dsl::*;

        let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
        let query = project_schedules
            .filter(filter_cond)
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Updates a project schedule.
    pub async fn update_project_schedule(
        conn: &mut AsyncPgConnection,
        schedule_id: Uuid,
        form: ProjectScheduleUpdateInput,
    ) -> DatabaseResult<()> {
        use schema::project_schedules::dsl::*;

        let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
        update(project_schedules.filter(filter_cond))
            .set(form)
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Flags the project schedule as deleted.
    pub async fn delete_project_schedule(
        conn: &mut AsyncPgConnection,
        schedule_id: Uuid,
    ) -> DatabaseResult<()> {
        use schema::project_schedules::dsl::*;

        let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
        update(project_schedules.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
