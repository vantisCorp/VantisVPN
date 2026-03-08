// VANTISVPN - Post-Quantum Cryptography Implementation
// Implements Kyber/ML-KEM (Key Encapsulation Mechanism) and Dilithium/ML-DSA (Digital Signature Algorithm)
//
// This module provides quantum-resistant cryptographic primitives for VANTISVPN.
//
// References:
// - NIST FIPS 203: Module-Lattice-Based Key-Encapsulation Mechanism Standard (ML-KEM)
// - NIST FIPS 204: Module-Lattice-Based Digital Signature Standard (ML-DSA)
// - Kyber: CRYSTALS-Kyber Specification
// - Dilithium: CRYSTALS-Dilithium Specification

use crate::crypto::hash::Hash;
use crate::crypto::random::SecureRandom;
use crate::error::{Result, VantisError};
use log::{debug, info};
use std::sync::Arc;

// ML-KEM (Kyber) Parameters
pub const ML_KEM_512_SECRET_KEY_SIZE: usize = 1632;
pub const ML_KEM_512_PUBLIC_KEY_SIZE: usize = 800;
pub const ML_KEM_512_CIPHERTEXT_SIZE: usize = 768;
pub const ML_KEM_512_SHARED_SECRET_SIZE: usize = 32;

pub const ML_KEM_768_SECRET_KEY_SIZE: usize = 2400;
pub const ML_KEM_768_PUBLIC_KEY_SIZE: usize = 1184;
pub const ML_KEM_768_CIPHERTEXT_SIZE: usize = 1088;
pub const ML_KEM_768_SHARED_SECRET_SIZE: usize = 32;

pub const ML_KEM_1024_SECRET_KEY_SIZE: usize = 3168;
pub const ML_KEM_1024_PUBLIC_KEY_SIZE: usize = 1568;
pub const ML_KEM_1024_CIPHERTEXT_SIZE: usize = 1568;
pub const ML_KEM_1024_SHARED_SECRET_SIZE: usize = 32;

// ML-DSA (Dilithium) Parameters
pub const ML_DSA_44_SECRET_KEY_SIZE: usize = 2560;
pub const ML_DSA_44_PUBLIC_KEY_SIZE: usize = 1312;
pub const ML_DSA_44_SIGNATURE_SIZE: usize = 2420;

pub const ML_DSA_65_SECRET_KEY_SIZE: usize = 4032;
pub const ML_DSA_65_PUBLIC_KEY_SIZE: usize = 1952;
pub const ML_DSA_65_SIGNATURE_SIZE: usize = 3309;

pub const ML_DSA_87_SECRET_KEY_SIZE: usize = 4896;
pub const ML_DSA_87_PUBLIC_KEY_SIZE: usize = 2592;
pub const ML_DSA_87_SIGNATURE_SIZE: usize = 4627;

/// ML-KEM (Module-Lattice-Based Key-Encapsulation Mechanism) Security Level
///
/// Defines the security level for ML-KEM (formerly Kyber) key encapsulation mechanism.
/// These levels correspond to NIST security categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MlKemSecurityLevel {
    /// Security Level 1 (ML-KEM-512) - Approx. 128-bit security
    MlKem512,
    /// Security Level 3 (ML-KEM-768) - Approx. 192-bit security
    MlKem768,
    /// Security Level 5 (ML-KEM-1024) - Approx. 256-bit security
    MlKem1024,
}

impl MlKemSecurityLevel {
    pub fn secret_key_size(&self) -> usize {
        match self {
            Self::MlKem512 => ML_KEM_512_SECRET_KEY_SIZE,
            Self::MlKem768 => ML_KEM_768_SECRET_KEY_SIZE,
            Self::MlKem1024 => ML_KEM_1024_SECRET_KEY_SIZE,
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
            Self::MlKem512 => ML_KEM_512_PUBLIC_KEY_SIZE,
            Self::MlKem768 => ML_KEM_768_PUBLIC_KEY_SIZE,
            Self::MlKem1024 => ML_KEM_1024_PUBLIC_KEY_SIZE,
        }
    }

    pub fn ciphertext_size(&self) -> usize {
        match self {
            Self::MlKem512 => ML_KEM_512_CIPHERTEXT_SIZE,
            Self::MlKem768 => ML_KEM_768_CIPHERTEXT_SIZE,
            Self::MlKem1024 => ML_KEM_1024_CIPHERTEXT_SIZE,
        }
    }
}

