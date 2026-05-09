use uuid::Uuid;

use crate::{AppError, AppState, api, app, db};

pub async fn create(
    state: AppState,
    new_recipe: api::model::NewRecipe,
) -> Result<app::model::Recipe, AppError> {
    let r = app::model::Recipe {
        id: Uuid::now_v7(),
        name: new_recipe.name,
    };
    db::queries::insert_recipe(&state.db_pool, &r).await?;

    Ok(r)
}

pub async fn retrieve(state: AppState, id: &Uuid) -> Result<Option<app::model::Recipe>, AppError> {
    Ok(db::queries::get_recipe(&state.db_pool, id).await?)
}
