mod forms;
use forms::*;

use crate::{
    AppState,
    api::{self, model::NewRecipe},
    app,
};
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

#[derive(Template)]
#[template(path = "pages/newrecipe.html")]
struct RecipeForm {
    input: RecipeFormInput,
    errors: RecipeFormErrors,
}

#[axum::debug_handler]
pub(crate) async fn post(
    State(state): State<AppState>,
    Form(new_recipe): Form<RecipeFormInput>,
) -> Result<impl IntoResponse, AppError> {
    debug!("{new_recipe:?}");
    match NewRecipe::try_from(new_recipe.clone()) {
        Ok(v) => {
            let recipe = app::recipes::create(state, v).await?;
            let recipe_uri = format!("/recipes/{}", recipe.id);
            Ok((StatusCode::OK, [("hx-redirect", &recipe_uri)]).into_response())
        }
        Err(e) => {
            let response = Html(
                RecipeForm {
                    input: new_recipe,
                    errors: e,
                }
                .render()?,
            );
            Ok((StatusCode::UNPROCESSABLE_ENTITY, response).into_response())
        }
    }
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

pub async fn new_recipe_form() -> Result<impl IntoResponse, AppError> {
    let response = Html(
        RecipeForm {
            input: RecipeFormInput::default(),
            errors: RecipeFormErrors::default(),
        }
        .render()?,
    );
    Ok(response)
}
