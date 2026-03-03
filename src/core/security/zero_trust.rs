// Zero Trust - Micro-segmentation and Access Control
// Phase 4: User Security & Protection
// Implements Zero Trust security model with micro-segmentation

use crate::error::VantisError;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};

/// Zero Trust policy action
/// 
/// Defines the action to take when a Zero Trust policy matches an access request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyAction {
    /// Allow the access request
    /// 
    /// Grants access to the requested resource without additional requirements.
    Allow,
    /// Deny the access request
    /// 
    /// Blocks access to the requested resource.
    Deny,
    /// Require authentication
    /// 
    /// Requires the user to authenticate before granting access.
    RequireAuth,
    /// Require multi-factor authentication
    /// 
    /// Requires the user to provide additional authentication factors.
    RequireMfa,
    /// Log only
    /// 
    /// Logs the access request without making an access decision,
    /// allowing subsequent policies to be evaluated.
    LogOnly,
}

/// Zero Trust policy rule
/// 
/// Defines a Zero Trust access control policy with matching criteria
/// and actions to take when the policy matches an access request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroTrustPolicy {
    /// Unique policy ID
    /// 
    /// Unique identifier for this policy.
    pub id: String,
    /// Policy name
    /// 
    /// Human-readable name for this policy.
    pub name: String,
    /// Description
    /// 
    /// Detailed description of what this policy does.
    pub description: String,
    /// Source IP/CIDR (empty = any)
    /// 
    /// Source IP address or CIDR range to match. Empty means any source.
    pub source: Option<String>,
    /// Destination IP/CIDR (empty = any)
    /// 
    /// Destination IP address or CIDR range to match. Empty means any destination.
    pub destination: Option<String>,
    /// Destination port (0 = any)
    /// 
    /// Destination port number to match. 0 means any port.
    pub port: u16,
    /// Protocol (tcp/udp/any)
    /// 
    /// Network protocol to match (tcp, udp, or any).
    pub protocol: String,
    /// Action to take
    /// 
    /// The action to take when this policy matches an access request.
    pub action: PolicyAction,
    /// Priority (higher = more important)
    /// 
    /// Policy priority. Higher values are evaluated first.
    pub priority: u32,
    /// Enabled status
    /// 
    /// Whether this policy is currently active.
    pub enabled: bool,
    /// Tags for organization
    /// 
    /// Tags for organizing and filtering policies.
    pub tags: Vec<String>,
    /// Creation timestamp
    /// 
    /// When this policy was created.
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    /// 
    /// When this policy was last modified.
    pub modified_at: DateTime<Utc>,
}

/// Access request
/// 
/// Represents an access request to be evaluated against Zero Trust policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRequest {
    /// Source IP address
    /// 
    /// IP address of the source making the access request.
    pub source: IpAddr,
    /// Destination IP address
    /// 
    /// IP address of the destination being accessed.
    pub destination: IpAddr,
    /// Destination port
    /// 
    /// Port number of the destination service.
    pub port: u16,
    /// Protocol (tcp/udp)
    /// 
    /// Network protocol being used (tcp or udp).
    pub protocol: String,
    /// User ID (if authenticated)
    /// 
    /// User ID if the request is authenticated, None otherwise.
    pub user_id: Option<String>,
    /// Device ID
    /// 
    /// Unique identifier of the device making the request.
    pub device_id: String,
    /// Request timestamp
    /// 
    /// When the access request was made.
    pub timestamp: DateTime<Utc>,
}

/// Access decision
/// 
/// Represents the result of evaluating an access request against
/// Zero Trust policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessDecision {
    /// Whether access is granted
    /// 
    /// True if access is allowed, false if denied.
    pub allowed: bool,
    /// Policy that made the decision
    /// 
    /// ID of the policy that made this decision, if any.
    pub policy_id: Option<String>,
    /// Reason for decision
    /// 
    /// Human-readable explanation of why this decision was made.
    pub reason: String,
    /// Additional requirements (e.g., MFA)
    /// 
    /// List of additional requirements that must be met for access.
    pub requirements: Vec<String>,
    /// Decision timestamp
    /// 
    /// When this access decision was made.
    pub timestamp: DateTime<Utc>,
}

