use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewRecipe {
    pub name: String,
}
