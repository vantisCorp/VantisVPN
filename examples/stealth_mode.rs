// Stealth Mode Example
// Demonstrates traffic obfuscation to make VPN traffic indistinguishable from normal HTTPS

use vantis_core::crypto::keys::EphemeralKeyPair;
use vantis_core::crypto::cipher::{CipherSuite, EncryptionContext, DecryptionContext};
use vantis_core::network::stealth::{StealthHandler, StealthConfig, PaddingStrategy};
use vantis_core::error::VantisError;

fn main() -> Result<(), VantisError> {
    println!("🕵️ VANTISVPN - Stealth Mode Example");
    println!("══════════════════════════════════════\n");

    // Step 1: Generate keys
    println!("Step 1: Generating cryptographic keys...");
    let keys = EphemeralKeyPair::new()?;
    let public_key = keys.public_key();
    println!("✓ Keys generated\n");

    // Step 2: Create stealth handler with TLS mimicry
    println!("Step 2: Configuring stealth mode...");
    let stealth_config = StealthConfig {
        tls_mimicry: true,
        http2_obfuscation: true,
        domain_fronting: true,
        padding_strategy: PaddingStrategy::Random,
        timing_obfuscation: true,
        jitter_ms: 50,
    };
    let stealth_handler = StealthHandler::new(stealth_config)?;
    println!("✓ Stealth handler configured");
    println!("  - TLS 1.3 mimicry: Enabled");
    println!("  - HTTP/2 obfuscation: Enabled");
    println!("  - Domain fronting: Enabled");
    println!("  - Random padding: Enabled");
    println!("  - Timing obfuscation: Enabled (50ms jitter)\n");

    // Step 3: Create encryption contexts
    println!("Step 3: Setting up encryption...");
    let mut encrypt_ctx = EncryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305)?;
    let mut decrypt_ctx = DecryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305)?;
    println!("✓ Encryption contexts created\n");

    // Step 4: Obfuscate and encrypt data
    println!("Step 4: Obfuscating and encrypting data...");
    let original_data = b"Secret message through stealth VPN";
    println!("  Original: {}", String::from_utf8_lossy(original_data));
    
    // First, obfuscate the data
    let obfuscated = stealth_handler.obfuscate(original_data)?;
    println!("  Obfuscated: {} bytes", obfuscated.len());
    
    // Then encrypt the obfuscated data
    let encrypted = encrypt_ctx.encrypt_packet(&obfuscated)?;
    println!("  Encrypted: {} bytes", encrypted.len());
    println!();

    // Step 5: Decrypt and de-obfuscate
    println!("Step 5: Decrypting and de-obfuscating...");
    let decrypted_obfuscated = decrypt_ctx.decrypt_packet(&encrypted)?;
    println!("  Decrypted: {} bytes", decrypted_obfuscated.len());
    
    let deobfuscated = stealth_handler.deobfuscate(&decrypted_obfuscated)?;
    println!("  De-obfuscated: {}", String::from_utf8_lossy(&deobfuscated));
    println!();

    // Step 6: Demonstrate packet size normalization
    println!("Step 6: Packet size normalization...");
    let test_packets = vec![
        b"Small".to_vec(),
        b"Medium sized packet".to_vec(),
        b"This is a much larger packet that will be padded to match standard sizes".to_vec(),
    ];
    
    for (i, packet) in test_packets.iter().enumerate() {
        let normalized = stealth_handler.normalize_packet_size(packet)?;
        println!("  Packet {}: {} → {} bytes", i + 1, packet.len(), normalized.len());
    }
    println!();

    // Summary
    println!("══════════════════════════════════════");
    println!("✅ Stealth mode demonstration complete!");
    println!();
    println!("Stealth mode features:");
    println!("• Traffic appears as normal HTTPS/TLS");
    println!("• Packet sizes normalized to standard values");
    println!("• Timing patterns randomized");
    println!("• Domain fronting support");
    println!("• Resists deep packet inspection (DPI)");
    
    Ok(())
}