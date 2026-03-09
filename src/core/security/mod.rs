// Security Module - User Security & Protection
// Phase 4: User Security & Protection

/// Avantis Mesh module.
pub mod avantis_mesh;
/// Daita module.
pub mod daita;
/// Kill Switch module.
pub mod kill_switch;
/// Netshield module.
pub mod netshield;
/// Quantum Vault module.
pub mod quantum_vault;
/// Rbi module.
pub mod rbi;
/// Split Tunnel module.
pub mod split_tunnel;
/// Zero Trust module.
pub mod zero_trust;

// Re-exports
pub use avantis_mesh::{AvantisMesh, MeshConfig, MeshMessage, MeshNode, MeshStats};
pub use daita::{Daita, DaitaConfig, DaitaStrategy, TrafficStats};
pub use kill_switch::{
    KillSwitchConfig, KillSwitchManager, KillSwitchMode, KillSwitchState, KillSwitchStats,
};
pub use netshield::{
    BlocklistCategory, BlocklistEntry, DnsQuery, DnsQueryType, DnsResponse, NetShieldConfig,
    NetShieldManager, NetShieldStats,
};
pub use quantum_vault::{QuantumVault, VaultConfig, VaultEntry, VaultState, VaultStats};
pub use rbi::{
    BrowserEvent, BrowserSession, BrowserType, IsolationLevel, RbiConfig, RbiManager, RbiStats,
    RenderedFrame,
};
pub use split_tunnel::{
    RuleType, SplitTunnelConfig, SplitTunnelManager, SplitTunnelMode, SplitTunnelRoutingDecision,
    SplitTunnelRule, SplitTunnelStats,
};
pub use zero_trust::{
    AccessDecision, AccessLog, AccessRequest, DeviceTrust, PolicyAction, ZeroTrust,
    ZeroTrustConfig, ZeroTrustPolicy,
};

#[cfg(test)]
mod comprehensive_tests;
