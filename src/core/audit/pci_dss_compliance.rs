// PCI DSS Compliance - Payment Card Industry Data Security Standard
// Phase 7: Audit & Certification
// Provides framework for PCI DSS compliance and requirement tracking

use crate::error::VantisError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Status of PCI DSS compliance requirement
///
/// Current compliance state of a specific PCI DSS requirement,
/// tracking implementation progress and audit results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PciRequirementStatus {
    /// Not compliant
    NotCompliant,
    /// Partially compliant
    PartiallyCompliant,
    /// Compliant
    Compliant,
    /// Not applicable
    NotApplicable,
}

/// PCI DSS requirement
///
/// Represents a single PCI DSS requirement with its compliance status,
/// evidence, and review schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciRequirement {
    /// Unique identifier for this requirement
    pub requirement_id: String,
    /// Requirement number (e.g., "1.1", "2.3")
    pub requirement_number: String,
    /// Title of the requirement
    pub title: String,
    /// Detailed description of the requirement
    pub description: String,
    /// Current compliance status
    pub status: PciRequirementStatus,
    /// Evidence collected to support compliance
    pub evidence: Vec<String>,
    /// Additional notes or observations
    pub notes: String,
    /// Timestamp of last review, if any
    pub last_reviewed: Option<DateTime<Utc>>,
    /// Scheduled date for next review, if any
    pub next_review: Option<DateTime<Utc>>,
}

/// PCI DSS compliance report
///
/// Contains a comprehensive assessment of PCI DSS compliance status,
/// including all requirements, findings, and recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciReport {
    /// Unique identifier for this report
    pub report_id: String,
    /// Report type (SAQ A, SAQ D, ROC, etc.)
    pub report_type: String,
    /// List of all PCI DSS requirements assessed
    pub requirements: Vec<PciRequirement>,
    /// Overall compliance status across all requirements
    pub overall_status: PciRequirementStatus,
    /// Overall compliance score (0-100)
    pub compliance_score: u8,
    /// List of findings and issues discovered
    pub findings: Vec<String>,
    /// Recommendations for improving compliance
    pub recommendations: Vec<String>,
    /// Name of the assessor who conducted the review
    pub assessor_name: String,
    /// Date when the assessment was conducted
    pub assessment_date: DateTime<Utc>,
    /// Date until which this report is valid
    pub valid_until: DateTime<Utc>,
    /// Timestamp when the report was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the report was last updated
    pub updated_at: DateTime<Utc>,
}

/// PCI DSS compliance configuration
///
/// Configuration settings for PCI DSS compliance monitoring and reporting,
/// including automatic checking intervals and notification settings.
#[derive(Debug, Clone)]
pub struct PciConfig {
    /// Enable automatic compliance checking on a schedule
    pub enable_auto_check: bool,
    /// Number of days between automatic compliance checks
    pub check_interval_days: u32,
    /// Type of PCI DSS report to generate (SAQ A, SAQ D, ROC, etc.)
    pub report_type: String,
    /// Name of the qualified security assessor
    pub assessor_name: String,
    /// Number of days before expiration to send notifications
    pub notify_before_expiration: u32,
    /// Minimum compliance score (0-100) required for passing
    pub min_compliance_score: u8,
}

impl Default for PciConfig {
    fn default() -> Self {
        Self {
            enable_auto_check: true,
            check_interval_days: 90,
            report_type: "SAQ A".to_string(),
            assessor_name: "Qualified Security Assessor".to_string(),
            notify_before_expiration: 30,
            min_compliance_score: 80,
        }
    }
}

/// PCI DSS Compliance Manager
///
/// Manages PCI DSS compliance monitoring, reporting, and requirement tracking
/// for the VPN system to ensure compliance with Payment Card Industry standards.
pub struct PciDssCompliance {
    /// Configuration settings for PCI DSS compliance
    config: PciConfig,
    /// Map of report IDs to compliance reports
    reports: Arc<Mutex<HashMap<String, PciReport>>>,
    /// Map of requirement IDs to requirement details
    requirements: Arc<Mutex<HashMap<String, PciRequirement>>>,
}