/// Access log entry
/// 
/// Represents a logged access request and its corresponding decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLog {
    /// Unique log ID
    /// 
    /// Unique identifier for this log entry.
    pub id: String,
    /// Access request
    /// 
    /// The access request that was logged.
    pub request: AccessRequest,
    /// Access decision
    /// 
    /// The decision made for this access request.
    pub decision: AccessDecision,
    /// Log timestamp
    /// 
    /// When this log entry was created.
    pub timestamp: DateTime<Utc>,
}

/// Zero Trust configuration
/// 
/// Configuration settings for the Zero Trust access control system.
#[derive(Debug, Clone)]
pub struct ZeroTrustConfig {
    /// Default action when no policy matches
    /// 
    /// The action to take when no matching policy is found.
    pub default_action: PolicyAction,
    /// Enable logging for all requests
    /// 
    /// Whether to log all access requests regardless of policy matches.
    pub log_all_requests: bool,
    /// Enable anomaly detection
    /// 
    /// Whether to enable anomaly detection for suspicious access patterns.
    pub enable_anomaly_detection: bool,
    /// Anomaly threshold (0-1)
    /// 
    /// Threshold for anomaly detection (0.0 to 1.0).
    pub anomaly_threshold: f64,
    /// Session timeout in seconds
    /// 
    /// Maximum duration of an active session before re-authentication is required.
    pub session_timeout: u64,
    /// Maximum failed attempts before lockout
    /// 
    /// Maximum number of failed access attempts before account lockout.
    pub max_failed_attempts: u32,
    /// Lockout duration in seconds
    /// 
    /// Duration of account lockout after too many failed attempts.
    pub lockout_duration: u64,
}

impl Default for ZeroTrustConfig {
    fn default() -> Self {
        Self {
            default_action: PolicyAction::Deny,
            log_all_requests: true,
            enable_anomaly_detection: true,
            anomaly_threshold: 0.7,
            session_timeout: 3600, // 1 hour
            max_failed_attempts: 5,
            lockout_duration: 900, // 15 minutes
        }
    }
}

/// Device trust score
/// 
/// Represents the trust score and assessment factors for a device.
#[derive(Debug, Clone)]
pub struct DeviceTrust {
    /// Device ID
    /// 
    /// Unique identifier for this device.
    pub device_id: String,
    /// Trust score (0-100)
    /// 
    /// Trust score for this device (0 = untrusted, 100 = fully trusted).
    pub score: u8,
    /// Last assessment timestamp
    /// 
    /// When the device trust was last assessed.
    pub last_assessed: DateTime<Utc>,
    /// Factors affecting score
    /// 
    /// List of factors that influenced the trust score.
    pub factors: Vec<String>,
}

/// Zero Trust Manager
/// 
/// Implements micro-segmentation and access control based on the Zero Trust
/// security model, requiring continuous verification for all access requests
/// and implementing least privilege access principles.
pub struct ZeroTrust {
    config: ZeroTrustConfig,
    policies: Arc<Mutex<HashMap<String, ZeroTrustPolicy>>>,
    access_logs: Arc<Mutex<Vec<AccessLog>>>,
    device_trust: Arc<Mutex<HashMap<String, DeviceTrust>>>,
    failed_attempts: Arc<Mutex<HashMap<String, u32>>>,
    lockout_until: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
    active_sessions: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
}

