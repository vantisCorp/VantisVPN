// NetShield AI - On-Device DNS Blocker
// AI-powered DNS filtering and blocking system
// Blocks malicious domains, trackers, ads, and phishing sites

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use crate::error::Result;

/// Blocklist Category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlocklistCategory {
    /// Malware domains
    Malware,
    /// Phishing domains
    Phishing,
    /// Tracker domains
    Tracker,
    /// Ad domains
    Ad,
    /// Adult content
    Adult,
    /// Gambling
    Gambling,
    /// Social media
    SocialMedia,
    /// Custom blocklist
    Custom,
}

/// DNS Query Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsQueryType {
    /// A record
    A,
    /// AAAA record
    AAAA,
    /// CNAME record
    CNAME,
    /// MX record
    MX,
    /// TXT record
    TXT,
    /// NS record
    NS,
}

/// DNS Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQuery {
    pub query_id: u64,
    pub domain: String,
    pub query_type: DnsQueryType,
    pub client_ip: String,
    pub timestamp: u64,
}

/// DNS Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResponse {
    pub query_id: u64,
    pub blocked: bool,
    pub block_reason: Option<String>,
    pub block_category: Option<BlocklistCategory>,
    pub response_ip: Option<String>,
    pub ttl: u32,
}

/// Blocklist Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocklistEntry {
    pub domain: String,
    pub category: BlocklistCategory,
    pub added_at: u64,
    pub source: String,
}

/// NetShield Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetShieldConfig {
    /// Enable NetShield
    pub enabled: bool,
    /// Enable AI-based detection
    pub enable_ai_detection: bool,
    /// Enable blocklist filtering
    pub enable_blocklist_filtering: bool,
    /// Enable safe search
    pub enable_safe_search: bool,
    /// Enable family mode
    pub enable_family_mode: bool,
    /// Blocked categories
    pub blocked_categories: HashSet<BlocklistCategory>,
    /// Enable logging
    pub enable_logging: bool,
    /// Log file path
    pub log_path: String,
    /// Enable statistics
    pub enable_stats: bool,
}

impl Default for NetShieldConfig {
    fn default() -> Self {
        let mut blocked_categories = HashSet::new();
        blocked_categories.insert(BlocklistCategory::Malware);
        blocked_categories.insert(BlocklistCategory::Phishing);
        blocked_categories.insert(BlocklistCategory::Tracker);
        blocked_categories.insert(BlocklistCategory::Ad);

        Self {
            enabled: true,
            enable_ai_detection: true,
            enable_blocklist_filtering: true,
            enable_safe_search: false,
            enable_family_mode: false,
            blocked_categories,
            enable_logging: true,
            log_path: "/var/log/vantisvpn/netshield.log".to_string(),
            enable_stats: true,
        }
    }
}

/// NetShield Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetShieldStats {
    pub total_queries: u64,
    pub blocked_queries: u64,
    pub allowed_queries: u64,
    pub queries_by_category: HashMap<BlocklistCategory, u64>,
    pub top_blocked_domains: Vec<(String, u64)>,
    pub average_response_time_ms: f64,
}

/// NetShield AI Manager
pub struct NetShieldManager {
    config: NetShieldConfig,
    blocklist: Arc<RwLock<HashMap<String, BlocklistEntry>>>,
    stats: Arc<Mutex<NetShieldStats>>,
    domain_cache: Arc<Mutex<HashMap<String, bool>>>,
    query_counter: Arc<Mutex<u64>>,
}

