# Changelog

All notable changes to VantisVPN will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Monorepo architecture with Turborepo structure
- World's most advanced README with extensive features
- GPG signing configuration support
- Gitleaks secrets scanning
- Pre-commit hooks for code quality
- DevContainers for consistent development environment
- EditorConfig for consistent editor settings
- Prettier configuration for Markdown formatting
- ESLint configuration with TypeScript support
- CITATION.cff for academic citations
- Comprehensive Makefile with colored output
- Dual licensing (AGPL v3)
- Post-quantum cryptography feature flags (placeholders)
- Hardware-level optimization feature flags
- Support for ML-KEM and ML-DSA (NIST FIPS 203/204) - placeholders

### Changed
- **BREAKING**: Complete repository restructure from monolith to monorepo
- **BREAKING**: Rust version updated from 1.70 to 1.82
- **BREAKING**: Replaced unmaintained `bincode` with `postcard`
- Tokio updated to 1.50.0 (latest)
- Rand updated to 0.10.0 (latest stable)
- Quinn updated to 0.11.9 (latest)
- Serde updated to 1.0.228 (latest)
- Thiserror updated to 2.0.18 (latest)
- Getrandom updated to 0.4
- Zeroize updated to 1.7 with zeroize_derive
- Base64 updated to 0.22
- License changed from "Proprietary" to "AGPL-3.0-or-later"

### Security
- Added Gitleaks configuration for secrets detection
- Configured pre-commit hooks for security scanning
- Added post-quantum cryptography preparation
- Zero Trust architecture documentation
- Branch protection rules guidance
- Private vulnerability reporting setup

### Deprecated
- `bincode` crate (unmaintained) - replaced with `postcard`

### Removed
- Old monolithic directory structure
- Outdated dependency versions
- Legacy build configurations

## [1.1.0] - 2025-01-XX

### Added
- Initial monorepo structure with apps/, packages/, infra/
- Feature-Sliced Design (FSD) architecture
- Turborepo configuration
- DevContainer configuration
- Comprehensive documentation structure

### Changed
- Migrated to workspace-based Cargo configuration
- Updated all dependencies to latest stable versions

## [1.0.0] - 2024-XX-XX

### Added
- Initial release
- Core VPN functionality
- WireGuard protocol support
- QUIC protocol implementation
- Basic encryption
- Cross-platform support

---

## Dependency Versions (Current)

| Dependency | Version | Status |
|------------|---------|--------|
| Rust | 1.82 | ✅ Latest Stable |
| Tokio | 1.50 | ✅ Latest |
| Rand | 0.10.0 | ✅ Latest Stable |
| Quinn | 0.11.9 | ✅ Latest |
| Serde | 1.0.228 | ✅ Latest |
| Thiserror | 2.0.18 | ✅ Latest |
| Postcard | 1.0 | ✅ Latest (replaces bincode) |
| Getrandom | 0.4 | ✅ Latest |
| Zeroize | 1.7 | ✅ Latest |
| Chacha20poly1305 | 0.10 | ✅ Latest |
| Blake2 | 0.10 | ✅ Latest |
| X25519-dalek | 2.0 | ✅ Latest |
| Ed25519-dalek | 2.0 | ✅ Latest |

---

## Migration Guide

### From 1.0.x to 1.1.0

#### Breaking Changes

1. **Repository Structure**
   ```bash
   # Old structure
   src/
   
   # New structure
   apps/
   packages/
   infra/
   src/core/
   ```

2. **Bincode → Postcard**
   ```toml
   # Old
   bincode = "2.0"
   
   # New
   postcard = { version = "1.0", features = ["alloc"] }
   ```

   ```rust
   // Old
   use bincode::{serialize, deserialize};
   
   // New
   use postcard::{to_allocvec, from_bytes};
   ```

3. **Workspace Dependencies**
   ```toml
   # Now use workspace dependencies
   tokio = { workspace = true }
   serde = { workspace = true }
   ```

4. **Rust Version**
   - Minimum Rust version: 1.82
   - Update your toolchain: `rustup update stable`

---

## Security Policy

For security vulnerabilities, please see [SECURITY.md](SECURITY.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

This project is licensed under AGPL-3.0-or-later - see the [LICENSE](LICENSE) file for details.