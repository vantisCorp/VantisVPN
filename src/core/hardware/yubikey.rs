// VANTISVPN YubiKey 2FA Support
// Hardware-based two-factor authentication using YubiKey

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};
use crate::error::VantisError;
use crate::crypto::hash::Hash;

/// YubiKey configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyConfig {
    pub enabled: bool,
    pub require_for_login: bool,
    pub require_for_admin: bool,
    pub require_for_vpn: bool,
    pub allowed_slots: Vec<YubiKeySlot>,
    pub challenge_timeout: Duration,
    pub max_attempts: u32,
    pub lockout_duration: Duration,
    pub backup_codes_enabled: bool,
    pub backup_codes_count: u32,
}

/// YubiKey slot
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum YubiKeySlot {
    Slot1,
    Slot2,
}

/// YubiKey authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YubiKeyAuth {
    ChallengeResponse {
        slot: YubiKeySlot,
        challenge: Vec<u8>,
        response: Vec<u8>,
    },
    Hmac {
        slot: YubiKeySlot,
        data: Vec<u8>,
        hmac: Vec<u8>,
    },
    Otp {
        otp: String,
    },
}

/// YubiKey challenge-response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyChallengeResponse {
    pub slot: YubiKeySlot,
    pub challenge: Vec<u8>,
    pub response: Vec<u8>,
    pub timestamp: SystemTime,
    pub attempts: u32,
}

/// YubiKey HMAC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyHmac {
    pub slot: YubiKeySlot,
    pub secret_key: Vec<u8>,
    pub data: Vec<u8>,
    pub hmac: Vec<u8>,
    pub timestamp: SystemTime,
}

/// YubiKey OTP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YubiKeyOtp {
    pub public_id: String,
    pub private_id: String,
    pub secret_key: Vec<u8>,
    pub counter: u32,
    pub timestamp: SystemTime,
    pub use_count: u32,
}

/// YubiKey manager
pub struct YubiKeyManager {
    config: YubiKeyConfig,
    registered_keys: HashMap<String, RegisteredYubiKey>,
    active_challenges: HashMap<String, YubiKeyChallengeResponse>,
    failed_attempts: HashMap<String, u32>,
    lockout_until: HashMap<String, SystemTime>,
    backup_codes: HashMap<String, Vec<String>>,
}

/// Registered YubiKey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredYubiKey {
    pub key_id: String,
    pub public_id: String,
    pub user_id: String,
    pub slot1_config: Option<SlotConfig>,
    pub slot2_config: Option<SlotConfig>,
    pub registered_at: SystemTime,
    pub last_used: Option<SystemTime>,
    pub enabled: bool,
}

/// Slot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotConfig {
    pub slot: YubiKeySlot,
    pub config_type: SlotConfigType,
    pub secret_key: Vec<u8>,
    pub require_touch: bool,
}

/// Slot configuration type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlotConfigType {
    ChallengeResponse,
    Hmac,
    Otp,
}

impl YubiKeyManager {
    /// Create new YubiKey manager
    pub fn new(config: YubiKeyConfig) -> Self {
        Self {
            config,
            registered_keys: HashMap::new(),
            active_challenges: HashMap::new(),
            failed_attempts: HashMap::new(),
            lockout_until: HashMap::new(),
            backup_codes: HashMap::new(),
        }
    }