impl NetShieldManager {
    pub fn new(config: NetShieldConfig) -> Self {
        let stats = NetShieldStats {
            total_queries: 0,
            blocked_queries: 0,
            allowed_queries: 0,
            queries_by_category: HashMap::new(),
            top_blocked_domains: Vec::new(),
            average_response_time_ms: 0.0,
        };

        Self {
            config,
            blocklist: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
            domain_cache: Arc::new(Mutex::new(HashMap::new())),
            query_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Process DNS query
    pub async fn process_query(&self, query: DnsQuery) -> Result<DnsResponse> {
        if !self.config.enabled {
            // NetShield disabled, allow all queries
            return Ok(DnsResponse {
                query_id: query.query_id,
                blocked: false,
                block_reason: None,
                block_category: None,
                response_ip: Some("0.0.0.0".to_string()), // Placeholder
                ttl: 300,
            });
        }

        let start_time = std::time::Instant::now();

        // Check cache first
        {
            let cache = self.domain_cache.lock().await;
            if let Some(&blocked) = cache.get(&query.domain) {
                let response = if blocked {
                    self.create_blocked_response(&query, "Cached block".to_string(), None)
                } else {
                    self.create_allowed_response(&query)
                };

                self.update_stats(&query, &response, start_time.elapsed()).await;
                return Ok(response);
            }
        }

        // Check blocklist
        let blocklist_result = self.check_blocklist(&query.domain).await;

        // Check AI detection if enabled
        let ai_result = if self.config.enable_ai_detection {
            self.ai_detection(&query).await
        } else {
            false
        };

        // Determine if blocked
        let blocked = blocklist_result || ai_result;

        // Cache the result
        {
            let mut cache = self.domain_cache.lock().await;
            cache.insert(query.domain.clone(), blocked);
        }

        // Create response
        let response = if blocked {
            let category = self.get_domain_category(&query.domain).await;
            self.create_blocked_response(&query, "Domain blocked".to_string(), category)
        } else {
            self.create_allowed_response(&query)
        };

        self.update_stats(&query, &response, start_time.elapsed()).await;
        Ok(response)
    }

    /// Check if domain is in blocklist
    async fn check_blocklist(&self, domain: &str) -> bool {
        if !self.config.enable_blocklist_filtering {
            return false;
        }

        let blocklist = self.blocklist.read().await;
        
        // Check exact match
        if let Some(entry) = blocklist.get(domain) {
            return self.config.blocked_categories.contains(&entry.category);
        }

        // Check subdomain match
        for (blocklisted_domain, entry) in blocklist.iter() {
            if domain.ends_with(blocklisted_domain) && self.config.blocked_categories.contains(&entry.category) {
                return true;
            }
        }

        false
    }

    /// AI-based domain detection
    async fn ai_detection(&self, query: &DnsQuery) -> bool {
        // In production, this would use ML model to detect malicious domains
        // Features to consider:
        // - Domain age
        // - Domain length
        // - Character patterns
        // - TLD reputation
        // - DNS query patterns
        // - Historical behavior

        // Placeholder: simple heuristic
        let domain = &query.domain;
        
        // Check for suspicious patterns
        let suspicious_patterns = vec![
            "login",
            "verify",
            "account",
            "secure",
            "update",
            "bank",
            "paypal",
            "apple",
            "microsoft",
            "google",
        ];

        for pattern in suspicious_patterns {
            if domain.contains(pattern) && domain.len() > 30 {
                return true; // Suspicious: long domain with brand name
            }
        }

        false
    }

    /// Get domain category
    async fn get_domain_category(&self, domain: &str) -> Option<BlocklistCategory> {
        let blocklist = self.blocklist.read().await;
        
        if let Some(entry) = blocklist.get(domain) {
            return Some(entry.category);
        }

        // Check subdomain match
        for (blocklisted_domain, entry) in blocklist.iter() {
            if domain.ends_with(blocklisted_domain) {
                return Some(entry.category);
            }
        }

        None
    }

    /// Create blocked response
    fn create_blocked_response(&self, query: &DnsQuery, reason: String, category: Option<BlocklistCategory>) -> DnsResponse {
        DnsResponse {
            query_id: query.query_id,
            blocked: true,
            block_reason: Some(reason),
            block_category: category,
            response_ip: Some("0.0.0.0".to_string()), // Block by returning 0.0.0.0
            ttl: 300,
        }
    }

    /// Create allowed response
    fn create_allowed_response(&self, query: &DnsQuery) -> DnsResponse {
        DnsResponse {
            query_id: query.query_id,
            blocked: false,
            block_reason: None,
            block_category: None,
            response_ip: None, // Let DNS resolver handle
            ttl: 300,
        }
    }

    /// Add domain to blocklist
    pub async fn add_to_blocklist(&self, entry: BlocklistEntry) -> Result<()> {
        {
            let mut blocklist = self.blocklist.write().await;
            blocklist.insert(entry.domain.clone(), entry);
        }

        // Clear cache to force re-evaluation
        self.domain_cache.lock().await.clear();

        Ok(())
    }

    /// Remove domain from blocklist
    pub async fn remove_from_blocklist(&self, domain: &str) -> Result<()> {
        {
            let mut blocklist = self.blocklist.write().await;
            blocklist.remove(domain);
        }

        // Clear cache
        self.domain_cache.lock().await.clear();

        Ok(())
    }

    /// Get blocklist
    pub async fn get_blocklist(&self) -> Vec<BlocklistEntry> {
        self.blocklist.read().await.values().cloned().collect()
    }

    /// Get statistics
    pub async fn get_stats(&self) -> NetShieldStats {
        self.stats.lock().await.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: NetShieldConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        self.domain_cache.lock().await.clear();
    }

    /// Update statistics
    async fn update_stats(&self, query: &DnsQuery, response: &DnsResponse, duration: std::time::Duration) {
        let mut stats = self.stats.lock().await;
        stats.total_queries += 1;

        if response.blocked {
            stats.blocked_queries += 1;

            // Update category stats
            if let Some(category) = response.block_category {
                *stats.queries_by_category.entry(category).or_insert(0) += 1;
            }

            // Update top blocked domains
            let domain = query.domain.clone();
            let entry = stats.top_blocked_domains.iter_mut().find(|(d, _)| d == &domain);
            if let Some((_, count)) = entry {
                *count += 1;
            } else {
                stats.top_blocked_domains.push((domain, 1));
                stats.top_blocked_domains.sort_by(|a, b| b.1.cmp(&a.1));
                stats.top_blocked_domains.truncate(10);
            }
        } else {
            stats.allowed_queries += 1;
        }

        // Update average response time
        let response_time_ms = duration.as_millis() as f64;
        stats.average_response_time_ms = 
            (stats.average_response_time_ms * (stats.total_queries - 1) as f64 + response_time_ms) / stats.total_queries as f64;
    }

    /// Import blocklist
    pub async fn import_blocklist(&self, entries: Vec<BlocklistEntry>) -> Result<()> {
        {
            let mut blocklist = self.blocklist.write().await;
            for entry in entries {
                blocklist.insert(entry.domain.clone(), entry);
            }
        }

        self.domain_cache.lock().await.clear();
        Ok(())
    }

    /// Export blocklist
    pub async fn export_blocklist(&self) -> Vec<BlocklistEntry> {
        self.blocklist.read().await.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_netshield_initialization() {
        let config = NetShieldConfig::default();
        let manager = NetShieldManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_queries, 0);
    }

    #[tokio::test]
    async fn test_process_query_allowed() {
        let config = NetShieldConfig::default();
        let manager = NetShieldManager::new(config);

        let query = DnsQuery {
            query_id: 1,
            domain: "example.com".to_string(),
            query_type: DnsQueryType::A,
            client_ip: "192.168.1.1".to_string(),
            timestamp: 0,
        };

        let response = manager.process_query(query).await.unwrap();
        assert!(!response.blocked);
    }

    #[tokio::test]
    async fn test_add_to_blocklist() {
        let config = NetShieldConfig::default();
        let manager = NetShieldManager::new(config);

        let entry = BlocklistEntry {
            domain: "malicious.com".to_string(),
            category: BlocklistCategory::Malware,
            added_at: 0,
            source: "manual".to_string(),
        };

        manager.add_to_blocklist(entry).await.unwrap();

        let query = DnsQuery {
            query_id: 1,
            domain: "malicious.com".to_string(),
            query_type: DnsQueryType::A,
            client_ip: "192.168.1.1".to_string(),
            timestamp: 0,
        };

        let response = manager.process_query(query).await.unwrap();
        assert!(response.blocked);
    }

    #[tokio::test]
    async fn test_dns_cache() {
        let config = NetShieldConfig::default();
        let manager = NetShieldManager::new(config);

        let query = DnsQuery {
            query_id: 1,
            domain: "example.com".to_string(),
            query_type: DnsQueryType::A,
            client_ip: "192.168.1.1".to_string(),
            timestamp: 0,
        };

        // First query
        let response1 = manager.process_query(query.clone()).await.unwrap();
        
        // Second query should use cache
        let response2 = manager.process_query(query).await.unwrap();
        
        assert_eq!(response1.blocked, response2.blocked);
    }

    #[tokio::test]
    async fn test_statistics() {
        let config = NetShieldConfig::default();
        let manager = NetShieldManager::new(config);

        let entry = BlocklistEntry {
            domain: "blocked.com".to_string(),
            category: BlocklistCategory::Malware,
            added_at: 0,
            source: "manual".to_string(),
        };

        manager.add_to_blocklist(entry).await.unwrap();

        let query = DnsQuery {
            query_id: 1,
            domain: "blocked.com".to_string(),
            query_type: DnsQueryType::A,
            client_ip: "192.168.1.1".to_string(),
            timestamp: 0,
        };

        manager.process_query(query).await.unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_queries, 1);
        assert_eq!(stats.blocked_queries, 1);
    }
}