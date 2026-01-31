//! Cryptography primitives for Nomade
//!
//! This crate provides:
//! - Device identity keys (Ed25519)
//! - QR code payload encoding/decoding
//! - Encryption helpers (AES-256-GCM)
//! - Key derivation (HKDF)

pub mod identity;
pub mod qr_payload;
pub mod encryption;

pub use identity::{DeviceId, DeviceKeypair, generate_keypair};
pub use qr_payload::{PairingOffer, encode_pairing_offer, decode_pairing_offer};
pub use encryption::{encrypt_data, decrypt_data, EncryptedData};

/// Common error type for crypto operations
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid key format")]
    InvalidKey,
    
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CryptoError>;

