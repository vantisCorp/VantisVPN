//! # Server Module Comprehensive Tests
//!
//! Comprehensive tests for server infrastructure components including
//! RAM-only architecture, TEE, secure boot, Starlink FEC, WiFi 7 MLO,
//! FTTH jumbo frames, smart routing, and colocated infrastructure.

use super::*;
use crate::error::{Result, VantisError};
use crate::server::secure_boot::{ComponentType, IntegrityStatus};
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
            "United States".to_string(),
            "New York".to_string(),
            "NY".to_string(),
            40.7128,
            -74.0060,
        );

        let json = serde_json::to_string(&location).unwrap();
        let deserialized: ServerLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(location.country, deserialized.country);
        assert_eq!(location.city, deserialized.city);
    }
}

#[cfg(test)]
mod vpn_server_tests {
    use super::*;

    #[test]
    fn test_vpn_server_creation() {
        let server = VpnServer::new(
            "server-001".to_string(),
            "vpn-nyc-01.vantis.com".to_string(),
            "192.168.1.1".to_string(),
            ServerLocation::new(
                "United States".to_string(),
                "New York".to_string(),
                "NY".to_string(),
                40.7128,
                -74.0060,
            ),
        );

        assert_eq!(server.server_id, "server-001");
        assert_eq!(server.status, ServerStatus::Offline); // Default status
        assert_eq!(server.load_percentage, 0.0); // Default load
    }
}

#[cfg(test)]
mod load_balancing_tests {
    use super::*;

    #[test]
    fn test_load_balancing_strategy_variants() {
        // Test that all strategy variants can be created
        let _rr = LoadBalancingStrategy::RoundRobin;
        let _lc = LoadBalancingStrategy::LeastConnections;
        let _geo = LoadBalancingStrategy::Geographic;
        let _w = LoadBalancingStrategy::Weighted;
        let _r = LoadBalancingStrategy::Random;

        // Note: LoadBalancingStrategy doesn't implement Display
        // This test verifies all variants are accessible
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
        assert_eq!(
            config.session_timeout_secs,
            deserialized.session_timeout_secs
        );
    }
}

#[cfg(test)]
mod tee_tests {
    use super::*;

    #[test]
    fn test_tee_config_default() {
        let config = TeeConfig::default();

        assert!(config.enable_attestation);
        assert!(config.enable_memory_encryption);
        assert_eq!(config.attestation_timeout_secs, 30);
    }

    #[test]
    fn test_tee_config_serialization() {
        let config = TeeConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TeeConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enable_attestation, deserialized.enable_attestation);
    }
}

#[cfg(test)]
mod secure_boot_tests {
    use super::*;

    #[test]
    fn test_boot_component_creation() {
        let component = BootComponent::new(
            "shim-001".to_string(),
            ComponentType::Bootloader,
            "/boot/shim.efi".to_string(),
            vec![1u8; 32],
            1,
        );

        assert_eq!(component.component_id, "shim-001");
        assert_eq!(component.component_type, ComponentType::Bootloader);
        assert_eq!(component.load_order, 1);
    }

    #[test]
    fn test_boot_result_creation() {
        let result = BootResult {
            success: true,
            verified_count: 5,
            failed_count: 0,
            warnings: vec![],
            boot_time: 1500,
        };

        assert!(result.success);
        assert_eq!(result.verified_count, 5);
        assert_eq!(result.failed_count, 0);
    }
}

#[cfg(test)]
mod fec_tests {
    use super::*;

    #[test]
    fn test_fec_config_default() {
        let config = FecConfig::default();

        assert_eq!(config.algorithm, FecAlgorithm::Hybrid);
        assert_eq!(config.block_size, 1400);
        assert_eq!(config.parity_symbols, 4);
    }

    #[test]
    fn test_fec_algorithm_name() {
        assert_eq!(FecAlgorithm::ReedSolomon.name(), "Reed-Solomon");
        assert_eq!(FecAlgorithm::Ldpc.name(), "LDPC");
        assert_eq!(FecAlgorithm::Turbo.name(), "Turbo");
        assert_eq!(FecAlgorithm::Hybrid.name(), "Hybrid");
    }

    #[test]
    fn test_fec_algorithm_overhead() {
        // Verify overhead ratios are reasonable
        assert!(FecAlgorithm::ReedSolomon.overhead_ratio() > 0.0);
        assert!(FecAlgorithm::Ldpc.overhead_ratio() > 0.0);
    }
}

#[cfg(test)]
mod wifi7_mlo_tests {
    use super::*;

    #[test]
    fn test_mlo_config_default() {
        let config = MloConfig::default();

        assert!(config.enabled);
        assert_eq!(config.max_links, 3);
        assert!(config.enable_failover);
    }

    #[test]
    fn test_wifi_band_variants() {
        // Test that all band variants can be created
        let _band_24 = WifiBand::Band24GHz;
        let _band_5 = WifiBand::Band5GHz;
        let _band_6 = WifiBand::Band6GHz;
        let _band_60 = WifiBand::Band60GHz;
    }

    #[test]
    fn test_mlo_config_serialization() {
        let config = MloConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: MloConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.max_links, deserialized.max_links);
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
    fn test_frame_type_variants() {
        // Test that all frame type variants can be created
        let _standard = FrameType::Standard;
        let _jumbo = FrameType::Jumbo;
        let _super_jumbo = FrameType::SuperJumbo;
    }

    #[test]
    fn test_jumbo_frame_config_serialization() {
        let config = JumboFrameConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: JumboFrameConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.mtu, deserialized.mtu);
    }
}

#[cfg(test)]
mod smart_routing_tests {
    use super::*;

    #[test]
    fn test_smart_routing_config_default() {
        let config = SmartRoutingConfig::default();

        assert!(config.enable_ml);
        assert_eq!(config.primary_metric, RoutingMetric::Balanced);
    }

    #[test]
    fn test_routing_metric_variants() {
        // Test that all metric variants can be created
        let _latency = RoutingMetric::Latency;
        let _throughput = RoutingMetric::Throughput;
        let _cost = RoutingMetric::Cost;
        let _reliability = RoutingMetric::Reliability;
        let _balanced = RoutingMetric::Balanced;
    }

    #[test]
    fn test_routing_decision_creation() {
        let decision = RoutingDecision {
            path_id: "path-001".to_string(),
            metric: RoutingMetric::Latency,
            score: 0.95,
            confidence: 0.95,
            timestamp: 1234567890,
        };

        assert_eq!(decision.path_id, "path-001");
        assert_eq!(decision.confidence, 0.95);
    }

    #[test]
    fn test_smart_routing_config_serialization() {
        let config = SmartRoutingConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: SmartRoutingConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enable_ml, deserialized.enable_ml);
    }
}

#[cfg(test)]
mod colocated_tests {
    use super::*;

    #[test]
    fn test_colocated_config_default() {
        let config = ColocatedConfig::default();

        assert!(config.enable_failover);
        assert!(config.enable_geographic_routing);
    }

    #[test]
    fn test_colocated_config_serialization() {
        let config = ColocatedConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ColocatedConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enable_failover, deserialized.enable_failover);
    }
}

#[cfg(test)]
mod server_capabilities_tests {
    use super::*;

    #[test]
    fn test_server_capabilities_default() {
        let caps = ServerCapabilities::default();

        // Verify default capabilities are reasonable
        assert!(caps.max_connections > 0);
    }

    #[test]
    fn test_server_capabilities_serialization() {
        let caps = ServerCapabilities::default();
        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: ServerCapabilities = serde_json::from_str(&json).unwrap();

        assert_eq!(caps.max_connections, deserialized.max_connections);
    }
}
