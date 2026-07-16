set dotenv-path := "dev/.env"
set dotenv-load

@help:
  just --list --justfile {{justfile()}}

@build-dev-conf:
    [ -f dev/.env ] || (echo "Error: dev/.env not found, copy dev/.env.example to dev/.env" && exit 1)
    @envsubst < dev/pickeat.toml.tmpl > dev/pickeat.toml
    @envsubst < dev/nginx/conf.d/pickeat.conf.tmpl > dev/nginx/conf.d/pickeat.conf

sqlx-prepare:
    cargo sqlx prepare -- --all-targets --all-features
