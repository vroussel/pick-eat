use uuid::Uuid;

use crate::{AppError, AppState, api, app, db};

pub async fn create(
    state: AppState,
    new_recipe: api::model::NewRecipe,
) -> Result<app::model::Recipe, AppError> {
    let r = new_recipe.into();
    db::queries::insert_recipe(&state.db_pool, &r).await?;

    Ok(r)
}

pub async fn retrieve(state: AppState, id: &Uuid) -> Result<Option<app::model::Recipe>, AppError> {
    Ok(db::queries::get_recipe(&state.db_pool, id).await?)
}
