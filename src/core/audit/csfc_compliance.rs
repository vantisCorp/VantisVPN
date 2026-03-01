// CSfC Compliance - NSA Commercial Solutions for Classified
// Phase 7: Audit & Certification
// Provides framework for NSA CSfC compliance and component certification

use crate::error::VantisError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// CSfC component type
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

/// CSfC component status
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
pub struct CsfcComponent {
    /// Component ID
    pub component_id: String,
    /// Component name
    pub name: String,
    /// Component type
    pub component_type: CsfcComponentType,
    /// Version
    pub version: String,
    /// Status
    pub status: CsfcComponentStatus,
    /// Certification ID
    pub certification_id: Option<String>,
    /// Valid from
    pub valid_from: Option<DateTime<Utc>>,
    /// Valid until
    pub valid_until: Option<DateTime<Utc>>,
    /// Vendor
    pub vendor: String,
    /// Documentation
    pub documentation: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// CSfC report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsfcReport {
    /// Report ID
    pub report_id: String,
    /// Report type
    pub report_type: String,
    /// Components evaluated
    pub components: Vec<CsfcComponent>,
    /// Overall compliance status
    pub overall_status: CsfcComponentStatus,
    /// Findings
    pub findings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Compliance score (0-100)
    pub compliance_score: u8,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// CSfC configuration
#[derive(Debug, Clone)]
pub struct CsfcConfig {
    /// Enable automatic compliance checking
    pub enable_auto_check: bool,
    /// Check interval in days
    pub check_interval_days: u32,
    /// Notify before expiration (days)
    pub notify_before_expiration: u32,
    /// NSA contact email
    pub nsa_contact_email: String,
    /// Minimum compliance score required
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

/// CSfC Compliance - NSA Commercial Solutions for Classified
pub struct CsfcCompliance {
    config: CsfcConfig,
    reports: Arc<Mutex<HashMap<String, CsfcReport>>>,
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
    pub async fn update_component_status(&self, component_id: &str, status: CsfcComponentStatus, certification_id: Option<String>) -> Result<(), VantisError> {
        let mut components = self.components.lock().await;
        if let Some(component) = components.get_mut(component_id) {
            component.status = status;
            component.certification_id = certification_id;
            component.updated_at = Utc::now();
            Ok(())
        } else {
            Err(VantisError::NotFound(format!("Component {} not found", component_id)))
        }
    }

    /// Create compliance report
    pub async fn create_report(&self, report_type: String) -> Result<String, VantisError> {
        let report_id = format!("csfc_{}", Utc::now().timestamp());

        let components = self.components.lock().await;
        let component_list: Vec<_> = components.values().cloned().collect();

        let overall_status = if component_list.iter().all(|c| c.status == CsfcComponentStatus::Approved) {
            CsfcComponentStatus::Approved
        } else if component_list.iter().any(|c| c.status == CsfcComponentStatus::Rejected) {
            CsfcComponentStatus::Rejected
        } else {
            CsfcComponentStatus::InEvaluation
        };

        let compliance_score = if component_list.is_empty() {
            0
        } else {
            let approved_count = component_list.iter().filter(|c| c.status == CsfcComponentStatus::Approved).count();
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
    pub async fn get_component(&self, component_id: &str) -> Result<Option<CsfcComponent>, VantisError> {
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

        let approved_count = component_list.iter().filter(|c| c.status == CsfcComponentStatus::Approved).count();
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