use image::DynamicImage;

#[derive(Debug)]
pub struct NewRecipe {
    pub name: String,
    pub prep_time: u16,
    pub cook_time: u16,
    pub image: DynamicImage,
}
