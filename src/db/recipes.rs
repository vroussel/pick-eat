use sqlx::PgPool;
use uuid::Uuid;

use crate::{images::StoredImage, model};

pub struct RecipeRow {
    pub id: Uuid,
    pub name: String,
    pub prep_time: i32,
    pub cook_time: i32,
    pub image_stem: Option<Uuid>,
    pub image_extension: Option<String>,
}

impl From<model::Recipe> for RecipeRow {
    fn from(value: model::Recipe) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prep_time: value.prep_time as i32,
            cook_time: value.cook_time as i32,
            image_stem: value.image.as_ref().map(|img| img.stem.clone()),
            image_extension: value.image.map(|img| img.extension),
        }
    }
}

impl From<RecipeRow> for model::Recipe {
    fn from(value: RecipeRow) -> Self {
        let image = match (value.image_stem, value.image_extension) {
            (Some(stem), Some(extension)) => Some(StoredImage { stem, extension }),
            _ => None,
        };
        Self {
            id: value.id,
            name: value.name,
            prep_time: value.prep_time as u16,
            cook_time: value.cook_time as u16,
            image,
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
        INSERT INTO recipes (id, name, prep_time, cook_time, image_stem, image_extension)
        VALUES($1, $2, $3, $4, $5, $6)
    "#,
        row.id,
        row.name,
        row.prep_time,
        row.cook_time,
        row.image_stem,
        row.image_extension
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn get_recipe(db_pool: &PgPool, id: &Uuid) -> Result<Option<model::Recipe>, sqlx::Error> {
    let row = sqlx::query_as!(
        RecipeRow,
        r#"
        SELECT id, name, prep_time, cook_time, image_stem, image_extension
        FROM recipes
        WHERE id = $1
    "#,
        id
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(row.map(|r| r.into()))
}
