//! # Tunnel Module Comprehensive Tests
//!
//! Comprehensive tests for VPN tunnel management, state transitions,
//! configuration, and statistics tracking.

use super::*;
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// Tunnel Statistics Tests
// =============================================================================

#[cfg(test)]
mod tunnel_stats_tests {
    use super::*;

    #[test]
    fn test_stats_default() {
        let stats = TunnelStats::default();
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.packets_sent, 0);
        assert_eq!(stats.packets_received, 0);
        assert_eq!(stats.uptime, 0);
    }

    #[test]
    fn test_stats_update() {
        let mut stats = TunnelStats::default();
        stats.update(1000, 500);

        assert_eq!(stats.bytes_sent, 1000);
        assert_eq!(stats.bytes_received, 500);
        assert_eq!(stats.packets_sent, 1);
        assert_eq!(stats.packets_received, 1);
    }

    #[test]
    fn test_stats_multiple_updates() {
        let mut stats = TunnelStats::default();

        stats.update(1000, 500);
        stats.update(2000, 1000);
        stats.update(500, 250);

        assert_eq!(stats.bytes_sent, 3500);
        assert_eq!(stats.bytes_received, 1750);
        assert_eq!(stats.packets_sent, 3);
        assert_eq!(stats.packets_received, 3);
    }

    #[test]
    fn test_stats_large_values() {
        let mut stats = TunnelStats::default();
        
        // Simulate large data transfer (1GB)
        stats.update(1_000_000_000, 1_000_000_000);
        
        assert_eq!(stats.bytes_sent, 1_000_000_000);
        assert_eq!(stats.bytes_received, 1_000_000_000);
    }

    #[test]
    fn test_stats_zero_values() {
        let mut stats = TunnelStats::default();
        stats.update(0, 0);

        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.packets_sent, 1);
        assert_eq!(stats.packets_received, 1);
    }
}

// =============================================================================
// Tunnel Configuration Tests
// =============================================================================

#[cfg(test)]
mod tunnel_config_tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = TunnelConfig::default();

        assert_eq!(config.server_endpoint, "");
        assert_eq!(config.virtual_ip, "10.0.0.2");
        assert_eq!(config.dns_servers, vec!["1.1.1.1", "1.0.0.1"]);
        assert_eq!(config.mtu, 1420);
        assert!(config.enable_kill_switch);
        assert!(!config.enable_split_tunneling);
        assert!(config.split_tunnel_apps.is_empty());
    }

    #[test]
    fn test_config_custom() {
        let config = TunnelConfig {
            server_endpoint: "vpn.example.com:443".to_string(),
            virtual_ip: "10.0.0.5".to_string(),
            dns_servers: vec!["8.8.8.8".to_string()],
            mtu: 1500,
            enable_kill_switch: false,
            enable_split_tunneling: true,
            split_tunnel_apps: vec!["firefox".to_string(), "chrome".to_string()],
        };

        assert_eq!(config.server_endpoint, "vpn.example.com:443");
        assert_eq!(config.virtual_ip, "10.0.0.5");
        assert_eq!(config.dns_servers.len(), 1);
        assert_eq!(config.mtu, 1500);
        assert!(!config.enable_kill_switch);
        assert!(config.enable_split_tunneling);
        assert_eq!(config.split_tunnel_apps.len(), 2);
    }

    #[test]
    fn test_config_clone() {
        let config = TunnelConfig::default();
        let cloned = config.clone();

        assert_eq!(config.server_endpoint, cloned.server_endpoint);
        assert_eq!(config.virtual_ip, cloned.virtual_ip);
        assert_eq!(config.mtu, cloned.mtu);
    }

    #[test]
    fn test_config_debug() {
        let config = TunnelConfig::default();
        let debug = format!("{:?}", config);

        assert!(debug.contains("TunnelConfig"));
    }
}

// =============================================================================
// Tunnel State Tests
// =============================================================================

#[cfg(test)]
mod tunnel_state_tests {
    use super::*;

    #[test]
    fn test_state_default() {
        let state = TunnelState::default();
        assert_eq!(state, TunnelState::Disconnected);
    }

    #[test]
    fn test_state_equality() {
        assert_eq!(TunnelState::Connected, TunnelState::Connected);
        assert_ne!(TunnelState::Connected, TunnelState::Disconnected);
    }

    #[test]
    fn test_state_copy() {
        let state1 = TunnelState::Connected;
        let state2 = state1;
        
        assert_eq!(state1, TunnelState::Connected);
        assert_eq!(state2, TunnelState::Connected);
    }

    #[test]
    fn test_state_display() {
        assert_eq!(TunnelState::Disconnected.to_string(), "Disconnected");
        assert_eq!(TunnelState::Connecting.to_string(), "Connecting");
        assert_eq!(TunnelState::Connected.to_string(), "Connected");
        assert_eq!(TunnelState::Disconnecting.to_string(), "Disconnecting");
        assert_eq!(TunnelState::Reconnecting.to_string(), "Reconnecting");
        assert_eq!(TunnelState::Error.to_string(), "Error");
    }

