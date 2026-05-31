use uuid::Uuid;

use crate::api;

#[derive(Clone)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub prep_time: u16,
    pub cook_time: u16,
}

impl From<api::model::NewRecipe> for Recipe {
    fn from(value: api::model::NewRecipe) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: value.name,
            prep_time: value.prep_time,
            cook_time: value.cook_time,
        }
    }
}
