# Use the official PostgreSQL image as a base image.
# This ensures we're starting from a stable, optimized environment.
FROM postgres:17

# Install pg_cron and its dependencies.
RUN apt-get update
RUN apt-get -y install postgresql-17-cron
RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*

# After the entrypoint calls initdb to create the default postgres user and database,
# it will run any *.sql files, run any executable *.sh scripts, and source any
# non-executable *.sh scripts found in the directory to do further initialization
# before starting the service.
COPY scripts/001_custom_config.sh /docker-entrypoint-initdb.d/001_custom_config.sh
COPY scripts/002_setup_query.sql /docker-entrypoint-initdb.d/002_setup_query.sql

# Grant appropriate permissions and set executable flag for the script.
RUN chmod +x /docker-entrypoint-initdb.d/001_custom_config.sh
