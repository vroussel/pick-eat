use sqlx::PgPool;
use uuid::Uuid;

use crate::app;

pub async fn insert_recipe(
    db_pool: &PgPool,
    new_recipe: &app::model::Recipe,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO recipes (id, name)
        VALUES($1, $2)
    "#,
        new_recipe.id,
        new_recipe.name
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn get_recipe(
    db_pool: &PgPool,
    id: &Uuid,
) -> Result<Option<app::model::Recipe>, sqlx::Error> {
    sqlx::query_as!(
        app::model::Recipe,
        r#"
        SELECT id, name
        FROM recipes
        WHERE id = $1
    "#,
        id
    )
    .fetch_optional(db_pool)
    .await
}
