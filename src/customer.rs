#[derive(Debug)]
pub struct CustomerRequestDTO {
    pub id: u32,
    pub name: String,
    pub mail: String,
    pub is_active: bool,
}
