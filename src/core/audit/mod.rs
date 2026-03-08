// Audit Module - Audit & Certification
// Phase 7: Audit & Certification
// Provides audit readiness, compliance frameworks, and certification support

/// CMMC/CSFC compliance framework implementation
pub mod csfc_compliance;
/// HITRUST CSF compliance framework implementation
pub mod hitrust_compliance;
/// No-logs audit system for privacy verification
pub mod no_logs_audit;
/// PCI DSS compliance framework for payment card security
pub mod pci_dss_compliance;
/// Security penetration testing utilities and reports
pub mod security_pentest;
/// SOC 2 compliance framework implementation
pub mod soc2_compliance;

// Re-exports
pub use csfc_compliance::{CsfcCompliance, CsfcComponent, CsfcConfig, CsfcReport};
pub use hitrust_compliance::{HitrustCompliance, HitrustConfig, HitrustControl, HitrustReport};
pub use no_logs_audit::{AuditConfig, AuditEvidence, AuditReport, NoLogsAudit};
pub use pci_dss_compliance::{PciConfig, PciDssCompliance, PciReport, PciRequirement};
pub use security_pentest::{PentestConfig, PentestReport, SecurityPentest, Vulnerability};
pub use soc2_compliance::{Soc2Compliance, Soc2Config, Soc2Control, Soc2Report};
