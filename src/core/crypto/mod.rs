//! # Cryptographic Primitives
//! 
//! This module provides all cryptographic operations used by VANTISVPN,
//! including post-quantum cryptography, classical crypto, and key management.
//! 
//! ## Design Principles
//! 
//! 1. **Post-Quantum Ready**: Integration of Kyber (ML-KEM) and Dilithium (ML-DSA)
//! 2. **Ephemeral Keys**: All keys are temporary and securely discarded
//! 3. **Memory Safety**: All sensitive data is zeroized when dropped
//! 4. **FIPS 140-3 Compliance**: Use of NIST-approved algorithms

use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};

pub mod keys;
pub mod cipher;
pub mod pqc;
pub mod pqc_full;
pub mod hash;
pub mod random;

pub use keys::{EphemeralKeyPair, Cipher, CipherSuite};
pub use cipher::CipherMode;
pub use pqc_full::{
    MlKemKeyPair, MlDsaKeyPair, HybridKeyExchange, PqcManager,
    MlKemSecurityLevel, MlDsaSecurityLevel
};
pub use hash::Hash;
pub use random::SecureRandom;

/// Global crypto initialization state
static CRYPTO_INITIALIZED: AtomicBool = AtomicBool::new(false);
static INIT_ONCE: Once = Once::new();

/// Initialize the cryptographic subsystem
/// 
/// This function must be called once before any crypto operations.
/// It initializes the random number generator and sets up global state.
/// Safe to call multiple times (idempotent).
pub fn init() -> crate::Result<()> {
    INIT_ONCE.call_once(|| {
        // Initialize random number generator
        random::init();
        
        // Log initialization
        tracing::info!("Cryptographic subsystem initialized");
        
        CRYPTO_INITIALIZED.store(true, Ordering::SeqCst);
    });
    
    Ok(())
}

/// Cleanup cryptographic subsystem
/// 
/// Securely clears all sensitive data from memory.
pub fn cleanup() -> crate::Result<()> {
    if CRYPTO_INITIALIZED.load(Ordering::SeqCst) {
        tracing::info!("Cleaning up cryptographic subsystem");
        
        // Zeroize any remaining sensitive data
        // (This is handled by Drop implementations in submodules)
        
        CRYPTO_INITIALIZED.store(false, Ordering::SeqCst);
    }
    
    Ok(())
}

/// Verify that crypto subsystem is initialized
pub fn ensure_initialized() -> crate::Result<()> {
    if !CRYPTO_INITIALIZED.load(Ordering::SeqCst) {
        return Err(crate::VantisError::CryptoNotInitialized);
    }
    Ok(())
}

// Comprehensive test module (see tests.rs)
#[cfg(test)]
mod comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_init() {
        init().expect("Failed to initialize");
        ensure_initialized().expect("Crypto not initialized");
    }

    #[test]
    fn test_double_init() {
        init().expect("First init");
        init().expect("Second init should be idempotent");
    }

    #[test]
    fn test_cleanup() {
        init().expect("Init");
        cleanup().expect("Cleanup");
        
        // After cleanup, crypto should need re-initialization
        assert!(ensure_initialized().is_err());
    }
}