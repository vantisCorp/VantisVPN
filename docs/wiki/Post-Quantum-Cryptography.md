# Post-Quantum Cryptography

This document describes VantisVPN's implementation of post-quantum cryptographic algorithms.

## Table of Contents

- [Overview](#overview)
- [Why Post-Quantum?](#why-post-quantum)
- [Algorithms](#algorithms)
- [Implementation](#implementation)
- [Performance](#performance)
- [Migration Guide](#migration-guide)

## Overview

VantisVPN is at the forefront of VPN security by implementing post-quantum cryptography (PQC) algorithms standardized by NIST. This ensures your VPN connections remain secure even against future quantum computers.

### What is Post-Quantum Cryptography?

Post-quantum cryptography refers to cryptographic algorithms that are believed to be secure against attacks by quantum computers. While quantum computers powerful enough to break current encryption don't exist yet, the threat is real:

- **Harvest Now, Decrypt Later**: Adversaries can store encrypted traffic today and decrypt it when quantum computers become available
- **Long-term Secrets**: Data that needs to remain secret for decades should be protected now

### VantisVPN's Approach

We use a **hybrid approach** combining classical and post-quantum algorithms:

```
┌─────────────────────────────────────────────────────────────┐
│                    Hybrid Key Exchange                       │
│                                                              │
│   Classical (X25519)  +  Post-Quantum (ML-KEM)  =  Secure   │
│         256-bit              768-bit              Key       │
│                                                              │
│   If either algorithm is compromised, connection remains    │
│   secure. Both must be broken to compromise the session.    │
└─────────────────────────────────────────────────────────────┘
```

## Why Post-Quantum?

### The Quantum Threat

Quantum computers pose a significant threat to current cryptographic systems:

| Algorithm | Classical Security | Quantum Security |
|-----------|-------------------|------------------|
| RSA-2048 | 112-bit | Broken (Shor's Algorithm) |
| ECDHE (P-256) | 128-bit | Broken (Shor's Algorithm) |
| AES-256 | 256-bit | 128-bit (Grover's Algorithm) |
| ChaCha20 | 256-bit | 128-bit (Grover's Algorithm) |

### Timeline Estimates

| Milestone | Estimated Year |
|-----------|----------------|
| Quantum computers with 1000 qubits | 2025-2027 |
| Cryptographically relevant quantum computers | 2030-2035 |
| RSA/ECC widely broken | 2035-2040 |

### Why Act Now?

1. **Harvest Now, Decrypt Later**: Your VPN traffic could be stored today
2. **Regulatory Requirements**: Some industries require quantum-safe encryption
3. **Long-term Data Protection**: Data needing decades of protection
4. **Future-proofing**: Smooth transition before quantum computers arrive

## Algorithms

### ML-KEM (Kyber)

**ML-KEM** (Module-Lattice-Based Key-Encapsulation Mechanism) is a NIST-standardized algorithm for key exchange.

| Parameter Set | Security Level | Key Size | Ciphertext Size |
|---------------|---------------|----------|-----------------|
| ML-KEM-512 | NIST Level 1 (AES-128) | 800 bytes | 768 bytes |
| ML-KEM-768 | NIST Level 3 (AES-192) | 1184 bytes | 1088 bytes |
| ML-KEM-1024 | NIST Level 5 (AES-256) | 1568 bytes | 1568 bytes |

VantisVPN uses **ML-KEM-768** by default, providing NIST Level 3 security (equivalent to AES-192).

### ML-DSA (Dilithium)

**ML-DSA** (Module-Lattice-Based Digital Signature Algorithm) is used for digital signatures.

| Parameter Set | Security Level | Public Key Size | Signature Size |
|---------------|---------------|-----------------|----------------|
| ML-DSA-44 | NIST Level 2 | 1312 bytes | 2420 bytes |
| ML-DSA-65 | NIST Level 3 | 1952 bytes | 3293 bytes |
| ML-DSA-87 | NIST Level 5 | 2592 bytes | 4595 bytes |

VantisVPN uses **ML-DSA-65** for server signatures.

### Hybrid Schemes

We combine classical and post-quantum algorithms:

| Component | Classical | Post-Quantum | Combined |
|-----------|-----------|--------------|----------|
| Key Exchange | X25519 | ML-KEM-768 | Hybrid KEM |
| Signatures | Ed25519 | ML-DSA-65 | Hybrid Signatures |
| Encryption | ChaCha20-Poly1305 | ChaCha20-Poly1305 | ChaCha20-Poly1305 |

## Implementation

### Enabling Post-Quantum Cryptography

```bash
# Enable post-quantum mode
vantisvpn config set security.encryption quantum-safe

# Verify configuration
vantisvpn config get security.encryption
```

### Configuration

```yaml
security:
  encryption:
    # Options: standard, high, quantum-safe
    level: quantum-safe
    # Post-quantum algorithms
    post_quantum:
      # Key encapsulation mechanism
      kem: ml-kem-768
      # Digital signatures
      signature: ml-dsa-65
      # Enable hybrid mode (recommended)
      hybrid: true
```

### Connection Process

```
Client                                    Server
  │                                         │
  │ ──────── ClientHello ──────────────────>│
  │         Supported Groups:               │
  │         - x25519                        │
  │         - ml-kem-768                    │
  │                                         │
  │ <────── ServerHello ─────────────────── │
  │         Selected: x25519+ml-kem-768     │
  │                                         │
  │ ─────── Key Share (X25519) ────────────>│
  │                                         │
  │ ─────── Encapsulated Key (ML-KEM) ─────>│
  │                                         │
  │ <────── Finished (Hybrid Signature) ─── │
  │                                         │
  │        [Quantum-Safe Channel]           │
```

### API Usage

```rust
use vantis_core::crypto::{KeyExchange, Signature};

// Hybrid key exchange
let mut key_exchange = KeyExchange::new_hybrid()?;

// Generate key pair
let (public_key, secret_key) = key_exchange.generate_keypair()?;

// Encapsulate shared secret
let (ciphertext, shared_secret) = key_exchange.encapsulate(&public_key)?;

// Decapsulate shared secret
let shared_secret = key_exchange.decapsulate(&secret_key, &ciphertext)?;
```

## Performance

### Benchmark Results

Tested on Intel i9-13900K @ 5.8 GHz:

| Operation | Algorithm | Time (μs) | Throughput |
|-----------|-----------|-----------|------------|
| KeyGen | X25519 | 25 | - |
| KeyGen | ML-KEM-768 | 45 | - |
| KeyGen | Hybrid | 70 | - |
| Encapsulate | X25519 | 30 | - |
| Encapsulate | ML-KEM-768 | 55 | - |
| Encapsulate | Hybrid | 85 | - |
| Sign | Ed25519 | 15 | - |
| Sign | ML-DSA-65 | 95 | - |
| Verify | Ed25519 | 45 | - |
| Verify | ML-DSA-65 | 85 | - |
| Encrypt | ChaCha20-Poly1305 | - | 5.2 GB/s |

### Network Impact

| Metric | Standard | Quantum-Safe | Difference |
|--------|----------|--------------|------------|
| Handshake Size | 1.2 KB | 3.8 KB | +2.6 KB |
| Handshake Time | 12 ms | 18 ms | +6 ms |
| CPU Usage | 2% | 3% | +1% |
| Memory | 8 MB | 12 MB | +4 MB |

### Optimization Tips

1. **Use Hardware Acceleration**
   ```yaml
   advanced:
     hardware_acceleration: true
   ```

2. **Connection Pooling**
   ```yaml
   advanced:
     connection_pool: true
   ```

3. **Session Resumption**
   ```yaml
   security:
     session_resumption: true
   ```

## Migration Guide

### For Users

1. **Update to Latest Version**
   ```bash
   vantisvpn update
   ```

2. **Enable Quantum-Safe Mode**
   ```bash
   vantisvpn config set security.encryption quantum-safe
   ```

3. **Verify Connection**
   ```bash
   vantisvpn status --detailed
   # Look for "Encryption: quantum-safe"
   ```

### For Developers

1. **Update Dependencies**
   ```toml
   [dependencies]
   vantis-core = { version = "1.1", features = ["post-quantum"] }
   ```

2. **Enable Feature Flag**
   ```toml
   [features]
   default = ["std", "post-quantum"]
   post-quantum = []
   ```

3. **Update Code**
   ```rust
   // Before
   let crypto = Crypto::new_standard();
   
   // After
   let crypto = Crypto::new_quantum_safe();
   ```

## FAQ

### Is post-quantum cryptography slower?

Yes, but the performance impact is minimal:
- ~6ms additional handshake time
- ~2.6 KB additional data per connection
- Negligible impact on throughput

### Do I need special hardware?

No. Post-quantum algorithms run on standard hardware. Hardware acceleration can improve performance but is not required.

### What if quantum computers never become a threat?

That's the best-case scenario! The hybrid approach ensures you remain secure with classical algorithms regardless of quantum developments.

### Can I disable post-quantum cryptography?

Yes, but not recommended:
```yaml
security:
  encryption: high  # or 'standard'
```

## References

- [NIST Post-Quantum Cryptography Standardization](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [FIPS 203: ML-KEM](https://csrc.nist.gov/pubs/fips/203/final)
- [FIPS 204: ML-DSA](https://csrc.nist.gov/pubs/fips/204/final)
- [Open Quantum Safe](https://openquantumsafe.org/)

## See Also

- [Security Model](Security-Model)
- [Configuration Guide](Configuration)
- [Cryptographic Design](Cryptographic-Design)