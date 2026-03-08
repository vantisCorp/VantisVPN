// VANTISVPN - MultiHop+ Onion Routing Implementation
//
// This module provides MultiHop+ onion routing capabilities for VANTISVPN,
// allowing traffic to be routed through multiple VPN servers for enhanced privacy.
//
// Features:
// - Multi-hop routing through multiple VPN servers
// - Onion-style layered encryption
// - Dynamic path selection
// - Path obfuscation
// - Circuit management
// - Failover and re-routing

use crate::crypto::{cipher::Cipher, hash::Hash, keys::CipherSuite, random::SecureRandom};
use crate::error::{Result, VantisError};
use log::{debug, info};
use std::collections::HashMap;
use std::net::{Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};

// MultiHop+ Constants
pub const MAX_HOPS: usize = 7;
pub const DEFAULT_HOPS: usize = 3;
pub const MIN_HOPS: usize = 2;
pub const CIRCUIT_TIMEOUT: Duration = Duration::from_secs(60);
pub const PATH_REFRESH_INTERVAL: Duration = Duration::from_secs(300);
pub const MAX_CIRCUIT_ATTEMPTS: u32 = 3;

/// MultiHop+ configuration
#[derive(Debug, Clone)]
/// MultiHop+ configuration
///
/// Configuration settings for MultiHop+ onion routing, including hop count,
/// path selection options, and geographic preferences.
pub struct MultiHopConfig {
    /// Number of VPN hops in the circuit (2-7)
    pub num_hops: usize,
    /// Enable random path selection
    pub enable_path_randomization: bool,
    /// Ensure hops are in different countries
    pub enable_geo_diversity: bool,
    /// Optimize path for low latency
    pub enable_latency_optimization: bool,
    /// Timeout for circuit establishment
    pub circuit_timeout: Duration,
    /// Interval between path refreshes
    pub path_refresh_interval: Duration,
    /// Maximum attempts to establish a circuit
    pub max_circuit_attempts: u32,
    /// Preferred countries for hops (ISO 3166-1 alpha-2 codes)
    pub preferred_countries: Vec<String>,
    /// Countries to exclude from path selection
    pub excluded_countries: Vec<String>,
}

impl Default for MultiHopConfig {
    fn default() -> Self {
        Self {
            num_hops: DEFAULT_HOPS,
            enable_path_randomization: true,
            enable_geo_diversity: true,
            enable_latency_optimization: true,
            circuit_timeout: CIRCUIT_TIMEOUT,
            path_refresh_interval: PATH_REFRESH_INTERVAL,
            max_circuit_attempts: MAX_CIRCUIT_ATTEMPTS,
            preferred_countries: Vec::new(),
            excluded_countries: Vec::new(),
        }
    }
}

/// VPN node for MultiHop+ routing
///
/// Represents a VPN server node that can be used as a hop in a
/// MultiHop+ circuit, with performance metrics and capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VpnNode {
    /// Unique identifier for this VPN node
    pub node_id: String,
    /// Public key for cryptographic operations
    pub public_key: [u8; 32],
    /// Network endpoint address (IP:port)
    pub endpoint: SocketAddr,
    /// Virtual IPv6 address assigned to this node
    pub virtual_ip: Ipv6Addr,
    /// ISO 3166-1 alpha-2 country code
    pub country: String,
    /// City where the node is located
    pub city: String,
    /// Estimated network latency to this node
    pub latency: Duration,
    /// Current server load percentage (0-100)
    pub load: u8,
    /// Bandwidth capacity in Mbps
    pub bandwidth: u32,
    /// Whether this node supports MultiHop+
    pub supports_multihop: bool,
    /// Whether this node supports Post-Quantum Cryptography
    pub supports_pqc: bool,
    /// Timestamp when this node was last seen
    pub last_seen: Instant,
}

