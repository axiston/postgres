//! Data layer for project member management.
//!
//! # Tables
//!
//! - project_members

use axiston_db_schema::enumerations::ProjectRole;
use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::project_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectMemberCreateInput {
    pub project_id: Uuid,
    pub account_id: Uuid,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::project_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectMemberOutput {
    pub project_id: Uuid,
    pub account_id: Uuid,
    pub show_order: i32,
    pub is_pinned: bool,
    pub is_hidden: bool,
    pub created_by: Uuid,
    pub updated_by: Uuid,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[must_use]
#[derive(Debug, Default, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::project_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectMemberUpdateInput {
    pub show_order: Option<i32>,
    pub is_pinned: Option<bool>,
    pub is_hidden: Option<bool>,
    pub account_role: Option<ProjectRole>,
    pub updated_by: Option<Uuid>,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ProjectMembersRepository;

impl ProjectMembersRepository {
    /// Adds a new member to a project.
    pub async fn create_project_member(
        conn: &mut AsyncPgConnection,
        member_form: &ProjectMemberCreateInput,
    ) -> DatabaseResult<()> {
        use schema::project_members::dsl::*;

        let _query = insert_into(project_members)
            .values(member_form)
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Retrieves a member of a project.
    pub async fn get_project_member(
        conn: &mut AsyncPgConnection,
        form_project_id: Uuid,
        form_account_id: Uuid,
    ) -> DatabaseResult<ProjectMemberOutput> {
        use schema::project_members::dsl::*;

        let filter_cond = project_id
            .eq(form_project_id)
            .and(account_id.eq(form_account_id));

        let query = project_members
            .filter(filter_cond)
            .select(ProjectMemberOutput::as_select())
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Updates a member's details in a project.
    pub async fn update_project_member(
        conn: &mut AsyncPgConnection,
        form_project_id: Uuid,
        form_account_id: Uuid,
        form: ProjectMemberUpdateInput,
    ) -> DatabaseResult<()> {
        use schema::project_members::dsl::*;

        let filter_cond = project_id
            .eq(form_project_id)
            .and(account_id.eq(form_account_id));

        let _query = update(project_members.filter(filter_cond))
            .set(form)
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Deletes a member from a project.
    pub async fn remove_project_member(
        conn: &mut AsyncPgConnection,
        form_project_id: Uuid,
        form_account_id: Uuid,
    ) -> DatabaseResult<()> {
        use schema::project_members::dsl::*;

        let filter_cond = project_id
            .eq(form_project_id)
            .and(account_id.eq(form_account_id));

        let _query = delete(project_members.filter(filter_cond))
            .execute(conn)
            .await?;

        Ok(())
    }
}
