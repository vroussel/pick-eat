use crate::common::spawn_app;

pub(crate) mod common;

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