    /// Get configuration
    pub fn config(&self) -> &YubiKeyConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: YubiKeyConfig) -> Result<(), VantisError> {
        self.config = config;
        Ok(())
    }

    /// Register YubiKey
    pub fn register_key(
        &mut self,
        key_id: String,
        public_id: String,
        user_id: String,
        slot1_config: Option<SlotConfig>,
        slot2_config: Option<SlotConfig>,
    ) -> Result<(), VantisError> {
        if self.registered_keys.contains_key(&key_id) {
            return Err(VantisError::InvalidData("YubiKey already registered".to_string()));
        }

        // Generate backup codes if enabled before moving user_id
        if self.config.backup_codes_enabled {
            self.generate_backup_codes(&user_id)?;
        }

        let registered_key = RegisteredYubiKey {
            key_id: key_id.clone(),
            public_id,
            user_id,
            slot1_config,
            slot2_config,
            registered_at: SystemTime::now(),
            last_used: None,
            enabled: true,
        };

        self.registered_keys.insert(key_id, registered_key);

        Ok(())
    }

    /// Unregister YubiKey
    pub fn unregister_key(&mut self, key_id: &str) -> Result<(), VantisError> {
        if !self.registered_keys.contains_key(key_id) {
            return Err(VantisError::NotFound("YubiKey not found".to_string()));
        }

        self.registered_keys.remove(key_id);
        self.active_challenges.remove(key_id);
        self.failed_attempts.remove(key_id);
        self.lockout_until.remove(key_id);
        
        Ok(())
    }

    /// Generate challenge
    pub fn generate_challenge(&mut self, key_id: &str, slot: YubiKeySlot) -> Result<Vec<u8>, VantisError> {
        // Check if key is registered and enabled
        let key = self.registered_keys.get(key_id)
            .ok_or_else(|| VantisError::NotFound("YubiKey not found".to_string()))?;
        
        if !key.enabled {
            return Err(VantisError::InvalidData("YubiKey is disabled".to_string()));
        }

        // Check if slot is configured
        let slot_config = match slot {
            YubiKeySlot::Slot1 => &key.slot1_config,
            YubiKeySlot::Slot2 => &key.slot2_config,
        };

        if slot_config.is_none() {
            return Err(VantisError::InvalidData("Slot not configured".to_string()));
        }

        // Check lockout
        if let Some(lockout_time) = self.lockout_until.get(key_id) {
            if SystemTime::now() < *lockout_time {
                return Err(VantisError::AuthenticationFailed("Account locked out".to_string()));
            }
        }

        // Generate random challenge (32 bytes)
        let challenge = vec![0u8; 32]; // Placeholder - should use CSPRNG

        // Store challenge
        let challenge_response = YubiKeyChallengeResponse {
            slot,
            challenge: challenge.clone(),
            response: vec![],
            timestamp: SystemTime::now(),
            attempts: 0,
        };

        self.active_challenges.insert(key_id.to_string(), challenge_response);

        Ok(challenge)
    }

    /// Verify challenge response
    pub fn verify_challenge_response(
        &mut self,
        key_id: &str,
        response: Vec<u8>,
    ) -> Result<bool, VantisError> {
        // Get active challenge
        let challenge = self.active_challenges.get_mut(key_id)
            .ok_or_else(|| VantisError::InvalidData("No active challenge".to_string()))?;

        // Check timeout
        let elapsed = SystemTime::now()
            .duration_since(challenge.timestamp)
            .map_err(|_| VantisError::InvalidData("Invalid timestamp".to_string()))?;

        if elapsed > self.config.challenge_timeout {
            self.active_challenges.remove(key_id);
            return Err(VantisError::InvalidData("Challenge expired".to_string()));
        }

        // Verify response (placeholder - should use actual YubiKey verification)
        let is_valid = response.len() == 32; // Simplified check

        if is_valid {
            // Clear challenge
            self.active_challenges.remove(key_id);
            self.failed_attempts.remove(key_id);
            self.lockout_until.remove(key_id);

            // Update last used
            if let Some(key) = self.registered_keys.get_mut(key_id) {
                key.last_used = Some(SystemTime::now());
            }

            Ok(true)
        } else {
            // Increment failed attempts
            let attempts = self.failed_attempts.entry(key_id.to_string()).or_insert(0);
            *attempts += 1;
            challenge.attempts = *attempts;

            // Check if lockout should be triggered
            if *attempts >= self.config.max_attempts {
                let lockout_time = SystemTime::now() + self.config.lockout_duration;
                self.lockout_until.insert(key_id.to_string(), lockout_time);
                self.active_challenges.remove(key_id);
            }

            Ok(false)
        }
    }

    /// Verify HMAC
    pub fn verify_hmac(
        &mut self,
        key_id: &str,
        slot: YubiKeySlot,
        data: Vec<u8>,
        hmac: Vec<u8>,
    ) -> Result<bool, VantisError> {
        // Get registered key
        let key = self.registered_keys.get(key_id)
            .ok_or_else(|| VantisError::NotFound("YubiKey not found".to_string()))?;

        if !key.enabled {
            return Err(VantisError::InvalidData("YubiKey is disabled".to_string()));
        }

        // Get slot config
        let slot_config = match slot {
            YubiKeySlot::Slot1 => &key.slot1_config,
            YubiKeySlot::Slot2 => &key.slot2_config,
        };

        let slot_config = slot_config.as_ref()
            .ok_or_else(|| VantisError::InvalidData("Slot not configured".to_string()))?;

        // Compute expected HMAC (placeholder)
        let hash_instance = Hash::new()?;
        let expected_hmac = hash_instance.compute(&data)?;

        // Verify HMAC
        let is_valid = hmac == expected_hmac;

        if is_valid {
            // Update last used
            if let Some(key) = self.registered_keys.get_mut(key_id) {
                key.last_used = Some(SystemTime::now());
            }
        }

        Ok(is_valid)
    }

    /// Verify OTP
    pub fn verify_otp(&mut self, key_id: &str, otp: String) -> Result<bool, VantisError> {
        // Get registered key
        let key = self.registered_keys.get(key_id)
            .ok_or_else(|| VantisError::NotFound("YubiKey not found".to_string()))?;

        if !key.enabled {
            return Err(VantisError::InvalidData("YubiKey is disabled".to_string()));
        }

        // Verify OTP (placeholder - should use actual YubiKey OTP verification)
        let is_valid = otp.len() == 44 && otp.starts_with("cccc");

        if is_valid {
            // Update last used
            if let Some(key) = self.registered_keys.get_mut(key_id) {
                key.last_used = Some(SystemTime::now());
            }
        }

        Ok(is_valid)
    }

    /// Generate backup codes
    pub fn generate_backup_codes(&mut self, user_id: &str) -> Result<(), VantisError> {
        let mut codes = Vec::new();
        
        for _ in 0..self.config.backup_codes_count {
            // Generate random 8-character code
            let code = format!("{:08x}", rand::random::<u32>());
            codes.push(code);
        }

        self.backup_codes.insert(user_id.to_string(), codes);
        
        Ok(())
    }

    /// Verify backup code
    pub fn verify_backup_code(&mut self, user_id: &str, code: &str) -> Result<bool, VantisError> {
        let codes = self.backup_codes.get_mut(user_id)
            .ok_or_else(|| VantisError::NotFound("No backup codes for user".to_string()))?;

        if let Some(pos) = codes.iter().position(|c| c == code) {
            // Remove used code
            codes.remove(pos);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get registered keys
    pub fn get_registered_keys(&self) -> Vec<&RegisteredYubiKey> {
        self.registered_keys.values().collect()
    }

    /// Get key by ID
    pub fn get_key(&self, key_id: &str) -> Option<&RegisteredYubiKey> {
        self.registered_keys.get(key_id)
    }

    /// Enable key
    pub fn enable_key(&mut self, key_id: &str) -> Result<(), VantisError> {
        let key = self.registered_keys.get_mut(key_id)
            .ok_or_else(|| VantisError::NotFound("YubiKey not found".to_string()))?;
        
        key.enabled = true;
        Ok(())
    }

    /// Disable key
    pub fn disable_key(&mut self, key_id: &str) -> Result<(), VantisError> {
        let key = self.registered_keys.get_mut(key_id)
            .ok_or_else(|| VantisError::NotFound("YubiKey not found".to_string()))?;
        
        key.enabled = false;
        Ok(())
    }

    /// Clear expired challenges
    pub fn clear_expired_challenges(&mut self) {
        let now = SystemTime::now();
        
        self.active_challenges.retain(|key_id, challenge| {
            let elapsed = now.duration_since(challenge.timestamp).unwrap_or(Duration::MAX);
            elapsed <= self.config.challenge_timeout
        });
    }

    /// Clear expired lockouts
    pub fn clear_expired_lockouts(&mut self) {
        let now = SystemTime::now();
        
        self.lockout_until.retain(|_key_id, lockout_time| {
            now < *lockout_time
        });
    }
}

impl Default for YubiKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            require_for_login: true,
            require_for_admin: true,
            require_for_vpn: false,
            allowed_slots: vec![YubiKeySlot::Slot1, YubiKeySlot::Slot2],
            challenge_timeout: Duration::from_secs(60),
            max_attempts: 5,
            lockout_duration: Duration::from_secs(300), // 5 minutes
            backup_codes_enabled: true,
            backup_codes_count: 10,
        }
    }
}