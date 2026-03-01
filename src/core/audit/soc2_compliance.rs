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
pub struct Soc2Control {
    /// Control ID
    pub control_id: String,
    /// Control number
    pub control_number: String,
    /// Trust service criteria
    pub criteria: Soc2TrustServiceCriteria,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Status
    pub status: Soc2ControlStatus,
    /// Evidence
    pub evidence: Vec<String>,
    /// Notes
    pub notes: String,
    /// Last tested
    pub last_tested: Option<DateTime<Utc>>,
    /// Next test
    pub next_test: Option<DateTime<Utc>>,
}

/// SOC 2 report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soc2Report {
    /// Report ID
    pub report_id: String,
    /// Report period start
    pub period_start: DateTime<Utc>,
    /// Report period end
    pub period_end: DateTime<Utc>,
    /// Controls
    pub controls: Vec<Soc2Control>,
    /// Overall status
    pub overall_status: Soc2ControlStatus,
    /// Compliance score (0-100)
    pub compliance_score: u8,
    /// Auditor name
    pub auditor_name: String,
    /// Audit firm
    pub audit_firm: String,
    /// Findings
    pub findings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// SOC 2 configuration
#[derive(Debug, Clone)]
pub struct Soc2Config {
    /// Enable automatic compliance checking
    pub enable_auto_check: bool,
    /// Check interval in days
    pub check_interval_days: u32,
    /// Audit firm
    pub audit_firm: String,
    /// Auditor name
    pub auditor_name: String,
    /// Notify before expiration (days)
    pub notify_before_expiration: u32,
    /// Minimum compliance score required
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

/// SOC 2 Compliance - Service Organization Control 2
pub struct Soc2Compliance {
    config: Soc2Config,
    reports: Arc<Mutex<HashMap<String, Soc2Report>>>,
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