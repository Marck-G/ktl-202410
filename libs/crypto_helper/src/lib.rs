use errors::crypto::{CryptoError, DecryptionError, EncryptionError, KeyGenerationError, KeyParseError, KeyReadError, KeySaveError, UTF8Error};
use rand_core::OsRng as OsRngCore;
use rsa::{pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey}, pkcs8::EncodePublicKey, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::fs::{self, File};
use std::io::Write;

pub struct CryptoHelper {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

impl CryptoHelper {
    /// Generates a new RSA key pair and saves them to the given paths
    pub fn generate_keys(private_key_path: &str, public_key_path: &str) -> Result<(), CryptoError> {
        let mut rng = OsRngCore;
        let bits = 2048; // Key size
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|e| CryptoError::KeyGenerationError(KeyGenerationError::new(e.to_string().as_str())))?;
        let public_key = RsaPublicKey::from(&private_key);

        // Save private key
        let private_pem = private_key.to_pkcs1_pem(Default::default())
            .map_err(|e| CryptoError::KeySaveError(KeySaveError::new(e.to_string().as_str())))?;
        File::create(private_key_path)
            .and_then(|mut f| f.write_all(private_pem.as_bytes()))
            .map_err(|e| CryptoError::KeySaveError(KeySaveError::new(e.to_string().as_str())))?;

        // Save public key
        let public_pem = public_key.to_public_key_pem(Default::default())
            .map_err(|e| CryptoError::KeySaveError(KeySaveError::new(e.to_string().as_str())))?;
        File::create(public_key_path)
            .and_then(|mut f| f.write_all(public_pem.as_bytes()))
            .map_err(|e| CryptoError::KeySaveError(KeySaveError::new(e.to_string().as_str())))?;

        Ok(())
    }

    /// Loads the private key from a given path and generates the public key
    pub fn new(private_key_path: &str) -> Result<Self, CryptoError> {
        let private_key_pem = fs::read_to_string(private_key_path)
            .map_err(|e| CryptoError::KeyReadError(KeyReadError::new(e.to_string().as_str())))?;
        let private_key = RsaPrivateKey::from_pkcs1_pem(&private_key_pem)
            .map_err(|e| CryptoError::KeyParseError(KeyParseError::new(e.to_string().as_str())))?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self { private_key, public_key })
    }

    /// Encrypts a password using the public key
    pub fn encrypt_password(&self, password: String) -> Result<String, CryptoError> {
        let mut rng = OsRngCore;
        let encrypted_data = self.public_key.encrypt(
            &mut rng,
            Pkcs1v15Encrypt,
            password.as_bytes(),
        ).map_err(|e| CryptoError::EncryptionError(EncryptionError::new(e.to_string().as_str())))?;

        Ok(STANDARD.encode::<Vec<u8>>(encrypted_data)) // Convert to Base64 for storage
    }

    /// Decrypts a password using the private key
    pub fn decrypt_password(&self, encrypted_password: String) -> Result<String, CryptoError> {
        let encrypted_bytes = STANDARD.decode::<String>(encrypted_password)
            .map_err(|e| CryptoError::DecyptionError(DecryptionError::new(e.to_string().as_str())))?;
        let decrypted_data = self.private_key.decrypt(
            Pkcs1v15Encrypt,
            &encrypted_bytes,
        ).map_err(|e| CryptoError::DecyptionError(DecryptionError::new(e.to_string().as_str())))?;

        String::from_utf8(decrypted_data).map_err(|_| CryptoError::UTF8Error(UTF8Error::new("loading the decripted data")))
    }
}
