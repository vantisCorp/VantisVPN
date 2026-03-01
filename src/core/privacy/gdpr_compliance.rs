// GDPR/RODO Compliance
// Phase 5: Privacy & Identity Management
// Implements GDPR/RODO compliance features: consent, data subject rights, right to be forgotten

use crate::error::VantisError;
use crate::crypto::hash::Hash;
use crate::crypto::random::SecureRandom;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};

/// Consent type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentType {
    /// Consent for data collection
    DataCollection,
    /// Consent for data processing
    DataProcessing,
    /// Consent for data sharing
    DataSharing,
    /// Consent for marketing communications
    Marketing,
    /// Consent for analytics
    Analytics,
    /// Consent for cookies
    Cookies,
}

/// Consent status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentStatus {
    /// Consent granted
    Granted,
    /// Consent denied
    Denied,
    /// Consent withdrawn
    Withdrawn,
    /// Consent expired
    Expired,
}

/// Data request type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataRequestType {
    /// Right to access
    Access,
    /// Right to rectification
    Rectification,
    /// Right to erasure (right to be forgotten)
    Erasure,
    /// Right to restriction of processing
    Restriction,
    /// Right to data portability
    Portability,
    /// Right to object
    Object,
}

/// Data request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataRequestStatus {
    /// Request pending
    Pending,
    /// Request in progress
    InProgress,
    /// Request completed
    Completed,
    /// Request rejected
    Rejected,
    /// Request cancelled
    Cancelled,
}

/// Data subject (user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubject {
    /// Subject ID
    pub subject_id: String,
    /// Email address
    pub email: String,
    /// Name
    pub name: String,
    /// Country of residence
    pub country: String,
    /// Date of consent
    pub consent_date: DateTime<Utc>,
    /// Is subject active
    pub is_active: bool,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// Consent record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Consent ID
    pub consent_id: String,
    /// Subject ID
    pub subject_id: String,
    /// Consent type
    pub consent_type: ConsentType,
    /// Consent status
    pub status: ConsentStatus,
    /// Granted at
    pub granted_at: DateTime<Utc>,
    /// Withdrawn at
    pub withdrawn_at: Option<DateTime<Utc>>,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// Consent version
    pub version: u32,
    /// IP address at time of consent
    pub ip_address: String,
    /// User agent
    pub user_agent: String,
}

/// Data request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequest {
    /// Request ID
    pub request_id: String,
    /// Subject ID
    pub subject_id: String,
    /// Request type
    pub request_type: DataRequestType,
    /// Request status
    pub status: DataRequestStatus,
    /// Request description
    pub description: String,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Response data
    pub response_data: Option<String>,
    /// Rejection reason
    pub rejection_reason: Option<String>,
}

/// Right to be forgotten request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightToBeForgotten {
    /// Request ID
    pub request_id: String,
    /// Subject ID
    pub subject_id: String,
    /// Request status
    pub status: DataRequestStatus,
    /// Data categories to delete
    pub data_categories: Vec<String>,
    /// Reason for request
    pub reason: String,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Processed at
    pub processed_at: Option<DateTime<Utc>>,
    /// Verification token
    pub verification_token: String,
}

/// Data portability export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPortability {
    /// Export ID
    pub export_id: String,
    /// Subject ID
    pub subject_id: String,
    /// Export format (JSON, XML, CSV)
    pub format: String,
    /// Export data
    pub data: String,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: DateTime<Utc>,
    /// Download URL
    pub download_url: Option<String>,
}

/// GDPR Compliance configuration
#[derive(Debug, Clone)]
pub struct GdprConfig {
    /// Consent validity period in days
    pub consent_validity_days: u32,
    /// Data retention period in days
    pub data_retention_days: u32,
    /// Request processing time limit in days
    pub request_processing_days: u32,
    /// Enable automatic consent expiration
    pub auto_expire_consent: bool,
    /// Enable data anonymization on deletion
    pub anonymize_on_deletion: bool,
    /// Require explicit consent for marketing
    pub explicit_marketing_consent: bool,
    /// Enable cookie consent banner
    pub cookie_consent_banner: bool,
}

impl Default for GdprConfig {
    fn default() -> Self {
        Self {
            consent_validity_days: 365,
            data_retention_days: 2555, // 7 years
            request_processing_days: 30,
            auto_expire_consent: true,
            anonymize_on_deletion: true,
            explicit_marketing_consent: true,
            cookie_consent_banner: true,
        }
    }
}

/// GDPR Compliance Manager
pub struct GdprCompliance {
    config: GdprConfig,
    subjects: Arc<Mutex<HashMap<String, DataSubject>>>,
    consents: Arc<Mutex<HashMap<String, ConsentRecord>>>,
    requests: Arc<Mutex<HashMap<String, DataRequest>>>,
    rtbf_requests: Arc<Mutex<HashMap<String, RightToBeForgotten>>>,
    exports: Arc<Mutex<HashMap<String, DataPortability>>>,
    rng: Arc<Mutex<SecureRandom>>,
    hash: Arc<Mutex<Hash>>,
}

