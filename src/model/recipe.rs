use derive_more::Debug;
use uuid::Uuid;

use crate::images::{RawImage, StoredImage};

#[derive(Debug)]
pub struct NewRecipe {
    pub name: String,
    pub prep_time: u16,
    pub cook_time: u16,
    #[debug(skip)]
    pub image: Option<RawImage>,
}

#[derive(Clone)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub prep_time: u16,
    pub cook_time: u16,
    pub image: Option<StoredImage>,
}
