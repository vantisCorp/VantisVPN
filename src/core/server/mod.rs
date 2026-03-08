// Server Infrastructure Module
// Exports all server infrastructure components

pub mod colocated;
pub mod ftth_jumbo;
pub mod ram_only;
pub mod secure_boot;
pub mod smart_routing;
pub mod starlink_fec;
pub mod tee;
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
