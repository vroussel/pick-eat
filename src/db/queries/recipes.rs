use sqlx::PgPool;

use crate::app;

pub async fn insert_recipe(
    db_pool: &PgPool,
    new_recipe: &app::model::Recipe,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO recipes (name)
        VALUES($1)
    "#,
        new_recipe.name
    )
    .execute(db_pool)
    .await?;
    Ok(())
}
