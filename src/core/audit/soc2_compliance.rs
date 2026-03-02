// SOC 2 Type II Compliance - Service Organization Control 2
// Phase 7: Audit & Certification
// Provides framework for SOC 2 Type II compliance and control tracking

use crate::error::VantisError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// SOC 2 trust service criteria
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Soc2TrustServiceCriteria {
    /// Security
    Security,
    /// Availability
    Availability,
    /// Processing Integrity
    ProcessingIntegrity,
    /// Confidentiality
    Confidentiality,
    /// Privacy
    Privacy,
}

/// SOC 2 control status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Soc2ControlStatus {
    /// Not implemented
    NotImplemented,
    /// Partially implemented
    PartiallyImplemented,
    /// Implemented
    Implemented,
    /// Operating effectively
    OperatingEffectively,
}

/// SOC 2 control
#[derive(Debug, Clone, Serialize, Deserialize)]
/// SOC 2 control
/// 
/// Represents a single SOC 2 control with its implementation status,
/// evidence, and testing schedule.
pub struct Soc2Control {
    /// Unique identifier for this control
    pub control_id: String,
    /// Control number from the SOC 2 framework
    pub control_number: String,
    /// Trust service criteria this control addresses
    pub criteria: Soc2TrustServiceCriteria,
    /// Title of the control
    pub title: String,
    /// Detailed description of the control requirements
    pub description: String,
    /// Current implementation status
    pub status: Soc2ControlStatus,
    /// Evidence collected to support implementation
    pub evidence: Vec<String>,
    /// Additional notes or observations
    pub notes: String,
    /// Timestamp of last test, if any
    pub last_tested: Option<DateTime<Utc>>,
    /// Scheduled date for next test, if any
    pub next_test: Option<DateTime<Utc>>,
}

/// SOC 2 compliance report
/// 
/// Contains a comprehensive assessment of SOC 2 compliance status,
/// including all controls, findings, and recommendations for a specified period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soc2Report {
    /// Unique identifier for this report
    pub report_id: String,
    /// Start date of the audit period
    pub period_start: DateTime<Utc>,
    /// End date of the audit period
    pub period_end: DateTime<Utc>,
    /// List of all SOC 2 controls assessed
    pub controls: Vec<Soc2Control>,
    /// Overall compliance status across all controls
    pub overall_status: Soc2ControlStatus,
    /// Overall compliance score (0-100)
    pub compliance_score: u8,
    /// Name of the auditor who conducted the review
    pub auditor_name: String,
    /// Name of the audit firm
    pub audit_firm: String,
    /// List of findings and issues discovered
    pub findings: Vec<String>,
    /// Recommendations for improving compliance
    pub recommendations: Vec<String>,
    /// Timestamp when the report was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the report was last updated
    pub updated_at: DateTime<Utc>,
}

/// SOC 2 compliance configuration
/// 
/// Configuration settings for SOC 2 compliance monitoring and reporting,
/// including automatic checking intervals and notification settings.
#[derive(Debug, Clone)]
pub struct Soc2Config {
    /// Enable automatic compliance checking on a schedule
    pub enable_auto_check: bool,
    /// Number of days between automatic compliance checks
    pub check_interval_days: u32,
    /// Name of the audit firm conducting the assessment
    pub audit_firm: String,
    /// Name of the qualified SOC 2 auditor
    pub auditor_name: String,
    /// Number of days before expiration to send notifications
    pub notify_before_expiration: u32,
    /// Minimum compliance score (0-100) required for passing
    pub min_compliance_score: u8,
}

impl Default for Soc2Config {
    fn default() -> Self {
        Self {
            enable_auto_check: true,
            check_interval_days: 90,
            audit_firm: "Big Four".to_string(),
            auditor_name: "Qualified SOC 2 Auditor".to_string(),
            notify_before_expiration: 30,
            min_compliance_score: 80,
        }
    }
}

