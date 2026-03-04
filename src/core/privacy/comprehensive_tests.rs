//! # Privacy Module Comprehensive Tests
//!
//! Comprehensive tests for privacy features including IP rotation,
//! zero-knowledge login, and identity management.

use super::*;
use std::net::{IpAddr, Ipv4Addr};
use chrono::Utc;

// =============================================================================
// Rotation Strategy Tests
// =============================================================================

#[cfg(test)]
mod rotation_strategy_tests {
    use super::*;

    #[test]
    fn test_strategy_equality() {
        assert_eq!(RotationStrategy::PerConnection, RotationStrategy::PerConnection);
        assert_eq!(RotationStrategy::TimeInterval, RotationStrategy::TimeInterval);
        assert_ne!(RotationStrategy::PerConnection, RotationStrategy::TimeInterval);
    }

    #[test]
    fn test_strategy_debug() {
        let strategy = RotationStrategy::PerConnection;
        let debug = format!("{:?}", strategy);
        assert!(debug.contains("PerConnection"));
    }

    #[test]
    fn test_strategy_serialization() {
        let strategy = RotationStrategy::Geographic;
        let json = serde_json::to_string(&strategy).expect("Serialization failed");
        let decoded: RotationStrategy = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(strategy, decoded);
    }

    #[test]
    fn test_all_strategies() {
        let strategies = [
            RotationStrategy::PerConnection,
            RotationStrategy::TimeInterval,
            RotationStrategy::DataThreshold,
            RotationStrategy::Geographic,
            RotationStrategy::Adaptive,
        ];

        for strategy in &strategies {
            let json = serde_json::to_string(strategy).expect("Serialization failed");
            let decoded: RotationStrategy = serde_json::from_str(&json).expect("Deserialization failed");
            assert_eq!(*strategy, decoded);
        }
    }
}

// =============================================================================
// IP Endpoint Tests
// =============================================================================

#[cfg(test)]
mod ip_endpoint_tests {
    use super::*;

    fn create_test_endpoint() -> IpEndpoint {
        IpEndpoint {
            ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            port: 443,
            country_code: "US".to_string(),
            city: "New York".to_string(),
            isp: "Test ISP".to_string(),
            latency_ms: 50,
            load: 30,
            available: true,
            last_used: Utc::now(),
        }
    }

    #[test]
    fn test_endpoint_creation() {
        let endpoint = create_test_endpoint();
        assert_eq!(endpoint.port, 443);
        assert_eq!(endpoint.country_code, "US");
        assert!(endpoint.available);
    }

    #[test]
    fn test_endpoint_ipv6() {
        let endpoint = IpEndpoint {
            ip_address: IpAddr::V6("2001:db8::1".parse().unwrap()),
            port: 51820,
            country_code: "DE".to_string(),
            city: "Berlin".to_string(),
            isp: "German ISP".to_string(),
            latency_ms: 100,
            load: 50,
            available: true,
            last_used: Utc::now(),
        };

        assert!(endpoint.ip_address.is_ipv6());
        assert_eq!(endpoint.country_code, "DE");
    }

    #[test]
    fn test_endpoint_clone() {
        let endpoint = create_test_endpoint();
        let cloned = endpoint.clone();
        assert_eq!(endpoint.port, cloned.port);
        assert_eq!(endpoint.country_code, cloned.country_code);
    }

    #[test]
    fn test_endpoint_debug() {
        let endpoint = create_test_endpoint();
        let debug = format!("{:?}", endpoint);
        assert!(debug.contains("IpEndpoint"));
    }

    #[test]
    fn test_endpoint_serialization() {
        let endpoint = create_test_endpoint();
        let json = serde_json::to_string(&endpoint).expect("Serialization failed");
        let decoded: IpEndpoint = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(endpoint.ip_address, decoded.ip_address);
    }

    #[test]
    fn test_endpoint_availability() {
        let mut endpoint = create_test_endpoint();
        assert!(endpoint.available);

        endpoint.available = false;
        assert!(!endpoint.available);
    }

    #[test]
    fn test_endpoint_load_values() {
        let mut endpoint = create_test_endpoint();

        endpoint.load = 0;
        assert_eq!(endpoint.load, 0);

        endpoint.load = 100;
        assert_eq!(endpoint.load, 100);
    }

