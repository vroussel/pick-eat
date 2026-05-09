set dotenv-load

@help:
  just --list --justfile {{justfile()}}

@build-dev-conf:
    [ -f .env ] || (echo "Error: .env not found, copy .env.example to .env" && exit 1)
    @envsubst < config/dev.toml.tmpl > config/dev.toml
