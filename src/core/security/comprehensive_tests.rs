//! # Security Module Comprehensive Tests
//!
//! Comprehensive tests for security features including kill switch,
//! split tunneling, and network protection.

use super::*;
use crate::error::{VantisError, Result};

// =============================================================================
// Kill Switch State Tests
// =============================================================================

#[cfg(test)]
mod kill_switch_state_tests {
    use super::*;

    #[test]
    fn test_state_default() {
        let state = KillSwitchState::Disabled;
        assert_eq!(state, KillSwitchState::Disabled);
    }

    #[test]
    fn test_state_equality() {
        assert_eq!(KillSwitchState::Active, KillSwitchState::Active);
        assert_ne!(KillSwitchState::Active, KillSwitchState::Disabled);
    }

    #[test]
    fn test_state_copy() {
        let state1 = KillSwitchState::Active;
        let state2 = state1;
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_state_serialization() {
        let state = KillSwitchState::Active;
        let json = serde_json::to_string(&state).expect("Serialization failed");
        let decoded: KillSwitchState = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(state, decoded);
    }

    #[test]
    fn test_all_states() {
        let states = [
            KillSwitchState::Disabled,
            KillSwitchState::Enabled,
            KillSwitchState::Active,
            KillSwitchState::Error,
        ];

        for state in &states {
            let json = serde_json::to_string(state).expect("Serialization failed");
            let decoded: KillSwitchState = serde_json::from_str(&json).expect("Deserialization failed");
            assert_eq!(*state, decoded);
        }
    }
}

// =============================================================================
// Kill Switch Mode Tests
// =============================================================================

#[cfg(test)]
mod kill_switch_mode_tests {
    use super::*;

    #[test]
    fn test_mode_equality() {
        assert_eq!(KillSwitchMode::BlockAll, KillSwitchMode::BlockAll);
        assert_ne!(KillSwitchMode::BlockAll, KillSwitchMode::AllowLanOnly);
    }

    #[test]
    fn test_mode_copy() {
        let mode1 = KillSwitchMode::BlockAll;
        let mode2 = mode1;
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_mode_serialization() {
        let mode = KillSwitchMode::BlockAll;
        let json = serde_json::to_string(&mode).expect("Serialization failed");
        let decoded: KillSwitchMode = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(mode, decoded);
    }

    #[test]
    fn test_all_modes() {
        let modes = [
            KillSwitchMode::BlockAll,
            KillSwitchMode::BlockUnencrypted,
            KillSwitchMode::AllowLanOnly,
            KillSwitchMode::AllowAppsOnly,
        ];

        for mode in &modes {
            let json = serde_json::to_string(mode).expect("Serialization failed");
            let decoded: KillSwitchMode = serde_json::from_str(&json).expect("Deserialization failed");
            assert_eq!(*mode, decoded);
        }
    }
}

// =============================================================================
// Kill Switch Config Tests
// =============================================================================

#[cfg(test)]
mod kill_switch_config_tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = KillSwitchConfig::default();
        assert!(config.enabled);
        assert_eq!(config.mode, KillSwitchMode::BlockAll);
        assert!(config.auto_activate);
        assert!(!config.allow_lan);
        assert!(config.enable_logging);
    }

    #[test]
    fn test_config_default_lan_subnets() {
        let config = KillSwitchConfig::default();
        assert_eq!(config.lan_subnets.len(), 3);
        assert!(config.lan_subnets.contains(&"192.168.0.0/16".to_string()));
        assert!(config.lan_subnets.contains(&"10.0.0.0/8".to_string()));
        assert!(config.lan_subnets.contains(&"172.16.0.0/12".to_string()));
    }

    #[test]
    fn test_config_custom() {
        let config = KillSwitchConfig {
            enabled: false,
            mode: KillSwitchMode::AllowLanOnly,
            auto_activate: false,
            allow_lan: true,
            lan_subnets: vec!["192.168.1.0/24".to_string()],
            allowed_apps: vec!["firefox".to_string(), "chrome".to_string()],
            enable_logging: false,
            log_path: "/tmp/killswitch.log".to_string(),
        };

        assert!(!config.enabled);
        assert_eq!(config.mode, KillSwitchMode::AllowLanOnly);
        assert!(config.allow_lan);
        assert_eq!(config.allowed_apps.len(), 2);
    }

    #[test]
    fn test_config_clone() {
        let config = KillSwitchConfig::default();
        let cloned = config.clone();
        assert_eq!(config.enabled, cloned.enabled);
        assert_eq!(config.mode, cloned.mode);
    }

    #[test]
    fn test_config_serialization() {
        let config = KillSwitchConfig::default();
        let json = serde_json::to_string(&config).expect("Serialization failed");
        let decoded: KillSwitchConfig = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(config.enabled, decoded.enabled);
        assert_eq!(config.mode, decoded.mode);
    }

