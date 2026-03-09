// Server Infrastructure Module
// Exports all server infrastructure components

/// Colocated module.
pub mod colocated;
/// Ftth Jumbo module.
pub mod ftth_jumbo;
/// Ram Only module.
pub mod ram_only;
/// Secure Boot module.
pub mod secure_boot;
/// Smart Routing module.
pub mod smart_routing;
/// Starlink Fec module.
pub mod starlink_fec;
/// Tee module.
pub mod tee;
/// Wifi7 Mlo module.
pub mod wifi7_mlo;

// Re-export main types
pub use colocated::{
    ColocatedConfig, ColocatedInfrastructureManager, InfrastructureStats, LoadBalancingStrategy,
    ServerCapabilities, ServerLocation, ServerStatus, VpnServer,
};
pub use ftth_jumbo::{
    FrameType, JumboFrameConfig, JumboFrameManager, JumboFrameStats, NetworkPath,
};
pub use ram_only::{MemoryStats, RamOnlyConfig, RamOnlyServer, SessionData};
pub use secure_boot::{
    BootComponent, BootResult, IntegrityReport, SecureBootConfig, SecureBootManager,
};
pub use smart_routing::{
    NetworkPath as RoutingPath, RoutingDecision, RoutingMetric, RoutingStats, SmartRoutingConfig,
    SmartRoutingManager,
};
pub use starlink_fec::{
    FecAlgorithm, FecBlock, FecConfig, FecDecoder, FecEncoder, FecManager, FecStats,
};
pub use tee::{AttestationReport, SecureEnclave, TeeConfig, TeeManager, TeeStats, TeeType};
pub use wifi7_mlo::{MloConfig, MloManager, MloStats, WifiBand, WifiLink};

#[cfg(test)]
mod comprehensive_tests;
