// CSfC Compliance - NSA Commercial Solutions for Classified
// Phase 7: Audit & Certification
// Provides framework for NSA CSfC compliance and component certification

use crate::error::VantisError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Type of NSA CSfC component
///
/// Categories of components within the NSA Commercial Solutions for
/// Classified (CSfC) architecture that must be implemented for certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CsfcComponentType {
    /// Cryptographic module
    CryptoModule,
    /// Network component
    NetworkComponent,
    /// Operating system
    OperatingSystem,
    /// Application
    Application,
    /// Hardware
    Hardware,
}

/// Status of NSA CSfC component
///
/// Current implementation and certification status of a CSfC component,
/// tracking deployment and validation progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CsfcComponentStatus {
    /// Not started
    NotStarted,
    /// In evaluation
    InEvaluation,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Expired
    Expired,
}

/// CSfC component
#[derive(Debug, Clone, Serialize, Deserialize)]
/// CSfC component
///
/// Represents a single Commercial Solutions for Classified (CSfC) component
/// with its certification status and documentation.
pub struct CsfcComponent {
    /// Unique identifier for this component
    pub component_id: String,
    /// Name of the component
    pub name: String,
    /// Type of CSfC component
    pub component_type: CsfcComponentType,
    /// Version of the component
    pub version: String,
    /// Current certification status
    pub status: CsfcComponentStatus,
    /// ID of the certification, if certified
    pub certification_id: Option<String>,
    /// Date from which certification is valid, if applicable
    pub valid_from: Option<DateTime<Utc>>,
    /// Date until which certification is valid, if applicable
    pub valid_until: Option<DateTime<Utc>>,
    /// Vendor or manufacturer of the component
    pub vendor: String,
    /// Documentation references for the component
    pub documentation: Vec<String>,
    /// Timestamp when the component record was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the component record was last updated
    pub updated_at: DateTime<Utc>,
}

/// CSfC compliance report
///
/// Contains a comprehensive assessment of Commercial Solutions for Classified (CSfC)
/// compliance status, including all components, findings, and recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsfcReport {
    /// Unique identifier for this report
    pub report_id: String,
    /// Type of CSfC report
    pub report_type: String,
    /// List of all CSfC components evaluated
    pub components: Vec<CsfcComponent>,
    /// Overall compliance status across all components
    pub overall_status: CsfcComponentStatus,
    /// List of findings and issues discovered
    pub findings: Vec<String>,
    /// Recommendations for improving compliance
    pub recommendations: Vec<String>,
    /// Overall compliance score (0-100)
    pub compliance_score: u8,
    /// Timestamp when the report was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the report was last updated
    pub updated_at: DateTime<Utc>,
}

/// CSfC compliance configuration
///
/// Configuration settings for Commercial Solutions for Classified (CSfC)
/// compliance monitoring and reporting.
#[derive(Debug, Clone)]
pub struct CsfcConfig {
    /// Enable automatic compliance checking on a schedule
    pub enable_auto_check: bool,
    /// Number of days between automatic compliance checks
    pub check_interval_days: u32,
    /// Number of days before expiration to send notifications
    pub notify_before_expiration: u32,
    /// NSA contact email for CSfC certification inquiries
    pub nsa_contact_email: String,
    /// Minimum compliance score (0-100) required for passing
    pub min_compliance_score: u8,
}

impl Default for CsfcConfig {
    fn default() -> Self {
        Self {
            enable_auto_check: true,
            check_interval_days: 30,
            notify_before_expiration: 30,
            nsa_contact_email: "csfc@nsa.gov".to_string(),
            min_compliance_score: 80,
        }
    }
}

/// CSfC Compliance Manager
///
/// Manages Commercial Solutions for Classified (CSfC) compliance monitoring,
/// reporting, and component tracking for the VPN system to ensure compliance
/// with NSA CSfC requirements.
pub struct CsfcCompliance {
    /// Configuration settings for CSfC compliance
    config: CsfcConfig,
    /// Map of report IDs to compliance reports
    reports: Arc<Mutex<HashMap<String, CsfcReport>>>,
    /// Map of component IDs to component details
    components: Arc<Mutex<HashMap<String, CsfcComponent>>>,
}

