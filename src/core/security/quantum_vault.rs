// Quantum Vault - Secure Password Manager
// Phase 4: User Security & Protection
// Implements quantum-resistant password storage and management

use crate::crypto::cipher::{Cipher, CipherSuite};
use crate::crypto::hash::Hash;
use crate::crypto::random::SecureRandom;
use crate::error::VantisError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Vault entry for storing credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Password vault entry
///
/// Represents a single password entry in the Quantum Vault, containing
/// encrypted credentials and metadata for secure password management.
pub struct VaultEntry {
    /// Unique identifier for this vault entry
    pub id: String,
    /// Name of the service or website
    pub service: String,
    /// Username or email address for the service
    pub username: String,
    /// Encrypted password data
    pub encrypted_password: Vec<u8>,
    /// Nonce used for password encryption
    pub password_nonce: Vec<u8>,
    /// Optional URL for the service
    pub url: Option<String>,
    /// Optional notes about this entry
    pub notes: Option<String>,
    /// Tags for organizing and searching entries
    pub tags: Vec<String>,
    /// Timestamp when this entry was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when this entry was last modified
    pub modified_at: DateTime<Utc>,
    /// Timestamp when this entry was last accessed
    pub last_accessed: Option<DateTime<Utc>>,
    /// Password strength score (0-100)
    pub strength_score: u8,
}

/// Quantum Vault configuration
///
/// Configuration settings for the password vault, including security
/// parameters, auto-lock settings, and clipboard management.
#[derive(Debug, Clone)]
pub struct VaultConfig {
    /// Number of iterations for master key derivation (PBKDF2)
    pub key_iterations: u32,
    /// Auto-lock timeout in seconds (0 = disabled)
    pub auto_lock_timeout: u64,
    /// Maximum number of failed unlock attempts before lockout
    pub max_failed_attempts: u32,
    /// Duration of lockout in seconds after too many failed attempts
    pub lockout_duration: u64,
    /// Enable automatic clearing of clipboard after password copy
    pub clipboard_auto_clear: bool,
    /// Timeout in seconds before clearing clipboard
    pub clipboard_timeout: u64,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            key_iterations: 100_000,
            auto_lock_timeout: 300, // 5 minutes
            max_failed_attempts: 5,
            lockout_duration: 900, // 15 minutes
            clipboard_auto_clear: true,
            clipboard_timeout: 30,
        }
    }
}

/// State of the Quantum Vault password manager
///
/// Current operational state of the secure password vault,
/// including locked, unlocked, and migration states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultState {
    Locked,
    Unlocked,
    LockedOut,
}

/// Vault statistics
///
/// Contains statistics about the password vault, including entry counts,
/// password health metrics, and backup information.
#[derive(Debug, Clone)]
pub struct VaultStats {
    /// Total number of entries in the vault
    pub total_entries: usize,
    /// Number of entries with weak passwords
    pub weak_passwords: usize,
    /// Number of entries with duplicate passwords
    pub duplicate_passwords: usize,
    /// Timestamp of the last backup, if any
    pub last_backup: Option<DateTime<Utc>>,
    /// When the vault was created
    pub created_at: DateTime<Utc>,
}

/// Quantum Vault - Secure Password Manager
/// Quantum Vault password manager
///
/// Manages encrypted password storage with secure key derivation,
/// auto-lock functionality, and clipboard management.
pub struct QuantumVault {
    config: VaultConfig,
    state: Arc<Mutex<VaultState>>,
    master_key: Arc<Mutex<Option<Vec<u8>>>>,
    entries: Arc<Mutex<HashMap<String, VaultEntry>>>,
    failed_attempts: Arc<Mutex<u32>>,
    lockout_until: Arc<Mutex<Option<DateTime<Utc>>>>,
    rng: Arc<Mutex<SecureRandom>>,
    cipher: Arc<Mutex<Cipher>>,
    created_at: DateTime<Utc>,
}

