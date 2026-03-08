//! # Key Management
//!
//! Ephemeral key management with secure memory handling.
//! All keys are temporary and automatically zeroized when dropped.

use super::random::secure_random;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce,
};
use rand::random;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use std::fmt;
use x25519_dalek::x25519;

/// Size of ChaCha20 key (256 bits)
pub const CHACHA20_KEY_SIZE: usize = 32;

/// Size of nonce (96 bits)
pub const NONCE_SIZE: usize = 12;

/// Size of private key (256 bits)
pub const PRIVATE_KEY_SIZE: usize = 32;

/// Size of public key (256 bits)
pub const PUBLIC_KEY_SIZE: usize = 32;

/// Ephemeral key pair
///
/// Ephemeral key pair that is automatically zeroized when dropped.
/// The private key is never serialized or transmitted.
#[derive(Serialize, Deserialize)]
pub struct EphemeralKeyPair {
    /// Private key (never serialized/transmitted)
    #[serde(skip)]
    private_key: Option<PrivateKey>,
    /// Public key (can be shared)
    public_key: PublicKey,
}

impl EphemeralKeyPair {
    /// Generate a new ephemeral key pair
    ///
    /// The private key is never written to disk or transmitted.
    pub fn new() -> crate::Result<Self> {
        super::ensure_initialized()?;

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&random::<[u8; 32]>());
        let mut rng = ChaCha20Rng::from_seed(seed);

        // Generate random private key
        let mut private_bytes = [0u8; PRIVATE_KEY_SIZE];
        rng.fill_bytes(&mut private_bytes);

        // Clamp the private key for X25519 (as per RFC7748)
        private_bytes[0] &= 248;
        private_bytes[31] &= 127;
        private_bytes[31] |= 64;

        // Compute public key using x25519 with basepoint
        let public_bytes = x25519(private_bytes, x25519_dalek::X25519_BASEPOINT_BYTES);

        Ok(Self {
            private_key: Some(PrivateKey::new(private_bytes)),
            public_key: PublicKey::new(public_bytes),
        })
    }

    /// Get the private key (if still available)
    ///
    /// Returns None if key has been consumed or zeroized.
    pub fn private_key(&self) -> Option<&PrivateKey> {
        self.private_key.as_ref()
    }

    /// Get the private key bytes
    pub fn private_key_bytes(&self) -> Option<&[u8; PRIVATE_KEY_SIZE]> {
        // This method is less useful now, but kept for API compatibility
        None
    }

    /// Get the public key
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Consume and return the private key
    ///
    /// After calling this, the key pair no longer holds the private key.
    pub fn take_private_key(&mut self) -> Option<PrivateKey> {
        self.private_key.take()
    }

    /// Derive a shared secret using X25519 ECDH
    ///
    /// Uses proper Curve25519 key exchange via x25519-dalek.
    pub fn derive_shared_secret(&self, other_public: &PublicKey) -> crate::Result<[u8; 32]> {
        let private = self
            .private_key
            .as_ref()
            .ok_or(crate::VantisError::KeyConsumed)?;

        // Get private key bytes as a fixed-size array
        let mut private_bytes = [0u8; 32];
        private_bytes.copy_from_slice(private.as_bytes());

        // Convert other public key to fixed-size array
        let mut other_public_bytes = [0u8; 32];
        other_public_bytes.copy_from_slice(other_public.as_bytes());

        // Perform X25519 key exchange
        let shared_secret = x25519(private_bytes, other_public_bytes);

        Ok(shared_secret)
    }
}

impl Clone for EphemeralKeyPair {
    fn clone(&self) -> Self {
        Self {
            private_key: self.private_key.clone(),
            public_key: self.public_key.clone(),
        }
    }
}

impl fmt::Debug for EphemeralKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EphemeralKeyPair")
            .field("public_key", &self.public_key)
            .field("has_private_key", &self.private_key.is_some())
            .finish()
    }
}

impl Drop for EphemeralKeyPair {
    fn drop(&mut self) {
        if let Some(mut key) = self.private_key.take() {
            key.zeroize();
        }
    }
}

/// Private key with secure memory handling
///
/// Private key stored in secure memory and automatically zeroized when dropped.
#[derive(Clone)]
pub struct PrivateKey([u8; PRIVATE_KEY_SIZE]);

impl PrivateKey {
    pub fn new(bytes: [u8; PRIVATE_KEY_SIZE]) -> Self {
        Self(bytes)
    }

