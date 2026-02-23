# Changelog

All notable changes to VANTISVPN will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and documentation
- Rust core library with cryptographic primitives
- Ephemeral key management with auto-zeroization
- ChaCha20-Poly1305 authenticated encryption
- BLAKE2s hashing implementation
- Post-quantum cryptography placeholders (Kyber, Dilithium)
- Hybrid key exchange (classical + PQC)
- Network layer with IPv6 support
- QUIC transport protocol implementation
- WireGuard-like protocol with modifications
- Tunnel management with state machine
- Multiple tunnel support
- Privacy by Design architecture
- RAM-only server design
- Zero-knowledge authentication framework
- CI/CD pipeline with GitHub Actions
- Docker containerization
- Monitoring stack (Prometheus, Grafana)
- Logging stack (ELK)
- Service discovery (Consul)
- Makefile for common tasks
- Contributing guidelines
- Security policy
- Comprehensive documentation

### Changed
- Initial project setup

### Security
- Memory-safe implementation using Rust
- Automatic zeroization of sensitive data
- No-logs architecture (technical guarantee)
- Post-quantum cryptography ready

## [0.1.0] - 2024-XX-XX

### Added
- Initial release of VANTISVPN
- Core cryptographic library
- Network protocol implementation
- Basic tunnel management
- Documentation

### Security
- Privacy by Design implementation
- Ephemeral key management
- Secure random generation

---

## Version Format

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

## Categories

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes or improvements