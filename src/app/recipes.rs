use uuid::Uuid;

use crate::model::{NewRecipe, Recipe};
use crate::{AppError, AppState, db};

impl From<NewRecipe> for Recipe {
    fn from(value: NewRecipe) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: value.name,
            prep_time: value.prep_time,
            cook_time: value.cook_time,
        }
    }
}

pub async fn create(state: AppState, new_recipe: NewRecipe) -> Result<Recipe, AppError> {
    let r = new_recipe.into();
    db::insert_recipe(&state.db_pool, &r).await?;

    Ok(r)
}

pub async fn retrieve(state: AppState, id: &Uuid) -> Result<Option<Recipe>, AppError> {
    Ok(db::get_recipe(&state.db_pool, id).await?)
}
