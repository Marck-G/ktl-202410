use crypto_helper::CryptoHelper;
use errors::crypto::CryptoError;

use crate::domain::crypto::PasswordTools;

pub struct PgPasswordTools {
    helper: CryptoHelper
}

impl PgPasswordTools {
     /// Create new PasswordTool for Postgres
     /// 
     /// # Example
     /// ```
     /// PgPasswordTools::new("./private.pem".to_string()).unwrap()
     /// ```
    pub fn new(private_key: String) -> Self {
        Self {
            helper: CryptoHelper::new(private_key.as_str()).unwrap()
        }

    }
}

impl PasswordTools for PgPasswordTools {
    fn decrypt(&mut self, bas64: String) -> Result<String, CryptoError> {
        self.helper.decrypt_password(bas64)  
    }

    fn encrypt(&mut self, password: String) -> Result<String, CryptoError> {
        self.helper.encrypt_password(password)
    }
}
