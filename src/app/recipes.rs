use crate::{AppError, AppState, api, app, db};

pub async fn create(
    state: AppState,
    new_recipe: api::model::NewRecipe,
) -> Result<app::model::Recipe, AppError> {
    let r = app::model::Recipe {
        name: new_recipe.name,
    };
    db::queries::insert_recipe(&state.db_pool, &r).await?;

    Ok(r)
}
