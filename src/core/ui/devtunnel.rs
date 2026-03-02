// DevTunnel - Developer Tunnel Management
// Phase 6: UX/UI & Additional Features
// Provides secure tunneling for development and testing

use crate::error::VantisError;
use crate::crypto::random::SecureRandom;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Tunnel protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelProtocol {
    /// SSH tunnel
    Ssh,
    /// HTTP tunnel
    Http,
    /// HTTPS tunnel
    Https,
    /// WebSocket tunnel
    WebSocket,
    /// Custom protocol
    Custom,
}

/// Tunnel status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelStatus {
    /// Tunnel not started
    NotStarted,
    /// Tunnel connecting
    Connecting,
    /// Tunnel active
    Active,
    /// Tunnel paused
    Paused,
    /// Tunnel closed
    Closed,
    /// Tunnel failed
    Failed,
}

/// Tunnel configuration
#[derive(Debug, Clone)]
/// DevTunnel configuration
/// 
/// Configuration settings for developer tunnels, including connection
/// parameters, protocol selection, and reconnection behavior.
pub struct TunnelConfig {
    /// Local port to bind for the tunnel
    pub local_port: u16,
    /// Remote host to connect to
    pub remote_host: String,
    /// Remote port on the host
    pub remote_port: u16,
    /// Protocol to use for the tunnel
    pub protocol: TunnelProtocol,
    /// Enable data compression
    pub enable_compression: bool,
    /// Enable end-to-end encryption
    pub enable_encryption: bool,
    /// Connection timeout in seconds
    pub timeout_secs: u64,
    /// Automatically reconnect on connection failure
    pub auto_reconnect: bool,
    /// Maximum number of reconnection attempts
    pub max_reconnect_attempts: u32,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            local_port: 8080,
            remote_host: "localhost".to_string(),
            remote_port: 80,
            protocol: TunnelProtocol::Https,
            enable_compression: true,
            enable_encryption: true,
            timeout_secs: 30,
            auto_reconnect: true,
            max_reconnect_attempts: 3,
        }
    }
}

/// DevTunnel session
/// 
/// Represents an active tunnel session with connection details,
/// status, and usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelSession {
    /// Unique identifier for this session
    pub session_id: String,
    /// ID of the tunnel this session belongs to
    pub tunnel_id: String,
    /// Local socket address
    pub local_address: SocketAddr,
    /// Remote socket address
    pub remote_address: SocketAddr,
    /// Current status of the session
    pub status: TunnelStatus,
    /// Total bytes transferred in this session
    pub bytes_transferred: u64,
    /// Timestamp when the session started
    pub started_at: DateTime<Utc>,
    /// Timestamp of last activity
    pub last_activity: DateTime<Utc>,
}

/// DevTunnel statistics
/// 
/// Contains statistics about developer tunnel operations, including
/// tunnel counts, traffic metrics, and connection statistics.
#[derive(Debug, Clone)]
pub struct TunnelStats {
    /// Total number of tunnels created
    pub total_tunnels: u64,
    /// Number of currently active tunnels
    pub active_tunnels: u64,
    /// Total bytes transferred across all tunnels
    pub total_bytes_transferred: u64,
    /// Total number of connections established
    pub total_connections: u64,
    /// Number of failed connection attempts
    pub failed_connections: u64,
    /// Average connection latency in milliseconds
    pub avg_latency_ms: f64,
}

/// DevTunnel - Developer Tunnel Management
/// DevTunnel manager
///
/// Manages developer tunnels for secure remote access to development
/// environments, supporting multiple protocols and connection management.
pub struct DevTunnel {
    config: TunnelConfig,
    sessions: Arc<Mutex<HashMap<String, TunnelSession>>>,
    stats: Arc<Mutex<TunnelStats>>,
    rng: Arc<Mutex<SecureRandom>>,
    is_active: Arc<Mutex<bool>>,
}

