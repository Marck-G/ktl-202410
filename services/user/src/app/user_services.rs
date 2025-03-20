use std::sync::Arc;

use errors::services::ServiceError;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::app::dtos::request::api::{user_create::CreateUserDTO, user_update::UpdateUserDTO};

use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::user_entity::UserEntity;
use crate::infra::postgres::repositories::PgUserRepository;

pub struct UserService {
    repo: Arc<Mutex<PgUserRepository>>,
}

impl UserService {
    pub async fn new(repo: Arc<Mutex<PgUserRepository>>) -> Self {
        Self { repo }
    }

    /// Create a new user
    pub async fn create_user(&mut self, dto: CreateUserDTO) -> Result<UserEntity, ServiceError> {

        let user = UserEntity::new(Uuid::new_v4(), dto.email, dto.password);
        let mut repo = self.repo.lock().await;
        repo.create(user.clone()).await?;
        Ok(user)
    }

    /// Get a user by ID
    pub async fn get_user_by_id(&mut self, user_id: Uuid) -> Result<UserEntity, ServiceError> {
        let mut repo = self.repo.lock().await;
        repo.find_one_by_id(user_id).await.map_err(ServiceError::from)
    }

    /// Update a user
    pub async fn update_user(&mut self, user_id: Uuid, dto: UpdateUserDTO) -> Result<UserEntity, ServiceError> {
        let mut repo = self.repo.lock().await;
        let mut user = repo.find_one_by_id(user_id).await?;

        // if let Some(deleted) = dto.deleted {
        //     user.deleted = deleted;
        // }
        if let Some(verified) = dto.verified {
            user.verified = verified;
        }

        repo.update(user.clone()).await?;
        Ok(user)
    }

    /// Delete a user
    pub async fn delete_user(&mut self, user_id: Uuid) -> Result<bool, ServiceError> {
        let mut repo = self.repo.lock().await;
        repo.soft_delete(user_id).await.map_err(ServiceError::from)
    }

    /// List users with pagination
    pub async fn list_users(&mut self, page: i32, limit: i32) -> Result<Vec<UserEntity>, ServiceError> {
        let mut repo = self.repo.lock().await;
        repo.list(page, limit).await.map_err(ServiceError::from)
    }

    /// Change user password
    pub async fn change_password(&mut self, user_id: Uuid, new_password: String) -> Result<bool, ServiceError> {
        let mut repo = self.repo.lock().await;
        let user = repo.find_one_by_id(user_id).await?;
        // Update user password
        let mut mod_user: UserEntity = UserEntity::new(user.get_id(), user.get_email(), new_password);
        mod_user.date_created = user.date_created;
        mod_user.date_modified = user.date_modified;
        mod_user.verified = user.verified;
        mod_user.person = user.person;
        mod_user.metadata = user.metadata;
        // mod_user.deleted = user.deleted;
        let result: bool = repo.update(mod_user).await?;
        Ok(result)
    }
}