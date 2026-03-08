// Family Shield - DNS Protection for Families
// Phase 6: UX/UI & Additional Features
// Provides family-friendly DNS filtering and protection

use crate::crypto::hash::Hash;
use crate::error::VantisError;
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shield category
///
/// Categories of content that can be filtered by the Family Shield DNS protection system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShieldCategory {
    /// Adult content
    ///
    /// Adult-oriented content and websites.
    Adult,
    /// Gambling
    ///
    /// Online gambling and betting websites.
    Gambling,
    /// Violence
    ///
    /// Violence and gore content.
    Violence,
    /// Drugs
    ///
    /// Drug-related content and websites.
    Drugs,
    /// Social media
    ///
    /// Social media platforms and networks.
    SocialMedia,
    /// Streaming services
    ///
    /// Video streaming and media platforms.
    Streaming,
    /// Gaming
    ///
    /// Online gaming and gambling sites.
    Gaming,
    /// Malware
    ///
    /// Known malware distribution sites.
    Malware,
    /// Phishing
    ///
    /// Phishing and scam websites.
    Phishing,
    /// Custom category
    ///
    /// User-defined custom category.
    Custom,
}

/// Shield action
///
/// Actions that can be taken when a DNS query matches a filtering rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShieldAction {
    /// Block domain
    ///
    /// Block access to the domain entirely.
    Block,
    /// Allow domain
    ///
    /// Allow access to the domain.
    Allow,
    /// Redirect to safe page
    ///
    /// Redirect to a safe landing page.
    Redirect,
    /// Warn user
    ///
    /// Show a warning to the user before proceeding.
    Warn,
}

/// Shield rule
///
/// Represents a DNS filtering rule that defines how to handle specific domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldRule {
    /// Rule ID
    ///
    /// Unique identifier for this rule.
    pub rule_id: String,
    /// Domain pattern
    ///
    /// Pattern to match against domain names (supports wildcards).
    pub domain_pattern: String,
    /// Category
    ///
    /// Category this rule belongs to.
    pub category: ShieldCategory,
    /// Action
    ///
    /// Action to take when this rule matches.
    pub action: ShieldAction,
    /// Priority (higher = more important)
    ///
    /// Priority of this rule. Higher priority rules are evaluated first.
    pub priority: u32,
    /// Enabled
    ///
    /// Whether this rule is currently active.
    pub enabled: bool,
    /// Created at
    ///
    /// When this rule was created.
    pub created_at: DateTime<Utc>,
}

/// DNS query
///
/// Represents a DNS query that has been processed by the Family Shield system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQuery {
    /// Query ID
    ///
    /// Unique identifier for this query.
    pub query_id: String,
    /// Domain name
    ///
    /// Domain name being queried.
    pub domain: String,
    /// Query type (A, AAAA, etc.)
    ///
    /// Type of DNS query (A, AAAA, MX, etc.).
    pub query_type: String,
    /// Source IP
    ///
    /// IP address of the client making the query.
    pub source_ip: IpAddr,
    /// Timestamp
    ///
    /// When this query was received.
    pub timestamp: DateTime<Utc>,
}

/// DNS response
///
/// Represents the response to a DNS query after filtering rules have been applied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResponse {
    /// Query ID
    ///
    /// ID of the query this response is for.
    pub query_id: String,
    /// Domain name
    ///
    /// Domain name that was queried.
    pub domain: String,
    /// Response IP
    ///
    /// IP address to return (if not blocked).
    pub response_ip: Option<IpAddr>,
    /// Action taken
    ///
    /// Action that was taken on this query.
    pub action: ShieldAction,
    /// Rule that matched
    ///
    /// ID of the rule that matched (if any).
    pub matched_rule: Option<String>,
    /// Timestamp
    ///
    /// When this response was generated.
    pub timestamp: DateTime<Utc>,
}

