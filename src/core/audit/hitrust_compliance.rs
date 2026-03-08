// HITRUST CSF Compliance - Health Information Trust Alliance Common Security Framework
// Phase 7: Audit & Certification
// Provides framework for HITRUST CSF compliance and control tracking

use crate::error::VantisError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Category of HITRUST CSF control
///
/// Classification of security controls within the HITRUST CSF framework,
/// organizing requirements by domain and function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitrustControlCategory {
    /// Access Control
    AccessControl,
    /// Asset Management
    AssetManagement,
    /// Asset Management and Inventory
    AssetManagementAndInventory,
    /// Business Continuity
    BusinessContinuity,
    /// Communication and Training
    CommunicationAndTraining,
    /// Configuration Management
    ConfigurationManagement,
    /// Identification and Authentication
    IdentificationAndAuthentication,
    /// Incident Management
    IncidentManagement,
    /// Malicious Software Protection
    MaliciousSoftwareProtection,
    /// Network Protection
    NetworkProtection,
    /// Physical and Environmental Security
    PhysicalAndEnvironmentalSecurity,
    /// Risk Management
    RiskManagement,
    /// Security Management Process
    SecurityManagementProcess,
    /// System and Communications Protection
    SystemAndCommunicationsProtection,
    /// Supply Chain Risk Management
    SupplyChainRiskManagement,
    /// System and Information Acquisition
    SystemAndInformationAcquisition,
}

/// Status of HITRUST CSF control
///
/// Current implementation and compliance status of a HITRUST control,
/// tracking assessment results and remediation progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitrustControlStatus {
    /// Not implemented
    NotImplemented,
    /// Partially implemented
    PartiallyImplemented,
    /// Implemented
    Implemented,
    /// Transmitted
    Transmitted,
    /// Validated
    Validated,
}

/// HITRUST control
#[derive(Debug, Clone, Serialize, Deserialize)]
/// HITRUST CSF control
///
/// Represents a single HITRUST CSF control with its implementation status,
/// evidence, and assessment details.
pub struct HitrustControl {
    /// Unique identifier for this control
    pub control_id: String,
    /// Control number from the HITRUST CSF
    pub control_number: String,
    /// Category this control belongs to
    pub category: HitrustControlCategory,
    /// Title of the control
    pub title: String,
    /// Detailed description of the control requirements
    pub description: String,
    /// Current implementation status
    pub status: HitrustControlStatus,
    /// Level of implementation (0-100)
    pub implementation_level: u8,
    /// Evidence collected to support implementation
    pub evidence: Vec<String>,
    /// Additional notes or observations
    pub notes: String,
    /// Timestamp of last update
    pub last_updated: DateTime<Utc>,
}

/// HITRUST CSF compliance report
///
/// Contains a comprehensive assessment of HITRUST CSF compliance status,
/// including all controls, findings, and recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitrustReport {
    /// Unique identifier for this report
    pub report_id: String,
    /// Report type (e.g., "CSF Certification", "CSF Assessment")
    pub report_type: String,
    /// List of all HITRUST CSF controls assessed
    pub controls: Vec<HitrustControl>,
    /// Overall compliance status across all controls
    pub overall_status: HitrustControlStatus,
    /// Overall compliance score (0-100)
    pub compliance_score: u8,
    /// Name of the assessor who conducted the review
    pub assessor_name: String,
    /// Date when the assessment was conducted
    pub assessment_date: DateTime<Utc>,
    /// Date until which this report is valid
    pub valid_until: DateTime<Utc>,
    /// List of findings and issues discovered
    pub findings: Vec<String>,
    /// Recommendations for improving compliance
    pub recommendations: Vec<String>,
    /// Timestamp when the report was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the report was last updated
    pub updated_at: DateTime<Utc>,
}

/// HITRUST CSF compliance configuration
///
/// Configuration settings for HITRUST CSF compliance monitoring and reporting,
/// including automatic checking intervals and notification settings.
#[derive(Debug, Clone)]
pub struct HitrustConfig {
    /// Enable automatic compliance checking on a schedule
    pub enable_auto_check: bool,
    /// Number of days between automatic compliance checks
    pub check_interval_days: u32,
    /// Type of HITRUST report to generate (CSF Certification, CSF Assessment, etc.)
    pub report_type: String,
    /// Name of the HITRUST CSF assessor
    pub assessor_name: String,
    /// Number of days before expiration to send notifications
    pub notify_before_expiration: u32,
    /// Minimum compliance score (0-100) required for passing
    pub min_compliance_score: u8,
}

impl Default for HitrustConfig {
    fn default() -> Self {
        Self {
            enable_auto_check: true,
            check_interval_days: 90,
            report_type: "CSF Certification".to_string(),
            assessor_name: "HITRUST CSF Assessor".to_string(),
            notify_before_expiration: 30,
            min_compliance_score: 80,
        }
    }
}