    #[test]
    fn test_config_all_modes() {
        for mode in [
            KillSwitchMode::BlockAll,
            KillSwitchMode::BlockUnencrypted,
            KillSwitchMode::AllowLanOnly,
            KillSwitchMode::AllowAppsOnly,
        ] {
            let config = KillSwitchConfig {
                mode,
                ..Default::default()
            };
            assert_eq!(config.mode, mode);
        }
    }
}

// =============================================================================
// Kill Switch Manager Tests
// =============================================================================

#[cfg(test)]
mod kill_switch_manager_tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_creation() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        assert_eq!(manager.get_state().await, KillSwitchState::Disabled);
        assert!(!manager.is_active().await);
    }

    #[tokio::test]
    async fn test_manager_enable() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");
        assert_eq!(manager.get_state().await, KillSwitchState::Enabled);
    }

    #[tokio::test]
    async fn test_manager_disable() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");
        assert_eq!(manager.get_state().await, KillSwitchState::Enabled);

        manager.disable().await.expect("Disable failed");
        assert_eq!(manager.get_state().await, KillSwitchState::Disabled);
    }

    #[tokio::test]
    async fn test_manager_activate() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");
        manager.activate().await.expect("Activate failed");

        assert!(manager.is_active().await);
        assert_eq!(manager.get_state().await, KillSwitchState::Active);
    }

    #[tokio::test]
    async fn test_manager_deactivate() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");
        manager.activate().await.expect("Activate failed");
        assert!(manager.is_active().await);

        manager.deactivate().await.expect("Deactivate failed");
        assert!(!manager.is_active().await);
        assert_eq!(manager.get_state().await, KillSwitchState::Enabled);
    }

    #[tokio::test]
    async fn test_activate_without_enable() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        let result = manager.activate().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_full_lifecycle() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        // Disabled -> Enabled
        manager.enable().await.expect("Enable failed");
        assert_eq!(manager.get_state().await, KillSwitchState::Enabled);

        // Enabled -> Active
        manager.activate().await.expect("Activate failed");
        assert_eq!(manager.get_state().await, KillSwitchState::Active);
        assert!(manager.is_active().await);

        // Active -> Enabled
        manager.deactivate().await.expect("Deactivate failed");
        assert_eq!(manager.get_state().await, KillSwitchState::Enabled);
        assert!(!manager.is_active().await);

        // Enabled -> Disabled
        manager.disable().await.expect("Disable failed");
        assert_eq!(manager.get_state().await, KillSwitchState::Disabled);
    }

    #[tokio::test]
    async fn test_multiple_activations() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");

        for i in 0..5 {
            manager.activate().await.expect(&format!("Activation {} failed", i));
            assert!(manager.is_active().await);
            manager.deactivate().await.expect(&format!("Deactivation {} failed", i));
            assert!(!manager.is_active().await);
        }

        let stats = manager.get_stats().await;
        assert_eq!(stats.activation_count, 5);
        assert_eq!(stats.deactivation_count, 5);
    }

    #[tokio::test]
    async fn test_get_statistics() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.state, KillSwitchState::Disabled);
        assert_eq!(stats.activation_count, 0);
        assert_eq!(stats.deactivation_count, 0);
    }

    #[tokio::test]
    async fn test_statistics_tracking() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");
        manager.activate().await.expect("Activate failed");

        let stats = manager.get_stats().await;
        assert_eq!(stats.activation_count, 1);
        assert!(stats.last_activation_time.is_some());

        manager.deactivate().await.expect("Deactivate failed");

        let stats = manager.get_stats().await;
        assert_eq!(stats.deactivation_count, 1);
        assert!(stats.last_deactivation_time.is_some());
    }

    #[tokio::test]
    async fn test_record_blocked_packet() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.record_blocked_packet(1024).await;
        manager.record_blocked_packet(512).await;

        let stats = manager.get_stats().await;
        assert_eq!(stats.blocked_packets, 2);
        assert_eq!(stats.blocked_bytes, 1536);
    }

    #[tokio::test]
    async fn test_update_config() {
        let config = KillSwitchConfig::default();
        let mut manager = KillSwitchManager::new(config.clone());

        let new_config = KillSwitchConfig {
            mode: KillSwitchMode::AllowLanOnly,
            ..config
        };

        manager.update_config(new_config).await.expect("Config update failed");
    }

    #[tokio::test]
    async fn test_update_config_while_active() {
        let config = KillSwitchConfig::default();
        let mut manager = KillSwitchManager::new(config.clone());

        manager.enable().await.expect("Enable failed");
        manager.activate().await.expect("Activate failed");
        assert!(manager.is_active().await);

        let new_config = KillSwitchConfig {
            mode: KillSwitchMode::AllowLanOnly,
            ..config
        };

        manager.update_config(new_config).await.expect("Config update failed");
        // Should be reactivated with new config
        assert!(manager.is_active().await);
    }

    #[tokio::test]
    async fn test_start_monitoring() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        let _handle = manager.start_monitoring().await;
        // Monitoring task started
    }
}

