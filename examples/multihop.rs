// MultiHop+ Onion Routing Example
// Demonstrates multi-hop routing through multiple VPN servers

use vantis_core::crypto::keys::EphemeralKeyPair;
use vantis_core::crypto::cipher::{CipherSuite, EncryptionContext, DecryptionContext};
use vantis_core::network::multihop::{MultiHopManager, MultiHopConfig, Circuit};
use vantis_core::network::multihop::VpnNode;
use vantis_core::error::VantisError;

#[tokio::main]
async fn main() -> Result<(), VantisError> {
    println!("🧅 VANTISVPN - MultiHop+ Onion Routing Example");
    println!("════════════════════════════════════════════════\n");

    // Step 1: Create VPN nodes (simulated servers)
    println!("Step 1: Creating VPN nodes...");
    let nodes = vec![
        VpnNode {
            node_id: "node1".to_string(),
            ip_address: "198.51.100.1".parse().unwrap(),
            port: 51820,
            latency_ms: 20,
            load_percent: 30,
            bandwidth_mbps: 1000,
            country: "Germany".to_string(),
        },
        VpnNode {
            node_id: "node2".to_string(),
            ip_address: "203.0.113.1".parse().unwrap(),
            port: 51820,
            latency_ms: 35,
            load_percent: 45,
            bandwidth_mbps: 800,
            country: "Switzerland".to_string(),
        },
        VpnNode {
            node_id: "node3".to_string(),
            ip_address: "192.0.2.1".parse().unwrap(),
            port: 51820,
            latency_ms: 50,
            load_percent: 25,
            bandwidth_mbps: 1200,
            country: "Iceland".to_string(),
        },
    ];
    
    for node in &nodes {
        println!("  ✓ {} ({}) - {}ms latency, {}% load",
            node.node_id, node.country, node.latency_ms, node.load_percent);
    }
    println!();

    // Step 2: Configure MultiHop+ with 3 hops
    println!("Step 2: Configuring MultiHop+...");
    let multihop_config = MultiHopConfig {
        min_hops: 3,
        max_hops: 3,
        geographic_diversity: true,
        latency_optimization: true,
        path_obfuscation: true,
        failover_enabled: true,
    };
    
    let mut multihop_manager = MultiHopManager::new(multihop_config)?;
    
    // Add nodes to manager
    for node in nodes {
        multihop_manager.add_node(node)?;
    }
    println!("✓ MultiHop+ configured with 3 hops");
    println!("  - Geographic diversity: Enabled");
    println!("  - Latency optimization: Enabled");
    println!("  - Path obfuscation: Enabled");
    println!("  - Failover: Enabled\n");

    // Step 3: Create circuit through nodes
    println!("Step 3: Creating circuit through nodes...");
    let circuit = multihop_manager.create_circuit().await?;
    println!("✓ Circuit created");
    println!("  Circuit ID: {}", circuit.id());
    println!("  Hops: {}", circuit.hops().len());
    
    for (i, hop) in circuit.hops().iter().enumerate() {
        println!("    Hop {}: {} ({})", i + 1, hop.node_id, hop.country);
    }
    println!();

    // Step 4: Generate keys for layered encryption
    println!("Step 4: Generating keys for layered encryption...");
    let keys = vec![
        EphemeralKeyPair::new()?,
        EphemeralKeyPair::new()?,
        EphemeralKeyPair::new()?,
    ];
    println!("✓ Generated {} key pairs for layered encryption", keys.len());
    println!();

    // Step 5: Create encryption contexts for each hop
    println!("Step 5: Creating encryption contexts...");
    let mut encrypt_contexts: Vec<EncryptionContext> = Vec::new();
    let mut decrypt_contexts: Vec<DecryptionContext> = Vec::new();
    
    for key in &keys {
        let public_key = key.public_key();
        encrypt_contexts.push(EncryptionContext::new(
            public_key.as_bytes(),
            CipherSuite::ChaCha20Poly1305
        )?);
        decrypt_contexts.push(DecryptionContext::new(
            public_key.as_bytes(),
            CipherSuite::ChaCha20Poly1305
        )?);
    }
    println!("✓ Created {} encryption/decryption contexts", encrypt_contexts.len());
    println!();

    // Step 6: Send data through onion routing
    println!("Step 6: Sending data through onion routing...");
    let original_data = b"Secret message through MultiHop+";
    println!("  Original: {}", String::from_utf8_lossy(original_data));
    
    // Layer 1: Encrypt for first hop
    let mut encrypted = encrypt_contexts[0].encrypt_packet(original_data)?;
    println!("  After hop 1 encryption: {} bytes", encrypted.len());
    
    // Layer 2: Encrypt for second hop
    encrypted = encrypt_contexts[1].encrypt_packet(&encrypted)?;
    println!("  After hop 2 encryption: {} bytes", encrypted.len());
    
    // Layer 3: Encrypt for third hop
    encrypted = encrypt_contexts[2].encrypt_packet(&encrypted)?;
    println!("  After hop 3 encryption: {} bytes", encrypted.len());
    println!();

    // Step 7: Decrypt through layers (reverse order)
    println!("Step 7: Decrypting through layers...");
    let mut decrypted = encrypted;
    
    // Layer 3: Decrypt at third hop
    decrypted = decrypt_contexts[2].decrypt_packet(&decrypted)?;
    println!("  After hop 3 decryption: {} bytes", decrypted.len());
    
    // Layer 2: Decrypt at second hop
    decrypted = decrypt_contexts[1].decrypt_packet(&decrypted)?;
    println!("  After hop 2 decryption: {} bytes", decrypted.len());
    
    // Layer 1: Decrypt at first hop
    decrypted = decrypt_contexts[0].decrypt_packet(&decrypted)?;
    println!("  After hop 1 decryption: {} bytes", decrypted.len());
    println!("  Final: {}", String::from_utf8_lossy(&decrypted));
    println!();

    // Step 8: Get circuit statistics
    println!("Step 8: Circuit statistics...");
    let stats = circuit.stats();
    println!("  Packets sent: {}", stats.packets_sent);
    println!("  Packets received: {}", stats.packets_received);
    println!("  Bytes sent: {}", stats.bytes_sent);
    println!("  Bytes received: {}", stats.bytes_received);
    println!("  Uptime: {:?}", stats.uptime);
    println!();

    // Summary
    println!("════════════════════════════════════════════════");
    println!("✅ MultiHop+ demonstration complete!");
    println!();
    println!("MultiHop+ features:");
    println!("• Onion-style layered encryption");
    println!("• 2-7 hop routing through VPN servers");
    println!("• Geographic diversity");
    println!("• Latency optimization");
    println!("• Path obfuscation");
    println!("• Automatic failover");
    println!("• Enhanced privacy and anonymity");
    
    Ok(())
}