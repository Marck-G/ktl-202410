use errors::services::ServiceError;
use uuid::Uuid;

use crate::domain::{dtos::request::api::{user_create::CreateUserDTO, user_update::UpdateUserDTO}, repositories::user_repository::UserRepository, user_entity::UserEntity};

pub struct UserService<'a> {
    repo: &'a mut dyn UserRepository,
}

impl<'a> UserService<'a> {
    pub fn new(repo: &'a mut dyn UserRepository) -> Self {
        Self { repo }
    }

    /// Create a new user
    pub fn create_user(&mut self, dto: CreateUserDTO) -> Result<UserEntity, ServiceError> {

        let user = UserEntity::new(Uuid::new_v4(), dto.email, dto.password);

        self.repo.create(user.clone())?;
        Ok(user)
    }

    /// Get a user by ID
    pub fn get_user_by_id(&mut self, user_id: Uuid) -> Result<UserEntity, ServiceError> {
        self.repo.find_one_by_id(user_id).map_err(ServiceError::from)
    }

    /// Update a user
    pub fn update_user(&mut self, user_id: Uuid, dto: UpdateUserDTO) -> Result<UserEntity, ServiceError> {
        let mut user = self.repo.find_one_by_id(user_id)?;

        // if let Some(deleted) = dto.deleted {
        //     user.deleted = deleted;
        // }
        if let Some(verified) = dto.verified {
            user.verified = verified;
        }

        self.repo.update(user.clone())?;
        Ok(user)
    }

    /// Delete a user
    pub fn delete_user(&mut self, user_id: Uuid) -> Result<bool, ServiceError> {
        self.repo.soft_delete(user_id).map_err(ServiceError::from)
    }

    /// List users with pagination
    pub fn list_users(&mut self, page: i32, limit: i32) -> Result<Vec<UserEntity>, ServiceError> {
        self.repo.list(page, limit).map_err(ServiceError::from)
    }

    /// Change user password
    pub fn change_password(&mut self, user_id: Uuid, new_password: String) -> Result<bool, ServiceError> {
        let user = self.repo.find_one_by_id(user_id)?;
        // Update user password
        let mut mod_user: UserEntity = UserEntity::new(user.get_id(), user.get_email(), new_password);
        mod_user.date_created = user.date_created;
        mod_user.date_modified = user.date_modified;
        mod_user.verified = user.verified;
        mod_user.person = user.person;
        mod_user.metadata = user.metadata;
        // mod_user.deleted = user.deleted;
        let result: bool = self.repo.update(mod_user)?;
        Ok(result)
    }
}