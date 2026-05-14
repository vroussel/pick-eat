pub mod queries;

use std::time::Duration;

use sqlx::{
    ConnectOptions, Connection, PgConnection, PgPool, Postgres, migrate,
    migrate::MigrateDatabase,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use tracing::log::LevelFilter;

use crate::conf::DBConf;
pub(crate) async fn init(conf: &DBConf) -> Result<PgPool, sqlx::Error> {
    let db_migration_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        conf.migration_user.name, conf.migration_user.password, conf.host, conf.port, conf.name
    );
    if !Postgres::database_exists(&db_migration_url).await? {
        Postgres::create_database(&db_migration_url).await?;
    }

    let mut migr_conn = PgConnection::connect(&db_migration_url).await?;
    migrate!("db/migrations").run(&mut migr_conn).await?;

    let db_app_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        conf.app_user.name, conf.app_user.password, conf.host, conf.port, conf.name
    );

    let mut opts: PgConnectOptions = db_app_url.parse()?;

    opts = opts
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Warn, Duration::from_millis(500));

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
}