    #[test]
    fn test_endpoint_latency() {
        let mut endpoint = create_test_endpoint();

        endpoint.latency_ms = 1;
        assert_eq!(endpoint.latency_ms, 1);

        endpoint.latency_ms = 500;
        assert_eq!(endpoint.latency_ms, 500);
    }
}

// =============================================================================
// IP Pool Tests
// =============================================================================

#[cfg(test)]
mod ip_pool_tests {
    use super::*;

    fn create_test_pool() -> IpPool {
        let endpoints = vec![
            IpEndpoint {
                ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
                port: 443,
                country_code: "US".to_string(),
                city: "New York".to_string(),
                isp: "ISP1".to_string(),
                latency_ms: 50,
                load: 30,
                available: true,
                last_used: Utc::now(),
            },
            IpEndpoint {
                ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
                port: 443,
                country_code: "DE".to_string(),
                city: "Berlin".to_string(),
                isp: "ISP2".to_string(),
                latency_ms: 80,
                load: 50,
                available: true,
                last_used: Utc::now(),
            },
        ];

        IpPool {
            pool_id: "test_pool".to_string(),
            name: "Test Pool".to_string(),
            endpoints,
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_pool_creation() {
        let pool = create_test_pool();
        assert_eq!(pool.pool_id, "test_pool");
        assert_eq!(pool.name, "Test Pool");
        assert_eq!(pool.endpoints.len(), 2);
    }

    #[test]
    fn test_pool_empty() {
        let pool = IpPool {
            pool_id: "empty_pool".to_string(),
            name: "Empty Pool".to_string(),
            endpoints: vec![],
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        };

        assert!(pool.endpoints.is_empty());
    }

    #[test]
    fn test_pool_clone() {
        let pool = create_test_pool();
        let cloned = pool.clone();
        assert_eq!(pool.pool_id, cloned.pool_id);
        assert_eq!(pool.endpoints.len(), cloned.endpoints.len());
    }

    #[test]
    fn test_pool_multiple_countries() {
        let pool = create_test_pool();
        let countries: std::collections::HashSet<_> = pool.endpoints
            .iter()
            .map(|e| e.country_code.as_str())
            .collect();

        assert!(countries.contains("US"));
        assert!(countries.contains("DE"));
    }

    #[test]
    fn test_pool_rotation_count() {
        let mut pool = create_test_pool();
        assert_eq!(pool.total_rotations, 0);

        pool.total_rotations = 10;
        assert_eq!(pool.total_rotations, 10);
    }
}

// =============================================================================
// Rotator Config Tests
// =============================================================================

#[cfg(test)]
mod rotator_config_tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = RotatorConfig::default();
        assert_eq!(config.strategy, RotationStrategy::TimeInterval);
        assert_eq!(config.rotation_interval, 300);
        assert_eq!(config.data_threshold, 1024 * 1024 * 1024);
        assert!(config.enable_geo_diversity);
    }

    #[test]
    fn test_config_custom() {
        let config = RotatorConfig {
            strategy: RotationStrategy::Geographic,
            rotation_interval: 600,
            data_threshold: 2 * 1024 * 1024 * 1024,
            min_latency_ms: 50,
            max_load: 70,
            enable_geo_diversity: false,
            min_countries: 5,
            adaptive_threshold: 0.8,
        };

        assert_eq!(config.strategy, RotationStrategy::Geographic);
        assert_eq!(config.rotation_interval, 600);
        assert!(!config.enable_geo_diversity);
    }

    #[test]
    fn test_config_clone() {
        let config = RotatorConfig::default();
        let cloned = config.clone();
        assert_eq!(config.strategy, cloned.strategy);
        assert_eq!(config.rotation_interval, cloned.rotation_interval);
    }

    #[test]
    fn test_config_debug() {
        let config = RotatorConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("RotatorConfig"));
    }

    #[test]
    fn test_all_strategies_config() {
        for strategy in [
            RotationStrategy::PerConnection,
            RotationStrategy::TimeInterval,
            RotationStrategy::DataThreshold,
            RotationStrategy::Geographic,
            RotationStrategy::Adaptive,
        ] {
            let config = RotatorConfig {
                strategy,
                ..Default::default()
            };
            assert_eq!(config.strategy, strategy);
        }
    }
}

