// VANTISVPN Vantis OS
// Tails-like secure USB operating system

use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};
use crate::error::VantisError;

/// Vantis OS configuration
/// 
/// Complete configuration for the Vantis OS secure USB operating system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VantisOsConfig {
    /// Operating system name
    /// 
    /// Name of the operating system.
    pub os_name: String,
    /// Version number
    /// 
    /// Version string of the operating system.
    pub version: String,
    /// Build number
    /// 
    /// Build number of this release.
    pub build_number: String,
    /// Boot configuration
    /// 
    /// Boot configuration settings.
    pub boot_config: BootConfig,
    /// Persistence configuration
    /// 
    /// Data persistence configuration.
    pub persistence_config: PersistenceConfig,
    /// Security configuration
    /// 
    /// Security settings and policies.
    pub security_config: SecurityConfig,
    /// Network configuration
    /// 
    /// Network configuration settings.
    pub network_config: NetworkConfig,
    /// List of applications
    /// 
    /// Pre-installed applications.
    pub applications: Vec<ApplicationConfig>,
    /// System locale
    /// 
    /// System locale setting.
    pub locale: String,
    /// System timezone
    /// 
    /// System timezone setting.
    pub timezone: String,
    /// Keyboard layout
    /// 
    /// Keyboard layout setting.
    pub keyboard_layout: String,
}

/// Boot configuration
/// 
/// Boot configuration settings for Vantis OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootConfig {
    /// Boot mode
    /// 
    /// Mode of operation (live, persistent, or encrypted).
    pub boot_mode: BootMode,
    /// Secure boot
    /// 
    /// Whether secure boot is enabled.
    pub secure_boot: bool,
    /// Boot timeout
    /// 
    /// Timeout before automatic boot in seconds.
    pub boot_timeout: Duration,
    /// Default boot option
    /// 
    /// Default boot option to use.
    pub default_boot_option: BootOption,
    /// Kernel parameters
    /// 
    /// Additional kernel parameters.
    pub kernel_parameters: Vec<String>,
    /// Initramfs compression
    /// 
    /// Compression algorithm for initramfs.
    pub initramfs_compression: String,
    /// Bootloader
    /// 
    /// Bootloader to use (GRUB, Syslinux, or systemd-boot).
    pub bootloader: Bootloader,
}

/// Boot mode
/// 
/// Available boot modes for Vantis OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BootMode {
    /// Live mode
    /// 
    /// Boot from USB without persistence.
    Live,
    /// Persistent mode
    /// 
    /// Boot from USB with encrypted persistence.
    Persistent,
    /// Encrypted mode
    /// 
    /// Fully encrypted system mode.
    Encrypted,
}

/// Boot option
/// 
/// Available boot options for the bootloader menu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BootOption {
    /// Live mode
    /// 
    /// Boot in live mode (no persistence).
    LiveMode,
    /// Persistent mode
    /// 
    /// Boot in persistent mode (encrypted persistence).
    PersistentMode,
    /// Encrypted mode
    /// 
    /// Boot in fully encrypted mode.
    EncryptedMode,
    /// Diagnostic mode
    /// 
    /// Boot in diagnostic/recovery mode.
    DiagnosticMode,
}

/// Bootloader
/// 
/// Supported bootloaders for Vantis OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bootloader {
    /// GRUB bootloader
    /// 
    /// GNU GRUB bootloader (default).
    Grub,
    /// Syslinux bootloader
    /// 
    /// Syslinux bootloader (lightweight).
    Syslinux,
    /// systemd-boot bootloader
    /// 
    /// systemd-boot bootloader (UEFI).
    SystemdBoot,
}

/// Persistence configuration for Vantis OS
///
/// Controls how Vantis OS handles data persistence across reboots.
/// Vantis OS is designed to be a live OS that runs from RAM,
/// but can optionally persist specific data to encrypted storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    pub enabled: bool,
    pub encryption_enabled: bool,
    pub encryption_algorithm: String,
    pub key_derivation: String,
    pub persistence_size: u64, // bytes
    pub persistence_location: String,
    pub auto_mount: bool,
    pub hidden_volume: bool,
    pub plausible_deniability: bool,
}