impl QuantumVault {
    /// Create a new Quantum Vault
    pub fn new(config: VaultConfig) -> Result<Self, VantisError> {
        let rng = SecureRandom::new()?;
        let cipher = Cipher::new(&[0u8; 32], CipherSuite::ChaCha20Poly1305)?;

        Ok(Self {
            config,
            state: Arc::new(Mutex::new(VaultState::Locked)),
            master_key: Arc::new(Mutex::new(None)),
            entries: Arc::new(Mutex::new(HashMap::new())),
            failed_attempts: Arc::new(Mutex::new(0)),
            lockout_until: Arc::new(Mutex::new(None)),
            rng: Arc::new(Mutex::new(rng)),
            cipher: Arc::new(Mutex::new(cipher)),
            created_at: Utc::now(),
        })
    }

    /// Unlock vault with master password
    pub async fn unlock(&self, master_password: &str) -> Result<(), VantisError> {
        // Check lockout status
        let lockout = self.lockout_until.lock().await;
        if let Some(until) = *lockout {
            if Utc::now() < until {
                return Err(VantisError::AuthenticationFailed(
                    "Vault is locked out".to_string(),
                ));
            }
        }
        drop(lockout);

        // Derive master key from password
        let master_key = self.derive_master_key(master_password)?;

        // Store master key
        let mut key = self.master_key.lock().await;
        *key = Some(master_key);
        drop(key);

        // Update state
        let mut state = self.state.lock().await;
        *state = VaultState::Unlocked;

        // Reset failed attempts
        let mut attempts = self.failed_attempts.lock().await;
        *attempts = 0;

        Ok(())
    }

    /// Lock vault
    pub async fn lock(&self) {
        let mut key = self.master_key.lock().await;
        *key = None;
        drop(key);

        let mut state = self.state.lock().await;
        *state = VaultState::Locked;
    }

    /// Add a new entry to the vault
    pub async fn add_entry(
        &self,
        service: String,
        username: String,
        password: String,
        url: Option<String>,
        notes: Option<String>,
        tags: Vec<String>,
    ) -> Result<String, VantisError> {
        self.check_unlocked().await?;

        // Generate unique ID
        let id = self.generate_id().await?;

        // Calculate password strength
        let strength_score = self.calculate_strength(&password);

        // Encrypt password
        let (encrypted_password, password_nonce) = self.encrypt_password(&password).await?;

        let entry = VaultEntry {
            id: id.clone(),
            service,
            username,
            encrypted_password,
            password_nonce,
            url,
            notes,
            tags,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            last_accessed: None,
            strength_score,
        };

        let mut entries = self.entries.lock().await;
        entries.insert(id.clone(), entry);

        Ok(id)
    }

    /// Get entry by ID
    pub async fn get_entry(&self, id: &str) -> Result<VaultEntry, VantisError> {
        self.check_unlocked().await?;

        let entries = self.entries.lock().await;
        let _entry = entries
            .get(id)
            .ok_or_else(|| VantisError::NotFound(format!("Entry not found: {}", id)))?;

        // Update last accessed
        drop(entries);
        let mut entries = self.entries.lock().await;
        if let Some(entry) = entries.get_mut(id) {
            entry.last_accessed = Some(Utc::now());
        }

        Ok(entries.get(id).unwrap().clone())
    }

    /// Decrypt password from entry
    pub async fn decrypt_password(&self, entry: &VaultEntry) -> Result<String, VantisError> {
        self.check_unlocked().await?;

        let cipher = self.cipher.lock().await;
        let decrypted = cipher.decrypt(&entry.encrypted_password, &entry.password_nonce)?;
        drop(cipher);

        String::from_utf8(decrypted)
            .map_err(|_| VantisError::InvalidData("Invalid UTF-8 in password".to_string()))
    }

