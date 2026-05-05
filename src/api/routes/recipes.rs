use crate::{AppState, api, app};
use axum::{Form, extract::State};

use reqwest::StatusCode;

use crate::AppError;

pub(crate) async fn post(
    State(state): State<AppState>,
    Form(new_recipe): Form<api::model::NewRecipe>,
) -> Result<StatusCode, AppError> {
    app::recipes::create(state, new_recipe).await?;
    Ok(StatusCode::OK)
}
