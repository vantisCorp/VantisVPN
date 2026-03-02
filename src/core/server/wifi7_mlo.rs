// Wi-Fi 7 Multi-Link Operation (MLO) Support
// Implements IEEE 802.11be MLO for enhanced throughput and reliability
// Supports simultaneous transmission over multiple frequency bands

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use crate::error::{VantisError, Result};

/// Wi-Fi Band
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WifiBand {
    /// 2.4 GHz
    Band24GHz,
    /// 5 GHz
    Band5GHz,
    /// 6 GHz
    Band6GHz,
    /// 60 GHz (mmWave)
    Band60GHz,
}

impl WifiBand {
    pub fn frequency_mhz(&self) -> u32 {
        match self {
            WifiBand::Band24GHz => 2400,
            WifiBand::Band5GHz => 5000,
            WifiBand::Band6GHz => 6000,
            WifiBand::Band60GHz => 60000,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            WifiBand::Band24GHz => "2.4 GHz",
            WifiBand::Band5GHz => "5 GHz",
            WifiBand::Band6GHz => "6 GHz",
            WifiBand::Band60GHz => "60 GHz",
        }
    }
}

/// Link State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkState {
    /// Link not active
    Inactive,
    /// Link establishing
    Establishing,
    /// Link active
    Active,
    /// Link failed
    Failed,
}

/// Wi-Fi Link
#[derive(Debug, Clone)]
/// Wi-Fi 7 link for Multi-Link Operation
/// 
/// Represents a single Wi-Fi link in a Multi-Link Operation (MLO) setup,
/// with performance metrics and link state information.
pub struct WifiLink {
    /// Unique identifier for this link
    pub link_id: String,
    /// Frequency band of the link (2.4GHz, 5GHz, 6GHz)
    pub band: WifiBand,
    /// Channel number used by this link
    pub channel: u32,
    /// Channel bandwidth in MHz
    pub bandwidth_mhz: u32,
    /// Current state of the link
    pub state: LinkState,
    /// Received Signal Strength Indicator in dBm
    pub rssi_dbm: i32,
    /// Signal-to-Noise Ratio in dB
    pub snr_db: f64,
    /// Current throughput in Mbps
    pub throughput_mbps: f64,
    /// Current latency in milliseconds
    pub latency_ms: f64,
    /// Current packet loss rate (0.0-1.0)
    pub packet_loss_rate: f64,
}

impl WifiLink {
    pub fn new(link_id: String, band: WifiBand, channel: u32, bandwidth_mhz: u32) -> Self {
        Self {
            link_id,
            band,
            channel,
            bandwidth_mhz,
            state: LinkState::Inactive,
            rssi_dbm: -100,
            snr_db: 0.0,
            throughput_mbps: 0.0,
            latency_ms: 0.0,
            packet_loss_rate: 0.0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == LinkState::Active
    }

    pub fn quality_score(&self) -> f64 {
        if !self.is_active() {
            return 0.0;
        }

        // Calculate quality score based on RSSI, SNR, and packet loss
        let rssi_score = ((self.rssi_dbm + 100) as f64) / 60.0; // -40 to -100 range
        let snr_score = (self.snr_db / 40.0).min(1.0); // 0 to 40 dB range
        let loss_score = 1.0 - self.packet_loss_rate;

        (rssi_score * 0.4 + snr_score * 0.3 + loss_score * 0.3).max(0.0).min(1.0)
    }
}

/// Multi-Link Operation (MLO) configuration
/// 
/// Configuration settings for Wi-Fi 7 Multi-Link Operation, including
/// link aggregation, failover, and adaptive selection options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MloConfig {
    /// Enable Multi-Link Operation
    pub enabled: bool,
    /// Maximum number of simultaneous links to use
    pub max_links: usize,
    /// Minimum number of active links required
    pub min_active_links: usize,
    /// Enable link aggregation for increased throughput
    pub enable_aggregation: bool,
    /// Enable automatic link failover
    pub enable_failover: bool,
    /// Timeout for link failover in milliseconds
    pub failover_timeout_ms: u64,
    /// Enable adaptive link selection based on quality
    pub enable_adaptive_selection: bool,
    /// Interval for updating link quality metrics in milliseconds
    pub quality_update_interval_ms: u64,
}

impl Default for MloConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_links: 3,
            min_active_links: 1,
            enable_aggregation: true,
            enable_failover: true,
            failover_timeout_ms: 100,
            enable_adaptive_selection: true,
            quality_update_interval_ms: 100,
        }
    }
}

