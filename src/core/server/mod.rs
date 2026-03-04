// Server Infrastructure Module
// Exports all server infrastructure components

pub mod ram_only;
pub mod tee;
pub mod secure_boot;
pub mod starlink_fec;
pub mod wifi7_mlo;
pub mod ftth_jumbo;
pub mod smart_routing;
pub mod colocated;

// Re-export main types
pub use ram_only::{RamOnlyConfig, RamOnlyServer, SessionData, MemoryStats};
pub use tee::{TeeConfig, TeeManager, TeeType, SecureEnclave, AttestationReport, TeeStats};
pub use secure_boot::{SecureBootConfig, SecureBootManager, BootComponent, BootResult, IntegrityReport};
pub use starlink_fec::{FecConfig, FecManager, FecEncoder, FecDecoder, FecBlock, FecStats, FecAlgorithm};
pub use wifi7_mlo::{MloConfig, MloManager, WifiLink, WifiBand, MloStats};
pub use ftth_jumbo::{JumboFrameConfig, JumboFrameManager, NetworkPath, JumboFrameStats, FrameType};
pub use smart_routing::{SmartRoutingConfig, SmartRoutingManager, NetworkPath as RoutingPath, RoutingDecision, RoutingStats, RoutingMetric};
pub use colocated::{ColocatedConfig, ColocatedInfrastructureManager, VpnServer, ServerLocation, ServerStatus, InfrastructureStats, LoadBalancingStrategy};

#[cfg(test)]
mod comprehensive_tests;