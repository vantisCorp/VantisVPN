//! # Hardware Module Comprehensive Tests
//!
//! Comprehensive tests for hardware components including router OS,
//! YubiKey authentication, and Vantis OS firmware.

use super::*;
use crate::error::{VantisError, Result};
use std::net::Ipv4Addr;
use std::time::{Duration, SystemTime};

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
        use std::net::IpAddr;
        
        let interface = NetworkInterface {
            name: "eth0".to_string(),
            interface_type: InterfaceType::Ethernet,
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_addresses: vec!["192.168.1.1".parse().unwrap()],
            mtu: 1500,
            enabled: true,
            is_up: true,
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
        };

        assert_eq!(interface.name, "eth0");
        assert!(interface.enabled);
    }

    #[test]
    fn test_firewall_rule_creation() {
        let rule = FirewallRule {
            id: "rule-001".to_string(),
            name: "block-external".to_string(),
            action: FirewallAction::Deny,
            direction: FirewallDirection::Inbound,
            protocol: Some("tcp".to_string()),
            source_ip: None,
            source_port: None,
            destination_ip: None,
            destination_port: Some(22),
            enabled: true,
            priority: 100,
            log: false,
        };

        assert_eq!(rule.name, "block-external");
        assert_eq!(rule.action, FirewallAction::Deny);
    }

    #[test]
    fn test_qos_policy_creation() {
        let policy = QosPolicy {
            id: "qos-001".to_string(),
            name: "video-streaming".to_string(),
            priority: QosPriority::High,
            bandwidth_limit: Some(5000),
            guaranteed_bandwidth: Some(1000),
            protocol: None,
            source_ip: None,
            destination_ip: None,
            enabled: true,
        };

        assert_eq!(policy.name, "video-streaming");
        assert_eq!(policy.priority, QosPriority::High);
    }

    #[test]
    fn test_router_state_struct() {
        // RouterState is a struct, not an enum
        let state = RouterState {
            uptime: Duration::from_secs(3600),
            cpu_usage: 25.5,
            memory_usage: 40.0,
            temperature: 45.0,
            vpn_connected: true,
            vpn_uptime: Some(Duration::from_secs(1800)),
            active_connections: 10,
            last_update: SystemTime::now(),
        };

        assert_eq!(state.cpu_usage, 25.5);
        assert!(state.vpn_connected);
    }

    #[test]
    fn test_router_stats_struct() {
        let stats = RouterStats {
            total_bytes_sent: 1000000,
            total_bytes_received: 2000000,
            total_packets_sent: 10000,
            total_packets_received: 20000,
            vpn_bytes_sent: 500000,
            vpn_bytes_received: 600000,
            connection_count: 50,
            blocked_connections: 5,
            uptime: Duration::from_secs(3600),
            reboot_count: 2,
        };
        
        assert_eq!(stats.connection_count, 50);
        assert_eq!(stats.blocked_connections, 5);
    }

    #[test]
    fn test_firewall_action_variants() {
        // Test that all variants exist
        let _accept = FirewallAction::Accept;
        let _allow = FirewallAction::Allow;
        let _drop = FirewallAction::Drop;
        let _deny = FirewallAction::Deny;
        let _reject = FirewallAction::Reject;
        let _log = FirewallAction::Log;
    }

    #[test]
    fn test_wan_connection_type_variants() {
        // Test that all variants exist
        let _dhcp = WanConnectionType::Dhcp;
        let _static_ip = WanConnectionType::Static;
        let _pppoe = WanConnectionType::Pppoe;
    }

    #[test]
    fn test_interface_type_variants() {
        // Test that all variants exist
        let _ethernet = InterfaceType::Ethernet;
        let _wifi = InterfaceType::Wifi;
        let _vpn = InterfaceType::Vpn;
        let _bridge = InterfaceType::Bridge;
        let _vlan = InterfaceType::Vlan;
    }

    #[test]
    fn test_qos_priority_variants() {
        // Test that all variants exist
        let _low = QosPriority::Low;
        let _medium = QosPriority::Medium;
        let _high = QosPriority::High;
        let _critical = QosPriority::Critical;
    }

    #[test]
    fn test_router_state_debug() {
        let state = RouterState {
            uptime: Duration::from_secs(3600),
            cpu_usage: 25.5,
            memory_usage: 40.0,
            temperature: 45.0,
            vpn_connected: true,
            vpn_uptime: None,
            active_connections: 10,
            last_update: SystemTime::now(),
        };
        assert!(format!("{:?}", state).contains("RouterState"));
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
                dns_servers: vec![], kill_switch: true,
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
        // YubiKeyAuth::Otp only has otp field, not slot
        let auth = YubiKeyAuth::Otp {
            otp: "123456".to_string(),
        };

        match auth {
            YubiKeyAuth::Otp { otp } => {
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
    fn test_yubikey_slot_variants() {
        // Test that all slot variants exist
        let _slot1 = YubiKeySlot::Slot1;
        let _slot2 = YubiKeySlot::Slot2;
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
            otp: "123456".to_string(),
        };

        let json = serde_json::to_string(&auth).unwrap();
        let deserialized: YubiKeyAuth = serde_json::from_str(&json).unwrap();
        
        match deserialized {
            YubiKeyAuth::Otp { otp } => {
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
                encryption_enabled: true,
                encryption_algorithm: "aes-256-gcm".to_string(),
                key_derivation: "argon2".to_string(),
                persistence_size: 8589934592, // 8GB in bytes
                persistence_location: "/home".to_string(),
                auto_mount: true,
                hidden_volume: false,
                plausible_deniability: false,
            },
            security_config: SecurityConfig {
                memory_wipe_on_shutdown: true,
                disable_swap: true,
                disable_hibernation: true,
                firewall_enabled: true,
                network_isolation: false,
                mac_address_spoofing: false,
                dns_over_https: true,
                tor_enabled: false,
                vpn_enabled: true,
                kill_switch_enabled: true,
                secure_delete: true,
                disable_usb_storage: false,
                disable_bluetooth: true,
                disable_webcam: true,
                disable_microphone: true,
                screen_lock_timeout: Duration::from_secs(300),
                auto_logout_timeout: Duration::from_secs(600),
            },
            network_config: NetworkConfig {
                tor_config: TorConfig {
                    enabled: false,
                    bridge_mode: false,
                    bridges: vec![],
                    obfs4_enabled: false,
                    meek_enabled: false,
                    snowflake_enabled: false,
                    circuit_isolation: false,
                    exit_node_country: None,
                },
                vpn_config: VpnOsConfig {
                    enabled: true,
                    provider: "VantisVPN".to_string(),
                    server_address: "vpn.vantis.com".to_string(),
                    port: 1194,
                    protocol: "udp".to_string(),
                    cipher_suite: "chacha20-poly1305".to_string(),
                    auto_connect: true,
                    kill_switch: true,
                },
                dns_servers: vec!["8.8.8.8".to_string()],
                proxy_config: None,
                network_manager: NetworkManager::NetworkManager,
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
    fn test_boot_mode_variants() {
        // Test that all variants exist
        let _live = BootMode::Live;
        let _persistent = BootMode::Persistent;
        let _encrypted = BootMode::Encrypted;
    }

    #[test]
    fn test_boot_option_variants() {
        // Test that all variants exist
        let _live = BootOption::LiveMode;
        let _persistent = BootOption::PersistentMode;
        let _encrypted = BootOption::EncryptedMode;
        let _diagnostic = BootOption::DiagnosticMode;
        let _standard = BootOption::Standard;
    }

    #[test]
    fn test_bootloader_variants() {
        // Test that all variants exist
        let _grub = Bootloader::Grub;
        let _syslinux = Bootloader::Syslinux;
        let _systemd = Bootloader::SystemdBoot;
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
            encryption_enabled: true,
            encryption_algorithm: "aes-256-gcm".to_string(),
            key_derivation: "argon2".to_string(),
            persistence_size: 17179869184, // 16GB in bytes
            persistence_location: "/persistent".to_string(),
            auto_mount: true,
            hidden_volume: false,
            plausible_deniability: false,
        };

        assert!(config.enabled);
        assert!(config.encryption_enabled);
        assert_eq!(config.persistence_size, 17179869184);
    }

    #[test]
    fn test_security_config_creation() {
        let config = SecurityConfig {
            memory_wipe_on_shutdown: true,
            disable_swap: true,
            disable_hibernation: true,
            firewall_enabled: true,
            network_isolation: false,
            mac_address_spoofing: false,
            dns_over_https: true,
            tor_enabled: false,
            vpn_enabled: true,
            kill_switch_enabled: true,
            secure_delete: true,
            disable_usb_storage: false,
            disable_bluetooth: true,
            disable_webcam: true,
            disable_microphone: true,
            screen_lock_timeout: Duration::from_secs(300),
            auto_logout_timeout: Duration::from_secs(600),
        };

        assert!(config.firewall_enabled);
        assert!(config.dns_over_https);
        assert!(config.vpn_enabled);
    }

    #[test]
    fn test_network_config_creation() {
        let config = NetworkConfig {
            tor_config: TorConfig {
                enabled: false,
                bridge_mode: false,
                bridges: vec![],
                obfs4_enabled: false,
                meek_enabled: false,
                snowflake_enabled: false,
                circuit_isolation: false,
                exit_node_country: None,
            },
            vpn_config: VpnOsConfig {
                enabled: true,
                provider: "VantisVPN".to_string(),
                server_address: "vpn.vantis.com".to_string(),
                port: 1194,
                protocol: "udp".to_string(),
                cipher_suite: "chacha20-poly1305".to_string(),
                auto_connect: true,
                kill_switch: true,
            },
            dns_servers: vec!["8.8.8.8".to_string()],
            proxy_config: None,
            network_manager: NetworkManager::NetworkManager,
        };

        assert!(config.vpn_config.enabled);
        assert!(!config.tor_config.enabled);
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
                encryption_enabled: false,
                encryption_algorithm: "aes-256-gcm".to_string(),
                key_derivation: "argon2".to_string(),
                persistence_size: 0,
                persistence_location: "".to_string(),
                auto_mount: false,
                hidden_volume: false,
                plausible_deniability: false,
            },
            security_config: SecurityConfig {
                memory_wipe_on_shutdown: true,
                disable_swap: true,
                disable_hibernation: true,
                firewall_enabled: false,
                network_isolation: false,
                mac_address_spoofing: false,
                dns_over_https: false,
                tor_enabled: false,
                vpn_enabled: false,
                kill_switch_enabled: false,
                secure_delete: true,
                disable_usb_storage: false,
                disable_bluetooth: false,
                disable_webcam: false,
                disable_microphone: false,
                screen_lock_timeout: Duration::from_secs(300),
                auto_logout_timeout: Duration::from_secs(600),
            },
            network_config: NetworkConfig {
                tor_config: TorConfig {
                    enabled: false,
                    bridge_mode: false,
                    bridges: vec![],
                    obfs4_enabled: false,
                    meek_enabled: false,
                    snowflake_enabled: false,
                    circuit_isolation: false,
                    exit_node_country: None,
                },
                vpn_config: VpnOsConfig {
                    enabled: false,
                    provider: "".to_string(),
                    server_address: "".to_string(),
                    port: 1194,
                    protocol: "udp".to_string(),
                    cipher_suite: "chacha20-poly1305".to_string(),
                    auto_connect: false,
                    kill_switch: false,
                },
                dns_servers: vec![],
                proxy_config: None,
                network_manager: NetworkManager::NetworkManager,
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
    fn test_vantis_os_builder() {
        let builder = VantisOsBuilder::new()
            .os_name("Vantis OS".to_string())
            .version("1.0.0".to_string());

        // Verify builder can be created
        assert!(true);
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
                    id: "rule-001".to_string(),
                    name: "allow-vpn".to_string(),
                    action: FirewallAction::Allow,
                    direction: FirewallDirection::Outbound,
                    protocol: Some("udp".to_string()),
                    source_ip: None,
                    source_port: None,
                    destination_ip: None,
                    destination_port: Some(1194),
                    enabled: true,
                    priority: 100,
                    log: false,
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
                dns_servers: vec![], kill_switch: true,
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
                encryption_enabled: true,
                encryption_algorithm: "aes-256-gcm".to_string(),
                key_derivation: "argon2".to_string(),
                persistence_size: 17179869184, // 16GB in bytes
                persistence_location: "/home".to_string(),
                auto_mount: true,
                hidden_volume: false,
                plausible_deniability: false,
            },
            security_config: SecurityConfig {
                memory_wipe_on_shutdown: true,
                disable_swap: true,
                disable_hibernation: true,
                firewall_enabled: true,
                network_isolation: true,
                mac_address_spoofing: true,
                dns_over_https: true,
                tor_enabled: true,
                vpn_enabled: true,
                kill_switch_enabled: true,
                secure_delete: true,
                disable_usb_storage: false,
                disable_bluetooth: true,
                disable_webcam: true,
                disable_microphone: true,
                screen_lock_timeout: Duration::from_secs(300),
                auto_logout_timeout: Duration::from_secs(600),
            },
            network_config: NetworkConfig {
                tor_config: TorConfig {
                    enabled: true,
                    bridge_mode: false,
                    bridges: vec![],
                    obfs4_enabled: false,
                    meek_enabled: false,
                    snowflake_enabled: false,
                    circuit_isolation: false,
                    exit_node_country: None,
                },
                vpn_config: VpnOsConfig {
                    enabled: true,
                    provider: "VantisVPN".to_string(),
                    server_address: "vpn.vantis.com".to_string(),
                    port: 1194,
                    protocol: "udp".to_string(),
                    cipher_suite: "chacha20-poly1305".to_string(),
                    auto_connect: true,
                    kill_switch: true,
                },
                dns_servers: vec!["8.8.8.8".to_string()],
                proxy_config: None,
                network_manager: NetworkManager::NetworkManager,
            },
            applications: vec![],
            locale: "en_US.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
        };

        assert!(config.security_config.firewall_enabled);
        assert!(config.network_config.vpn_config.enabled);
        assert!(config.network_config.tor_config.enabled);
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
                dns_servers: vec![], kill_switch: true,
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