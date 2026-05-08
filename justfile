set dotenv-load

help:
  @just --list --justfile {{justfile()}}

build-dev-conf:
    @[ -f .env ] || (echo "Error: .env not found, copy .env.example to .env" && exit 1)
    envsubst < config/dev.toml.tmpl > config/dev.toml
@clean-test-dbs:
    echo "Dropping all test DBs..."
    psql "$DATABASE_URL" --no-psqlrc --tuples-only --quiet \
        --command "SELECT datname FROM pg_catalog.pg_database WHERE datname like '__test__%'" \
    | xargs -I{} \
        psql "$DATABASE_URL" --no-psqlrc --tuples-only --quiet \
            --command "DROP DATABASE \"{}\""
    echo "Done"
