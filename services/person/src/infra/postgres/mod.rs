pub mod mapper;
pub mod  models;
pub mod schema;
pub mod repositories;
pub mod types;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}