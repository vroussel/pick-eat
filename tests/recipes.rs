use crate::common::spawn_app;

pub(crate) mod common;

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
