//! # Server Module Comprehensive Tests
//!
//! Comprehensive tests for server infrastructure components including
//! RAM-only architecture, TEE, secure boot, Starlink FEC, WiFi 7 MLO,
//! FTTH jumbo frames, smart routing, and colocated infrastructure.

use super::*;
use crate::error::{VantisError, Result};
use std::time::Duration;

#[cfg(test)]
mod server_status_tests {
    use super::*;

    #[test]
    fn test_server_status_equality() {
        assert_eq!(ServerStatus::Online, ServerStatus::Online);
        assert_eq!(ServerStatus::Offline, ServerStatus::Offline);
        assert_ne!(ServerStatus::Online, ServerStatus::Offline);
    }

    #[test]
    fn test_server_status_transitions() {
        let mut status = ServerStatus::Offline;
        
        status = ServerStatus::Starting;
        assert_eq!(status, ServerStatus::Starting);
        
        status = ServerStatus::Online;
        assert_eq!(status, ServerStatus::Online);
        
        status = ServerStatus::Maintenance;
        assert_eq!(status, ServerStatus::Maintenance);
        
        status = ServerStatus::Degraded;
        assert_eq!(status, ServerStatus::Degraded);
    }

    #[test]
    fn test_server_status_debug() {
        let status = ServerStatus::Online;
        assert!(format!("{:?}", status).contains("Online"));
    }

    #[test]
    fn test_server_status_serialization() {
        let status = ServerStatus::Online;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ServerStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }
}

#[cfg(test)]
mod server_location_tests {
    use super::*;

    #[test]
    fn test_server_location_creation() {
        let location = ServerLocation::new(
            "United States".to_string(),
            "New York".to_string(),
            "NY".to_string(),
            40.7128,
            -74.0060,
        );

        assert_eq!(location.country, "United States");
        assert_eq!(location.city, "New York");
        assert_eq!(location.latitude, 40.7128);
        assert_eq!(location.longitude, -74.0060);
    }

    #[test]
    fn test_server_location_distance() {
        let ny = ServerLocation::new(
            "United States".to_string(),
            "New York".to_string(),
            "NY".to_string(),
            40.7128,
            -74.0060,
        );

        let la = ServerLocation::new(
            "United States".to_string(),
            "Los Angeles".to_string(),
            "CA".to_string(),
            34.0522,
            -118.2437,
        );

        let distance = ny.distance_to(&la);
        // Distance between NY and LA is approximately 3935 km
        assert!(distance > 3900.0 && distance < 4000.0);
    }

    #[test]
    fn test_server_location_serialization() {
        let location = ServerLocation::new(
            "Germany".to_string(),
            "Frankfurt".to_string(),
            "Hesse".to_string(),
            50.1109,
            8.6821,
        );

        let json = serde_json::to_string(&location).unwrap();
        let deserialized: ServerLocation = serde_json::from_str(&json).unwrap();
        
        assert_eq!(location.country, deserialized.country);
        assert_eq!(location.city, deserialized.city);
    }
}

#[cfg(test)]
mod server_capabilities_tests {
    use super::*;

    #[test]
    fn test_server_capabilities_default() {
        let capabilities = ServerCapabilities::default();
        
        assert_eq!(capabilities.max_connections, 10000);
        assert_eq!(capabilities.bandwidth_mbps, 10000);
        assert!(capabilities.supports_pqc);
        assert!(capabilities.supports_stealth);
        assert!(capabilities.supports_wireguard);
    }

    #[test]
    fn test_server_capabilities_custom() {
        let capabilities = ServerCapabilities {
            max_connections: 5000,
            bandwidth_mbps: 5000,
            supports_pqc: false,
            supports_stealth: true,
            supports_multihop: false,
            supports_wireguard: true,
            supports_quic: false,
        };

        assert_eq!(capabilities.max_connections, 5000);
        assert!(!capabilities.supports_pqc);
        assert!(!capabilities.supports_multihop);
    }
}

#[cfg(test)]
mod vpn_server_tests {
    use super::*;

    #[test]
    fn test_vpn_server_creation() {
        let server = VpnServer {
            id: "server-001".to_string(),
            hostname: "vpn-nyc-01.vantis.com".to_string(),
            ip_address: "192.168.1.1".to_string(),
            location: ServerLocation::new(
                "United States".to_string(),
                "New York".to_string(),
                "NY".to_string(),
                40.7128,
                -74.0060,
            ),
            status: ServerStatus::Online,
            capabilities: ServerCapabilities::default(),
            load: 0.5,
            current_connections: 500,
        };

        assert_eq!(server.id, "server-001");
        assert_eq!(server.status, ServerStatus::Online);
        assert_eq!(server.load, 0.5);
    }
}

#[cfg(test)]
mod load_balancing_tests {
    use super::*;

