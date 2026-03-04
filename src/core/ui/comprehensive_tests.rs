//! # UI Module Comprehensive Tests
//!
//! Comprehensive tests for UI components including devtunnel,
//! family shield, biometric authentication, and theme manager.

use super::*;
use crate::error::{VantisError, Result};
use chrono::{DateTime, Utc};

#[cfg(test)]
mod theme_manager_tests {
    use super::*;

    #[test]
    fn test_theme_mode_equality() {
        assert_eq!(ThemeMode::Light, ThemeMode::Light);
        assert_eq!(ThemeMode::Dark, ThemeMode::Dark);
        assert_eq!(ThemeMode::Auto, ThemeMode::Auto);
        assert_ne!(ThemeMode::Light, ThemeMode::Dark);
    }

    #[test]
    fn test_theme_mode_display() {
        assert_eq!(format!("{}", ThemeMode::Light), "Light");
        assert_eq!(format!("{}", ThemeMode::Dark), "Dark");
        assert_eq!(format!("{}", ThemeMode::Auto), "Auto");
    }

    #[test]
    fn test_theme_mode_serialization() {
        let mode = ThemeMode::Dark;
        let json = serde_json::to_string(&mode).unwrap();
        let deserialized: ThemeMode = serde_json::from_str(&json).unwrap();
        assert_eq!(mode, deserialized);
    }

    #[test]
    fn test_haptic_type_equality() {
        assert_eq!(HapticType::Light, HapticType::Light);
        assert_eq!(HapticType::Medium, HapticType::Medium);
        assert_eq!(HapticType::Heavy, HapticType::Heavy);
        assert_eq!(HapticType::Success, HapticType::Success);
        assert_eq!(HapticType::Error, HapticType::Error);
        assert_eq!(HapticType::Warning, HapticType::Warning);
        assert_eq!(HapticType::Custom, HapticType::Custom);
    }

    #[test]
    fn test_haptic_type_display() {
        assert_eq!(format!("{}", HapticType::Light), "Light");
        assert_eq!(format!("{}", HapticType::Medium), "Medium");
        assert_eq!(format!("{}", HapticType::Heavy), "Heavy");
        assert_eq!(format!("{}", HapticType::Success), "Success");
        assert_eq!(format!("{}", HapticType::Error), "Error");
        assert_eq!(format!("{}", HapticType::Warning), "Warning");
        assert_eq!(format!("{}", HapticType::Custom), "Custom");
    }

    #[test]
    fn test_haptic_type_serialization() {
        let haptic_type = HapticType::Success;
        let json = serde_json::to_string(&haptic_type).unwrap();
        let deserialized: HapticType = serde_json::from_str(&json).unwrap();
        assert_eq!(haptic_type, deserialized);
    }

    #[test]
    fn test_haptic_pattern_creation() {
        let pattern = HapticPattern {
            pattern_id: "pattern-001".to_string(),
            name: "Custom Tap".to_string(),
            durations: vec![50, 100, 50],
            intensities: vec![0.5, 0.8, 0.5],
        };

        assert_eq!(pattern.pattern_id, "pattern-001");
        assert_eq!(pattern.durations.len(), 3);
        assert_eq!(pattern.intensities.len(), 3);
    }

    #[test]
    fn test_haptic_pattern_serialization() {
        let pattern = HapticPattern {
            pattern_id: "pattern-002".to_string(),
            name: "Double Tap".to_string(),
            durations: vec![100, 100],
            intensities: vec![0.7, 0.7],
        };

        let json = serde_json::to_string(&pattern).unwrap();
        let deserialized: HapticPattern = serde_json::from_str(&json).unwrap();
        
        assert_eq!(pattern.pattern_id, deserialized.pattern_id);
        assert_eq!(pattern.durations, deserialized.durations);
    }

    #[test]
    fn test_theme_config_default() {
        let config = ThemeConfig::default();
        
        assert_eq!(config.theme_mode, ThemeMode::Auto);
        assert!(config.enable_haptics);
    }
}

#[cfg(test)]
mod biometric_auth_tests {
    use super::*;

    #[test]
    fn test_biometric_type_equality() {
        assert_eq!(BiometricType::Fingerprint, BiometricType::Fingerprint);
        assert_eq!(BiometricType::Face, BiometricType::Face);
        assert_eq!(BiometricType::Iris, BiometricType::Iris);
        assert_eq!(BiometricType::Voice, BiometricType::Voice);
        assert_eq!(BiometricType::Palm, BiometricType::Palm);
    }

