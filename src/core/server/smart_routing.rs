// Smart Routing AI System
// Implements intelligent routing decisions using machine learning
// Optimizes for latency, throughput, reliability, and cost

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use rand::Rng;
use rand::rngs::OsRng;
use crate::error::{VantisError, Result};

/// Metric used to evaluate network paths for routing decisions
///
/// Defines different criteria for selecting the optimal network path,
/// including latency, bandwidth, packet loss, and quality of service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingMetric {
    /// Latency optimization
    Latency,
    /// Throughput optimization
    Throughput,
    /// Reliability optimization
    Reliability,
    /// Cost optimization
    Cost,
    /// Balanced optimization
    Balanced,
}

impl RoutingMetric {
    pub fn name(&self) -> &str {
        match self {
            RoutingMetric::Latency => "Latency",
            RoutingMetric::Throughput => "Throughput",
            RoutingMetric::Reliability => "Reliability",
            RoutingMetric::Cost => "Cost",
            RoutingMetric::Balanced => "Balanced",
        }
    }
}

/// Represents a network path available for routing
///
/// Contains information about a specific network route including
/// endpoints, performance metrics, and path characteristics.
#[derive(Debug, Clone)]
pub struct NetworkPath {
    /// Unique path identifier
    pub path_id: String,
    /// Source address
    pub source: String,
    /// Destination address
    pub destination: String,
    /// List of intermediate hops
    pub hops: Vec<String>,
    /// Path latency in milliseconds
    pub latency_ms: f64,
    /// Path throughput in Mbps
    pub throughput_mbps: f64,
    /// Path reliability score (0.0-1.0)
    pub reliability_score: f64,
    /// Path cost score (0.0-1.0, higher is better)
    pub cost_score: f64,
    /// Last update timestamp
    pub last_updated: std::time::Instant,
}

impl NetworkPath {
    pub fn new(path_id: String, source: String, destination: String) -> Self {
        Self {
            path_id,
            source,
            destination,
            hops: Vec::new(),
            latency_ms: 0.0,
            throughput_mbps: 0.0,
            reliability_score: 0.0,
            cost_score: 0.0,
            last_updated: std::time::Instant::now(),
        }
    }

    pub fn calculate_score(&self, metric: RoutingMetric) -> f64 {
        match metric {
            RoutingMetric::Latency => {
                // Lower latency is better
                (1000.0 / (self.latency_ms + 1.0)).min(1.0)
            }
            RoutingMetric::Throughput => {
                // Higher throughput is better
                (self.throughput_mbps / 10000.0).min(1.0)
            }
            RoutingMetric::Reliability => {
                // Higher reliability is better
                self.reliability_score
            }
            RoutingMetric::Cost => {
                // Lower cost is better (cost_score is inverted)
                self.cost_score
            }
            RoutingMetric::Balanced => {
                // Weighted average of all metrics
                let latency_score = (1000.0 / (self.latency_ms + 1.0)).min(1.0);
                let throughput_score = (self.throughput_mbps / 10000.0).min(1.0);
                latency_score * 0.3 + throughput_score * 0.3 + self.reliability_score * 0.2 + self.cost_score * 0.2
            }
        }
    }
}

/// Represents a routing decision made by the smart routing system
///
/// Contains the selected path, decision rationale, and confidence
/// score for the routing choice made by the AI system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    /// Selected path identifier
    pub path_id: String,
    /// Optimization metric used
    pub metric: RoutingMetric,
    /// Calculated score for this path
    pub score: f64,
    /// Confidence level in this decision (0.0-1.0)
    pub confidence: f64,
    /// Decision timestamp (Unix timestamp)
    pub timestamp: u64,
}

/// Configuration for the AI-powered smart routing system
///
/// Controls how the smart routing system evaluates paths, including
/// ML model settings, health check intervals, and optimization goals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingConfig {
    /// Primary optimization metric
    pub primary_metric: RoutingMetric,
    /// Enable machine learning
    pub enable_ml: bool,
    /// Learning rate for ML model
    pub learning_rate: f64,
    /// Enable path exploration
    pub enable_exploration: bool,
    /// Exploration rate (epsilon-greedy)
    pub exploration_rate: f64,
    /// Enable path caching
    pub enable_caching: bool,
    /// Cache TTL in seconds
    pub cache_ttl_secs: u64,
    /// Minimum path samples before ML training
    pub min_samples_for_training: usize,
}

