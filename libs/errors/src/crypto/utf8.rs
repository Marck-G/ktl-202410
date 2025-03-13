use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct UTF8Error {
    base: BaseError
}

impl UTF8Error {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for UTF8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UTF-8: {}", self.base.message)
    }
}

impl std::error::Error for UTF8Error {}