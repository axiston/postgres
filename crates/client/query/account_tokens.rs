//! Data layer for account tokens management.
//!
//! # Tables
//!
//! - account_tokens

use axiston_db_schema::enumerations::TokenAction;
use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DatabaseResult;

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountTokenCreateInput {
    pub account_id: Uuid,
    pub action_type: TokenAction,
    pub action_data: serde_json::Value,
    pub ip_address: IpNet,
    pub user_agent: String,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountTokenCreateOutput {
    pub action_token: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTokenViewInput {
    pub account_id: Uuid,
    pub action_token: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountTokenViewOutput {
    pub action_type: TokenAction,
    pub action_data: serde_json::Value,
    pub ip_address: IpNet,
    pub user_agent: String,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct AccountTokensRepository;

impl AccountTokensRepository {
    /// Creates and returns the new action token.
    pub async fn create_action_token(
        conn: &mut AsyncPgConnection,
        form: AccountTokenCreateInput,
    ) -> DatabaseResult<AccountTokenCreateOutput> {
        use schema::account_tokens::dsl::*;

        let query = insert_into(account_tokens)
            .values(form)
            .returning(action_token)
            .get_result(conn)
            .await?;

        Ok(AccountTokenCreateOutput {
            action_token: query,
        })
    }

    /// Flags the action token as used and returns the action data.
    pub async fn consume_action_token(
        conn: &mut AsyncPgConnection,
        form: AccountTokenViewInput,
    ) -> DatabaseResult<AccountTokenViewOutput> {
        use schema::account_tokens::dsl::*;

        let filter_cond = account_id
            .eq(form.account_id)
            .and(action_token.eq(form.action_token))
            .and(used_at.is_null());

        let query = update(account_tokens.filter(filter_cond))
            .set(used_at.eq(now))
            .returning((action_type, action_data, ip_address, user_agent))
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// TODO.
    pub async fn delete_all_tokens(
        conn: &mut AsyncPgConnection,
        form_account_id: Uuid,
    ) -> DatabaseResult<()> {
        use schema::account_tokens::dsl::*;

        let filter_cond = account_id.eq(form_account_id).and(used_at.is_null());
        let _query = update(account_tokens.filter(filter_cond))
            .set(used_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