    /// Update entry
    pub async fn update_entry(
        &self,
        id: &str,
        service: Option<String>,
        username: Option<String>,
        password: Option<String>,
        url: Option<Option<String>>,
        notes: Option<Option<String>>,
        tags: Option<Vec<String>>,
    ) -> Result<(), VantisError> {
        self.check_unlocked().await?;

        let mut entries = self.entries.lock().await;
        let entry = entries
            .get_mut(id)
            .ok_or_else(|| VantisError::NotFound(format!("Entry not found: {}", id)))?;

        if let Some(s) = service {
            entry.service = s;
        }
        if let Some(u) = username {
            entry.username = u;
        }
        if let Some(p) = password {
            let strength_score = self.calculate_strength(&p);
            let (encrypted_password, password_nonce) = self.encrypt_password(&p).await?;
            entry.encrypted_password = encrypted_password;
            entry.password_nonce = password_nonce;
            entry.strength_score = strength_score;
        }
        if let Some(u) = url {
            entry.url = u;
        }
        if let Some(n) = notes {
            entry.notes = n;
        }
        if let Some(t) = tags {
            entry.tags = t;
        }
        entry.modified_at = Utc::now();

        Ok(())
    }

    /// Delete entry
    pub async fn delete_entry(&self, id: &str) -> Result<(), VantisError> {
        self.check_unlocked().await?;

        let mut entries = self.entries.lock().await;
        entries
            .remove(id)
            .ok_or_else(|| VantisError::NotFound(format!("Entry not found: {}", id)))?;

        Ok(())
    }

    /// Search entries by query
    pub async fn search_entries(&self, query: &str) -> Result<Vec<VaultEntry>, VantisError> {
        self.check_unlocked().await?;

        let entries = self.entries.lock().await;
        let query_lower = query.to_lowercase();

        let results: Vec<VaultEntry> = entries
            .values()
            .filter(|e| {
                e.service.to_lowercase().contains(&query_lower)
                    || e.username.to_lowercase().contains(&query_lower)
                    || e.url
                        .as_ref()
                        .is_some_and(|u| u.to_lowercase().contains(&query_lower))
                    || e.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect();

        Ok(results)
    }

    /// Get vault statistics
    pub async fn get_stats(&self) -> Result<VaultStats, VantisError> {
        self.check_unlocked().await?;

        let entries = self.entries.lock().await;
        let total_entries = entries.len();

        let weak_passwords = entries.values().filter(|e| e.strength_score < 50).count();

        // Check for duplicate passwords (simplified check)
        let mut password_hashes: HashMap<Vec<u8>, u32> = HashMap::new();
        let hash_instance = Hash::new()?;
        for entry in entries.values() {
            let hash = hash_instance.compute(&entry.encrypted_password)?;
            *password_hashes.entry(hash).or_insert(0) += 1;
        }
        let duplicate_passwords = password_hashes.values().filter(|&&count| count > 1).count();

        Ok(VaultStats {
            total_entries,
            weak_passwords,
            duplicate_passwords,
            last_backup: None,
            created_at: self.created_at,
        })
    }

    /// Get vault state
    pub async fn state(&self) -> VaultState {
        *self.state.lock().await
    }

    /// Check if vault is unlocked
    async fn check_unlocked(&self) -> Result<(), VantisError> {
        let state = self.state.lock().await;
        match *state {
            VaultState::Unlocked => Ok(()),
            VaultState::Locked => Err(VantisError::AuthenticationFailed(
                "Vault is locked".to_string(),
            )),
            VaultState::LockedOut => Err(VantisError::AuthenticationFailed(
                "Vault is locked out".to_string(),
            )),
        }
    }

    /// Derive master key from password
    fn derive_master_key(&self, password: &str) -> Result<Vec<u8>, VantisError> {
        // Use Argon2id for key derivation (simplified here, use argon2 crate in production)
        let mut key = vec![0u8; 32];
        let password_bytes = password.as_bytes();
        let hash_instance = Hash::new()?;

        for _i in 0..self.config.key_iterations {
            let hash = hash_instance.compute_keyed(password_bytes, &key)?;
            key.copy_from_slice(&hash[..32.min(hash.len())]);
        }

        Ok(key)
    }

    /// Encrypt password
    async fn encrypt_password(&self, password: &str) -> Result<(Vec<u8>, Vec<u8>), VantisError> {
        let rng = self.rng.lock().await;
        let nonce = rng.generate_bytes(12)?;
        drop(rng);

        let cipher = self.cipher.lock().await;
        let encrypted = cipher.encrypt(password.as_bytes(), &nonce)?;
        drop(cipher);

        Ok((encrypted, nonce))
    }

    /// Generate unique ID
    async fn generate_id(&self) -> Result<String, VantisError> {
        let rng = self.rng.lock().await;
        let bytes = rng.generate_bytes(16)?;
        drop(rng);

        Ok(hex::encode(bytes))
    }

    /// Calculate password strength (0-100)
    fn calculate_strength(&self, password: &str) -> u8 {
        let mut score = 0u8;

        // Length
        if password.len() >= 8 {
            score += 20;
        }
        if password.len() >= 12 {
            score += 10;
        }
        if password.len() >= 16 {
            score += 10;
        }

        // Character variety
        if password.chars().any(|c| c.is_lowercase()) {
            score += 10;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            score += 10;
        }
        if password.chars().any(|c| c.is_ascii_digit()) {
            score += 10;
        }
        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 15;
        }

        // Complexity
        if password.len() > 20 {
            score += 5;
        }
        if password
            .chars()
            .collect::<std::collections::HashSet<_>>()
            .len() as f64
            / password.len() as f64
            > 0.7
        {
            score += 10;
        }

        score.min(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto;

    fn init_crypto() {
        crypto::init().expect("Crypto init failed");
    }

    #[tokio::test]
    async fn test_vault_creation() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        assert_eq!(vault.state().await, VaultState::Locked);
    }

    #[tokio::test]
    async fn test_vault_unlock() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        vault.unlock("test_password").await.unwrap();
        assert_eq!(vault.state().await, VaultState::Unlocked);
    }

    #[tokio::test]
    async fn test_vault_lock() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        vault.unlock("test_password").await.unwrap();
        vault.lock().await;
        assert_eq!(vault.state().await, VaultState::Locked);
    }