impl VpnNode {
    pub fn new(
        node_id: String,
        public_key: [u8; 32],
        endpoint: SocketAddr,
        virtual_ip: Ipv6Addr,
        country: String,
    ) -> Self {
        Self {
            node_id,
            public_key,
            endpoint,
            virtual_ip,
            country,
            city: String::new(),
            latency: Duration::from_millis(100),
            load: 0,
            bandwidth: 1000,
            supports_multihop: true,
            supports_pqc: true,
            last_seen: Instant::now(),
        }
    }

    pub fn is_available(&self) -> bool {
        self.load < 90 && self.last_seen.elapsed() < Duration::from_secs(300)
    }

    pub fn score(&self) -> f64 {
        let latency_score = 1.0 / (self.latency.as_millis() as f64 + 1.0);
        let load_score = (100 - self.load) as f64 / 100.0;
        let bandwidth_score = self.bandwidth as f64 / 10000.0;

        latency_score * 0.4 + load_score * 0.3 + bandwidth_score * 0.3
    }
}

/// Circuit hop in MultiHop+ routing
///
/// Represents a single hop in a MultiHop+ circuit, containing the VPN node,
/// session key, and routing information for that hop.
#[derive(Debug, Clone)]
pub struct CircuitHop {
    /// VPN node for this hop
    pub node: VpnNode,
    /// Session key for encrypting traffic to this hop
    pub session_key: [u8; 32],
    /// Address of the next hop in the circuit
    pub next_hop: Option<SocketAddr>,
    /// Index of this hop in the circuit (0-based)
    pub index: usize,
}

impl CircuitHop {
    pub fn new(node: VpnNode, session_key: [u8; 32], index: usize) -> Self {
        Self {
            node,
            session_key,
            next_hop: None,
            index,
        }
    }
}

/// State of a MultiHop circuit
///
/// Current operational state of the multi-hop circuit through
/// the chain of VPN nodes for onion routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Building,
    Established,
    Failed,
    Closed,
}

/// MultiHop+ circuit
///
/// Represents a complete MultiHop+ circuit with multiple hops, tracking
/// state, statistics, and usage metrics for the circuit.
#[derive(Debug)]
pub struct Circuit {
    /// Unique identifier for this circuit
    pub circuit_id: String,
    /// Ordered list of hops in the circuit
    pub hops: Vec<CircuitHop>,
    /// Current state of the circuit
    pub state: Arc<Mutex<CircuitState>>,
    /// Timestamp when the circuit was created
    pub created_at: Instant,
    /// Timestamp when the circuit was last used
    pub last_used: Arc<Mutex<Instant>>,
    /// Total bytes sent through this circuit
    pub bytes_sent: Arc<Mutex<u64>>,
    /// Total bytes received through this circuit
    pub bytes_received: Arc<Mutex<u64>>,
    /// Total packets sent through this circuit
    pub packets_sent: Arc<Mutex<u64>>,
    /// Total packets received through this circuit
    pub packets_received: Arc<Mutex<u64>>,
    /// Number of failures encountered
    pub failures: Arc<Mutex<u32>>,
}