/// Security configuration for Vantis OS
///
/// Defines security parameters for the Vantis OS including authentication,
/// encryption, and access control settings to protect against unauthorized
/// access and data leakage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub memory_wipe_on_shutdown: bool,
    pub disable_swap: bool,
    pub disable_hibernation: bool,
    pub firewall_enabled: bool,
    pub network_isolation: bool,
    pub mac_address_spoofing: bool,
    pub dns_over_https: bool,
    pub tor_enabled: bool,
    pub vpn_enabled: bool,
    pub kill_switch_enabled: bool,
    pub secure_delete: bool,
    pub disable_usb_storage: bool,
    pub disable_bluetooth: bool,
    pub disable_webcam: bool,
    pub disable_microphone: bool,
    pub screen_lock_timeout: Duration,
    pub auto_logout_timeout: Duration,
}

/// Network configuration for Vantis OS
///
/// Controls network settings including interface configuration,
/// DNS settings, and network security options for the Vantis OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub tor_config: TorConfig,
    pub vpn_config: VpnOsConfig,
    pub dns_servers: Vec<String>,
    pub proxy_config: Option<ProxyConfig>,
    pub network_manager: NetworkManager,
}

/// Tor configuration for Vantis OS
///
/// Settings for the Tor network integration in Vantis OS,
/// allowing all traffic to be routed through the Tor network
/// for enhanced anonymity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorConfig {
    pub enabled: bool,
    pub bridge_mode: bool,
    pub bridges: Vec<String>,
    pub obfs4_enabled: bool,
    pub meek_enabled: bool,
    pub snowflake_enabled: bool,
    pub circuit_isolation: bool,
    pub exit_node_country: Option<String>,
}

/// VPN configuration for Vantis OS
///
/// Defines VPN settings that are integrated into the Vantis OS,
/// allowing all network traffic to be automatically routed through
/// the VANTISVPN service for protection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnOsConfig {
    pub enabled: bool,
    pub provider: String,
    pub server_address: String,
    pub port: u16,
    pub protocol: String,
    pub cipher_suite: String,
    pub auto_connect: bool,
    pub kill_switch: bool,
}

/// Proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy type (HTTP, SOCKS5, etc.)
    pub proxy_type: String,
    /// Proxy server address
    pub address: String,
    /// Proxy server port
    pub port: u16,
    /// Optional username for authentication
    pub username: Option<String>,
    /// Optional password for authentication
    pub password: Option<String>,
}

/// Network manager for Vantis OS
///
/// Different network management systems available for configuring
/// network interfaces and connections in the live USB OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkManager {
    /// NetworkManager (GNOME)
    NetworkManager,
    /// Connman (lightweight)
    Connman,
    /// Wicd (simple)
    Wicd,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Whether the application is enabled
    pub enabled: bool,
    /// Whether the application starts automatically
    pub autostart: bool,
    /// Whether the application runs in a sandbox
    pub sandboxed: bool,
    /// Whether the application has network access
    pub network_access: bool,
    /// Whether the application can access persistent storage
    pub persistence_access: bool,
}

/// Vantis OS image
pub struct VantisOsImage {
    config: VantisOsConfig,
    image_size: u64,
    checksum: String,
    created_at: SystemTime,
}

/// Vantis OS builder
pub struct VantisOsBuilder {
    config: VantisOsConfig,
    included_packages: Vec<String>,
    excluded_packages: Vec<String>,
    custom_scripts: Vec<String>,
}

