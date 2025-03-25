use std::env;
use diesel::{Connection, PgConnection};

pub mod  models;
pub mod schema;
// pub mod repositories;
// pub mod mapper;

/// Create a postgresql connection using Diesel, the database url get from the env variables
/// so must be set before init the service.
/// 
/// __`DATABASE_URL`__ = postgres://user:pass@host:port/db_name
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