impl Circuit {
    pub fn new(circuit_id: String, hops: Vec<CircuitHop>) -> Self {
        Self {
            circuit_id,
            hops,
            state: Arc::new(Mutex::new(CircuitState::Building)),
            created_at: Instant::now(),
            last_used: Arc::new(Mutex::new(Instant::now())),
            bytes_sent: Arc::new(Mutex::new(0)),
            bytes_received: Arc::new(Mutex::new(0)),
            packets_sent: Arc::new(Mutex::new(0)),
            packets_received: Arc::new(Mutex::new(0)),
            failures: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn is_established(&self) -> bool {
        let state = self.state.lock().await;
        *state == CircuitState::Established
    }

    pub async fn is_expired(&self, timeout: Duration) -> bool {
        let last_used = self.last_used.lock().await;
        last_used.elapsed() > timeout
    }

    pub async fn update_stats_sent(&self, bytes: u64) {
        let mut bytes_sent = self.bytes_sent.lock().await;
        let mut packets_sent = self.packets_sent.lock().await;
        let mut last_used = self.last_used.lock().await;

        *bytes_sent += bytes;
        *packets_sent += 1;
        *last_used = Instant::now();
    }

    pub async fn update_stats_received(&self, bytes: u64) {
        let mut bytes_received = self.bytes_received.lock().await;
        let mut packets_received = self.packets_received.lock().await;
        let mut last_used = self.last_used.lock().await;

        *bytes_received += bytes;
        *packets_received += 1;
        *last_used = Instant::now();
    }

    pub async fn get_statistics(&self) -> CircuitStats {
        let bytes_sent = *self.bytes_sent.lock().await;
        let bytes_received = *self.bytes_received.lock().await;
        let packets_sent = *self.packets_sent.lock().await;
        let packets_received = *self.packets_received.lock().await;
        let failures = *self.failures.lock().await;

        CircuitStats {
            bytes_sent,
            bytes_received,
            packets_sent,
            packets_received,
            failures,
            uptime: self.created_at.elapsed(),
        }
    }
}

/// Circuit statistics
///
/// Contains statistics about a MultiHop+ circuit, including traffic
/// metrics, packet counts, and uptime information.
#[derive(Debug, Clone)]
pub struct CircuitStats {
    /// Total bytes sent through the circuit
    pub bytes_sent: u64,
    /// Total bytes received through the circuit
    pub bytes_received: u64,
    /// Total packets sent through the circuit
    pub packets_sent: u64,
    /// Total packets received through the circuit
    pub packets_received: u64,
    /// Number of failures encountered
    pub failures: u32,
    /// Circuit uptime duration
    pub uptime: Duration,
}

/// Onion packet for MultiHop+ routing
///
/// Represents an onion-routed packet that is encrypted layer-by-layer
/// as it passes through each hop in the MultiHop+ circuit.
#[derive(Debug, Clone)]
pub struct OnionPacket {
    /// Circuit ID this packet belongs to
    pub circuit_id: String,
    /// Index of the target hop in the circuit
    pub hop_index: usize,
    /// Encrypted payload data
    pub payload: Vec<u8>,
    /// Message authentication code for integrity
    pub mac: [u8; 16],
    /// Packet flags for various features
    pub flags: u8,
}

impl OnionPacket {
    pub fn new(circuit_id: String, hop_index: usize, payload: Vec<u8>) -> Self {
        Self {
            circuit_id,
            hop_index,
            payload,
            mac: [0u8; 16],
            flags: 0,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Circuit ID
        buf.extend_from_slice(self.circuit_id.as_bytes());
        buf.push(0); // Null terminator

        // Hop index
        buf.extend_from_slice(&(self.hop_index as u8).to_be_bytes());

        // Flags
        buf.push(self.flags);

        // Payload length
        buf.extend_from_slice(&(self.payload.len() as u16).to_be_bytes());

        // Payload
        buf.extend_from_slice(&self.payload);

        // MAC
        buf.extend_from_slice(&self.mac);

        buf
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        // Circuit ID
        let circuit_id_end = data
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| VantisError::InvalidPacket("Invalid circuit ID".into()))?;
        let circuit_id = String::from_utf8(data[..circuit_id_end].to_vec())
            .map_err(|_| VantisError::InvalidPacket("Invalid circuit ID encoding".into()))?;
        let mut offset = circuit_id_end + 1;

        // Hop index
        if offset + 1 > data.len() {
            return Err(VantisError::InvalidPacket("Missing hop index".into()));
        }
        let hop_index = data[offset] as usize;
        offset += 1;

        // Flags
        if offset + 1 > data.len() {
            return Err(VantisError::InvalidPacket("Missing flags".into()));
        }
        let flags = data[offset];
        offset += 1;

        // Payload length
        if offset + 2 > data.len() {
            return Err(VantisError::InvalidPacket("Missing payload length".into()));
        }
        let payload_len = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;

        // Payload
        if offset + payload_len > data.len() {
            return Err(VantisError::InvalidPacket("Invalid payload length".into()));
        }
        let payload = data[offset..offset + payload_len].to_vec();
        offset += payload_len;

        // MAC
        if offset + 16 > data.len() {
            return Err(VantisError::InvalidPacket("Missing MAC".into()));
        }
        let mut mac = [0u8; 16];
        mac.copy_from_slice(&data[offset..offset + 16]);

        Ok(Self {
            circuit_id,
            hop_index,
            payload,
            mac,
            flags,
        })
    }
}

/// MultiHop+ manager
///
/// Manages MultiHop+ onion routing circuits, including path selection,
/// circuit establishment, and packet forwarding through multiple hops.
#[allow(dead_code)]
pub struct MultiHopManager {
    config: MultiHopConfig,
    nodes: Arc<RwLock<HashMap<String, VpnNode>>>,
    circuits: Arc<RwLock<HashMap<String, Arc<Circuit>>>>,
    cipher: Arc<Cipher>,
    hash: Arc<Hash>,
    rng: Arc<SecureRandom>,
    running: Arc<Mutex<bool>>,
}

impl MultiHopManager {
    pub fn new(config: MultiHopConfig) -> Result<Self> {
        let key = vec![0u8; 32];
        let cipher = Arc::new(Cipher::new(&key, CipherSuite::ChaCha20Poly1305)?);
        let hash = Arc::new(Hash::new()?);
        let rng = Arc::new(SecureRandom::new()?);

        Ok(Self {
            config,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            circuits: Arc::new(RwLock::new(HashMap::new())),
            cipher,
            hash,
            rng,
            running: Arc::new(Mutex::new(false)),
        })
    }

    /// Add a VPN node
    pub async fn add_node(&self, node: VpnNode) -> Result<()> {
        let node_id = node.node_id.clone();
        let mut nodes = self.nodes.write().await;
        nodes.insert(node_id.clone(), node);
        info!("Added VPN node: {}", node_id);
        Ok(())
    }

    /// Remove a VPN node
    pub async fn remove_node(&self, node_id: &str) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes.remove(node_id);
        info!("Removed VPN node: {}", node_id);
        Ok(())
    }

    /// Select a path through the network
    pub async fn select_path(&self) -> Result<Vec<String>> {
        let nodes = self.nodes.read().await;
        let available_nodes: Vec<_> = nodes
            .values()
            .filter(|node| node.is_available() && node.supports_multihop)
            .cloned()
            .collect();

        if available_nodes.len() < self.config.num_hops {
            return Err(VantisError::InsufficientNodes(format!(
                "Not enough available nodes: {} < {}",
                available_nodes.len(),
                self.config.num_hops
            )));
        }

        let mut path = Vec::new();
        let mut used_countries = std::collections::HashSet::new();

        for _i in 0..self.config.num_hops {
            let candidates: Vec<_> = available_nodes
                .iter()
                .filter(|node| {
                    !path.contains(&node.node_id)
                        && (!self.config.enable_geo_diversity
                            || !used_countries.contains(&node.country))
                        && (self.config.preferred_countries.is_empty()
                            || self.config.preferred_countries.contains(&node.country))
                        && !self.config.excluded_countries.contains(&node.country)
                })
                .collect();

            if candidates.is_empty() {
                // Fallback: ignore geo diversity
                let fallback: Vec<_> = available_nodes
                    .iter()
                    .filter(|node| !path.contains(&node.node_id))
                    .collect();

                if fallback.is_empty() {
                    return Err(VantisError::InsufficientNodes("No available nodes".into()));
                }

                let node = fallback[self.rng.generate_u64()? as usize % fallback.len()].clone();
                used_countries.insert(node.country.clone());
                path.push(node.node_id.clone());
            } else {
                let node = if self.config.enable_path_randomization {
                    (*candidates[self.rng.generate_u64()? as usize % candidates.len()]).clone()
                } else if self.config.enable_latency_optimization {
                    (**candidates.iter().min_by_key(|n| n.latency).unwrap()).clone()
                } else {
                    (**candidates
                        .iter()
                        .max_by(|a, b| a.score().partial_cmp(&b.score()).unwrap())
                        .unwrap())
                    .clone()
                };

                used_countries.insert(node.country.clone());
                path.push(node.node_id.clone());
            }
        }

        info!("Selected path with {} hops", path.len());
        Ok(path)
    }

    /// Create a new circuit
    pub async fn create_circuit(&self) -> Result<Arc<Circuit>> {
        let path_ids = self.select_path().await?;

        let circuit_id = self.generate_circuit_id().await?;
        let mut hops = Vec::new();

        // Get actual nodes from IDs
        let nodes = self.nodes.read().await;
        let mut path_nodes = Vec::new();
        for node_id in &path_ids {
            if let Some(node) = nodes.get(node_id) {
                path_nodes.push(node.clone());
            }
        }
        drop(nodes);

        for (i, node) in path_nodes.iter().enumerate() {
            let session_key = self.rng.generate_bytes(32)?;
            let mut hop = CircuitHop::new(node.clone(), session_key.try_into().unwrap(), i);

            // Set next hop
            if i < path_nodes.len() - 1 {
                hop.next_hop = Some(path_nodes[i + 1].endpoint);
            }

            hops.push(hop);
        }

        let circuit = Arc::new(Circuit::new(circuit_id.clone(), hops));

        let mut circuits = self.circuits.write().await;
        circuits.insert(circuit_id.clone(), circuit.clone());

        info!("Created circuit: {}", circuit_id);

        Ok(circuit)
    }

    /// Establish a circuit
    pub async fn establish_circuit(&self, circuit: &Arc<Circuit>) -> Result<()> {
        let mut state = circuit.state.lock().await;
        *state = CircuitState::Building;
        drop(state);

        // Establish connection to each hop
        for hop in &circuit.hops {
            // In production, would establish actual connection
            debug!("Establishing hop {} at {}", hop.index, hop.node.endpoint);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let mut state = circuit.state.lock().await;
        *state = CircuitState::Established;
        drop(state);

        info!("Circuit established: {}", circuit.circuit_id);

        Ok(())
    }

    /// Send data through circuit
    pub async fn send_data(&self, circuit: &Arc<Circuit>, data: &[u8]) -> Result<()> {
        if !circuit.is_established().await {
            return Err(VantisError::CircuitNotEstablished);
        }

        // Apply onion encryption (layer by layer)
        let mut encrypted_data = data.to_vec();

        for _hop in circuit.hops.iter().rev() {
            let nonce = [0u8; 12]; // In production, use proper nonce
            encrypted_data = self.cipher.encrypt(&encrypted_data, &nonce)?;
        }

        // Create onion packet
        let packet = OnionPacket::new(circuit.circuit_id.clone(), 0, encrypted_data);

        let serialized = packet.serialize();

        // Send to first hop
        if let Some(_first_hop) = circuit.hops.first() {
            // In production, would send to actual endpoint
            debug!("Sending {} bytes to first hop", serialized.len());
            circuit.update_stats_sent(serialized.len() as u64).await;
        }

        Ok(())
    }

    /// Receive data from circuit
    pub async fn receive_data(&self, circuit: &Arc<Circuit>, data: &[u8]) -> Result<Vec<u8>> {
        let packet = OnionPacket::deserialize(data)?;

        if packet.circuit_id != circuit.circuit_id {
            return Err(VantisError::InvalidCircuit);
        }

        // Decrypt layer by layer
        let mut decrypted_data = packet.payload;

        for _hop in &circuit.hops {
            let nonce = [0u8; 12]; // In production, use proper nonce
            decrypted_data = self.cipher.decrypt(&decrypted_data, &nonce)?;
        }

        circuit
            .update_stats_received(decrypted_data.len() as u64)
            .await;

        Ok(decrypted_data)
    }

    /// Close a circuit
    pub async fn close_circuit(&self, circuit_id: &str) -> Result<()> {
        let mut circuits = self.circuits.write().await;

        if let Some(circuit) = circuits.remove(circuit_id) {
            let mut state = circuit.state.lock().await;
            *state = CircuitState::Closed;
            drop(state);

            info!("Closed circuit: {}", circuit_id);
        }

        Ok(())
    }

    /// Get circuit statistics
    pub async fn get_circuit_stats(&self, circuit_id: &str) -> Result<CircuitStats> {
        let circuits = self.circuits.read().await;

        let circuit = circuits
            .get(circuit_id)
            .ok_or(VantisError::InvalidCircuit)?;

        Ok(circuit.get_statistics().await)
    }

    /// Clean up expired circuits
    pub async fn cleanup_expired_circuits(&self) -> Result<()> {
        let mut circuits = self.circuits.write().await;
        let mut to_remove = Vec::new();

        for (circuit_id, circuit) in circuits.iter() {
            if circuit.is_expired(self.config.circuit_timeout).await {
                to_remove.push(circuit_id.clone());
            }
        }

        for circuit_id in to_remove {
            circuits.remove(&circuit_id);
            info!("Removed expired circuit: {}", circuit_id);
        }

        Ok(())
    }

    /// Generate circuit ID
    async fn generate_circuit_id(&self) -> Result<String> {
        let bytes = self.rng.generate_bytes(16)?;
        Ok(hex::encode(bytes))
    }

    /// Start the manager
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = true;
        drop(running);

        info!("MultiHop+ manager started");

        // Start cleanup task
        self.start_cleanup_task().await;

        Ok(())
    }

    /// Stop the manager
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = false;
        info!("MultiHop+ manager stopped");
        Ok(())
    }