/// HITRUST CSF Compliance Manager
///
/// Manages HITRUST CSF compliance monitoring, reporting, and control tracking
/// for the VPN system to ensure compliance with healthcare security standards.
pub struct HitrustCompliance {
    /// Configuration settings for HITRUST CSF compliance
    config: HitrustConfig,
    /// Map of report IDs to compliance reports
    reports: Arc<Mutex<HashMap<String, HitrustReport>>>,
    /// Map of control IDs to control details
    controls: Arc<Mutex<HashMap<String, HitrustControl>>>,
}

impl HitrustCompliance {
    /// Create a new HITRUST CSF Compliance instance
    pub fn new(config: HitrustConfig) -> Self {
        Self {
            config,
            reports: Arc::new(Mutex::new(HashMap::new())),
            controls: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add control
    pub async fn add_control(&self, control: HitrustControl) -> Result<String, VantisError> {
        let control_id = control.control_id.clone();

        let mut controls = self.controls.lock().await;
        controls.insert(control_id.clone(), control);

        Ok(control_id)
    }

    /// Update control status
    pub async fn update_control_status(
        &self,
        control_id: &str,
        status: HitrustControlStatus,
        implementation_level: u8,
        evidence: Vec<String>,
        notes: String,
    ) -> Result<(), VantisError> {
        let mut controls = self.controls.lock().await;
        if let Some(control) = controls.get_mut(control_id) {
            control.status = status;
            control.implementation_level = implementation_level;
            control.evidence = evidence;
            control.notes = notes;
            control.last_updated = Utc::now();
            Ok(())
        } else {
            Err(VantisError::NotFound(format!(
                "Control {} not found",
                control_id
            )))
        }
    }

    /// Create compliance report
    pub async fn create_report(&self) -> Result<String, VantisError> {
        let report_id = format!("hitrust_{}", Utc::now().timestamp());

        let controls = self.controls.lock().await;
        let control_list: Vec<_> = controls.values().cloned().collect();

        let overall_status = if control_list
            .iter()
            .all(|c| c.status == HitrustControlStatus::Validated)
        {
            HitrustControlStatus::Validated
        } else if control_list
            .iter()
            .any(|c| c.status == HitrustControlStatus::NotImplemented)
        {
            HitrustControlStatus::NotImplemented
        } else {
            HitrustControlStatus::Implemented
        };

        let compliance_score = if control_list.is_empty() {
            0
        } else {
            let validated_count = control_list
                .iter()
                .filter(|c| c.status == HitrustControlStatus::Validated)
                .count();
            ((validated_count as f64) / (control_list.len() as f64) * 100.0) as u8
        };

        let report = HitrustReport {
            report_id: report_id.clone(),
            report_type: self.config.report_type.clone(),
            controls: control_list,
            overall_status,
            compliance_score,
            assessor_name: self.config.assessor_name.clone(),
            assessment_date: Utc::now(),
            valid_until: Utc::now() + chrono::Duration::days(365),
            findings: Vec::new(),
            recommendations: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let mut reports = self.reports.lock().await;
        reports.insert(report_id.clone(), report);

        Ok(report_id)
    }

    /// Get control
    pub async fn get_control(
        &self,
        control_id: &str,
    ) -> Result<Option<HitrustControl>, VantisError> {
        let controls = self.controls.lock().await;
        Ok(controls.get(control_id).cloned())
    }

    /// Get all controls
    pub async fn get_all_controls(&self) -> Vec<HitrustControl> {
        let controls = self.controls.lock().await;
        controls.values().cloned().collect()
    }

    /// Get report
    pub async fn get_report(&self, report_id: &str) -> Result<Option<HitrustReport>, VantisError> {
        let reports = self.reports.lock().await;
        Ok(reports.get(report_id).cloned())
    }

    /// Get all reports
    pub async fn get_all_reports(&self) -> Vec<HitrustReport> {
        let reports = self.reports.lock().await;
        reports.values().cloned().collect()
    }

    /// Check compliance
    pub async fn check_compliance(&self) -> Result<u8, VantisError> {
        let controls = self.controls.lock().await;
        let control_list: Vec<_> = controls.values().cloned().collect();

        if control_list.is_empty() {
            return Ok(0);
        }

        let validated_count = control_list
            .iter()
            .filter(|c| c.status == HitrustControlStatus::Validated)
            .count();
        let score = ((validated_count as f64) / (control_list.len() as f64) * 100.0) as u8;

        Ok(score)
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: HitrustConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &HitrustConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hitrust_compliance_creation() {
        let config = HitrustConfig::default();
        let hitrust = HitrustCompliance::new(config);
        assert_eq!(hitrust.config.report_type, "CSF Certification");
    }

    #[test]
    fn test_hitrust_control_status() {
        let status = HitrustControlStatus::Validated;
        assert_eq!(status, HitrustControlStatus::Validated);
    }
}
