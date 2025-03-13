use std::fmt;

#[derive(Debug)]
pub struct BaseError {
    pub message: String,
}

impl BaseError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

// Implement Display for user-friendly error messages
impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR - {}", self.message)
    }
}

// Implement std::error::Error for compatibility
impl std::error::Error for BaseError {}