impl VantisOsBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            config: VantisOsConfig::default(),
            included_packages: vec![],
            excluded_packages: vec![],
            custom_scripts: vec![],
        }
    }

    /// Set OS name
    pub fn os_name(mut self, name: String) -> Self {
        self.config.os_name = name;
        self
    }

    /// Set version
    pub fn version(mut self, version: String) -> Self {
        self.config.version = version;
        self
    }

    /// Set boot configuration
    pub fn boot_config(mut self, boot_config: BootConfig) -> Self {
        self.config.boot_config = boot_config;
        self
    }

    /// Set persistence configuration
    pub fn persistence_config(mut self, persistence_config: PersistenceConfig) -> Self {
        self.config.persistence_config = persistence_config;
        self
    }

    /// Set security configuration
    pub fn security_config(mut self, security_config: SecurityConfig) -> Self {
        self.config.security_config = security_config;
        self
    }

    /// Set network configuration
    pub fn network_config(mut self, network_config: NetworkConfig) -> Self {
        self.config.network_config = network_config;
        self
    }

    /// Add application
    pub fn add_application(mut self, app: ApplicationConfig) -> Self {
        self.config.applications.push(app);
        self
    }

    /// Include package
    pub fn include_package(mut self, package: String) -> Self {
        self.included_packages.push(package);
        self
    }

    /// Exclude package
    pub fn exclude_package(mut self, package: String) -> Self {
        self.excluded_packages.push(package);
        self
    }

    /// Add custom script
    pub fn add_custom_script(mut self, script: String) -> Self {
        self.custom_scripts.push(script);
        self
    }

    /// Build Vantis OS image
    pub fn build(self) -> Result<VantisOsImage, VantisError> {
        // Validate configuration
        self.validate_config()?;

        // Calculate image size (placeholder)
        let image_size = 4 * 1024 * 1024 * 1024; // 4 GB default

        // Generate checksum (placeholder)
        let checksum = "sha256:placeholder_checksum".to_string();

        let image = VantisOsImage {
            config: self.config,
            image_size,
            checksum,
            created_at: SystemTime::now(),
        };

        Ok(image)
    }

    /// Validate configuration
    fn validate_config(&self) -> Result<(), VantisError> {
        // Check if persistence is enabled but size is 0
        if self.config.persistence_config.enabled && self.config.persistence_config.persistence_size == 0 {
            return Err(VantisError::InvalidData("Persistence size must be > 0".to_string()));
        }

        // Check if both Tor and VPN are enabled
        if self.config.network_config.tor_config.enabled && self.config.network_config.vpn_config.enabled {
            return Err(VantisError::InvalidData("Cannot enable both Tor and VPN simultaneously".to_string()));
        }

        Ok(())
    }
}

impl VantisOsImage {
    /// Get configuration
    pub fn config(&self) -> &VantisOsConfig {
        &self.config
    }

    /// Get image size
    pub fn image_size(&self) -> u64 {
        self.image_size
    }

    /// Get checksum
    pub fn checksum(&self) -> &str {
        &self.checksum
    }

    /// Get creation time
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    /// Generate ISO image
    pub fn generate_iso(&self, _output_path: PathBuf) -> Result<(), VantisError> {
        // Placeholder for ISO generation
        // In production, this would use tools like xorriso, mkisofs, etc.
        
        Ok(())
    }

    /// Generate USB image
    pub fn generate_usb_image(&self, _output_path: PathBuf) -> Result<(), VantisError> {
        // Placeholder for USB image generation
        // In production, this would use dd, or create a hybrid ISO
        
        Ok(())
    }

    /// Verify image integrity
    pub fn verify_integrity(&self, checksum: &str) -> Result<bool, VantisError> {
        Ok(self.checksum == checksum)
    }

    /// Export configuration
    pub fn export_config(&self) -> Result<Vec<u8>, VantisError> {
        serde_json::to_vec(&self.config)
            .map_err(|e| VantisError::InvalidData(format!("Failed to export config: {}", e)))
    }

    /// Import configuration
    pub fn import_config(data: &[u8]) -> Result<VantisOsConfig, VantisError> {
        serde_json::from_slice(data)
            .map_err(|e| VantisError::InvalidData(format!("Failed to import config: {}", e)))
    }
}