impl GdprCompliance {
    /// Create a new GDPR Compliance Manager
    pub fn new(config: GdprConfig) -> Result<Self, VantisError> {
        let rng = SecureRandom::new()?;
        let hash = Hash::new()?;
        Ok(Self {
            config,
            subjects: Arc::new(Mutex::new(HashMap::new())),
            consents: Arc::new(Mutex::new(HashMap::new())),
            requests: Arc::new(Mutex::new(HashMap::new())),
            rtbf_requests: Arc::new(Mutex::new(HashMap::new())),
            exports: Arc::new(Mutex::new(HashMap::new())),
            rng: Arc::new(Mutex::new(rng)),
            hash: Arc::new(Mutex::new(hash)),
        })
    }

    /// Register data subject
    pub async fn register_subject(&self, email: String, name: String, country: String) -> Result<String, VantisError> {
        let mut rng = self.rng.lock().await;
        let subject_id = format!("subject_{}", hex::encode(rng.generate_bytes(16)?));
        drop(rng);

        let now = Utc::now();
        let subject = DataSubject {
            subject_id: subject_id.clone(),
            email,
            name,
            country,
            consent_date: now,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        let mut subjects = self.subjects.lock().await;
        subjects.insert(subject_id.clone(), subject);

        Ok(subject_id)
    }

    /// Grant consent
    pub async fn grant_consent(&self, subject_id: &str, consent_type: ConsentType, ip_address: String, user_agent: String) -> Result<String, VantisError> {
        let mut rng = self.rng.lock().await;
        let consent_id = format!("consent_{}", hex::encode(rng.generate_bytes(16)?));
        drop(rng);

        let now = Utc::now();
        let expires_at = if self.config.auto_expire_consent {
            Some(now + Duration::days(self.config.consent_validity_days as i64))
        } else {
            None
        };

        let consent = ConsentRecord {
            consent_id: consent_id.clone(),
            subject_id: subject_id.to_string(),
            consent_type,
            status: ConsentStatus::Granted,
            granted_at: now,
            withdrawn_at: None,
            expires_at,
            version: 1,
            ip_address,
            user_agent,
        };

        let mut consents = self.consents.lock().await;
        consents.insert(consent_id.clone(), consent);

        Ok(consent_id)
    }

    /// Withdraw consent
    pub async fn withdraw_consent(&self, consent_id: &str) -> Result<(), VantisError> {
        let mut consents = self.consents.lock().await;
        let consent = consents.get_mut(consent_id)
            .ok_or_else(|| VantisError::NotFound(format!("Consent {} not found", consent_id)))?;

        consent.status = ConsentStatus::Withdrawn;
        consent.withdrawn_at = Some(Utc::now());

        Ok(())
    }

    /// Check if consent is valid
    pub async fn is_consent_valid(&self, subject_id: &str, consent_type: ConsentType) -> Result<bool, VantisError> {
        let consents = self.consents.lock().await;
        let now = Utc::now();

        for consent in consents.values() {
            if consent.subject_id == subject_id && consent.consent_type == consent_type {
                match consent.status {
                    ConsentStatus::Granted => {
                        if let Some(expires_at) = consent.expires_at {
                            if now < expires_at {
                                return Ok(true);
                            }
                        } else {
                            return Ok(true);
                        }
                    }
                    _ => continue,
                }
            }
        }

        Ok(false)
    }

    /// Create data request
    pub async fn create_data_request(&self, subject_id: &str, request_type: DataRequestType, description: String) -> Result<String, VantisError> {
        let mut rng = self.rng.lock().await;
        let request_id = format!("request_{}", hex::encode(rng.generate_bytes(16)?));
        drop(rng);

        let now = Utc::now();
        let request = DataRequest {
            request_id: request_id.clone(),
            subject_id: subject_id.to_string(),
            request_type,
            status: DataRequestStatus::Pending,
            description,
            created_at: now,
            updated_at: now,
            completed_at: None,
            response_data: None,
            rejection_reason: None,
        };

        let mut requests = self.requests.lock().await;
        requests.insert(request_id.clone(), request);

        Ok(request_id)
    }

    /// Process data request
    pub async fn process_data_request(&self, request_id: &str, response_data: String) -> Result<(), VantisError> {
        let mut requests = self.requests.lock().await;
        let request = requests.get_mut(request_id)
            .ok_or_else(|| VantisError::NotFound(format!("Request {} not found", request_id)))?;

        request.status = DataRequestStatus::Completed;
        request.response_data = Some(response_data);
        request.completed_at = Some(Utc::now());
        request.updated_at = Utc::now();

        Ok(())
    }

    /// Create right to be forgotten request
    pub async fn create_rtbf_request(&self, subject_id: &str, data_categories: Vec<String>, reason: String) -> Result<String, VantisError> {
        let mut rng = self.rng.lock().await;
        let request_id = format!("rtbf_{}", hex::encode(rng.generate_bytes(16)?));
        let verification_token = format!("token_{}", hex::encode(rng.generate_bytes(32)?));
        drop(rng);

        let now = Utc::now();
        let request = RightToBeForgotten {
            request_id: request_id.clone(),
            subject_id: subject_id.to_string(),
            status: DataRequestStatus::Pending,
            data_categories,
            reason,
            created_at: now,
            processed_at: None,
            verification_token,
        };

        let mut rtbf_requests = self.rtbf_requests.lock().await;
        rtbf_requests.insert(request_id.clone(), request);

        Ok(request_id)
    }

    /// Process right to be forgotten request
    pub async fn process_rtbf_request(&self, request_id: &str, verification_token: String) -> Result<(), VantisError> {
        let mut rtbf_requests = self.rtbf_requests.lock().await;
        let request = rtbf_requests.get_mut(request_id)
            .ok_or_else(|| VantisError::NotFound(format!("RTBF request {} not found", request_id)))?;

        if request.verification_token != verification_token {
            return Err(VantisError::AuthenticationFailed("Invalid verification token".to_string()));
        }

        request.status = DataRequestStatus::Completed;
        request.processed_at = Some(Utc::now());

        // In production, this would trigger actual data deletion/anonymization
        if self.config.anonymize_on_deletion {
            // Anonymize subject data
            let mut subjects = self.subjects.lock().await;
            if let Some(subject) = subjects.get_mut(&request.subject_id) {
                subject.is_active = false;
                subject.email = "anonymized@example.com".to_string();
                subject.name = "Anonymized".to_string();
            }
        }

        Ok(())
    }

    /// Export data for portability
    pub async fn export_data(&self, subject_id: &str, format: String) -> Result<String, VantisError> {
        let mut rng = self.rng.lock().await;
        let export_id = format!("export_{}", hex::encode(rng.generate_bytes(16)?));
        drop(rng);

        let now = Utc::now();
        let expires_at = now + Duration::days(7); // Export valid for 7 days

        // Collect subject data
        let subjects = self.subjects.lock().await;
        let subject = subjects.get(subject_id)
            .ok_or_else(|| VantisError::NotFound(format!("Subject {} not found", subject_id)))?;

        let consents = self.consents.lock().await;
        let subject_consents: Vec<_> = consents.values()
            .filter(|c| c.subject_id == subject_id)
            .collect();

        // Create export data
        let data = serde_json::json!({
            "subject": subject,
            "consents": subject_consents,
            "exported_at": now,
        });

        let export_data = match format.as_str() {
            "json" => serde_json::to_string_pretty(&data).unwrap_or_default(),
            "xml" => format!("<!-- XML export -->\n<data>{}</data>", data),
            "csv" => format!("CSV export of data for subject {}", subject_id),
            _ => serde_json::to_string_pretty(&data).unwrap_or_default(),
        };

        let export = DataPortability {
            export_id: export_id.clone(),
            subject_id: subject_id.to_string(),
            format,
            data: export_data,
            created_at: now,
            expires_at,
            download_url: None,
        };

        let mut exports = self.exports.lock().await;
        exports.insert(export_id.clone(), export);

        Ok(export_id)
    }

    /// Get data export
    pub async fn get_export(&self, export_id: &str) -> Result<Option<DataPortability>, VantisError> {
        let exports = self.exports.lock().await;
        Ok(exports.get(export_id).cloned())
    }

    /// Get subject data
    pub async fn get_subject(&self, subject_id: &str) -> Result<Option<DataSubject>, VantisError> {
        let subjects = self.subjects.lock().await;
        Ok(subjects.get(subject_id).cloned())
    }

    /// Get consent records for subject
    pub async fn get_subject_consents(&self, subject_id: &str) -> Vec<ConsentRecord> {
        let consents = self.consents.lock().await;
        consents.values()
            .filter(|c| c.subject_id == subject_id)
            .cloned()
            .collect()
    }

    /// Get data requests for subject
    pub async fn get_subject_requests(&self, subject_id: &str) -> Vec<DataRequest> {
        let requests = self.requests.lock().await;
        requests.values()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: GdprConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &GdprConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdpr_compliance_creation() {
        let config = GdprConfig::default();
        let compliance = GdprCompliance::new(config);
        assert!(compliance.is_ok());
    }

    #[test]
    fn test_consent_record_creation() {
        let consent = ConsentRecord {
            consent_id: "test".to_string(),
            subject_id: "subject_1".to_string(),
            consent_type: ConsentType::DataCollection,
            status: ConsentStatus::Granted,
            granted_at: Utc::now(),
            withdrawn_at: None,
            expires_at: None,
            version: 1,
            ip_address: "127.0.0.1".to_string(),
            user_agent: "Test".to_string(),
        };
        assert_eq!(consent.status, ConsentStatus::Granted);
    }
}