// Simple VPN Connection Example
// Demonstrates basic VPN tunnel establishment

use vantis_core::crypto::keys::EphemeralKeyPair;
use vantis_core::crypto::cipher::{CipherSuite, EncryptionContext, DecryptionContext};
use vantis_core::tunnel::manager::TunnelManager;
use vantis_core::tunnel::state::TunnelState;
use vantis_core::error::VantisError;

#[tokio::main]
async fn main() -> Result<(), VantisError> {
    println!("🚀 VANTISVPN - Simple Connection Example");
    println!("═════════════════════════════════════════\n");

    // Step 1: Generate keys for the client
    println!("Step 1: Generating client keys...");
    let client_keys = EphemeralKeyPair::new()?;
    println!("✓ Client keys generated\n");

    // Step 2: Create tunnel manager
    println!("Step 2: Initializing tunnel manager...");
    let tunnel_manager = TunnelManager::new().await?;
    println!("✓ Tunnel manager initialized\n");

    // Step 3: Create encryption contexts
    println!("Step 3: Setting up encryption...");
    let public_key = client_keys.public_key();
    let mut encrypt_ctx = EncryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305)?;
    let mut decrypt_ctx = DecryptionContext::new(public_key.as_bytes(), CipherSuite::ChaCha20Poly1305)?;
    println!("✓ Encryption contexts created\n");

    // Step 4: Simulate tunnel establishment
    println!("Step 4: Establishing VPN tunnel...");
    let tunnel_id = tunnel_manager.create_tunnel(
        "192.168.1.100".parse().unwrap(),
        "10.0.0.2".parse().unwrap(),
    ).await?;
    println!("✓ Tunnel created with ID: {}\n", tunnel_id);

    // Step 5: Check tunnel state
    println!("Step 5: Checking tunnel status...");
    let tunnel = tunnel_manager.get_tunnel(tunnel_id).await?;
    println!("✓ Tunnel state: {:?}\n", tunnel.state());

    // Step 6: Send test data through tunnel
    println!("Step 6: Sending test data...");
    let test_data = b"Hello from VANTISVPN!";
    let encrypted = encrypt_ctx.encrypt_packet(test_data)?;
    let decrypted = decrypt_ctx.decrypt_packet(&encrypted)?;
    println!("✓ Data sent and received: {}\n", String::from_utf8_lossy(&decrypted));

    // Step 7: Close tunnel
    println!("Step 7: Closing tunnel...");
    tunnel_manager.close_tunnel(tunnel_id).await?;
    println!("✓ Tunnel closed\n");

    println!("═════════════════════════════════════════");
    println!("✅ VPN connection example completed successfully!");
    
    Ok(())
}