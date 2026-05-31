use sqlx::PgPool;
use uuid::Uuid;

use crate::{app, db};

pub async fn insert_recipe(
    db_pool: &PgPool,
    new_recipe: &app::model::Recipe,
) -> Result<(), sqlx::Error> {
    let row: db::model::RecipeRow = new_recipe.clone().into();
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

pub async fn get_recipe(
    db_pool: &PgPool,
    id: &Uuid,
) -> Result<Option<app::model::Recipe>, sqlx::Error> {
    let row = sqlx::query_as!(
        db::model::RecipeRow,
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
