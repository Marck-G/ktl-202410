#[derive(Debug, Clone)]
pub struct UpdateUserDTO {
    pub verified: Option<bool>,
    pub deleted: Option<bool>
}