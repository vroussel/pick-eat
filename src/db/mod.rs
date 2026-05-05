pub mod queries;

use sqlx::{
    Connection, PgConnection, PgPool, Postgres, migrate, migrate::MigrateDatabase,
    postgres::PgPoolOptions,
};

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

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_app_url)
        .await
}
