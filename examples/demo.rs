// VANTISVPN Demo Application
// Demonstrates core functionality of the VPN system

use vantis_core::crypto::keys::EphemeralKeyPair;
use vantis_core::crypto::cipher::{CipherSuite, EncryptionContext, DecryptionContext};
use vantis_core::crypto::hash::Hash;
use vantis_core::crypto::random::SecureRandom;
use vantis_core::error::VantisError;

fn main() -> Result<(), VantisError> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           VANTISVPN - Demo Application                      ║");
    println!("║           Next-Generation Secure VPN System                 ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // Demo 1: Key Generation
    println!("📋 Demo 1: Ephemeral Key Generation");
    println!("─────────────────────────────────────");
    let key_pair = EphemeralKeyPair::new()?;
    println!("✓ Generated ephemeral key pair");
    println!("  Public Key: {}", hex::encode(key_pair.public_key().as_bytes()));
    println!();

    // Demo 2: Hash Computation
    println!("📋 Demo 2: Cryptographic Hashing");
    println!("──────────────────────────────────");
    let data = b"VANTISVPN - Secure by Design";
    let hash_instance = Hash::new()?;
    let hash = hash_instance.compute(data)?;
    println!("✓ Computed BLAKE2s hash");
    println!("  Input: {}", String::from_utf8_lossy(data));
    println!("  Hash: {}", hex::encode(&hash));
    println!();

    // Demo 3: Encryption/Decryption
    println!("📋 Demo 3: Secure Encryption");
    println!("─────────────────────────────");
    let plaintext = b"This is a secret message protected by VANTISVPN";
    let public_key = key_pair.public_key();
    
    let mut encrypt_ctx = EncryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305)?;
    let mut decrypt_ctx = DecryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305)?;
    
    let ciphertext = encrypt_ctx.encrypt_packet(plaintext)?;
    let decrypted = decrypt_ctx.decrypt_packet(&ciphertext)?;
    
    println!("✓ Encrypted and decrypted message");
    println!("  Original: {}", String::from_utf8_lossy(plaintext));
    println!("  Ciphertext: {} bytes", ciphertext.len());
    println!("  Decrypted: {}", String::from_utf8_lossy(&decrypted));
    println!();

    // Demo 4: Random Number Generation
    println!("📋 Demo 4: Cryptographically Secure Random");
    println!("───────────────────────────────────────────");
    let rng = SecureRandom::new()?;
    let random_bytes = rng.generate_bytes(32)?;
    println!("✓ Generated 32 random bytes");
    println!("  Random: {}", hex::encode(&random_bytes));
    println!();

    // Demo 5: Multiple Encryption Rounds
    println!("📋 Demo 5: Multiple Encryption Rounds");
    println!("──────────────────────────────────────");
    let messages = vec![
        b"Message 1: First secure transmission".to_vec(),
        b"Message 2: Second secure transmission".to_vec(),
        b"Message 3: Third secure transmission".to_vec(),
    ];
    
    println!("✓ Encrypted and decrypted {} messages", messages.len());
    for (i, message) in messages.iter().enumerate() {
        let encrypted = encrypt_ctx.encrypt_packet(message)?;
        let decrypted = decrypt_ctx.decrypt_packet(&encrypted)?;
        println!("  Message {}: {}", i + 1, String::from_utf8_lossy(&decrypted));
    }
    println!();

    // Demo 6: Performance Metrics
    println!("📋 Demo 6: Performance Metrics");
    println!("───────────────────────────────");
    let start = std::time::Instant::now();
    
    // Generate 100 key pairs
    for _ in 0..100 {
        let _ = EphemeralKeyPair::new()?;
    }
    let key_gen_time = start.elapsed();
    
    // Encrypt 100 messages
    let start = std::time::Instant::now();
    let test_data = vec![0u8; 1024];
    for _ in 0..100 {
        let _ = encrypt_ctx.encrypt_packet(&test_data);
    }
    let encrypt_time = start.elapsed();
    
    println!("✓ Performance metrics (100 operations):");
    println!("  Key Generation: {:?}", key_gen_time);
    println!("  Encryption (1KB): {:?}", encrypt_time);
    println!();

    // Summary
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    Demo Complete!                           ║");
    println!("║                                                            ║");
    println!("║  VANTISVPN provides:                                        ║");
    println!("║  • Post-Quantum Cryptography (Kyber/ML-KEM)                ║");
    println!("║  • Quantum-Resistant Signatures (Dilithium/ML-DSA)          ║");
    println!("║  • WireGuard Protocol with Modifications                   ║");
    println!("║  • QUIC/HTTP/3 Transport Layer                             ║");
    println!("║  • Stealth Protocol for Obfuscation                        ║");
    println!("║  • MultiHop+ Onion Routing                                 ║");
    println!("║  • Zero-Knowledge Authentication                           ║");
    println!("║  • Military-Grade Security                                 ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    
    Ok(())
}