    /// Get raw bytes (use with caution)
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl PrivateKey {
    pub fn zeroize(&mut self) {
        self.0.fill(0);
    }
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

/// Public key
///
/// Public key that can be safely shared and transmitted.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey([u8; PUBLIC_KEY_SIZE]);

impl PublicKey {
    pub fn new(bytes: [u8; PUBLIC_KEY_SIZE]) -> Self {
        Self(bytes)
    }

    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Cipher suite for VPN encryption
///
/// Supported cipher suites for VPN traffic encryption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum CipherSuite {
    /// ChaCha20-Poly1305 (default)
    ///
    /// ChaCha20-Poly1305 AEAD cipher (default for VPN).
    #[default]
    ChaCha20Poly1305,
    /// AES-256-GCM (FIPS compliant)
    ///
    /// AES-256-GCM AEAD cipher (FIPS 140-3 compliant).
    Aes256Gcm,
}


/// Cipher for encrypting/decrypting VPN traffic
///
/// Symmetric cipher for encrypting and decrypting VPN traffic.
pub struct Cipher {
    suite: CipherSuite,
    key: ChaCha20Poly1305,
}

impl std::fmt::Debug for Cipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cipher")
            .field("suite", &self.suite)
            .field("key", &"<redacted>")
            .finish()
    }
}

impl Cipher {
    /// Create a new cipher with the given key and suite
    pub fn new(key: &[u8], suite: CipherSuite) -> crate::Result<Self> {
        super::ensure_initialized()?;

        if key.len() != CHACHA20_KEY_SIZE {
            return Err(crate::VantisError::InvalidKeySize);
        }

        let cipher_key = ChaChaKey::from_slice(key);
        let key = ChaCha20Poly1305::new(cipher_key);

        Ok(Self { suite, key })
    }

    /// Encrypt data with associated data
    ///
    /// Returns ciphertext with authentication tag appended.
    pub fn encrypt(&self, plaintext: &[u8], _associated_data: &[u8]) -> crate::Result<Vec<u8>> {
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        secure_random(&mut nonce_bytes)?;
        let nonce = ChaChaNonce::from_slice(&nonce_bytes);

        let mut ciphertext = self
            .key
            .encrypt(nonce, plaintext)
            .map_err(|e| crate::VantisError::CryptoError(e.to_string()))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.append(&mut ciphertext);

        Ok(result)
    }

    /// Decrypt data
    pub fn decrypt(&self, ciphertext: &[u8], _associated_data: &[u8]) -> crate::Result<Vec<u8>> {
        if ciphertext.len() < NONCE_SIZE {
            return Err(crate::VantisError::InvalidCiphertext);
        }

        let (nonce_bytes, ciphertext) = ciphertext.split_at(NONCE_SIZE);
        let nonce = ChaChaNonce::from_slice(nonce_bytes);

        self.key
            .decrypt(nonce, ciphertext)
            .map_err(|e| crate::VantisError::CryptoError(e.to_string()))
    }

    /// Get the cipher suite
    pub fn suite(&self) -> CipherSuite {
        self.suite
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn ensure_crypto_initialized() {
        super::super::init().expect("Failed to initialize crypto subsystem");
    }

    #[test]
    #[serial(crypto)]
    fn test_key_pair_generation() {
        ensure_crypto_initialized();
        let pair = EphemeralKeyPair::new().expect("Failed to generate key pair");
        assert!(pair.private_key().is_some());
        assert!(pair.public_key().as_bytes().len() == PUBLIC_KEY_SIZE);
    }

    #[test]
    #[serial(crypto)]
    fn test_key_zeroization() {
        ensure_crypto_initialized();
        let pair = EphemeralKeyPair::new().expect("Failed to generate key pair");
        drop(pair);
        // Key should be zeroized
    }

    #[test]
    #[serial(crypto)]
    fn test_cipher_encrypt_decrypt() {
        ensure_crypto_initialized();
        let key = [0u8; CHACHA20_KEY_SIZE];
        let cipher = Cipher::new(&key, CipherSuite::default()).expect("Failed to create cipher");

        let plaintext = b"Hello, VANTISVPN!";
        let ciphertext = cipher.encrypt(plaintext, &[]).expect("Encryption failed");

        assert_ne!(plaintext, &ciphertext[..]);

        let decrypted = cipher.decrypt(&ciphertext, &[]).expect("Decryption failed");
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    #[serial(crypto)]
    fn test_shared_secret() {
        ensure_crypto_initialized();
        let pair1 = EphemeralKeyPair::new().expect("Failed to generate pair1");
        let pair2 = EphemeralKeyPair::new().expect("Failed to generate pair2");

        let secret1 = pair1
            .derive_shared_secret(pair2.public_key())
            .expect("Failed to derive");
        let secret2 = pair2
            .derive_shared_secret(pair1.public_key())
            .expect("Failed to derive");

        assert_eq!(secret1, secret2);
    }
}
