//! Cryptography helpers module
//!
//! Encryption and decryption utilities.

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};

use crate::{Error, Result};

/// Encrypt data with ChaCha20-Poly1305
pub fn encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| Error::Crypto(format!("Encryption failed: {}", e)))
}

/// Decrypt data with ChaCha20-Poly1305
pub fn decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| Error::Crypto(format!("Decryption failed: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let plaintext = b"test message";

        let ciphertext = encrypt(&key, &nonce, plaintext).unwrap();
        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_decrypt_with_wrong_key() {
        let key1 = [0u8; 32];
        let key2 = [1u8; 32];
        let nonce = [0u8; 12];
        let plaintext = b"test message";

        let ciphertext = encrypt(&key1, &nonce, plaintext).unwrap();
        let result = decrypt(&key2, &nonce, &ciphertext);

        assert!(result.is_err());
    }
}