/// Shield configuration
///
/// Configuration settings for the Family Shield DNS protection system.
#[derive(Debug, Clone)]
pub struct ShieldConfig {
    /// Enable family shield
    ///
    /// Whether Family Shield DNS filtering is enabled.
    pub enabled: bool,
    /// Default action for unmatched domains
    ///
    /// Action to take when no rule matches a domain.
    pub default_action: ShieldAction,
    /// Enable logging
    ///
    /// Whether to log DNS queries and responses.
    pub enable_logging: bool,
    /// Enable statistics
    ///
    /// Whether to collect statistics on DNS queries.
    pub enable_stats: bool,
    /// Safe search enabled
    ///
    /// Whether to enforce safe search on search engines.
    pub safe_search_enabled: bool,
    /// Time-based restrictions enabled
    ///
    /// Whether to enforce time-based access restrictions.
    pub time_restrictions_enabled: bool,
    /// Bedtime start hour (0-23)
    ///
    /// Hour when bedtime restrictions start (0-23).
    pub bedtime_start: u8,
    /// Bedtime end hour (0-23)
    ///
    /// Hour when bedtime restrictions end (0-23).
    pub bedtime_end: u8,
}

impl Default for ShieldConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_action: ShieldAction::Allow,
            enable_logging: true,
            enable_stats: true,
            safe_search_enabled: true,
            time_restrictions_enabled: false,
            bedtime_start: 22, // 10 PM
            bedtime_end: 7,    // 7 AM
        }
    }
}

/// Shield statistics
///
/// Statistics about DNS queries processed by the Family Shield system.
#[derive(Debug, Clone)]
pub struct ShieldStats {
    /// Total queries processed
    ///
    /// Total number of DNS queries processed.
    pub total_queries: u64,
    /// Queries blocked
    ///
    /// Number of queries that were blocked.
    pub blocked_queries: u64,
    /// Queries allowed
    ///
    /// Number of queries that were allowed.
    pub allowed_queries: u64,
    /// Queries redirected
    ///
    /// Number of queries that were redirected.
    pub redirected_queries: u64,
    /// Queries warned
    ///
    /// Number of queries that triggered a warning.
    pub warned_queries: u64,
    /// Unique domains accessed
    ///
    /// Number of unique domains accessed.
    pub unique_domains: usize,
    /// Top blocked domains
    ///
    /// List of most frequently blocked domains and their block counts.
    pub top_blocked: Vec<(String, u64)>,
}

/// Family Shield - DNS Protection for Families
///
/// Main structure for Family Shield DNS protection system. Provides family-friendly
/// DNS filtering to protect against inappropriate content, malware, and phishing.
pub struct FamilyShield {
    config: ShieldConfig,
    rules: Arc<Mutex<HashMap<String, ShieldRule>>>,
    query_log: Arc<Mutex<Vec<DnsQuery>>>,
    response_log: Arc<Mutex<Vec<DnsResponse>>>,
    stats: Arc<Mutex<ShieldStats>>,
    domain_cache: Arc<Mutex<HashMap<String, ShieldAction>>>,
    hash: Arc<Mutex<Hash>>,
}