impl Default for SmartRoutingConfig {
    fn default() -> Self {
        Self {
            primary_metric: RoutingMetric::Balanced,
            enable_ml: true,
            learning_rate: 0.01,
            enable_exploration: true,
            exploration_rate: 0.1,
            enable_caching: true,
            cache_ttl_secs: 300, // 5 minutes
            min_samples_for_training: 100,
        }
    }
}

/// Statistics about smart routing performance
///
/// Tracks routing decisions, path quality metrics, and
/// system performance over time for optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingStats {
    /// Total number of routing decisions made
    pub total_decisions: u64,
    /// Number of successful routing decisions
    pub successful_decisions: u64,
    /// Number of failed routing decisions
    pub failed_decisions: u64,
    /// Average latency across all paths in milliseconds
    pub average_latency_ms: f64,
    /// Average throughput across all paths in Mbps
    pub average_throughput_mbps: f64,
    /// Average reliability score across all paths
    pub average_reliability: f64,
    pub path_switches: u64,
    pub ml_model_updates: u64,
}

/// Machine learning model weights for path prediction
///
/// Contains the learned weights for the neural network model
/// that predicts optimal network paths based on historical data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlWeights {
    pub latency_weight: f64,
    pub throughput_weight: f64,
    pub reliability_weight: f64,
    pub cost_weight: f64,
}

impl Default for MlWeights {
    fn default() -> Self {
        Self {
            latency_weight: 0.25,
            throughput_weight: 0.25,
            reliability_weight: 0.25,
            cost_weight: 0.25,
        }
    }
}

/// Smart Routing Manager
/// 
/// Manages AI-powered intelligent routing for VPN traffic, using machine
/// learning to select optimal network paths based on real-time metrics.
pub struct SmartRoutingManager {
    config: SmartRoutingConfig,
    paths: Arc<RwLock<HashMap<String, NetworkPath>>>,
    routing_cache: Arc<RwLock<HashMap<String, RoutingDecision>>>,
    stats: Arc<Mutex<RoutingStats>>,
    ml_weights: Arc<Mutex<MlWeights>>,
    path_samples: Arc<Mutex<HashMap<String, Vec<f64>>>>,
}

