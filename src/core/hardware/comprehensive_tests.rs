//! # Hardware Module Comprehensive Tests
//!
//! Comprehensive tests for hardware components including router OS,
//! YubiKey authentication, and Vantis OS firmware.

use super::*;
use crate::error::{VantisError, Result};
use std::net::Ipv4Addr;

#[cfg(test)]
mod router_os_tests {
    use super::*;

    #[test]
    fn test_router_config_creation() {
        let config = RouterConfig {
            router_id: "router-001".to_string(),
            hostname: "vantis-router".to_string(),
            admin_password_hash: "hash123".to_string(),
            firmware_version: "1.0.0".to_string(),
            hardware_model: "VantisRouter-1".to_string(),
            interfaces: vec![],
            firewall_rules: vec![],
            port_forwarding: vec![],
            qos_policies: vec![],
            vpn_config: VpnRouterConfig {
                enabled: true,
                server_address: "vpn.vantis.com".to_string(),
                port: 1194,
                cipher_suite: "chacha20-poly1305".to_string(),
                keepalive_interval: 25,
                dns_servers: vec![],
                kill_switch: true,
                split_tunneling: false,
                allowed_ips: vec![],
                protocol: "udp".to_string(),
            },
            wifi_config: None,
            lan_config: LanConfig {
                ip_address: Ipv4Addr::new(192, 168, 1, 1),
                subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                dhcp_enabled: true,
                dhcp_pool_start: Ipv4Addr::new(192, 168, 1, 100),
                dhcp_pool_end: Ipv4Addr::new(192, 168, 1, 200),
                dhcp_lease_time: 86400,
            },
            wan_config: WanConfig {
                connection_type: WanConnectionType::Dhcp,
                ip_address: None,
                subnet_mask: None,
                gateway: None,
                pppoe_username: None,
                pppoe_password: None,
                mtu: 1500,
                dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            },
            logging_enabled: true,
            remote_management: false,
            auto_update: true,
        };

        assert_eq!(config.router_id, "router-001");
        assert_eq!(config.hostname, "vantis-router");
        assert!(config.vpn_config.enabled);
    }

    #[test]
    fn test_network_interface_creation() {
        let interface = NetworkInterface {
            name: "eth0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: Some("192.168.1.1".to_string()),
            enabled: true,
            interface_type: InterfaceType::Ethernet,
        };

        assert_eq!(interface.name, "eth0");
        assert!(interface.enabled);
    }

    #[test]
    fn test_firewall_rule_creation() {
        let rule = FirewallRule {
            name: "block-external".to_string(),
            action: FirewallAction::Deny,
            protocol: Some(FirewallProtocol::Tcp),
            source_port: None,
            destination_port: Some(22),
            source_address: None,
            destination_address: None,
            enabled: true,
        };

        assert_eq!(rule.name, "block-external");
        assert_eq!(rule.action, FirewallAction::Deny);
    }

    #[test]
    fn test_qos_policy_creation() {
        let policy = QosPolicy {
            name: "video-streaming".to_string(),
            priority: QosPriority::High,
            min_bandwidth: Some(1000),
            max_bandwidth: Some(5000),
            enabled: true,
        };

        assert_eq!(policy.name, "video-streaming");
        assert_eq!(policy.priority, QosPriority::High);
    }

    #[test]
    fn test_router_state_transitions() {
        let mut state = RouterState::Initializing;
        
        state = RouterState::Running;
        assert_eq!(state, RouterState::Running);
        
        state = RouterState::Maintenance;
        assert_eq!(state, RouterState::Maintenance);
    }

    #[test]
    fn test_router_stats_tracking() {
        let mut stats = RouterStats::default();
        
        stats.total_connections = 100;
        stats.active_connections = 50;
        stats.uptime_seconds = 3600;
        
        assert_eq!(stats.total_connections, 100);
        assert_eq!(stats.active_connections, 50);
    }

