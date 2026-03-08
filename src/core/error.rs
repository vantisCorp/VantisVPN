//! # Error Types
//!
//! Comprehensive error handling for VANTISVPN.

use std::fmt;

/// VANTISVPN error types
///
/// Error types used throughout the VANTISVPN codebase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VantisError {
    /// Cryptographic subsystem not initialized
    ///
    /// Cryptographic subsystem has not been initialized.
    CryptoNotInitialized,
    /// Cryptographic operation failed
    ///
    /// A cryptographic operation failed.
    CryptoError(String),
    /// Invalid key size
    ///
    /// Key size is invalid for the operation.
    InvalidKeySize,
    /// Invalid ciphertext
    ///
    /// Ciphertext is invalid or corrupted.
    InvalidCiphertext,
    /// Key has been consumed/zeroized
    ///
    /// Key has been consumed or zeroized.
    KeyConsumed,
    /// Replay attack detected
    ///
    /// A replay attack was detected with the given sequence number.
    ReplayAttack(u64),
    /// Invalid hash size
    ///
    /// Hash size is invalid.
    InvalidHashSize,

    /// Network error
    ///
    /// A network error occurred.
    NetworkError(String),
    /// Invalid address
    ///
    /// Network address is invalid.
    InvalidAddress,
    /// Invalid endpoint
    ///
    /// Network endpoint is invalid.
    InvalidEndpoint,
    /// Invalid MTU
    ///
    /// Maximum Transmission Unit is invalid.
    InvalidMtu,
    /// Not connected
    ///
    /// Not connected to the network.
    NotConnected,
    /// Already connected
    ///
    /// Already connected to the network.
    AlreadyConnected,

    /// Tunnel error
    ///
    /// Tunnel with given ID not found.
    TunnelNotFound(String),
    /// Tunnel already exists
    ///
    /// Tunnel with given ID already exists.
    TunnelExists(String),
    /// No active tunnel
    ///
    /// No active tunnel exists.
    NoActiveTunnel,

    /// Invalid state
    ///
    /// Operation is invalid in the current state.
    InvalidState,

    /// IP pool exhausted
    ///
    /// IP address pool is exhausted.
    IpPoolExhausted,

    /// Invalid packet
    ///
    /// Packet is invalid.
    InvalidPacket(String),
    /// Invalid stream
    ///
    /// Stream is invalid.
    InvalidStream(String),
    /// Stream closed
    ///
    /// Stream has been closed.
    StreamClosed,
    /// Invalid peer
    ///
    /// Peer is invalid.
    InvalidPeer(String),
    /// Circuit not established
    ///
    /// Circuit has not been established.
    CircuitNotEstablished,
    /// Invalid circuit
    ///
    /// Circuit is invalid.
    InvalidCircuit,
    /// Insufficient nodes
    ///
    /// Insufficient nodes available for operation.
    InsufficientNodes(String),

    /// Generic error
    ///
    /// Generic error message.
    Generic(String),

    /// Authentication failed
    ///
    /// Authentication operation failed.
    AuthenticationFailed(String),
    /// Resource not found
    ///
    /// Resource not found.
    NotFound(String),
    /// Invalid data
    ///
    /// Invalid data provided.
    InvalidData(String),
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

            Self::InvalidPacket(msg) => write!(f, "Invalid packet: {}", msg),
            Self::InvalidStream(msg) => write!(f, "Invalid stream: {}", msg),
            Self::StreamClosed => write!(f, "Stream closed"),
            Self::InvalidPeer(msg) => write!(f, "Invalid peer: {}", msg),
            Self::CircuitNotEstablished => write!(f, "Circuit not established"),
            Self::InvalidCircuit => write!(f, "Invalid circuit"),
            Self::InsufficientNodes(msg) => write!(f, "Insufficient nodes: {}", msg),

            Self::Generic(msg) => write!(f, "Error: {}", msg),
            Self::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
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

impl From<std::time::SystemTimeError> for VantisError {
    fn from(err: std::time::SystemTimeError) -> Self {
        Self::Generic(err.to_string())
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
