use derive_more::Debug;
use image::{DynamicImage, ImageFormat};

#[derive(Debug)]
pub struct NewRecipe {
    pub name: String,
    pub prep_time: u16,
    pub cook_time: u16,
    #[debug(skip)]
    pub image: RawImage,
}

#[derive(Debug)]
pub struct RawImage {
    pub data: DynamicImage,
    pub ext: ImageFormat,
}
