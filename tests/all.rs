use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};
use tempfile::NamedTempFile;

struct TestApp {
    cmd: Command,
    port_file: NamedTempFile,
}

impl TestApp {
    fn new() -> Self {
        let bin_path = PathBuf::from(env!("CARGO_BIN_EXE_pickeat-server"));
        let mut cmd = Command::new(bin_path);
        cmd.stdout(Stdio::null());
        let port_file = NamedTempFile::new().expect("Unable to create temp file");
        cmd.env(
            "TEST_LISTENING_PORT_FILE",
            port_file.path().to_str().unwrap(),
        );

        Self { cmd, port_file }
    }
    fn get_listening_port(&self) -> Result<u16, &str> {
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
}

#[test]
fn port_file_written() {
    let mut app = TestApp::new();
    let _ = app.cmd.spawn();
    let port = app.get_listening_port();
    assert!(port.is_ok())
}

#[tokio::test]
async fn isalive_works() {
    let mut app = TestApp::new();
    let _ = app.cmd.spawn();
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
    let _ = app.cmd.spawn();
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
    let _ = app.cmd.spawn();
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
