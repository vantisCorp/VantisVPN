//! # VPN Tunnel Management
//! 
//! Manages VPN tunnel lifecycle, state, and routing.

use std::sync::Arc;
use tokio::sync::RwLock;

pub mod state;
pub mod manager;

/// Tunnel statistics and performance metrics
///
/// Tracks real-time statistics for a VPN tunnel including
/// data transfer rates and connection uptime.
#[derive(Debug, Clone, Default)]
pub struct TunnelStats {
    /// Total bytes sent through the tunnel
    pub bytes_sent: u64,
    /// Total bytes received through the tunnel
    pub bytes_received: u64,
    /// Total packets sent through the tunnel
    pub packets_sent: u64,
    /// Total packets received through the tunnel
    pub packets_received: u64,
    /// Connection uptime in seconds
    pub uptime: u64,
}

impl TunnelStats {
    /// Update statistics
    pub fn update(&mut self, bytes_sent: u64, bytes_received: u64) {
        self.bytes_sent += bytes_sent;
        self.bytes_received += bytes_received;
        self.packets_sent += 1;
        self.packets_received += 1;
    }
}

/// Tunnel configuration parameters
///
/// Contains all configurable parameters for a VPN tunnel
/// including server settings, network configuration, and feature toggles.
#[derive(Debug, Clone)]
pub struct TunnelConfig {
    /// Server endpoint address (host:port)
    pub server_endpoint: String,
    /// Virtual IP address assigned to this tunnel
    pub virtual_ip: String,
    /// DNS servers to use while connected
    pub dns_servers: Vec<String>,
    /// Maximum Transmission Unit for the tunnel
    pub mtu: u16,
    /// Enable kill switch to block traffic on disconnect
    pub enable_kill_switch: bool,
    /// Enable split tunneling for selective routing
    pub enable_split_tunneling: bool,
    /// Applications to route through VPN when split tunneling is enabled
    pub split_tunnel_apps: Vec<String>,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            server_endpoint: String::new(),
            virtual_ip: "10.0.0.2".to_string(),
            dns_servers: vec!["1.1.1.1".to_string(), "1.0.0.1".to_string()],
            mtu: 1420,
            enable_kill_switch: true,
            enable_split_tunneling: false,
            split_tunnel_apps: vec![],
        }
    }
}

/// VPN tunnel for secure network connections
///
/// Represents an active VPN tunnel with configuration, state management,
/// and statistics tracking. Supports async operations for connect/disconnect.
pub struct Tunnel {
    /// Unique identifier for this tunnel
    id: String,
    /// Tunnel configuration parameters
    config: TunnelConfig,
    /// Current tunnel state (connecting, connected, etc.)
    state: Arc<RwLock<state::TunnelState>>,
    /// Real-time statistics for this tunnel
    stats: Arc<RwLock<TunnelStats>>,
}

impl Tunnel {
    /// Create a new tunnel
    pub fn new(id: String, config: TunnelConfig) -> Self {
        Self {
            id,
            config,
            state: Arc::new(RwLock::new(state::TunnelState::Disconnected)),
            stats: Arc::new(RwLock::new(TunnelStats::default())),
        }
    }
    
    /// Get tunnel ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get configuration
    pub fn config(&self) -> &TunnelConfig {
        &self.config
    }
    
    /// Get state
    pub async fn state(&self) -> state::TunnelState {
        *self.state.read().await
    }
    
    /// Get statistics
    pub async fn stats(&self) -> TunnelStats {
        self.stats.read().await.clone()
    }
    
    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        matches!(*self.state.read().await, state::TunnelState::Connected)
    }
    
    /// Connect the tunnel
    pub async fn connect(&self) -> crate::Result<()> {
        tracing::info!("Connecting tunnel: {}", self.id);
        
        // Update state
        *self.state.write().await = state::TunnelState::Connecting;
        
        // Simulate connection
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        *self.state.write().await = state::TunnelState::Connected;
        tracing::info!("Tunnel connected: {}", self.id);
        
        Ok(())
    }
    
    /// Disconnect the tunnel
    pub async fn disconnect(&self) -> crate::Result<()> {
        tracing::info!("Disconnecting tunnel: {}", self.id);
        
        *self.state.write().await = state::TunnelState::Disconnecting;
        
        // Simulate disconnection
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        *self.state.write().await = state::TunnelState::Disconnected;
        tracing::info!("Tunnel disconnected: {}", self.id);
        
        Ok(())
    }
    
    /// Send data through tunnel
    pub async fn send(&self, data: &[u8]) -> crate::Result<()> {
        if !self.is_connected().await {
            return Err(crate::VantisError::NotConnected);
        }
        
        let mut stats = self.stats.write().await;
        stats.update(data.len() as u64, 0);
        
        Ok(())
    }
    
    /// Receive data from tunnel
    pub async fn receive(&self) -> crate::Result<Vec<u8>> {
        if !self.is_connected().await {
            return Err(crate::VantisError::NotConnected);
        }
        
        let mut stats = self.stats.write().await;
        stats.update(0, 100); // Simulated receive
        
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tunnel_connection() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel".to_string(), config);
        
        assert_eq!(tunnel.state().await, state::TunnelState::Disconnected);
        
        tunnel.connect().await.expect("Connection failed");
        assert_eq!(tunnel.state().await, state::TunnelState::Connected);
        assert!(tunnel.is_connected().await);
        
        tunnel.disconnect().await.expect("Disconnect failed");
        assert_eq!(tunnel.state().await, state::TunnelState::Disconnected);
    }
}