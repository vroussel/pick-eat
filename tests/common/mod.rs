pub mod inputs;

use askama::Template;
use serde::Serialize;
use sqlx::{PgPool, postgres::PgConnectOptions};
use std::{
    fs,
    path::PathBuf,
    process::{Child, Command},
    time::Duration,
};
use tempfile::NamedTempFile;

pub struct TestApp {
    process: Child,
    db_pool: PgPool,
    api_base_url: String,
    _conf_file: NamedTempFile,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.process.kill().expect("Error while killing TestApp");
    }
}

impl TestApp {
    pub async fn new(admin_db_pool: PgPool) -> Self {
        dotenv::dotenv().ok();

        let bin_path = PathBuf::from(env!("CARGO_BIN_EXE_pickeat-server"));
        let mut cmd = Command::new(bin_path);

        let port_file = NamedTempFile::new().expect("Unable to create temp file");
        cmd.env(
            "TEST_LISTENING_PORT_FILE",
            port_file.path().to_str().unwrap(),
        );
        cmd.env(
            "RUST_LOG",
            std::env::var("RUST_LOG").unwrap_or("off".to_string()),
        );

        let test_db_name = sqlx::query!("SELECT current_database()")
            .fetch_one(&admin_db_pool)
            .await
            .ok()
            .and_then(|r| r.current_database)
            .expect("Unable to retrieve db name from test DB");

        let app_user_password = std::env::var("DB_PICKEAT_APP_PASSWORD")
            .expect("Missing DB_PICKEAT_APP_PASSWORD env var");
        let migration_user_password =
            std::env::var("DB_PICKEAT_PASSWORD").expect("Missing DB_PICKEAT_PASSWORD env var");

        let app_conf = TestAppConf {
            test_db_name,
            app_user_password: app_user_password.clone(),
            migration_user_password,
        };
        let mut conf_file = tempfile::NamedTempFile::new().unwrap();
        app_conf.write_into(&mut conf_file).unwrap();

        cmd.args(["--conf", conf_file.path().to_str().unwrap()]);

        let process = cmd.spawn().expect("Error while running TestApp");
        let port = TestApp::fetch_listening_port(&port_file)
            .expect("Unable to retrieve listening port from temp file");
        let api_base_url = format!("http://127.0.0.1:{port}");

        let app_db_pool = admin_db_pool;
        app_db_pool.set_connect_options(
            PgConnectOptions::new()
                .username("pickeat_app")
                .password(&app_user_password),
        );

        Self {
            process,
            db_pool: app_db_pool,
            api_base_url,
            _conf_file: conf_file,
        }
    }

    fn fetch_listening_port(port_file: &NamedTempFile) -> Result<u16, &'static str> {
        let mut port = None;

        for _ in 0..50 {
            if let Ok(content) = fs::read_to_string(port_file)
                && let Ok(p) = content.trim().parse::<u16>()
            {
                port = Some(p);
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        port.ok_or("Unable to retrieve app listening port")
    }

    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }

    pub fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }
}

/// ```askama
/// [http]
/// ip = "127.0.0.1"
/// port = 0
///
/// [db]
/// host = "localhost"
/// port = 5432
/// name = "{{ test_db_name }}"
///
/// [db.app_user]
/// name = "pickeat_app"
/// password = "{{ app_user_password }}"
///
/// [db.migration_user]
/// name = "pickeat"
/// password = "{{ migration_user_password }}"
///
/// [images]
/// storage_root = "/tmp/images"
/// url_prefix = "/images/"
/// ```
#[derive(Template)]
#[template(ext = "txt", in_doc = true)]
struct TestAppConf {
    test_db_name: String,
    app_user_password: String,
    migration_user_password: String,
}
