#[derive(Debug, Clone)]
pub struct UpdateUserDTO {
    pub email: Option<String>,
    pub password: Option<String>,
    pub verified: Option<bool>,
}