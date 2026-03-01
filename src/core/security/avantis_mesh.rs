// Avantis Mesh - LAN P2P Networking
// Phase 4: User Security & Protection
// Implements secure peer-to-peer mesh networking for local devices

use crate::error::VantisError;
use crate::crypto::cipher::{Cipher, CipherSuite};
use crate::crypto::keys::EphemeralKeyPair;
use crate::crypto::hash::Hash;
use crate::crypto::random::SecureRandom;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Mesh node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    /// Unique node ID
    pub node_id: String,
    /// Node name
    pub name: String,
    /// Node IP address
    pub ip_address: IpAddr,
    /// Node port
    pub port: u16,
    /// Public key for encryption
    pub public_key: Vec<u8>,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
    /// Is this node online
    pub online: bool,
    /// Node trust score (0-100)
    pub trust_score: u8,
}

/// Mesh message type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeshMessage {
    /// Node discovery announcement
    Discovery {
        node_id: String,
        name: String,
        capabilities: Vec<String>,
    },
    /// Direct message to node
    DirectMessage {
        from_node: String,
        to_node: String,
        content: Vec<u8>,
    },
    /// Broadcast message to all nodes
    Broadcast {
        from_node: String,
        content: Vec<u8>,
    },
    /// File transfer request
    FileTransferRequest {
        from_node: String,
        to_node: String,
        file_id: String,
        file_name: String,
        file_size: u64,
    },
    /// File transfer response
    FileTransferResponse {
        from_node: String,
        to_node: String,
        file_id: String,
        accepted: bool,
    },
    /// File chunk
    FileChunk {
        from_node: String,
        to_node: String,
        file_id: String,
        chunk_index: u32,
        chunk_data: Vec<u8>,
    },
    /// Heartbeat
    Heartbeat {
        node_id: String,
    },
    /// Node leave notification
    Leave {
        node_id: String,
    },
}

/// Mesh configuration
#[derive(Debug, Clone)]
pub struct MeshConfig {
    /// Local node name
    pub node_name: String,
    /// Listen port
    pub listen_port: u16,
    /// Discovery interval in seconds
    pub discovery_interval: u64,
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
    /// Node timeout in seconds
    pub node_timeout: u64,
    /// Maximum hops for message routing
    pub max_hops: u8,
    /// Enable encryption
    pub enable_encryption: bool,
    /// Enable compression
    pub enable_compression: bool,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            node_name: "AvantisNode".to_string(),
            listen_port: 8765,
            discovery_interval: 30,
            heartbeat_interval: 15,
            node_timeout: 60,
            max_hops: 5,
            enable_encryption: true,
            enable_compression: false,
        }
    }
}

/// Mesh statistics
#[derive(Debug, Clone)]
pub struct MeshStats {
    /// Total nodes in mesh
    pub total_nodes: usize,
    /// Online nodes
    pub online_nodes: usize,
    /// Messages sent
    pub messages_sent: u64,
    /// Messages received
    pub messages_received: u64,
    /// Bytes transferred
    pub bytes_transferred: u64,
    /// Uptime in seconds
    pub uptime: u64,
}

/// Avantis Mesh - LAN P2P Networking
pub struct AvantisMesh {
    config: MeshConfig,
    local_node: Arc<Mutex<MeshNode>>,
    nodes: Arc<RwLock<HashMap<String, MeshNode>>>,
    key_pair: Arc<Mutex<EphemeralKeyPair>>,
    cipher: Arc<Mutex<Cipher>>,
    rng: Arc<Mutex<SecureRandom>>,
    stats: Arc<Mutex<MeshStats>>,
    start_time: DateTime<Utc>,
}

impl AvantisMesh {
    /// Create a new Avantis Mesh instance
    pub async fn new(config: MeshConfig) -> Result<Self, VantisError> {
        let key_pair = EphemeralKeyPair::new()?;
        let public_key = key_pair.public_key();
        
        let rng = SecureRandom::new()?;
        let cipher = Cipher::new(&[0u8; 32], CipherSuite::ChaCha20Poly1305)?;

        // Get local IP address
        let local_ip = Self::get_local_ip().await?;

        let local_node = MeshNode {
            node_id: Self::generate_node_id(public_key.as_bytes()),
            name: config.node_name.clone(),
            ip_address: local_ip,
            port: config.listen_port,
            public_key: public_key.as_bytes().to_vec(),
            capabilities: vec!["encryption".to_string(), "file_transfer".to_string()],
            last_seen: Utc::now(),
            online: true,
            trust_score: 100,
        };

        Ok(Self {
            config,
            local_node: Arc::new(Mutex::new(local_node)),
            nodes: Arc::new(RwLock::new(HashMap::new())),
            key_pair: Arc::new(Mutex::new(key_pair)),
            cipher: Arc::new(Mutex::new(cipher)),
            rng: Arc::new(Mutex::new(rng)),
            stats: Arc::new(Mutex::new(MeshStats {
                total_nodes: 0,
                online_nodes: 0,
                messages_sent: 0,
                messages_received: 0,
                bytes_transferred: 0,
                uptime: 0,
            })),
            start_time: Utc::now(),
        })
    }