    #[test]
    fn test_state_debug() {
        let state = TunnelState::Connected;
        let debug = format!("{:?}", state);

        assert!(debug.contains("Connected"));
    }
}

// =============================================================================
// State Transition Tests
// =============================================================================

#[cfg(test)]
mod state_transition_tests {
    use super::*;

    fn test_transition(from: TunnelState, to: TunnelState, expected_valid: bool) {
        let transition = StateTransition { from, to };
        assert_eq!(transition.is_valid(), expected_valid);
    }

    #[test]
    fn test_valid_transitions() {
        // Disconnected -> Connecting
        test_transition(
            TunnelState::Disconnected,
            TunnelState::Connecting,
            true
        );

        // Connecting -> Connected
        test_transition(
            TunnelState::Connecting,
            TunnelState::Connected,
            true
        );

        // Connecting -> Error
        test_transition(
            TunnelState::Connecting,
            TunnelState::Error,
            true
        );

        // Connected -> Disconnecting
        test_transition(
            TunnelState::Connected,
            TunnelState::Disconnecting,
            true
        );

        // Connected -> Reconnecting
        test_transition(
            TunnelState::Connected,
            TunnelState::Reconnecting,
            true
        );

        // Disconnecting -> Disconnected
        test_transition(
            TunnelState::Disconnecting,
            TunnelState::Disconnected,
            true
        );

        // Reconnecting -> Connecting
        test_transition(
            TunnelState::Reconnecting,
            TunnelState::Connecting,
            true
        );

        // Error -> Disconnected
        test_transition(
            TunnelState::Error,
            TunnelState::Disconnected,
            true
        );

        // Error -> Reconnecting
        test_transition(
            TunnelState::Error,
            TunnelState::Reconnecting,
            true
        );
    }

    #[test]
    fn test_invalid_transitions() {
        // Connected -> Connected (no change)
        test_transition(
            TunnelState::Connected,
            TunnelState::Connected,
            false
        );

        // Disconnected -> Connected (skip connecting)
        test_transition(
            TunnelState::Disconnected,
            TunnelState::Connected,
            false
        );

        // Connected -> Disconnected (skip disconnecting)
        test_transition(
            TunnelState::Connected,
            TunnelState::Disconnected,
            false
        );

        // Disconnecting -> Connected (invalid)
        test_transition(
            TunnelState::Disconnecting,
            TunnelState::Connected,
            false
        );
    }

    #[test]
    fn test_all_state_transitions() {
        let states = [
            TunnelState::Disconnected,
            TunnelState::Connecting,
            TunnelState::Connected,
            TunnelState::Disconnecting,
            TunnelState::Reconnecting,
            TunnelState::Error,
        ];

        for from in states.iter() {
            for to in states.iter() {
                let transition = StateTransition {
                    from: *from,
                    to: *to,
                };
                
                // Just verify it doesn't panic
                let _is_valid = transition.is_valid();
            }
        }
    }
}

// =============================================================================
// Tunnel Integration Tests
// =============================================================================

