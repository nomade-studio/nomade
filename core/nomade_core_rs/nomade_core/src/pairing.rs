//! QR code pairing module
//!
//! Handles QR code generation and parsing for device pairing.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result};

/// QR code pairing payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingPayload {
    pub version: u8,
    pub device_id: Uuid,
    pub device_name: String,
    pub public_key: Vec<u8>,
    pub endpoint: String,
    pub timestamp: i64,
    pub signature: Vec<u8>,
}

impl PairingPayload {
    /// Create a new pairing payload
    pub fn new(
        device_id: Uuid,
        device_name: String,
        public_key: Vec<u8>,
        endpoint: String,
        timestamp: i64,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            version: 1,
            device_id,
            device_name,
            public_key,
            endpoint,
            timestamp,
            signature,
        }
    }

    /// Encode as JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
            .map_err(|e| Error::Serialization(format!("Failed to serialize: {}", e)))
    }

    /// Decode from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| Error::Serialization(format!("Failed to deserialize: {}", e)))
    }

    /// Create QR code URI (nomade://pair/<base64-json>)
    pub fn to_qr_uri(&self) -> Result<String> {
        let json = self.to_json()?;
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(json);
        Ok(format!("nomade://pair/{}", encoded))
    }

    /// Parse QR code URI
    pub fn from_qr_uri(uri: &str) -> Result<Self> {
        if !uri.starts_with("nomade://pair/") {
            return Err(Error::InvalidData("Invalid QR code URI".to_string()));
        }

        let encoded = &uri[14..]; // Skip "nomade://pair/"
        use base64::Engine;
        let json = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| Error::InvalidData(format!("Invalid base64: {}", e)))?;

        let json_str = String::from_utf8(json)
            .map_err(|e| Error::InvalidData(format!("Invalid UTF-8: {}", e)))?;

        Self::from_json(&json_str)
    }

    /// Validate timestamp (not too old or in the future)
    pub fn validate_timestamp(&self, max_age_seconds: i64) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let age = now - self.timestamp;

        if age > max_age_seconds {
            return Err(Error::InvalidData("QR code expired".to_string()));
        }

        if age < -60 {
            // Allow 1 minute clock skew
            return Err(Error::InvalidData("QR code from future".to_string()));
        }

        Ok(())
    }
}

// Note: Add base64 and chrono dependencies if not already in Cargo.toml

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairing_payload_serialization() {
        let payload = PairingPayload::new(
            Uuid::new_v4(),
            "Test Device".to_string(),
            vec![1, 2, 3, 4],
            "192.168.1.100:8443".to_string(),
            chrono::Utc::now().timestamp(),
            vec![5, 6, 7, 8],
        );

        let json = payload.to_json().unwrap();
        let decoded = PairingPayload::from_json(&json).unwrap();

        assert_eq!(payload.device_name, decoded.device_name);
        assert_eq!(payload.endpoint, decoded.endpoint);
    }
}
