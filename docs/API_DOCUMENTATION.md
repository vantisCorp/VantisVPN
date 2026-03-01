# VANTISVPN API Documentation

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Modules](#core-modules)
3. [Cryptographic Primitives](#cryptographic-primitives)
4. [Network Protocols](#network-protocols)
5. [Tunnel Management](#tunnel-management)
6. [Security Features](#security-features)
7. [Privacy Features](#privacy-features)
8. [Hardware Integration](#hardware-integration)
9. [Error Handling](#error-handling)
10. [Examples](#examples)

---

## Getting Started

### Installation

Add VANTISVPN to your `Cargo.toml`:

```toml
[dependencies]
vantis-core = "1.0.0"
```

### Basic Usage

```rust
use vantis_core::{init, cleanup, Result};

fn main() -> Result<()> {
    // Initialize the library
    init()?;
    
    // Your code here
    
    // Cleanup before exit
    cleanup()?;
    
    Ok(())
}
```

---

## Core Modules

### Initialization

```rust
use vantis_core::{init, cleanup, Result};

/// Initialize VANTISVPN core
/// 
/// This function must be called before any other core functions.
/// It sets up logging and initializes subsystems.
pub fn init() -> Result<()>

/// Cleanup VANTISVPN core
/// 
/// This function should be called before shutdown to ensure
/// all sensitive data is securely cleared from memory.
pub fn cleanup() -> Result<()>
```

### Constants

```rust
/// VANTISVPN version
pub const VERSION: &str = "1.0.0";

/// Maximum packet size for VPN traffic
pub const MAX_PACKET_SIZE: usize = 1500;

/// Default MTU for VPN tunnel
pub const DEFAULT_MTU: u16 = 1420;

/// Timeout for establishing VPN connection (seconds)
pub const CONNECTION_TIMEOUT: u64 = 30;

/// Heartbeat interval for keepalive (seconds)
pub const HEARTBEAT_INTERVAL: u64 = 10;

/// Number of retransmission attempts
pub const MAX_RETRANSMISSIONS: u32 = 3;
```

---

## Cryptographic Primitives

### Key Management

```rust
use vantis_core::crypto::keys::{EphemeralKeyPair, Cipher, CipherSuite};

/// Generate a new ephemeral key pair
let key_pair = EphemeralKeyPair::generate()?;

/// Get public key
let public_key = key_pair.public_key();

/// Get private key
let private_key = key_pair.private_key();

/// Create a cipher for encryption
let cipher = Cipher::new(key, CipherSuite::ChaCha20Poly1305)?;

/// Encrypt data
let ciphertext = cipher.encrypt(plaintext, nonce)?;

/// Decrypt data
let plaintext = cipher.decrypt(ciphertext, nonce)?;
```

### Post-Quantum Cryptography

```rust
use vantis_core::crypto::pqc_full::{
    MlKemKeyPair, MlDsaKeyPair, HybridKeyExchange, PqcManager,
    MlKemSecurityLevel, MlDsaSecurityLevel
};

/// Generate ML-KEM key pair
let kem_keypair = MlKemKeyPair::generate(MlKemSecurityLevel::MlKem512)?;

/// Generate ML-DSA key pair
let dsa_keypair = MlDsaKeyPair::generate(MlDsaSecurityLevel::MlDsa44)?;

/// Perform hybrid key exchange
let hybrid = HybridKeyExchange::new(
    kem_keypair,
    classical_keypair
)?;
let shared_secret = hybrid.perform_exchange(peer_public_key)?;

/// Use PQC manager
let manager = PqcManager::new();
let kem_key = manager.generate_kem_key(MlKemSecurityLevel::MlKem768)?;
let dsa_key = manager.generate_dsa_key(MlDsaSecurityLevel::MlDsa65)?;
```

### Hashing

```rust
use vantis_core::crypto::hash::Hash;

/// Create hash instance
let hash = Hash::new()?;

/// Compute hash of data
let digest = hash.compute(data)?;

/// Compute keyed hash (HMAC)
let mac = hash.compute_keyed(data, key)?;
```

### Random Number Generation

```rust
use vantis_core::crypto::random::SecureRandom;

/// Create secure random instance
let rng = SecureRandom::new()?;

/// Generate random bytes
let bytes = rng.generate_bytes(32)?;

/// Generate random u64
let number = rng.generate_u64()?;

/// Generate random u32
let number = rng.generate_u32()?;

/// Generate random bool
let flag = rng.generate_bool()?;
```

---

## Network Protocols

### WireGuard Protocol

```rust
use vantis_core::network::wireguard_full::{
    WireGuardDevice, InterfaceConfig, PeerConfig
};

/// Create WireGuard device
let config = InterfaceConfig {
    private_key: private_key,
    public_key: public_key,
    listen_port: 51820,
    ..Default::default()
};

let device = WireGuardDevice::new(config)?;

/// Add peer
let peer_config = PeerConfig {
    public_key: peer_public_key,
    endpoint: "192.168.1.100:51820".parse()?,
    allowed_ips: vec!["0.0.0.0/0".parse()?],
    ..Default::default()
};

device.add_peer(peer_config)?;

/// Connect to peer
device.connect_peer(&peer_id).await?;

/// Send data
device.send_data(&peer_id, data).await?;

/// Receive data
let data = device.receive_data(&peer_id).await?;
```

### QUIC/HTTP/3

```rust
use vantis_core::network::quic_full::{
    QuicEndpoint, QuicConnection, QuicStream, QuicConfig
};

/// Create QUIC endpoint
let endpoint = QuicEndpoint::bind("0.0.0.0:4433")?;

/// Connect to remote endpoint
let connection = endpoint.connect("example.com:4433").await?;

/// Open stream
let mut stream = connection.open_stream().await?;

/// Send data
stream.send(data).await?;

/// Receive data
let data = stream.receive().await?;

/// Close stream
stream.close().await?;
```

### Stealth Protocol

```rust
use vantis_core::network::stealth::{StealthHandler, StealthConfig};

/// Create stealth handler
let config = StealthConfig {
    tls_mimicry: true,
    http2_obfuscation: true,
    padding_strategy: PaddingStrategy::Random,
    ..Default::default()
};

let handler = StealthHandler::new(config)?;

/// Obfuscate packet
let obfuscated = handler.obfuscate(packet)?;

/// Deobfuscate packet
let packet = handler.deobfuscate(obfuscated)?;
```

### Multi-Hop Routing

```rust
use vantis_core::network::multihop::{
    MultiHopManager, MultiHopConfig, Circuit
};

/// Create multi-hop manager
let config = MultiHopConfig {
    max_hops: 5,
    min_hops: 3,
    geographic_diversity: true,
    ..Default::default()
};

let manager = MultiHopManager::new(config)?;

/// Add VPN node
manager.add_node(node)?;

/// Create circuit
let circuit = manager.create_circuit().await?;

/// Send data through circuit
let response = circuit.send_data(data).await?;

/// Destroy circuit
circuit.destroy().await?;
```

---

## Tunnel Management

### Tunnel Manager

```rust
use vantis_core::tunnel::manager::TunnelManager;
use vantis_core::tunnel::state::TunnelState;

/// Create tunnel manager
let manager = TunnelManager::new()?;

/// Create tunnel
let tunnel_id = manager.create_tunnel(config).await?;

/// Connect tunnel
manager.connect(&tunnel_id).await?;

/// Get tunnel state
let state = manager.get_state(&tunnel_id)?;

/// Disconnect tunnel
manager.disconnect(&tunnel_id).await?;

/// Destroy tunnel
manager.destroy_tunnel(&tunnel_id).await?;
```

### Tunnel States

```rust
pub enum TunnelState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error(String),
}
```

---

## Security Features

### Kill Switch

```rust
use vantis_core::security::kill_switch::{
    KillSwitchManager, KillSwitchConfig, KillSwitchMode
};

/// Create kill switch manager
let config = KillSwitchConfig {
    mode: KillSwitchMode::Strict,
    enabled: true,
    ..Default::default()
};

let manager = KillSwitchManager::new(config)?;

/// Enable kill switch
manager.enable()?;

/// Disable kill switch
manager.disable()?;

/// Check if kill switch is active
let is_active = manager.is_active()?;
```

### Split Tunneling

```rust
use vantis_core::security::split_tunnel::{
    SplitTunnelManager, SplitTunnelConfig, SplitTunnelRule
};

/// Create split tunnel manager
let config = SplitTunnelConfig {
    mode: SplitTunnelMode::Exclude,
    ..Default::default()
};

let manager = SplitTunnelManager::new(config)?;

/// Add rule
let rule = SplitTunnelRule {
    application: "chrome.exe".to_string(),
    action: RuleType::Exclude,
    ..Default::default()
};

manager.add_rule(rule)?;

/// Get routing decision
let decision = manager.get_routing_decision("chrome.exe", "8.8.8.8")?;
```

### Remote Browser Isolation

```rust
use vantis_core::security::rbi::{RbiManager, RbiConfig, BrowserType};

/// Create RBI manager
let config = RbiConfig {
    browser_type: BrowserType::Chrome,
    isolation_level: IsolationLevel::Strict,
    ..Default::default()
};

let manager = RbiManager::new(config)?;

/// Create isolated browser session
let session = manager.create_session(url).await?;

/// Render frame
let frame = session.render_frame().await?;

/// Send input
session.send_input(input).await?;

/// Close session
session.close().await?;
```

### NetShield AI

```rust
use vantis_core::security::netshield::{NetShieldManager, NetShieldConfig};

/// Create NetShield manager
let config = NetShieldConfig {
    block_malware: true,
    block_phishing: true,
    block_tracking: true,
    ..Default::default()
};

let manager = NetShieldManager::new(config)?;

/// Query DNS
let response = manager.query_dns("example.com")?;

/// Add blocklist entry
manager.add_blocklist_entry(entry)?;
```

### Quantum Vault

```rust
use vantis_core::security::quantum_vault::{QuantumVault, VaultConfig};

/// Create quantum vault
let config = VaultConfig {
    master_password_hash: hash,
    ..Default::default()
};

let vault = QuantumVault::new(config)?;

/// Unlock vault
vault.unlock(master_password)?;

/// Add entry
vault.add_entry(username, password, url)?;

/// Get entry
let entry = vault.get_entry(username)?;

/// Update entry
vault.update_entry(username, new_password)?;

/// Delete entry
vault.delete_entry(username)?;

/// Lock vault
vault.lock()?;
```

### Zero Trust

```rust
use vantis_core::security::zero_trust::{ZeroTrust, ZeroTrustConfig, ZeroTrustPolicy};

/// Create zero trust manager
let config = ZeroTrustConfig {
    ..Default::default()
};

let manager = ZeroTrust::new(config)?;

/// Add policy
let policy = ZeroTrustPolicy {
    name: "Admin Access".to_string(),
    action: PolicyAction::RequireMfa,
    priority: 100,
    ..Default::default()
};

manager.add_policy(policy)?;

/// Evaluate access request
let decision = manager.evaluate_access(request)?;

/// Log access
manager.log_access(request, decision)?;
```

---

## Privacy Features

### Zero-Knowledge Login

```rust
use vantis_core::privacy::zk_login::{ZkLoginManager, ZkLoginConfig};

/// Create ZK login manager
let config = ZkLoginConfig {
    proof_type: ZkProofType::ZkSnark,
    ..Default::default()
};

let manager = ZkLoginManager::new(config)?;

/// Generate challenge
let challenge = manager.generate_challenge(user_id)?;

/// Generate response
let response = manager.generate_response(challenge, credentials)?;

/// Verify authentication
let result = manager.verify(response)?;
```

### Avantis ID

```rust
use vantis_core::privacy::avantis_id::{AvantisIdManager, AvantisIdConfig};

/// Create Avantis ID manager
let config = AvantisIdConfig {
    identity_type: IdentityType::Anonymous,
    ..Default::default()
};

let manager = AvantisIdManager::new(config)?;

/// Generate identity
let identity = manager.generate_identity()?;

/// Create proof
let proof = manager.create_proof(&identity)?;

/// Verify proof
let is_valid = manager.verify_proof(&identity, proof)?;
```

### IP Rotator

```rust
use vantis_core::privacy::ip_rotator::{IpRotator, RotatorConfig, RotationStrategy};

/// Create IP rotator
let config = RotatorConfig {
    strategy: RotationStrategy::Random,
    rotation_interval: Duration::from_secs(300),
    ..Default::default()
};

let rotator = IpRotator::new(config)?;

/// Add IP endpoint
rotator.add_endpoint(endpoint)?;

/// Get current IP
let current_ip = rotator.get_current_ip()?;

/// Rotate IP
rotator.rotate().await?;
```

### Anonymous Payments

```rust
use vantis_core::privacy::anonymous_payments::{
    AnonymousPaymentManager, PaymentConfig, PaymentMethod
};

/// Create payment manager
let config = PaymentConfig {
    ..Default::default()
};

let manager = AnonymousPaymentManager::new(config)?;

/// Create Monero payment
let payment = manager.create_monero_payment(amount, address)?;

/// Create Lightning payment
let payment = manager.create_lightning_payment(amount, invoice)?;

/// Create cash payment
let payment = manager.create_cash_payment(amount)?;

/// Check payment status
let status = manager.get_payment_status(payment_id)?;
```

### GDPR Compliance

```rust
use vantis_core::privacy::gdpr_compliance::{GdprCompliance, GdprConfig};

/// Create GDPR compliance manager
let config = GdprConfig {
    ..Default::default()
};

let manager = GdprCompliance::new(config)?;

/// Request data access
let data = manager.request_data_access(subject_id)?;

/// Request data deletion
manager.request_data_deletion(subject_id)?;

/// Request data portability
let data = manager.request_data_portability(subject_id)?;

/// Record consent
manager.record_consent(subject_id, consent_type, consent)?;
```

---

## Hardware Integration

### Router OS

```rust
use vantis_core::hardware::router_os::{
    RouterFirmware, RouterConfig, RouterFirmwareBuilder
};

/// Create router firmware
let config = RouterConfig {
    router_id: "vantis-router-001".to_string(),
    hostname: "VantisRouter".to_string(),
    ..Default::default()
};

let firmware = RouterFirmware::new(config)?;

/// Connect to VPN
firmware.connect_vpn().await?;

/// Add firewall rule
firmware.add_firewall_rule(rule)?;

/// Add port forwarding
firmware.add_port_forwarding(forwarding)?;

/// Generate firmware image
let image = firmware.generate_firmware_image()?;

/// Using builder
let firmware = RouterFirmwareBuilder::new()
    .router_id("vantis-router-001".to_string())
    .hostname("VantisRouter".to_string())
    .vpn_config(vpn_config)
    .build()?;
```

### YubiKey 2FA

```rust
use vantis_core::hardware::yubikey::{
    YubiKeyManager, YubiKeyConfig, YubiKeySlot
};

/// Create YubiKey manager
let config = YubiKeyConfig {
    enabled: true,
    require_for_login: true,
    ..Default::default()
};

let manager = YubiKeyManager::new(config)?;

/// Register YubiKey
manager.register_key(
    key_id,
    public_id,
    user_id,
    slot1_config,
    slot2_config,
)?;

/// Generate challenge
let challenge = manager.generate_challenge(key_id, YubiKeySlot::Slot1)?;

/// Verify response
let is_valid = manager.verify_challenge_response(key_id, response)?;

/// Verify OTP
let is_valid = manager.verify_otp(key_id, otp)?;
```

### Vantis OS

```rust
use vantis_core::hardware::vantis_os::{
    VantisOsBuilder, VantisOsImage, BootConfig, PersistenceConfig
};

/// Create Vantis OS image
let image = VantisOsBuilder::new()
    .os_name("Vantis OS".to_string())
    .version("1.0.0".to_string())
    .boot_config(BootConfig::default())
    .persistence_config(PersistenceConfig::default())
    .build()?;

/// Generate ISO
image.generate_iso(output_path)?;

/// Generate USB image
image.generate_usb_image(output_path)?;

/// Verify integrity
let is_valid = image.verify_integrity(checksum)?;
```

---

## Error Handling

### Error Types

```rust
use vantis_core::error::{VantisError, Result};

pub enum VantisError {
    /// Cryptographic error
    Crypto(String),
    
    /// Network error
    Network(String),
    
    /// Tunnel error
    Tunnel(String),
    
    /// Configuration error
    Config(String),
    
    /// Authentication error
    AuthenticationFailed(String),
    
    /// Not found error
    NotFound(String),
    
    /// Invalid data error
    InvalidData(String),
    
    /// IO error
    Io(std::io::Error),
}
```

### Error Handling Example

```rust
use vantis_core::{Result, VantisError};

fn example_function() -> Result<()> {
    // Your code here
    
    Ok(())
}

fn main() {
    match example_function() {
        Ok(_) => println!("Success!"),
        Err(VantisError::Crypto(msg)) => {
            eprintln!("Crypto error: {}", msg);
        }
        Err(VantisError::Network(msg)) => {
            eprintln!("Network error: {}", msg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

---

## Examples

### Complete VPN Connection Example

```rust
use vantis_core::{
    init, cleanup,
    crypto::keys::EphemeralKeyPair,
    network::wireguard_full::{WireGuardDevice, InterfaceConfig, PeerConfig},
    tunnel::manager::TunnelManager,
    Result
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize library
    init()?;
    
    // Generate keys
    let key_pair = EphemeralKeyPair::generate()?;
    
    // Create WireGuard device
    let interface_config = InterfaceConfig {
        private_key: key_pair.private_key().clone(),
        public_key: key_pair.public_key().clone(),
        listen_port: 51820,
        ..Default::default()
    };
    
    let device = WireGuardDevice::new(interface_config)?;
    
    // Add peer
    let peer_config = PeerConfig {
        public_key: peer_public_key,
        endpoint: "vpn.example.com:51820".parse()?,
        allowed_ips: vec!["0.0.0.0/0".parse()?],
        ..Default::default()
    };
    
    device.add_peer(peer_config)?;
    
    // Connect to peer
    device.connect_peer(&peer_id).await?;
    
    // Create tunnel
    let tunnel_manager = TunnelManager::new()?;
    let tunnel_id = tunnel_manager.create_tunnel(config).await?;
    tunnel_manager.connect(&tunnel_id).await?;
    
    // Send/receive data
    device.send_data(&peer_id, data).await?;
    let received = device.receive_data(&peer_id).await?;
    
    // Cleanup
    tunnel_manager.disconnect(&tunnel_id).await?;
    device.disconnect_vpn().await?;
    cleanup()?;
    
    Ok(())
}
```

### Multi-Hop Routing Example

```rust
use vantis_core::{
    init, cleanup,
    network::multihop::{MultiHopManager, MultiHopConfig, VpnNode},
    Result
};

#[tokio::main]
async fn main() -> Result<()> {
    init()?;
    
    // Create multi-hop manager
    let config = MultiHopConfig {
        max_hops: 5,
        min_hops: 3,
        geographic_diversity: true,
        ..Default::default()
    };
    
    let manager = MultiHopManager::new(config)?;
    
    // Add VPN nodes
    let node1 = VpnNode {
        node_id: "node1".to_string(),
        location: "US".to_string(),
        endpoint: "us1.vpn.com:51820".parse()?,
        ..Default::default()
    };
    
    let node2 = VpnNode {
        node_id: "node2".to_string(),
        location: "DE".to_string(),
        endpoint: "de1.vpn.com:51820".parse()?,
        ..Default::default()
    };
    
    manager.add_node(node1)?;
    manager.add_node(node2)?;
    
    // Create circuit
    let circuit = manager.create_circuit().await?;
    
    // Send data through circuit
    let response = circuit.send_data(data).await?;
    
    // Destroy circuit
    circuit.destroy().await?;
    
    cleanup()?;
    Ok(())
}
```

### Zero-Knowledge Authentication Example

```rust
use vantis_core::{
    init, cleanup,
    privacy::zk_login::{ZkLoginManager, ZkLoginConfig, ZkProofType},
    Result
};

#[tokio::main]
async fn main() -> Result<()> {
    init()?;
    
    // Create ZK login manager
    let config = ZkLoginConfig {
        proof_type: ZkProofType::ZkSnark,
        ..Default::default()
    };
    
    let manager = ZkLoginManager::new(config)?;
    
    // Generate challenge
    let challenge = manager.generate_challenge(user_id)?;
    
    // Generate response (client-side)
    let response = manager.generate_response(challenge, credentials)?;
    
    // Verify authentication (server-side)
    let result = manager.verify(response)?;
    
    if result.is_authenticated {
        println!("Authentication successful!");
    }
    
    cleanup()?;
    Ok(())
}
```

---

## Support

For more information, see:
- [Project README](../README.md)
- [Architecture Documentation](architecture/)
- [Security Policy](../SECURITY.md)
- [Contributing Guidelines](../CONTRIBUTING.md)

For security issues, contact: security@vantisvpn.com

---

*Last Updated: 2024*
*Version: 1.0.0*