impl ZeroTrust {
    /// Create a new Zero Trust instance
    pub fn new(config: ZeroTrustConfig) -> Self {
        Self {
            config,
            policies: Arc::new(Mutex::new(HashMap::new())),
            access_logs: Arc::new(Mutex::new(Vec::new())),
            device_trust: Arc::new(Mutex::new(HashMap::new())),
            failed_attempts: Arc::new(Mutex::new(HashMap::new())),
            lockout_until: Arc::new(Mutex::new(HashMap::new())),
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a policy
    pub async fn add_policy(&mut self, policy: ZeroTrustPolicy) -> Result<(), VantisError> {
        let mut policies = self.policies.lock().await;
        policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Remove a policy
    pub async fn remove_policy(&self, policy_id: &str) -> Result<(), VantisError> {
        let mut policies = self.policies.lock().await;
        policies.remove(policy_id)
            .ok_or_else(|| VantisError::NotFound(format!("Policy not found: {}", policy_id)))?;
        Ok(())
    }

    /// Update a policy
    pub async fn update_policy(&mut self, policy: ZeroTrustPolicy) -> Result<(), VantisError> {
        let mut policies = self.policies.lock().await;
        if !policies.contains_key(&policy.id) {
            return Err(VantisError::NotFound(format!("Policy not found: {}", policy.id)));
        }
        policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Get a policy by ID
    pub async fn get_policy(&self, policy_id: &str) -> Result<ZeroTrustPolicy, VantisError> {
        let policies = self.policies.lock().await;
        policies.get(policy_id)
            .cloned()
            .ok_or_else(|| VantisError::NotFound(format!("Policy not found: {}", policy_id)))
    }

    /// List all policies
    pub async fn list_policies(&self) -> Vec<ZeroTrustPolicy> {
        let policies = self.policies.lock().await;
        policies.values().cloned().collect()
    }

    /// Evaluate an access request
    pub async fn evaluate_access(&self, request: AccessRequest) -> Result<AccessDecision, VantisError> {
        // Check lockout status
        if let Some(user_id) = &request.user_id {
            let lockout = self.lockout_until.lock().await;
            if let Some(until) = lockout.get(user_id) {
                if Utc::now() < *until {
                    return Ok(AccessDecision {
                        allowed: false,
                        policy_id: None,
                        reason: "Account is locked out due to too many failed attempts".to_string(),
                        requirements: vec![],
                        timestamp: Utc::now(),
                    });
                }
            }
            drop(lockout);
        }

        // Get matching policies
        let policies = self.policies.lock().await;
        let mut matching_policies: Vec<ZeroTrustPolicy> = policies.values()
            .filter(|p| p.enabled && self.policy_matches(p, &request))
            .cloned()
            .collect();
        drop(policies);

        // Sort by priority (higher first)
        matching_policies.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Evaluate policies
        for policy in matching_policies {
            match policy.action {
                PolicyAction::Allow => {
                    self.log_access(&request, &policy, true, "Allowed by policy").await;
                    return Ok(AccessDecision {
                        allowed: true,
                        policy_id: Some(policy.id.clone()),
                        reason: "Allowed by policy".to_string(),
                        requirements: vec![],
                        timestamp: Utc::now(),
                    });
                }
                PolicyAction::Deny => {
                    self.record_failed_attempt(&request).await;
                    self.log_access(&request, &policy, false, "Denied by policy").await;
                    return Ok(AccessDecision {
                        allowed: false,
                        policy_id: Some(policy.id.clone()),
                        reason: "Denied by policy".to_string(),
                        requirements: vec![],
                        timestamp: Utc::now(),
                    });
                }
                PolicyAction::RequireAuth => {
                    if request.user_id.is_some() {
                        self.log_access(&request, &policy, true, "Authenticated access allowed").await;
                        return Ok(AccessDecision {
                            allowed: true,
                            policy_id: Some(policy.id.clone()),
                            reason: "Authenticated access allowed".to_string(),
                            requirements: vec![],
                            timestamp: Utc::now(),
                        });
                    } else {
                        self.log_access(&request, &policy, false, "Authentication required").await;
                        return Ok(AccessDecision {
                            allowed: false,
                            policy_id: Some(policy.id.clone()),
                            reason: "Authentication required".to_string(),
                            requirements: vec!["authentication".to_string()],
                            timestamp: Utc::now(),
                        });
                    }
                }
                PolicyAction::RequireMfa => {
                    self.log_access(&request, &policy, false, "MFA required").await;
                    return Ok(AccessDecision {
                        allowed: false,
                        policy_id: Some(policy.id.clone()),
                        reason: "MFA required".to_string(),
                        requirements: vec!["multi-factor authentication".to_string()],
                        timestamp: Utc::now(),
                    });
                }
                PolicyAction::LogOnly => {
                    self.log_access(&request, &policy, true, "Logged only").await;
                    continue;
                }
            }
        }

        // No matching policy, use default action
        match self.config.default_action {
            PolicyAction::Allow => {
                self.log_access(&request, &ZeroTrustPolicy::default(), true, "Default allow").await;
                Ok(AccessDecision {
                    allowed: true,
                    policy_id: None,
                    reason: "Default allow".to_string(),
                    requirements: vec![],
                    timestamp: Utc::now(),
                })
            }
            _ => {
                self.record_failed_attempt(&request).await;
                self.log_access(&request, &ZeroTrustPolicy::default(), false, "Default deny").await;
                Ok(AccessDecision {
                    allowed: false,
                    policy_id: None,
                    reason: "Default deny".to_string(),
                    requirements: vec![],
                    timestamp: Utc::now(),
                })
            }
        }
    }

    /// Check if a policy matches a request
    fn policy_matches(&self, policy: &ZeroTrustPolicy, request: &AccessRequest) -> bool {
        // Check source
        if let Some(source) = &policy.source {
            if !self.matches_cidr(&request.source.to_string(), source) {
                return false;
            }
        }

        // Check destination
        if let Some(destination) = &policy.destination {
            if !self.matches_cidr(&request.destination.to_string(), destination) {
                return false;
            }
        }

        // Check port
        if policy.port != 0 && request.port != policy.port {
            return false;
        }

        // Check protocol
        if policy.protocol != "any" && policy.protocol.to_lowercase() != request.protocol.to_lowercase() {
            return false;
        }

        true
    }

    /// Check if IP matches CIDR
    fn matches_cidr(&self, ip: &str, cidr: &str) -> bool {
        // Simplified CIDR matching (use ipnetwork crate in production)
        if cidr == "any" || cidr == "0.0.0.0/0" || cidr == "::/0" {
            return true;
        }
        
        if !cidr.contains('/') {
            return ip == cidr;
        }

        // Basic CIDR parsing
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return false;
        }

        let network = parts[0];
        let prefix_len: u32 = parts[1].parse().unwrap_or(0);

        // Simplified check - in production use proper CIDR library
        if prefix_len == 0 {
            return true;
        }

        ip.starts_with(network)
    }

    /// Record failed access attempt
    async fn record_failed_attempt(&self, request: &AccessRequest) {
        if let Some(user_id) = &request.user_id {
            let mut attempts = self.failed_attempts.lock().await;
            let count = attempts.entry(user_id.clone()).or_insert(0);
            *count += 1;

            if *count >= self.config.max_failed_attempts {
                let mut lockout = self.lockout_until.lock().await;
                lockout.insert(user_id.clone(), Utc::now() + Duration::seconds(self.config.lockout_duration as i64));
            }
        }
    }

    /// Log access request
    async fn log_access(&self, request: &AccessRequest, policy: &ZeroTrustPolicy, allowed: bool, reason: &str) {
        if !self.config.log_all_requests {
            return;
        }

        let log = AccessLog {
            id: self.generate_log_id(),
            request: request.clone(),
            decision: AccessDecision {
                allowed,
                policy_id: Some(policy.id.clone()),
                reason: reason.to_string(),
                requirements: vec![],
                timestamp: Utc::now(),
            },
            timestamp: Utc::now(),
        };

        let mut logs = self.access_logs.lock().await;
        logs.push(log);

        // Keep only last 10000 logs
        let len = logs.len();
        if len > 10000 {
            logs.drain(0..len - 10000);
        }
    }

    /// Get access logs
    pub async fn get_access_logs(&self, limit: usize) -> Vec<AccessLog> {
        let logs = self.access_logs.lock().await;
        let start = if logs.len() > limit { logs.len() - limit } else { 0 };
        logs[start..].to_vec()
    }

    /// Update device trust score
    pub async fn update_device_trust(&self, device_id: String, score: u8, factors: Vec<String>) {
        let trust = DeviceTrust {
            device_id: device_id.clone(),
            score,
            last_assessed: Utc::now(),
            factors,
        };

        let mut device_trust = self.device_trust.lock().await;
        device_trust.insert(device_id, trust);
    }

    /// Get device trust score
    pub async fn get_device_trust(&self, device_id: &str) -> Option<DeviceTrust> {
        let device_trust = self.device_trust.lock().await;
        device_trust.get(device_id).cloned()
    }

    /// Create active session
    pub async fn create_session(&self, user_id: String) {
        let mut sessions = self.active_sessions.lock().await;
        sessions.insert(user_id, Utc::now());
    }

    /// Validate session
    pub async fn validate_session(&self, user_id: &str) -> bool {
        let sessions = self.active_sessions.lock().await;
        if let Some(created) = sessions.get(user_id) {
            let elapsed = Utc::now().signed_duration_since(*created).num_seconds();
            return elapsed < self.config.session_timeout as i64;
        }
        false
    }

    /// Revoke session
    pub async fn revoke_session(&self, user_id: &str) {
        let mut sessions = self.active_sessions.lock().await;
        sessions.remove(user_id);
    }

    /// Generate log ID
    fn generate_log_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("log_{}", timestamp)
    }
}

impl Default for ZeroTrustPolicy {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default Policy".to_string(),
            description: "Default policy".to_string(),
            source: None,
            destination: None,
            port: 0,
            protocol: "any".to_string(),
            action: PolicyAction::Deny,
            priority: 0,
            enabled: true,
            tags: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[tokio::test]
    async fn test_zero_trust_creation() {
        let config = ZeroTrustConfig::default();
        let zt = ZeroTrust::new(config);
        assert_eq!(zt.list_policies().await.len(), 0);
    }

    #[tokio::test]
    async fn test_add_policy() {
        let config = ZeroTrustConfig::default();
        let mut zt = ZeroTrust::new(config);
        
        let policy = ZeroTrustPolicy {
            id: "policy1".to_string(),
            name: "Test Policy".to_string(),
            description: "Test".to_string(),
            source: Some("192.168.1.0/24".to_string()),
            destination: None,
            port: 0,
            protocol: "any".to_string(),
            action: PolicyAction::Allow,
            priority: 100,
            enabled: true,
            tags: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        zt.add_policy(policy).await.unwrap();
        assert_eq!(zt.list_policies().await.len(), 1);
    }

    #[tokio::test]
    async fn test_evaluate_access_allow() {
        let config = ZeroTrustConfig::default();
        let mut zt = ZeroTrust::new(config);
        
        let policy = ZeroTrustPolicy {
            id: "policy1".to_string(),
            name: "Allow Policy".to_string(),
            description: "Allow all".to_string(),
            source: None,
            destination: None,
            port: 0,
            protocol: "any".to_string(),
            action: PolicyAction::Allow,
            priority: 100,
            enabled: true,
            tags: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        zt.add_policy(policy).await.unwrap();

        let request = AccessRequest {
            source: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            destination: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            port: 443,
            protocol: "tcp".to_string(),
            user_id: Some("user1".to_string()),
            device_id: "device1".to_string(),
            timestamp: Utc::now(),
        };

        let decision = zt.evaluate_access(request).await.unwrap();
        assert!(decision.allowed);
    }

    #[tokio::test]
    async fn test_evaluate_access_deny() {
        let config = ZeroTrustConfig::default();
        let mut zt = ZeroTrust::new(config);
        
        let policy = ZeroTrustPolicy {
            id: "policy1".to_string(),
            name: "Deny Policy".to_string(),
            description: "Deny all".to_string(),
            source: None,
            destination: None,
            port: 0,
            protocol: "any".to_string(),
            action: PolicyAction::Deny,
            priority: 100,
            enabled: true,
            tags: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        zt.add_policy(policy).await.unwrap();

        let request = AccessRequest {
            source: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            destination: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            port: 443,
            protocol: "tcp".to_string(),
            user_id: Some("user1".to_string()),
            device_id: "device1".to_string(),
            timestamp: Utc::now(),
        };

        let decision = zt.evaluate_access(request).await.unwrap();
        assert!(!decision.allowed);
    }

    #[tokio::test]
    async fn test_device_trust() {
        let config = ZeroTrustConfig::default();
        let zt = ZeroTrust::new(config);

        zt.update_device_trust("device1".to_string(), 85, vec!["updated".to_string()]).await;
        
        let trust = zt.get_device_trust("device1").await.unwrap();
        assert_eq!(trust.score, 85);
        assert_eq!(trust.device_id, "device1");
    }

    #[tokio::test]
    async fn test_session_management() {
        let config = ZeroTrustConfig::default();
        let zt = ZeroTrust::new(config);

        zt.create_session("user1".to_string()).await;
        assert!(zt.validate_session("user1").await);

        zt.revoke_session("user1").await;
        assert!(!zt.validate_session("user1").await);
    }
}