impl Default for VantisOsConfig {
    fn default() -> Self {
        Self {
            os_name: "Vantis OS".to_string(),
            version: "1.0.0".to_string(),
            build_number: "20240101".to_string(),
            boot_config: BootConfig::default(),
            persistence_config: PersistenceConfig::default(),
            security_config: SecurityConfig::default(),
            network_config: NetworkConfig::default(),
            applications: vec![
                ApplicationConfig {
                    name: "Tor Browser".to_string(),
                    version: "13.0".to_string(),
                    enabled: true,
                    autostart: false,
                    sandboxed: true,
                    network_access: true,
                    persistence_access: false,
                },
                ApplicationConfig {
                    name: "VantisVPN Client".to_string(),
                    version: "1.0.0".to_string(),
                    enabled: true,
                    autostart: true,
                    sandboxed: true,
                    network_access: true,
                    persistence_access: true,
                },
                ApplicationConfig {
                    name: "Electrum Bitcoin Wallet".to_string(),
                    version: "4.5.0".to_string(),
                    enabled: true,
                    autostart: false,
                    sandboxed: true,
                    network_access: true,
                    persistence_access: true,
                },
                ApplicationConfig {
                    name: "VeraCrypt".to_string(),
                    version: "1.26.7".to_string(),
                    enabled: true,
                    autostart: false,
                    sandboxed: false,
                    network_access: false,
                    persistence_access: true,
                },
            ],
            locale: "en_US".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
        }
    }
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            boot_mode: BootMode::Live,
            secure_boot: true,
            boot_timeout: Duration::from_secs(5),
            default_boot_option: BootOption::LiveMode,
            kernel_parameters: vec![
                "quiet".to_string(),
                "splash".to_string(),
                "toram".to_string(),
            ],
            initramfs_compression: "zstd".to_string(),
            bootloader: Bootloader::Grub,
        }
    }
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            encryption_enabled: true,
            encryption_algorithm: "aes-256-xts".to_string(),
            key_derivation: "argon2id".to_string(),
            persistence_size: 0,
            persistence_location: "/dev/disk/by-label/VantisPersistence".to_string(),
            auto_mount: true,
            hidden_volume: false,
            plausible_deniability: false,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            memory_wipe_on_shutdown: true,
            disable_swap: true,
            disable_hibernation: true,
            firewall_enabled: true,
            network_isolation: false,
            mac_address_spoofing: true,
            dns_over_https: true,
            tor_enabled: false,
            vpn_enabled: false,
            kill_switch_enabled: true,
            secure_delete: true,
            disable_usb_storage: false,
            disable_bluetooth: true,
            disable_webcam: false,
            disable_microphone: false,
            screen_lock_timeout: Duration::from_secs(300), // 5 minutes
            auto_logout_timeout: Duration::from_secs(1800), // 30 minutes
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            tor_config: TorConfig::default(),
            vpn_config: VpnOsConfig::default(),
            dns_servers: vec![
                "1.1.1.1".to_string(),
                "1.0.0.1".to_string(),
            ],
            proxy_config: None,
            network_manager: NetworkManager::NetworkManager,
        }
    }
}

impl Default for TorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bridge_mode: false,
            bridges: vec![],
            obfs4_enabled: false,
            meek_enabled: false,
            snowflake_enabled: false,
            circuit_isolation: true,
            exit_node_country: None,
        }
    }
}

impl Default for VpnOsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "VantisVPN".to_string(),
            server_address: "".to_string(),
            port: 51820,
            protocol: "udp".to_string(),
            cipher_suite: "chacha20-poly1305".to_string(),
            auto_connect: false,
            kill_switch: true,
        }
    }
}

impl Default for VantisOsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// WAN connection types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WanConnectionType {
    Dhcp,
    Static,
    Pppoe,
    Pptp,
    L2tp,
}

impl std::fmt::Display for WanConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dhcp => write!(f, "DHCP"),
            Self::Static => write!(f, "Static"),
            Self::Pppoe => write!(f, "PPPoE"),
            Self::Pptp => write!(f, "PPTP"),
            Self::L2tp => write!(f, "L2TP"),
        }
    }
}