mod form_validation;

use form_validation::*;

use crate::{
    AppState, api, app,
    images::{ImageBank, ImageSize},
    model::{NewRecipe, Recipe},
};
use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
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
pub async fn post(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut new_recipe = RecipeFormInput::default();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("name") => new_recipe.name = field.text().await.unwrap(),
            Some("prep_time") => new_recipe.prep_time = field.text().await.unwrap(),
            Some("cook_time") => new_recipe.cook_time = field.text().await.unwrap(),
            Some("image") => new_recipe.image = Some(field.bytes().await.unwrap()),
            _ => {}
        }
    }

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
    recipe: Recipe,
    image_bank: ImageBank,
}

pub async fn get(
    State(state): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let recipe = app::recipes::retrieve(&state, &recipe_id).await?;

    let response = match recipe {
        Some(r) => (
            StatusCode::OK,
            Html(
                RecipePage {
                    recipe: r,
                    image_bank: state.image_bank,
                }
                .render()?,
            ),
        ),
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
