use serde::Serialize;

#[derive(Serialize)]
pub struct NewRecipe {
    pub name: String,
}
