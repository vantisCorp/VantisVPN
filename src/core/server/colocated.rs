// Colocated Server Infrastructure
// Manages colocated VPN servers across multiple data centers
// Implements load balancing, failover, and geographic distribution

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use rand::Rng;
use rand::rngs::OsRng;
use crate::error::{VantisError, Result};

/// Server Status
/// 
/// Operational status of a VPN server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerStatus {
    /// Server offline
    /// 
    /// Server is not running or cannot be reached.
    Offline,
    /// Server starting
    /// 
    /// Server is in the process of starting up.
    Starting,
    /// Server online
    /// 
    /// Server is running and accepting connections.
    Online,
    /// Server under maintenance
    /// 
    /// Server is under maintenance and not accepting new connections.
    Maintenance,
    /// Server degraded
    /// 
    /// Server is running but with reduced performance or capabilities.
    Degraded,
}

/// Server Location
/// 
/// Geographic location of a VPN server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerLocation {
    /// Country
    /// 
    /// Country where the server is located.
    pub country: String,
    /// City
    /// 
    /// City where the server is located.
    pub city: String,
    /// Region
    /// 
    /// Region or state where the server is located.
    pub region: String,
    /// Latitude
    /// 
    /// Geographic latitude coordinate.
    pub latitude: f64,
    /// Longitude
    /// 
    /// Geographic longitude coordinate.
    pub longitude: f64,
    /// Timezone
    /// 
    /// Timezone of the server location.
    pub timezone: String,
}

impl ServerLocation {
    pub fn new(country: String, city: String, region: String, lat: f64, lon: f64) -> Self {
        Self {
            country,
            city,
            region,
            latitude: lat,
            longitude: lon,
            timezone: "UTC".to_string(),
        }
    }

    /// Calculate distance to another location in kilometers
    pub fn distance_to(&self, other: &ServerLocation) -> f64 {
        // Haversine formula
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        6371.0 * c // Earth's radius in km
    }
}

/// Server Capabilities
/// 
/// Capabilities and resources available on a VPN server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Maximum connections
    /// 
    /// Maximum number of simultaneous VPN connections supported.
    pub max_connections: u32,
    /// Bandwidth in Mbps
    /// 
    /// Available bandwidth in megabits per second.
    pub bandwidth_mbps: u32,
    /// Supports PQC
    /// 
    /// Whether the server supports post-quantum cryptography.
    pub supports_pqc: bool,
    /// Supports stealth
    /// 
    /// Whether the server supports stealth protocol obfuscation.
    pub supports_stealth: bool,
    /// Supports multihop
    /// 
    /// Whether the server supports multi-hop routing.
    pub supports_multihop: bool,
    /// Supports WireGuard
    /// 
    /// Whether the server supports WireGuard protocol.
    pub supports_wireguard: bool,
    /// Supports QUIC
    /// 
    /// Whether the server supports QUIC/HTTP3 transport.
    pub supports_quic: bool,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            max_connections: 10000,
            bandwidth_mbps: 10000,
            supports_pqc: true,
            supports_stealth: true,
            supports_multihop: true,
            supports_wireguard: true,
            supports_quic: true,
        }
    }
}

/// VPN Server
/// 
/// Represents a VPN server in the colocated infrastructure.
#[derive(Debug, Clone)]
pub struct VpnServer {
    /// Server ID
    /// 
    /// Unique identifier for this server.
    pub server_id: String,
    /// Hostname
    /// 
    /// Fully qualified domain name of the server.
    pub hostname: String,
    /// IP address
    /// 
    /// Public IP address of the server.
    pub ip_address: String,
    /// Location
    /// 
    /// Geographic location of the server.
    pub location: ServerLocation,
    /// Status
    /// 
    /// Current operational status of the server.
    pub status: ServerStatus,
    /// Capabilities
    /// 
    /// Capabilities and resources available on this server.
    pub capabilities: ServerCapabilities,
    /// Current connections
    /// 
    /// Number of active VPN connections.
    pub current_connections: u32,
    /// Load percentage
    /// 
    /// Current load as percentage of capacity.
    pub load_percentage: f64,
    /// Uptime in seconds
    /// 
    /// Server uptime in seconds.
    pub uptime_secs: u64,
    /// Last health check
    /// 
    /// Timestamp of the last health check.
    pub last_health_check: std::time::Instant,
}

