// IP Rotator - Dynamic IP Address Rotation
// Phase 5: Privacy & Identity Management
// Implements automatic IP rotation for enhanced privacy

use crate::crypto::random::SecureRandom;
use crate::error::VantisError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// Strategy for rotating IP addresses to enhance privacy
///
/// Defines different algorithms for selecting new IP endpoints from
/// the available pool, balancing privacy requirements with performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationStrategy {
    /// Rotate on every connection
    PerConnection,
    /// Rotate at fixed time intervals
    TimeInterval,
    /// Rotate after data transfer threshold
    DataThreshold,
    /// Rotate based on geographic location
    Geographic,
    /// Adaptive rotation based on threat level
    Adaptive,
}

/// IP endpoint with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// IP endpoint for VPN connections
///
/// Represents a VPN server endpoint with its IP address, location,
/// performance metrics, and availability status.
pub struct IpEndpoint {
    /// IP address of the endpoint
    pub ip_address: IpAddr,
    /// Port number for the VPN connection
    pub port: u16,
    /// ISO country code where the endpoint is located
    pub country_code: String,
    /// City where the endpoint is located
    pub city: String,
    /// Internet Service Provider hosting the endpoint
    pub isp: String,
    /// Network latency to this endpoint in milliseconds
    pub latency_ms: u32,
    /// Current server load percentage (0-100)
    pub load: u8,
    /// Whether the endpoint is currently available
    pub available: bool,
    /// Timestamp when this endpoint was last used
    pub last_used: DateTime<Utc>,
}

/// IP pool for rotation
///
/// Manages a pool of IP endpoints for rotation, tracking usage
/// statistics and maintaining rotation state.
#[derive(Debug, Clone)]
pub struct IpPool {
    /// Unique identifier for this IP pool
    pub pool_id: String,
    /// Human-readable name for the pool
    pub name: String,
    /// List of available IP endpoints in the pool
    pub endpoints: Vec<IpEndpoint>,
    /// Index of the currently active endpoint
    pub current_index: usize,
    /// Total number of IP rotations performed
    pub total_rotations: u64,
    /// Timestamp when the pool was created
    pub created_at: DateTime<Utc>,
}

/// IP rotator configuration
///
/// Configuration settings for IP rotation, including rotation strategies,
/// thresholds, and geographic diversity options.
#[derive(Debug, Clone)]
pub struct RotatorConfig {
    /// Strategy to use for IP rotation
    pub strategy: RotationStrategy,
    /// Rotation interval in seconds (for TimeInterval strategy)
    pub rotation_interval: u64,
    /// Data threshold in bytes before rotation (for DataThreshold strategy)
    pub data_threshold: u64,
    /// Minimum acceptable latency in milliseconds
    pub min_latency_ms: u32,
    /// Maximum acceptable server load percentage (0-100)
    pub max_load: u8,
    /// Enable geographic diversity in endpoint selection
    pub enable_geo_diversity: bool,
    /// Minimum number of different countries to rotate through
    pub min_countries: usize,
    /// Adaptive threat level threshold for automatic rotation (0-1)
    pub adaptive_threshold: f64,
}

impl Default for RotatorConfig {
    fn default() -> Self {
        Self {
            strategy: RotationStrategy::TimeInterval,
            rotation_interval: 300,             // 5 minutes
            data_threshold: 1024 * 1024 * 1024, // 1 GB
            min_latency_ms: 100,
            max_load: 80,
            enable_geo_diversity: true,
            min_countries: 3,
            adaptive_threshold: 0.7,
        }
    }
}

/// IP rotation statistics
///
/// Contains statistics about IP rotation operations, including
/// rotation counts, timing, and data transfer metrics.
#[derive(Debug, Clone)]
pub struct RotationStats {
    /// Total rotations performed
    pub total_rotations: u64,
    /// Current IP address
    pub current_ip: Option<IpAddr>,
    /// Last rotation time
    pub last_rotation: Option<DateTime<Utc>>,
    /// Next scheduled rotation
    pub next_rotation: Option<DateTime<Utc>>,
    /// Data transferred since last rotation
    pub data_transferred: u64,
    /// Unique IPs used
    pub unique_ips_used: usize,
    /// Countries visited
    pub countries_visited: Vec<String>,
}

