use crate::{AppState, api, app};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse},
};

use reqwest::StatusCode;
use tracing::debug;
use uuid::Uuid;

use crate::AppError;

pub(crate) async fn post(
    State(state): State<AppState>,
    Form(new_recipe): Form<api::model::NewRecipe>,
) -> Result<StatusCode, AppError> {
    debug!("{new_recipe:?}");
    app::recipes::create(state, new_recipe).await?;
    Ok(StatusCode::OK)
}

#[derive(Template)]
#[template(path = "pages/recipe.html")]
struct RecipePage {
    recipe: app::model::Recipe,
}

pub async fn get(
    State(state): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let recipe = app::recipes::retrieve(state, &recipe_id).await?;

    let response = match recipe {
        Some(r) => (StatusCode::OK, Html(RecipePage { recipe: r }.render()?)),
        None => (StatusCode::NOT_FOUND, api::not_found_page()?),
    };

    Ok(response)
}
