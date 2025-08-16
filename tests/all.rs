use askama::Template;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};
use tempfile::NamedTempFile;
use uuid::Uuid;

mod test_app {
    use super::*;
    pub struct TestApp {
        cmd: Command,
        port_file: NamedTempFile,
        test_db_name: String,
        _conf_file: NamedTempFile,
    }

    impl TestApp {
        pub fn new() -> Self {
            dotenv::dotenv().ok();

            let bin_path = PathBuf::from(env!("CARGO_BIN_EXE_pickeat-server"));
            let mut cmd = Command::new(bin_path);

            let port_file = NamedTempFile::new().expect("Unable to create temp file");
            cmd.env(
                "TEST_LISTENING_PORT_FILE",
                port_file.path().to_str().unwrap(),
            );

            let test_db_name = Uuid::new_v4().to_string();
            let app_user_password = std::env::var("PG_PICKEAT_APP_PASSWORD")
                .expect("Missing PG_PICKEAT_APP_PASSWORD env var");
            let migration_user_password =
                std::env::var("PG_PICKEAT_PASSWORD").expect("Missing PG_PICKEAT_PASSWORD env var");

            let app_conf = TestAppConf {
                test_db_name: test_db_name.clone(),
                app_user_password,
                migration_user_password,
            };
            let mut conf_file = tempfile::NamedTempFile::new().unwrap();
            app_conf.write_into(&mut conf_file).unwrap();

            cmd.args(["--conf", conf_file.path().to_str().unwrap()]);
            cmd.stdout(Stdio::null());

            Self {
                cmd,
                port_file,
                test_db_name,
                _conf_file: conf_file,
            }
        }
        pub fn get_listening_port(&self) -> Result<u16, &str> {
            let mut port = None;
            for _ in 0..50 {
                if let Ok(content) = fs::read_to_string(&self.port_file)
                    && let Ok(p) = content.trim().parse::<u16>()
                {
                    port = Some(p);
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            port.ok_or("Unable to retrieve app listening port")
        }

        pub fn cmd(&mut self) -> &mut Command {
            &mut self.cmd
        }
    }
}

use test_app::TestApp;

#[derive(Template)]
#[template(path = "../tests/test_conf.toml.j2")]
struct TestAppConf {
    test_db_name: String,
    app_user_password: String,
    migration_user_password: String,
}

#[test]
fn port_file_written() {
    let mut app = TestApp::new();
    let _ = app.cmd().spawn();
    let port = app.get_listening_port();
    assert!(port.is_ok())
}

#[tokio::test]
async fn isalive_works() {
    let mut app = TestApp::new();
    let _ = app.cmd().spawn();
    let port = app.get_listening_port().unwrap();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{port}/isalive"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn add_recipe_returns_200_with_valid_data() {
    let mut app = TestApp::new();
    let _ = app.cmd().spawn();
    let port = app.get_listening_port().unwrap();

    let client = reqwest::Client::new();

    let body = "name=pizza%204%20fromages";
    let response = client
        .post(format!("http://127.0.0.1:{port}/recipes"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn add_recipe_returns_422_when_data_is_missing() {
    let mut app = TestApp::new();
    let _ = app.cmd().spawn();
    let port = app.get_listening_port().unwrap();

    let client = reqwest::Client::new();

    let test_cases = [("", "missing the name")];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("http://127.0.0.1:{port}/recipes"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            response.status().as_u16(),
            422,
            "The API did not fail with HTTP 422 when the payload was {error_message}",
        );
    }
}
