//! FFI bridge module
//!
//! Provides FFI interface for Flutter via flutter_rust_bridge.
//! This module will be implemented when flutter_rust_bridge is set up.

use crate::{identity::Identity, pairing::PairingPayload, Result};

/// Initialize the core library
pub fn ffi_init() -> Result<()> {
    crate::init()
}

/// Generate a new device identity
pub fn ffi_generate_identity(device_name: String) -> Result<Identity> {
    Identity::generate(device_name)
}

/// Create a pairing QR code payload
pub fn ffi_create_pairing_qr(
    device_id: String,
    device_name: String,
    public_key: Vec<u8>,
    endpoint: String,
) -> Result<String> {
    use uuid::Uuid;

    let device_id = Uuid::parse_str(&device_id)
        .map_err(|e| crate::Error::InvalidData(format!("Invalid UUID: {}", e)))?;

    let timestamp = chrono::Utc::now().timestamp();

    // TODO: Generate proper signature
    let signature = vec![0u8; 64];

    let payload = PairingPayload::new(
        device_id,
        device_name,
        public_key,
        endpoint,
        timestamp,
        signature,
    );

    payload.to_qr_uri()
}

// Note: When flutter_rust_bridge is set up, use #[flutter_rust_bridge::frb] macros
// to generate the actual FFI bindings.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_init() {
        assert!(ffi_init().is_ok());
    }

    #[test]
    fn test_ffi_generate_identity() {
        let identity = ffi_generate_identity("Test Device".to_string()).unwrap();
        assert_eq!(identity.device_name, "Test Device");
    }
}
