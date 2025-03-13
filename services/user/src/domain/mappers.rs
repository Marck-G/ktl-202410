use super::user_entity::UserEntity;

pub trait UserMapper<T, U> {
    fn to_domain(model: T, metadata: Vec<U>) -> UserEntity;
    fn to_infrastructure(entity: &UserEntity) -> (T, Vec<U>);
}