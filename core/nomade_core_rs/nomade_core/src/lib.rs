//! Nomade Core - Rust implementation of core functionality
//!
//! This library provides:
//! - Identity and key management
//! - QR pairing protocol
//! - QUIC networking
//! - Artifact storage
//! - CRDT sync engine
//! - Encryption helpers
//!
//! # FFI Bridge
//!
//! This library exposes an FFI interface for Flutter via flutter_rust_bridge.
//! See the `bridge` module for FFI exports.

pub mod identity;
pub mod pairing;
pub mod networking;
pub mod artifacts;
pub mod sync;
pub mod crypto;
pub mod bridge;

pub mod error;
pub use error::{Error, Result};

/// Core library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the library
pub fn init() -> Result<()> {
    // TODO: Initialization logic
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
