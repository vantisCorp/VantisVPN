//! # Network Layer
//! 
//! Provides networking primitives and protocol implementations for VANTISVPN.
//! 
//! ## Features
//! - QUIC/HTTP3 transport
//! - WireGuard-like protocol with enhancements
//! - IPv6 native support
//! - Kernel bypass (DPDK/eBPF) ready

use serde::{Serialize, Deserialize};

pub mod protocol;
pub mod quic;
pub mod quic_full;
pub mod wireguard;
pub mod wireguard_full;
pub mod stealth;
pub mod multihop;

/// Supported IP protocol versions
///
/// Defines the IP address types supported by VANTISVPN.
/// IPv6 is the primary protocol for DoDI 8310.01 compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpVersion {
    /// IPv4 protocol version 4 (32-bit addresses)
    IPv4,
    /// IPv6 protocol version 6 (128-bit addresses)
    IPv6,
}

/// Network address representation
///
/// Represents either an IPv4 or IPv6 address with validation
/// and conversion utilities for network operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAddress {
    /// IPv4 address (4 bytes)
    IPv4([u8; 4]),
    /// IPv6 address (16 bytes)
    IPv6([u8; 16]),
}

impl NetworkAddress {
    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        match bytes.len() {
            4 => Ok(Self::IPv4(bytes.try_into().unwrap())),
            16 => Ok(Self::IPv6(bytes.try_into().unwrap())),
            _ => Err(crate::VantisError::InvalidAddress),
        }
    }
    
    /// Get as bytes
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::IPv4(addr) => addr,
            Self::IPv6(addr) => addr,
        }
    }
    
    /// Get IP version
    pub fn version(&self) -> IpVersion {
        match self {
            Self::IPv4(_) => IpVersion::IPv4,
            Self::IPv6(_) => IpVersion::IPv6,
        }
    }
    
    /// Check if it's an IPv6 address
    pub fn is_ipv6(&self) -> bool {
        matches!(self, Self::IPv6(_))
    }
}

/// Network endpoint representing an address and port
///
/// Combines a network address with a port number to specify
/// a complete network endpoint for connection establishment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    /// Network address (IPv4 or IPv6)
    pub address: NetworkAddress,
    /// Port number (0-65535)
    pub port: u16,
}

impl Endpoint {
    /// Create a new endpoint
    pub fn new(address: NetworkAddress, port: u16) -> Self {
        Self { address, port }
    }
    
    /// Parse from "address:port" string
    pub fn from_str(s: &str) -> crate::Result<Self> {
        let parts: Vec<&str> = s.rsplitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(crate::VantisError::InvalidEndpoint);
        }
        
        let port: u16 = parts[0].parse()
            .map_err(|_| crate::VantisError::InvalidEndpoint)?;
        let address = NetworkAddress::from_str(parts[1])?;
        
        Ok(Self { address, port })
    }
}

impl NetworkAddress {
    pub fn from_str(s: &str) -> crate::Result<Self> {
        // Parse IPv4 or IPv6 address
        if s.contains(':') && !s.contains('.') {
            // IPv6
            let mut addr = [0u8; 16];
            // Simplified parsing - production would use proper IP parsing
            let parts: Vec<&str> = s.split(':').collect();
            for (i, part) in parts.iter().enumerate() {
                if i >= 8 {
                    break;
                }
                let val = u16::from_str_radix(part, 16)
                    .unwrap_or(0);
                addr[i * 2] = (val >> 8) as u8;
                addr[i * 2 + 1] = val as u8;
            }
            Ok(Self::IPv6(addr))
        } else {
            // IPv4
            let mut addr = [0u8; 4];
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() != 4 {
                return Err(crate::VantisError::InvalidAddress);
            }
            for (i, part) in parts.iter().enumerate() {
                addr[i] = part.parse()
                    .map_err(|_| crate::VantisError::InvalidAddress)?;
            }
            Ok(Self::IPv4(addr))
        }
    }
}

impl std::fmt::Display for NetworkAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IPv4(addr) => write!(f, "{}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3]),
            Self::IPv6(addr) => {
                for i in (0..16).step_by(2) {
                    if i > 0 {
                        write!(f, ":")?;
                    }
                    write!(f, "{:02x}{:02x}", addr[i], addr[i + 1])?;
                }
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.address.is_ipv6() {
            write!(f, "[{}]:{}", self.address, self.port)
        } else {
            write!(f, "{}:{}", self.address, self.port)
        }
    }
}

pub use protocol::MessageType;
pub use wireguard_full::{
    WireGuardDevice, InterfaceConfig, PeerConfig, PeerState, PeerStats,
    HandshakeInitiation, HandshakeResponse, CookieReply, TransportData,
    SessionKeys, ReplayWindow
};
pub use quic_full::{
    QuicEndpoint, QuicConnection, QuicStream, QuicConfig, QuicPacketHeader,
    QuicPacketType, QuicFrame, StreamType, ConnectionState, StreamState,
    Bbrv3State, BbrState, ConnectionStats
};
pub use stealth::{
    StealthHandler, StealthConfig, StealthPacket, PaddingStrategy,
    TlsRecordHeader, Http2FrameHeader
};
pub use multihop::{
    MultiHopManager, MultiHopConfig, Circuit, CircuitHop, CircuitState,
    VpnNode, OnionPacket, CircuitStats
};

/// MTU (Maximum Transmission Unit) for network packets
///
/// Encapsulates MTU values with validation to ensure packets
/// stay within network size limits for reliable transmission.
#[derive(Debug, Clone, Copy)]
pub struct Mtu {
    /// MTU value in bytes (576-9000)
    value: u16,
}

impl Mtu {
    /// Default MTU for VPN
    pub const DEFAULT_VPN: u16 = 1420;
    
    /// Maximum MTU
    pub const MAX: u16 = 9000;
    
    /// Create new MTU
    pub fn new(value: u16) -> crate::Result<Self> {
        if value > Self::MAX {
            return Err(crate::VantisError::InvalidMtu);
        }
        if value < 576 {
            return Err(crate::VantisError::InvalidMtu);
        }
        Ok(Self { value })
    }
    
    /// Get value
    pub fn value(&self) -> u16 {
        self.value
    }
    
    /// Default VPN MTU
    pub fn default_vpn() -> Self {
        Self { value: Self::DEFAULT_VPN }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_address_ipv4() {
        let addr = NetworkAddress::from_str("192.168.1.1").expect("Failed to parse");
        assert!(matches!(addr, NetworkAddress::IPv4(_)));
        
        let display = format!("{}", addr);
        assert_eq!(display, "192.168.1.1");
    }

    #[test]
    fn test_network_address_ipv6() {
        let addr = NetworkAddress::from_str("2001:db8::1").expect("Failed to parse");
        assert!(matches!(addr, NetworkAddress::IPv6(_)));
        assert!(addr.is_ipv6());
    }

    #[test]
    fn test_endpoint() {
        let endpoint = Endpoint::from_str("192.168.1.1:443").expect("Failed to parse");
        assert_eq!(endpoint.port, 443);
        
        let display = format!("{}", endpoint);
        assert_eq!(display, "192.168.1.1:443");
    }

    #[test]
    fn test_mtu() {
        let mtu = Mtu::new(1420).expect("Failed to create MTU");
        assert_eq!(mtu.value(), 1420);
        
        assert!(Mtu::new(0).is_err());
        assert!(Mtu::new(10000).is_err());
    }
}