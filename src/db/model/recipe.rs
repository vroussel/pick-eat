use uuid::Uuid;

use crate::app;

pub struct RecipeRow {
    pub id: Uuid,
    pub name: String,
    pub prep_time: i32,
    pub cook_time: i32,
}

impl From<app::model::Recipe> for RecipeRow {
    fn from(value: app::model::Recipe) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prep_time: value.prep_time as i32,
            cook_time: value.cook_time as i32,
        }
    }
}

impl From<RecipeRow> for app::model::Recipe {
    fn from(value: RecipeRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prep_time: value.prep_time as u16,
            cook_time: value.cook_time as u16,
        }
    }
}