// =============================================================================
// IP Rotator Integration Tests
// =============================================================================

#[cfg(test)]
mod ip_rotator_tests {
    use super::*;

    fn create_test_endpoint() -> IpEndpoint {
        IpEndpoint {
            ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            port: 443,
            country_code: "US".to_string(),
            city: "New York".to_string(),
            isp: "Test ISP".to_string(),
            latency_ms: 50,
            load: 30,
            available: true,
            last_used: Utc::now(),
        }
    }

    fn create_test_pool_with_endpoints() -> IpPool {
        let endpoints = vec![
            IpEndpoint {
                ip_address: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
                port: 443,
                country_code: "US".to_string(),
                city: "New York".to_string(),
                isp: "ISP1".to_string(),
                latency_ms: 30,
                load: 20,
                available: true,
                last_used: Utc::now(),
            },
            IpEndpoint {
                ip_address: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
                port: 443,
                country_code: "DE".to_string(),
                city: "Berlin".to_string(),
                isp: "ISP2".to_string(),
                latency_ms: 50,
                load: 40,
                available: true,
                last_used: Utc::now(),
            },
            IpEndpoint {
                ip_address: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 3)),
                port: 443,
                country_code: "JP".to_string(),
                city: "Tokyo".to_string(),
                isp: "ISP3".to_string(),
                latency_ms: 100,
                load: 60,
                available: true,
                last_used: Utc::now(),
            },
        ];

        IpPool {
            pool_id: "test_pool".to_string(),
            name: "Test Pool".to_string(),
            endpoints,
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_rotator_creation() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config);
        assert!(rotator.is_ok());
    }

    #[tokio::test]
    async fn test_add_pool() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");
        let pool = create_test_pool_with_endpoints();

        let result = rotator.add_pool(pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_pool() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");
        let pool = create_test_pool_with_endpoints();

        rotator.add_pool(pool).await.expect("Failed to add pool");
        let result = rotator.remove_pool("test_pool").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_pool() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");
        let pool = create_test_pool_with_endpoints();

        rotator.add_pool(pool).await.expect("Failed to add pool");
        let result = rotator.set_pool("test_pool").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_nonexistent_pool() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let result = rotator.set_pool("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rotate_without_pool() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let result = rotator.rotate().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rotate_with_pool() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");
        let pool = create_test_pool_with_endpoints();

        rotator.add_pool(pool).await.expect("Failed to add pool");
        rotator.set_pool("test_pool").await.expect("Failed to set pool");

        let result = rotator.rotate().await;
        assert!(result.is_ok());

        let endpoint = result.unwrap();
        assert!(endpoint.available);
    }

    #[tokio::test]
    async fn test_multiple_rotations() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");
        let pool = create_test_pool_with_endpoints();

        rotator.add_pool(pool).await.expect("Failed to add pool");
        rotator.set_pool("test_pool").await.expect("Failed to set pool");

        for _ in 0..5 {
            let result = rotator.rotate().await;
            assert!(result.is_ok());
        }

        let stats = rotator.get_stats().await;
        assert_eq!(stats.total_rotations, 5);
    }

    #[tokio::test]
    async fn test_get_current_endpoint() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");
        let pool = create_test_pool_with_endpoints();

        rotator.add_pool(pool).await.expect("Failed to add pool");
        rotator.set_pool("test_pool").await.expect("Failed to set pool");

        // Initially no endpoint
        let current = rotator.get_current_endpoint().await.expect("Failed to get endpoint");
        assert!(current.is_none());

        // After rotation
        rotator.rotate().await.expect("Failed to rotate");
        let current = rotator.get_current_endpoint().await.expect("Failed to get endpoint");
        assert!(current.is_some());
    }

    #[tokio::test]
    async fn test_record_transfer() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        rotator.record_transfer(1000).await;
        rotator.record_transfer(500).await;

        let stats = rotator.get_stats().await;
        assert_eq!(stats.data_transferred, 1500);
    }

    #[tokio::test]
    async fn test_should_rotate_initial() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        // Initially should rotate
        let should = rotator.should_rotate().await.expect("Failed to check");
        assert!(should);
    }

    #[tokio::test]
    async fn test_get_stats() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let stats = rotator.get_stats().await;
        assert_eq!(stats.total_rotations, 0);
        assert!(stats.current_ip.is_none());
    }

    #[tokio::test]
    async fn test_get_config() {
        let config = RotatorConfig {
            strategy: RotationStrategy::Geographic,
            ..Default::default()
        };
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let config = rotator.get_config();
        assert_eq!(config.strategy, RotationStrategy::Geographic);
    }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_rotate_without_pool_set() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let pool = IpPool {
            pool_id: "test".to_string(),
            name: "Test".to_string(),
            endpoints: vec![],
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        };

        rotator.add_pool(pool).await.expect("Failed to add pool");
        // Don't set pool

        let result = rotator.rotate().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rotate_empty_pool() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let pool = IpPool {
            pool_id: "empty".to_string(),
            name: "Empty".to_string(),
            endpoints: vec![],
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        };

        rotator.add_pool(pool).await.expect("Failed to add pool");
        rotator.set_pool("empty").await.expect("Failed to set pool");

        let result = rotator.rotate().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rotate_no_available_endpoints() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let endpoint = IpEndpoint {
            ip_address: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            port: 443,
            country_code: "US".to_string(),
            city: "Test".to_string(),
            isp: "ISP".to_string(),
            latency_ms: 50,
            load: 30,
            available: false, // Not available
            last_used: Utc::now(),
        };

        let pool = IpPool {
            pool_id: "unavailable".to_string(),
            name: "Unavailable".to_string(),
            endpoints: vec![endpoint],
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        };

        rotator.add_pool(pool).await.expect("Failed to add pool");
        rotator.set_pool("unavailable").await.expect("Failed to set pool");

        let result = rotator.rotate().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rotate_high_load_endpoints() {
        let config = RotatorConfig {
            max_load: 50,
            ..Default::default()
        };
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let endpoint = IpEndpoint {
            ip_address: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            port: 443,
            country_code: "US".to_string(),
            city: "Test".to_string(),
            isp: "ISP".to_string(),
            latency_ms: 50,
            load: 90, // Exceeds max_load
            available: true,
            last_used: Utc::now(),
        };

        let pool = IpPool {
            pool_id: "highload".to_string(),
            name: "High Load".to_string(),
            endpoints: vec![endpoint],
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        };

        rotator.add_pool(pool).await.expect("Failed to add pool");
        rotator.set_pool("highload").await.expect("Failed to set pool");

        let result = rotator.rotate().await;
        assert!(result.is_err());
    }
}

