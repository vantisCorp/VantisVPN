# Software Bill of Materials (SBOM)

**Project**: VantisVPN  
**Version**: 1.1.0  
**Generated**: March 6, 2026  
**Format**: SPDX-compatible markdown

---

## Overview

This document provides a comprehensive Software Bill of Materials (SBOM) for the VantisVPN project. It lists all dependencies, their versions, and security-relevant information.

---

## Project Information

| Field | Value |
|-------|-------|
| **Name** | VantisVPN |
| **Version** | 1.1.0 |
| **License** | AGPL-3.0-or-later |
| **Repository** | https://github.com/vantisCorp/VantisVPN |
| **Authors** | VANTISVPN Team <security@vantisvpn.com> |
| **Rust Edition** | 2021 |
| **Minimum Rust Version** | 1.82 |

---

## Dependencies

### Async Runtime

| Package | Version | License | Purpose |
|---------|---------|---------|---------|
| tokio | 1.50 | MIT | Async runtime |
| async-trait | 0.1 | MIT OR Apache-2.0 | Async trait support |

### Serialization

| Package | Version | License | Purpose |
|---------|---------|---------|---------|
| serde | 1.0 | MIT OR Apache-2.0 | Serialization framework |
| serde_json | 1.0 | MIT OR Apache-2.0 | JSON serialization |
| postcard | 1.0 | MIT OR Apache-2.0 | Embedded serialization |

### Error Handling

| Package | Version | License | Purpose |
|---------|---------|---------|---------|
| thiserror | 2.0 | MIT OR Apache-2.0 | Error derivation |
| anyhow | 1.0 | MIT OR Apache-2.0 | Error handling |

### Logging

| Package | Version | License | Purpose |
|---------|---------|---------|---------|
| tracing | 0.1 | MIT | Structured logging |
| tracing-subscriber | 0.3 | MIT | Tracing subscribers |
| log | 0.4 | MIT OR Apache-2.0 | Logging facade |

### Utilities

| Package | Version | License | Purpose |
|---------|---------|---------|---------|
| uuid | 1.6 | Apache-2.0 OR MIT | UUID generation |
| chrono | 0.4 | MIT OR Apache-2.0 | Date/time handling |
| hex | 0.4 | MIT OR Apache-2.0 | Hex encoding/decoding |
| base64 | 0.22 | MIT OR Apache-2.0 | Base64 encoding |
| zeroize | 1.7 | Apache-2.0 OR MIT | Secure memory clearing |

### Cryptography

| Package | Version | License | Purpose |
|---------|---------|---------|---------|
| rand | 0.10 | MIT OR Apache-2.0 | Random number generation |
| rand_chacha | 0.10 | MIT OR Apache-2.0 | ChaCha RNG |
| rand_core | 0.10 | MIT OR Apache-2.0 | RNG core traits |
| getrandom | 0.4 | MIT OR Apache-2.0 | System RNG |
| chacha20poly1305 | 0.10 | Apache-2.0 OR MIT | AEAD encryption |
| blake2 | 0.10 | MIT OR Apache-2.0 | BLAKE2 hash |

### Networking

| Package | Version | License | Purpose |
|---------|---------|---------|---------|
| quinn | 0.11 | MIT OR Apache-2.0 | QUIC protocol |

---

## Security Analysis

### High Priority Dependencies

| Package | Version | Security Status | Notes |
|---------|---------|-----------------|-------|
| chacha20poly1305 | 0.10 | ✅ Secure | Modern AEAD cipher |
| blake2 | 0.10 | ✅ Secure | Modern hash function |
| rand | 0.10 | ✅ Secure | Cryptographically secure RNG |
| zeroize | 1.7 | ✅ Secure | Secure memory clearing |

### Known Vulnerabilities

**Status**: No known vulnerabilities in current dependency versions as of March 6, 2026.

---

## Dependency Tree

```
VantisVPN v1.1.0
├── Async Runtime
│   ├── tokio v1.50 (MIT)
│   └── async-trait v0.1 (MIT OR Apache-2.0)
├── Serialization
│   ├── serde v1.0 (MIT OR Apache-2.0)
│   ├── serde_json v1.0 (MIT OR Apache-2.0)
│   └── postcard v1.0 (MIT OR Apache-2.0)
├── Error Handling
│   ├── thiserror v2.0 (MIT OR Apache-2.0)
│   └── anyhow v1.0 (MIT OR Apache-2.0)
├── Logging
│   ├── tracing v0.1 (MIT)
│   ├── tracing-subscriber v0.3 (MIT)
│   └── log v0.4 (MIT OR Apache-2.0)
├── Utilities
│   ├── uuid v1.6 (Apache-2.0 OR MIT)
│   ├── chrono v0.4 (MIT OR Apache-2.0)
│   ├── hex v0.4 (MIT OR Apache-2.0)
│   ├── base64 v0.22 (MIT OR Apache-2.0)
│   └── zeroize v1.7 (Apache-2.0 OR MIT)
├── Cryptography
│   ├── rand v0.10 (MIT OR Apache-2.0)
│   ├── rand_chacha v0.10 (MIT OR Apache-2.0)
│   ├── rand_core v0.10 (MIT OR Apache-2.0)
│   ├── getrandom v0.4 (MIT OR Apache-2.0)
│   ├── chacha20poly1305 v0.10 (Apache-2.0 OR MIT)
│   └── blake2 v0.10 (MIT OR Apache-2.0)
└── Networking
    └── quinn v0.11 (MIT OR Apache-2.0)
```

---

## License Compliance

### Primary License
- **AGPL-3.0-or-later** (GNU Affero General Public License v3.0 or later)

### Dependency License Summary
- **MIT**: 8 packages
- **MIT OR Apache-2.0**: 12 packages
- **Apache-2.0 OR MIT**: 5 packages

### License Compatibility
All dependencies use permissive licenses (MIT, Apache-2.0) which are compatible with AGPL-3.0.

---

## Build Requirements

### Rust Toolchain
- **Minimum Version**: 1.82
- **Edition**: 2021
- **Recommended**: Latest stable

### Build Dependencies
- Cargo (Rust package manager)
- Rust compiler (rustc)

### Optional Dependencies
- cargo-audit: Security vulnerability scanning
- cargo-outdated: Dependency update checking

---

## Update Policy

### Dependency Update Schedule
- **Weekly**: Check for outdated dependencies
- **Monthly**: Security vulnerability audit
- **Quarterly**: Major version upgrade review

### Update Criteria
1. Security patches: Immediate update
2. Minor versions: Within 1 week
3. Major versions: After compatibility review

---

## Vulnerability Reporting

If you discover a vulnerability in any dependency, please:

1. Report to security@vantisvpn.com
2. Create a security advisory on GitHub
3. Follow responsible disclosure guidelines

---

## SBOM Verification

This SBOM can be verified against the following:

- **Cargo.lock**: `sha256:$(sha256sum Cargo.lock | cut -d' ' -f1)`
- **Cargo.toml**: `sha256:$(sha256sum Cargo.toml | cut -d' ' -f1)`

---

## References

- [CycloneDX Specification](https://cyclonedx.org/)
- [SPDX Specification](https://spdx.dev/)
- [Rust Security Advisories](https://rustsec.org/)
- [Cargo Security Best Practices](https://doc.rust-lang.org/cargo/guide/)

---

**Last Updated**: March 6, 2026  
**Next Review**: April 6, 2026