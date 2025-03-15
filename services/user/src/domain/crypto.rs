use errors::crypto::CryptoError;

pub trait PasswordTools {
    fn encrypt(&mut self, password: String) -> Result<String, CryptoError>;
    fn decrypt(&mut self, bas64: String) -> Result<String, CryptoError>;
}