//! # UI Module Comprehensive Tests
//!
//! Comprehensive tests for UI components including devtunnel,
//! family shield, biometric authentication, and theme manager.

use super::*;
use chrono::Utc;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_auth_result_serialization() {
        let result = AuthResult {
            success: true,
            confidence: 0.95,
            method: BiometricType::Fingerprint,
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
            template_data: vec![1, 2, 3, 4],
            created_at: Utc::now(),
            last_used: Some(Utc::now()),
            is_active: true,
        };

        assert_eq!(template.template_id, "template-001");
        assert_eq!(template.user_id, "user-001");
        assert!(template.is_active);
    }

    #[test]
    fn test_biometric_template_serialization() {
        let template = BiometricTemplate {
            template_id: "template-002".to_string(),
            user_id: "user-002".to_string(),
            biometric_type: BiometricType::Face,
            template_data: vec![5, 6, 7, 8],
            created_at: Utc::now(),
            last_used: Some(Utc::now()),
            is_active: true,
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: BiometricTemplate = serde_json::from_str(&json).unwrap();

        assert_eq!(template.template_id, deserialized.template_id);
        assert_eq!(template.user_id, deserialized.user_id);
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
        let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let remote_addr: SocketAddr =
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 443);

        let session = TunnelSession {
            session_id: "session-001".to_string(),
            tunnel_id: "tunnel-001".to_string(),
            local_address: local_addr,
            remote_address: remote_addr,
            status: TunnelStatus::Active,
            bytes_transferred: 3072,
            started_at: Utc::now(),
            last_activity: Utc::now(),
        };

        assert_eq!(session.session_id, "session-001");
        assert_eq!(session.status, TunnelStatus::Active);
        assert_eq!(session.bytes_transferred, 3072);
    }

    #[test]
    fn test_tunnel_session_serialization() {
        let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
        let remote_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), 443);

        let session = TunnelSession {
            session_id: "session-002".to_string(),
            tunnel_id: "tunnel-002".to_string(),
            local_address: local_addr,
            remote_address: remote_addr,
            status: TunnelStatus::Active,
            bytes_transferred: 0,
            started_at: Utc::now(),
            last_activity: Utc::now(),
        };

        let json = serde_json::to_string(&session).unwrap();
        let deserialized: TunnelSession = serde_json::from_str(&json).unwrap();

        assert_eq!(session.session_id, deserialized.session_id);
        assert_eq!(session.tunnel_id, deserialized.tunnel_id);
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
    fn test_shield_action_serialization() {
        let action = ShieldAction::Block;
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: ShieldAction = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }

    #[test]
    fn test_shield_rule_creation() {
        use chrono::Utc;

        let rule = ShieldRule {
            rule_id: "rule-001".to_string(),
            domain_pattern: "*.example.com".to_string(),
            category: ShieldCategory::Adult,
            action: ShieldAction::Block,
            priority: 100,
            enabled: true,
            created_at: Utc::now(),
        };

        assert_eq!(rule.rule_id, "rule-001");
        assert_eq!(rule.domain_pattern, "*.example.com");
        assert_eq!(rule.category, ShieldCategory::Adult);
    }
}
