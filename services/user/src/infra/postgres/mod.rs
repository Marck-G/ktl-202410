use std::{env, sync::Arc};
use diesel::{Connection, PgConnection};
use tokio::sync::Mutex;

pub mod  models;
pub mod schema;
pub mod repositories;
pub mod mapper;
pub mod crypto;

/// Create a postgresql connection using Diesel, the database url get from the env variables
/// so must be set before init the service.
/// 
/// __`DATABASE_URL`__ = postgres://user:pass@host:port/db_name
pub fn establish_connection() -> Arc<Mutex<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new( Mutex::new(PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))))
}
