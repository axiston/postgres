//! Data layer for project management.
//!
//! # Tables
//!
//! - projects

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
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectCreateInput<'a> {
    pub display_name: &'a str,
    pub metadata: Value,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectCreateOutput {
    pub id: Uuid,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectViewOutput {
    pub id: Uuid,
    pub display_name: String,
    pub metadata: Value,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectUpdateInput<'a> {
    pub display_name: Option<&'a str>,
    pub metadata: Value,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ProjectsRepository;

impl ProjectsRepository {
    /// Creates a new project and returns its details.
    pub async fn create_project(
        conn: &mut AsyncPgConnection,
        project_form: &ProjectCreateInput<'_>,
    ) -> DatabaseResult<ProjectCreateOutput> {
        use schema::projects::dsl::*;

        let query = insert_into(projects)
            .values(project_form)
            .returning((id, created_at, updated_at, deleted_at))
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Retrieves a project by its unique ID.
    pub async fn view_project(
        conn: &mut AsyncPgConnection,
        form_project_id: Uuid,
    ) -> DatabaseResult<ProjectViewOutput> {
        use schema::projects::dsl::*;

        let filter_cond = id.eq(form_project_id).and(deleted_at.is_null());
        let query = projects
            .filter(filter_cond)
            .select(ProjectViewOutput::as_select())
            .limit(1)
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Updates a project's details.
    pub async fn update_project(
        conn: &mut AsyncPgConnection,
        project_id: Uuid,
        form: ProjectUpdateInput<'_>,
    ) -> DatabaseResult<()> {
        use schema::projects::dsl::*;

        let filter_cond = id.eq(project_id).and(deleted_at.is_null());
        let _query = update(projects.filter(filter_cond))
            .set(form)
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Flags the specified project as deleted.
    pub async fn delete_project(
        conn: &mut AsyncPgConnection,
        form_project_id: Uuid,
    ) -> DatabaseResult<()> {
        use schema::projects::dsl::*;

        let filter_cond = id.eq(form_project_id).and(deleted_at.is_null());
        let _query = update(projects.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
