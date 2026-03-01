// PCI DSS Compliance - Payment Card Industry Data Security Standard
// Phase 7: Audit & Certification
// Provides framework for PCI DSS compliance and requirement tracking

use crate::error::VantisError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// PCI DSS requirement status
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciRequirement {
    /// Requirement ID
    pub requirement_id: String,
    /// Requirement number (e.g., "1.1", "2.3")
    pub requirement_number: String,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Status
    pub status: PciRequirementStatus,
    /// Evidence
    pub evidence: Vec<String>,
    /// Notes
    pub notes: String,
    /// Last reviewed
    pub last_reviewed: Option<DateTime<Utc>>,
    /// Next review
    pub next_review: Option<DateTime<Utc>>,
}

/// PCI DSS report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciReport {
    /// Report ID
    pub report_id: String,
    /// Report type (SAQ A, SAQ D, ROC, etc.)
    pub report_type: String,
    /// Requirements
    pub requirements: Vec<PciRequirement>,
    /// Overall compliance status
    pub overall_status: PciRequirementStatus,
    /// Compliance score (0-100)
    pub compliance_score: u8,
    /// Findings
    pub findings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Assessor name
    pub assessor_name: String,
    /// Assessment date
    pub assessment_date: DateTime<Utc>,
    /// Valid until
    pub valid_until: DateTime<Utc>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// PCI DSS configuration
#[derive(Debug, Clone)]
pub struct PciConfig {
    /// Enable automatic compliance checking
    pub enable_auto_check: bool,
    /// Check interval in days
    pub check_interval_days: u32,
    /// Report type
    pub report_type: String,
    /// Assessor name
    pub assessor_name: String,
    /// Notify before expiration (days)
    pub notify_before_expiration: u32,
    /// Minimum compliance score required
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

/// PCI DSS Compliance - Payment Card Industry Data Security Standard
pub struct PciDssCompliance {
    config: PciConfig,
    reports: Arc<Mutex<HashMap<String, PciReport>>>,
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
    pub async fn add_requirement(&self, requirement: PciRequirement) -> Result<String, VantisError> {
        let req_id = requirement.requirement_id.clone();

        let mut requirements = self.requirements.lock().await;
        requirements.insert(req_id.clone(), requirement);

        Ok(req_id)
    }

    /// Update requirement status
    pub async fn update_requirement_status(&self, req_id: &str, status: PciRequirementStatus, evidence: Vec<String>, notes: String) -> Result<(), VantisError> {
        let mut requirements = self.requirements.lock().await;
        if let Some(req) = requirements.get_mut(req_id) {
            req.status = status;
            req.evidence = evidence;
            req.notes = notes;
            req.last_reviewed = Some(Utc::now());
            req.next_review = Some(Utc::now() + chrono::Duration::days(90));
            Ok(())
        } else {
            Err(VantisError::NotFound(format!("Requirement {} not found", req_id)))
        }
    }

    /// Create compliance report
    pub async fn create_report(&self) -> Result<String, VantisError> {
        let report_id = format!("pci_{}", Utc::now().timestamp());

        let requirements = self.requirements.lock().await;
        let req_list: Vec<_> = requirements.values().cloned().collect();

        let overall_status = if req_list.iter().all(|r| r.status == PciRequirementStatus::Compliant) {
            PciRequirementStatus::Compliant
        } else if req_list.iter().any(|r| r.status == PciRequirementStatus::NotCompliant) {
            PciRequirementStatus::NotCompliant
        } else {
            PciRequirementStatus::PartiallyCompliant
        };

        let compliance_score = if req_list.is_empty() {
            0
        } else {
            let compliant_count = req_list.iter().filter(|r| r.status == PciRequirementStatus::Compliant).count();
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
    pub async fn get_requirement(&self, req_id: &str) -> Result<Option<PciRequirement>, VantisError> {
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

        let compliant_count = req_list.iter().filter(|r| r.status == PciRequirementStatus::Compliant).count();
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