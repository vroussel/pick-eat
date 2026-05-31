use uuid::Uuid;

pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub prep_time: u16,
    pub cook_time: u16,
}
