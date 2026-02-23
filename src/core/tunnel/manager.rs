//! # Tunnel Manager
//! 
//! Manages multiple VPN tunnels.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::Tunnel;

/// Tunnel manager
pub struct TunnelManager {
    tunnels: Arc<RwLock<HashMap<String, Arc<Tunnel>>>>,
}

impl TunnelManager {
    /// Create a new tunnel manager
    pub fn new() -> Self {
        Self {
            tunnels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add a tunnel
    pub async fn add_tunnel(&self, tunnel: Arc<Tunnel>) -> crate::Result<()> {
        let mut tunnels = self.tunnels.write().await;
        let id = tunnel.id().to_string();
        
        if tunnels.contains_key(&id) {
            return Err(crate::VantisError::TunnelExists(id));
        }
        
        tunnels.insert(id, tunnel);
        Ok(())
    }
    
    /// Remove a tunnel
    pub async fn remove_tunnel(&self, id: &str) -> crate::Result<()> {
        let mut tunnels = self.tunnels.write().await;
        
        if !tunnels.contains_key(id) {
            return Err(crate::VantisError::TunnelNotFound(id.to_string()));
        }
        
        tunnels.remove(id);
        Ok(())
    }
    
    /// Get a tunnel by ID
    pub async fn get_tunnel(&self, id: &str) -> crate::Result<Arc<Tunnel>> {
        let tunnels = self.tunnels.read().await;
        
        tunnels
            .get(id)
            .cloned()
            .ok_or_else(|| crate::VantisError::TunnelNotFound(id.to_string()))
    }
    
    /// Get all tunnels
    pub async fn list_tunnels(&self) -> Vec<String> {
        let tunnels = self.tunnels.read().await;
        tunnels.keys().cloned().collect()
    }
    
    /// Get active tunnel
    pub async fn get_active_tunnel(&self) -> crate::Result<Arc<Tunnel>> {
        let tunnels = self.tunnels.read().await;
        
        for tunnel in tunnels.values() {
            if tunnel.is_connected().await {
                return Ok(tunnel.clone());
            }
        }
        
        Err(crate::VantisError::NoActiveTunnel)
    }
}

impl Default for TunnelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = TunnelManager::new();
        assert_eq!(manager.list_tunnels().await.len(), 0);
    }
}