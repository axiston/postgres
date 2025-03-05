### axiston/postgres

[![Build Status][action-badge]][action-url]
[![DockerHub][docker-badge]][docker-url]

**Check out other `axiston` projects [here](https://github.com/axiston).**

[action-badge]: https://img.shields.io/github/actions/workflow/status/axiston/postgres/build.yaml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/axiston/postgres/actions/workflows/build.yaml
[docker-badge]: https://img.shields.io/docker/automated/_/postgres?style=flat-square&logo=docker&logoColor=white&color=%232496ED
[docker-url]: https://hub.docker.com/u/axiston/postgres

A standard PostgreSQL 17 Docker image with all project-required extensions,
migrations, and a database client including a generated schema.

#### Notes

- See the [official Postgres image] for more details on PostgreSQL configuration
  and features.
- If you already have a running PostgreSQL instance that you want to use with
  this service, you must install, configure, and enable the required extensions
  before starting the gateway. Refer to the [Dockerfile][dockerfile] for
  details.

[official Postgres image]: https://hub.docker.com/_/postgres
[pg_cron]: https://github.com/citusdata/pg_cron
[dockerfile]: ./Dockerfile

#### Dependencies

- Uses [diesel][diesel] as an object–relational mapping and
  [diesel_migrations][diesel_migrations] for database migrations.
- Depends on [diesel_async][diesel_async] for asynchronous connections and
  [diesel_derive_enum][diesel_derive_enum] to streamline enum handling.

[diesel]: https://crates.io/crates/diesel
[diesel_migrations]: https://crates.io/crates/diesel_migrations
[diesel_async]: https://crates.io/crates/diesel-async/
[diesel_derive_enum]: https://crates.io/crates/diesel-derive-enum

#### Guidelines

- Migrations are append-only. Once a migration is merged into the `main` branch,
  do not modify it.
- Migrations in `migration/` must be idempotent, ensuring they can be run
  multiple times without causing issues.
- Self-hosted service users should update role passwords manually after running
  all migrations.
- Production releases are created by publishing a new GitHub release from the
  `main` branch.
