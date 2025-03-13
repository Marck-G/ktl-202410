use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct KeyReadError {
    base: BaseError
}

impl KeyReadError {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for KeyReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key Read: {}", self.base.message)
    }
}

impl std::error::Error for KeyReadError {}