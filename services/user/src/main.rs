use std::env;

use diesel::{Connection, PgConnection};
use domain::{repositories::user_repository::UserRepository, user_entity::{Metadata, UserEntity}};
use dotenvy::dotenv;
use infra::postgres::repositories::PgUserRepository;
use uuid::Uuid;

mod domain;
mod infra;

fn main() {
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

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}