impl PciDssCompliance {
    /// Create a new PCI DSS Compliance instance
    pub fn new(config: PciConfig) -> Self {
        Self {
            config,
            reports: Arc::new(Mutex::new(HashMap::new())),
            requirements: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add requirement
    pub async fn add_requirement(
        &self,
        requirement: PciRequirement,
    ) -> Result<String, VantisError> {
        let req_id = requirement.requirement_id.clone();

        let mut requirements = self.requirements.lock().await;
        requirements.insert(req_id.clone(), requirement);

        Ok(req_id)
    }

    /// Update requirement status
    pub async fn update_requirement_status(
        &self,
        req_id: &str,
        status: PciRequirementStatus,
        evidence: Vec<String>,
        notes: String,
    ) -> Result<(), VantisError> {
        let mut requirements = self.requirements.lock().await;
        if let Some(req) = requirements.get_mut(req_id) {
            req.status = status;
            req.evidence = evidence;
            req.notes = notes;
            req.last_reviewed = Some(Utc::now());
            req.next_review = Some(Utc::now() + chrono::Duration::days(90));
            Ok(())
        } else {
            Err(VantisError::NotFound(format!(
                "Requirement {} not found",
                req_id
            )))
        }
    }

    /// Create compliance report
    pub async fn create_report(&self) -> Result<String, VantisError> {
        let report_id = format!("pci_{}", Utc::now().timestamp());

        let requirements = self.requirements.lock().await;
        let req_list: Vec<_> = requirements.values().cloned().collect();

        let overall_status = if req_list
            .iter()
            .all(|r| r.status == PciRequirementStatus::Compliant)
        {
            PciRequirementStatus::Compliant
        } else if req_list
            .iter()
            .any(|r| r.status == PciRequirementStatus::NotCompliant)
        {
            PciRequirementStatus::NotCompliant
        } else {
            PciRequirementStatus::PartiallyCompliant
        };

        let compliance_score = if req_list.is_empty() {
            0
        } else {
            let compliant_count = req_list
                .iter()
                .filter(|r| r.status == PciRequirementStatus::Compliant)
                .count();
            ((compliant_count as f64) / (req_list.len() as f64) * 100.0) as u8
        };

        let report = PciReport {
            report_id: report_id.clone(),
            report_type: self.config.report_type.clone(),
            requirements: req_list,
            overall_status,
            compliance_score,
            findings: Vec::new(),
            recommendations: Vec::new(),
            assessor_name: self.config.assessor_name.clone(),
            assessment_date: Utc::now(),
            valid_until: Utc::now() + chrono::Duration::days(365),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let mut reports = self.reports.lock().await;
        reports.insert(report_id.clone(), report);

        Ok(report_id)
    }

    /// Get requirement
    pub async fn get_requirement(
        &self,
        req_id: &str,
    ) -> Result<Option<PciRequirement>, VantisError> {
        let requirements = self.requirements.lock().await;
        Ok(requirements.get(req_id).cloned())
    }

    /// Get all requirements
    pub async fn get_all_requirements(&self) -> Vec<PciRequirement> {
        let requirements = self.requirements.lock().await;
        requirements.values().cloned().collect()
    }

    /// Get report
    pub async fn get_report(&self, report_id: &str) -> Result<Option<PciReport>, VantisError> {
        let reports = self.reports.lock().await;
        Ok(reports.get(report_id).cloned())
    }

    /// Get all reports
    pub async fn get_all_reports(&self) -> Vec<PciReport> {
        let reports = self.reports.lock().await;
        reports.values().cloned().collect()
    }

    /// Check compliance
    pub async fn check_compliance(&self) -> Result<u8, VantisError> {
        let requirements = self.requirements.lock().await;
        let req_list: Vec<_> = requirements.values().cloned().collect();

        if req_list.is_empty() {
            return Ok(0);
        }

        let compliant_count = req_list
            .iter()
            .filter(|r| r.status == PciRequirementStatus::Compliant)
            .count();
        let score = ((compliant_count as f64) / (req_list.len() as f64) * 100.0) as u8;

        Ok(score)
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: PciConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &PciConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_dss_compliance_creation() {
        let config = PciConfig::default();
        let pci = PciDssCompliance::new(config);
        assert_eq!(pci.config.report_type, "SAQ A");
    }

    #[test]
    fn test_pci_requirement_status() {
        let status = PciRequirementStatus::Compliant;
        assert_eq!(status, PciRequirementStatus::Compliant);
    }
}