    #[test]
    fn test_load_balancing_strategy_display() {
        assert_eq!(format!("{}", LoadBalancingStrategy::RoundRobin), "RoundRobin");
        assert_eq!(format!("{}", LoadBalancingStrategy::LeastConnections), "LeastConnections");
        assert_eq!(format!("{}", LoadBalancingStrategy::LeastLatency), "LeastLatency");
        assert_eq!(format!("{}", LoadBalancingStrategy::Geographic), "Geographic");
    }

    #[test]
    fn test_load_balancing_strategy_serialization() {
        let strategy = LoadBalancingStrategy::LeastConnections;
        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: LoadBalancingStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(strategy, deserialized);
    }
}

#[cfg(test)]
mod ram_only_tests {
    use super::*;

    #[test]
    fn test_ram_only_config_default() {
        let config = RamOnlyConfig::default();
        
        assert_eq!(config.max_memory_mb, 8192);
        assert_eq!(config.session_timeout_secs, 3600);
        assert!(config.enable_memory_monitoring);
        assert!(config.enable_auto_cleanup);
        assert_eq!(config.cleanup_interval_secs, 300);
    }

    #[test]
    fn test_ram_only_config_custom() {
        let config = RamOnlyConfig {
            max_memory_mb: 16384,
            session_timeout_secs: 7200,
            enable_memory_monitoring: false,
            enable_auto_cleanup: true,
            cleanup_interval_secs: 600,
        };

        assert_eq!(config.max_memory_mb, 16384);
        assert_eq!(config.session_timeout_secs, 7200);
        assert!(!config.enable_memory_monitoring);
    }

    #[test]
    fn test_ram_only_config_serialization() {
        let config = RamOnlyConfig {
            max_memory_mb: 4096,
            session_timeout_secs: 1800,
            enable_memory_monitoring: true,
            enable_auto_cleanup: false,
            cleanup_interval_secs: 150,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: RamOnlyConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.max_memory_mb, deserialized.max_memory_mb);
        assert_eq!(config.session_timeout_secs, deserialized.session_timeout_secs);
    }

    #[test]
    fn test_session_data_creation() {
        let session = SessionData::new(
            "session-001".to_string(),
            "user-001".to_string(),
        );

        assert_eq!(session.session_id, "session-001");
        assert_eq!(session.user_id, "user-001");
        assert_eq!(session.bytes_sent, 0);
        assert_eq!(session.bytes_received, 0);
    }

    #[test]
    fn test_memory_stats_default() {
        let stats = MemoryStats::default();
        
        assert_eq!(stats.used_memory_mb, 0);
        assert_eq!(stats.total_memory_mb, 0);
        assert_eq!(stats.active_sessions, 0);
    }
}

#[cfg(test)]
mod tee_tests {
    use super::*;

    #[test]
    fn test_tee_type_equality() {
        assert_eq!(TeeType::IntelSGX, TeeType::IntelSGX);
        assert_eq!(TeeType::AmdSEV, TeeType::AmdSEV);
        assert_eq!(TeeType::ArmTrustZone, TeeType::ArmTrustZone);
        assert_eq!(TeeType::SoftwareTEE, TeeType::SoftwareTEE);
    }

    #[test]
    fn test_tee_type_name() {
        assert_eq!(TeeType::IntelSGX.name(), "Intel SGX");
        assert_eq!(TeeType::AmdSEV.name(), "AMD SEV");
        assert_eq!(TeeType::ArmTrustZone.name(), "ARM TrustZone");
        assert_eq!(TeeType::SoftwareTEE.name(), "Software TEE");
    }

    #[test]
    fn test_tee_type_is_hardware_backed() {
        assert!(TeeType::IntelSGX.is_hardware_backed());
        assert!(TeeType::AmdSEV.is_hardware_backed());
        assert!(TeeType::ArmTrustZone.is_hardware_backed());
        assert!(!TeeType::SoftwareTEE.is_hardware_backed());
    }

    #[test]
    fn test_tee_config_default() {
        let config = TeeConfig::default();
        
        assert_eq!(config.tee_type, TeeType::SoftwareTEE);
        assert!(config.enable_attestation);
        assert!(config.enable_secure_key_storage);
        assert!(config.enable_memory_encryption);
        assert_eq!(config.max_enclave_size_mb, 1024);
        assert_eq!(config.attestation_timeout_secs, 30);
    }

    #[test]
    fn test_tee_config_custom() {
        let config = TeeConfig {
            tee_type: TeeType::IntelSGX,
            enable_attestation: true,
            enable_secure_key_storage: true,
            enable_memory_encryption: false,
            max_enclave_size_mb: 2048,
            attestation_timeout_secs: 60,
        };

        assert_eq!(config.tee_type, TeeType::IntelSGX);
        assert_eq!(config.max_enclave_size_mb, 2048);
    }

