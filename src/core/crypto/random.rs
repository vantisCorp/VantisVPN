//! # Cryptographically Secure Random Number Generation
//! 
//! Uses ChaCha20-based CSPRNG for all cryptographic operations.

use rand::{Rng, random};
use rand_core::RngCore;
use rand_core::SeedableRng;
use crate::error::VantisError;
use rand_chacha::ChaCha20Rng;
use std::sync::Mutex;

/// Thread-safe CSPRNG instance
static CSPRNG: Mutex<Option<ChaCha20Rng>> = Mutex::new(None);

/// Cryptographically secure random number generator
/// 
/// Thread-safe CSPRNG using ChaCha20 for cryptographic operations.
#[derive(Debug)]
pub struct SecureRandom {
    rng: Mutex<ChaCha20Rng>,
}

impl SecureRandom {
    /// Create a new SecureRandom instance
    pub fn new() -> crate::Result<Self> {
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&random::<[u8; 32]>());
        Ok(Self {
            rng: Mutex::new(ChaCha20Rng::from_seed(seed)),
        })
    }
    
    /// Generate random bytes
    pub fn generate_bytes(&self, len: usize) -> crate::Result<Vec<u8>> {
        let mut bytes = vec![0u8; len];
        let mut rng = self.rng.lock()
            .map_err(|_| crate::VantisError::CryptoError("RNG lock failed".to_string()))?;
        rng.fill_bytes(&mut bytes);
        Ok(bytes)
    }
    
    /// Generate a random u64 value
    pub fn generate_u64(&self) -> crate::Result<u64> {
        let mut bytes = [0u8; 8];
        let mut rng = self.rng.lock()
            .map_err(|_| crate::VantisError::CryptoError("RNG lock failed".to_string()))?;
        rng.fill_bytes(&mut bytes);
        Ok(u64::from_be_bytes(bytes))
    }
    
    /// Generate a random u32 value
    pub fn generate_u32(&self) -> crate::Result<u32> {
        let mut bytes = [0u8; 4];
        let mut rng = self.rng.lock()
            .map_err(|_| crate::VantisError::CryptoError("RNG lock failed".to_string()))?;
        rng.fill_bytes(&mut bytes);
        Ok(u32::from_be_bytes(bytes))
    }
    
    /// Generate a random boolean
    pub fn generate_bool(&self) -> crate::Result<bool> {
        Ok(self.generate_u32()? % 2 == 0)
    }
}

/// Initialize the random number generator
/// 
/// Uses system entropy to seed the generator.
pub fn init() {
    let mut rng = CSPRNG.lock().unwrap();
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&random::<[u8; 32]>());
    *rng = Some(ChaCha20Rng::from_seed(seed));
    tracing::debug!("CSPRNG initialized");
}

/// Generate cryptographically secure random bytes
/// 
/// This is the preferred method for generating random data.
pub fn secure_random(bytes: &mut [u8]) -> crate::Result<()> {
    let mut rng = CSPRNG
        .lock()
        .map_err(|_| crate::VantisError::CryptoError("CSPRNG lock failed".to_string()))?;
    
    let rng = rng.as_mut()
        .ok_or_else(|| crate::VantisError::CryptoError("CSPRNG not initialized".to_string()))?;
    
    rng.fill_bytes(bytes);
    Ok(())
}

/// Generate a random u64 value
pub fn random_u64() -> crate::Result<u64> {
    let mut bytes = [0u8; 8];
    secure_random(&mut bytes)?;
    Ok(u64::from_be_bytes(bytes))
}

/// Generate a random u32 value
pub fn random_u32() -> crate::Result<u32> {
    let mut bytes = [0u8; 4];
    secure_random(&mut bytes)?;
    Ok(u32::from_be_bytes(bytes))
}

/// Generate a random boolean
pub fn random_bool() -> crate::Result<bool> {
    Ok(random_u32()? % 2 == 0)
}

/// Generate a random nonce of the given size
pub fn random_nonce(size: usize) -> crate::Result<Vec<u8>> {
    let mut nonce = vec![0u8; size];
    secure_random(&mut nonce)?;
    Ok(nonce)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bytes() {
        let mut bytes = [0u8; 32];
        secure_random(&mut bytes).expect("Failed to generate random bytes");
        
        // Check that not all bytes are the same (extremely unlikely)
        let first = bytes[0];
        let all_same = bytes.iter().all(|&b| b == first);
        assert!(!all_same);
    }

    #[test]
    fn test_random_u64() {
        let val1 = random_u64().expect("Failed to generate u64");
        let val2 = random_u64().expect("Failed to generate u64");
        
        // They should be different (extremely unlikely to be same)
        assert_ne!(val1, val2);
    }

    #[test]
    fn test_random_nonce() {
        let nonce1 = random_nonce(12).expect("Failed to generate nonce");
        let nonce2 = random_nonce(12).expect("Failed to generate nonce");
        
        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1, nonce2);
    }
}