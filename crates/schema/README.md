### database/schema

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

[action-badge]: https://img.shields.io/github/actions/workflow/status/axiston/axiston/build.yaml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/axiston/axiston/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/axiston-db-schema.svg?logo=rust&style=flat-square
[crates-url]: https://crates.io/crates/axiston-db-schema
[docs-badge]: https://img.shields.io/docsrs/axiston-db-schema?logo=Docs.rs&style=flat-square
[docs-url]: http://docs.rs/axiston-db-schema

**Check out other `axiston` projects [here](https://github.com/axiston).**

Contains the database schema for the application, generated using
[diesel_cli][diesel_cli].

The schema is updated automatically after all migrations are applied during a
GitHub Action triggered by a pull request to the `main` branch. For instructions
on how to regenerate the schema locally, refer to the `Makefile` in the project
root and `build.rs` files.

[diesel_cli]: https://crates.io/crates/diesel_cli

#### Dependencies

- Uses [diesel][diesel] as an object–relational mapping and
  [diesel_migrations][diesel_migrations] for database migrations.
- Depends on [diesel_async][diesel_async] for asynchronous connections and
  [diesel_derive_enum][diesel_derive_enum] to streamline enum handling.

[diesel]: https://crates.io/crates/diesel
[diesel_migrations]: https://crates.io/crates/diesel_migrations
[diesel_async]: https://crates.io/crates/diesel-async/
[diesel_derive_enum]: https://crates.io/crates/diesel-derive-enum