    #[test]
    fn test_firewall_action_display() {
        assert_eq!(format!("{}", FirewallAction::Allow), "Allow");
        assert_eq!(format!("{}", FirewallAction::Deny), "Deny");
        assert_eq!(format!("{}", FirewallAction::Reject), "Reject");
    }

    #[test]
    fn test_wan_connection_type_display() {
        assert_eq!(format!("{}", WanConnectionType::Dhcp), "DHCP");
        assert_eq!(format!("{}", WanConnectionType::Static), "Static");
        assert_eq!(format!("{}", WanConnectionType::Pppoe), "PPPoE");
    }

    #[test]
    fn test_interface_type_display() {
        assert_eq!(format!("{}", InterfaceType::Ethernet), "Ethernet");
        assert_eq!(format!("{}", InterfaceType::Wifi), "WiFi");
        assert_eq!(format!("{}", InterfaceType::Vlan), "VLAN");
    }

    #[test]
    fn test_qos_priority_display() {
        assert_eq!(format!("{}", QosPriority::Low), "Low");
        assert_eq!(format!("{}", QosPriority::Medium), "Medium");
        assert_eq!(format!("{}", QosPriority::High), "High");
        assert_eq!(format!("{}", QosPriority::Critical), "Critical");
    }

    #[test]
    fn test_router_state_debug() {
        let state = RouterState::Running;
        assert!(format!("{:?}", state).contains("Running"));
    }

    #[test]
    fn test_router_config_serialization() {
        let config = RouterConfig {
            router_id: "router-001".to_string(),
            hostname: "vantis-router".to_string(),
            admin_password_hash: "hash123".to_string(),
            firmware_version: "1.0.0".to_string(),
            hardware_model: "VantisRouter-1".to_string(),
            interfaces: vec![],
            firewall_rules: vec![],
            port_forwarding: vec![],
            qos_policies: vec![],
            vpn_config: VpnRouterConfig {
                enabled: true,
                server_address: "vpn.vantis.com".to_string(),
                port: 1194,
                cipher_suite: "chacha20-poly1305".to_string(),
                keepalive_interval: 25,
                dns_servers: vec![],
                mtu: 1500,
                kill_switch: true,
                split_tunneling: false,
                allowed_ips: vec![],
                protocol: "udp".to_string(),
            },
            wifi_config: None,
            lan_config: LanConfig {
                ip_address: Ipv4Addr::new(192, 168, 1, 1),
                subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                dhcp_enabled: true,
                dhcp_pool_start: Ipv4Addr::new(192, 168, 1, 100),
                dhcp_pool_end: Ipv4Addr::new(192, 168, 1, 200),
                dhcp_lease_time: 86400,
            },
            wan_config: WanConfig {
                connection_type: WanConnectionType::Dhcp,
                ip_address: None,\n                mtu: 1500,
                subnet_mask: None,
                gateway: None,
                pppoe_username: None,
                pppoe_password: None,
                dns_servers: vec![],
                mtu: 1500,
            },
            logging_enabled: true,
            remote_management: false,
            auto_update: true,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: RouterConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.router_id, deserialized.router_id);
        assert_eq!(config.hostname, deserialized.hostname);
    }
}

#[cfg(test)]
mod yubikey_tests {
    use super::*;

