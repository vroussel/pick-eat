mod common;
use common::*;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = false)]
async fn isalive(admin_db_pool: PgPool) {
    let app = TestApp::new(admin_db_pool).await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/isalive", app.api_base_url()))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[sqlx::test(migrations = false)]
async fn add_recipe(admin_db_pool: PgPool) {
    let app = TestApp::new(admin_db_pool).await;
    let client = reqwest::Client::new();

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
        .fetch_all(app.db_pool())
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(recipes.len(), 1);
    assert_eq!(recipes[0].name, new_recipe.name);
}

#[sqlx::test(migrations = false)]
async fn add_recipe_and_retrieve_it_by_id(admin_db_pool: PgPool) {
    let app = TestApp::new(admin_db_pool).await;
    let client = reqwest::Client::new();

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
        .fetch_one(app.db_pool())
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

#[sqlx::test(migrations = false)]
async fn get_non_existing_page(admin_db_pool: PgPool) {
    let app = TestApp::new(admin_db_pool).await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/yolo", app.api_base_url()))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 404);
}

#[sqlx::test(migrations = false)]
async fn get_non_existing_recipe(admin_db_pool: PgPool) {
    let app = TestApp::new(admin_db_pool).await;
    let client = reqwest::Client::new();

    let random_id = Uuid::now_v7();

    let response = client
        .get(format!("{}/recipes/{random_id}", app.api_base_url()))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 404);
}
