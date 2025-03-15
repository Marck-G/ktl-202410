use errors::services::ServiceError;
use uuid::Uuid;

use crate::domain::{repositories::user_repository::UserRepository, user_entity::{Metadata, UserEntity}};

use super::dtos::request::api::search_meta::{SearchByMetadataDto, SearchUserDto};

pub struct SearchService<'a> {
    repo: &'a mut dyn UserRepository,
}

impl<'a> SearchService<'a> {
    pub fn new(repo: &'a mut dyn UserRepository) -> Self {
        Self { repo }
    }

    /// **Search users based on multiple filters**
    pub fn search_users(&mut self, dto: SearchUserDto) -> Result<Vec<UserEntity>, ServiceError> {
        let mut find_object: UserEntity = UserEntity::new(Uuid::nil(), dto.email.unwrap_or_default(), String::new());
        find_object.person = dto.person_id;
        if let Some(verified) = dto.verified {
            find_object.verified = verified;
        }
        find_object.metadata = dto.metadata_filters.unwrap_or_default().into_iter()
        .map(|m| Metadata {
            id: Uuid::nil(),
            key: m.key,
            value: m.value,
            date_created: chrono::NaiveDateTime::UNIX_EPOCH,
            date_modified: chrono::NaiveDateTime::UNIX_EPOCH
        })
        .collect();
        let users = self.repo.filter(find_object)?;

        if users.is_empty() {
            return Err(ServiceError::NotFound("No users found".to_string()));
        }

        Ok(users)
    }

    /// **Search users by specific metadata fields**
    pub fn search_by_metadata(&mut self, metadata_dto: SearchByMetadataDto) -> Result<Vec<UserEntity>, ServiceError> {
        let metadata: Vec<(String, String)> = vec![(metadata_dto.key, metadata_dto.value)];
        let users = self.repo.filter_by_metadata(metadata)?;

        if users.is_empty() {
            return Err(ServiceError::NotFound("No users found with specified metadata".to_string()));
        }

        Ok(users)
    }
}