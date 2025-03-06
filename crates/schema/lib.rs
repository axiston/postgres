#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

pub use crate::types::{constraints, enumerations};

pub mod schema;
mod types;

/// Embeds all migrations into the final binary.
pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("migrations");
