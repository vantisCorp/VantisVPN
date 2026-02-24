//! # VANTISVPN Core Library
//! 
//! This is the shared core library for VANTISVPN, providing cryptographic primitives,
//! network protocols, and core functionality shared across all platforms.
//! 
//! ## Architecture
//! 
//! The core is designed with privacy and security as first principles:
//! - Zero-knowledge architecture
//! - Post-quantum cryptography ready
//! - No persistent state
//! - Ephemeral key management
//! 
//! ## Modules
//! 
//! - `crypto`: Cryptographic primitives (PQC, classical crypto)
//! - `network`: Network protocols and tunneling
//! - `tunnel`: VPN tunnel management
//! - `error`: Error types and handling
//! - `config`: Configuration management
//! - `utils`: Utility functions

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod crypto;
pub mod network;
pub mod tunnel;
pub mod error;
pub mod config;
pub mod utils;

// Re-export commonly used types
pub use crypto::{
    keys::{EphemeralKeyPair, Cipher, CipherSuite},
    cipher::CipherMode,
    pqc_full::{MlKemKeyPair, MlDsaKeyPair, HybridKeyExchange, PqcManager},
    hash::Hash,
    random::SecureRandom,
};
pub use network::{
    protocol::{Message, MessageType, ProtocolHandler},
    quic_full::{QuicEndpoint, QuicConnection, QuicStream, QuicConfig},
    wireguard_full::{WireGuardDevice, InterfaceConfig, PeerConfig},
    stealth::{StealthHandler, StealthConfig},
    multihop::{MultiHopManager, MultiHopConfig, Circuit},
};
pub use tunnel::{
    manager::TunnelManager,
    state::TunnelState,
};
pub use error::{VantisError, Result};
pub use config::{Config, AppConfig};

/// VANTISVPN version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum packet size for VPN traffic
pub const MAX_PACKET_SIZE: usize = 1500;

/// Default MTU for VPN tunnel
pub const DEFAULT_MTU: u16 = 1420;

/// Timeout for establishing VPN connection (seconds)
pub const CONNECTION_TIMEOUT: u64 = 30;

/// Heartbeat interval for keepalive (seconds)
pub const HEARTBEAT_INTERVAL: u64 = 10;

/// Number of retransmission attempts
pub const MAX_RETRANSMISSIONS: u32 = 3;

/// Initialize VANTISVPN core
/// 
/// This function must be called before any other core functions.
/// It sets up logging and initializes subsystems.
pub fn init() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    tracing::info!("VANTISVPN Core v{} initialized", VERSION);
    
    // Initialize crypto subsystem
    crypto::init()?;
    
    Ok(())
}

/// Cleanup VANTISVPN core
/// 
/// This function should be called before shutdown to ensure
/// all sensitive data is securely cleared from memory.
pub fn cleanup() -> Result<()> {
    tracing::info!("Cleaning up VANTISVPN Core");
    
    // Securely clear all sensitive data
    crypto::cleanup()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_cleanup() {
        init().expect("Failed to initialize");
        cleanup().expect("Failed to cleanup");
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}