    #[test]
    fn test_yubikey_config_creation() {
        let config = YubiKeyConfig {
            enabled: true,
            require_for_login: true,
            require_for_admin: true,
            require_for_vpn: false,
            allowed_slots: vec![YubiKeySlot::Slot1, YubiKeySlot::Slot2],
            challenge_timeout: Duration::from_secs(30),
            max_attempts: 3,
            lockout_duration: Duration::from_secs(300),
            backup_codes_enabled: true,
            backup_codes_count: 10,
        };

        assert!(config.enabled);
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.allowed_slots.len(), 2);
    }

    #[test]
    fn test_yubikey_slot_equality() {
        assert_eq!(YubiKeySlot::Slot1, YubiKeySlot::Slot1);
        assert_eq!(YubiKeySlot::Slot2, YubiKeySlot::Slot2);
        assert_ne!(YubiKeySlot::Slot1, YubiKeySlot::Slot2);
    }

    #[test]
    fn test_yubikey_auth_challenge_response() {
        let auth = YubiKeyAuth::ChallengeResponse {
            slot: YubiKeySlot::Slot1,
            challenge: vec![1, 2, 3, 4],
            response: vec![5, 6, 7, 8],
        };

        match auth {
            YubiKeyAuth::ChallengeResponse { slot, challenge, response } => {
                assert_eq!(slot, YubiKeySlot::Slot1);
                assert_eq!(challenge.len(), 4);
                assert_eq!(response.len(), 4);
            }
            _ => panic!("Expected ChallengeResponse"),
        }
    }

    #[test]
    fn test_yubikey_auth_hmac() {
        let auth = YubiKeyAuth::Hmac {
            slot: YubiKeySlot::Slot2,
            data: vec![1, 2, 3],
            hmac: vec![4, 5, 6],
        };

        match auth {
            YubiKeyAuth::Hmac { slot, data, hmac } => {
                assert_eq!(slot, YubiKeySlot::Slot2);
                assert_eq!(data.len(), 3);
                assert_eq!(hmac.len(), 3);
            }
            _ => panic!("Expected Hmac"),
        }
    }

    #[test]
    fn test_yubikey_auth_otp() {
        let auth = YubiKeyAuth::Otp {
            slot: YubiKeySlot::Slot1,
            otp: "123456".to_string(),
        };

        match auth {
            YubiKeyAuth::Otp { slot, otp } => {
                assert_eq!(slot, YubiKeySlot::Slot1);
                assert_eq!(otp, "123456");
            }
            _ => panic!("Expected Otp"),
        }
    }

    #[test]
    fn test_yubikey_slot_hash() {
        use std::collections::HashSet;
        
        let mut set = HashSet::new();
        set.insert(YubiKeySlot::Slot1);
        set.insert(YubiKeySlot::Slot2);
        
        assert_eq!(set.len(), 2);
        assert!(set.contains(&YubiKeySlot::Slot1));
        assert!(set.contains(&YubiKeySlot::Slot2));
    }

    #[test]
    fn test_yubikey_slot_display() {
        assert_eq!(format!("{}", YubiKeySlot::Slot1), "Slot1");
        assert_eq!(format!("{}", YubiKeySlot::Slot2), "Slot2");
    }

    #[test]
    fn test_yubikey_config_serialization() {
        let config = YubiKeyConfig {
            enabled: true,
            require_for_login: true,
            require_for_admin: false,
            require_for_vpn: false,
            allowed_slots: vec![YubiKeySlot::Slot1],
            challenge_timeout: Duration::from_secs(30),
            max_attempts: 5,
            lockout_duration: Duration::from_secs(600),
            backup_codes_enabled: false,
            backup_codes_count: 0,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: YubiKeyConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.max_attempts, deserialized.max_attempts);
    }

    #[test]
    fn test_yubikey_auth_serialization() {
        let auth = YubiKeyAuth::Otp {
            slot: YubiKeySlot::Slot1,
            otp: "123456".to_string(),
        };

        let json = serde_json::to_string(&auth).unwrap();
        let deserialized: YubiKeyAuth = serde_json::from_str(&json).unwrap();
        
        match deserialized {
            YubiKeyAuth::Otp { otp, .. } => {
                assert_eq!(otp, "123456");
            }
            _ => panic!("Expected Otp"),
        }
    }
}

#[cfg(test)]
mod vantis_os_tests {
    use super::*;

