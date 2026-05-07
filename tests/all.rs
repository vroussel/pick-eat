mod common;
use common::*;

#[tokio::test]
async fn isalive() {
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
async fn add_recipe() {
    let app = TestApp::new();
    let client = reqwest::Client::new();
    let mut db_conn = app.open_db_conn().await;

    let new_recipe = inputs::NewRecipe {
        name: "pizza 4 fromages".to_string(),
    };

    let body = url_encode_form(&new_recipe).unwrap();
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
    assert_eq!(recipes[0].name, new_recipe.name);
}

#[tokio::test]
async fn add_recipe_with_missing_data() {
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

#[tokio::test]
async fn add_recipe_and_retrieve_it_by_id() {
    let app = TestApp::new();
    let client = reqwest::Client::new();
    let mut db_conn = app.open_db_conn().await;

    let new_recipe = inputs::NewRecipe {
        name: "pizza 4 fromages".to_string(),
    };

    let body = url_encode_form(&new_recipe).unwrap();
    client
        .post(format!("{}/recipes", app.api_base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    let recipe_id = sqlx::query!("SELECT id from recipes")
        .fetch_one(&mut db_conn)
        .await
        .unwrap()
        .id;

    let response = client
        .get(format!("{}/recipes/{recipe_id}", app.api_base_url()))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);
    assert!(
        response.text().await.unwrap().contains(&new_recipe.name),
        "GET /recipes/<id> response body did not contain recipe name"
    );
}
