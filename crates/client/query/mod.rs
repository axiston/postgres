//! Data access layer (queries, inputs and outputs).

use serde::{Deserialize, Serialize};

pub mod account_permissions;
pub mod account_sessions;
pub mod account_tokens;
pub mod accounts;
pub mod project_invites;
pub mod project_members;
pub mod project_schedules;
pub mod project_webhooks;
pub mod projects;

/// Creates a SQL `ASC` or `DESC` expressions.
///
/// Used to specify the ordering in select methods.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QueryOrderBy {
    /// Creates a SQL `ASC` expression, representing this expression
    /// in the ascending order.
    #[serde(rename = "asc")]
    Ascending,

    /// Creates a SQL `DESC` expression, representing this expression
    /// in the descending order.
    #[serde(rename = "desc")]
    Descending,
}
