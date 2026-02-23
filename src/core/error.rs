//! # Error Types
//! 
//! Comprehensive error handling for VANTISVPN.

use std::fmt;

/// VANTISVPN error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VantisError {
    /// Cryptographic subsystem not initialized
    CryptoNotInitialized,
    /// Cryptographic operation failed
    CryptoError(String),
    /// Invalid key size
    InvalidKeySize,
    /// Invalid ciphertext
    InvalidCiphertext,
    /// Key has been consumed/zeroized
    KeyConsumed,
    /// Replay attack detected
    ReplayAttack(u64),
    /// Invalid hash size
    InvalidHashSize,
    
    /// Network error
    NetworkError(String),
    /// Invalid address
    InvalidAddress,
    /// Invalid endpoint
    InvalidEndpoint,
    /// Invalid MTU
    InvalidMtu,
    /// Not connected
    NotConnected,
    /// Already connected
    AlreadyConnected,
    
    /// Tunnel error
    TunnelNotFound(String),
    TunnelExists(String),
    NoActiveTunnel,
    
    /// Invalid state
    InvalidState,
    
    /// IP pool exhausted
    IpPoolExhausted,
    
    /// Generic error
    Generic(String),
}

impl fmt::Display for VantisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CryptoNotInitialized => write!(f, "Cryptographic subsystem not initialized"),
            Self::CryptoError(msg) => write!(f, "Cryptographic error: {}", msg),
            Self::InvalidKeySize => write!(f, "Invalid key size"),
            Self::InvalidCiphertext => write!(f, "Invalid ciphertext"),
            Self::KeyConsumed => write!(f, "Key has been consumed"),
            Self::ReplayAttack(seq) => write!(f, "Replay attack detected (sequence: {})", seq),
            Self::InvalidHashSize => write!(f, "Invalid hash size"),
            
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::InvalidAddress => write!(f, "Invalid address"),
            Self::InvalidEndpoint => write!(f, "Invalid endpoint"),
            Self::InvalidMtu => write!(f, "Invalid MTU"),
            Self::NotConnected => write!(f, "Not connected"),
            Self::AlreadyConnected => write!(f, "Already connected"),
            
            Self::TunnelNotFound(id) => write!(f, "Tunnel not found: {}", id),
            Self::TunnelExists(id) => write!(f, "Tunnel already exists: {}", id),
            Self::NoActiveTunnel => write!(f, "No active tunnel"),
            
            Self::InvalidState => write!(f, "Invalid state"),
            
            Self::IpPoolExhausted => write!(f, "IP pool exhausted"),
            
            Self::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for VantisError {}

/// Result type alias
pub type Result<T> = std::result::Result<T, VantisError>;

// Implement conversions from common error types
impl From<anyhow::Error> for VantisError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<std::io::Error> for VantisError {
    fn from(err: std::io::Error) -> Self {
        Self::NetworkError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = VantisError::CryptoNotInitialized;
        assert_eq!(err.to_string(), "Cryptographic subsystem not initialized");
    }

    #[test]
    fn test_error_debug() {
        let err = VantisError::NotConnected;
        assert_eq!(format!("{:?}", err), "NotConnected");
    }

    #[test]
    fn test_result_type() {
        let result: Result<()> = Ok(());
        assert!(result.is_ok());
        
        let result: Result<()> = Err(VantisError::InvalidState);
        assert!(result.is_err());
    }
}