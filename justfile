set dotenv-load

@help:
  just --list --justfile {{justfile()}}

@setup-dev-env:
    [ -f dev/.env -a -f .env ] || { \
        cp ./dev/.env.example ./dev/.env; \
        ln -nfs ./dev/.env .env; \
    }
    @envsubst < dev/pickeat.toml.tmpl > dev/pickeat.toml
    @envsubst < dev/nginx/conf.d/pickeat.conf.tmpl > dev/nginx/conf.d/pickeat.conf

sqlx-prepare:
    cargo sqlx prepare -- --all-targets --all-features
