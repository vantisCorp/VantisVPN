// Biometric Authorization
// Phase 6: UX/UI & Additional Features
// Provides biometric authentication support

use crate::error::VantisError;
use crate::crypto::hash::Hash;
use crate::crypto::random::SecureRandom;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Type of biometric authentication method
///
/// Different biometric modalities supported for authentication,
/// including fingerprint, facial recognition, and voice analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiometricType {
    /// Fingerprint
    Fingerprint,
    /// Face recognition
    Face,
    /// Iris scan
    Iris,
    /// Voice recognition
    Voice,
    /// Palm print
    Palm,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Biometric authentication result
/// 
/// Contains the result of a biometric authentication attempt, including
/// success status, confidence score, and error information.
pub struct AuthResult {
    /// Whether authentication was successful
    pub success: bool,
    /// Confidence score of the match (0.0-1.0)
    pub confidence: f64,
    /// Type of biometric method used
    pub method: BiometricType,
    /// Timestamp when authentication was performed
    pub timestamp: DateTime<Utc>,
    /// Error message if authentication failed
    pub error_message: Option<String>,
}

/// Biometric template
/// 
/// Represents a stored biometric template for authentication,
/// containing encrypted biometric data and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricTemplate {
    /// Unique identifier for this template
    pub template_id: String,
    /// ID of the user this template belongs to
    pub user_id: String,
    /// Type of biometric data stored
    pub biometric_type: BiometricType,
    /// Encrypted biometric template data
    pub template_data: Vec<u8>,
    /// Timestamp when the template was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when this template was last used
    pub last_used: Option<DateTime<Utc>>,
    /// Whether this template is currently active
    pub is_active: bool,
}

/// Biometric authentication configuration
/// 
/// Configuration settings for biometric authentication, including
/// confidence thresholds, lockout policies, and multi-factor options.
#[derive(Debug, Clone)]
pub struct BiometricConfig {
    /// Enable biometric authentication
    pub enabled: bool,
    /// Minimum confidence threshold required (0.0-1.0)
    pub confidence_threshold: f64,
    /// Maximum number of failed attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration in seconds after too many failures
    pub lockout_duration: u64,
    /// Enable multi-factor biometric authentication
    pub enable_multi_factor: bool,
    /// Required biometric types for multi-factor authentication
    pub required_types: Vec<BiometricType>,
    /// Enable fallback to password authentication
    pub enable_password_fallback: bool,
}

impl Default for BiometricConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            confidence_threshold: 0.85,
            max_failed_attempts: 5,
            lockout_duration: 900, // 15 minutes
            enable_multi_factor: false,
            required_types: vec![BiometricType::Fingerprint],
            enable_password_fallback: true,
        }
    }
}

/// Biometric Auth - Biometric Authorization
/// Biometric authentication manager
///
/// Manages biometric authentication operations, including template storage,
/// authentication attempts, and lockout policies.
pub struct BiometricAuth {
    config: BiometricConfig,
    templates: Arc<Mutex<HashMap<String, BiometricTemplate>>>,
    failed_attempts: Arc<Mutex<HashMap<String, u32>>>,
    lockout_until: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
    rng: Arc<Mutex<SecureRandom>>,
    hash: Arc<Mutex<Hash>>,
}

impl BiometricAuth {
    /// Create a new Biometric Auth instance
    pub fn new(config: BiometricConfig) -> Result<Self, VantisError> {
        let rng = SecureRandom::new()?;
        let hash = Hash::new()?;
        Ok(Self {
            config,
            templates: Arc::new(Mutex::new(HashMap::new())),
            failed_attempts: Arc::new(Mutex::new(HashMap::new())),
            lockout_until: Arc::new(Mutex::new(HashMap::new())),
            rng: Arc::new(Mutex::new(rng)),
            hash: Arc::new(Mutex::new(hash)),
        })
    }

    /// Register biometric template
    pub async fn register_template(&self, user_id: String, biometric_type: BiometricType, template_data: Vec<u8>) -> Result<String, VantisError> {
        let rng = self.rng.lock().await;
        let template_id = format!("bio_{}", hex::encode(rng.generate_bytes(16)?));
        drop(rng);

        let template = BiometricTemplate {
            template_id: template_id.clone(),
            user_id,
            biometric_type,
            template_data,
            created_at: Utc::now(),
            last_used: None,
            is_active: true,
        };

        let mut templates = self.templates.lock().await;
        templates.insert(template_id.clone(), template);

        Ok(template_id)
    }

