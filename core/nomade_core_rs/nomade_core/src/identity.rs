//! Identity and key management module
//!
//! Handles device identity, key generation, and key storage.

use ed25519_dalek::{Signer, SigningKey, Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result};

/// Device identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub device_id: Uuid,
    pub device_name: String,
    pub public_key: Vec<u8>,
    #[serde(skip)]
    signing_key: Option<SigningKey>,
}

impl Identity {
    /// Generate a new identity with a random keypair
    pub fn generate(device_name: String) -> Result<Self> {
        let mut csprng = rand::rngs::OsRng;
        let signing_key = SigningKey::from_bytes(&rand::Rng::gen(&mut csprng));
        let verifying_key = signing_key.verifying_key();

        Ok(Identity {
            device_id: Uuid::new_v4(),
            device_name,
            public_key: verifying_key.to_bytes().to_vec(),
            signing_key: Some(signing_key),
        })
    }

    /// Sign data with the private key
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let signing_key = self
            .signing_key
            .as_ref()
            .ok_or_else(|| Error::Crypto("No signing key available".to_string()))?;

        let signature: Signature = signing_key.sign(data);
        Ok(signature.to_bytes().to_vec())
    }

    /// Verify a signature against this identity's public key
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        // Convert public key bytes to VerifyingKey
        let verifying_key_bytes: [u8; 32] = self.public_key.as_slice().try_into()
            .map_err(|_| Error::Crypto("Public key must be 32 bytes".to_string()))?;
        let verifying_key = VerifyingKey::from_bytes(&verifying_key_bytes)
            .map_err(|e| Error::Crypto(format!("Invalid public key: {}", e)))?;

        // Convert signature slice to array
        let signature_bytes: [u8; 64] = signature.try_into()
            .map_err(|_| Error::Crypto("Signature must be 64 bytes".to_string()))?;
        
        let signature = Signature::from_bytes(&signature_bytes);

        Ok(verifying_key.verify(data, &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_identity() {
        let identity = Identity::generate("Test Device".to_string()).unwrap();
        assert_eq!(identity.device_name, "Test Device");
        assert_eq!(identity.public_key.len(), 32);
    }

    #[test]
    fn test_sign_and_verify() {
        let identity = Identity::generate("Test Device".to_string()).unwrap();
        let data = b"test message";

        let signature = identity.sign(data).unwrap();
        assert!(identity.verify(data, &signature).unwrap());

        // Wrong data should fail verification
        let wrong_data = b"wrong message";
        assert!(!identity.verify(wrong_data, &signature).unwrap());
    }
}
