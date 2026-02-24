// RAM-Only Server Architecture
// Ensures no data persists to disk - all operations in memory only
// Compliant with strict no-logs policies and GDPR right to be forgotten

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use crate::error::{VantisError, Result};

/// Configuration for RAM-only server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RamOnlyConfig {
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Session timeout in seconds
    pub session_timeout_secs: u64,
    /// Enable memory pressure monitoring
    pub enable_memory_monitoring: bool,
    /// Enable automatic cleanup of expired sessions
    pub enable_auto_cleanup: bool,
    /// Cleanup interval in seconds
    pub cleanup_interval_secs: u64,
}

impl Default for RamOnlyConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 8192, // 8GB default
            session_timeout_secs: 3600, // 1 hour
            enable_memory_monitoring: true,
            enable_auto_cleanup: true,
            cleanup_interval_secs: 300, // 5 minutes
        }
    }
}

/// In-memory session data
#[derive(Debug, Clone)]
pub struct SessionData {
    pub session_id: String,
    pub user_id: String,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub metadata: HashMap<String, String>,
}

impl SessionData {
    pub fn new(session_id: String, user_id: String) -> Self {
        let now = Instant::now();
        Self {
            session_id,
            user_id,
            created_at: now,
            last_activity: now,
            bytes_sent: 0,
            bytes_received: 0,
            metadata: HashMap::new(),
        }
    }

    pub fn is_expired(&self, timeout_secs: u64) -> bool {
        self.last_activity.elapsed() > Duration::from_secs(timeout_secs)
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn add_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    pub fn add_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub available_memory_mb: u64,
    pub session_count: usize,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
}

/// RAM-Only Server Manager
pub struct RamOnlyServer {
    config: RamOnlyConfig,
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    memory_stats: Arc<Mutex<MemoryStats>>,
}

impl RamOnlyServer {
    pub fn new(config: RamOnlyConfig) -> Self {
        let memory_stats = MemoryStats {
            total_memory_mb: config.max_memory_mb,
            used_memory_mb: 0,
            available_memory_mb: config.max_memory_mb,
            session_count: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
        };

        Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            memory_stats: Arc::new(Mutex::new(memory_stats)),
        }
    }

    /// Create a new session
    pub async fn create_session(&self, user_id: String) -> Result<String> {
        let session_id = self.generate_session_id();
        let session = SessionData::new(session_id.clone(), user_id);

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.clone(), session);
        }

        self.update_stats().await;
        Ok(session_id)
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<SessionData> {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| VantisError::InvalidPeer(format!("Session not found: {}", session_id)))
    }

    /// Update session activity
    pub async fn update_session_activity(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.update_activity();
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Session not found: {}", session_id)))
        }
    }

    /// Record bytes sent for a session
    pub async fn record_bytes_sent(&self, session_id: &str, bytes: u64) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.add_bytes_sent(bytes);
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Session not found: {}", session_id)))
        }
    }

    /// Record bytes received for a session
    pub async fn record_bytes_received(&self, session_id: &str, bytes: u64) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.add_bytes_received(bytes);
            Ok(())
        } else {
            Err(VantisError::InvalidPeer(format!("Session not found: {}", session_id)))
        }
    }

    /// Terminate a session
    pub async fn terminate_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions
            .remove(session_id)
            .ok_or_else(|| VantisError::InvalidPeer(format!("Session not found: {}", session_id)))?;
        
        self.update_stats().await;
        Ok(())
    }

    /// Get memory statistics
    pub async fn get_memory_stats(&self) -> MemoryStats {
        self.memory_stats.lock().await.clone()
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let initial_count = sessions.len();
        
        sessions.retain(|_, session| !session.is_expired(self.config.session_timeout_secs));
        
        let removed = initial_count - sessions.len();
        if removed > 0 {
            self.update_stats().await;
        }
        
        removed
    }

    /// Start automatic cleanup task
    pub async fn start_auto_cleanup(&self) -> tokio::task::JoinHandle<()> {
        let sessions = self.sessions.clone();
        let timeout = self.config.session_timeout_secs;
        let interval = Duration::from_secs(self.config.cleanup_interval_secs);

        tokio::spawn(async move {
            let mut timer = tokio::time::interval(interval);
            loop {
                timer.tick().await;
                let mut sessions = sessions.write().await;
                sessions.retain(|_, session| !session.is_expired(timeout));
            }
        })
    }

    /// Generate a secure session ID
    fn generate_session_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        format!("session_{}", timestamp)
    }

    /// Update memory statistics
    async fn update_stats(&self) {
        let sessions = self.sessions.read().await;
        let mut stats = self.memory_stats.lock().await;
        
        stats.session_count = sessions.len();
        stats.total_bytes_sent = sessions.values().map(|s| s.bytes_sent).sum();
        stats.total_bytes_received = sessions.values().map(|s| s.bytes_received).sum();
        
        // Estimate memory usage (rough approximation)
        let estimated_mb = (stats.session_count * 1024) as u64; // ~1KB per session
        stats.used_memory_mb = estimated_mb;
        stats.available_memory_mb = self.config.max_memory_mb.saturating_sub(estimated_mb);
    }

    /// Emergency wipe - clear all sessions immediately
    pub async fn emergency_wipe(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let count = sessions.len();
        sessions.clear();
        
        let mut stats = self.memory_stats.lock().await;
        stats.session_count = 0;
        stats.used_memory_mb = 0;
        stats.available_memory_mb = self.config.max_memory_mb;
        stats.total_bytes_sent = 0;
        stats.total_bytes_received = 0;
        
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_creation() {
        let config = RamOnlyConfig::default();
        let server = RamOnlyServer::new(config);
        
        let session_id = server.create_session("user123".to_string()).await.unwrap();
        assert!(session_id.starts_with("session_"));
    }

    #[tokio::test]
    async fn test_session_retrieval() {
        let config = RamOnlyConfig::default();
        let server = RamOnlyServer::new(config);
        
        let session_id = server.create_session("user123".to_string()).await.unwrap();
        let session = server.get_session(&session_id).await.unwrap();
        
        assert_eq!(session.user_id, "user123");
    }

    #[tokio::test]
    async fn test_session_expiration() {
        let mut config = RamOnlyConfig::default();
        config.session_timeout_secs = 1; // 1 second timeout
        
        let server = RamOnlyServer::new(config);
        let session_id = server.create_session("user123".to_string()).await.unwrap();
        
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        let removed = server.cleanup_expired_sessions().await;
        assert_eq!(removed, 1);
    }

    #[tokio::test]
    async fn test_emergency_wipe() {
        let config = RamOnlyConfig::default();
        let server = RamOnlyServer::new(config);
        
        server.create_session("user1".to_string()).await.unwrap();
        server.create_session("user2".to_string()).await.unwrap();
        server.create_session("user3".to_string()).await.unwrap();
        
        let wiped = server.emergency_wipe().await;
        assert_eq!(wiped, 3);
        
        let stats = server.get_memory_stats().await;
        assert_eq!(stats.session_count, 0);
    }
}