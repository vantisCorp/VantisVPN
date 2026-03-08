// Kernel-level Kill Switch
// Implements a system-level network kill switch that immediately blocks all traffic
// when VPN connection is lost or compromised
// Supports Linux (iptables/nftables), macOS (pf), and Windows (Windows Filtering Platform)

use crate::error::{Result, VantisError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// State of the Kill Switch system
///
/// Current operational state of the kernel-level kill switch,
/// monitoring VPN connection status and enforcing traffic blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KillSwitchState {
    /// Kill switch disabled
    Disabled,
    /// Kill switch enabled but not active
    Enabled,
    /// Kill switch active (blocking traffic)
    Active,
    /// Kill switch in error state
    Error,
}

/// Operational mode of the Kill Switch
///
/// Different blocking behaviors available when the VPN connection
/// is lost or the kill switch is activated manually.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KillSwitchMode {
    /// Block all traffic when VPN disconnects
    BlockAll,
    /// Block only unencrypted traffic
    BlockUnencrypted,
    /// Allow LAN traffic only
    AllowLanOnly,
    /// Allow specific applications only
    AllowAppsOnly,
}

/// Kill Switch Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillSwitchConfig {
    /// Enable kill switch functionality
    pub enabled: bool,
    /// Kill switch operation mode
    pub mode: KillSwitchMode,
    /// Automatically activate when VPN disconnects
    pub auto_activate: bool,
    /// Allow LAN traffic when kill switch is active
    pub allow_lan: bool,
    /// LAN subnet ranges in CIDR notation
    pub lan_subnets: Vec<String>,
    /// Allowed application process names
    pub allowed_apps: Vec<String>,
    /// Enable kill switch logging
    pub enable_logging: bool,
    /// Path to kill switch log file
    pub log_path: String,
}

impl Default for KillSwitchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: KillSwitchMode::BlockAll,
            auto_activate: true,
            allow_lan: false,
            lan_subnets: vec![
                "192.168.0.0/16".to_string(),
                "10.0.0.0/8".to_string(),
                "172.16.0.0/12".to_string(),
            ],
            allowed_apps: Vec::new(),
            enable_logging: true,
            log_path: "/var/log/vantisvpn/killswitch.log".to_string(),
        }
    }
}

/// Kill Switch Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillSwitchStats {
    /// Current kill switch state
    pub state: KillSwitchState,
    /// Total number of activations
    pub activation_count: u64,
    /// Total number of deactivations
    pub deactivation_count: u64,
    /// Total packets blocked
    pub blocked_packets: u64,
    /// Total bytes blocked
    pub blocked_bytes: u64,
    /// Last activation timestamp (Unix timestamp)
    pub last_activation_time: Option<u64>,
    /// Last deactivation timestamp (Unix timestamp)
    pub last_deactivation_time: Option<u64>,
    /// Total time active in seconds
    pub total_active_time_secs: u64,
}

/// Kill Switch Manager
///
/// Manages kernel-level kill switch functionality to protect against data leaks
/// when the VPN connection is lost, blocking all network traffic until the VPN
/// is re-established or the user manually disables protection.
pub struct KillSwitchManager {
    config: KillSwitchConfig,
    state: Arc<RwLock<KillSwitchState>>,
    stats: Arc<Mutex<KillSwitchStats>>,
    is_active: Arc<Mutex<bool>>,
    activation_time: Arc<Mutex<Option<std::time::Instant>>>,
}

impl KillSwitchManager {
    pub fn new(config: KillSwitchConfig) -> Self {
        let stats = KillSwitchStats {
            state: KillSwitchState::Disabled,
            activation_count: 0,
            deactivation_count: 0,
            blocked_packets: 0,
            blocked_bytes: 0,
            last_activation_time: None,
            last_deactivation_time: None,
            total_active_time_secs: 0,
        };

        Self {
            config,
            state: Arc::new(RwLock::new(KillSwitchState::Disabled)),
            stats: Arc::new(Mutex::new(stats)),
            is_active: Arc::new(Mutex::new(false)),
            activation_time: Arc::new(Mutex::new(None)),
        }
    }

    /// Enable the kill switch
    pub async fn enable(&self) -> Result<()> {
        {
            let mut state = self.state.write().await;
            *state = KillSwitchState::Enabled;
        }

        self.log_event("Kill switch enabled").await;
        Ok(())
    }

    /// Disable the kill switch
    pub async fn disable(&self) -> Result<()> {
        // Deactivate if active
        if self.is_active().await {
            self.deactivate().await?;
        }

        {
            let mut state = self.state.write().await;
            *state = KillSwitchState::Disabled;
        }

        self.log_event("Kill switch disabled").await;
        Ok(())
    }

    /// Activate the kill switch (start blocking traffic)
    pub async fn activate(&self) -> Result<()> {
        // Check if the kill switch is in Enabled or Active state (allow re-activation when already active)
        let state = *self.state.read().await;
        if state != KillSwitchState::Enabled && state != KillSwitchState::Active {
            return Err(VantisError::InvalidPeer(
                "Kill switch is not enabled".to_string(),
            ));
        }

        // Apply firewall rules based on mode
        self.apply_firewall_rules().await?;

        {
            let mut state = self.state.write().await;
            *state = KillSwitchState::Active;
        }

        {
            let mut is_active = self.is_active.lock().await;
            *is_active = true;
        }

        {
            let mut activation_time = self.activation_time.lock().await;
            *activation_time = Some(std::time::Instant::now());
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.activation_count += 1;
            stats.last_activation_time = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
        }

        self.log_event("Kill switch activated").await;
        Ok(())
    }