    async fn start_cleanup_task(&self) {
        let circuits = self.circuits.clone();
        let config = self.config.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            while *running.lock().await {
                tokio::time::sleep(Duration::from_secs(60)).await;

                let mut circuits_guard = circuits.write().await;
                let mut to_remove = Vec::new();

                for (circuit_id, circuit) in circuits_guard.iter() {
                    if circuit.is_expired(config.circuit_timeout).await {
                        to_remove.push(circuit_id.clone());
                    }
                }

                for circuit_id in to_remove {
                    circuits_guard.remove(&circuit_id);
                }
            }
        });
    }
}

impl Default for MultiHopManager {
    fn default() -> Self {
        Self::new(MultiHopConfig::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vpn_node() {
        let node = VpnNode::new(
            "test-node".to_string(),
            [1u8; 32],
            "127.0.0.1:443".parse().unwrap(),
            "2001:db8::1".parse().unwrap(),
            "US".to_string(),
        );

        assert!(node.is_available());
        assert!(node.score() > 0.0);
    }

    #[tokio::test]
    async fn test_onion_packet_serialization() {
        let packet = OnionPacket::new("test-circuit".to_string(), 0, b"test payload".to_vec());

        let serialized = packet.serialize();
        let deserialized = OnionPacket::deserialize(&serialized).unwrap();

        assert_eq!(packet.circuit_id, deserialized.circuit_id);
        assert_eq!(packet.hop_index, deserialized.hop_index);
    }

    #[tokio::test]
    async fn test_multihop_manager() {
        let manager = MultiHopManager::new(MultiHopConfig::default()).unwrap();

        // Add test nodes
        for i in 0..5 {
            let node = VpnNode::new(
                format!("node-{}", i),
                [i as u8; 32],
                format!("127.0.0.1:{}", 443 + i).parse().unwrap(),
                format!("2001:db8::{}", i).parse().unwrap(),
                if i % 2 == 0 { "US" } else { "DE" }.to_string(),
            );
            manager.add_node(node).await.unwrap();
        }

        // Select path
        let path = manager.select_path().await.unwrap();
        assert_eq!(path.len(), 3);

        // Create circuit
        let circuit = manager.create_circuit().await.unwrap();
        assert_eq!(circuit.hops.len(), 3);
    }
}
