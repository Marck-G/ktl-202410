use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserDTO {
    pub verified: Option<bool>,
    pub deleted: Option<bool>
}