impl FamilyShield {
    /// Create a new Family Shield instance
    pub fn new(config: ShieldConfig) -> Result<Self, VantisError> {
        let hash = Hash::new()?;
        Ok(Self {
            config,
            rules: Arc::new(Mutex::new(HashMap::new())),
            query_log: Arc::new(Mutex::new(Vec::new())),
            response_log: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(ShieldStats {
                total_queries: 0,
                blocked_queries: 0,
                allowed_queries: 0,
                redirected_queries: 0,
                warned_queries: 0,
                unique_domains: 0,
                top_blocked: Vec::new(),
            })),
            domain_cache: Arc::new(Mutex::new(HashMap::new())),
            hash: Arc::new(Mutex::new(hash)),
        })
    }

    /// Add shield rule
    pub async fn add_rule(&self, rule: ShieldRule) -> Result<(), VantisError> {
        let mut rules = self.rules.lock().await;
        rules.insert(rule.rule_id.clone(), rule);
        Ok(())
    }

    /// Remove shield rule
    pub async fn remove_rule(&self, rule_id: &str) -> Result<(), VantisError> {
        let mut rules = self.rules.lock().await;
        rules.remove(rule_id);
        Ok(())
    }

    /// Process DNS query
    pub async fn process_query(&self, query: DnsQuery) -> Result<DnsResponse, VantisError> {
        // Check cache first
        {
            let cache = self.domain_cache.lock().await;
            if let Some(&action) = cache.get(&query.domain) {
                return Ok(DnsResponse {
                    query_id: query.query_id.clone(),
                    domain: query.domain.clone(),
                    response_ip: None,
                    action,
                    matched_rule: None,
                    timestamp: Utc::now(),
                });
            }
        }

        // Check time restrictions
        if self.config.time_restrictions_enabled && self.is_bedtime() {
            return Ok(DnsResponse {
                query_id: query.query_id.clone(),
                domain: query.domain.clone(),
                response_ip: None,
                action: ShieldAction::Block,
                matched_rule: Some("bedtime_restriction".to_string()),
                timestamp: Utc::now(),
            });
        }

        // Check rules
        let rules = self.rules.lock().await;
        let mut matched_rule: Option<&ShieldRule> = None;
        let mut highest_priority = 0u32;

        for rule in rules.values().filter(|r| r.enabled) {
            if self.domain_matches(&query.domain, &rule.domain_pattern)
                && rule.priority > highest_priority
            {
                matched_rule = Some(rule);
                highest_priority = rule.priority;
            }
        }

        let action = matched_rule
            .map(|r| r.action)
            .unwrap_or(self.config.default_action);

        // Cache the result
        {
            let mut cache = self.domain_cache.lock().await;
            cache.insert(query.domain.clone(), action);
        }

        // Update stats
        let mut stats = self.stats.lock().await;
        stats.total_queries += 1;
        match action {
            ShieldAction::Block => stats.blocked_queries += 1,
            ShieldAction::Allow => stats.allowed_queries += 1,
            ShieldAction::Redirect => stats.redirected_queries += 1,
            ShieldAction::Warn => stats.warned_queries += 1,
        }

        Ok(DnsResponse {
            query_id: query.query_id.clone(),
            domain: query.domain.clone(),
            response_ip: None,
            action,
            matched_rule: matched_rule.map(|r| r.rule_id.clone()),
            timestamp: Utc::now(),
        })
    }

    /// Check if domain matches pattern
    fn domain_matches(&self, domain: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with("*.") {
            let suffix = &pattern[2..];
            return domain.ends_with(suffix);
        }

        domain == pattern
    }

    /// Check if current time is bedtime
    fn is_bedtime(&self) -> bool {
        let now = Utc::now();
        let hour = now.hour() as u8;

        if self.config.bedtime_start < self.config.bedtime_end {
            // Same day (e.g., 22:00 - 07:00)
            hour >= self.config.bedtime_start || hour < self.config.bedtime_end
        } else {
            // Crosses midnight (e.g., 22:00 - 07:00)
            hour >= self.config.bedtime_start || hour < self.config.bedtime_end
        }
    }

    /// Get shield statistics
    pub async fn get_stats(&self) -> ShieldStats {
        let stats = self.stats.lock().await;
        stats.clone()
    }

    /// Get all rules
    pub async fn get_rules(&self) -> Vec<ShieldRule> {
        let rules = self.rules.lock().await;
        rules.values().cloned().collect()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: ShieldConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &ShieldConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_family_shield_creation() {
        let config = ShieldConfig::default();
        let shield = FamilyShield::new(config);
        assert!(shield.is_ok());
    }

    #[test]
    fn test_domain_matching() {
        let config = ShieldConfig::default();
        let shield = FamilyShield::new(config).unwrap();

        assert!(shield.domain_matches("example.com", "example.com"));
        assert!(shield.domain_matches("sub.example.com", "*.example.com"));
        assert!(shield.domain_matches("anything.com", "*"));
    }
}
