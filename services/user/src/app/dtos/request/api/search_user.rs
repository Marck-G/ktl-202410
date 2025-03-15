use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::search_meta::SearchByMetadataDto;

/// DTO for searching users with multiple filters
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUserDto {
    pub email: Option<String>,
    pub verified: Option<bool>,
    pub person_id: Option<Uuid>,
    pub metadata_filters: Option<Vec<SearchByMetadataDto>>,
}
