use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// DTO for removing metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveMetadataDto {
    pub user_id: Uuid,
    pub key: String,
}