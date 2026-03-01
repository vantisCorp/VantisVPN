// Audit Module - Audit & Certification
// Phase 7: Audit & Certification
// Provides audit readiness, compliance frameworks, and certification support

pub mod no_logs_audit;
pub mod security_pentest;
pub mod csfc_compliance;
pub mod pci_dss_compliance;
pub mod soc2_compliance;
pub mod hitrust_compliance;

// Re-exports
pub use no_logs_audit::{NoLogsAudit, AuditConfig, AuditReport, AuditEvidence};
pub use security_pentest::{SecurityPentest, PentestConfig, PentestReport, Vulnerability};
pub use csfc_compliance::{CsfcCompliance, CsfcConfig, CsfcReport, CsfcComponent};
pub use pci_dss_compliance::{PciDssCompliance, PciConfig, PciReport, PciRequirement};
pub use soc2_compliance::{Soc2Compliance, Soc2Config, Soc2Report, Soc2Control};
pub use hitrust_compliance::{HitrustCompliance, HitrustConfig, HitrustReport, HitrustControl};