/// IP Rotator - Dynamic IP Address Rotation
/// IP rotator manager
///
/// Manages IP address rotation across multiple pools of VPN endpoints,
/// implementing various rotation strategies for enhanced privacy.
pub struct IpRotator {
    config: RotatorConfig,
    pools: Arc<Mutex<HashMap<String, IpPool>>>,
    current_pool: Arc<Mutex<Option<String>>>,
    current_endpoint: Arc<Mutex<Option<IpEndpoint>>>,
    stats: Arc<Mutex<RotationStats>>,
    rng: Arc<Mutex<SecureRandom>>,
    last_rotation_time: Arc<Mutex<Option<Instant>>>,
    data_transferred: Arc<Mutex<u64>>,
    start_time: Instant,
}

impl IpRotator {
    /// Create a new IP Rotator
    pub fn new(config: RotatorConfig) -> Result<Self, VantisError> {
        let rng = SecureRandom::new()?;
        Ok(Self {
            config,
            pools: Arc::new(Mutex::new(HashMap::new())),
            current_pool: Arc::new(Mutex::new(None)),
            current_endpoint: Arc::new(Mutex::new(None)),
            stats: Arc::new(Mutex::new(RotationStats {
                total_rotations: 0,
                current_ip: None,
                last_rotation: None,
                next_rotation: None,
                data_transferred: 0,
                unique_ips_used: 0,
                countries_visited: Vec::new(),
            })),
            rng: Arc::new(Mutex::new(rng)),
            last_rotation_time: Arc::new(Mutex::new(None)),
            data_transferred: Arc::new(Mutex::new(0)),
            start_time: Instant::now(),
        })
    }

    /// Add IP pool to rotator
    pub async fn add_pool(&self, pool: IpPool) -> Result<(), VantisError> {
        let mut pools = self.pools.lock().await;
        pools.insert(pool.pool_id.clone(), pool);
        Ok(())
    }

    /// Remove IP pool from rotator
    pub async fn remove_pool(&self, pool_id: &str) -> Result<(), VantisError> {
        let mut pools = self.pools.lock().await;
        pools.remove(pool_id);
        Ok(())
    }

    /// Set current IP pool
    pub async fn set_pool(&self, pool_id: &str) -> Result<(), VantisError> {
        let pools = self.pools.lock().await;
        if pools.contains_key(pool_id) {
            let mut current_pool = self.current_pool.lock().await;
            *current_pool = Some(pool_id.to_string());
            Ok(())
        } else {
            Err(VantisError::NotFound(format!("Pool {} not found", pool_id)))
        }
    }

    /// Get current IP endpoint
    pub async fn get_current_endpoint(&self) -> Result<Option<IpEndpoint>, VantisError> {
        let endpoint = self.current_endpoint.lock().await;
        Ok(endpoint.clone())
    }

    /// Rotate to next IP endpoint
    pub async fn rotate(&self) -> Result<IpEndpoint, VantisError> {
        let pool_id = self
            .current_pool
            .lock()
            .await
            .clone()
            .ok_or_else(|| VantisError::InvalidState)?;

        let mut pools = self.pools.lock().await;
        let pool = pools
            .get_mut(&pool_id)
            .ok_or_else(|| VantisError::NotFound(format!("Pool {} not found", pool_id)))?;

        // Find available endpoints
        let available: Vec<_> = pool
            .endpoints
            .iter()
            .filter(|e| e.available && e.load <= self.config.max_load)
            .collect();

        if available.is_empty() {
            return Err(VantisError::NotFound("No available endpoints".to_string()));
        }

        // Select endpoint based on strategy
        let selected = self.select_endpoint(&available).await?;

        // Update pool state
        pool.current_index = pool
            .endpoints
            .iter()
            .position(|e| e.ip_address == selected.ip_address)
            .unwrap_or(0);
        pool.total_rotations += 1;

        // Update current endpoint
        let mut current_endpoint = self.current_endpoint.lock().await;
        *current_endpoint = Some(selected.clone());

        // Update rotation time
        let mut last_rotation = self.last_rotation_time.lock().await;
        *last_rotation = Some(Instant::now());

        // Reset data transferred
        let mut data = self.data_transferred.lock().await;
        *data = 0;

        // Update stats
        let mut stats = self.stats.lock().await;
        stats.total_rotations += 1;
        stats.current_ip = Some(selected.ip_address);
        stats.last_rotation = Some(Utc::now());
        stats.data_transferred = 0;

        // Track unique IPs and countries
        if !stats.countries_visited.contains(&selected.country_code) {
            stats.countries_visited.push(selected.country_code.clone());
        }

        Ok(selected)
    }