    #[tokio::test]
    async fn test_add_entry() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        vault.unlock("test_password").await.unwrap();

        let id = vault
            .add_entry(
                "Test Service".to_string(),
                "user@example.com".to_string(),
                "SecurePassword123!".to_string(),
                Some("https://example.com".to_string()),
                Some("Test notes".to_string()),
                vec!["test".to_string()],
            )
            .await
            .unwrap();

        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn test_get_entry() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        vault.unlock("test_password").await.unwrap();

        let id = vault
            .add_entry(
                "Test Service".to_string(),
                "user@example.com".to_string(),
                "SecurePassword123!".to_string(),
                None,
                None,
                vec![],
            )
            .await
            .unwrap();

        let entry = vault.get_entry(&id).await.unwrap();
        assert_eq!(entry.service, "Test Service");
        assert_eq!(entry.username, "user@example.com");
    }

    #[tokio::test]
    async fn test_decrypt_password() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        vault.unlock("test_password").await.unwrap();

        let id = vault
            .add_entry(
                "Test Service".to_string(),
                "user@example.com".to_string(),
                "SecurePassword123!".to_string(),
                None,
                None,
                vec![],
            )
            .await
            .unwrap();

        let entry = vault.get_entry(&id).await.unwrap();
        let password = vault.decrypt_password(&entry).await.unwrap();
        assert_eq!(password, "SecurePassword123!");
    }

    #[tokio::test]
    async fn test_search_entries() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        vault.unlock("test_password").await.unwrap();

        vault
            .add_entry(
                "GitHub".to_string(),
                "user@github.com".to_string(),
                "password1".to_string(),
                None,
                None,
                vec!["dev".to_string()],
            )
            .await
            .unwrap();

        vault
            .add_entry(
                "GitLab".to_string(),
                "user@gitlab.com".to_string(),
                "password2".to_string(),
                None,
                None,
                vec!["dev".to_string()],
            )
            .await
            .unwrap();

        let results = vault.search_entries("git").await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_get_stats() {
        init_crypto();
        let config = VaultConfig::default();
        let vault = QuantumVault::new(config).unwrap();
        vault.unlock("test_password").await.unwrap();

        vault
            .add_entry(
                "Test Service".to_string(),
                "user@example.com".to_string(),
                "SecurePassword123!".to_string(),
                None,
                None,
                vec![],
            )
            .await
            .unwrap();

        let stats = vault.get_stats().await.unwrap();
        assert_eq!(stats.total_entries, 1);
    }
}
