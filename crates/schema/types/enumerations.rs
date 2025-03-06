//! Implements type-safe enumerations for database queries.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Implements a type-safe `TokenAction` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[ExistingTypePath = "crate::schema::sql_types::TokenAction"]
pub enum TokenAction {
    #[db_rename = "activate_account"]
    #[cfg_attr(feature = "serde", serde(rename = "activate_account"))]
    ActivateAccount,
    #[db_rename = "deactivate_account"]
    #[cfg_attr(feature = "serde", serde(rename = "deactivate_account"))]
    DeactivateAccount,
    #[db_rename = "update_email"]
    #[cfg_attr(feature = "serde", serde(rename = "update_email"))]
    UpdateEmail,
    #[db_rename = "reset_password"]
    #[cfg_attr(feature = "serde", serde(rename = "reset_password"))]
    ResetPassword,
    #[db_rename = "pending_invite"]
    #[cfg_attr(feature = "serde", serde(rename = "pending_invite"))]
    PendingInvite,
}

/// Implements a type-safe `InviteStatus` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[ExistingTypePath = "crate::schema::sql_types::InviteStatus"]
pub enum InviteStatus {
    #[db_rename = "pending"]
    #[cfg_attr(feature = "serde", serde(rename = "pending"))]
    Pending,
    #[db_rename = "accepted"]
    #[cfg_attr(feature = "serde", serde(rename = "accepted"))]
    Accepted,
    #[db_rename = "declined"]
    #[cfg_attr(feature = "serde", serde(rename = "declined"))]
    Declined,
    #[db_rename = "canceled"]
    #[cfg_attr(feature = "serde", serde(rename = "canceled"))]
    Canceled,
}

/// Implements a type-safe `ProjectRole` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[ExistingTypePath = "crate::schema::sql_types::ProjectRole"]
pub enum ProjectRole {
    #[db_rename = "owner"]
    #[cfg_attr(feature = "serde", serde(rename = "owner"))]
    Owner,
    #[db_rename = "member"]
    #[cfg_attr(feature = "serde", serde(rename = "member"))]
    Member,
}
