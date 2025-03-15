use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// DTO for adding new metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct AddMetadataDto {
    pub user_id: Uuid,
    pub key: String,
    pub value: String,
}

