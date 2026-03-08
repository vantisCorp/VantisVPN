//! # Protocol Implementation
//!
//! VANTISVPN protocol based on WireGuard with enhancements:
//! - Post-quantum key exchange
//! - Dynamic IP allocation
//! - Key rotation
//! - QUIC transport

use serde::{Deserialize, Serialize};

/// Protocol version
pub const PROTOCOL_VERSION: u8 = 1;

/// Message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    /// Handshake initiation
    HandshakeInit = 1,
    /// Handshake response
    HandshakeResponse = 2,
    /// Cookie reply
    CookieReply = 3,
    /// Transport data
    Transport = 4,
}

/// Handshake initiation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeInit {
    /// Ephemeral public key for this session
    pub ephemeral_public: Vec<u8>,
    /// Static public key (optional, for identity verification)
    pub static_public: Option<Vec<u8>>,
    /// Post-quantum cryptography public key
    pub pqc_public: Vec<u8>,
    /// Message timestamp (Unix timestamp)
    pub timestamp: u64,
    /// Encrypted payload containing handshake data
    pub encrypted: Vec<u8>,
}

/// Handshake response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponse {
    /// Ephemeral public key for this session
    pub ephemeral_public: Vec<u8>,
    /// Post-quantum cryptography ciphertext
    pub pqc_ciphertext: Vec<u8>,
    /// Encrypted payload containing handshake response data
    pub encrypted: Vec<u8>,
}

/// Transport data message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportMessage {
    /// Receiver index (identifies the receiving peer)
    pub receiver: u32,
    /// Packet counter (for replay protection)
    pub counter: u64,
    /// Encrypted payload data
    pub data: Vec<u8>,
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    /// Enable post-quantum key exchange
    pub enable_pqc: bool,
    /// Enable automatic key rotation
    pub enable_key_rotation: bool,
    /// Key rotation interval in seconds
    pub key_rotation_interval: u64,
    /// Enable QUIC transport layer
    pub enable_quic: bool,
    /// Maximum Transmission Unit (bytes)
    pub mtu: u16,
    /// Keepalive interval in seconds
    pub keepalive_interval: u64,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            enable_pqc: true,
            enable_key_rotation: true,
            key_rotation_interval: 600, // 10 minutes
            enable_quic: true,
            mtu: 1420,
            keepalive_interval: 10,
        }
    }
}

/// Protocol state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolState {
    /// Not connected
    Disconnected,
    /// Initiating handshake
    Handshaking,
    /// Handshake complete, connected
    Connected,
    /// Closing connection
    Closing,
}

/// Protocol implementation for VPN connections
///
/// Manages the state machine for the VANTISVPN protocol including
/// handshake initiation, key exchange, and secure transport.
#[allow(dead_code)]
pub struct Protocol {
    /// Protocol configuration parameters
    config: ProtocolConfig,
    /// Current protocol state machine state
    state: ProtocolState,
    /// Whether handshake has been successfully completed
    handshake_complete: bool,
    /// Local session index for this connection
    local_index: u32,
    /// Remote session index from the peer
    remote_index: u32,
}

impl Protocol {
    /// Create a new protocol instance
    pub fn new(config: ProtocolConfig) -> Self {
        Self {
            config,
            state: ProtocolState::Disconnected,
            handshake_complete: false,
            local_index: 0,
            remote_index: 0,
        }
    }

    /// Get current state
    pub fn state(&self) -> ProtocolState {
        self.state
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.state == ProtocolState::Connected && self.handshake_complete
    }

    /// Initiate handshake
    pub fn initiate_handshake(&mut self) -> crate::Result<HandshakeInit> {
        self.state = ProtocolState::Handshaking;

        // Generate ephemeral keys
        let classical_pair = crate::crypto::keys::EphemeralKeyPair::new()
            .map_err(|e| crate::VantisError::CryptoError(e.to_string()))?;
        let (_pqc_pair, pqc_public) = crate::crypto::pqc::KyberKEM::generate_keypair()
            .map_err(|e| crate::VantisError::CryptoError(e.to_string()))?;

        // Store local index
        self.local_index = crate::crypto::random::random_u32()
            .map_err(|e| crate::VantisError::CryptoError(e.to_string()))?;

        let classical_public = classical_pair.public_key().as_bytes().to_vec();

        Ok(HandshakeInit {
            ephemeral_public: classical_public,
            static_public: None,
            pqc_public,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            encrypted: vec![],
        })
    }

    /// Process handshake response
    pub fn process_handshake_response(
        &mut self,
        _response: HandshakeResponse,
    ) -> crate::Result<()> {
        if self.state != ProtocolState::Handshaking {
            return Err(crate::VantisError::InvalidState);
        }

        // Process response and derive shared secrets
        // (Simplified - production would do full handshake)

        self.state = ProtocolState::Connected;
        self.handshake_complete = true;
        self.remote_index = crate::crypto::random::random_u32()
            .map_err(|e| crate::VantisError::CryptoError(e.to_string()))?;

        Ok(())
    }

    /// Create transport message
    pub fn create_transport_message(&mut self, data: &[u8]) -> crate::Result<TransportMessage> {
        if !self.is_connected() {
            return Err(crate::VantisError::NotConnected);
        }

        Ok(TransportMessage {
            receiver: self.remote_index,
            counter: 0, // Would be sequence number
            data: data.to_vec(),
        })
    }

    /// Process transport message
    pub fn process_transport_message(&mut self, msg: TransportMessage) -> crate::Result<Vec<u8>> {
        if !self.is_connected() {
            return Err(crate::VantisError::NotConnected);
        }

        Ok(msg.data)
    }

    /// Close connection
    pub fn close(&mut self) {
        self.state = ProtocolState::Closing;
        self.handshake_complete = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_creation() {
        let config = ProtocolConfig::default();
        let protocol = Protocol::new(config);

        assert_eq!(protocol.state(), ProtocolState::Disconnected);
        assert!(!protocol.is_connected());
    }

    #[test]
    fn test_handshake_initiation() {
        let mut protocol = Protocol::new(ProtocolConfig::default());
        let init = protocol.initiate_handshake().expect("Failed to initiate");

        assert_eq!(protocol.state(), ProtocolState::Handshaking);
        assert!(!init.pqc_public.is_empty());
    }

    #[test]
    fn test_transport_message() {
        let mut protocol = Protocol::new(ProtocolConfig::default());
        protocol.state = ProtocolState::Connected;
        protocol.handshake_complete = true;
        protocol.remote_index = 123;

        let msg = protocol
            .create_transport_message(b"test data")
            .expect("Failed to create message");

        assert_eq!(msg.receiver, 123);
    }
}