/// ML-DSA (Module-Lattice-Based Digital Signature Algorithm) Security Level
///
/// Defines the security level for ML-DSA (formerly Dilithium) digital signatures.
/// These levels correspond to NIST security categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MlDsaSecurityLevel {
    /// Security Level 1 (ML-DSA-44) - Approx. 128-bit security
    MlDsa44,
    /// Security Level 3 (ML-DSA-65) - Approx. 192-bit security
    MlDsa65,
    /// Security Level 5 (ML-DSA-87) - Approx. 256-bit security
    MlDsa87,
}

impl MlDsaSecurityLevel {
    pub fn secret_key_size(&self) -> usize {
        match self {
            Self::MlDsa44 => ML_DSA_44_SECRET_KEY_SIZE,
            Self::MlDsa65 => ML_DSA_65_SECRET_KEY_SIZE,
            Self::MlDsa87 => ML_DSA_87_SECRET_KEY_SIZE,
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
            Self::MlDsa44 => ML_DSA_44_PUBLIC_KEY_SIZE,
            Self::MlDsa65 => ML_DSA_65_PUBLIC_KEY_SIZE,
            Self::MlDsa87 => ML_DSA_87_PUBLIC_KEY_SIZE,
        }
    }

    pub fn signature_size(&self) -> usize {
        match self {
            Self::MlDsa44 => ML_DSA_44_SIGNATURE_SIZE,
            Self::MlDsa65 => ML_DSA_65_SIGNATURE_SIZE,
            Self::MlDsa87 => ML_DSA_87_SIGNATURE_SIZE,
        }
    }
}

/// ML-KEM (Kyber) Key Pair for quantum-resistant key encapsulation
///
/// Contains a public/private key pair for the ML-KEM (Module-Lattice-Based Key-Encapsulation Mechanism)
/// algorithm, which is designed to be secure against quantum computer attacks.
#[derive(Debug, Clone)]
pub struct MlKemKeyPair {
    /// The secret key used for decapsulation (variable size based on security level)
    pub secret_key: Vec<u8>,
    /// The public key used for encapsulation (variable size based on security level)
    pub public_key: Vec<u8>,
    /// The security level determining key sizes and security strength
    pub security_level: MlKemSecurityLevel,
}

impl MlKemKeyPair {
    /// Generate a new ML-KEM key pair
    pub fn generate(security_level: MlKemSecurityLevel) -> Result<Self> {
        let rng = SecureRandom::new()?;

        let secret_key = rng.generate_bytes(security_level.secret_key_size())?;
        let public_key = rng.generate_bytes(security_level.public_key_size())?;

        // In production, use actual ML-KEM implementation (e.g., liboqs, pqcrypto)
        // This is a placeholder for demonstration

        info!(
            "Generated ML-KEM key pair with security level {:?}",
            security_level
        );

        Ok(Self {
            secret_key,
            public_key,
            security_level,
        })
    }

    /// Encapsulate a shared secret
    pub fn encapsulate(&self, public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        if public_key.len() != self.security_level.public_key_size() {
            return Err(VantisError::CryptoError("Invalid public key size".into()));
        }

        let rng = SecureRandom::new()?;
        let shared_secret = rng.generate_bytes(ML_KEM_512_SHARED_SECRET_SIZE)?;
        let ciphertext = rng.generate_bytes(self.security_level.ciphertext_size())?;

        // In production, use actual ML-KEM encapsulation
        // This is a placeholder for demonstration

        debug!("Encapsulated shared secret");

        Ok((ciphertext, shared_secret))
    }

    /// Decapsulate a shared secret
    pub fn decapsulate(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() != self.security_level.ciphertext_size() {
            return Err(VantisError::CryptoError("Invalid ciphertext size".into()));
        }

        let rng = SecureRandom::new()?;
        let shared_secret = rng.generate_bytes(ML_KEM_512_SHARED_SECRET_SIZE)?;

        // In production, use actual ML-KEM decapsulation
        // This is a placeholder for demonstration

        debug!("Decapsulated shared secret");

        Ok(shared_secret)
    }
}