    #[test]
    fn test_vantis_os_config_creation() {
        let config = VantisOsConfig {
            os_name: "Vantis OS".to_string(),
            version: "1.0.0".to_string(),
            build_number: "100".to_string(),
            boot_config: BootConfig {
                boot_mode: BootMode::Persistent,
                secure_boot: true,
                boot_timeout: Duration::from_secs(5),
                default_boot_option: BootOption::Standard,
                kernel_parameters: vec![],
                initramfs_compression: "zstd".to_string(),
                bootloader: Bootloader::Grub,
            },
            persistence_config: PersistenceConfig {
                enabled: true,
                encrypted: true,
                size_gb: 8,
                mount_point: "/home".to_string(),
            },
            security_config: SecurityConfig {
                firewall_enabled: true,
                dns_blocking_enabled: true,
                https_only: true,
                microphone_disabled: true,
                webcam_disabled: true,
                persistence_password_hash: Some("hash".to_string()),
            },
            network_config: NetworkConfig {
                vpn_enabled: true,
                vpn_config: None,
                wifi_enabled: false,
                wifi_config: None,
                tor_enabled: false,
                bridge_mode: false,
            },
            applications: vec![],
            locale: "en_US.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
        };

        assert_eq!(config.os_name, "Vantis OS");
        assert_eq!(config.version, "1.0.0");
        assert!(config.boot_config.secure_boot);
    }

    #[test]
    fn test_boot_mode_display() {
        assert_eq!(format!("{}", BootMode::Live), "Live");
        assert_eq!(format!("{}", BootMode::Persistent), "Persistent");
        assert_eq!(format!("{}", BootMode::Encrypted), "Encrypted");
    }

    #[test]
    fn test_boot_option_display() {
        assert_eq!(format!("{}", BootOption::Standard), "Standard");
        assert_eq!(format!("{}", BootOption::FailSafe), "FailSafe");
        assert_eq!(format!("{}", BootOption::Recovery), "Recovery");
    }

    #[test]
    fn test_bootloader_display() {
        assert_eq!(format!("{}", Bootloader::Grub), "Grub");
        assert_eq!(format!("{}", Bootloader::Syslinux), "Syslinux");
        assert_eq!(format!("{}", Bootloader::SystemdBoot), "SystemdBoot");
    }

    #[test]
    fn test_boot_config_default() {
        let config = BootConfig::default();
        
        assert_eq!(config.boot_mode, BootMode::Live);
        assert!(!config.secure_boot);
        assert_eq!(config.default_boot_option, BootOption::Standard);
        assert_eq!(config.bootloader, Bootloader::Grub);
    }

    #[test]
    fn test_persistence_config_creation() {
        let config = PersistenceConfig {
            enabled: true,
            encrypted: true,
            size_gb: 16,
            mount_point: "/persistent".to_string(),
        };

        assert!(config.enabled);
        assert!(config.encrypted);
        assert_eq!(config.size_gb, 16);
    }

    #[test]
    fn test_security_config_creation() {
        let config = SecurityConfig {
            firewall_enabled: true,
            dns_blocking_enabled: true,
            https_only: true,
            microphone_disabled: true,
            webcam_disabled: true,
            persistence_password_hash: Some("password_hash".to_string()),
        };

        assert!(config.firewall_enabled);
        assert!(config.dns_blocking_enabled);
        assert!(config.https_only);
    }

    #[test]
    fn test_network_config_creation() {
        let config = NetworkConfig {
            vpn_enabled: true,
            vpn_config: None,
            wifi_enabled: false,
            wifi_config: None,
            tor_enabled: false,
            bridge_mode: false,
        };

        assert!(config.vpn_enabled);
        assert!(!config.wifi_enabled);
    }

