use axum::{Form, extract::State};
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;

use crate::AppError;

#[derive(Debug, Deserialize)]
pub(crate) struct NewRecipe {
    #[allow(dead_code)]
    name: String,
}

pub(crate) async fn post(
    State(db_pool): State<PgPool>,
    Form(new_recipe): Form<NewRecipe>,
) -> Result<StatusCode, AppError> {
    info!("Adding new recipe: {new_recipe:?}");
    sqlx::query!(
        r#"
        INSERT INTO recipes (name)
        VALUES($1)"#,
        new_recipe.name
    )
    .execute(&db_pool)
    .await?;
    Ok(StatusCode::OK)
}