    /// Select endpoint based on strategy
    async fn select_endpoint(&self, available: &[&IpEndpoint]) -> Result<IpEndpoint, VantisError> {
        match self.config.strategy {
            RotationStrategy::PerConnection => {
                // Random selection
                let rng = self.rng.lock().await;
                let index = rng.generate_u32()? as usize % available.len();
                Ok(available[index].clone())
            },
            RotationStrategy::TimeInterval => {
                // Select based on latency
                let best = available
                    .iter()
                    .min_by_key(|e| e.latency_ms)
                    .ok_or_else(|| VantisError::NotFound("No endpoints available".to_string()))?;
                Ok((*best).clone())
            },
            RotationStrategy::DataThreshold => {
                // Select based on load
                let best = available
                    .iter()
                    .min_by_key(|e| e.load)
                    .ok_or_else(|| VantisError::NotFound("No endpoints available".to_string()))?;
                Ok((*best).clone())
            },
            RotationStrategy::Geographic => {
                // Select from different country if possible
                let current_country = self
                    .current_endpoint
                    .lock()
                    .await
                    .as_ref()
                    .map(|e| e.country_code.clone());

                if let Some(country) = current_country {
                    let different_country = available.iter().find(|e| e.country_code != country);

                    if let Some(endpoint) = different_country {
                        return Ok((*endpoint).clone());
                    }
                }

                // Fallback to random
                let rng = self.rng.lock().await;
                let index = rng.generate_u32()? as usize % available.len();
                Ok(available[index].clone())
            },
            RotationStrategy::Adaptive => {
                // Select based on combined metrics
                let best = available
                    .iter()
                    .min_by(|a, b| {
                        let score_a = (a.latency_ms as f64) * 0.5 + (a.load as f64) * 0.5;
                        let score_b = (b.latency_ms as f64) * 0.5 + (b.load as f64) * 0.5;
                        score_a
                            .partial_cmp(&score_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .ok_or_else(|| VantisError::NotFound("No endpoints available".to_string()))?;
                Ok((*best).clone())
            },
        }
    }

    /// Check if rotation is needed
    pub async fn should_rotate(&self) -> Result<bool, VantisError> {
        match self.config.strategy {
            RotationStrategy::PerConnection => Ok(true),
            RotationStrategy::TimeInterval => {
                let last_rotation = self.last_rotation_time.lock().await;
                if let Some(last) = *last_rotation {
                    let elapsed = last.elapsed().as_secs();
                    Ok(elapsed >= self.config.rotation_interval)
                } else {
                    Ok(true)
                }
            },
            RotationStrategy::DataThreshold => {
                let data = self.data_transferred.lock().await;
                Ok(*data >= self.config.data_threshold)
            },
            RotationStrategy::Geographic => {
                // Rotate periodically for geographic diversity
                let last_rotation = self.last_rotation_time.lock().await;
                if let Some(last) = *last_rotation {
                    let elapsed = last.elapsed().as_secs();
                    Ok(elapsed >= self.config.rotation_interval)
                } else {
                    Ok(true)
                }
            },
            RotationStrategy::Adaptive => {
                // Adaptive rotation based on threat level
                let last_rotation = self.last_rotation_time.lock().await;
                if let Some(last) = *last_rotation {
                    let elapsed = last.elapsed().as_secs();
                    let threshold = (self.config.rotation_interval as f64)
                        * (1.0 - self.config.adaptive_threshold);
                    Ok(elapsed >= threshold as u64)
                } else {
                    Ok(true)
                }
            },
        }
    }

    /// Record data transfer
    pub async fn record_transfer(&self, bytes: u64) {
        let mut data = self.data_transferred.lock().await;
        *data += bytes;

        let mut stats = self.stats.lock().await;
        stats.data_transferred += bytes;
    }

    /// Get rotation statistics
    pub async fn get_stats(&self) -> RotationStats {
        let stats = self.stats.lock().await;
        stats.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: RotatorConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &RotatorConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotator_creation() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config);
        assert!(rotator.is_ok());
    }

    #[test]
    fn test_pool_creation() {
        let endpoint = IpEndpoint {
            ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            port: 443,
            country_code: "US".to_string(),
            city: "New York".to_string(),
            isp: "Test ISP".to_string(),
            latency_ms: 50,
            load: 30,
            available: true,
            last_used: Utc::now(),
        };

        let pool = IpPool {
            pool_id: "test_pool".to_string(),
            name: "Test Pool".to_string(),
            endpoints: vec![endpoint],
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        };

        assert_eq!(pool.endpoints.len(), 1);
    }
}
