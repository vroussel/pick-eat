use uuid::Uuid;

use crate::model::{NewRecipe, Recipe};
use crate::{AppError, AppState, db};

pub async fn create(state: AppState, new_recipe: NewRecipe) -> Result<Recipe, AppError> {
    let stored_image = new_recipe.image.map(|img| state.image_bank.add_image(img));
    let recipe = Recipe {
        id: Uuid::now_v7(),
        image: stored_image,
        name: new_recipe.name,
        prep_time: new_recipe.prep_time,
        cook_time: new_recipe.cook_time,
    };

    db::insert_recipe(&state.db_pool, &recipe).await?;

    Ok(recipe)
}

pub async fn retrieve(state: &AppState, id: &Uuid) -> Result<Option<Recipe>, AppError> {
    Ok(db::get_recipe(&state.db_pool, id).await?)
}