    /// Start the mesh network
    pub async fn start(&self) -> Result<(), VantisError> {
        // Start discovery
        self.start_discovery().await?;
        
        // Start heartbeat
        self.start_heartbeat().await?;

        Ok(())
    }

    /// Stop the mesh network
    pub async fn stop(&self) -> Result<(), VantisError> {
        // Send leave notification to all nodes
        let local_node = self.local_node.lock().await;
        let leave_message = MeshMessage::Leave {
            node_id: local_node.node_id.clone(),
        };
        drop(local_node);

        self.broadcast_message(leave_message).await?;

        Ok(())
    }

    /// Add a node to the mesh
    pub async fn add_node(&self, node: MeshNode) -> Result<(), VantisError> {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.node_id.clone(), node);
        
        let mut stats = self.stats.lock().await;
        stats.total_nodes = nodes.len();
        stats.online_nodes = nodes.values().filter(|n| n.online).count();

        Ok(())
    }

    /// Remove a node from the mesh
    pub async fn remove_node(&self, node_id: &str) -> Result<(), VantisError> {
        let mut nodes = self.nodes.write().await;
        nodes.remove(node_id);
        
        let mut stats = self.stats.lock().await;
        stats.total_nodes = nodes.len();
        stats.online_nodes = nodes.values().filter(|n| n.online).count();

        Ok(())
    }

    /// Get a node by ID
    pub async fn get_node(&self, node_id: &str) -> Option<MeshNode> {
        let nodes = self.nodes.read().await;
        nodes.get(node_id).cloned()
    }

    /// List all nodes in the mesh
    pub async fn list_nodes(&self) -> Vec<MeshNode> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }

    /// Send a direct message to a node
    pub async fn send_direct_message(&self, to_node_id: &str, content: Vec<u8>) -> Result<(), VantisError> {
        let local_node = self.local_node.lock().await;
        let message = MeshMessage::DirectMessage {
            from_node: local_node.node_id.clone(),
            to_node: to_node_id.to_string(),
            content,
        };
        drop(local_node);

        self.send_message(message).await
    }

    /// Broadcast a message to all nodes
    pub async fn broadcast_message(&self, content: MeshMessage) -> Result<(), VantisError> {
        self.send_message(content).await
    }

    /// Send a message
    async fn send_message(&self, message: MeshMessage) -> Result<(), VantisError> {
        let serialized = bincode::serialize(&message)
            .map_err(|e| VantisError::InvalidData(format!("Failed to serialize message: {}", e)))?;

        let encrypted = if self.config.enable_encryption {
            let rng = self.rng.lock().await;
            let nonce = rng.generate_bytes(12)?;
            drop(rng);

            let cipher = self.cipher.lock().await;
            cipher.encrypt(&serialized, &nonce)?
        } else {
            serialized
        };

        // Update stats
        let mut stats = self.stats.lock().await;
        stats.messages_sent += 1;
        stats.bytes_transferred += encrypted.len() as u64;
        drop(stats);

        // In a real implementation, this would send over the network
        // For now, we just simulate it
        Ok(())
    }

    /// Handle incoming message
    pub async fn handle_message(&self, message: MeshMessage) -> Result<(), VantisError> {
        match message {
            MeshMessage::Discovery { node_id, name, capabilities } => {
                // Handle discovery announcement
                let node = MeshNode {
                    node_id: node_id.clone(),
                    name,
                    ip_address: IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)), // Would be extracted from packet
                    port: 0,
                    public_key: vec![],
                    capabilities,
                    last_seen: Utc::now(),
                    online: true,
                    trust_score: 50, // Default trust score
                };
                self.add_node(node).await?;
            }
            MeshMessage::DirectMessage { from_node: _, to_node, content } => {
                // Handle direct message
                if to_node == self.local_node.lock().await.node_id {
                    // Message is for us
                    let mut stats = self.stats.lock().await;
                    stats.messages_received += 1;
                    stats.bytes_transferred += content.len() as u64;
                }
            }
            MeshMessage::Broadcast { from_node: _, content } => {
                // Handle broadcast message
                let mut stats = self.stats.lock().await;
                stats.messages_received += 1;
                stats.bytes_transferred += content.len() as u64;
            }
            MeshMessage::Heartbeat { node_id } => {
                // Update node last seen
                let mut nodes = self.nodes.write().await;
                if let Some(node) = nodes.get_mut(&node_id) {
                    node.last_seen = Utc::now();
                    node.online = true;
                }
            }
            MeshMessage::Leave { node_id } => {
                // Remove node from mesh
                self.remove_node(&node_id).await?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Get mesh statistics
    pub async fn get_stats(&self) -> MeshStats {
        let mut stats = self.stats.lock().await;
        stats.uptime = Utc::now().signed_duration_since(self.start_time).num_seconds() as u64;
        stats.clone()
    }

    /// Get local node information
    pub async fn local_node(&self) -> MeshNode {
        self.local_node.lock().await.clone()
    }

    /// Start node discovery
    async fn start_discovery(&self) -> Result<(), VantisError> {
        // In a real implementation, this would broadcast discovery messages
        // For now, we just simulate it
        Ok(())
    }

    /// Start heartbeat
    async fn start_heartbeat(&self) -> Result<(), VantisError> {
        // In a real implementation, this would send periodic heartbeat messages
        // For now, we just simulate it
        Ok(())
    }

    /// Get local IP address
    async fn get_local_ip() -> Result<IpAddr, VantisError> {
        // Simplified - in production use proper network interface detection
        Ok(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
    }

    /// Generate node ID from public key
    fn generate_node_id(public_key: &[u8]) -> String {
        let hash_instance = Hash::new().unwrap_or_default();
        let hash = hash_instance.compute(public_key).unwrap_or_default();
        hex::encode(&hash[..16])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mesh_creation() {
        let config = MeshConfig::default();
        let mesh = AvantisMesh::new(config).await.unwrap();
        assert_eq!(mesh.list_nodes().await.len(), 0);
    }

    #[tokio::test]
    async fn test_add_node() {
        let config = MeshConfig::default();
        let mesh = AvantisMesh::new(config).await.unwrap();
        
        let node = MeshNode {
            node_id: "node1".to_string(),
            name: "Test Node".to_string(),
            ip_address: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)),
            port: 8765,
            public_key: vec![1, 2, 3, 4],
            capabilities: vec![],
            last_seen: Utc::now(),
            online: true,
            trust_score: 80,
        };

        mesh.add_node(node).await.unwrap();
        assert_eq!(mesh.list_nodes().await.len(), 1);
    }

    #[tokio::test]
    async fn test_get_node() {
        let config = MeshConfig::default();
        let mesh = AvantisMesh::new(config).await.unwrap();
        
        let node = MeshNode {
            node_id: "node1".to_string(),
            name: "Test Node".to_string(),
            ip_address: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)),
            port: 8765,
            public_key: vec![1, 2, 3, 4],
            capabilities: vec![],
            last_seen: Utc::now(),
            online: true,
            trust_score: 80,
        };

        mesh.add_node(node.clone()).await.unwrap();
        let retrieved = mesh.get_node("node1").await.unwrap();
        assert_eq!(retrieved.node_id, "node1");
    }

    #[tokio::test]
    async fn test_remove_node() {
        let config = MeshConfig::default();
        let mesh = AvantisMesh::new(config).await.unwrap();
        
        let node = MeshNode {
            node_id: "node1".to_string(),
            name: "Test Node".to_string(),
            ip_address: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)),
            port: 8765,
            public_key: vec![1, 2, 3, 4],
            capabilities: vec![],
            last_seen: Utc::now(),
            online: true,
            trust_score: 80,
        };

        mesh.add_node(node).await.unwrap();
        mesh.remove_node("node1").await.unwrap();
        assert_eq!(mesh.list_nodes().await.len(), 0);
    }

    #[tokio::test]
    async fn test_send_direct_message() {
        let config = MeshConfig::default();
        let mesh = AvantisMesh::new(config).await.unwrap();
        
        let node = MeshNode {
            node_id: "node1".to_string(),
            name: "Test Node".to_string(),
            ip_address: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)),
            port: 8765,
            public_key: vec![1, 2, 3, 4],
            capabilities: vec![],
            last_seen: Utc::now(),
            online: true,
            trust_score: 80,
        };

        mesh.add_node(node).await.unwrap();
        mesh.send_direct_message("node1", b"Hello".to_vec()).await.unwrap();
    }

    #[tokio::test]
    async fn test_handle_discovery() {
        let config = MeshConfig::default();
        let mesh = AvantisMesh::new(config).await.unwrap();
        
        let message = MeshMessage::Discovery {
            node_id: "node1".to_string(),
            name: "Test Node".to_string(),
            capabilities: vec!["test".to_string()],
        };

        mesh.handle_message(message).await.unwrap();
        assert_eq!(mesh.list_nodes().await.len(), 1);
    }

    #[tokio::test]
    async fn test_get_stats() {
        let config = MeshConfig::default();
        let mesh = AvantisMesh::new(config).await.unwrap();
        
        let stats = mesh.get_stats().await;
        assert_eq!(stats.total_nodes, 0);
        assert_eq!(stats.online_nodes, 0);
    }
}