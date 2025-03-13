use errors::database::data::query::QueryError;
use uuid::Uuid;

use crate::domain::user_entity::UserEntity;

pub trait UserRepository {
    fn find_one_by_id(&mut self, user_id: Uuid) -> Result<UserEntity, QueryError>;
    fn list(&mut self, page: i32, limit: i32) -> Result<Vec<UserEntity>, QueryError>;
    fn create(&mut self, user: UserEntity) -> Result<bool, QueryError>;
}
