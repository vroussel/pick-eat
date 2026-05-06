mod common;
use common::TestApp;

#[tokio::test]
async fn isalive_works() {
    let app = TestApp::new();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/isalive", app.api_base_url()))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn add_recipe_returns_200_with_valid_data() {
    let app = TestApp::new();
    let client = reqwest::Client::new();
    let mut db_conn = app.open_db_conn().await;

    let body = "name=pizza%204%20fromages";
    let response = client
        .post(format!("{}/recipes", app.api_base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    let recipes = sqlx::query!("SELECT name from recipes")
        .fetch_all(&mut db_conn)
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(recipes.len(), 1);
    assert_eq!(recipes[0].name, "pizza 4 fromages");
}

#[tokio::test]
async fn add_recipe_returns_422_when_data_is_missing() {
    let app = TestApp::new();
    let client = reqwest::Client::new();
    let mut db_conn = app.open_db_conn().await;

    let test_cases = [("", "missing the name")];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/recipes", app.api_base_url()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        let saved = sqlx::query!("SELECT count(*) from recipes")
            .fetch_one(&mut db_conn)
            .await
            .unwrap();

        assert_eq!(
            response.status().as_u16(),
            422,
            "The API did not fail with HTTP 422 when the payload was {error_message}",
        );
        assert_eq!(saved.count, Some(0));
    }
}
