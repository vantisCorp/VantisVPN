// VANTISVPN Hardware Ecosystem Module
// Provides hardware integration and firmware components

/// Router Os module.
pub mod router_os;
/// Vantis Os module.
pub mod vantis_os;
/// Yubikey module.
pub mod yubikey;

pub use router_os::{
    FirewallAction, FirewallDirection, FirewallProtocol, FirewallRule, InterfaceType, LanConfig,
    NetworkInterface, PortForwarding, QosPolicy, QosPriority, RouterConfig, RouterFirmware,
    RouterFirmwareBuilder, RouterState, RouterStats, VpnRouterConfig, WanConfig,
};

pub use yubikey::{
    YubiKeyAuth, YubiKeyChallengeResponse, YubiKeyConfig, YubiKeyHmac, YubiKeyManager, YubiKeyOtp,
    YubiKeySlot,
};

pub use vantis_os::{
    BootConfig, BootMode, BootOption, Bootloader, NetworkConfig, NetworkManager, PersistenceConfig,
    ProxyConfig, SecurityConfig, TorConfig, VantisOsBuilder, VantisOsConfig, VantisOsImage,
    VpnOsConfig, WanConnectionType,
};

#[cfg(test)]
mod comprehensive_tests;