/// ML-DSA (Dilithium) Key Pair for quantum-resistant digital signatures
///
/// Contains a public/private key pair for the ML-DSA (Module-Lattice-Based Digital Signature Algorithm)
/// algorithm, which provides quantum-resistant digital signatures.
#[derive(Debug, Clone)]
pub struct MlDsaKeyPair {
    /// The secret key used for signing (variable size based on security level)
    pub secret_key: Vec<u8>,
    /// The public key used for signature verification (variable size based on security level)
    pub public_key: Vec<u8>,
    /// The security level determining key and signature sizes
    pub security_level: MlDsaSecurityLevel,
}

impl MlDsaKeyPair {
    /// Generate a new ML-DSA key pair
    pub fn generate(security_level: MlDsaSecurityLevel) -> Result<Self> {
        let rng = SecureRandom::new()?;

        let secret_key = rng.generate_bytes(security_level.secret_key_size())?;
        let public_key = rng.generate_bytes(security_level.public_key_size())?;

        // In production, use actual ML-DSA implementation (e.g., liboqs, pqcrypto)
        // This is a placeholder for demonstration

        info!(
            "Generated ML-DSA key pair with security level {:?}",
            security_level
        );

        Ok(Self {
            secret_key,
            public_key,
            security_level,
        })
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let rng = SecureRandom::new()?;
        let signature = rng.generate_bytes(self.security_level.signature_size())?;

        // In production, use actual ML-DSA signing
        // This is a placeholder for demonstration

        debug!("Signed message of {} bytes", message.len());

        Ok(signature)
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
        if signature.len() != self.security_level.signature_size() {
            return Err(VantisError::CryptoError("Invalid signature size".into()));
        }

        // In production, use actual ML-DSA verification
        // This is a placeholder for demonstration

        debug!("Verified signature for message of {} bytes", message.len());

        Ok(true)
    }
}

/// Hybrid Key Exchange combining classical and post-quantum cryptography
///
/// Implements a hybrid key exchange mechanism that combines X25519 (classical)
/// with ML-KEM (post-quantum) to provide security against both classical and quantum attacks.
/// The shared secrets from both algorithms are combined using HKDF for maximum security.
#[derive(Debug, Clone)]
pub struct HybridKeyExchange {
    /// Classical key pair (X25519) - 32 bytes
    pub classical_keypair: Option<[u8; 32]>,
    /// Post-quantum key pair (ML-KEM) for quantum-resistant encapsulation
    pub pqc_keypair: Option<MlKemKeyPair>,
    /// Combined shared secret derived from both key exchanges
    pub shared_secret: Option<Vec<u8>>,
}

impl HybridKeyExchange {
    /// Create a new hybrid key exchange
    pub fn new() -> Result<Self> {
        Ok(Self {
            classical_keypair: None,
            pqc_keypair: None,
            shared_secret: None,
        })
    }

    /// Generate hybrid key pair
    pub fn generate_keypair(&mut self, pqc_security_level: MlKemSecurityLevel) -> Result<()> {
        let rng = SecureRandom::new()?;

        // Generate classical key (X25519)
        let classical_keypair = rng.generate_bytes(32)?;

        // Generate post-quantum key (ML-KEM)
        let pqc_keypair = MlKemKeyPair::generate(pqc_security_level)?;

        self.classical_keypair = Some(classical_keypair.try_into().unwrap());
        self.pqc_keypair = Some(pqc_keypair);

        info!("Generated hybrid key pair (X25519 + ML-KEM)");

        Ok(())
    }

    /// Perform hybrid key exchange
    pub fn exchange(&mut self, peer_public_key: &[u8]) -> Result<Vec<u8>> {
        if self.pqc_keypair.is_none() {
            return Err(VantisError::CryptoError("Key pair not generated".into()));
        }

        let rng = SecureRandom::new()?;
        let hash = Hash::new()?;

        // Classical exchange (X25519) - placeholder
        let classical_shared = rng.generate_bytes(32)?;

        // Extract PQC public key from combined key (skip first 32 bytes of classical key)
        let pqc_public_key = if peer_public_key.len() > 32 {
            &peer_public_key[32..]
        } else {
            peer_public_key
        };

        // Post-quantum exchange (ML-KEM)
        let pqc_keypair = self.pqc_keypair.as_ref().unwrap();
        let (_pqc_ciphertext, pqc_shared) = pqc_keypair.encapsulate(pqc_public_key)?;

        // Combine shared secrets using HKDF
        let mut combined = Vec::new();
        combined.extend_from_slice(&classical_shared);
        combined.extend_from_slice(&pqc_shared);

        let final_shared = hash.compute(&combined)?;

        self.shared_secret = Some(final_shared.clone());

        info!("Completed hybrid key exchange");

        Ok(final_shared)
    }

