//! Comprehensive list of all constraint violations.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Comprehensive list of all constraint violations.
///
/// This includes unique constraint violations as well as foreign key
/// constraint violations for all generated tables.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use = "constraints do nothing unless they are used"]
pub enum ConstraintViolation {
    #[strum(serialize = "accounts_non_empty_display_name")]
    AccountsNonEmptyDisplayName,
    #[strum(serialize = "accounts_non_empty_email_address")]
    AccountsNonEmptyEmailAddress,
    #[strum(serialize = "accounts_non_empty_password_hash")]
    AccountsNonEmptyPasswordHash,
    #[strum(serialize = "accounts_updated_after_created")]
    AccountsUpdatedAfterCreated,
    #[strum(serialize = "accounts_deleted_after_created")]
    AccountsDeletedAfterCreated,
    #[strum(serialize = "accounts_deleted_after_updated")]
    AccountsDeletedAfterUpdated,

    #[strum(serialize = "projects_non_empty_display_name")]
    ProjectsNonEmptyDisplayName,
    #[strum(serialize = "projects_prj_metadata_limit")]
    ProjectsPrjMetadataLimit,
    #[strum(serialize = "projects_updated_after_created")]
    ProjectsUpdatedAfterCreated,
    #[strum(serialize = "projects_deleted_after_created")]
    ProjectsDeletedAfterCreated,
    #[strum(serialize = "projects_deleted_after_updated")]
    ProjectsDeletedAfterUpdated,
}

impl ConstraintViolation {
    /// Creates a new [`ConstraintViolation`] from the constraint name.
    pub fn new(constraint: &str) -> Option<Self> {
        constraint.parse().ok()
    }
}

#[cfg(test)]
mod test {
    use crate::constraints::ConstraintViolation;

    #[test]
    fn parse_constraint_violation() {
        assert_eq!(
            ConstraintViolation::new("accounts_non_empty_display_name"),
            Some(ConstraintViolation::AccountsNonEmptyDisplayName)
        );
        assert_eq!(ConstraintViolation::new("unknown_constraint"), None);
    }

    #[test]
    fn stringify_constraint_violation() {
        assert_eq!(
            ConstraintViolation::AccountsNonEmptyDisplayName.to_string(),
            "accounts_non_empty_display_name"
        );
    }
}
