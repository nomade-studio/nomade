//! Encryption helpers using AES-256-GCM

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use serde::{Deserialize, Serialize};

use crate::{CryptoError, Result};

/// Encrypted data with nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub algorithm: String,
}

/// Encrypt data with AES-256-GCM
pub fn encrypt_data(plaintext: &[u8], key: &[u8; 32]) -> Result<EncryptedData> {
    let cipher = Aes256Gcm::new(key.into());

    // Generate random nonce (96 bits for GCM)
    let mut nonce_bytes = [0u8; 12];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce.to_vec(),
        algorithm: "AES-256-GCM".to_string(),
    })
}

/// Decrypt data with AES-256-GCM
pub fn decrypt_data(encrypted: &EncryptedData, key: &[u8; 32]) -> Result<Vec<u8>> {
    if encrypted.algorithm != "AES-256-GCM" {
        return Err(CryptoError::DecryptionFailed(
            "Unsupported algorithm".into(),
        ));
    }

    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(&encrypted.nonce);

    cipher
        .decrypt(nonce, encrypted.ciphertext.as_ref())
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
}

/// Derive key using HKDF-SHA256
pub fn derive_key(master_key: &[u8], salt: &[u8], info: &[u8]) -> [u8; 32] {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hkdf = Hkdf::<Sha256>::new(Some(salt), master_key);
    let mut okm = [0u8; 32];
    hkdf.expand(info, &mut okm).expect("HKDF expand failed");
    okm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [42u8; 32];
        let plaintext = b"Hello, Nomade!";

        let encrypted = encrypt_data(plaintext, &key).unwrap();
        let decrypted = decrypt_data(&encrypted, &key).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_derive_key() {
        let master_key = b"master secret key";
        let salt = b"salt";
        let info = b"context info";

        let key1 = derive_key(master_key, salt, info);
        let key2 = derive_key(master_key, salt, info);

        assert_eq!(key1, key2); // Deterministic

        let key3 = derive_key(master_key, b"different salt", info);
        assert_ne!(key1, key3); // Different salt = different key
    }
}
