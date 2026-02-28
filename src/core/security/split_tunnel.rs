// Split Tunneling System
// Allows selective routing of traffic through VPN
// Supports application-based, domain-based, and IP-based split tunneling

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use crate::error::{VantisError, Result};

/// Split Tunneling Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitTunnelMode {
    /// Route all traffic through VPN
    AllTraffic,
    /// Route only specified traffic through VPN
    Include,
    /// Route all traffic except specified through VPN
    Exclude,
    /// Smart split tunneling based on application
    Smart,
}

/// Rule Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    /// Application-based rule
    Application,
    /// Domain-based rule
    Domain,
    /// IP-based rule
    Ip,
    /// Port-based rule
    Port,
    /// Protocol-based rule
    Protocol,
}

/// Split Tunneling Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTunnelRule {
    pub rule_id: String,
    pub rule_type: RuleType,
    pub value: String,
    pub enabled: bool,
    pub priority: u32,
    pub description: String,
}

impl SplitTunnelRule {
    pub fn new(rule_id: String, rule_type: RuleType, value: String, priority: u32) -> Self {
        Self {
            rule_id,
            rule_type,
            value,
            enabled: true,
            priority,
            description: String::new(),
        }
    }
}

/// Split Tunneling Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTunnelConfig {
    /// Enable split tunneling
    pub enabled: bool,
    /// Split tunneling mode
    pub mode: SplitTunnelMode,
    /// Default action for unmatched traffic
    pub default_route_vpn: bool,
    /// Enable DNS leak protection
    pub enable_dns_leak_protection: bool,
    /// Enable IPv6 leak protection
    pub enable_ipv6_leak_protection: bool,
    /// Enable logging
    pub enable_logging: bool,
}

impl Default for SplitTunnelConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: SplitTunnelMode::AllTraffic,
            default_route_vpn: true,
            enable_dns_leak_protection: true,
            enable_ipv6_leak_protection: true,
            enable_logging: true,
        }
    }
}

/// Routing Decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTunnelRoutingDecision {
    pub route_through_vpn: bool,
    pub matched_rule: Option<String>,
    pub confidence: f64,
}

/// Split Tunneling Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTunnelStats {
    pub total_rules: usize,
    pub active_rules: usize,
    pub vpn_routed_packets: u64,
    pub direct_routed_packets: u64,
    pub vpn_routed_bytes: u64,
    pub direct_routed_bytes: u64,
    pub dns_queries_routed_vpn: u64,
    pub dns_queries_routed_direct: u64,
}

/// Split Tunneling Manager
pub struct SplitTunnelManager {
    config: SplitTunnelConfig,
    rules: Arc<RwLock<Vec<SplitTunnelRule>>>,
    stats: Arc<Mutex<SplitTunnelStats>>,
    dns_cache: Arc<Mutex<HashMap<String, bool>>>,
}