impl VpnServer {
    pub fn new(
        server_id: String,
        hostname: String,
        ip_address: String,
        location: ServerLocation,
    ) -> Self {
        Self {
            server_id,
            hostname,
            ip_address,
            location,
            status: ServerStatus::Offline,
            capabilities: ServerCapabilities::default(),
            current_connections: 0,
            load_percentage: 0.0,
            uptime_secs: 0,
            last_health_check: std::time::Instant::now(),
        }
    }

    pub fn is_online(&self) -> bool {
        self.status == ServerStatus::Online || self.status == ServerStatus::Degraded
    }

    pub fn is_available(&self) -> bool {
        self.is_online()
            && self.current_connections < self.capabilities.max_connections
            && self.load_percentage < 90.0
    }

    pub fn capacity_score(&self) -> f64 {
        if !self.is_available() {
            return 0.0;
        }

        let connection_score = 1.0
            - (self.current_connections as f64 / self.capabilities.max_connections as f64);
        let load_score = 1.0 - (self.load_percentage / 100.0);

        (connection_score * 0.6 + load_score * 0.4).max(0.0).min(1.0)
    }
}

/// Load Balancing Strategy
/// 
/// Strategies for distributing connections across multiple VPN servers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    /// 
    /// Distribute connections evenly across all servers.
    RoundRobin,
    /// Least connections
    /// 
    /// Route connections to the server with the fewest active connections.
    LeastConnections,
    /// Geographic proximity
    /// 
    /// Route connections to the geographically closest server.
    Geographic,
    /// Weighted by capacity
    /// 
    /// Distribute connections based on server capacity.
    Weighted,
    /// Random selection
    /// 
    /// Randomly select a server for each connection.
    Random,
}

/// Colocated Infrastructure Configuration
/// 
/// Configuration settings for the colocated server infrastructure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColocatedConfig {
    /// Enable automatic failover
    /// 
    /// Whether to automatically failover to other servers when one goes offline.
    pub enable_failover: bool,
    /// Health check interval in seconds
    /// 
    /// Interval between health checks on servers.
    pub health_check_interval_secs: u64,
    /// Health check timeout in seconds
    /// 
    /// Timeout for health check requests.
    pub health_check_timeout_secs: u64,
    /// Maximum failed health checks before marking offline
    /// 
    /// Number of consecutive failed health checks before marking a server offline.
    pub max_failed_health_checks: u32,
    /// Load balancing strategy
    /// 
    /// Strategy for distributing connections across servers.
    pub load_balancing_strategy: LoadBalancingStrategy,
    /// Enable geographic routing
    /// 
    /// Whether to route connections to the geographically closest server.
    pub enable_geographic_routing: bool,
    /// Maximum distance for geographic routing in km
    /// 
    /// Maximum distance for geographic routing in kilometers.
    pub max_geographic_distance_km: f64,
    /// Enable server auto-scaling
    /// 
    /// Whether to automatically scale server capacity based on load.
    pub enable_auto_scaling: bool,
    /// Auto-scaling threshold (load percentage)
    /// 
    /// Load percentage threshold at which to trigger auto-scaling.
    pub auto_scaling_threshold: f64,
}

impl Default for ColocatedConfig {
    fn default() -> Self {
        Self {
            enable_failover: true,
            health_check_interval_secs: 30,
            health_check_timeout_secs: 10,
            max_failed_health_checks: 3,
            load_balancing_strategy: LoadBalancingStrategy::LeastConnections,
            enable_geographic_routing: true,
            max_geographic_distance_km: 5000.0,
            enable_auto_scaling: false,
            auto_scaling_threshold: 80.0,
        }
    }
}

