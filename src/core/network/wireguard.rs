//! # WireGuard-like Protocol Implementation
//!
//! Modified WireGuard protocol with VANTISVPN enhancements:
//! - Post-quantum key exchange
//! - Dynamic IP allocation
//! - Key rotation
//! - Enhanced obfuscation

use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/// WireGuard peer configuration
///
/// Configuration for a WireGuard peer connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConfig {
    /// Public key
    ///
    /// Public key of the peer.
    pub public_key: Vec<u8>,
    /// Pre-shared key (optional)
    ///
    /// Optional pre-shared key for additional security.
    pub preshared_key: Option<Vec<u8>>,
    /// Endpoint
    ///
    /// Network endpoint of the peer.
    pub endpoint: Option<String>,
    /// Allowed IPs (CIDR notation)
    ///
    /// List of allowed IP ranges in CIDR notation.
    pub allowed_ips: Vec<String>,
    /// Keepalive interval (seconds)
    ///
    /// Persistent keepalive interval in seconds.
    pub persistent_keepalive: Option<u32>,
}

/// WireGuard interface configuration
///
/// Configuration for a WireGuard network interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    /// Private key
    ///
    /// Private key for the interface.
    pub private_key: Vec<u8>,
    /// Public key
    ///
    /// Public key for the interface.
    pub public_key: Vec<u8>,
    /// Listen port
    ///
    /// Port to listen on for incoming connections.
    pub listen_port: u16,
    /// MTU
    ///
    /// Maximum Transmission Unit size.
    pub mtu: u16,
    /// Peers
    ///
    /// List of peer configurations.
    pub peers: Vec<PeerConfig>,
}

impl Default for InterfaceConfig {
    fn default() -> Self {
        Self {
            private_key: vec![],
            public_key: vec![],
            listen_port: 51820,
            mtu: 1420,
            peers: vec![],
        }
    }
}

/// WireGuard device
///
/// Represents a WireGuard network device.
pub struct WireGuardDevice {
    config: InterfaceConfig,
    is_up: bool,
}

impl WireGuardDevice {
    /// Create a new WireGuard device
    pub fn new(config: InterfaceConfig) -> Self {
        Self {
            config,
            is_up: false,
        }
    }

    /// Generate a new key pair
    pub fn generate_keypair() -> crate::Result<(Vec<u8>, Vec<u8>)> {
        let key_pair = crate::crypto::keys::EphemeralKeyPair::new()
            .map_err(|e| crate::VantisError::CryptoError(e.to_string()))?;

        let private = key_pair
            .private_key()
            .ok_or_else(|| crate::VantisError::CryptoError("No private key".to_string()))?
            .as_bytes()
            .to_vec();

        let public_key = key_pair.public_key().as_bytes().to_vec();

        Ok((private, public_key))
    }

    /// Bring the interface up
    pub async fn up(&mut self) -> crate::Result<()> {
        if self.is_up {
            return Ok(());
        }

        tracing::info!(
            "Bringing WireGuard interface up (port: {})",
            self.config.listen_port
        );

        // Production would create actual WireGuard device
        self.is_up = true;

        Ok(())
    }

    /// Bring the interface down
    pub async fn down(&mut self) -> crate::Result<()> {
        if !self.is_up {
            return Ok(());
        }

        tracing::info!("Bringing WireGuard interface down");

        // Production would bring down actual WireGuard device
        self.is_up = false;

        Ok(())
    }

    /// Check if interface is up
    pub fn is_up(&self) -> bool {
        self.is_up
    }

    /// Get configuration
    pub fn config(&self) -> &InterfaceConfig {
        &self.config
    }

    /// Add a peer
    pub fn add_peer(&mut self, peer: PeerConfig) {
        self.config.peers.push(peer);
        tracing::debug!("Added peer (total: {})", self.config.peers.len());
    }

    /// Remove peer by public key
    pub fn remove_peer(&mut self, public_key: &[u8]) {
        self.config
            .peers
            .retain(|p| p.public_key.as_slice() != public_key);
        tracing::debug!("Removed peer");
    }
}

/// Virtual IP pool for dynamic allocation
/// Virtual IP pool
///
/// Manages a pool of virtual IP addresses for VPN connections.
pub struct VirtualIpPool {
    base_ip: Ipv4Addr,
    current: u32,
}

impl VirtualIpPool {
    /// Create a new IP pool
    pub fn new(base_ip: Ipv4Addr) -> Self {
        Self {
            base_ip,
            current: 1,
        }
    }

    /// Allocate next IP
    pub fn allocate(&mut self) -> crate::Result<Ipv4Addr> {
        if self.current > 254 {
            return Err(crate::VantisError::IpPoolExhausted);
        }

        let ip = self.base_ip;
        let octets = ip.octets();

        let allocated = Ipv4Addr::new(octets[0], octets[1], octets[2], self.current as u8);
        self.current += 1;

        Ok(allocated)
    }

    /// Reset the pool
    pub fn reset(&mut self) {
        self.current = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let (private, public) = WireGuardDevice::generate_keypair().expect("Failed to generate");
        assert_eq!(private.len(), 32);
        assert_eq!(public.len(), 32);
    }

    #[tokio::test]
    async fn test_device_up_down() {
        let config = InterfaceConfig::default();
        let mut device = WireGuardDevice::new(config);

        assert!(!device.is_up());

        device.up().await.expect("Failed to bring up");
        assert!(device.is_up());

        device.down().await.expect("Failed to bring down");
        assert!(!device.is_up());
    }

    #[test]
    fn test_virtual_ip_pool() {
        let base = Ipv4Addr::new(10, 0, 0, 0);
        let mut pool = VirtualIpPool::new(base);

        let ip1 = pool.allocate().expect("Failed to allocate");
        let ip2 = pool.allocate().expect("Failed to allocate");

        assert_eq!(ip1, Ipv4Addr::new(10, 0, 0, 1));
        assert_eq!(ip2, Ipv4Addr::new(10, 0, 0, 2));
    }
}