/// Multi-Link Operation (MLO) statistics
/// 
/// Contains statistics about MLO performance, including link counts,
/// throughput metrics, and failover statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MloStats {
    /// Total number of configured links
    pub total_links: usize,
    /// Number of currently active links
    pub active_links: usize,
    /// Total aggregated throughput in Mbps
    pub total_throughput_mbps: f64,
    /// Average latency across all active links in milliseconds
    pub average_latency_ms: f64,
    /// Average packet loss rate across all active links (0.0-1.0)
    pub average_packet_loss_rate: f64,
    /// Total bytes sent through MLO
    pub total_bytes_sent: u64,
    /// Total bytes received through MLO
    pub total_bytes_received: u64,
    /// Number of link failovers that have occurred
    pub failover_count: u64,
}

/// MLO Manager
/// Multi-Link Operation (MLO) manager
///
/// Manages Wi-Fi 7 Multi-Link Operation, including link aggregation,
/// failover, and adaptive link selection for optimal performance.
pub struct MloManager {
    config: MloConfig,
    links: Arc<RwLock<HashMap<String, WifiLink>>>,
    stats: Arc<Mutex<MloStats>>,
    failover_count: Arc<Mutex<u64>>,
}

impl MloManager {
    pub fn new(config: MloConfig) -> Self {
        let stats = MloStats {
            total_links: 0,
            active_links: 0,
            total_throughput_mbps: 0.0,
            average_latency_ms: 0.0,
            average_packet_loss_rate: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            failover_count: 0,
        };

        Self {
            config,
            links: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
            failover_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Add a new Wi-Fi link
    pub async fn add_link(&self, link: WifiLink) -> Result<()> {
        if self.links.read().await.len() >= self.config.max_links {
            return Err(VantisError::InvalidPeer("Maximum number of links reached".to_string()));
        }

        {
            let mut links = self.links.write().await;
            links.insert(link.link_id.clone(), link);
        }

        self.update_stats().await;
        Ok(())
    }

    /// Remove a Wi-Fi link
    pub async fn remove_link(&self, link_id: &str) -> Result<()> {
        {
            let mut links = self.links.write().await;
            links.remove(link_id)
                .ok_or_else(|| VantisError::InvalidPeer(format!("Link not found: {}", link_id)))?;
        }

        self.update_stats().await;
        Ok(())
    }

    /// Get link by ID
    pub async fn get_link(&self, link_id: &str) -> Result<WifiLink> {
        let links = self.links.read().await;
        links
            .get(link_id)
            .cloned()
            .ok_or_else(|| VantisError::InvalidPeer(format!("Link not found: {}", link_id)))
    }

    /// Activate a link
    pub async fn activate_link(&self, link_id: &str) -> Result<()> {
        let mut links = self.links.write().await;
        if let Some(link) = links.get_mut(link_id) {
            link.state = LinkState::Active;
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Link not found: {}", link_id)))
        }
    }

    /// Deactivate a link
    pub async fn deactivate_link(&self, link_id: &str) -> Result<()> {
        let mut links = self.links.write().await;
        if let Some(link) = links.get_mut(link_id) {
            link.state = LinkState::Inactive;
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Link not found: {}", link_id)))
        }
    }

    /// Update link quality metrics
    pub async fn update_link_quality(
        &self,
        link_id: &str,
        rssi_dbm: i32,
        snr_db: f64,
        throughput_mbps: f64,
        latency_ms: f64,
        packet_loss_rate: f64,
    ) -> Result<()> {
        let mut links = self.links.write().await;
        if let Some(link) = links.get_mut(link_id) {
            link.rssi_dbm = rssi_dbm;
            link.snr_db = snr_db;
            link.throughput_mbps = throughput_mbps;
            link.latency_ms = latency_ms;
            link.packet_loss_rate = packet_loss_rate;
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Link not found: {}", link_id)))
        }
    }

    /// Get best link for transmission
    pub async fn get_best_link(&self) -> Result<WifiLink> {
        let links = self.links.read().await;
        let active_links: Vec<_> = links
            .values()
            .filter(|l| l.is_active())
            .collect();

        if active_links.is_empty() {
            return Err(VantisError::InvalidPeer("No active links available".to_string()));
        }

        // Return link with highest quality score
        Ok(active_links
            .into_iter()
            .max_by(|a, b| a.quality_score().partial_cmp(&b.quality_score()).unwrap())
            .unwrap()
            .clone())
    }

    /// Get all active links
    pub async fn get_active_links(&self) -> Vec<WifiLink> {
        let links = self.links.read().await;
        links
            .values()
            .filter(|l| l.is_active())
            .cloned()
            .collect()
    }

    /// Send data using MLO
    pub async fn send_data(&self, data: Vec<u8>) -> Result<()> {
        let active_links = self.get_active_links().await;

        if active_links.is_empty() {
            return Err(VantisError::InvalidPeer("No active links available".to_string()));
        }

        if self.config.enable_aggregation && active_links.len() > 1 {
            // Aggregate across multiple links
            self.send_aggregated(data, active_links).await?;
        } else {
            // Send on best link
            let best_link = self.get_best_link().await?;
            self.send_on_link(&best_link, data).await?;
        }

        Ok(())
    }

    /// Send data aggregated across multiple links
    async fn send_aggregated(&self, data: Vec<u8>, links: Vec<WifiLink>) -> Result<()> {
        // Split data across links
        let chunk_size = (data.len() / links.len()).max(1);
        
        for (i, link) in links.iter().enumerate() {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(data.len());
            
            if start < data.len() {
                let chunk = data[start..end].to_vec();
                self.send_on_link(link, chunk).await?;
            }
        }

        Ok(())
    }

    /// Send data on a specific link
    async fn send_on_link(&self, _link: &WifiLink, data: Vec<u8>) -> Result<()> {
        // In production, this would actually send data over the link
        // For now, just update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.total_bytes_sent += data.len() as u64;
        }

        Ok(())
    }

    /// Handle link failure
    pub async fn handle_link_failure(&self, link_id: &str) -> Result<()> {
        if !self.config.enable_failover {
            return Ok(());
        }

        // Deactivate failed link
        self.deactivate_link(link_id).await?;

        // Check if we have minimum active links
        let active_links = self.get_active_links().await;
        if active_links.len() < self.config.min_active_links {
            // Activate additional links if available
            let all_links = self.links.read().await;
            for link in all_links.values() {
                if !link.is_active() && active_links.len() < self.config.min_active_links {
                    self.activate_link(&link.link_id).await?;
                }
            }
        }

        // Increment failover count
        {
            let mut count = self.failover_count.lock().await;
            *count += 1;
        }

        Ok(())
    }

    /// Get MLO statistics
    pub async fn get_stats(&self) -> MloStats {
        self.stats.lock().await.clone()
    }

    /// Update statistics
    async fn update_stats(&self) {
        let links = self.links.read().await;
        let active_links: Vec<_> = links.values().filter(|l| l.is_active()).collect();

        let mut stats = self.stats.lock().await;
        stats.total_links = links.len();
        stats.active_links = active_links.len();
        stats.total_throughput_mbps = active_links.iter().map(|l| l.throughput_mbps).sum();
        stats.average_latency_ms = if !active_links.is_empty() {
            active_links.iter().map(|l| l.latency_ms).sum::<f64>() / active_links.len() as f64
        } else {
            0.0
        };
        stats.average_packet_loss_rate = if !active_links.is_empty() {
            active_links.iter().map(|l| l.packet_loss_rate).sum::<f64>() / active_links.len() as f64
        } else {
            0.0
        };
        stats.failover_count = *self.failover_count.lock().await;
    }

    /// Start adaptive link selection
    pub async fn start_adaptive_selection(&self) -> tokio::task::JoinHandle<()> {
        let _links = self.links.clone();
        let interval = std::time::Duration::from_millis(self.config.quality_update_interval_ms);

        tokio::spawn(async move {
            let mut timer = tokio::time::interval(interval);
            loop {
                timer.tick().await;
                
                // In production, this would:
                // 1. Monitor link quality
                // 2. Switch to better links if needed
                // 3. Handle link failures
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mlo_initialization() {
        let config = MloConfig::default();
        let manager = MloManager::new(config);
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_links, 0);
    }

    #[tokio::test]
    async fn test_link_addition() {
        let config = MloConfig::default();
        let manager = MloManager::new(config);
        
        let link = WifiLink::new(
            "link1".to_string(),
            WifiBand::Band5GHz,
            36,
            80,
        );
        
        manager.add_link(link).await.unwrap();
        let stats = manager.get_stats().await;
        
        assert_eq!(stats.total_links, 1);
    }

    #[tokio::test]
    async fn test_link_activation() {
        let config = MloConfig::default();
        let manager = MloManager::new(config);
        
        let link = WifiLink::new(
            "link1".to_string(),
            WifiBand::Band5GHz,
            36,
            80,
        );
        
        manager.add_link(link).await.unwrap();
        manager.activate_link("link1").await.unwrap();
        
        let retrieved = manager.get_link("link1").await.unwrap();
        assert!(retrieved.is_active());
    }

    #[tokio::test]
    async fn test_best_link_selection() {
        let config = MloConfig::default();
        let manager = MloManager::new(config);
        
        let mut link1 = WifiLink::new("link1".to_string(), WifiBand::Band5GHz, 36, 80);
        link1.state = LinkState::Active;
        link1.rssi_dbm = -50;
        link1.snr_db = 30.0;
        
        let mut link2 = WifiLink::new("link2".to_string(), WifiBand::Band6GHz, 100, 160);
        link2.state = LinkState::Active;
        link2.rssi_dbm = -40;
        link2.snr_db = 35.0;
        
        manager.add_link(link1).await.unwrap();
        manager.add_link(link2).await.unwrap();
        
        let best = manager.get_best_link().await.unwrap();
        assert_eq!(best.link_id, "link2"); // Better quality
    }
}