//! # Configuration Management
//! 
//! Manages configuration for VANTISVPN components.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Application settings
    pub app: AppConfig,
    /// Network settings
    pub network: NetworkConfig,
    /// Security settings
    pub security: SecurityConfig,
    /// Logging settings
    pub logging: LoggingConfig,
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Data directory
    pub data_dir: PathBuf,
    /// Config directory
    pub config_dir: PathBuf,
    /// Cache directory
    pub cache_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "VANTISVPN".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            data_dir: PathBuf::from("/var/lib/vantisvpn"),
            config_dir: PathBuf::from("/etc/vantisvpn"),
            cache_dir: PathBuf::from("/var/cache/vantisvpn"),
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Default server endpoint
    pub default_server: String,
    /// Default port
    pub default_port: u16,
    /// MTU
    pub mtu: u16,
    /// Enable IPv6
    pub enable_ipv6: bool,
    /// Enable QUIC
    pub enable_quic: bool,
    /// Keepalive interval (seconds)
    pub keepalive_interval: u64,
    /// Connection timeout (seconds)
    pub connection_timeout: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            default_server: String::new(),
            default_port: 443,
            mtu: 1420,
            enable_ipv6: true,
            enable_quic: true,
            keepalive_interval: 10,
            connection_timeout: 30,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable post-quantum cryptography
    pub enable_pqc: bool,
    /// Enable kill switch
    pub enable_kill_switch: bool,
    /// Enable split tunneling
    pub enable_split_tunneling: bool,
    /// Enable DNS over HTTPS
    pub enable_doh: bool,
    /// DNS servers
    pub dns_servers: Vec<String>,
    /// Enable strict mode (blocks all traffic if VPN disconnects)
    pub strict_mode: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_pqc: true,
            enable_kill_switch: true,
            enable_split_tunneling: false,
            enable_doh: true,
            dns_servers: vec![
                "1.1.1.1".to_string(),
                "1.0.0.1".to_string(),
            ],
            strict_mode: true,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log file path
    pub log_file: Option<PathBuf>,
    /// Enable console logging
    pub console: bool,
    /// Enable file logging
    pub file: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_file: Some(PathBuf::from("/var/log/vantisvpn/vpn.log")),
            console: true,
            file: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app: AppConfig::default(),
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Config {
    /// Create a new default configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Load configuration from file
    pub fn load_from_file(path: &PathBuf) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::VantisError::Generic(format!("Failed to read config: {}", e)))?;
        
        let config: Config = serde_json::from_str(&content)
            .map_err(|e| crate::VantisError::Generic(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file(&self, path: &PathBuf) -> crate::Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| crate::VantisError::Generic(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| crate::VantisError::Generic(format!("Failed to write config: {}", e)))?;
        
        Ok(())
    }
    
    /// Validate configuration
    pub fn validate(&self) -> crate::Result<()> {
        // Validate MTU
        if self.network.mtu < 576 || self.network.mtu > 9000 {
            return Err(crate::VantisError::InvalidMtu);
        }
        
        // Validate DNS servers
        for dns in &self.security.dns_servers {
            if dns.is_empty() {
                return Err(crate::VantisError::Generic("Empty DNS server".to_string()));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::new();
        let json = serde_json::to_string(&config).expect("Failed to serialize");
        assert!(json.len() > 0);
    }

    #[test]
    fn test_invalid_mtu() {
        let mut config = Config::new();
        config.network.mtu = 100; // Too low
        
        assert!(config.validate().is_err());
    }
}