//! # VANTISVPN Core Library
//!
//! This is the shared core library for VANTISVPN, providing cryptographic primitives,
//! network protocols, and core functionality shared across all platforms.
//!
//! ## Architecture
//!
//! The core is designed with privacy and security as first principles:
//! - Zero-knowledge architecture
//! - Post-quantum cryptography ready
//! - No persistent state
//! - Ephemeral key management
//!
//! ## Modules
//!
//! - `crypto`: Cryptographic primitives (PQC, classical crypto)
//! - `network`: Network protocols and tunneling
//! - `tunnel`: VPN tunnel management
//! - `error`: Error types and handling
//! - `config`: Configuration management
//! - `utils`: Utility functions

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod audit;
pub mod config;
pub mod crypto;
pub mod error;
pub mod hardware;
pub mod network;
pub mod privacy;
pub mod security;
pub mod server;
pub mod tunnel;
pub mod ui;
pub mod utils;

// Re-export commonly used types
pub use audit::{
    csfc_compliance::{CsfcCompliance, CsfcComponent, CsfcConfig, CsfcReport},
    hitrust_compliance::{HitrustCompliance, HitrustConfig, HitrustControl, HitrustReport},
    no_logs_audit::{AuditConfig, AuditEvidence, AuditReport, NoLogsAudit},
    pci_dss_compliance::{PciConfig, PciDssCompliance, PciReport, PciRequirement},
    security_pentest::{PentestConfig, PentestReport, SecurityPentest, Vulnerability},
    soc2_compliance::{Soc2Compliance, Soc2Config, Soc2Control, Soc2Report},
};
pub use config::{AppConfig, Config};
pub use crypto::{
    cipher::CipherMode,
    hash::Hash,
    keys::{Cipher, CipherSuite, EphemeralKeyPair},
    pqc_full::{HybridKeyExchange, MlDsaKeyPair, MlKemKeyPair, PqcManager},
    random::SecureRandom,
};
pub use error::{Result, VantisError};
pub use hardware::{
    router_os::{
        FirewallRule, NetworkInterface, PortForwarding, QosPolicy, RouterConfig, RouterFirmware,
        RouterFirmwareBuilder, RouterState, RouterStats,
    },
    vantis_os::{
        BootConfig, NetworkConfig, PersistenceConfig, SecurityConfig, VantisOsBuilder,
        VantisOsConfig, VantisOsImage,
    },
    yubikey::{
        YubiKeyAuth, YubiKeyChallengeResponse, YubiKeyConfig, YubiKeyHmac, YubiKeyManager,
        YubiKeyOtp, YubiKeySlot,
    },
};
pub use network::{
    multihop::{Circuit, MultiHopConfig, MultiHopManager},
    protocol::MessageType,
    quic_full::{QuicConfig, QuicConnection, QuicEndpoint, QuicStream},
    stealth::{StealthConfig, StealthHandler},
    wireguard_full::{InterfaceConfig, PeerConfig, WireGuardDevice},
};
pub use privacy::{
    anonymous_payments::{
        AnonymousPaymentManager, CashPayment, LightningPayment, MoneroPayment, PaymentConfig,
        PaymentMethod, PaymentStatus,
    },
    avantis_id::{AvantisIdConfig, AvantisIdManager, DigitalIdentity, IdentityProof, IdentityType},
    gdpr_compliance::{
        ConsentRecord, ConsentType, DataPortability, DataRequest, DataSubject, GdprCompliance,
        GdprConfig, RightToBeForgotten,
    },
    ip_rotator::{IpEndpoint, IpPool, IpRotator, RotationStrategy, RotatorConfig},
    zk_login::{
        AuthState, UserCredentials, ZkAuthResult, ZkChallenge, ZkLoginConfig, ZkLoginManager,
        ZkProofType, ZkResponse,
    },
};
pub use security::{
    avantis_mesh::{AvantisMesh, MeshConfig, MeshMessage, MeshNode, MeshStats},
    daita::{Daita, DaitaConfig, DaitaStrategy, TrafficStats},
    kill_switch::{
        KillSwitchConfig, KillSwitchManager, KillSwitchMode, KillSwitchState, KillSwitchStats,
    },
    netshield::{
        BlocklistCategory, BlocklistEntry, DnsQuery, DnsQueryType, DnsResponse, NetShieldConfig,
        NetShieldManager, NetShieldStats,
    },
    quantum_vault::{QuantumVault, VaultConfig, VaultEntry, VaultState, VaultStats},
    rbi::{
        BrowserEvent, BrowserSession, BrowserType, IsolationLevel, RbiConfig, RbiManager, RbiStats,
        RenderedFrame,
    },
    split_tunnel::{
        RuleType, SplitTunnelConfig, SplitTunnelManager, SplitTunnelMode,
        SplitTunnelRoutingDecision, SplitTunnelRule, SplitTunnelStats,
    },
    zero_trust::{
        AccessDecision, AccessLog, AccessRequest, DeviceTrust, PolicyAction, ZeroTrust,
        ZeroTrustConfig, ZeroTrustPolicy,
    },
};
pub use server::{
    colocated::{
        ColocatedConfig, ColocatedInfrastructureManager, InfrastructureStats,
        LoadBalancingStrategy, ServerLocation, ServerStatus, VpnServer,
    },
    ftth_jumbo::{
        FrameType, JumboFrameConfig, JumboFrameManager, JumboFrameStats,
        NetworkPath as JumboNetworkPath,
    },
    ram_only::{MemoryStats, RamOnlyConfig, RamOnlyServer, SessionData},
    secure_boot::{
        BootComponent, BootResult, IntegrityReport, SecureBootConfig, SecureBootManager,
    },
    smart_routing::{
        NetworkPath as RoutingPath, RoutingDecision, RoutingMetric, RoutingStats,
        SmartRoutingConfig, SmartRoutingManager,
    },
    starlink_fec::{
        FecAlgorithm, FecBlock, FecConfig, FecDecoder, FecEncoder, FecManager, FecStats,
    },
    tee::{AttestationReport, SecureEnclave, TeeConfig, TeeManager, TeeStats, TeeType},
    wifi7_mlo::{MloConfig, MloManager, MloStats, WifiBand, WifiLink},
};
pub use tunnel::{manager::TunnelManager, state::TunnelState};
pub use ui::{
    biometric_auth::{AuthResult, BiometricAuth, BiometricConfig, BiometricType},
    devtunnel::{DevTunnel, TunnelConfig, TunnelSession, TunnelStats},
    family_shield::{FamilyShield, ShieldConfig, ShieldRule, ShieldStats},
    theme_manager::{HapticPattern, HapticType, ThemeColors, ThemeConfig, ThemeManager, ThemeMode},
};

/// VANTISVPN version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum packet size for VPN traffic
pub const MAX_PACKET_SIZE: usize = 1500;

/// Default MTU for VPN tunnel
pub const DEFAULT_MTU: u16 = 1420;

/// Timeout for establishing VPN connection (seconds)
pub const CONNECTION_TIMEOUT: u64 = 30;

/// Heartbeat interval for keepalive (seconds)
pub const HEARTBEAT_INTERVAL: u64 = 10;

/// Number of retransmission attempts
pub const MAX_RETRANSMISSIONS: u32 = 3;

/// Initialize VANTISVPN core
///
/// This function must be called before any other core functions.
/// It sets up logging and initializes subsystems.
pub fn init() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("VANTISVPN Core v{} initialized", VERSION);

    // Initialize crypto subsystem
    crypto::init()?;

    Ok(())
}

/// Cleanup VANTISVPN core
///
/// This function should be called before shutdown to ensure
/// all sensitive data is securely cleared from memory.
pub fn cleanup() -> Result<()> {
    tracing::info!("Cleaning up VANTISVPN Core");

    // Securely clear all sensitive data
    crypto::cleanup()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_cleanup() {
        init().expect("Failed to initialize");
        cleanup().expect("Failed to cleanup");
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