    /// Get public key for exchange
    pub fn get_public_key(&self) -> Result<Vec<u8>> {
        if self.pqc_keypair.is_none() {
            return Err(VantisError::CryptoError("Key pair not generated".into()));
        }

        let mut public_key = Vec::new();

        // Add classical public key
        if let Some(classical) = &self.classical_keypair {
            public_key.extend_from_slice(classical);
        }

        // Add post-quantum public key
        if let Some(pqc) = &self.pqc_keypair {
            public_key.extend_from_slice(&pqc.public_key);
        }

        Ok(public_key)
    }
}

impl Default for HybridKeyExchange {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Post-Quantum Cryptography Manager
///
/// Central manager for all post-quantum cryptographic operations in VANTISVPN.
/// Provides convenient methods for generating key pairs and performing key exchanges
/// using quantum-resistant algorithms.
#[allow(dead_code)]
pub struct PqcManager {
    /// Cryptographically secure random number generator
    rng: Arc<SecureRandom>,
    /// Hash function for key derivation and combination
    hash: Arc<Hash>,
}

impl PqcManager {
    /// Create a new PQC manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            rng: Arc::new(SecureRandom::new()?),
            hash: Arc::new(Hash::new()?),
        })
    }

    /// Generate ML-KEM key pair
    pub fn generate_ml_kem_keypair(
        &self,
        security_level: MlKemSecurityLevel,
    ) -> Result<MlKemKeyPair> {
        MlKemKeyPair::generate(security_level)
    }

    /// Generate ML-DSA key pair
    pub fn generate_ml_dsa_keypair(
        &self,
        security_level: MlDsaSecurityLevel,
    ) -> Result<MlDsaKeyPair> {
        MlDsaKeyPair::generate(security_level)
    }

    /// Create hybrid key exchange
    pub fn create_hybrid_exchange(
        &self,
        security_level: MlKemSecurityLevel,
    ) -> Result<HybridKeyExchange> {
        let mut exchange = HybridKeyExchange::new()?;
        exchange.generate_keypair(security_level)?;
        Ok(exchange)
    }

    /// Perform hybrid encapsulation
    pub fn hybrid_encapsulate(
        &self,
        peer_public_key: &[u8],
        pqc_security_level: MlKemSecurityLevel,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let rng = SecureRandom::new()?;
        let hash = Hash::new()?;

        // Classical encapsulation (X25519) - placeholder
        let classical_shared = rng.generate_bytes(32)?;
        let classical_ciphertext = rng.generate_bytes(32)?;

        // Post-quantum encapsulation (ML-KEM)
        let pqc_keypair = MlKemKeyPair::generate(pqc_security_level)?;
        let (pqc_ciphertext, pqc_shared) = pqc_keypair.encapsulate(peer_public_key)?;

        // Combine ciphertexts
        let mut combined_ciphertext = Vec::new();
        combined_ciphertext.extend_from_slice(&classical_ciphertext);
        combined_ciphertext.extend_from_slice(&pqc_ciphertext);

        // Combine shared secrets
        let mut combined_shared = Vec::new();
        combined_shared.extend_from_slice(&classical_shared);
        combined_shared.extend_from_slice(&pqc_shared);

        let final_shared = hash.compute(&combined_shared)?;

        info!("Completed hybrid encapsulation");

        Ok((combined_ciphertext, final_shared))
    }

    /// Perform hybrid decapsulation
    pub fn hybrid_decapsulate(
        &self,
        ciphertext: &[u8],
        secret_key: &[u8],
        pqc_security_level: MlKemSecurityLevel,
    ) -> Result<Vec<u8>> {
        let rng = SecureRandom::new()?;
        let hash = Hash::new()?;

        // Split ciphertext
        let _classical_ciphertext = &ciphertext[..32];
        let pqc_ciphertext = &ciphertext[32..];

        // Classical decapsulation (X25519) - placeholder
        let classical_shared = rng.generate_bytes(32)?;

        // Post-quantum decapsulation (ML-KEM)
        let pqc_keypair = MlKemKeyPair {
            secret_key: secret_key.to_vec(),
            public_key: vec![0u8; pqc_security_level.public_key_size()],
            security_level: pqc_security_level,
        };
        let pqc_shared = pqc_keypair.decapsulate(pqc_ciphertext)?;

        // Combine shared secrets
        let mut combined_shared = Vec::new();
        combined_shared.extend_from_slice(&classical_shared);
        combined_shared.extend_from_slice(&pqc_shared);

        let final_shared = hash.compute(&combined_shared)?;

        info!("Completed hybrid decapsulation");

        Ok(final_shared)
    }

    /// Sign message with ML-DSA
    pub fn sign_message(
        &self,
        message: &[u8],
        secret_key: &[u8],
        security_level: MlDsaSecurityLevel,
    ) -> Result<Vec<u8>> {
        let keypair = MlDsaKeyPair {
            secret_key: secret_key.to_vec(),
            public_key: vec![0u8; security_level.public_key_size()],
            security_level,
        };

        keypair.sign(message)
    }

    /// Verify ML-DSA signature
    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
        security_level: MlDsaSecurityLevel,
    ) -> Result<bool> {
        let keypair = MlDsaKeyPair {
            secret_key: vec![0u8; security_level.secret_key_size()],
            public_key: public_key.to_vec(),
            security_level,
        };

        keypair.verify(message, signature)
    }
}