/// Infrastructure Statistics
/// 
/// Statistics about the colocated server infrastructure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureStats {
    /// Total servers
    /// 
    /// Total number of servers in the infrastructure.
    pub total_servers: usize,
    /// Online servers
    /// 
    /// Number of servers currently online.
    pub online_servers: usize,
    /// Offline servers
    /// 
    /// Number of servers currently offline.
    pub offline_servers: usize,
    /// Total connections
    /// 
    /// Total number of active VPN connections across all servers.
    pub total_connections: u32,
    /// Average load percentage
    /// 
    /// Average load percentage across all online servers.
    pub average_load_percentage: f64,
    /// Total bandwidth in Mbps
    /// 
    /// Total bandwidth capacity across all servers.
    pub total_bandwidth_mbps: u32,
    /// Used bandwidth in Mbps
    /// 
    /// Bandwidth currently being used.
    pub used_bandwidth_mbps: u32,
    /// Failover count
    /// 
    /// Number of failover events that have occurred.
    pub failover_count: u64,
}

/// Colocated Infrastructure Manager
/// Colocated Infrastructure Manager
/// 
/// Manages the colocated VPN server infrastructure including load balancing,
/// failover, health checks, and geographic routing.
pub struct ColocatedInfrastructureManager {
    config: ColocatedConfig,
    servers: Arc<RwLock<HashMap<String, VpnServer>>>,
    stats: Arc<Mutex<InfrastructureStats>>,
    round_robin_index: Arc<Mutex<usize>>,
    failover_count: Arc<Mutex<u64>>,
}

