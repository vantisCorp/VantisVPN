//! # Symmetric Encryption Ciphers
//!
//! Provides authenticated encryption for VPN tunnel traffic.

use serde::{Deserialize, Serialize};

// Re-export from keys module
pub use super::keys::{Cipher, CipherSuite, CHACHA20_KEY_SIZE, NONCE_SIZE};

/// Supported cipher modes
///
/// Encryption modes supported by the cipher system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CipherMode {
    /// AEAD (Authenticated Encryption with Associated Data)
    ///
    /// Authenticated encryption mode that provides both confidentiality and integrity.
    Aead,
    /// Stream cipher
    ///
    /// Stream cipher mode for encrypting continuous data streams.
    Stream,
}

/// Encryption context for VPN tunnel
///
/// Maintains cipher state and sequence numbers for encrypting VPN traffic.
/// Sequence numbers are used for replay protection and packet ordering.
pub struct EncryptionContext {
    /// Symmetric cipher instance for encryption
    cipher: Cipher,
    /// Sequence number for replay protection and ordering
    sequence_number: u64,
}

impl EncryptionContext {
    /// Create a new encryption context
    pub fn new(key: &[u8], suite: CipherSuite) -> crate::Result<Self> {
        let cipher = Cipher::new(key, suite)?;
        Ok(Self {
            cipher,
            sequence_number: 0,
        })
    }

    /// Encrypt a VPN packet
    ///
    /// Returns encrypted packet with sequence number prepended.
    pub fn encrypt_packet(&mut self, plaintext: &[u8]) -> crate::Result<Vec<u8>> {
        // Include sequence number as associated data
        let seq_bytes = self.sequence_number.to_be_bytes();

        let mut ciphertext = self.cipher.encrypt(plaintext, &seq_bytes)?;

        // Prepend sequence number
        let mut result = seq_bytes.to_vec();
        result.append(&mut ciphertext);

        self.sequence_number = self.sequence_number.wrapping_add(1);
        Ok(result)
    }

    /// Decrypt a VPN packet
    pub fn decrypt_packet(&mut self, ciphertext: &[u8]) -> crate::Result<Vec<u8>> {
        if ciphertext.len() < 8 {
            return Err(crate::VantisError::InvalidCiphertext);
        }

        let (seq_bytes, ciphertext) = ciphertext.split_at(8);
        let plaintext = self.cipher.decrypt(ciphertext, seq_bytes)?;

        // Verify sequence number (optional, prevents replay attacks)
        let seq = u64::from_be_bytes(seq_bytes.try_into().unwrap());
        if seq < self.sequence_number {
            tracing::warn!(
                "Received out-of-order packet: {} < {}",
                seq,
                self.sequence_number
            );
        }

        self.sequence_number = seq.max(self.sequence_number);
        Ok(plaintext)
    }

    /// Get the current sequence number
    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    /// Reset the sequence number
    pub fn reset_sequence(&mut self) {
        self.sequence_number = 0;
    }
}

/// Decryption context for VPN tunnel
///
/// Maintains cipher state and sequence number verification for decrypting VPN traffic.
/// Detects replay attacks by rejecting packets with duplicate sequence numbers.
pub struct DecryptionContext {
    /// Symmetric cipher instance for decryption
    cipher: Cipher,
    /// Expected sequence number (replay protection)
    expected_sequence: u64,
}

impl DecryptionContext {
    /// Create a new decryption context
    pub fn new(key: &[u8], suite: CipherSuite) -> crate::Result<Self> {
        let cipher = Cipher::new(key, suite)?;
        Ok(Self {
            cipher,
            expected_sequence: 0,
        })
    }

    /// Decrypt a VPN packet
    pub fn decrypt_packet(&mut self, ciphertext: &[u8]) -> crate::Result<Vec<u8>> {
        if ciphertext.len() < 8 {
            return Err(crate::VantisError::InvalidCiphertext);
        }

        let (seq_bytes, ciphertext) = ciphertext.split_at(8);
        let seq = u64::from_be_bytes(seq_bytes.try_into().unwrap());

        // Check for replay attacks
        if seq < self.expected_sequence {
            return Err(crate::VantisError::ReplayAttack(seq));
        }

        let plaintext = self.cipher.decrypt(ciphertext, seq_bytes)?;
        self.expected_sequence = seq + 1;

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto;
    use serial_test::serial;

    fn init_crypto() {
        crypto::init().expect("Crypto init failed");
    }

    #[test]
    #[serial(crypto)]
    fn test_encryption_context() {
        init_crypto();
        let key = [0u8; CHACHA20_KEY_SIZE];
        let mut ctx =
            EncryptionContext::new(&key, CipherSuite::default()).expect("Failed to create context");

        let plaintext = b"Test VPN packet";
        let ciphertext = ctx.encrypt_packet(plaintext).expect("Encryption failed");

        assert_ne!(plaintext, &ciphertext[..]);
        assert!(ciphertext.len() > plaintext.len());
    }

    #[test]
    #[serial(crypto)]
    fn test_decryption_context() {
        init_crypto();
        let key = [0u8; CHACHA20_KEY_SIZE];
        let mut encrypt_ctx = EncryptionContext::new(&key, CipherSuite::default())
            .expect("Failed to create encrypt context");
        let mut decrypt_ctx = DecryptionContext::new(&key, CipherSuite::default())
            .expect("Failed to create decrypt context");

        let plaintext = b"Test VPN packet";
        let ciphertext = encrypt_ctx
            .encrypt_packet(plaintext)
            .expect("Encryption failed");
        let decrypted = decrypt_ctx
            .decrypt_packet(&ciphertext)
            .expect("Decryption failed");

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    #[serial(crypto)]
    fn test_sequence_numbers() {
        init_crypto();
        let key = [0u8; CHACHA20_KEY_SIZE];
        let mut ctx =
            EncryptionContext::new(&key, CipherSuite::default()).expect("Failed to create context");

        assert_eq!(ctx.sequence_number(), 0);

        ctx.encrypt_packet(b"test1").expect("Encrypt failed");
        assert_eq!(ctx.sequence_number(), 1);

        ctx.encrypt_packet(b"test2").expect("Encrypt failed");
        assert_eq!(ctx.sequence_number(), 2);
    }
}