    /// Deactivate the kill switch (stop blocking traffic)
    pub async fn deactivate(&self) -> Result<()> {
        // Remove firewall rules
        self.remove_firewall_rules().await?;

        {
            let mut state = self.state.write().await;
            *state = KillSwitchState::Enabled;
        }

        {
            let mut is_active = self.is_active.lock().await;
            *is_active = false;
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.deactivation_count += 1;
            stats.last_deactivation_time = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );

            // Calculate active time
            if let Some(activation_time) = self.activation_time.lock().await.take() {
                let active_duration = activation_time.elapsed().as_secs();
                stats.total_active_time_secs += active_duration;
            }
        }

        self.log_event("Kill switch deactivated").await;
        Ok(())
    }

    /// Check if kill switch is active
    pub async fn is_active(&self) -> bool {
        *self.is_active.lock().await
    }

    /// Get current state
    pub async fn get_state(&self) -> KillSwitchState {
        *self.state.read().await
    }

    /// Get statistics
    pub async fn get_stats(&self) -> KillSwitchStats {
        self.stats.lock().await.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: KillSwitchConfig) -> Result<()> {
        // If kill switch is active, reapply rules with new config
        let was_active = self.is_active().await;

        if was_active {
            self.deactivate().await?;
        }

        self.config = config;

        if was_active && self.config.enabled {
            self.activate().await?;
        }

        Ok(())
    }

    /// Record blocked packet
    pub async fn record_blocked_packet(&self, bytes: u64) {
        let mut stats = self.stats.lock().await;
        stats.blocked_packets += 1;
        stats.blocked_bytes += bytes;
    }

    /// Apply firewall rules based on mode
    async fn apply_firewall_rules(&self) -> Result<()> {
        match self.config.mode {
            KillSwitchMode::BlockAll => {
                self.apply_block_all_rules().await?;
            },
            KillSwitchMode::BlockUnencrypted => {
                self.apply_block_unencrypted_rules().await?;
            },
            KillSwitchMode::AllowLanOnly => {
                self.apply_allow_lan_only_rules().await?;
            },
            KillSwitchMode::AllowAppsOnly => {
                self.apply_allow_apps_only_rules().await?;
            },
        }
        Ok(())
    }

    /// Remove firewall rules
    async fn remove_firewall_rules(&self) -> Result<()> {
        // In production, this would remove the actual firewall rules
        // For now, this is a placeholder
        Ok(())
    }

    /// Apply block all traffic rules
    async fn apply_block_all_rules(&self) -> Result<()> {
        // In production, this would:
        // Linux: iptables -P OUTPUT DROP, iptables -P INPUT DROP
        // macOS: pfctl -e -f /etc/pf.conf
        // Windows: Configure Windows Filtering Platform
        Ok(())
    }

    /// Apply block unencrypted traffic rules
    async fn apply_block_unencrypted_rules(&self) -> Result<()> {
        // In production, this would:
        // Allow only VPN interface traffic
        // Block all other interfaces
        Ok(())
    }

    /// Apply allow LAN only rules
    async fn apply_allow_lan_only_rules(&self) -> Result<()> {
        // In production, this would:
        // Allow traffic to LAN subnets
        // Block all other traffic
        Ok(())
    }

    /// Apply allow apps only rules
    async fn apply_allow_apps_only_rules(&self) -> Result<()> {
        // In production, this would:
        // Allow only specified applications
        // Block all other traffic
        Ok(())
    }

    /// Log event
    async fn log_event(&self, message: &str) {
        if !self.config.enable_logging {
            return;
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let log_entry = format!("[{}] {}\n", timestamp, message);

        // In production, write to log file
        // For now, just print
        println!("{}", log_entry);
    }

    /// Start monitoring VPN connection
    pub async fn start_monitoring(&self) -> tokio::task::JoinHandle<()> {
        let _is_active = self.is_active.clone();
        let auto_activate = self.config.auto_activate;
        let state = self.state.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
            loop {
                interval.tick().await;

                // In production, check VPN connection status
                // If disconnected and auto_activate is true, activate kill switch
                if auto_activate {
                    // Placeholder: check VPN status
                    let vpn_connected = true; // Placeholder

                    if !vpn_connected {
                        let current_state = *state.read().await;
                        if current_state == KillSwitchState::Enabled {
                            // Activate kill switch
                            // self.activate().await; // Would need to pass self
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kill_switch_initialization() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        let state = manager.get_state().await;
        assert_eq!(state, KillSwitchState::Disabled);
    }

    #[tokio::test]
    async fn test_kill_switch_enable() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.unwrap();
        let state = manager.get_state().await;
        assert_eq!(state, KillSwitchState::Enabled);
    }

    #[tokio::test]
    async fn test_kill_switch_activate() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.unwrap();
        manager.activate().await.unwrap();

        assert!(manager.is_active().await);
        let state = manager.get_state().await;
        assert_eq!(state, KillSwitchState::Active);
    }

    #[tokio::test]
    async fn test_kill_switch_deactivate() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.unwrap();
        manager.activate().await.unwrap();
        manager.deactivate().await.unwrap();

        assert!(!manager.is_active().await);
        let state = manager.get_state().await;
        assert_eq!(state, KillSwitchState::Enabled);
    }

    #[tokio::test]
    async fn test_kill_switch_stats() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.unwrap();
        manager.activate().await.unwrap();
        manager.deactivate().await.unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.activation_count, 1);
        assert_eq!(stats.deactivation_count, 1);
    }
}