impl SplitTunnelManager {
    pub fn new(config: SplitTunnelConfig) -> Self {
        let stats = SplitTunnelStats {
            total_rules: 0,
            active_rules: 0,
            vpn_routed_packets: 0,
            direct_routed_packets: 0,
            vpn_routed_bytes: 0,
            direct_routed_bytes: 0,
            dns_queries_routed_vpn: 0,
            dns_queries_routed_direct: 0,
        };

        Self {
            config,
            rules: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(Mutex::new(stats)),
            dns_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a split tunneling rule
    pub async fn add_rule(&self, rule: SplitTunnelRule) -> Result<()> {
        {
            let mut rules = self.rules.write().await;
            rules.push(rule);
            rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        }

        self.update_stats().await;
        Ok(())
    }

    /// Remove a rule
    pub async fn remove_rule(&self, rule_id: &str) -> Result<()> {
        {
            let mut rules = self.rules.write().await;
            rules.retain(|r| r.rule_id != rule_id);
        }

        self.update_stats().await;
        Ok(())
    }

    /// Enable a rule
    pub async fn enable_rule(&self, rule_id: &str) -> Result<()> {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.iter_mut().find(|r| r.rule_id == rule_id) {
            rule.enabled = true;
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Rule not found: {}", rule_id)))
        }
    }

    /// Disable a rule
    pub async fn disable_rule(&self, rule_id: &str) -> Result<()> {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.iter_mut().find(|r| r.rule_id == rule_id) {
            rule.enabled = false;
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Rule not found: {}", rule_id)))
        }
    }

    /// Get all rules
    pub async fn get_rules(&self) -> Vec<SplitTunnelRule> {
        self.rules.read().await.clone()
    }

    /// Make routing decision for traffic
    pub async fn route_traffic(&self, destination: String, port: u16, protocol: String) -> Result<SplitTunnelRoutingDecision> {
        if !self.config.enabled {
            // Split tunneling disabled, route all through VPN
            return Ok(SplitTunnelRoutingDecision {
                route_through_vpn: true,
                matched_rule: None,
                confidence: 1.0,
            });
        }

        let rules = self.rules.read().await;
        let active_rules: Vec<_> = rules.iter().filter(|r| r.enabled).collect();

        for rule in active_rules {
            if self.matches_rule(rule, &destination, port, &protocol) {
                let route_vpn = match self.config.mode {
                    SplitTunnelMode::Include => true,
                    SplitTunnelMode::Exclude => false,
                    _ => self.config.default_route_vpn,
                };

                return Ok(SplitTunnelRoutingDecision {
                    route_through_vpn: route_vpn,
                    matched_rule: Some(rule.rule_id.clone()),
                    confidence: 0.95,
                });
            }
        }

        // No rule matched, use default
        Ok(SplitTunnelRoutingDecision {
            route_through_vpn: self.config.default_route_vpn,
            matched_rule: None,
            confidence: 0.8,
        })
    }

    /// Make routing decision for DNS query
    pub async fn route_dns(&self, domain: String) -> Result<SplitTunnelRoutingDecision> {
        // Check DNS cache first
        {
            let cache = self.dns_cache.lock().await;
            if let Some(&route_vpn) = cache.get(&domain) {
                return Ok(SplitTunnelRoutingDecision {
                    route_through_vpn: route_vpn,
                    matched_rule: None,
                    confidence: 1.0,
                });
            }
        }

        // Make routing decision
        let decision = self.route_traffic(domain.clone(), 53, "UDP".to_string()).await?;

        // Cache the decision
        {
            let mut cache = self.dns_cache.lock().await;
            cache.insert(domain, decision.route_through_vpn);
        }

        Ok(decision)
    }

    /// Check if traffic matches a rule
    fn matches_rule(&self, rule: &SplitTunnelRule, destination: &str, port: u16, protocol: &str) -> bool {
        match rule.rule_type {
            RuleType::Application => {
                // In production, check if destination matches application
                false // Placeholder
            }
            RuleType::Domain => {
                // Check if destination matches domain pattern
                destination.contains(&rule.value) || destination == rule.value
            }
            RuleType::Ip => {
                // Check if destination matches IP/CIDR
                destination == rule.value // Placeholder - should support CIDR
            }
            RuleType::Port => {
                // Check if port matches
                if let Ok(rule_port) = rule.value.parse::<u16>() {
                    port == rule_port
                } else {
                    false
                }
            }
            RuleType::Protocol => {
                // Check if protocol matches
                protocol.to_lowercase() == rule.value.to_lowercase()
            }
        }
    }

    /// Record routed packet
    pub async fn record_packet(&self, route_through_vpn: bool, bytes: u64) {
        let mut stats = self.stats.lock().await;
        if route_through_vpn {
            stats.vpn_routed_packets += 1;
            stats.vpn_routed_bytes += bytes;
        } else {
            stats.direct_routed_packets += 1;
            stats.direct_routed_bytes += bytes;
        }
    }

    /// Record DNS query
    pub async fn record_dns_query(&self, route_through_vpn: bool) {
        let mut stats = self.stats.lock().await;
        if route_through_vpn {
            stats.dns_queries_routed_vpn += 1;
        } else {
            stats.dns_queries_routed_direct += 1;
        }
    }

    /// Get statistics
    pub async fn get_stats(&self) -> SplitTunnelStats {
        self.stats.lock().await.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: SplitTunnelConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }

    /// Clear DNS cache
    pub async fn clear_dns_cache(&self) {
        self.dns_cache.lock().await.clear();
    }

    /// Update statistics
    async fn update_stats(&self) {
        let rules = self.rules.read().await;
        let mut stats = self.stats.lock().await;
        stats.total_rules = rules.len();
        stats.active_rules = rules.iter().filter(|r| r.enabled).count();
    }

    /// Import rules from configuration
    pub async fn import_rules(&self, rules: Vec<SplitTunnelRule>) -> Result<()> {
        {
            let mut current_rules = self.rules.write().await;
            *current_rules = rules;
            current_rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        }

        self.update_stats().await;
        Ok(())
    }

    /// Export rules to configuration
    pub async fn export_rules(&self) -> Vec<SplitTunnelRule> {
        self.rules.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_split_tunnel_initialization() {
        let config = SplitTunnelConfig::default();
        let manager = SplitTunnelManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_rules, 0);
    }

    #[tokio::test]
    async fn test_add_rule() {
        let config = SplitTunnelConfig::default();
        let manager = SplitTunnelManager::new(config);

        let rule = SplitTunnelRule::new(
            "rule1".to_string(),
            RuleType::Domain,
            "example.com".to_string(),
            10,
        );

        manager.add_rule(rule).await.unwrap();
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_rules, 1);
    }

    #[tokio::test]
    async fn test_route_traffic() {
        let mut config = SplitTunnelConfig::default();
        config.enabled = true;
        config.mode = SplitTunnelMode::Include;
        let manager = SplitTunnelManager::new(config);

        let rule = SplitTunnelRule::new(
            "rule1".to_string(),
            RuleType::Domain,
            "example.com".to_string(),
            10,
        );

        manager.add_rule(rule).await.unwrap();

        let decision = manager.route_traffic("example.com".to_string(), 443, "TCP".to_string()).await.unwrap();
        assert!(decision.route_through_vpn);
        assert_eq!(decision.matched_rule, Some("rule1".to_string()));
    }

    #[tokio::test]
    async fn test_route_dns() {
        let mut config = SplitTunnelConfig::default();
        config.enabled = true;
        config.mode = SplitTunnelMode::Include;
        let manager = SplitTunnelManager::new(config);

        let rule = SplitTunnelRule::new(
            "rule1".to_string(),
            RuleType::Domain,
            "example.com".to_string(),
            10,
        );

        manager.add_rule(rule).await.unwrap();

        let decision = manager.route_dns("example.com".to_string()).await.unwrap();
        assert!(decision.route_through_vpn);
    }

    #[tokio::test]
    async fn test_dns_cache() {
        let mut config = SplitTunnelConfig::default();
        config.enabled = true;
        let manager = SplitTunnelManager::new(config);

        // First query
        let decision1 = manager.route_dns("example.com".to_string()).await.unwrap();
        
        // Second query should use cache
        let decision2 = manager.route_dns("example.com".to_string()).await.unwrap();
        
        assert_eq!(decision1.route_through_vpn, decision2.route_through_vpn);
        assert_eq!(decision2.confidence, 1.0); // Cached decision has higher confidence
    }
}