// DTOs for creating and updating users
#[derive(Debug, Clone)]
pub struct CreateUserDTO {
    pub email: String,
    pub password: String,  // Plain text password (will be encrypted)
}