// =============================================================================
// Performance Tests
// =============================================================================

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    fn create_large_pool(size: usize) -> IpPool {
        let endpoints: Vec<IpEndpoint> = (0..size)
            .map(|i| IpEndpoint {
                ip_address: IpAddr::V4(Ipv4Addr::new(10, 0, (i / 256) as u8, (i % 256) as u8)),
                port: 443,
                country_code: format!("C{}", i % 10),
                city: format!("City {}", i),
                isp: format!("ISP {}", i),
                latency_ms: (i % 100 + 10) as u32,
                load: (i % 80) as u8,
                available: true,
                last_used: Utc::now(),
            })
            .collect();

        IpPool {
            pool_id: "large_pool".to_string(),
            name: "Large Pool".to_string(),
            endpoints,
            current_index: 0,
            total_rotations: 0,
            created_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_rotation_performance() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");
        let pool = create_large_pool(100);

        rotator.add_pool(pool).await.expect("Failed to add pool");
        rotator.set_pool("large_pool").await.expect("Failed to set pool");

        let start = Instant::now();
        for _ in 0..100 {
            let _ = rotator.rotate().await.expect("Rotation failed");
        }
        let duration = start.elapsed();

        println!("100 rotations with 100 endpoints: {:?}", duration);
    }

    #[tokio::test]
    async fn test_add_multiple_pools() {
        let config = RotatorConfig::default();
        let rotator = IpRotator::new(config).expect("Failed to create rotator");

        let start = Instant::now();
        for i in 0..50 {
            let pool = create_large_pool(10);
            let mut pool = pool;
            pool.pool_id = format!("pool_{}", i);

            rotator.add_pool(pool).await.expect("Failed to add pool");
        }
        let duration = start.elapsed();

        println!("50 pools added: {:?}", duration);
    }
}