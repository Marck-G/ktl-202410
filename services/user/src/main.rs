use domain::{repositories::user_repository::UserRepository, user_entity::{Metadata, UserEntity}};
use dotenvy::dotenv;
use infra::postgres::{repositories::PgUserRepository, establish_connection};
use uuid::Uuid;

mod domain;
mod infra;

fn main() {
    dotenv().ok();
    let connection = &mut establish_connection();
    let metadata: Vec<Metadata> = vec![
        Metadata::new(Uuid::new_v4(), String::from("meta"), String::from("test"))
    ];
    let mut user: UserEntity = UserEntity::new(Uuid::new_v4(), "test@2ta".to_string(), "234134".to_string());
    user.metadata = metadata;
    let mut repo = PgUserRepository::new(connection);
    let response = repo.create(user);
    match response {
        Ok(r) => println!("Created: {}", r),
        Err(q) => println!("{}", q)
    }
}

