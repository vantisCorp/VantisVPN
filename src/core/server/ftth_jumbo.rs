// FTTH (Fiber-to-the-Home) Jumbo Frames Support
// Implements support for jumbo frames (up to 9000 bytes) on fiber networks
// Optimizes throughput for high-speed fiber connections

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use crate::error::{VantisError, Result};

/// Jumbo Frame Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Jumbo frame configuration for FTTH
/// 
/// Configuration settings for jumbo frame support in Fiber-to-the-Home
/// networks, enabling larger packet sizes for improved throughput.
pub struct JumboFrameConfig {
    /// Enable jumbo frame support
    pub enabled: bool,
    /// Maximum Transmission Unit size in bytes (typically 9000 for jumbo frames)
    pub mtu: usize,
    /// Enable automatic Path MTU Discovery
    pub enable_mtu_discovery: bool,
    /// Enable frame fragmentation for paths with smaller MTU
    pub enable_fragmentation: bool,
    /// Threshold in bytes at which to fragment frames
    pub fragmentation_threshold: usize,
    /// Enable frame aggregation for efficiency
    pub enable_aggregation: bool,
    /// Timeout for frame aggregation in milliseconds
    pub aggregation_timeout_ms: u64,
    /// Enable caching of Path MTU values
    pub enable_pmtu_cache: bool,
    /// Time-to-live for PMTU cache entries in seconds
    pub pmtu_cache_ttl_secs: u64,
}

impl Default for JumboFrameConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mtu: 9000, // Standard jumbo frame size
            enable_mtu_discovery: true,
            enable_fragmentation: true,
            fragmentation_threshold: 1500, // Fall back to standard Ethernet
            enable_aggregation: true,
            aggregation_timeout_ms: 10,
            enable_pmtu_cache: true,
            pmtu_cache_ttl_secs: 600, // 10 minutes
        }
    }
}

/// Frame Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameType {
    /// Standard Ethernet frame (1500 bytes)
    Standard,
    /// Jumbo frame (1501-9000 bytes)
    Jumbo,
    /// Super jumbo frame (>9000 bytes)
    SuperJumbo,
}

/// Network path for MTU tracking
/// 
/// Represents a network path with its MTU size and jumbo frame support,
/// used for Path MTU Discovery and caching.
#[derive(Debug, Clone)]
pub struct NetworkPath {
    /// Unique identifier for this network path
    pub path_id: String,
    /// Destination address or endpoint
    pub destination: String,
    /// Maximum Transmission Unit for this path
    pub mtu: usize,
    /// Timestamp when this path was last updated
    pub last_updated: std::time::Instant,
    /// Whether jumbo frames are supported on this path
    pub is_jumbo_supported: bool,
}

impl NetworkPath {
    pub fn new(path_id: String, destination: String, mtu: usize) -> Self {
        Self {
            path_id,
            destination,
            mtu,
            last_updated: std::time::Instant::now(),
            is_jumbo_supported: mtu > 1500,
        }
    }

    pub fn is_expired(&self, ttl_secs: u64) -> bool {
        self.last_updated.elapsed() > std::time::Duration::from_secs(ttl_secs)
    }
}

/// Jumbo frame fragment
/// 
/// Represents a fragment of a jumbo frame that has been split for
/// transmission over a path with smaller MTU.
#[derive(Debug, Clone)]
pub struct FrameFragment {
    /// Unique identifier for the original frame
    pub fragment_id: u64,
    /// Index of this fragment in the sequence
    pub fragment_index: u32,
    /// Total number of fragments for the original frame
    pub total_fragments: u32,
    /// Fragment data payload
    pub data: Vec<u8>,
    /// Timestamp when this fragment was created
    pub timestamp: std::time::Instant,
}

impl FrameFragment {
    pub fn new(fragment_id: u64, fragment_index: u32, total_fragments: u32, data: Vec<u8>) -> Self {
        Self {
            fragment_id,
            fragment_index,
            total_fragments,
            data,
            timestamp: std::time::Instant::now(),
        }
    }
}

/// Jumbo frame statistics
/// 
/// Contains statistics about jumbo frame operations, including frame counts,
/// fragmentation metrics, and throughput measurements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumboFrameStats {
    /// Total number of frames sent
    pub total_frames_sent: u64,
    /// Total number of frames received
    pub total_frames_received: u64,
    /// Number of jumbo frames sent (>1500 bytes)
    pub jumbo_frames_sent: u64,
    /// Number of jumbo frames received (>1500 bytes)
    pub jumbo_frames_received: u64,
    /// Number of standard frames sent (≤1500 bytes)
    pub standard_frames_sent: u64,
    /// Number of standard frames received (≤1500 bytes)
    pub standard_frames_received: u64,
    /// Number of frame fragments sent
    pub fragments_sent: u64,
    /// Number of frame fragments received
    pub fragments_received: u64,
    /// Number of frames aggregated
    pub aggregated_frames: u64,
    /// Average frame size in bytes
    pub average_frame_size: f64,
    /// Current throughput in Mbps
    pub throughput_mbps: f64,
    /// Total bytes sent
    pub total_bytes_sent: u64,
    /// Total bytes received
    pub total_bytes_received: u64,
}

