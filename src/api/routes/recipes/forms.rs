use serde::Deserialize;

use crate::api;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RecipeFormInput {
    pub name: String,
    pub prep_time: String,
    pub cook_time: String,
}

#[derive(Debug, Default)]
pub struct RecipeFormErrors {
    pub name: Option<&'static str>,
    pub prep_time: Option<&'static str>,
    pub cook_time: Option<&'static str>,
}

pub struct RecipeFormValidator {
    pub name: Result<String, &'static str>,
    pub prep_time: Result<u16, &'static str>,
    pub cook_time: Result<u16, &'static str>,
}

impl TryFrom<RecipeFormInput> for api::model::NewRecipe {
    type Error = RecipeFormErrors;

    fn try_from(value: RecipeFormInput) -> Result<Self, Self::Error> {
        let name = match value.name.is_empty() {
            true => Err("Le nom de la recette est obligatoire"),
            false => Ok(value.name),
        };

        let prep_time = match value.prep_time.parse::<i32>() {
            Err(_) => Err("Le temps de préparation doit être un nombre"),
            Ok(v) if v <= 0 => Err("Le temps de préparation doit être supérieur a 0"),
            Ok(v) => match u16::try_from(v) {
                Ok(v) => Ok(v),
                Err(_) => Err("Le temps de préparation est trop elevé"),
            },
        };

        let cook_time = match value.cook_time.parse::<i32>() {
            Err(_) => Err("Le temps de cuisson doit être un nombre"),
            Ok(v) if v <= 0 => Err("Le temps de cuisson doit être supérieur a 0"),
            Ok(v) => match u16::try_from(v) {
                Ok(v) => Ok(v),
                Err(_) => Err("Le temps de cuisson est trop elevé"),
            },
        };

        let rv = RecipeFormValidator {
            name,
            prep_time,
            cook_time,
        };

        match (rv.name, rv.prep_time, rv.cook_time) {
            (Ok(name), Ok(prep_time), Ok(cook_time)) => Ok(api::model::NewRecipe {
                name,
                prep_time,
                cook_time,
            }),
            (name, prep_time, cook_time) => Err(RecipeFormErrors {
                name: name.err(),
                prep_time: prep_time.err(),
                cook_time: cook_time.err(),
            }),
        }
    }
}
