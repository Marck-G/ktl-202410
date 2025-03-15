use std::fmt;

use crate::database::QueryError;

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
}

impl From<QueryError> for ServiceError {
    fn from(err: QueryError) -> Self {
        ServiceError::DatabaseError(err.to_string())
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid data: {}", self.to_string())
    }
}