    /// Authenticate user
    pub async fn authenticate(&self, user_id: String, biometric_type: BiometricType, sample_data: Vec<u8>) -> Result<AuthResult, VantisError> {
        // Check lockout status
        {
            let lockout = self.lockout_until.lock().await;
            if let Some(&until) = lockout.get(&user_id) {
                if Utc::now() < until {
                    return Ok(AuthResult {
                        success: false,
                        confidence: 0.0,
                        method: biometric_type,
                        timestamp: Utc::now(),
                        error_message: Some("Account locked due to too many failed attempts".to_string()),
                    });
                }
            }
        }

        // Find matching template
        let templates = self.templates.lock().await;
        let template = templates.values()
            .find(|t| t.user_id == user_id && t.biometric_type == biometric_type && t.is_active)
            .cloned();
        drop(templates);

        let template = match template {
            Some(t) => t,
            None => {
                return Ok(AuthResult {
                    success: false,
                    confidence: 0.0,
                    method: biometric_type,
                    timestamp: Utc::now(),
                    error_message: Some("No biometric template found".to_string()),
                });
            }
        };

        // Compare biometric data (placeholder - in production, use actual biometric matching)
        let confidence = self.compare_biometric_data(&sample_data, &template.template_data)?;

        let success = confidence >= self.config.confidence_threshold;

        if success {
            // Reset failed attempts
            let mut failed = self.failed_attempts.lock().await;
            failed.remove(&user_id);

            // Update last used
            let mut templates = self.templates.lock().await;
            if let Some(t) = templates.get_mut(&template.template_id) {
                t.last_used = Some(Utc::now());
            }

            Ok(AuthResult {
                success: true,
                confidence,
                method: biometric_type,
                timestamp: Utc::now(),
                error_message: None,
            })
        } else {
            // Increment failed attempts
            let mut failed = self.failed_attempts.lock().await;
            let attempts = failed.entry(user_id.clone()).or_insert(0);
            *attempts += 1;

            // Check if should lockout
            if *attempts >= self.config.max_failed_attempts {
                let mut lockout = self.lockout_until.lock().await;
                lockout.insert(user_id.clone(), Utc::now() + chrono::Duration::seconds(self.config.lockout_duration as i64));
            }

            Ok(AuthResult {
                success: false,
                confidence,
                method: biometric_type,
                timestamp: Utc::now(),
                error_message: Some("Biometric verification failed".to_string()),
            })
        }
    }

    /// Compare biometric data (placeholder implementation)
    fn compare_biometric_data(&self, _sample: &[u8], _template: &[u8]) -> Result<f64, VantisError> {
        // In production, this would use actual biometric matching algorithms
        // For now, return a random confidence score
        let rng = self.rng.blocking_lock();
        let random_bytes = rng.generate_bytes(8)?;
        let confidence = (u64::from_be_bytes([random_bytes[0], random_bytes[1], random_bytes[2], random_bytes[3], 
                                                      random_bytes[4], random_bytes[5], random_bytes[6], random_bytes[7]]) as f64) / u64::MAX as f64;
        
        // Bias towards higher confidence for testing
        Ok(0.7 + confidence * 0.3)
    }

    /// Remove biometric template
    pub async fn remove_template(&self, template_id: &str) -> Result<(), VantisError> {
        let mut templates = self.templates.lock().await;
        templates.remove(template_id);
        Ok(())
    }

    /// Get user templates
    pub async fn get_user_templates(&self, user_id: &str) -> Vec<BiometricTemplate> {
        let templates = self.templates.lock().await;
        templates.values()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: BiometricConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &BiometricConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biometric_auth_creation() {
        let config = BiometricConfig::default();
        let auth = BiometricAuth::new(config);
        assert!(auth.is_ok());
    }

    #[test]
    fn test_auth_result_creation() {
        let result = AuthResult {
            success: true,
            confidence: 0.95,
            method: BiometricType::Fingerprint,
            timestamp: Utc::now(),
            error_message: None,
        };
        assert!(result.success);
        assert_eq!(result.confidence, 0.95);
    }
}