    #[test]
    fn test_tee_type_serialization() {
        let tee_type = TeeType::AmdSEV;
        let json = serde_json::to_string(&tee_type).unwrap();
        let deserialized: TeeType = serde_json::from_str(&json).unwrap();
        assert_eq!(tee_type, deserialized);
    }

    #[test]
    fn test_tee_config_serialization() {
        let config = TeeConfig {
            tee_type: TeeType::ArmTrustZone,
            enable_attestation: false,
            enable_secure_key_storage: true,
            enable_memory_encryption: true,
            max_enclave_size_mb: 512,
            attestation_timeout_secs: 45,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TeeConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.tee_type, deserialized.tee_type);
        assert_eq!(config.enable_attestation, deserialized.enable_attestation);
    }
}

#[cfg(test)]
mod secure_boot_tests {
    use super::*;

    #[test]
    fn test_secure_boot_config_default() {
        let config = SecureBootConfig::default();
        
        assert!(config.enabled);
        assert!(config.verify_signatures);
        assert!(config.dbx_update_enabled);
    }

    #[test]
    fn test_boot_component_creation() {
        let component = BootComponent {
            name: "shim".to_string(),
            version: "15.6".to_string(),
            signature_valid: true,
            checksum: "sha256:abc123".to_string(),
        };

        assert_eq!(component.name, "shim");
        assert!(component.signature_valid);
    }

    #[test]
    fn test_boot_result_creation() {
        let result = BootResult {
            component: BootComponent {
                name: "kernel".to_string(),
                version: "6.1.0".to_string(),
                signature_valid: true,
                checksum: "sha256:xyz789".to_string(),
            },
            verified: true,
            timestamp: std::time::SystemTime::now(),
        };

        assert!(result.verified);
        assert!(result.component.signature_valid);
    }
}

#[cfg(test)]
mod fec_tests {
    use super::*;

    #[test]
    fn test_fec_config_default() {
        let config = FecConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.redundancy_percentage, 20);
    }

    #[test]
    fn test_fec_algorithm_display() {
        assert_eq!(format!("{}", FecAlgorithm::ReedSolomon), "ReedSolomon");
        assert_eq!(format!("{}", FecAlgorithm::Raptor), "Raptor");
        assert_eq!(format!("{}", FecAlgorithm::Ldpc), "Ldpc");
    }

    #[test]
    fn test_fec_stats_default() {
        let stats = FecStats::default();
        
        assert_eq!(stats.packets_received, 0);
        assert_eq!(stats.packets_recovered, 0);
        assert_eq!(stats.recovery_success_rate, 0.0);
    }
}

#[cfg(test)]
mod wifi7_mlo_tests {
    use super::*;

    #[test]
    fn test_mlo_config_default() {
        let config = MloConfig::default();
        
        assert!(config.enabled);
        assert!(config.simultaneous_links);
    }

    #[test]
    fn test_wifi_band_display() {
        assert_eq!(format!("{}", WifiBand::Band2_4Ghz), "2.4GHz");
        assert_eq!(format!("{}", WifiBand::Band5Ghz), "5GHz");
        assert_eq!(format!("{}", WifiBand::Band6Ghz), "6GHz");
    }

    #[test]
    fn test_mlo_stats_default() {
        let stats = MloStats::default();
        
        assert_eq!(stats.active_links, 0);
        assert_eq!(stats.total_throughput_mbps, 0);
    }
}

#[cfg(test)]
mod ftth_jumbo_tests {
    use super::*;

    #[test]
    fn test_jumbo_frame_config_default() {
        let config = JumboFrameConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.mtu, 9000);
    }

    #[test]
    fn test_frame_type_display() {
        assert_eq!(format!("{}", FrameType::Standard), "Standard");
        assert_eq!(format!("{}", FrameType::Jumbo), "Jumbo");
        assert_eq!(format!("{}", FrameType::SuperJumbo), "SuperJumbo");
    }

    #[test]
    fn test_jumbo_frame_stats_default() {
        let stats = JumboFrameStats::default();
        
        assert_eq!(stats.frames_sent, 0);
        assert_eq!(stats.frames_received, 0);
        assert_eq!(stats.fragmentation_count, 0);
    }
}

#[cfg(test)]
mod smart_routing_tests {
    use super::*;

    #[test]
    fn test_smart_routing_config_default() {
        let config = SmartRoutingConfig::default();
        
        assert!(config.enabled);
        assert!(config.adaptive);
    }

    #[test]
    fn test_routing_metric_display() {
        assert_eq!(format!("{}", RoutingMetric::Latency), "Latency");
        assert_eq!(format!("{}", RoutingMetric::Bandwidth), "Bandwidth");
        assert_eq!(format!("{}", RoutingMetric::Cost), "Cost");
        assert_eq!(format!("{}", RoutingMetric::Reliability), "Reliability");
    }

