//! Data layer for account management.
//!
//! # Tables
//!
//! - accounts

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
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountCreateInput<'a> {
    pub display_name: &'a str,
    pub email_address: &'a str,
    pub password_hash: &'a str,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountCreateOutput {
    pub id: Uuid,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountViewByIdInput {
    pub id: Uuid,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountViewByEmailInput<'a> {
    pub email_address: &'a str,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountViewOutput {
    pub id: Uuid,

    pub display_name: String,
    pub email_address: String,
    pub password_hash: String,
    pub is_activated: bool,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountUpdateInput<'a> {
    pub display_name: Option<&'a str>,
    pub email_address: Option<&'a str>,
    pub password_hash: Option<&'a str>,
    pub is_activated: Option<bool>,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct AccountsRepository;

impl AccountsRepository {
    /// Creates the new account and returns its unique ID.
    pub async fn create_account(
        conn: &mut AsyncPgConnection,
        form: AccountCreateInput<'_>,
    ) -> DatabaseResult<AccountCreateOutput> {
        use schema::accounts::dsl::*;

        let query = insert_into(accounts)
            .values(form)
            .returning((id, created_at, updated_at, deleted_at))
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Returns the associated account data by its unique ID.
    pub async fn find_account_by_id(
        conn: &mut AsyncPgConnection,
        form: AccountViewByIdInput,
    ) -> DatabaseResult<AccountViewOutput> {
        use schema::accounts::dsl::*;

        let filter_cond = id.eq(form.id).and(deleted_at.is_null());
        let query = accounts
            .filter(filter_cond)
            .select(AccountViewOutput::as_select())
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Returns the account data by its unique email address.
    pub async fn find_account_by_email(
        conn: &mut AsyncPgConnection,
        form: AccountViewByEmailInput<'_>,
    ) -> DatabaseResult<AccountViewOutput> {
        use schema::accounts::dsl::*;

        let filter_cond = email_address
            .eq(form.email_address)
            .and(deleted_at.is_null());
        let query = accounts
            .filter(filter_cond)
            .select(AccountViewOutput::as_select())
            .get_result(conn)
            .await?;

        Ok(query)
    }

    /// Updates the account with provided data.
    pub async fn update_account(
        conn: &mut AsyncPgConnection,
        account_id: Uuid,
        form: AccountUpdateInput<'_>,
    ) -> DatabaseResult<()> {
        use schema::accounts::dsl::*;

        let filter_cond = id.eq(account_id).and(deleted_at.is_null());
        let _query = update(accounts.filter(filter_cond))
            .set(form)
            .execute(conn)
            .await?;

        Ok(())
    }

    /// Flags the provided account as deleted.
    pub async fn delete_account(
        conn: &mut AsyncPgConnection,
        account_id: Uuid,
    ) -> DatabaseResult<()> {
        use schema::accounts::dsl::*;

        let filter_cond = id.eq(account_id).and(deleted_at.is_null());
        let _query = update(accounts.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