// =============================================================================
// Kill Switch Stats Tests
// =============================================================================

#[cfg(test)]
mod kill_switch_stats_tests {
    use super::*;

    #[test]
    fn test_stats_creation() {
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

        assert_eq!(stats.state, KillSwitchState::Disabled);
    }

    #[test]
    fn test_stats_clone() {
        let stats = KillSwitchStats {
            state: KillSwitchState::Active,
            activation_count: 10,
            deactivation_count: 10,
            blocked_packets: 1000,
            blocked_bytes: 1024000,
            last_activation_time: Some(100),
            last_deactivation_time: Some(200),
            total_active_time_secs: 600,
        };

        let cloned = stats.clone();
        assert_eq!(stats.activation_count, cloned.activation_count);
        assert_eq!(stats.blocked_packets, cloned.blocked_packets);
    }

    #[test]
    fn test_stats_serialization() {
        let stats = KillSwitchStats {
            state: KillSwitchState::Active,
            activation_count: 1,
            deactivation_count: 0,
            blocked_packets: 100,
            blocked_bytes: 1024,
            last_activation_time: Some(100),
            last_deactivation_time: None,
            total_active_time_secs: 60,
        };

        let json = serde_json::to_string(&stats).expect("Serialization failed");
        let decoded: KillSwitchStats = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(stats.activation_count, decoded.activation_count);
    }

    #[test]
    fn test_stats_timestamps() {
        let stats = KillSwitchStats {
            state: KillSwitchState::Active,
            activation_count: 1,
            deactivation_count: 0,
            blocked_packets: 0,
            blocked_bytes: 0,
            last_activation_time: Some(1234567890),
            last_deactivation_time: Some(1234567950),
            total_active_time_secs: 60,
        };

        assert_eq!(stats.last_activation_time, Some(1234567890));
        assert_eq!(stats.last_deactivation_time, Some(1234567950));
    }

    #[test]
    fn test_stats_large_values() {
        let stats = KillSwitchStats {
            state: KillSwitchState::Active,
            activation_count: 1_000_000,
            deactivation_count: 999_999,
            blocked_packets: 10_000_000,
            blocked_bytes: 10_000_000_000,
            last_activation_time: Some(1234567890),
            last_deactivation_time: Some(1234567950),
            total_active_time_secs: 86400, // 24 hours
        };

        assert_eq!(stats.blocked_packets, 10_000_000);
        assert_eq!(stats.total_active_time_secs, 86400);
    }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_activate_when_disabled() {
        let config = KillSwitchConfig {
            enabled: false,
            ..Default::default()
        };
        let manager = KillSwitchManager::new(config);

        // Don't enable, try to activate directly
        let result = manager.activate().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_activate_when_already_active() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");
        manager.activate().await.expect("First activation failed");

        // Try to activate again when already active
        // Should still succeed (idempotent)
        let result = manager.activate().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_deactivate_when_not_active() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");

        // Try to deactivate when not active
        let result = manager.deactivate().await;
        assert!(result.is_ok()); // Should be idempotent
    }

    #[tokio::test]
    async fn test_disable_when_active() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");
        manager.activate().await.expect("Activate failed");
        assert!(manager.is_active().await);

        // Disable while active (should deactivate first)
        let result = manager.disable().await;
        assert!(result.is_ok());
        assert!(!manager.is_active().await);
    }
}

// =============================================================================
// Performance Tests
// =============================================================================

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_activation_performance() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        manager.enable().await.expect("Enable failed");

        let start = Instant::now();
        for _ in 0..100 {
            manager.activate().await.expect("Activate failed");
            manager.deactivate().await.expect("Deactivate failed");
        }
        let duration = start.elapsed();

        println!("100 activate/deactivate cycles: {:?}", duration);
    }

    #[tokio::test]
    async fn test_stats_tracking_performance() {
        let config = KillSwitchConfig::default();
        let manager = KillSwitchManager::new(config);

        let start = Instant::now();
        for _ in 0..10000 {
            manager.record_blocked_packet(1024).await;
        }
        let duration = start.elapsed();

        println!("10,000 packet records: {:?}", duration);

        let stats = manager.get_stats().await;
        assert_eq!(stats.blocked_packets, 10000);
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        use std::sync::Arc;
        let config = KillSwitchConfig::default();
        let manager = Arc::new(KillSwitchManager::new(config));

        manager.enable().await.expect("Enable failed");

        let start = Instant::now();
        let mut handles = vec![];

        for i in 0..50 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move {
                for j in 0..20 {
                    if j % 2 == 0 {
                        let _ = manager_clone.record_blocked_packet(1024).await;
                    } else {
                        let stats = manager_clone.get_stats().await;
                        let _ = stats.blocked_packets;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let duration = start.elapsed();
        println!("50 concurrent threads × 20 operations: {:?}", duration);

        let stats = manager.get_stats().await;
        assert!(stats.blocked_packets > 0);
    }
}