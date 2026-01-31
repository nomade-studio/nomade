//! Nomade Core Library
//!
//! Main library integrating all components

pub use nomade_crypto;
pub use nomade_events;
pub use nomade_quic;
pub use nomade_storage;

/// Initialize Nomade core with logging
pub fn init() {
    tracing_subscriber::fmt::init();
    tracing::info!("Nomade core initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
    }
}