    #[test]
    fn test_routing_decision_creation() {
        let decision = RoutingDecision {
            selected_path: RoutingPath::Direct,
            reason: "Lowest latency".to_string(),
            confidence: 0.95,
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(decision.selected_path, RoutingPath::Direct);
        assert_eq!(decision.confidence, 0.95);
    }

    #[test]
    fn test_routing_stats_default() {
        let stats = RoutingStats::default();
        
        assert_eq!(stats.decisions_made, 0);
        assert_eq!(stats.path_switches, 0);
    }
}

#[cfg(test)]
mod colocated_tests {
    use super::*;

    #[test]
    fn test_colocated_config_default() {
        let config = ColocatedConfig::default();
        
        assert!(config.enabled);
        assert!(config.auto_failover);
    }

    #[test]
    fn test_infrastructure_stats_default() {
        let stats = InfrastructureStats::default();
        
        assert_eq!(stats.total_servers, 0);
        assert_eq!(stats.active_servers, 0);
        assert_eq!(stats.total_connections, 0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_server_with_location_and_capabilities() {
        let server = VpnServer {
            id: "server-001".to_string(),
            hostname: "vpn-nyc-01.vantis.com".to_string(),
            ip_address: "10.0.0.1".to_string(),
            location: ServerLocation::new(
                "United States".to_string(),
                "New York".to_string(),
                "NY".to_string(),
                40.7128,
                -74.0060,
            ),
            status: ServerStatus::Online,
            capabilities: ServerCapabilities {
                max_connections: 15000,
                bandwidth_mbps: 20000,
                supports_pqc: true,
                supports_stealth: true,
                supports_multihop: true,
                supports_wireguard: true,
                supports_quic: true,
            },
            load: 0.25,
            current_connections: 1500,
        };

        assert_eq!(server.status, ServerStatus::Online);
        assert!(server.capabilities.supports_pqc);
        assert!(server.load < 1.0);
    }

    #[test]
    fn test_tee_with_secure_boot_integration() {
        let tee_config = TeeConfig {
            tee_type: TeeType::IntelSGX,
            enable_attestation: true,
            enable_secure_key_storage: true,
            enable_memory_encryption: true,
            max_enclave_size_mb: 2048,
            attestation_timeout_secs: 30,
        };

        let boot_config = SecureBootConfig::default();

        assert!(tee_config.enable_attestation);
        assert!(boot_config.enabled);
    }

    #[test]
    fn test_ram_only_with_tee_integration() {
        let ram_config = RamOnlyConfig {
            max_memory_mb: 16384,
            session_timeout_secs: 3600,
            enable_memory_monitoring: true,
            enable_auto_cleanup: true,
            cleanup_interval_secs: 300,
        };

        let tee_config = TeeConfig {
            tee_type: TeeType::AmdSEV,
            enable_memory_encryption: true,
            ..Default::default()
        };

        // Both should have memory protection features
        assert!(ram_config.enable_memory_monitoring);
        assert!(tee_config.enable_memory_encryption);
    }

    #[test]
    fn test_smart_routing_with_load_balancing() {
        let routing_config = SmartRoutingConfig {
            enabled: true,
            adaptive: true,
            metrics: vec![RoutingMetric::Latency, RoutingMetric::Bandwidth],
        };

        let lb_strategy = LoadBalancingStrategy::LeastLatency;

        assert!(routing_config.enabled);
        assert!(routing_config.metrics.contains(&RoutingMetric::Latency));
        assert_eq!(lb_strategy, LoadBalancingStrategy::LeastLatency);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_server_location_distance_performance() {
        let ny = ServerLocation::new(
            "United States".to_string(),
            "New York".to_string(),
            "NY".to_string(),
            40.7128,
            -74.0060,
        );

        let la = ServerLocation::new(
            "United States".to_string(),
            "Los Angeles".to_string(),
            "CA".to_string(),
            34.0522,
            -118.2437,
        );

        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let _ = ny.distance_to(&la);
        }
        let duration = start.elapsed();

        // Should calculate 10000 distances in less than 10ms
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_server_status_serialization_performance() {
        let status = ServerStatus::Online;
        
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let _ = serde_json::to_string(&status).unwrap();
        }
        let duration = start.elapsed();

        // Should serialize 10000 statuses in less than 50ms
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_tee_config_serialization_performance() {
        let config = TeeConfig::default();
        
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = serde_json::to_string(&config).unwrap();
        }
        let duration = start.elapsed();

        // Should serialize 1000 configs in less than 100ms
        assert!(duration.as_millis() < 100);
    }
}