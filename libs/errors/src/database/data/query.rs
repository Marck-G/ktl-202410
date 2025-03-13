use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct QueryError {
    base: BaseError
}

impl QueryError {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid data: {}", self.base.message)
    }
}

impl std::error::Error for QueryError {}