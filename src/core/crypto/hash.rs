//! # Cryptographic Hashing
//! 
//! BLAKE2s hashing implementation for integrity verification.

use blake2::{Blake2s256, Digest};
use serde::{Serialize, Deserialize};

/// Size of BLAKE2s hash output (256 bits)
pub const HASH_SIZE: usize = 32;

/// BLAKE2s hash output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash {
    bytes: [u8; HASH_SIZE],
}

impl Hash {
    /// Create a new Hash instance (zero-initialized)
    pub fn new() -> crate::Result<Self> {
        Ok(Self { bytes: [0u8; HASH_SIZE] })
    }
    
    /// Compute BLAKE2s hash of data
    pub fn compute(&self, data: &[u8]) -> crate::Result<Vec<u8>> {
        let mut hasher = Blake2s256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(result.to_vec())
    }
    
    /// Compute hash with key (MAC)
    pub fn compute_keyed(&self, key: &[u8], data: &[u8]) -> crate::Result<Vec<u8>> {
        // Simple keyed hash implementation
        // In production, use proper MAC like HMAC
        let mut combined = key.to_vec();
        combined.extend_from_slice(data);
        let hash = self.compute(&combined)?;
        
        // Clear sensitive data
        combined.fill(0);
        
        Ok(hash)
    }
    
    /// Compute MAC
    pub fn compute_mac(&self, data: &[u8], key: &[u8]) -> crate::Result<Vec<u8>> {
        self.compute_keyed(key, data)
    }
    
    /// Get hash as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    
    /// Get hash as hex string
    pub fn as_hex(&self) -> String {
        hex::encode(&self.bytes)
    }
    
    /// Parse from hex string
    pub fn from_hex(hex_str: &str) -> crate::Result<Self> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| crate::VantisError::CryptoError(format!("Invalid hex: {}", e)))?;
        if bytes.len() != HASH_SIZE {
            return Err(crate::VantisError::InvalidHashSize);
        }
        
        let mut arr = [0u8; HASH_SIZE];
        arr.copy_from_slice(&bytes);
        Ok(Self { bytes: arr })
    }
    
    /// Check if hash equals zero (all zeros)
    pub fn is_zero(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self {
            bytes: [0u8; HASH_SIZE],
        }
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_computation() {
        let data = b"Hello, VANTISVPN!";
        let hash_instance = Hash::new().unwrap();
        let hash_result = hash_instance.compute(data).unwrap();
        
        assert_eq!(hash_result.len(), HASH_SIZE);
    }

    #[test]
    fn test_hash_deterministic() {
        let data = b"Test data";
        let hash_instance = Hash::new().unwrap();
        let hash1 = hash_instance.compute(data).unwrap();
        let hash2 = hash_instance.compute(data).unwrap();
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_different_inputs() {
        let data1 = b"Data 1";
        let data2 = b"Data 2";
        let hash_instance = Hash::new().unwrap();
        let hash1 = hash_instance.compute(data1).unwrap();
        let hash2 = hash_instance.compute(data2).unwrap();
        
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_keyed_hash() {
        let key = b"secret key";
        let data = b"Test data";
        let hash_instance = Hash::new().unwrap();
        let hash = hash_instance.compute_keyed(key, data).unwrap();
        
        assert_eq!(hash.len(), HASH_SIZE);
    }

    #[test]
    fn test_hash_hex() {
        let data = b"Test";
        let hash_instance = Hash::new().unwrap();
        let hash_result = hash_instance.compute(data).unwrap();
        let hex = hex::encode(&hash_result);
        
        assert_eq!(hex.len(), HASH_SIZE * 2);
        
        let hash2 = Hash::from_hex(&hex).expect("Failed to parse hex");
        assert_eq!(hash_instance, hash2);
    }
}