#[cfg(test)]
mod tunnel_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_tunnel_creation() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-1".to_string(), config);

        assert_eq!(tunnel.id(), "test-tunnel-1");
        assert!(!tunnel.is_connected().await);
        assert_eq!(tunnel.state().await, TunnelState::Disconnected);
    }

    #[tokio::test]
    async fn test_tunnel_connect_disconnect() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-2".to_string(), config);

        // Connect
        tunnel.connect().await.expect("Connection failed");
        assert_eq!(tunnel.state().await, TunnelState::Connected);
        assert!(tunnel.is_connected().await);

        // Disconnect
        tunnel.disconnect().await.expect("Disconnect failed");
        assert_eq!(tunnel.state().await, TunnelState::Disconnected);
        assert!(!tunnel.is_connected().await);
    }

    #[tokio::test]
    async fn test_tunnel_config_access() {
        let config = TunnelConfig {
            server_endpoint: "test-server:443".to_string(),
            virtual_ip: "10.0.0.10".to_string(),
            ..Default::default()
        };

        let tunnel = Tunnel::new("test-tunnel-3".to_string(), config);
        assert_eq!(tunnel.config().server_endpoint, "test-server:443");
        assert_eq!(tunnel.config().virtual_ip, "10.0.0.10");
    }

    #[tokio::test]
    async fn test_tunnel_send_receive() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-4".to_string(), config);

        tunnel.connect().await.expect("Connection failed");

        // Send data
        tunnel.send(b"test data").await.expect("Send failed");

        // Receive data
        let _received = tunnel.receive().await.expect("Receive failed");

        // Check stats
        let stats = tunnel.stats().await;
        assert!(stats.bytes_sent > 0);
    }

    #[tokio::test]
    async fn test_send_without_connection() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-5".to_string(), config);

        // Should fail without connection
        let result = tunnel.send(b"test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_receive_without_connection() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-6".to_string(), config);

        // Should fail without connection
        let result = tunnel.receive().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_multiple_data_transfers() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-7".to_string(), config);

        tunnel.connect().await.expect("Connection failed");

        // Send multiple packets
        for i in 0..10 {
            let data = format!("packet {}", i).into_bytes();
            tunnel.send(&data).await.expect("Send failed");
        }

        let stats = tunnel.stats().await;
        assert_eq!(stats.packets_sent, 10);
    }

    #[tokio::test]
    async fn test_tunnel_concurrent_operations() {
        let config = TunnelConfig::default();
        let tunnel = Arc::new(Tunnel::new("test-tunnel-8".to_string(), config));

        tunnel.connect().await.expect("Connection failed");

        // Concurrent send operations
        let mut handles = vec![];
        for i in 0..5 {
            let tunnel_clone = tunnel.clone();
            let handle = tokio::spawn(async move {
                let data = format!("concurrent packet {}", i).into_bytes();
                tunnel_clone.send(&data).await
            });
            handles.push(handle);
        }

        // Wait for all to complete
        for handle in handles {
            handle.await.unwrap().expect("Send failed");
        }

        let stats = tunnel.stats().await;
        assert_eq!(stats.packets_sent, 5);
    }

    #[tokio::test]
    async fn test_tunnel_stats_tracking() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-9".to_string(), config);

        tunnel.connect().await.expect("Connection failed");

        // Send data of various sizes
        let sizes = vec![100u64, 200, 300, 400, 500];
        for size in &sizes {
            let data = vec![0u8; *size as usize];
            tunnel.send(&data).await.expect("Send failed");
        }

        let stats = tunnel.stats().await;
        let total_sent: u64 = sizes.iter().sum();
        assert_eq!(stats.bytes_sent, total_sent);
        assert_eq!(stats.packets_sent, 5);
    }

    #[tokio::test]
    async fn test_tunnel_reconnect_workflow() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("test-tunnel-10".to_string(), config);

        // Connect
        tunnel.connect().await.expect("Connection failed");
        assert!(tunnel.is_connected().await);

        // Disconnect
        tunnel.disconnect().await.expect("Disconnect failed");
        assert!(!tunnel.is_connected().await);

        // Reconnect
        tunnel.connect().await.expect("Reconnection failed");
        assert!(tunnel.is_connected().await);
    }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_error_on_send_when_disconnected() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("error-tunnel-1".to_string(), config);

        let result = tunnel.send(b"data").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_error_on_receive_when_disconnected() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("error-tunnel-2".to_string(), config);

        let result = tunnel.receive().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_data_send() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("error-tunnel-3".to_string(), config);

        tunnel.connect().await.expect("Connection failed");
        
        // Should handle empty data
        let result = tunnel.send(b"").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_large_data_send() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("error-tunnel-4".to_string(), config);

        tunnel.connect().await.expect("Connection failed");
        
        // 1MB of data
        let large_data = vec![0u8; 1024 * 1024];
        let result = tunnel.send(&large_data).await;
        assert!(result.is_ok());

        let stats = tunnel.stats().await;
        assert_eq!(stats.bytes_sent, 1024 * 1024);
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
    async fn test_connection_performance() {
        let config = TunnelConfig::default();
        
        let start = Instant::now();
        for i in 0..100 {
            let tunnel = Tunnel::new(format!("perf-tunnel-{}", i), config.clone());
            tunnel.connect().await.expect("Connection failed");
        }
        let duration = start.elapsed();

        println!("100 connections: {:?}", duration);
        // Should complete in reasonable time
    }

    #[tokio::test]
    async fn test_data_transfer_performance() {
        let config = TunnelConfig::default();
        let tunnel = Tunnel::new("perf-tunnel-data".to_string(), config);

        tunnel.connect().await.expect("Connection failed");

        let data = vec![0u8; 1024]; // 1KB packet
        let start = Instant::now();
        
        for _ in 0..1000 {
            tunnel.send(&data).await.expect("Send failed");
        }
        
        let duration = start.elapsed();
        println!("1000 packets (1KB each): {:?}", duration);
    }

    #[tokio::test]
    async fn test_concurrent_tunnel_operations() {
        let config = TunnelConfig::default();
        let tunnel = Arc::new(Tunnel::new("perf-tunnel-concurrent".to_string(), config));

        tunnel.connect().await.expect("Connection failed");

        let start = Instant::now();
        let mut handles = vec![];

        for i in 0..50 {
            let tunnel_clone = tunnel.clone();
            let handle = tokio::spawn(async move {
                let data = vec![i as u8; 1024];
                for _ in 0..20 {
                    let _ = tunnel_clone.send(&data).await;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let duration = start.elapsed();
        println!("50 concurrent threads × 20 sends: {:?}", duration);
    }
}