#!/usr/bin/env bash

# Sets default values for environment variables.
HOST="${POSTGRES_HOST:-localhost}"
PORT="${POSTGRES_PORT:-5432}"
USER="${POSTGRES_USER:-postgres}"
TIMEOUT="${POSTGRES_TIMEOUT:-30}"

echo "Waiting for Postgres at ${HOST}:${PORT} with timeout of ${TIMEOUT} seconds."
end=$((SECONDS+TIMEOUT))

# Waits for PostgreSQL to be ready.
while true; do
if pg_isready -h "${HOST}" -p "${PORT}" -U "${USER}"; then
  echo "Postgres is ready!"
  exit 0
fi
  if [ "${SECONDS}" -ge "${end}" ]; then
    echo "Timeout reached. Postgres is not ready." >&2
    exit 1
  fi
  sleep 1
done
