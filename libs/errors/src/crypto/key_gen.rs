use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct KeyGenerationError {
    base: BaseError
}

impl KeyGenerationError {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for KeyGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key Generation: {}", self.base.message)
    }
}

impl std::error::Error for KeyGenerationError {}