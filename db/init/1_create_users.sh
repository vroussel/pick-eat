#!/bin/bash
psql << EOF
CREATE ROLE pickeat WITH
    LOGIN
    CREATEDB
    CREATEROLE
    PASSWORD '$(cat /run/secrets/db_pickeat_password)';

CREATE USER pickeat_app WITH
    LOGIN
    PASSWORD '$(cat /run/secrets/db_pickeat_app_password)';

ALTER DEFAULT PRIVILEGES
    GRANT SELECT, INSERT, UPDATE, DELETE
    ON TABLES
    TO pickeat_app;

ALTER DEFAULT PRIVILEGES
    GRANT USAGE, SELECT
    ON SEQUENCES
    TO pickeat_app;
EOF
