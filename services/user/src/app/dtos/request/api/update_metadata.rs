use serde::{Deserialize, Serialize};
use uuid::Uuid;


/// DTO for updating metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMetadataDto {
    pub user_id: Uuid,
    pub key: String,
    pub new_value: String,
}