    #[test]
    fn test_biometric_type_serialization() {
        let biometric_type = BiometricType::Face;
        let json = serde_json::to_string(&biometric_type).unwrap();
        let deserialized: BiometricType = serde_json::from_str(&json).unwrap();
        assert_eq!(biometric_type, deserialized);
    }

    #[test]
    fn test_auth_result_creation() {
        let result = AuthResult {
            success: true,
            confidence: 0.95,
            method: BiometricType::Fingerprint,
            timestamp: Utc::now(),
            error_message: None,
        };

        assert!(result.success);
        assert_eq!(result.confidence, 0.95);
        assert_eq!(result.method, BiometricType::Fingerprint);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_auth_result_failure() {
        let result = AuthResult {
            success: false,
            confidence: 0.45,
            method: BiometricType::Face,
            timestamp: Utc::now(),
            error_message: Some("Face recognition failed".to_string()),
        };

        assert!(!result.success);
        assert_eq!(result.confidence, 0.45);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_auth_result_serialization() {
        let result = AuthResult {
            success: true,
            confidence: 0.92,
            method: BiometricType::Iris,
            timestamp: Utc::now(),
            error_message: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: AuthResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(result.success, deserialized.success);
        assert_eq!(result.confidence, deserialized.confidence);
    }

    #[test]
    fn test_biometric_template_creation() {
        let template = BiometricTemplate {
            template_id: "template-001".to_string(),
            user_id: "user-001".to_string(),
            biometric_type: BiometricType::Fingerprint,
            template_data: vec![1, 2, 3, 4, 5],
            created_at: Utc::now(),
            last_used: None,
            is_active: true,
        };

        assert_eq!(template.template_id, "template-001");
        assert_eq!(template.user_id, "user-001");
        assert!(template.is_active);
        assert!(template.last_used.is_none());
    }

    #[test]
    fn test_biometric_template_serialization() {
        let template = BiometricTemplate {
            template_id: "template-002".to_string(),
            user_id: "user-002".to_string(),
            biometric_type: BiometricType::Face,
            template_data: vec![10, 20, 30],
            created_at: Utc::now(),
            last_used: Some(Utc::now()),
            is_active: true,
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: BiometricTemplate = serde_json::from_str(&json).unwrap();
        
        assert_eq!(template.template_id, deserialized.template_id);
        assert_eq!(template.user_id, deserialized.user_id);
    }

    #[test]
    fn test_biometric_config_default() {
        let config = BiometricConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.confidence_threshold, 0.85);
        assert_eq!(config.max_failed_attempts, 5);
        assert!(!config.enable_multi_factor);
    }

    #[test]
    fn test_biometric_config_custom() {
        let config = BiometricConfig {
            enabled: true,
            confidence_threshold: 0.90,
            max_failed_attempts: 3,
            lockout_duration: 600,
            enable_multi_factor: true,
            required_types: vec![BiometricType::Fingerprint, BiometricType::Face],
            enable_password_fallback: false,
        };

        assert_eq!(config.confidence_threshold, 0.90);
        assert_eq!(config.max_failed_attempts, 3);
        assert!(config.enable_multi_factor);
        assert_eq!(config.required_types.len(), 2);
    }
}

#[cfg(test)]
mod devtunnel_tests {
    use super::*;

    #[test]
    fn test_tunnel_protocol_equality() {
        assert_eq!(TunnelProtocol::Ssh, TunnelProtocol::Ssh);
        assert_eq!(TunnelProtocol::Http, TunnelProtocol::Http);
        assert_eq!(TunnelProtocol::Https, TunnelProtocol::Https);
        assert_eq!(TunnelProtocol::WebSocket, TunnelProtocol::WebSocket);
        assert_eq!(TunnelProtocol::Custom, TunnelProtocol::Custom);
    }

    #[test]
    fn test_tunnel_protocol_display() {
        assert_eq!(format!("{}", TunnelProtocol::Ssh), "Ssh");
        assert_eq!(format!("{}", TunnelProtocol::Http), "Http");
        assert_eq!(format!("{}", TunnelProtocol::Https), "Https");
        assert_eq!(format!("{}", TunnelProtocol::WebSocket), "WebSocket");
        assert_eq!(format!("{}", TunnelProtocol::Custom), "Custom");
    }

    #[test]
    fn test_tunnel_protocol_serialization() {
        let protocol = TunnelProtocol::Https;
        let json = serde_json::to_string(&protocol).unwrap();
        let deserialized: TunnelProtocol = serde_json::from_str(&json).unwrap();
        assert_eq!(protocol, deserialized);
    }

    #[test]
    fn test_tunnel_status_transitions() {
        let mut status = TunnelStatus::NotStarted;
        
        status = TunnelStatus::Connecting;
        assert_eq!(status, TunnelStatus::Connecting);
        
        status = TunnelStatus::Active;
        assert_eq!(status, TunnelStatus::Active);
        
        status = TunnelStatus::Paused;
        assert_eq!(status, TunnelStatus::Paused);
        
        status = TunnelStatus::Closed;
        assert_eq!(status, TunnelStatus::Closed);
        
        status = TunnelStatus::Failed;
        assert_eq!(status, TunnelStatus::Failed);
    }

    #[test]
    fn test_tunnel_status_serialization() {
        let status = TunnelStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: TunnelStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_tunnel_config_default() {
        let config = TunnelConfig::default();
        
        assert_eq!(config.local_port, 8080);
        assert_eq!(config.remote_host, "localhost");
        assert_eq!(config.remote_port, 80);
        assert_eq!(config.protocol, TunnelProtocol::Https);
        assert!(config.enable_compression);
        assert!(config.enable_encryption);
        assert_eq!(config.timeout_secs, 30);
        assert!(config.auto_reconnect);
        assert_eq!(config.max_reconnect_attempts, 3);
    }

    #[test]
    fn test_tunnel_config_custom() {
        let config = TunnelConfig {
            local_port: 3000,
            remote_host: "example.com".to_string(),
            remote_port: 443,
            protocol: TunnelProtocol::Ssh,
            enable_compression: false,
            enable_encryption: true,
            timeout_secs: 60,
            auto_reconnect: false,
            max_reconnect_attempts: 5,
        };

        assert_eq!(config.local_port, 3000);
        assert_eq!(config.remote_host, "example.com");
        assert_eq!(config.protocol, TunnelProtocol::Ssh);
        assert!(!config.enable_compression);
    }

    #[test]
    fn test_tunnel_session_creation() {
        let session = TunnelSession {
            session_id: "session-001".to_string(),
            tunnel_id: "tunnel-001".to_string(),
            status: TunnelStatus::Active,
            local_address: "127.0.0.1:8080".to_string(),
            remote_address: "example.com:443".to_string(),
            started_at: Utc::now(),
            bytes_sent: 1024,
            bytes_received: 2048,
        };

        assert_eq!(session.session_id, "session-001");
        assert_eq!(session.status, TunnelStatus::Active);
        assert_eq!(session.bytes_sent, 1024);
        assert_eq!(session.bytes_received, 2048);
    }

    #[test]
    fn test_tunnel_session_serialization() {
        let session = TunnelSession {
            session_id: "session-002".to_string(),
            tunnel_id: "tunnel-002".to_string(),
            status: TunnelStatus::Active,
            local_address: "127.0.0.1:3000".to_string(),
            remote_address: "test.com:443".to_string(),
            started_at: Utc::now(),
            bytes_sent: 0,
            bytes_received: 0,
        };

        let json = serde_json::to_string(&session).unwrap();
        let deserialized: TunnelSession = serde_json::from_str(&json).unwrap();
        
        assert_eq!(session.session_id, deserialized.session_id);
        assert_eq!(session.tunnel_id, deserialized.tunnel_id);
    }

    #[test]
    fn test_tunnel_stats_default() {
        let stats = TunnelStats::default();
        
        assert_eq!(stats.total_sessions, 0);
        assert_eq!(stats.active_sessions, 0);
        assert_eq!(stats.total_bytes_transferred, 0);
    }
}

#[cfg(test)]
mod family_shield_tests {
    use super::*;

    #[test]
    fn test_shield_category_equality() {
        assert_eq!(ShieldCategory::Adult, ShieldCategory::Adult);
        assert_eq!(ShieldCategory::Gambling, ShieldCategory::Gambling);
        assert_eq!(ShieldCategory::Violence, ShieldCategory::Violence);
        assert_eq!(ShieldCategory::Drugs, ShieldCategory::Drugs);
        assert_eq!(ShieldCategory::SocialMedia, ShieldCategory::SocialMedia);
        assert_eq!(ShieldCategory::Streaming, ShieldCategory::Streaming);
        assert_eq!(ShieldCategory::Gaming, ShieldCategory::Gaming);
        assert_eq!(ShieldCategory::Malware, ShieldCategory::Malware);
        assert_eq!(ShieldCategory::Phishing, ShieldCategory::Phishing);
        assert_eq!(ShieldCategory::Custom, ShieldCategory::Custom);
    }

    #[test]
    fn test_shield_category_hash() {
        use std::collections::HashSet;
        
        let mut set = HashSet::new();
        set.insert(ShieldCategory::Adult);
        set.insert(ShieldCategory::Gambling);
        set.insert(ShieldCategory::Violence);
        
        assert_eq!(set.len(), 3);
        assert!(set.contains(&ShieldCategory::Adult));
        assert!(set.contains(&ShieldCategory::Gambling));
        assert!(set.contains(&ShieldCategory::Violence));
    }

    #[test]
    fn test_shield_category_serialization() {
        let category = ShieldCategory::Malware;
        let json = serde_json::to_string(&category).unwrap();
        let deserialized: ShieldCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(category, deserialized);
    }

    #[test]
    fn test_shield_action_equality() {
        assert_eq!(ShieldAction::Block, ShieldAction::Block);
        assert_eq!(ShieldAction::Allow, ShieldAction::Allow);
        assert_eq!(ShieldAction::Redirect, ShieldAction::Redirect);
        assert_eq!(ShieldAction::Warn, ShieldAction::Warn);
    }

    #[test]
    fn test_shield_action_display() {
        assert_eq!(format!("{}", ShieldAction::Block), "Block");
        assert_eq!(format!("{}", ShieldAction::Allow), "Allow");
        assert_eq!(format!("{}", ShieldAction::Redirect), "Redirect");
        assert_eq!(format!("{}", ShieldAction::Warn), "Warn");
    }

    #[test]
    fn test_shield_action_serialization() {
        let action = ShieldAction::Block;
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: ShieldAction = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }

    #[test]
    fn test_shield_rule_creation() {
        let rule = ShieldRule {
            rule_id: "rule-001".to_string(),
            domain_pattern: "*.malware.com".to_string(),
            category: ShieldCategory::Malware,
            action: ShieldAction::Block,
            enabled: true,
            priority: 100,
            description: Some("Block all malware domains".to_string()),
        };

        assert_eq!(rule.rule_id, "rule-001");
        assert_eq!(rule.domain_pattern, "*.malware.com");
        assert_eq!(rule.category, ShieldCategory::Malware);
        assert_eq!(rule.action, ShieldAction::Block);
        assert!(rule.enabled);
        assert_eq!(rule.priority, 100);
    }

    #[test]
    fn test_shield_rule_serialization() {
        let rule = ShieldRule {
            rule_id: "rule-002".to_string(),
            domain_pattern: "adult-site.com".to_string(),
            category: ShieldCategory::Adult,
            action: ShieldAction::Block,
            enabled: true,
            priority: 50,
            description: None,
        };

        let json = serde_json::to_string(&rule).unwrap();
        let deserialized: ShieldRule = serde_json::from_str(&json).unwrap();
        
        assert_eq!(rule.rule_id, deserialized.rule_id);
        assert_eq!(rule.domain_pattern, deserialized.domain_pattern);
    }

    #[test]
    fn test_shield_config_default() {
        let config = ShieldConfig::default();
        
        assert!(config.enabled);
        assert!(!config.block_all_adult);
        assert!(config.block_malware);
        assert!(config.block_phishing);
    }

    #[test]
    fn test_shield_config_custom() {
        let config = ShieldConfig {
            enabled: true,
            block_all_adult: true,
            block_gambling: true,
            block_violence: false,
            block_drugs: false,
            block_social_media: false,
            block_streaming: false,
            block_gaming: false,
            block_malware: true,
            block_phishing: true,
            custom_rules: vec![],
            safe_search_enabled: true,
        };

        assert!(config.enabled);
        assert!(config.block_all_adult);
        assert!(config.block_gambling);
        assert!(config.block_malware);
        assert!(config.block_phishing);
    }

    #[test]
    fn test_shield_stats_default() {
        let stats = ShieldStats::default();
        
        assert_eq!(stats.queries_processed, 0);
        assert_eq!(stats.queries_blocked, 0);
        assert_eq!(stats.queries_allowed, 0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_biometric_with_theme_integration() {
        let biometric_config = BiometricConfig {
            enabled: true,
            confidence_threshold: 0.90,
            max_failed_attempts: 3,
            lockout_duration: 600,
            enable_multi_factor: false,
            required_types: vec![],
            enable_password_fallback: true,
        };

        let theme_config = ThemeConfig {
            theme_mode: ThemeMode::Dark,
            enable_haptics: true,
            default_haptic_type: HapticType::Light,
        };

        assert!(biometric_config.enabled);
        assert_eq!(theme_config.theme_mode, ThemeMode::Dark);
    }

    #[test]
    fn test_devtunnel_with_biometric_integration() {
        let tunnel_config = TunnelConfig {
            local_port: 8080,
            remote_host: "secure.dev.com".to_string(),
            remote_port: 443,
            protocol: TunnelProtocol::Https,
            enable_compression: true,
            enable_encryption: true,
            timeout_secs: 30,
            auto_reconnect: true,
            max_reconnect_attempts: 3,
        };

        let biometric_auth = BiometricAuth::new(
            BiometricConfig::default(),
            BiometricType::Fingerprint,
        );

        // Both should have security features enabled
        assert!(tunnel_config.enable_encryption);
        assert!(biometric_auth.config.enabled);
    }

    #[test]
    fn test_family_shield_with_theme_integration() {
        let shield_config = ShieldConfig {
            enabled: true,
            block_all_adult: true,
            block_gambling: true,
            block_violence: false,
            block_drugs: false,
            block_social_media: false,
            block_streaming: false,
            block_gaming: false,
            block_malware: true,
            block_phishing: true,
            custom_rules: vec![],
            safe_search_enabled: true,
        };

        let theme_config = ThemeConfig::default();

        assert!(shield_config.enabled);
        assert!(theme_config.enable_haptics);
    }

    #[test]
    fn test_complete_ui_integration() {
        let biometric_config = BiometricConfig::default();
        let theme_config = ThemeConfig::default();
        let tunnel_config = TunnelConfig::default();
        let shield_config = ShieldConfig::default();

        // All UI components should be enabled by default
        assert!(biometric_config.enabled);
        assert!(theme_config.enable_haptics);
        assert!(tunnel_config.enable_encryption);
        assert!(shield_config.enabled);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_biometric_type_serialization_performance() {
        let biometric_type = BiometricType::Fingerprint;
        
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let _ = serde_json::to_string(&biometric_type).unwrap();
        }
        let duration = start.elapsed();

        // Should serialize 10000 types in less than 50ms
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_shield_category_hash_performance() {
        use std::collections::HashSet;
        
        let start = std::time::Instant::now();
        let mut set = HashSet::new();
        for i in 0..10000 {
            let category = match i % 10 {
                0 => ShieldCategory::Adult,
                1 => ShieldCategory::Gambling,
                2 => ShieldCategory::Violence,
                3 => ShieldCategory::Drugs,
                4 => ShieldCategory::SocialMedia,
                5 => ShieldCategory::Streaming,
                6 => ShieldCategory::Gaming,
                7 => ShieldCategory::Malware,
                8 => ShieldCategory::Phishing,
                _ => ShieldCategory::Custom,
            };
            set.insert(category);
        }
        let duration = start.elapsed();

        // Should handle 10000 insertions in less than 10ms
        assert!(duration.as_millis() < 10);
        assert_eq!(set.len(), 10);
    }

    #[test]
    fn test_tunnel_protocol_serialization_performance() {
        let protocol = TunnelProtocol::Https;
        
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let _ = serde_json::to_string(&protocol).unwrap();
        }
        let duration = start.elapsed();

        // Should serialize 10000 protocols in less than 50ms
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_auth_result_serialization_performance() {
        let result = AuthResult {
            success: true,
            confidence: 0.95,
            method: BiometricType::Face,
            timestamp: Utc::now(),
            error_message: None,
        };
        
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = serde_json::to_string(&result).unwrap();
        }
        let duration = start.elapsed();

        // Should serialize 1000 results in less than 100ms
        assert!(duration.as_millis() < 100);
    }
}