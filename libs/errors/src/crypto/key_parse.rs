use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct KeyParseError {
    base: BaseError
}

impl KeyParseError {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for KeyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key parse: {}", self.base.message)
    }
}

impl std::error::Error for KeyParseError {}