/// Test utilities for VantisVPN
/// Provides common setup functions for tests

use std::sync::Once;

static CRYPTO_INIT: Once = Once::new();

/// Initialize crypto subsystem for tests
/// This ensures crypto is initialized exactly once across all tests
pub fn init_crypto() {
    CRYPTO_INIT.call_once(|| {
        vantis_core::crypto::init().expect("Failed to initialize crypto for tests");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_crypto() {
        init_crypto();
        // Should not panic on second call
        init_crypto();
    }
}