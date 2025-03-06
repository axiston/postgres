//! Data layer for account sessions management.
//!
//! # Tables
//!
//! - account_sessions

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sql_types;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreateInput<'a> {
    pub account_id: Uuid,
    pub region_id: &'a str,
    pub ip_address: IpNet,
    pub user_agent: &'a str,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::account_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionCreateOutput {
    pub account_id: Uuid,
    pub token_seq: Uuid,
    pub update_seq: Uuid,

    pub issued_at: OffsetDateTime,
    pub expired_at: OffsetDateTime,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::account_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionTokenQuery {
    pub account_id: Uuid,
    pub access_seq: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::account_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionToken {
    pub account_id: Uuid,
    pub access_seq: Uuid,

    pub region_id: String,
    pub ip_address: IpNet,
    pub user_agent: String,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct SessionsRepository;

impl SessionsRepository {
    /// Creates the new session and returns the token sequence.
    pub async fn create_session(
        conn: &mut AsyncPgConnection,
        form: SessionCreateInput<'_>,
    ) -> DatabaseResult<SessionCreateOutput> {
        use schema::account_sessions::dsl::*;

        let query = insert_into(account_sessions)
            .values((
                account_id.eq(form.account_id),
                region_id.eq(form.region_id),
                ip_address.eq(form.ip_address),
                user_agent.eq(form.user_agent),
            ))
            .returning((account_id, access_seq, update_seq, issued_at, expired_at))
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Returns the active session.
    pub async fn find_active_session(
        conn: &mut AsyncPgConnection,
        form: SessionTokenQuery,
    ) -> DatabaseResult<Option<SessionToken>> {
        use schema::account_sessions::dsl::*;

        let filter_cond = account_id
            .eq(form.account_id)
            .and(access_seq.eq(form.access_seq))
            .and(expired_at.le(now))
            .and(deleted_at.is_null());

        let query = account_sessions
            .filter(filter_cond)
            .select(SessionToken::as_select())
            .get_result(conn)
            .await
            .optional()?;

        Ok(query)
    }

    /// Returns all active sessions.
    pub async fn view_active_sessions(
        conn: &mut AsyncPgConnection,
        form_account_id: Uuid,
    ) -> DatabaseResult<Vec<SessionToken>> {
        use schema::account_sessions::dsl::*;

        let filter_cond = account_id
            .eq(form_account_id)
            .and(expired_at.le(now))
            .and(deleted_at.is_null());

        let query = account_sessions
            .filter(filter_cond)
            .select(SessionToken::as_select())
            .get_results(conn)
            .await?;

        Ok(query)
    }

    /// Deletes a single active session.
    pub async fn delete_session(
        conn: &mut AsyncPgConnection,
        form: SessionTokenQuery,
    ) -> DatabaseResult<()> {
        use schema::account_sessions::dsl::*;

        let filter_cond = account_id
            .eq(form.account_id)
            .and(access_seq.eq(form.access_seq))
            .and(deleted_at.is_null());

        let _query = update(account_sessions.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Deletes all active sessions except one.
    pub async fn delete_all_sessions(
        conn: &mut AsyncPgConnection,
        form_account_id: Uuid,
        except_access_seq: Option<Uuid>,
    ) -> DatabaseResult<()> {
        use schema::account_sessions::dsl::*;

        let mut filter_cond: Box<dyn BoxableExpression<_, _, SqlType = sql_types::Bool>> =
            Box::new(account_id.eq(form_account_id).and(deleted_at.is_null()));

        if let Some(except_access_seq) = except_access_seq {
            filter_cond = Box::new(filter_cond.and(access_seq.ne(except_access_seq)));
        }

        let _query = update(account_sessions.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
