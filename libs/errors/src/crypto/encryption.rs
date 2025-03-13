use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct EncryptionError {
    base: BaseError
}

impl EncryptionError {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encryption: {}", self.base.message)
    }
}

impl std::error::Error for EncryptionError {}