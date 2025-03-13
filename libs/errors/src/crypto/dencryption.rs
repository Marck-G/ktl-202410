use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct DecryptionError {
    base: BaseError
}

impl DecryptionError {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for DecryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Decryption: {}", self.base.message)
    }
}

impl std::error::Error for DecryptionError {}