    #[test]
    fn test_vantis_os_config_serialization() {
        let config = VantisOsConfig {
            os_name: "Vantis OS".to_string(),
            version: "1.0.0".to_string(),
            build_number: "100".to_string(),
            boot_config: BootConfig::default(),
            persistence_config: PersistenceConfig {
                enabled: false,
                encrypted: false,
                size_gb: 0,
                mount_point: "".to_string(),
            },
            security_config: SecurityConfig {
                firewall_enabled: false,
                dns_blocking_enabled: false,
                https_only: false,
                microphone_disabled: false,
                webcam_disabled: false,
                persistence_password_hash: None,
            },
            network_config: NetworkConfig {
                vpn_enabled: false,
                vpn_config: None,
                wifi_enabled: false,
                wifi_config: None,
                tor_enabled: false,
                bridge_mode: false,
            },
            applications: vec![],
            locale: "en_US".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: VantisOsConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.os_name, deserialized.os_name);
        assert_eq!(config.version, deserialized.version);
    }

    #[test]
    fn test_vantis_os_image_creation() {
        let image = VantisOsImage {
            version: "1.0.0".to_string(),
            build_number: "100".to_string(),
            created_at: SystemTime::now(),
            size_bytes: 2_147_483_648, // 2GB
            checksum: "sha256:abc123".to_string(),
            download_url: "https://download.vantis.com/v1.0.0.img".to_string(),
        };

        assert_eq!(image.version, "1.0.0");
        assert_eq!(image.size_bytes, 2_147_483_648);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_router_with_vpn_integration() {
        let config = RouterConfig {
            router_id: "router-001".to_string(),
            hostname: "vantis-router".to_string(),
            admin_password_hash: "hash123".to_string(),
            firmware_version: "1.0.0".to_string(),
            hardware_model: "VantisRouter-1".to_string(),
            interfaces: vec![],
            firewall_rules: vec![
                FirewallRule {
                    name: "allow-vpn".to_string(),
                    action: FirewallAction::Allow,
                    protocol: Some(FirewallProtocol::Udp),
                    source_port: None,
                    destination_port: Some(1194),
                    source_address: None,
                    destination_address: None,
                    enabled: true,
                }
            ],
            port_forwarding: vec![],
            qos_policies: vec![],
            vpn_config: VpnRouterConfig {
                enabled: true,
                server_address: "vpn.vantis.com".to_string(),
                port: 1194,
                cipher_suite: "chacha20-poly1305".to_string(),
                keepalive_interval: 25,
                dns_servers: vec![],
                mtu: 1500,
                kill_switch: true,
                split_tunneling: false,
                allowed_ips: vec![],
                protocol: "udp".to_string(),
            },
            wifi_config: None,
            lan_config: LanConfig {
                ip_address: Ipv4Addr::new(192, 168, 1, 1),
                subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                dhcp_enabled: true,
                dhcp_pool_start: Ipv4Addr::new(192, 168, 1, 100),
                dhcp_pool_end: Ipv4Addr::new(192, 168, 1, 200),
                dhcp_lease_time: 86400,
            },
            wan_config: WanConfig {
                connection_type: WanConnectionType::Dhcp,
                ip_address: None,
                subnet_mask: None,
                gateway: None,
                pppoe_username: None,
                pppoe_password: None,
                dns_servers: vec!["8.8.8.8".to_string()],
                mtu: 1500,
            },
            logging_enabled: true,
            remote_management: false,
            auto_update: true,
        };

        assert!(config.vpn_config.enabled);
        assert!(!config.firewall_rules.is_empty());
        assert_eq!(config.firewall_rules[0].destination_port, Some(1194));
    }

    #[test]
    fn test_yubikey_with_security_integration() {
        let yubikey_config = YubiKeyConfig {
            enabled: true,
            require_for_login: true,
            require_for_admin: true,
            require_for_vpn: true,
            allowed_slots: vec![YubiKeySlot::Slot1],
            challenge_timeout: Duration::from_secs(30),
            max_attempts: 3,
            lockout_duration: Duration::from_secs(300),
            backup_codes_enabled: true,
            backup_codes_count: 10,
        };

        assert!(yubikey_config.require_for_login);
        assert!(yubikey_config.require_for_admin);
        assert!(yubikey_config.require_for_vpn);
    }

    #[test]
    fn test_vantis_os_with_network_integration() {
        let config = VantisOsConfig {
            os_name: "Vantis OS".to_string(),
            version: "1.0.0".to_string(),
            build_number: "100".to_string(),
            boot_config: BootConfig {
                boot_mode: BootMode::Encrypted,
                secure_boot: true,
                boot_timeout: Duration::from_secs(5),
                default_boot_option: BootOption::Standard,
                kernel_parameters: vec!["quiet".to_string()],
                initramfs_compression: "zstd".to_string(),
                bootloader: Bootloader::Grub,
            },
            persistence_config: PersistenceConfig {
                enabled: true,
                encrypted: true,
                size_gb: 16,
                mount_point: "/home".to_string(),
            },
            security_config: SecurityConfig {
                firewall_enabled: true,
                dns_blocking_enabled: true,
                https_only: true,
                microphone_disabled: true,
                webcam_disabled: true,
                persistence_password_hash: Some("secure_hash".to_string()),
            },
            network_config: NetworkConfig {
                vpn_enabled: true,
                vpn_config: None,
                wifi_enabled: false,
                wifi_config: None,
                tor_enabled: true,
                bridge_mode: false,
            },
            applications: vec![],
            locale: "en_US.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
        };

        assert!(config.security_config.firewall_enabled);
        assert!(config.network_config.vpn_enabled);
        assert!(config.network_config.tor_enabled);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_router_config_serialization_performance() {
        let config = RouterConfig {
            router_id: "router-001".to_string(),
            hostname: "vantis-router".to_string(),
            admin_password_hash: "hash123".to_string(),
            firmware_version: "1.0.0".to_string(),
            hardware_model: "VantisRouter-1".to_string(),
            interfaces: vec![],
            firewall_rules: vec![],
            port_forwarding: vec![],
            qos_policies: vec![],
            vpn_config: VpnRouterConfig {
                enabled: true,
                server_address: "vpn.vantis.com".to_string(),
                port: 1194,
                cipher_suite: "chacha20-poly1305".to_string(),
                keepalive_interval: 25,
                dns_servers: vec![],
                mtu: 1500,
                kill_switch: true,
                split_tunneling: false,
                allowed_ips: vec![],
                protocol: "udp".to_string(),
            },
            wifi_config: None,
            lan_config: LanConfig {
                ip_address: Ipv4Addr::new(192, 168, 1, 1),
                subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                dhcp_enabled: true,
                dhcp_pool_start: Ipv4Addr::new(192, 168, 1, 100),
                dhcp_pool_end: Ipv4Addr::new(192, 168, 1, 200),
                dhcp_lease_time: 86400,
            },
            wan_config: WanConfig {
                connection_type: WanConnectionType::Dhcp,
                ip_address: None,
                subnet_mask: None,
                gateway: None,
                pppoe_username: None,
                pppoe_password: None,
                dns_servers: vec![],
                mtu: 1500,
            },
            logging_enabled: true,
            remote_management: false,
            auto_update: true,
        };

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = serde_json::to_string(&config).unwrap();
        }
        let duration = start.elapsed();
        
        // Should serialize 1000 configs in less than 100ms
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_yubikey_slot_hash_performance() {
        use std::collections::HashSet;
        
        let start = std::time::Instant::now();
        let mut set = HashSet::new();
        for i in 0..10000 {
            let slot = if i % 2 == 0 { YubiKeySlot::Slot1 } else { YubiKeySlot::Slot2 };
            set.insert(slot);
        }
        let duration = start.elapsed();
        
        // Should handle 10000 insertions in less than 10ms
        assert!(duration.as_millis() < 10);
        assert_eq!(set.len(), 2);
    }
}