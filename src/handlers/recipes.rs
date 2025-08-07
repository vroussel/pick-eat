use axum::{Form, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub(crate) struct NewRecipe {
    #[allow(dead_code)]
    name: String,
}

pub(crate) async fn post(Form(new_recipe): Form<NewRecipe>) -> impl IntoResponse {
    info!("Adding new recipe: {new_recipe:?}");
    StatusCode::OK
}
