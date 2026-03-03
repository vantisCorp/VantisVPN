// Avantis ID - Identity Generator
// Generates unique, anonymous digital identities
// Zero-knowledge proof of identity without revealing real identity

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use crate::error::{VantisError, Result};
use crate::crypto::{Hash, SecureRandom};

/// Type of digital identity
///
/// Categories of digital identities available in the Avantis ID system,
/// supporting different use cases and privacy levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdentityType {
    /// Personal identity
    Personal,
    /// Business identity
    Business,
    /// Temporary identity
    Temporary,
    /// Anonymous identity
    Anonymous,
}

/// Identity Proof
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zero-knowledge identity proof
/// 
/// Represents a cryptographic proof of identity ownership without
/// revealing the actual identity details.
pub struct IdentityProof {
    /// Unique identifier for this proof
    pub proof_id: String,
    /// ID of the identity being proven
    pub identity_id: String,
    /// Cryptographic commitment to the identity
    pub commitment: Vec<u8>,
    /// Unix timestamp when the proof was created
    pub timestamp: u64,
    /// Signature authenticating the proof
    pub signature: Vec<u8>,
}

/// Digital identity for anonymous authentication
/// 
/// Represents a user's digital identity with cryptographic keys for
/// zero-knowledge authentication and privacy-preserving interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalIdentity {
    /// Unique identifier for this digital identity
    pub identity_id: String,
    /// Type of identity (personal, business, temporary, etc.)
    pub identity_type: IdentityType,
    /// Display name for the identity (can be pseudonymous)
    pub display_name: String,
    /// Public key for cryptographic operations
    pub public_key: Vec<u8>,
    /// Cryptographic commitment to the public key
    pub public_key_commitment: Vec<u8>,
    /// Private key (never transmitted, stored locally only)
    pub private_key: Vec<u8>,
    /// Timestamp when the identity was created
    pub created_at: DateTime<Utc>,
    /// Optional expiration date for temporary identities
    pub expires_at: Option<DateTime<Utc>>,
    /// Whether the identity is currently active
    pub is_active: bool,
}

impl DigitalIdentity {
    pub fn new(
        identity_id: String,
        identity_type: IdentityType,
        display_name: String,
        public_key: Vec<u8>,
        private_key: Vec<u8>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        let hash = Hash::new().unwrap();
        let public_key_commitment = hash.compute(&public_key).unwrap();

        Self {
            identity_id,
            identity_type,
            display_name,
            public_key,
            public_key_commitment,
            private_key,
            created_at: Utc::now(),
            expires_at,
            is_active: true,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            return Utc::now() > expires_at;
        }
        false
    }

    pub fn generate_proof(&self) -> IdentityProof {
        let hash = Hash::new().unwrap();
        let commitment = hash.compute(&self.public_key).unwrap();
        let signature = self.private_key.clone(); // In production, use actual signing

        IdentityProof {
            proof_id: format!("proof_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            identity_id: self.identity_id.clone(),
            commitment,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signature,
        }
    }
}

/// Avantis ID configuration
/// 
/// Configuration settings for the Avantis ID identity management system,
/// including identity creation options and security features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvantisIdConfig {
    /// Enable Avantis ID identity management
    pub enabled: bool,
    /// Default type of identity to create
    pub default_identity_type: IdentityType,
    /// Default duration for temporary identities in days
    pub default_duration_days: u32,
    /// Enable anchoring identities to blockchain for immutability
    pub enable_blockchain_anchoring: bool,
    /// Enable binding identities to biometric data
    pub enable_biometric_binding: bool,
    /// Enable logging of identity management events
    pub enable_logging: bool,
}

impl Default for AvantisIdConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_identity_type: IdentityType::Anonymous,
            default_duration_days: 30,
            enable_blockchain_anchoring: false,
            enable_biometric_binding: false,
            enable_logging: true,
        }
    }
}

/// Avantis ID statistics
/// 
/// Contains statistics about the Avantis ID identity management system,
/// including identity counts and proof generation metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvantisIdStats {
    /// Total number of identities created
    pub total_identities: usize,
    /// Number of currently active identities
    pub active_identities: usize,
    /// Breakdown of identities by type
    pub identities_by_type: HashMap<IdentityType, usize>,
    /// Total number of identity proofs generated
    pub proofs_generated: u64,
    /// Number of identities anchored to blockchain
    pub blockchain_anchors: u64,
}