impl CsfcCompliance {
    /// Create a new CSfC Compliance instance
    pub fn new(config: CsfcConfig) -> Self {
        Self {
            config,
            reports: Arc::new(Mutex::new(HashMap::new())),
            components: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add component
    pub async fn add_component(&self, component: CsfcComponent) -> Result<String, VantisError> {
        let component_id = component.component_id.clone();

        let mut components = self.components.lock().await;
        components.insert(component_id.clone(), component);

        Ok(component_id)
    }

    /// Update component status
    pub async fn update_component_status(
        &self,
        component_id: &str,
        status: CsfcComponentStatus,
        certification_id: Option<String>,
    ) -> Result<(), VantisError> {
        let mut components = self.components.lock().await;
        if let Some(component) = components.get_mut(component_id) {
            component.status = status;
            component.certification_id = certification_id;
            component.updated_at = Utc::now();
            Ok(())
        } else {
            Err(VantisError::NotFound(format!(
                "Component {} not found",
                component_id
            )))
        }
    }

    /// Create compliance report
    pub async fn create_report(&self, report_type: String) -> Result<String, VantisError> {
        let report_id = format!("csfc_{}", Utc::now().timestamp());

        let components = self.components.lock().await;
        let component_list: Vec<_> = components.values().cloned().collect();

        let overall_status = if component_list
            .iter()
            .all(|c| c.status == CsfcComponentStatus::Approved)
        {
            CsfcComponentStatus::Approved
        } else if component_list
            .iter()
            .any(|c| c.status == CsfcComponentStatus::Rejected)
        {
            CsfcComponentStatus::Rejected
        } else {
            CsfcComponentStatus::InEvaluation
        };

        let compliance_score = if component_list.is_empty() {
            0
        } else {
            let approved_count = component_list
                .iter()
                .filter(|c| c.status == CsfcComponentStatus::Approved)
                .count();
            ((approved_count as f64) / (component_list.len() as f64) * 100.0) as u8
        };

        let report = CsfcReport {
            report_id: report_id.clone(),
            report_type,
            components: component_list,
            overall_status,
            findings: Vec::new(),
            recommendations: Vec::new(),
            compliance_score,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let mut reports = self.reports.lock().await;
        reports.insert(report_id.clone(), report);

        Ok(report_id)
    }

    /// Get component
    pub async fn get_component(
        &self,
        component_id: &str,
    ) -> Result<Option<CsfcComponent>, VantisError> {
        let components = self.components.lock().await;
        Ok(components.get(component_id).cloned())
    }

    /// Get all components
    pub async fn get_all_components(&self) -> Vec<CsfcComponent> {
        let components = self.components.lock().await;
        components.values().cloned().collect()
    }

    /// Get report
    pub async fn get_report(&self, report_id: &str) -> Result<Option<CsfcReport>, VantisError> {
        let reports = self.reports.lock().await;
        Ok(reports.get(report_id).cloned())
    }

    /// Get all reports
    pub async fn get_all_reports(&self) -> Vec<CsfcReport> {
        let reports = self.reports.lock().await;
        reports.values().cloned().collect()
    }

    /// Check compliance
    pub async fn check_compliance(&self) -> Result<u8, VantisError> {
        let components = self.components.lock().await;
        let component_list: Vec<_> = components.values().cloned().collect();

        if component_list.is_empty() {
            return Ok(0);
        }

        let approved_count = component_list
            .iter()
            .filter(|c| c.status == CsfcComponentStatus::Approved)
            .count();
        let score = ((approved_count as f64) / (component_list.len() as f64) * 100.0) as u8;

        Ok(score)
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: CsfcConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &CsfcConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csfc_compliance_creation() {
        let config = CsfcConfig::default();
        let csfc = CsfcCompliance::new(config);
        assert_eq!(csfc.config.min_compliance_score, 80);
    }

    #[test]
    fn test_csfc_component_status() {
        let status = CsfcComponentStatus::Approved;
        assert_eq!(status, CsfcComponentStatus::Approved);
    }
}