impl DevTunnel {
    /// Create a new DevTunnel instance
    pub fn new(config: TunnelConfig) -> Result<Self, VantisError> {
        let rng = SecureRandom::new()?;
        Ok(Self {
            config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(TunnelStats {
                total_tunnels: 0,
                active_tunnels: 0,
                total_bytes_transferred: 0,
                total_connections: 0,
                failed_connections: 0,
                avg_latency_ms: 0.0,
            })),
            rng: Arc::new(Mutex::new(rng)),
            is_active: Arc::new(Mutex::new(false)),
        })
    }

    /// Start tunnel
    pub async fn start(&self) -> Result<String, VantisError> {
        let mut active = self.is_active.lock().await;
        *active = true;
        drop(active);

        let rng = self.rng.lock().await;
        let tunnel_id = format!("tunnel_{}", hex::encode(rng.generate_bytes(16)?));
        drop(rng);

        let local_address = SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), self.config.local_port);
        let remote_address = SocketAddr::new(
            self.config.remote_host.parse().unwrap_or_else(|_| IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
            self.config.remote_port,
        );

        let session = TunnelSession {
            session_id: tunnel_id.clone(),
            tunnel_id: tunnel_id.clone(),
            local_address,
            remote_address,
            status: TunnelStatus::Active,
            bytes_transferred: 0,
            started_at: Utc::now(),
            last_activity: Utc::now(),
        };

        let mut sessions = self.sessions.lock().await;
        sessions.insert(tunnel_id.clone(), session);

        // Update stats
        let mut stats = self.stats.lock().await;
        stats.total_tunnels += 1;
        stats.active_tunnels += 1;
        stats.total_connections += 1;

        Ok(tunnel_id)
    }

    /// Stop tunnel
    pub async fn stop(&self, tunnel_id: &str) -> Result<(), VantisError> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(tunnel_id) {
            session.status = TunnelStatus::Closed;
            
            // Update stats
            let mut stats = self.stats.lock().await;
            stats.active_tunnels = stats.active_tunnels.saturating_sub(1);
        }

        Ok(())
    }

    /// Pause tunnel
    pub async fn pause(&self, tunnel_id: &str) -> Result<(), VantisError> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(tunnel_id) {
            session.status = TunnelStatus::Paused;
        }
        Ok(())
    }

    /// Resume tunnel
    pub async fn resume(&self, tunnel_id: &str) -> Result<(), VantisError> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(tunnel_id) {
            session.status = TunnelStatus::Active;
        }
        Ok(())
    }

    /// Get tunnel session
    pub async fn get_session(&self, tunnel_id: &str) -> Result<Option<TunnelSession>, VantisError> {
        let sessions = self.sessions.lock().await;
        Ok(sessions.get(tunnel_id).cloned())
    }

    /// Get all active sessions
    pub async fn get_active_sessions(&self) -> Vec<TunnelSession> {
        let sessions = self.sessions.lock().await;
        sessions.values()
            .filter(|s| s.status == TunnelStatus::Active)
            .cloned()
            .collect()
    }

    /// Record data transfer
    pub async fn record_transfer(&self, tunnel_id: &str, bytes: u64) {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(tunnel_id) {
            session.bytes_transferred += bytes;
            session.last_activity = Utc::now();
        }

        let mut stats = self.stats.lock().await;
        stats.total_bytes_transferred += bytes;
    }

    /// Get tunnel statistics
    pub async fn get_stats(&self) -> TunnelStats {
        let stats = self.stats.lock().await;
        stats.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: TunnelConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &TunnelConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devtunnel_creation() {
        let config = TunnelConfig::default();
        let devtunnel = DevTunnel::new(config);
        assert!(devtunnel.is_ok());
    }

    #[test]
    fn test_tunnel_config_default() {
        let config = TunnelConfig::default();
        assert_eq!(config.local_port, 8080);
        assert_eq!(config.remote_port, 80);
        assert_eq!(config.protocol, TunnelProtocol::Https);
    }
}