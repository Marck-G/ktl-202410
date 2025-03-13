use errors::database::{data::query::QueryError, DatabaseError};
use uuid::Uuid;

use crate::domain::user_entity::UserEntity;

pub trait UserRepository {
    fn find_one_by_id(&mut self, user_id: Uuid) -> Result<UserEntity, QueryError>;
    fn list(&mut self, page: i32, limit: i32) -> Result<Vec<UserEntity>, QueryError>;
    fn create(&mut self, user: UserEntity) -> Result<bool, QueryError>;
    fn update(&mut self, user: UserEntity) -> Result<bool, QueryError>;
    fn change_password(&mut self, user_id: Uuid, new_password: String) -> Result<bool, DatabaseError>;
    fn filter(&mut self, user: UserEntity) -> Result<Vec<UserEntity>, QueryError>;
    fn filter_by_metadata(&mut self, metadata: Vec<(String, String)>) -> Result<Vec<UserEntity>, QueryError>;
    fn email_exists(&mut self, email: String) -> Result<bool, QueryError>;
    fn update_metadata(&mut self, user_id: Uuid, key: String, value: Option<String>) -> Result<bool, QueryError>;
}
