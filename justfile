set dotenv-load

@help:
  just --list --justfile {{justfile()}}

@build-dev-conf:
    [ -f .env ] || (echo "Error: .env not found, copy .env.example to .env" && exit 1)
    @envsubst < pickeat-dev.toml.tmpl > pickeat-dev.toml

sqlx-prepare:
    cargo sqlx prepare -- --all-targets --all-features