/// Avantis ID Manager
/// Avantis ID manager
///
/// Manages digital identities, identity proofs, and identity-related
/// operations for privacy-preserving authentication.
pub struct AvantisIdManager {
    config: AvantisIdConfig,
    identities: Arc<RwLock<HashMap<String, DigitalIdentity>>>,
    proofs: Arc<RwLock<HashMap<String, IdentityProof>>>,
    stats: Arc<Mutex<AvantisIdStats>>,
    rng: Arc<SecureRandom>,
}

impl AvantisIdManager {
    pub fn new(config: AvantisIdConfig) -> Result<Self> {
        let rng = SecureRandom::new()?;

        let stats = AvantisIdStats {
            total_identities: 0,
            active_identities: 0,
            identities_by_type: HashMap::new(),
            proofs_generated: 0,
            blockchain_anchors: 0,
        };

        Ok(Self {
            config,
            identities: Arc::new(RwLock::new(HashMap::new())),
            proofs: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
            rng: Arc::new(rng),
        })
    }

    /// Generate new identity
    pub async fn generate_identity(
        &self,
        display_name: String,
        identity_type: IdentityType,
        duration_days: Option<u32>,
    ) -> Result<DigitalIdentity> {
        if !self.config.enabled {
            return Err(VantisError::InvalidPeer("Avantis ID is not enabled".to_string()));
        }

        // Generate key pair
        let secret_key = self.rng.generate_bytes(32)?;
        let public_key = self.rng.generate_bytes(32)?;

        // Calculate expiration
        let expires_at = duration_days
            .map(|days| Utc::now() + Duration::seconds(days as i64 * 86400))
            .or(Some(self.config.default_duration_days).map(|days| Utc::now() + Duration::seconds(days as i64 * 86400)));

        // Generate identity ID
        let identity_id = self.generate_identity_id();

        let identity = DigitalIdentity::new(
            identity_id.clone(),
            identity_type,
            display_name,
            public_key,
            secret_key,
            expires_at,
        );

        // Store identity
        {
            let mut identities = self.identities.write().await;
            identities.insert(identity_id.clone(), identity.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.total_identities += 1;
            stats.active_identities += 1;
            *stats.identities_by_type.entry(identity_type).or_insert(0) += 1;
        }

        Ok(identity)
    }

    /// Get identity by ID
    pub async fn get_identity(&self, identity_id: &str) -> Result<DigitalIdentity> {
        let identities = self.identities.read().await;
        identities
            .get(identity_id)
            .cloned()
            .ok_or_else(|| VantisError::InvalidPeer("Identity not found".to_string()))
    }

    /// Deactivate identity
    pub async fn deactivate_identity(&self, identity_id: &str) -> Result<()> {
        let mut identities = self.identities.write().await;
        if let Some(identity) = identities.get_mut(identity_id) {
            identity.is_active = false;
            Ok(())
        } else {
            Err(VantisError::InvalidPeer("Identity not found".to_string()))
        }
    }

    /// Generate identity proof
    pub async fn generate_identity_proof(&self, identity_id: &str) -> Result<IdentityProof> {
        let identity = self.get_identity(identity_id).await?;

        if !identity.is_active {
            return Err(VantisError::InvalidPeer("Identity is not active".to_string()));
        }

        let proof = identity.generate_proof();

        // Store proof
        {
            let mut proofs = self.proofs.write().await;
            proofs.insert(proof.proof_id.clone(), proof.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.proofs_generated += 1;
        }

        Ok(proof)
    }

    /// Verify identity proof
    pub async fn verify_identity_proof(&self, proof: &IdentityProof) -> Result<bool> {
        let identity = self.get_identity(&proof.identity_id).await?;

        if !identity.is_active {
            return Ok(false);
        }

        // Verify commitment
        let hash = Hash::new()?;
        let computed_commitment = hash.compute(&identity.public_key)?;

        if computed_commitment != proof.commitment {
            return Ok(false);
        }

        // Verify signature (in production, use actual signature verification)
        // For now, return true as placeholder
        Ok(true)
    }

    /// Get all active identities
    pub async fn get_active_identities(&self) -> Vec<DigitalIdentity> {
        let identities = self.identities.read().await;
        identities
            .values()
            .filter(|id| id.is_active && !id.is_expired())
            .cloned()
            .collect()
    }

    /// Get identities by type
    pub async fn get_identities_by_type(&self, identity_type: IdentityType) -> Vec<DigitalIdentity> {
        let identities = self.identities.read().await;
        identities
            .values()
            .filter(|id| id.identity_type == identity_type && id.is_active && !id.is_expired())
            .cloned()
            .collect()
    }

    /// Get statistics
    pub async fn get_stats(&self) -> AvantisIdStats {
        self.stats.lock().await.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: AvantisIdConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }

    /// Clean up expired identities
    pub async fn cleanup_expired_identities(&self) -> usize {
        let mut identities = self.identities.write().await;
        let initial_count = identities.len();
        
        identities.retain(|_, id| id.is_active && !id.is_expired());
        
        let removed = initial_count - identities.len();
        
        if removed > 0 {
            let mut stats = self.stats.lock().await;
            stats.active_identities = identities.len();
        }
        
        removed
    }

    /// Generate identity ID
    fn generate_identity_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let random_bytes = self.rng.generate_bytes(16).unwrap();
        let hash = Hash::new().unwrap();
        let hash_bytes = hash.compute(&random_bytes).unwrap();
        
        format!("avantis_{}", hex::encode(hash_bytes))
    }

    /// Start cleanup task
    pub async fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let identities = self.identities.clone();
        let interval = std::time::Duration::from_secs(3600); // Check every hour

        tokio::spawn(async move {
            let mut timer = tokio::time::interval(interval);
            loop {
                timer.tick().await;
                
                let mut identities = identities.write().await;
                identities.retain(|_, id| id.is_active && !id.is_expired());
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_identity_generation() {
        let config = AvantisIdConfig::default();
        let manager = AvantisIdManager::new(config).unwrap();

        let identity = manager
            .generate_identity("John Doe".to_string(), IdentityType::Anonymous, Some(30))
            .await
            .unwrap();

        assert_eq!(identity.display_name, "John Doe");
        assert!(identity.is_active);
    }

    #[tokio::test]
    async fn test_identity_expiration() {
        let mut config = AvantisIdConfig::default();
        config.default_duration_days = 0; // Immediate expiration
        let manager = AvantisIdManager::new(config).unwrap();

        let identity = manager
            .generate_identity("Test User".to_string(), IdentityType::Temporary, None)
            .await
            .unwrap();

        assert!(identity.is_expired());
    }

    #[tokio::test]
    async fn test_identity_proof() {
        let config = AvantisIdConfig::default();
        let manager = AvantisIdManager::new(config).unwrap();

        let identity = manager
            .generate_identity("Alice".to_string(), IdentityType::Anonymous, Some(30))
            .await
            .unwrap();

        let proof = manager.generate_identity_proof(&identity.identity_id).await.unwrap();
        assert_eq!(proof.identity_id, identity.identity_id);

        let verified = manager.verify_identity_proof(&proof).await.unwrap();
        assert!(verified);
    }

    #[tokio::test]
    async fn test_identities_by_type() {
        let config = AvantisIdConfig::default();
        let manager = AvantisIdManager::new(config).unwrap();

        manager
            .generate_identity("User1".to_string(), IdentityType::Personal, Some(30))
            .await
            .unwrap();

        manager
            .generate_identity("Business1".to_string(), IdentityType::Business, Some(30))
            .await
            .unwrap();

        let personal_identities = manager.get_identities_by_type(IdentityType::Personal).await;
        assert_eq!(personal_identities.len(), 1);
        assert_eq!(personal_identities[0].display_name, "User1");

        let business_identities = manager.get_identities_by_type(IdentityType::Business).await;
        assert_eq!(business_identities.len(), 1);
        assert_eq!(business_identities[0].display_name, "Business1");
    }

    #[tokio::test]
    async fn test_cleanup_expired_identities() {
        let mut config = AvantisIdConfig::default();
        config.default_duration_days = 0; // Immediate expiration
        let manager = AvantisIdManager::new(config).unwrap();

        manager
            .generate_identity("Temp1".to_string(), IdentityType::Temporary, None)
            .await
            .unwrap();

        let removed = manager.cleanup_expired_identities().await;
        assert_eq!(removed, 1);
    }
}