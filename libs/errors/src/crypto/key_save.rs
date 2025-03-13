use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct KeySaveError {
    base: BaseError
}

impl KeySaveError {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for KeySaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key Save: {}", self.base.message)
    }
}

impl std::error::Error for KeySaveError {}