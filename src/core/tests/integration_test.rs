// Integration Tests for VANTISVPN
// Tests that verify multiple components work together

use vantis_core::crypto::cipher::{CipherSuite, DecryptionContext, EncryptionContext};
use vantis_core::crypto::hash::Hash;
use vantis_core::crypto::keys::EphemeralKeyPair;
use vantis_core::crypto::random::SecureRandom;

#[test]
fn test_full_encryption_flow() {
    // Initialize crypto subsystem
    vantis_core::crypto::init();

    // Test complete encryption/decryption flow
    let key_pair = EphemeralKeyPair::new().unwrap();
    let public_key = key_pair.public_key();

    let mut encrypt_ctx =
        EncryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let mut decrypt_ctx =
        DecryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let plaintext = b"Integration test message";

    let ciphertext = encrypt_ctx.encrypt_packet(plaintext).unwrap();
    let decrypted = decrypt_ctx.decrypt_packet(&ciphertext).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn test_hash_and_encryption_integration() {
    // Initialize crypto subsystem
    vantis_core::crypto::init();

    // Test hashing and encryption together
    let data = b"Test data for integration";

    let hash_instance = Hash::new().unwrap();
    let hash = hash_instance.compute(data).unwrap();

    let key_pair = EphemeralKeyPair::new().unwrap();
    let public_key = key_pair.public_key();
    let mut encrypt_ctx =
        EncryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let mut decrypt_ctx =
        DecryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();

    let encrypted_hash = encrypt_ctx.encrypt_packet(&hash).unwrap();
    let decrypted_hash = decrypt_ctx.decrypt_packet(&encrypted_hash).unwrap();

    assert_eq!(hash, decrypted_hash);
}

#[test]
fn test_random_key_generation() {
    // Test random key generation for encryption
    let rng = SecureRandom::new().unwrap();
    let key = rng.generate_bytes(32).unwrap();

    assert_eq!(key.len(), 32);

    // Verify keys are different
    let key2 = rng.generate_bytes(32).unwrap();
    assert_ne!(key, key2);
}

#[test]
fn test_multiple_encryption_rounds() {
    // Initialize crypto subsystem
    vantis_core::crypto::init();

    // Test multiple encryption/decryption rounds
    let key_pair = EphemeralKeyPair::new().unwrap();
    let public_key = key_pair.public_key();
    let mut encrypt_ctx =
        EncryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let mut decrypt_ctx =
        DecryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();

    let messages = vec![
        b"Message 1".to_vec(),
        b"Message 2".to_vec(),
        b"Message 3".to_vec(),
    ];

    for message in &messages {
        let encrypted = encrypt_ctx.encrypt_packet(message).unwrap();
        let decrypted = decrypt_ctx.decrypt_packet(&encrypted).unwrap();
        assert_eq!(*message, decrypted);
    }
}

#[test]
fn test_error_handling() {
    // Initialize crypto subsystem
    vantis_core::crypto::init();

    // Test error handling in integration scenarios
    let key_pair = EphemeralKeyPair::new().unwrap();
    let public_key = key_pair.public_key();
    let mut decrypt_ctx =
        DecryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();

    // Test decryption with invalid data
    let invalid_data = vec![0u8; 10];
    let result = decrypt_ctx.decrypt_packet(&invalid_data);
    assert!(result.is_err());

    // Test hash with empty data
    let hash_instance = Hash::new().unwrap();
    let empty_hash = hash_instance.compute(b"").unwrap();
    assert_eq!(empty_hash.len(), 32);
}
