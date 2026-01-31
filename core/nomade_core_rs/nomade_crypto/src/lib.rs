//! Cryptography primitives for Nomade
//!
//! This crate provides:
//! - Device identity keys (Ed25519)
//! - QR code payload encoding/decoding
//! - Encryption helpers (AES-256-GCM)
//! - Key derivation (HKDF)

pub mod encryption;
pub mod identity;
pub mod qr_payload;

pub use encryption::{decrypt_data, encrypt_data, EncryptedData};
pub use identity::{generate_keypair, DeviceId, DeviceKeypair};
pub use qr_payload::{decode_pairing_offer, encode_pairing_offer, PairingOffer};

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
