use std::fmt;

use data::{invalid_field::InvalidData, not_found::DataNotFound, query::QueryError};
pub mod data;

#[derive(Debug)]
pub enum DatabaseError {
    NotFound(DataNotFound),
    InvalidData(InvalidData),
    QueryError(QueryError)
}


impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::InvalidData(err) => write!(f, "{}", err),
            DatabaseError::NotFound(err) => write!(f, "{}", err),
            DatabaseError::QueryError(err) => write!(f, "{}", err),
            // DatabaseError::Database(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for DatabaseError {}
