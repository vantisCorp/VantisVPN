# VANTISVPN Examples

This directory contains example applications that demonstrate various features of the VANTISVPN system.

## Available Examples

### 1. Demo Application (`demo.rs`)
A comprehensive demo that showcases core VANTISVPN functionality:
- Ephemeral key generation
- Cryptographic hashing (BLAKE2s)
- Secure encryption/decryption
- Cryptographically secure random number generation
- Multiple encryption rounds
- Performance metrics

**Run:**
```bash
cargo run --example demo
```

### 2. Simple VPN Connection (`simple_vpn.rs`)
Demonstrates basic VPN tunnel establishment:
- Client key generation
- Tunnel manager initialization
- Encryption context setup
- Tunnel creation and management
- Data transmission through tunnel
- Tunnel closure

**Run:**
```bash
cargo run --example simple_vpn
```

### 3. Stealth Mode (`stealth_mode.rs`)
Shows traffic obfuscation to make VPN traffic indistinguishable from normal HTTPS:
- TLS 1.3 mimicry
- HTTP/2 obfuscation
- Domain fronting
- Packet size normalization
- Timing obfuscation with jitter

**Run:**
```bash
cargo run --example stealth_mode
```

### 4. MultiHop+ Onion Routing (`multihop.rs`)
Demonstrates multi-hop routing through multiple VPN servers:
- VPN node management
- Circuit creation through multiple hops
- Layered encryption (onion routing)
- Geographic diversity
- Path obfuscation
- Circuit statistics

**Run:**
```bash
cargo run --example multihop
```

## Running Examples

### Run a specific example:
```bash
cargo run --example <example_name>
```

### Run with release optimizations:
```bash
cargo run --release --example <example_name>
```

### Run with logging:
```bash
RUST_LOG=debug cargo run --example <example_name>
```

## Example Features

### Cryptographic Operations
- **Key Generation:** Ephemeral key pairs for secure communication
- **Hashing:** BLAKE2s for integrity verification
- **Encryption:** ChaCha20-Poly1305 authenticated encryption
- **Random Generation:** Cryptographically secure random numbers

### Network Protocols
- **WireGuard:** Modified WireGuard protocol with VANTISVPN enhancements
- **QUIC/HTTP/3:** Modern transport layer with 0-RTT support
- **Stealth Protocol:** Traffic obfuscation to resist DPI
- **MultiHop+:** Onion routing through multiple servers

### Security Features
- **Post-Quantum Cryptography:** Kyber/ML-KEM and Dilithium/ML-DSA
- **Zero-Knowledge Authentication:** Privacy-preserving login
- **Kill Switch:** Network-level protection
- **Split Tunneling:** Selective routing

### Privacy Features
- **IP Rotation:** Dynamic IP address changes
- **Anonymous Payments:** Monero, Lightning Network, cash
- **GDPR Compliance:** Privacy by design implementation

## Building Examples

All examples are built automatically when you run them. To build all examples without running:

```bash
cargo build --examples
```

To build examples in release mode:

```bash
cargo build --release --examples
```

## Customizing Examples

You can modify the examples to test different configurations:

### Change encryption cipher:
```rust
use vantis_core::crypto::cipher::CipherSuite;

// Use different cipher suite
let cipher = Cipher::new(key, CipherSuite::ChaCha20Poly1305)?;
```

### Adjust stealth mode settings:
```rust
let stealth_config = StealthConfig {
    tls_mimicry: true,
    http2_obfuscation: true,
    padding_strategy: PaddingStrategy::Random,
    timing_obfuscation: true,
    jitter_ms: 100, // Adjust jitter
};
```

### Configure MultiHop+:
```rust
let multihop_config = MultiHopConfig {
    min_hops: 2,
    max_hops: 5, // Adjust number of hops
    geographic_diversity: true,
    latency_optimization: true,
};
```

## Troubleshooting

### Example fails to run:
1. Ensure you're in the project root directory
2. Check that dependencies are installed: `cargo build`
3. Try running with more verbose output: `RUST_LOG=debug cargo run --example <name>`

### Performance issues:
1. Run in release mode: `cargo run --release --example <name>`
2. Check system resources (CPU, memory)
3. Reduce the number of operations in the example

### Cryptographic errors:
1. Ensure proper key generation
2. Check that encryption/decryption contexts match
3. Verify data integrity before encryption

## Learning Resources

- [API Documentation](../docs/API_DOCUMENTATION.md)
- [User Guide](../docs/USER_GUIDE.md)
- [Developer Guide](../docs/DEVELOPER_GUIDE.md)
- [Testing Guide](../docs/TESTING_GUIDE.md)

## Contributing Examples

To contribute a new example:

1. Create a new `.rs` file in this directory
2. Add comprehensive comments explaining the code
3. Include error handling
4. Add the example to this README
5. Test the example thoroughly
6. Submit a pull request

## Support

For questions or issues:
- Open an issue on GitHub: https://github.com/vantisCorp/VantisVPN/issues
- Check existing issues for similar problems
- Review the API documentation for detailed information