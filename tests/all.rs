use std::net::SocketAddr;

use pickeat_server::run;
use tokio::net::TcpListener;

#[cfg(test)]
pub(crate) async fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Unable to bind on address");
    let addr = listener
        .local_addr()
        .expect("Unable to retrieve server port");
    tokio::spawn(run(listener));
    addr
}

#[tokio::test]
async fn isalive_works() {
    let bind_addr = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{bind_addr}/isalive"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn add_recipe_returns_200_with_valid_data() {
    let app_addr = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=pizza%204%20fromages";
    let response = client
        .post(format!("http://{app_addr}/recipes"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn add_recipe_returns_422_when_data_is_missing() {
    let app_addr = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = [("", "missing the name")];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("http://{app_addr}/recipes"))
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