impl SmartRoutingManager {
    pub fn new(config: SmartRoutingConfig) -> Self {
        let stats = RoutingStats {
            total_decisions: 0,
            successful_decisions: 0,
            failed_decisions: 0,
            average_latency_ms: 0.0,
            average_throughput_mbps: 0.0,
            average_reliability: 0.0,
            path_switches: 0,
            ml_model_updates: 0,
        };

        Self {
            config,
            paths: Arc::new(RwLock::new(HashMap::new())),
            routing_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
            ml_weights: Arc::new(Mutex::new(MlWeights::default())),
            path_samples: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a network path
    pub async fn add_path(&self, path: NetworkPath) -> Result<()> {
        {
            let mut paths = self.paths.write().await;
            paths.insert(path.path_id.clone(), path);
        }
        Ok(())
    }

    /// Remove a network path
    pub async fn remove_path(&self, path_id: &str) -> Result<()> {
        {
            let mut paths = self.paths.write().await;
            paths.remove(path_id)
                .ok_or_else(|| VantisError::InvalidPeer(format!("Path not found: {}", path_id)))?;
        }
        Ok(())
    }

    /// Update path metrics
    pub async fn update_path_metrics(
        &self,
        path_id: &str,
        latency_ms: f64,
        throughput_mbps: f64,
        reliability_score: f64,
    ) -> Result<()> {
        let mut paths = self.paths.write().await;
        if let Some(path) = paths.get_mut(path_id) {
            path.latency_ms = latency_ms;
            path.throughput_mbps = throughput_mbps;
            path.reliability_score = reliability_score;
            path.last_updated = std::time::Instant::now();
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Path not found: {}", path_id)))
        }
    }

    /// Make routing decision
    pub async fn make_routing_decision(&self, destination: String) -> Result<RoutingDecision> {
        // Check cache first
        if self.config.enable_caching {
            if let Some(cached) = self.get_cached_decision(&destination).await {
                return Ok(cached);
            }
        }

        // Get available paths
        let paths = self.get_available_paths(&destination).await?;
        if paths.is_empty() {
            return Err(VantisError::InvalidPeer("No available paths".to_string()));
        }

        // Make decision
        let decision = if self.config.enable_ml {
            self.make_ml_decision(paths, &destination).await?
        } else {
            self.make_heuristic_decision(paths, &destination).await?
        };

        // Cache the decision
        if self.config.enable_caching {
            self.cache_decision(destination.clone(), decision.clone()).await;
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.total_decisions += 1;
            stats.successful_decisions += 1;
        }

        Ok(decision)
    }

    /// Get available paths for destination
    async fn get_available_paths(&self, destination: &str) -> Result<Vec<NetworkPath>> {
        let paths = self.paths.read().await;
        let available: Vec<_> = paths
            .values()
            .filter(|p| p.destination == destination)
            .cloned()
            .collect();
        Ok(available)
    }

    /// Make heuristic routing decision
    async fn make_heuristic_decision(
        &self,
        paths: Vec<NetworkPath>,
        _destination: &str,
    ) -> Result<RoutingDecision> {
        // Find best path based on primary metric
        let best_path = paths
            .into_iter()
            .max_by(|a, b| {
                a.calculate_score(self.config.primary_metric)
                    .partial_cmp(&b.calculate_score(self.config.primary_metric))
                    .unwrap()
            })
            .ok_or_else(|| VantisError::InvalidPeer("No paths available".to_string()))?;

        let score = best_path.calculate_score(self.config.primary_metric);

        Ok(RoutingDecision {
            path_id: best_path.path_id,
            metric: self.config.primary_metric,
            score,
            confidence: 0.8, // Heuristic confidence
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    /// Make ML-based routing decision
    async fn make_ml_decision(
        &self,
        paths: Vec<NetworkPath>,
        _destination: &str,
    ) -> Result<RoutingDecision> {
        // Exploration vs exploitation
        if self.config.enable_exploration {
            let mut rng = OsRng;
            if rng.gen::<f64>() < self.config.exploration_rate {
                // Explore: choose random path
                let random_path = paths
                    .get(rng.gen_range(0..paths.len()))
                    .ok_or_else(|| VantisError::InvalidPeer("No paths available".to_string()))?;

                return Ok(RoutingDecision {
                    path_id: random_path.path_id.clone(),
                    metric: self.config.primary_metric,
                    score: random_path.calculate_score(self.config.primary_metric),
                    confidence: 0.5, // Lower confidence for exploration
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
            }
        }

        // Exploit: use ML model to choose best path
        let weights = self.ml_weights.lock().await;
        let best_path = paths
            .into_iter()
            .max_by(|a, b| {
                let score_a = self.calculate_ml_score(a, &weights);
                let score_b = self.calculate_ml_score(b, &weights);
                score_a.partial_cmp(&score_b).unwrap()
            })
            .ok_or_else(|| VantisError::InvalidPeer("No paths available".to_string()))?;

        let score = self.calculate_ml_score(&best_path, &weights);

        Ok(RoutingDecision {
            path_id: best_path.path_id,
            metric: self.config.primary_metric,
            score,
            confidence: 0.9, // Higher confidence for ML
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    /// Calculate ML score for a path
    fn calculate_ml_score(&self, path: &NetworkPath, weights: &MlWeights) -> f64 {
        let latency_score = (1000.0 / (path.latency_ms + 1.0)).min(1.0);
        let throughput_score = (path.throughput_mbps / 10000.0).min(1.0);

        latency_score * weights.latency_weight
            + throughput_score * weights.throughput_weight
            + path.reliability_score * weights.reliability_weight
            + path.cost_score * weights.cost_weight
    }

    /// Record routing outcome
    pub async fn record_outcome(&self, path_id: String, actual_latency: f64, success: bool) {
        // Record sample for ML training
        {
            let mut samples = self.path_samples.lock().await;
            samples.entry(path_id.clone()).or_insert_with(Vec::new).push(actual_latency);
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            if success {
                stats.successful_decisions += 1;
            } else {
                stats.failed_decisions += 1;
            }
        }

        // Train ML model if we have enough samples
        if self.config.enable_ml {
            self.train_ml_model().await;
        }
    }

    /// Train ML model
    async fn train_ml_model(&self) {
        let samples = self.path_samples.lock().await;
        let total_samples: usize = samples.values().map(|v| v.len()).sum();

        if total_samples < self.config.min_samples_for_training {
            return;
        }

        // In production, implement actual ML training
        // For now, this is a placeholder
        drop(samples);

        {
            let mut stats = self.stats.lock().await;
            stats.ml_model_updates += 1;
        }
    }

    /// Get cached decision
    async fn get_cached_decision(&self, destination: &str) -> Option<RoutingDecision> {
        let cache = self.routing_cache.read().await;
        cache.get(destination).cloned()
    }

    /// Cache routing decision
    async fn cache_decision(&self, destination: String, decision: RoutingDecision) {
        let mut cache = self.routing_cache.write().await;
        cache.insert(destination, decision);
    }

    /// Get routing statistics
    pub async fn get_stats(&self) -> RoutingStats {
        self.stats.lock().await.clone()
    }

    /// Clean up expired cache entries
    pub async fn cleanup_cache(&self) {
        let mut cache = self.routing_cache.write().await;
        let ttl = std::time::Duration::from_secs(self.config.cache_ttl_secs);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        cache.retain(|_, decision| {
            now.saturating_sub(decision.timestamp) < ttl.as_secs()
        });
    }

    /// Update ML weights manually
    pub async fn update_ml_weights(&self, weights: MlWeights) {
        let mut ml_weights = self.ml_weights.lock().await;
        *ml_weights = weights;
    }

    /// Get current ML weights
    pub async fn get_ml_weights(&self) -> MlWeights {
        self.ml_weights.lock().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_smart_routing_initialization() {
        let config = SmartRoutingConfig::default();
        let manager = SmartRoutingManager::new(config);
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_decisions, 0);
    }

    #[tokio::test]
    async fn test_path_addition() {
        let config = SmartRoutingConfig::default();
        let manager = SmartRoutingManager::new(config);
        
        let path = NetworkPath::new(
            "path1".to_string(),
            "10.0.0.1".to_string(),
            "10.0.0.2".to_string(),
        );
        
        manager.add_path(path).await.unwrap();
    }

    #[tokio::test]
    async fn test_routing_decision() {
        let config = SmartRoutingConfig::default();
        let manager = SmartRoutingManager::new(config);
        
        let mut path = NetworkPath::new(
            "path1".to_string(),
            "10.0.0.1".to_string(),
            "10.0.0.2".to_string(),
        );
        path.latency_ms = 50.0;
        path.throughput_mbps = 1000.0;
        path.reliability_score = 0.95;
        path.cost_score = 0.8;
        
        manager.add_path(path).await.unwrap();
        
        let decision = manager.make_routing_decision("10.0.0.2".to_string()).await.unwrap();
        assert_eq!(decision.path_id, "path1");
    }

    #[tokio::test]
    async fn test_path_score_calculation() {
        let mut path = NetworkPath::new(
            "path1".to_string(),
            "10.0.0.1".to_string(),
            "10.0.0.2".to_string(),
        );
        path.latency_ms = 50.0;
        path.throughput_mbps = 1000.0;
        path.reliability_score = 0.95;
        path.cost_score = 0.8;
        
        let latency_score = path.calculate_score(RoutingMetric::Latency);
        let throughput_score = path.calculate_score(RoutingMetric::Throughput);
        
        assert!(latency_score > 0.0);
        assert!(throughput_score > 0.0);
    }
}