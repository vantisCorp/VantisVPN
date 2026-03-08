// Audit Module - Audit & Certification
// Phase 7: Audit & Certification
// Provides audit readiness, compliance frameworks, and certification support

pub mod csfc_compliance;
pub mod hitrust_compliance;
pub mod no_logs_audit;
pub mod pci_dss_compliance;
pub mod security_pentest;
pub mod soc2_compliance;

// Re-exports
pub use csfc_compliance::{CsfcCompliance, CsfcComponent, CsfcConfig, CsfcReport};
pub use hitrust_compliance::{HitrustCompliance, HitrustConfig, HitrustControl, HitrustReport};
pub use no_logs_audit::{AuditConfig, AuditEvidence, AuditReport, NoLogsAudit};
pub use pci_dss_compliance::{PciConfig, PciDssCompliance, PciReport, PciRequirement};
pub use security_pentest::{PentestConfig, PentestReport, SecurityPentest, Vulnerability};
pub use soc2_compliance::{Soc2Compliance, Soc2Config, Soc2Control, Soc2Report};
