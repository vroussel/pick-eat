use std::net::SocketAddr;

use pickeat_server::run;
use tokio::net::TcpListener;

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

async fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Unable to bind on address");
    let addr = listener
        .local_addr()
        .expect("Unable to retrieve server port");
    tokio::spawn(run(listener));
    addr
}
