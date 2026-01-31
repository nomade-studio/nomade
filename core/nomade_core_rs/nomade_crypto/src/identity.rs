//! Device identity management

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::{CryptoError, Result};

/// Device ID derived from public key
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    /// Create device ID from public key
    pub fn from_public_key(public_key: &VerifyingKey) -> Self {
        let hash = blake3::hash(public_key.as_bytes());
        Self(format!("blake3-{}", hash.to_hex()))
    }
}

impl std::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Device keypair for identity and signing
#[derive(Clone)]
pub struct DeviceKeypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    device_id: DeviceId,
}

impl DeviceKeypair {
    /// Create new keypair from SigningKey
    pub fn new(signing_key: SigningKey) -> Self {
        let verifying_key = signing_key.verifying_key();
        let device_id = DeviceId::from_public_key(&verifying_key);
        Self { signing_key, verifying_key, device_id }
    }
    
    /// Get device ID
    pub fn device_id(&self) -> &DeviceId {
        &self.device_id
    }
    
    /// Get verifying key (public key)
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }
    
    /// Sign message
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
    
    /// Verify signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<()> {
        self.verifying_key
            .verify(message, signature)
            .map_err(|_| CryptoError::InvalidSignature)
    }
    
    /// Serialize public key to bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.verifying_key.as_bytes().to_vec()
    }
    
    /// Serialize secret key to bytes (use carefully!)
    pub fn secret_key_bytes(&self) -> Vec<u8> {
        self.signing_key.to_bytes().to_vec()
    }
}

/// Generate new device keypair
pub fn generate_keypair() -> DeviceKeypair {
    use rand::RngCore;
    let mut csprng = OsRng;
    let mut secret_key_bytes = [0u8; 32];
    csprng.fill_bytes(&mut secret_key_bytes);
    let signing_key = SigningKey::from_bytes(&secret_key_bytes);
    DeviceKeypair::new(signing_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_keypair() {
        let keypair = generate_keypair();
        assert!(keypair.device_id().0.starts_with("blake3-"));
    }
    
    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_keypair();
        let message = b"Hello, Nomade!";
        
        let signature = keypair.sign(message);
        assert!(keypair.verify(message, &signature).is_ok());
        
        let wrong_message = b"Wrong message";
        assert!(keypair.verify(wrong_message, &signature).is_err());
    }
}
