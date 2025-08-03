use pickeat_server::run;
use tokio::net::TcpListener;

#[tokio::test]
async fn isalive_works() {
    spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:4242/isalive")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() {
    let listener = TcpListener::bind("127.0.0.1:4242")
        .await
        .expect("Unable to bind on address");
    tokio::spawn(run(listener));
}
