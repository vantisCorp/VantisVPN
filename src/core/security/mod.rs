// Security Module - User Security & Protection
// Phase 4: User Security & Protection

pub mod avantis_mesh;
pub mod daita;
pub mod kill_switch;
pub mod netshield;
pub mod quantum_vault;
pub mod rbi;
pub mod split_tunnel;
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
