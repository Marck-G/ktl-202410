use std::fmt;

use crate::base::BaseError;

#[derive(Debug)]
pub struct DataNotFound {
    base: BaseError
}

impl DataNotFound {
    pub fn new(message: &str) -> Self {
        Self {
            base: BaseError::new(message),
        }
    }
}
impl fmt::Display for DataNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data Not Found: {}", self.base.message)
    }
}

impl std::error::Error for DataNotFound {}