impl ColocatedInfrastructureManager {
    pub fn new(config: ColocatedConfig) -> Self {
        let stats = InfrastructureStats {
            total_servers: 0,
            online_servers: 0,
            offline_servers: 0,
            total_connections: 0,
            average_load_percentage: 0.0,
            total_bandwidth_mbps: 0,
            used_bandwidth_mbps: 0,
            failover_count: 0,
        };

        Self {
            config,
            servers: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
            round_robin_index: Arc::new(Mutex::new(0)),
            failover_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Add a server
    pub async fn add_server(&self, server: VpnServer) -> Result<()> {
        {
            let mut servers = self.servers.write().await;
            servers.insert(server.server_id.clone(), server);
        }
        self.update_stats().await;
        Ok(())
    }

    /// Remove a server
    pub async fn remove_server(&self, server_id: &str) -> Result<()> {
        {
            let mut servers = self.servers.write().await;
            servers.remove(server_id)
                .ok_or_else(|| VantisError::InvalidPeer(format!("Server not found: {}", server_id)))?;
        }
        self.update_stats().await;
        Ok(())
    }

    /// Get server by ID
    pub async fn get_server(&self, server_id: &str) -> Result<VpnServer> {
        let servers = self.servers.read().await;
        servers
            .get(server_id)
            .cloned()
            .ok_or_else(|| VantisError::InvalidPeer(format!("Server not found: {}", server_id)))
    }

    /// Get all servers
    pub async fn get_all_servers(&self) -> Vec<VpnServer> {
        let servers = self.servers.read().await;
        servers.values().cloned().collect()
    }

    /// Get online servers
    pub async fn get_online_servers(&self) -> Vec<VpnServer> {
        let servers = self.servers.read().await;
        servers
            .values()
            .filter(|s| s.is_online())
            .cloned()
            .collect()
    }

    /// Get available servers
    pub async fn get_available_servers(&self) -> Vec<VpnServer> {
        let servers = self.servers.read().await;
        servers
            .values()
            .filter(|s| s.is_available())
            .cloned()
            .collect()
    }

    /// Select server for connection
    pub async fn select_server(&self, client_location: Option<ServerLocation>) -> Result<VpnServer> {
        let available_servers = self.get_available_servers().await;

        if available_servers.is_empty() {
            return Err(VantisError::InvalidPeer("No available servers".to_string()));
        }

        let selected = match self.config.load_balancing_strategy {
            LoadBalancingStrategy::RoundRobin => self.select_round_robin(available_servers).await?,
            LoadBalancingStrategy::LeastConnections => self.select_least_connections(available_servers),
            LoadBalancingStrategy::Geographic => {
                if let Some(location) = client_location {
                    self.select_geographic(available_servers, location)?
                } else {
                    self.select_least_connections(available_servers)
                }
            }
            LoadBalancingStrategy::Weighted => self.select_weighted(available_servers),
            LoadBalancingStrategy::Random => self.select_random(available_servers),
        };

        Ok(selected)
    }

    /// Round-robin selection
    async fn select_round_robin(&self, servers: Vec<VpnServer>) -> Result<VpnServer> {
        let mut index = self.round_robin_index.lock().await;
        let selected = servers.get(*index % servers.len())
            .ok_or_else(|| VantisError::InvalidPeer("No servers available".to_string()))?
            .clone();
        *index += 1;
        Ok(selected)
    }

    /// Least connections selection
    fn select_least_connections(&self, servers: Vec<VpnServer>) -> VpnServer {
        servers
            .into_iter()
            .min_by(|a, b| a.current_connections.cmp(&b.current_connections))
            .unwrap()
    }

    /// Geographic selection
    fn select_geographic(&self, servers: Vec<VpnServer>, location: ServerLocation) -> Result<VpnServer> {
        let max_distance = self.config.max_geographic_distance_km;
        
        let nearby_servers: Vec<_> = servers
            .into_iter()
            .filter(|s| s.location.distance_to(&location) <= max_distance)
            .collect();

        if nearby_servers.is_empty() {
            return Err(VantisError::InvalidPeer("No nearby servers".to_string()));
        }

        // Select closest server
        Ok(nearby_servers
            .into_iter()
            .min_by(|a, b| {
                a.location
                    .distance_to(&location)
                    .partial_cmp(&b.location.distance_to(&location))
                    .unwrap()
            })
            .unwrap())
    }

    /// Weighted selection
    fn select_weighted(&self, servers: Vec<VpnServer>) -> VpnServer {
        let total_capacity: f64 = servers.iter().map(|s| s.capacity_score()).sum();
        let mut rng = OsRng;
        let mut threshold: f64 = rng.gen::<f64>() * total_capacity;

        for server in &servers {
            threshold -= server.capacity_score();
            if threshold <= 0.0 {
                return server.clone();
            }
        }

        servers.last().unwrap().clone()
    }

    /// Random selection
    fn select_random(&self, servers: Vec<VpnServer>) -> VpnServer {
        let mut rng = OsRng;
        servers[rng.gen_range(0..servers.len())].clone()
    }

    /// Update server status
    pub async fn update_server_status(
        &self,
        server_id: &str,
        status: ServerStatus,
        current_connections: u32,
        load_percentage: f64,
    ) -> Result<()> {
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.get_mut(server_id) {
            server.status = status;
            server.current_connections = current_connections;
            server.load_percentage = load_percentage;
            server.last_health_check = std::time::Instant::now();
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Server not found: {}", server_id)))
        }
    }

    /// Perform health check on a server
    pub async fn health_check(&self, server_id: &str) -> Result<bool> {
        let server = self.get_server(server_id).await?;

        // In production, this would perform actual health checks
        // For now, simulate health check
        let is_healthy = server.status != ServerStatus::Offline;

        if !is_healthy && self.config.enable_failover {
            self.handle_server_failure(server_id).await?;
        }

        Ok(is_healthy)
    }

    /// Handle server failure
    async fn handle_server_failure(&self, server_id: &str) -> Result<()> {
        // Mark server as offline
        self.update_server_status(server_id, ServerStatus::Offline, 0, 0.0).await?;

        // Increment failover count
        {
            let mut count = self.failover_count.lock().await;
            *count += 1;
        }

        Ok(())
    }

    /// Get servers by country
    pub async fn get_servers_by_country(&self, country: &str) -> Vec<VpnServer> {
        let servers = self.servers.read().await;
        servers
            .values()
            .filter(|s| s.location.country == country)
            .cloned()
            .collect()
    }

    /// Get servers by region
    pub async fn get_servers_by_region(&self, region: &str) -> Vec<VpnServer> {
        let servers = self.servers.read().await;
        servers
            .values()
            .filter(|s| s.location.region == region)
            .cloned()
            .collect()
    }

    /// Get infrastructure statistics
    pub async fn get_stats(&self) -> InfrastructureStats {
        self.stats.lock().await.clone()
    }

    /// Update statistics
    async fn update_stats(&self) {
        let servers = self.servers.read().await;
        let online_servers: Vec<_> = servers.values().filter(|s| s.is_online()).collect();
        let _available_servers: Vec<_> = servers.values().filter(|s| s.is_available()).collect();

        let mut stats = self.stats.lock().await;
        stats.total_servers = servers.len();
        stats.online_servers = online_servers.len();
        stats.offline_servers = servers.len() - online_servers.len();
        stats.total_connections = servers.values().map(|s| s.current_connections).sum();
        stats.average_load_percentage = if !servers.is_empty() {
            servers.values().map(|s| s.load_percentage).sum::<f64>() / servers.len() as f64
        } else {
            0.0
        };
        stats.total_bandwidth_mbps = servers.values().map(|s| s.capabilities.bandwidth_mbps).sum();
        stats.used_bandwidth_mbps = (stats.total_connections as f64 * 10.0) as u32; // Estimate
        stats.failover_count = *self.failover_count.lock().await;
    }

    /// Start health check task
    pub async fn start_health_checks(&self) -> tokio::task::JoinHandle<()> {
        let servers = self.servers.clone();
        let interval = std::time::Duration::from_secs(self.config.health_check_interval_secs);

        tokio::spawn(async move {
            let mut timer = tokio::time::interval(interval);
            loop {
                timer.tick().await;
                
                // Perform health checks on all servers
                let server_ids: Vec<String> = servers.read().await.keys().cloned().collect();
                for server_id in server_ids {
                    // In production, perform actual health check
                    // For now, just update last_health_check
                    let mut servers = servers.write().await;
                    if let Some(server) = servers.get_mut(&server_id) {
                        server.last_health_check = std::time::Instant::now();
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_infrastructure_initialization() {
        let config = ColocatedConfig::default();
        let manager = ColocatedInfrastructureManager::new(config);
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_servers, 0);
    }

    #[tokio::test]
    async fn test_server_addition() {
        let config = ColocatedConfig::default();
        let manager = ColocatedInfrastructureManager::new(config);
        
        let location = ServerLocation::new("US".to_string(), "New York".to_string(), "NA".to_string(), 40.7128, -74.0060);
        let server = VpnServer::new(
            "server1".to_string(),
            "vpn1.example.com".to_string(),
            "192.168.1.1".to_string(),
            location,
        );
        
        manager.add_server(server).await.unwrap();
        let stats = manager.get_stats().await;
        
        assert_eq!(stats.total_servers, 1);
    }

    #[tokio::test]
    async fn test_server_selection() {
        let config = ColocatedConfig::default();
        let manager = ColocatedInfrastructureManager::new(config);
        
        let location = ServerLocation::new("US".to_string(), "New York".to_string(), "NA".to_string(), 40.7128, -74.0060);
        let mut server = VpnServer::new(
            "server1".to_string(),
            "vpn1.example.com".to_string(),
            "192.168.1.1".to_string(),
            location,
        );
        server.status = ServerStatus::Online;
        
        manager.add_server(server).await.unwrap();
        
        let selected = manager.select_server(None).await.unwrap();
        assert_eq!(selected.server_id, "server1");
    }

    #[tokio::test]
    async fn test_geographic_distance() {
        let location1 = ServerLocation::new("US".to_string(), "New York".to_string(), "NA".to_string(), 40.7128, -74.0060);
        let location2 = ServerLocation::new("US".to_string(), "Los Angeles".to_string(), "NA".to_string(), 34.0522, -118.2437);
        
        let distance = location1.distance_to(&location2);
        assert!(distance > 3000.0 && distance < 5000.0); // Approximate distance
    }
}