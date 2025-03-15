use crate::domain::user_entity::{Metadata, UserEntity};
use crate::domain::repositories::user_repository::UserRepository;
use crate::app::dtos::request::api::{add_metadata::AddMetadataDto, update_metadata::UpdateMetadataDto, remove_metadata::RemoveMetadataDto};
use chrono::Utc;
use errors::services::ServiceError;
use uuid::Uuid;


pub struct MetadataService<'a> {
    repo: &'a mut dyn UserRepository,
}

impl<'a> MetadataService<'a> {
    pub fn new(repo: &'a mut dyn UserRepository) -> Self {
        Self { repo }
    }

    /// **Add new metadata using DTO**
    pub fn add_metadata(&mut self, dto: AddMetadataDto) -> Result<bool, ServiceError> {
        let mut user = self.repo.find_one_by_id(dto.user_id)?;

        // Check if metadata key already exists
        if user.metadata.iter().any(|m| m.key == dto.key) {
            return Err(ServiceError::ValidationError("Metadata key already exists".to_string()));
        }

        let metadata = Metadata::new(Uuid::new_v4(), dto.key, dto.value);
        user.metadata.push(metadata);

        self.repo.update(user)?;
        Ok(true)
    }

    /// **Update an existing metadata field using DTO**
    pub fn update_metadata(&mut self, dto: UpdateMetadataDto) -> Result<bool, ServiceError> {
        let mut user = self.repo.find_one_by_id(dto.user_id)?;

        if let Some(metadata) = user.metadata.iter_mut().find(|m| m.key == dto.key) {
            metadata.value = dto.new_value;
            metadata.date_modified = Utc::now().naive_utc();
        } else {
            return Err(ServiceError::NotFound("Metadata key not found".to_string()));
        }

        self.repo.update(user)?;
        Ok(true)
    }

    /// **Remove metadata from a user using DTO**
    pub fn remove_metadata(&mut self, dto: RemoveMetadataDto) -> Result<bool, ServiceError> {
        let mut user = self.repo.find_one_by_id(dto.user_id)?;

        let original_len = user.metadata.len();
        user.metadata.retain(|m| m.key != dto.key);

        if user.metadata.len() == original_len {
            return Err(ServiceError::NotFound("Metadata key not found".to_string()));
        }

        self.repo.update(user)?;
        Ok(true)
    }

    /// **Fetch metadata for a specific user**
    pub fn get_metadata(&mut self, user_id: Uuid) -> Result<Vec<Metadata>, ServiceError> {
        let user = self.repo.find_one_by_id(user_id)?;
        Ok(user.metadata)
    }
}
