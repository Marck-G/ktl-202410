use errors::database::{data::query::QueryError, DatabaseError};
use uuid::Uuid;

use crate::domain::user_entity::UserEntity;

pub trait UserRepository {
    async fn find_one_by_id(&mut self, user_id: Uuid) -> Result<UserEntity, QueryError>;
    async fn list(&mut self, page: i32, limit: i32) -> Result<Vec<UserEntity>, QueryError>;
    async fn create(&mut self, user: UserEntity) -> Result<bool, QueryError>;
    async fn update(&mut self, user: UserEntity) -> Result<bool, QueryError>;
    async fn change_password(&mut self, user_id: Uuid, new_password: String) -> Result<bool, DatabaseError>;
    async fn filter(&mut self, user: UserEntity) -> Result<Vec<UserEntity>, QueryError>;
    async fn filter_by_metadata(&mut self, metadata: Vec<(String, String)>) -> Result<Vec<UserEntity>, QueryError>;
    async fn email_exists(&mut self, email: String) -> Result<bool, QueryError>;
    async fn update_metadata(&mut self, user_id: Uuid, key: String, value: Option<String>) -> Result<bool, QueryError>;
    async fn soft_delete(&mut self, user_id: Uuid) -> Result<bool, QueryError>;
    async fn restore(&mut self, user_id: Uuid) -> Result<bool, QueryError>;
}