/// Jumbo Frame Manager
/// Jumbo frame manager for FTTH
///
/// Manages jumbo frame operations for Fiber-to-the-Home networks,
/// including MTU discovery, fragmentation, and aggregation.
pub struct JumboFrameManager {
    config: JumboFrameConfig,
    paths: Arc<RwLock<HashMap<String, NetworkPath>>>,
    fragment_buffer: Arc<Mutex<HashMap<u64, Vec<FrameFragment>>>>,
    stats: Arc<Mutex<JumboFrameStats>>,
    fragment_counter: Arc<Mutex<u64>>,
}

impl JumboFrameManager {
    pub fn new(config: JumboFrameConfig) -> Self {
        let stats = JumboFrameStats {
            total_frames_sent: 0,
            total_frames_received: 0,
            jumbo_frames_sent: 0,
            jumbo_frames_received: 0,
            standard_frames_sent: 0,
            standard_frames_received: 0,
            fragments_sent: 0,
            fragments_received: 0,
            aggregated_frames: 0,
            average_frame_size: 0.0,
            throughput_mbps: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
        };

        Self {
            config,
            paths: Arc::new(RwLock::new(HashMap::new())),
            fragment_buffer: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
            fragment_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Determine frame type based on size
    pub fn determine_frame_type(&self, size: usize) -> FrameType {
        if size <= 1500 {
            FrameType::Standard
        } else if size <= 9000 {
            FrameType::Jumbo
        } else {
            FrameType::SuperJumbo
        }
    }

    /// Send a frame
    pub async fn send_frame(&self, destination: String, data: Vec<u8>) -> Result<()> {
        let path_mtu = self.get_path_mtu(&destination).await?;

        if data.len() <= path_mtu {
            // Send as single frame
            self.send_single_frame(destination, data).await?;
        } else if self.config.enable_fragmentation {
            // Fragment the frame
            self.send_fragmented_frame(destination, data, path_mtu).await?;
        } else {
            return Err(VantisError::InvalidPeer(
                "Frame too large and fragmentation disabled".to_string(),
            ));
        }

        Ok(())
    }

    /// Send a single frame
    async fn send_single_frame(&self, _destination: String, data: Vec<u8>) -> Result<()> {
        let frame_type = self.determine_frame_type(data.len());

        // In production, this would actually send the frame
        // For now, just update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.total_frames_sent += 1;
            stats.total_bytes_sent += data.len() as u64;

            match frame_type {
                FrameType::Standard => stats.standard_frames_sent += 1,
                FrameType::Jumbo => stats.jumbo_frames_sent += 1,
                FrameType::SuperJumbo => stats.jumbo_frames_sent += 1,
            }
        }

        Ok(())
    }

    /// Send a fragmented frame
    async fn send_fragmented_frame(
        &self,
        _destination: String,
        data: Vec<u8>,
        mtu: usize,
    ) -> Result<()> {
        let fragment_id = {
            let mut counter = self.fragment_counter.lock().await;
            let id = *counter;
            *counter += 1;
            id
        };

        let fragment_size = mtu - 100; // Reserve space for headers
        let total_fragments = ((data.len() as f64) / (fragment_size as f64)).ceil() as u32;

        for i in 0..total_fragments {
            let start = (i as usize) * fragment_size;
            let end = ((i + 1) as usize * fragment_size).min(data.len());
            let fragment_data = data[start..end].to_vec();

            let _fragment = FrameFragment::new(fragment_id, i, total_fragments, fragment_data);

            // In production, this would actually send the fragment
            {
                let mut stats = self.stats.lock().await;
                stats.fragments_sent += 1;
            }
        }

        Ok(())
    }

    /// Receive a frame
    pub async fn receive_frame(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        // Check if this is a fragment
        if self.is_fragment(&data) {
            self.receive_fragment(data).await
        } else {
            // Single frame
            {
                let mut stats = self.stats.lock().await;
                stats.total_frames_received += 1;
                stats.total_bytes_received += data.len() as u64;

                let frame_type = self.determine_frame_type(data.len());
                match frame_type {
                    FrameType::Standard => stats.standard_frames_received += 1,
                    FrameType::Jumbo => stats.jumbo_frames_received += 1,
                    FrameType::SuperJumbo => stats.jumbo_frames_received += 1,
                }
            }

            Ok(data)
        }
    }

    /// Receive a fragment
    async fn receive_fragment(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        // In production, parse fragment header and reassemble
        // For now, return the data as-is
        {
            let mut stats = self.stats.lock().await;
            stats.fragments_received += 1;
        }

        Ok(data)
    }

    /// Check if data is a fragment
    fn is_fragment(&self, _data: &[u8]) -> bool {
        // In production, check fragment header
        false
    }

    /// Get path MTU
    pub async fn get_path_mtu(&self, destination: &str) -> Result<usize> {
        if self.config.enable_pmtu_cache {
            let paths = self.paths.read().await;
            if let Some(path) = paths.get(destination) {
                if !path.is_expired(self.config.pmtu_cache_ttl_secs) {
                    return Ok(path.mtu);
                }
            }
        }

        if self.config.enable_mtu_discovery {
            self.discover_mtu(destination).await
        } else {
            Ok(self.config.mtu)
        }
    }

    /// Discover path MTU
    async fn discover_mtu(&self, destination: &str) -> Result<usize> {
        // In production, perform actual MTU discovery using ICMP
        // For now, return configured MTU
        let mtu = self.config.mtu;

        // Cache the result
        {
            let mut paths = self.paths.write().await;
            paths.insert(
                destination.to_string(),
                NetworkPath::new(
                    format!("path_{}", destination),
                    destination.to_string(),
                    mtu,
                ),
            );
        }

        Ok(mtu)
    }

    /// Update path MTU
    pub async fn update_path_mtu(&self, destination: String, mtu: usize) -> Result<()> {
        let mut paths = self.paths.write().await;
        paths.insert(
            destination.clone(),
            NetworkPath::new(format!("path_{}", destination), destination, mtu),
        );
        Ok(())
    }

    /// Aggregate multiple small frames into a jumbo frame
    pub async fn aggregate_frames(&self, frames: Vec<Vec<u8>>) -> Result<Vec<u8>> {
        if !self.config.enable_aggregation {
            return Err(VantisError::InvalidPeer("Aggregation disabled".to_string()));
        }

        let total_size: usize = frames.iter().map(|f| f.len()).sum();
        if total_size > self.config.mtu {
            return Err(VantisError::InvalidPeer("Aggregated frame too large".to_string()));
        }

        // Aggregate frames
        let mut aggregated = Vec::with_capacity(total_size);
        for frame in frames {
            aggregated.extend_from_slice(&frame);
        }

        {
            let mut stats = self.stats.lock().await;
            stats.aggregated_frames += 1;
        }

        Ok(aggregated)
    }

    /// Get statistics
    pub async fn get_stats(&self) -> JumboFrameStats {
        self.stats.lock().await.clone()
    }

    /// Clean up expired path entries
    pub async fn cleanup_expired_paths(&self) {
        let mut paths = self.paths.write().await;
        paths.retain(|_, path| !path.is_expired(self.config.pmtu_cache_ttl_secs));
    }

    /// Clean up old fragments
    pub async fn cleanup_old_fragments(&self) {
        let mut buffer = self.fragment_buffer.lock().await;
        let timeout = std::time::Duration::from_secs(60); // 1 minute timeout

        buffer.retain(|_, fragments| {
            fragments.iter().any(|f| f.timestamp.elapsed() < timeout)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jumbo_frame_initialization() {
        let config = JumboFrameConfig::default();
        let manager = JumboFrameManager::new(config);
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_frames_sent, 0);
    }

    #[tokio::test]
    async fn test_frame_type_determination() {
        let config = JumboFrameConfig::default();
        let manager = JumboFrameManager::new(config);
        
        assert_eq!(manager.determine_frame_type(1000), FrameType::Standard);
        assert_eq!(manager.determine_frame_type(2000), FrameType::Jumbo);
        assert_eq!(manager.determine_frame_type(10000), FrameType::SuperJumbo);
    }

    #[tokio::test]
    async fn test_send_single_frame() {
        let config = JumboFrameConfig::default();
        let manager = JumboFrameManager::new(config);
        
        let data = vec![1u8; 1000];
        manager.send_frame("192.168.1.1".to_string(), data).await.unwrap();
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_frames_sent, 1);
        assert_eq!(stats.standard_frames_sent, 1);
    }

    #[tokio::test]
    async fn test_send_jumbo_frame() {
        let config = JumboFrameConfig::default();
        let manager = JumboFrameManager::new(config);
        
        let data = vec![1u8; 5000];
        manager.send_frame("192.168.1.1".to_string(), data).await.unwrap();
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_frames_sent, 1);
        assert_eq!(stats.jumbo_frames_sent, 1);
    }

    #[tokio::test]
    async fn test_frame_aggregation() {
        let config = JumboFrameConfig::default();
        let manager = JumboFrameManager::new(config);
        
        let frames = vec![
            vec![1u8; 1000],
            vec![2u8; 1000],
            vec![3u8; 1000],
        ];
        
        let aggregated = manager.aggregate_frames(frames).await.unwrap();
        assert_eq!(aggregated.len(), 3000);
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.aggregated_frames, 1);
    }
}