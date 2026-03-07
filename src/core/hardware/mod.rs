// VANTISVPN Hardware Ecosystem Module
// Provides hardware integration and firmware components

pub mod router_os;
pub mod yubikey;
pub mod vantis_os;

pub use router_os::{
    RouterConfig, VpnRouterConfig, RouterFirmware, RouterState, RouterStats,
    FirewallRule, PortForwarding, QosPolicy, NetworkInterface,
    RouterFirmwareBuilder, InterfaceType, FirewallAction, FirewallProtocol, QosPriority,
    LanConfig, WanConfig, FirewallDirection
};

pub use yubikey::{
    YubiKeyConfig, YubiKeyManager, YubiKeyAuth, YubiKeySlot,
    YubiKeyChallengeResponse, YubiKeyHmac, YubiKeyOtp
};

pub use vantis_os::{
    VantisOsConfig, VantisOsBuilder, VantisOsImage, BootConfig,
    PersistenceConfig, SecurityConfig, NetworkConfig,
    BootMode, BootOption, Bootloader, WanConnectionType,
    TorConfig, VpnOsConfig, NetworkManager, ProxyConfig
};

#[cfg(test)]
mod comprehensive_tests;