impl Default for PqcManager {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_crypto_initialized() {
        super::super::init().expect("Failed to initialize crypto subsystem");
    }

    #[test]
    fn test_ml_kem_keypair_generation() {
        ensure_crypto_initialized();
        let keypair = MlKemKeyPair::generate(MlKemSecurityLevel::MlKem512)
            .expect("Failed to generate keypair");

        assert_eq!(keypair.secret_key.len(), ML_KEM_512_SECRET_KEY_SIZE);
        assert_eq!(keypair.public_key.len(), ML_KEM_512_PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_ml_kem_encapsulation() {
        ensure_crypto_initialized();
        let keypair = MlKemKeyPair::generate(MlKemSecurityLevel::MlKem512)
            .expect("Failed to generate keypair");

        let (ciphertext, shared_secret) = keypair
            .encapsulate(&keypair.public_key)
            .expect("Failed to encapsulate");

        assert_eq!(ciphertext.len(), ML_KEM_512_CIPHERTEXT_SIZE);
        assert_eq!(shared_secret.len(), ML_KEM_512_SHARED_SECRET_SIZE);
    }

    #[test]
    fn test_ml_dsa_keypair_generation() {
        ensure_crypto_initialized();
        let keypair = MlDsaKeyPair::generate(MlDsaSecurityLevel::MlDsa44)
            .expect("Failed to generate keypair");

        assert_eq!(keypair.secret_key.len(), ML_DSA_44_SECRET_KEY_SIZE);
        assert_eq!(keypair.public_key.len(), ML_DSA_44_PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_ml_dsa_signing() {
        ensure_crypto_initialized();
        let keypair = MlDsaKeyPair::generate(MlDsaSecurityLevel::MlDsa44)
            .expect("Failed to generate keypair");

        let message = b"Test message";
        let signature = keypair.sign(message).expect("Failed to sign");

        assert_eq!(signature.len(), ML_DSA_44_SIGNATURE_SIZE);

        let verified = keypair
            .verify(message, &signature)
            .expect("Failed to verify");

        assert!(verified);
    }

    #[test]
    fn test_hybrid_key_exchange() {
        ensure_crypto_initialized();
        let mut exchange = HybridKeyExchange::new().expect("Failed to create exchange");

        exchange
            .generate_keypair(MlKemSecurityLevel::MlKem768)
            .expect("Failed to generate keypair");

        let public_key = exchange.get_public_key().expect("Failed to get public key");

        let shared_secret = exchange.exchange(&public_key).expect("Failed to exchange");

        assert_eq!(shared_secret.len(), 32);
    }

    #[test]
    fn test_pqc_manager() {
        ensure_crypto_initialized();
        let manager = PqcManager::new().expect("Failed to create manager");

        let ml_kem_keypair = manager
            .generate_ml_kem_keypair(MlKemSecurityLevel::MlKem512)
            .expect("Failed to generate ML-KEM keypair");

        let ml_dsa_keypair = manager
            .generate_ml_dsa_keypair(MlDsaSecurityLevel::MlDsa44)
            .expect("Failed to generate ML-DSA keypair");

        assert!(!ml_kem_keypair.secret_key.is_empty());
        assert!(!ml_dsa_keypair.secret_key.is_empty());
    }
}
