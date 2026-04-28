#!/bin/bash
if [ -n "${DB_PICKEAT_PASSWORD_FILE}" ]; then
    DB_PICKEAT_PASSWORD="$(cat "${DB_PICKEAT_PASSWORD_FILE}")"
fi
if [ -n "${DB_PICKEAT_APP_PASSWORD_FILE}" ]; then
    DB_PICKEAT_APP_PASSWORD="$(cat "${DB_PICKEAT_APP_PASSWORD_FILE}")"
fi

if [ -z "$DB_PICKEAT_PASSWORD" ]; then
    >&2 echo "Missing or invalid env var DB_PICKEAT_PASSWORD[_FILE]"
fi
if [ -z "$DB_PICKEAT_APP_PASSWORD" ]; then
    >&2 echo "Missing or invalid env var DB_PICKEAT_APP_PASSWORD[_FILE]"
fi

psql << EOF
CREATE ROLE pickeat WITH
    LOGIN
    CREATEDB
    CREATEROLE
    PASSWORD '${DB_PICKEAT_PASSWORD}';

CREATE ROLE pickeat_app WITH
    LOGIN
    PASSWORD '${DB_PICKEAT_APP_PASSWORD}';
EOF
