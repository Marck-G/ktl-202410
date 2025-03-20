use serde::{Deserialize, Serialize};

// DTOs for creating and updating users
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserDTO {
    pub email: String,
    pub password: String,  // Plain text password (will be encrypted)
}