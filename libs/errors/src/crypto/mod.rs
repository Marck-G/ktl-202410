use std::fmt;

pub use dencryption::DecryptionError;
pub use encryption::EncryptionError;
pub use key_gen::KeyGenerationError;
pub use key_parse::KeyParseError;
pub use key_read::KeyReadError;
pub use key_save::KeySaveError;
pub use utf8::UTF8Error;

mod dencryption;
mod encryption;
mod key_gen;
mod key_read;
mod key_save;
mod utf8;
mod key_parse;


#[derive(Debug)]
pub enum CryptoError {
    EncryptionError(EncryptionError),
    DecyptionError(DecryptionError),
    KeySaveError(KeySaveError),
    KeyGenerationError(KeyGenerationError),
    KeyReadError(KeyReadError),
    KeyParseError(KeyParseError),
    UTF8Error(UTF8Error)
}


impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::EncryptionError(err) => write!(f, "{}", err),
            CryptoError::DecyptionError(err) => write!(f, "{}", err),
            CryptoError::KeySaveError(err) => write!(f, "{}", err),
            CryptoError::KeyGenerationError(err) => write!(f, "{}", err),
            CryptoError::KeyReadError(err) => write!(f, "{}", err),
            CryptoError::KeyParseError(err) => write!(f, "{}", err),
            CryptoError::UTF8Error(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for CryptoError {}