/// SOC 2 Compliance Manager
/// 
/// Manages SOC 2 compliance monitoring, reporting, and control tracking
/// for the VPN system to ensure compliance with SOC 2 trust service criteria.
pub struct Soc2Compliance {
    /// Configuration settings for SOC 2 compliance
    config: Soc2Config,
    /// Map of report IDs to compliance reports
    reports: Arc<Mutex<HashMap<String, Soc2Report>>>,
    /// Map of control IDs to control details
    controls: Arc<Mutex<HashMap<String, Soc2Control>>>,
}

impl Soc2Compliance {
    /// Create a new SOC 2 Compliance instance
    pub fn new(config: Soc2Config) -> Self {
        Self {
            config,
            reports: Arc::new(Mutex::new(HashMap::new())),
            controls: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add control
    pub async fn add_control(&self, control: Soc2Control) -> Result<String, VantisError> {
        let control_id = control.control_id.clone();

        let mut controls = self.controls.lock().await;
        controls.insert(control_id.clone(), control);

        Ok(control_id)
    }

    /// Update control status
    pub async fn update_control_status(&self, control_id: &str, status: Soc2ControlStatus, evidence: Vec<String>, notes: String) -> Result<(), VantisError> {
        let mut controls = self.controls.lock().await;
        if let Some(control) = controls.get_mut(control_id) {
            control.status = status;
            control.evidence = evidence;
            control.notes = notes;
            control.last_tested = Some(Utc::now());
            control.next_test = Some(Utc::now() + chrono::Duration::days(90));
            Ok(())
        } else {
            Err(VantisError::NotFound(format!("Control {} not found", control_id)))
        }
    }

    /// Create compliance report
    pub async fn create_report(&self, period_start: DateTime<Utc>, period_end: DateTime<Utc>) -> Result<String, VantisError> {
        let report_id = format!("soc2_{}", Utc::now().timestamp());

        let controls = self.controls.lock().await;
        let control_list: Vec<_> = controls.values().cloned().collect();

        let overall_status = if control_list.iter().all(|c| c.status == Soc2ControlStatus::OperatingEffectively) {
            Soc2ControlStatus::OperatingEffectively
        } else if control_list.iter().any(|c| c.status == Soc2ControlStatus::NotImplemented) {
            Soc2ControlStatus::NotImplemented
        } else {
            Soc2ControlStatus::PartiallyImplemented
        };

        let compliance_score = if control_list.is_empty() {
            0
        } else {
            let effective_count = control_list.iter().filter(|c| c.status == Soc2ControlStatus::OperatingEffectively).count();
            ((effective_count as f64) / (control_list.len() as f64) * 100.0) as u8
        };

        let report = Soc2Report {
            report_id: report_id.clone(),
            period_start,
            period_end,
            controls: control_list,
            overall_status,
            compliance_score,
            auditor_name: self.config.auditor_name.clone(),
            audit_firm: self.config.audit_firm.clone(),
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
    pub async fn get_control(&self, control_id: &str) -> Result<Option<Soc2Control>, VantisError> {
        let controls = self.controls.lock().await;
        Ok(controls.get(control_id).cloned())
    }

    /// Get all controls
    pub async fn get_all_controls(&self) -> Vec<Soc2Control> {
        let controls = self.controls.lock().await;
        controls.values().cloned().collect()
    }

    /// Get report
    pub async fn get_report(&self, report_id: &str) -> Result<Option<Soc2Report>, VantisError> {
        let reports = self.reports.lock().await;
        Ok(reports.get(report_id).cloned())
    }

    /// Get all reports
    pub async fn get_all_reports(&self) -> Vec<Soc2Report> {
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

        let effective_count = control_list.iter().filter(|c| c.status == Soc2ControlStatus::OperatingEffectively).count();
        let score = ((effective_count as f64) / (control_list.len() as f64) * 100.0) as u8;

        Ok(score)
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: Soc2Config) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &Soc2Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soc2_compliance_creation() {
        let config = Soc2Config::default();
        let soc2 = Soc2Compliance::new(config);
        assert_eq!(soc2.config.audit_firm, "Big Four");
    }

    #[test]
    fn test_soc2_control_status() {
        let status = Soc2ControlStatus::OperatingEffectively;
        assert_eq!(status, Soc2ControlStatus::OperatingEffectively);
    }
}