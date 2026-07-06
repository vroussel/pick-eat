use sqlx::PgPool;
use uuid::Uuid;

use crate::model;

pub struct RecipeRow {
    pub id: Uuid,
    pub name: String,
    pub prep_time: i32,
    pub cook_time: i32,
}

impl From<model::Recipe> for RecipeRow {
    fn from(value: model::Recipe) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prep_time: value.prep_time as i32,
            cook_time: value.cook_time as i32,
        }
    }
}

impl From<RecipeRow> for model::Recipe {
    fn from(value: RecipeRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prep_time: value.prep_time as u16,
            cook_time: value.cook_time as u16,
        }
    }
}

pub async fn insert_recipe(
    db_pool: &PgPool,
    new_recipe: &model::Recipe,
) -> Result<(), sqlx::Error> {
    let row: RecipeRow = new_recipe.clone().into();
    sqlx::query!(
        r#"
        INSERT INTO recipes (id, name, prep_time, cook_time)
        VALUES($1, $2, $3, $4)
    "#,
        row.id,
        row.name,
        row.prep_time as i32,
        row.cook_time as i32
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn get_recipe(db_pool: &PgPool, id: &Uuid) -> Result<Option<model::Recipe>, sqlx::Error> {
    let row = sqlx::query_as!(
        RecipeRow,
        r#"
        SELECT id, name, prep_time, cook_time
        FROM recipes
        WHERE id = $1
    "#,
        id
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(row.map(|r| r.into()))
}
