//! Data layer for project invitations.
//!
//! # Tables
//!
//! - project_invites

use axiston_db_schema::enumerations::InviteStatus;
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
#[diesel(table_name = schema::project_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectInviteCreateInput {
    pub project_id: Uuid,
    pub account_id: Uuid,
    pub created_by: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::project_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectInviteCreateOutput {
    pub project_id: Uuid,
    pub invite_id: Uuid,
    pub status: InviteStatus,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::project_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectInviteViewOutput {
    pub project_id: Uuid,
    pub invite_id: Uuid,
    pub invite_status: InviteStatus,
    pub created_by: Uuid,
    pub updated_by: Uuid,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::project_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectInviteUpdateInput {
    pub invite_status: InviteStatus,
    pub updated_by: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ProjectInvitesRepository;

impl ProjectInvitesRepository {
    /// Creates a new project invitation.
    pub async fn create_project_invite(
        conn: &mut AsyncPgConnection,
        invite_form: &ProjectInviteCreateInput,
    ) -> DatabaseResult<ProjectInviteCreateOutput> {
        use schema::project_invites::dsl::*;

        let query = insert_into(project_invites)
            .values(invite_form)
            .returning((project_id, invite_id, invite_status, created_at, updated_at))
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Updates the status of a project invitation.
    pub async fn update_project_invite(
        conn: &mut AsyncPgConnection,
        project_id_val: Uuid,
        invite_id_val: Uuid,
        form: ProjectInviteUpdateInput,
    ) -> DatabaseResult<()> {
        use schema::project_invites::dsl::*;

        let filter_cond = project_id
            .eq(project_id_val)
            .and(invite_id.eq(invite_id_val));
        let _query = update(project_invites.filter(filter_cond))
            .set(form)
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Retrieves an invitation by project and invite ID.
    pub async fn view_project_invite(
        conn: &mut AsyncPgConnection,
        form_project_id: Uuid,
        invite_id_val: Uuid,
    ) -> DatabaseResult<ProjectInviteViewOutput> {
        use schema::project_invites::dsl::*;

        let filter_cond = project_id
            .eq(form_project_id)
            .and(invite_id.eq(invite_id_val));
        let query = project_invites
            .filter(filter_cond)
            .select(ProjectInviteViewOutput::as_select())
            .get_result(conn)
            .await?;

        Ok(query)
    }
}
