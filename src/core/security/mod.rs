// Security Module - User Security & Protection
// Phase 4: User Security & Protection

pub mod kill_switch;
pub mod split_tunnel;
pub mod rbi;
pub mod netshield;
pub mod daita;
pub mod quantum_vault;
pub mod zero_trust;
pub mod avantis_mesh;

// Re-exports
pub use kill_switch::{KillSwitchManager, KillSwitchConfig, KillSwitchState, KillSwitchMode, KillSwitchStats};
pub use split_tunnel::{SplitTunnelManager, SplitTunnelConfig, SplitTunnelRule, SplitTunnelMode, RuleType, SplitTunnelRoutingDecision, SplitTunnelStats};
pub use rbi::{RbiManager, RbiConfig, BrowserSession, BrowserType, IsolationLevel, RenderedFrame, BrowserEvent, RbiStats};
pub use netshield::{NetShieldManager, NetShieldConfig, BlocklistEntry, BlocklistCategory, DnsQuery, DnsResponse, DnsQueryType, NetShieldStats};
pub use daita::{Daita, DaitaConfig, DaitaStrategy, TrafficStats};
pub use quantum_vault::{QuantumVault, VaultConfig, VaultEntry, VaultState, VaultStats};
pub use zero_trust::{ZeroTrust, ZeroTrustConfig, ZeroTrustPolicy, PolicyAction, AccessRequest, AccessDecision, AccessLog, DeviceTrust};
pub use avantis_mesh::{AvantisMesh, MeshConfig, MeshNode, MeshMessage, MeshStats};

#[cfg(test)]
mod comprehensive_tests;