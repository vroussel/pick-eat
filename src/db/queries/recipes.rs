use sqlx::PgPool;

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
