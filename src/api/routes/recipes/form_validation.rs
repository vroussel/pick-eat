use std::io::Cursor;

use axum::body::Bytes;
use derive_more::Debug;
use image::ImageReader;
use serde::Deserialize;

use crate::images::RawImage;
use crate::model::NewRecipe;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RecipeFormInput {
    pub name: String,
    pub prep_time: String,
    pub cook_time: String,
    #[serde(skip)]
    #[debug(skip)]
    pub image: Bytes,
}

#[derive(Debug, Default)]
pub struct RecipeFormErrors {
    pub name: Option<&'static str>,
    pub prep_time: Option<&'static str>,
    pub cook_time: Option<&'static str>,
    pub image: Option<&'static str>,
}

pub struct RecipeFormValidator {
    pub name: Result<String, &'static str>,
    pub prep_time: Result<u16, &'static str>,
    pub cook_time: Result<u16, &'static str>,
    pub image: Result<RawImage, &'static str>,
}

impl TryFrom<RecipeFormInput> for NewRecipe {
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

        let image_reader = ImageReader::new(Cursor::new(value.image));
        let image = match image_reader.with_guessed_format() {
            Ok(reader) => {
                let format = reader.format().unwrap();
                match reader.decode() {
                    Ok(img) => Ok(RawImage {
                        data: img,
                        ext: format,
                    }),
                    Err(_) => Err("Image invalide"),
                }
            }
            Err(_) => Err("Image invalide"),
        };

        let rv = RecipeFormValidator {
            name,
            prep_time,
            cook_time,
            image,
        };

        match (rv.name, rv.prep_time, rv.cook_time, rv.image) {
            (Ok(name), Ok(prep_time), Ok(cook_time), Ok(image)) => Ok(NewRecipe {
                name,
                prep_time,
                cook_time,
                image,
            }),
            (name, prep_time, cook_time, image) => Err(RecipeFormErrors {
                name: name.err(),
                prep_time: prep_time.err(),
                cook_time: cook_time.err(),
                image: image.err(),
            }),
        }
    }
}
