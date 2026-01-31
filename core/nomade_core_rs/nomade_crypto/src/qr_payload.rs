//! QR code payload encoding/decoding for device pairing

use ed25519_dalek::Signature;
use serde::{Deserialize, Serialize};

use crate::{DeviceId, Result};


/// Pairing offer for QR code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingOffer {
    pub version: u8,
    pub device_id: DeviceId,
    pub device_name: String,
    pub public_key: Vec<u8>,
    pub endpoints: Vec<String>,
    pub nonce: Vec<u8>,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

impl PairingOffer {
    /// Create new pairing offer
    pub fn new(
        device_id: DeviceId,
        device_name: String,
        public_key: Vec<u8>,
        endpoints: Vec<String>,
    ) -> Self {
        let nonce = generate_nonce();
        let timestamp = current_timestamp();
        
        Self {
            version: 1,
            device_id,
            device_name,
            public_key,
            endpoints,
            nonce,
            timestamp,
            signature: vec![], // Will be signed separately
        }
    }
    
    /// Get signing payload
    pub fn signing_payload(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&[self.version]);
        payload.extend_from_slice(self.device_id.0.as_bytes());
        payload.extend_from_slice(self.device_name.as_bytes());
        payload.extend_from_slice(&self.public_key);
        for endpoint in &self.endpoints {
            payload.extend_from_slice(endpoint.as_bytes());
        }
        payload.extend_from_slice(&self.nonce);
        payload.extend_from_slice(&self.timestamp.to_le_bytes());
        payload
    }
}

/// Encode pairing offer as URL (for QR code)
pub fn encode_pairing_offer(offer: &PairingOffer) -> Result<String> {
    let json = serde_json::to_string(offer)?;
    let compressed = compress_data(json.as_bytes());
    let encoded = base64_encode(&compressed);
    Ok(format!("nomade://pair?v=1&d={}", encoded))
}

/// Decode pairing offer from URL
pub fn decode_pairing_offer(url: &str) -> Result<PairingOffer> {
    // Extract data parameter from URL
    let data = url
        .strip_prefix("nomade://pair?v=1&d=")
        .ok_or_else(|| crate::CryptoError::EncryptionFailed("Invalid URL format".into()))?;
    
    let compressed = base64_decode(data)?;
    let json = decompress_data(&compressed)?;
    let offer = serde_json::from_slice(&json)?;
    Ok(offer)
}

// Helper functions

fn generate_nonce() -> Vec<u8> {
    use rand::RngCore;
    let mut nonce = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn compress_data(data: &[u8]) -> Vec<u8> {
    // Placeholder: In production, use zstd compression
    // For now, just return data as-is
    data.to_vec()
}

fn decompress_data(data: &[u8]) -> Result<Vec<u8>> {
    // Placeholder: In production, use zstd decompression
    Ok(data.to_vec())
}

fn base64_encode(data: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

fn base64_decode(data: &str) -> Result<Vec<u8>> {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .map_err(|e| crate::CryptoError::EncryptionFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode_decode_pairing_offer() {
        let offer = PairingOffer::new(
            DeviceId("test-device".into()),
            "Test Device".into(),
            vec![1, 2, 3, 4],
            vec!["192.168.1.100:8765".into()],
        );
        
        let encoded = encode_pairing_offer(&offer).unwrap();
        assert!(encoded.starts_with("nomade://pair?v=1&d="));
        
        let decoded = decode_pairing_offer(&encoded).unwrap();
        assert_eq!(decoded.device_name, "Test Device");
        assert_eq!(decoded.endpoints, vec!["192.168.1.100:8765"]);
    }
}
