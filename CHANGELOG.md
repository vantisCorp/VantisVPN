# Changelog

All notable changes to VantisVPN will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0] - 2026-03-09

### Added
- Multilingual README translations for 8 languages: Polish, English, German, Chinese, Russian, Korean, Spanish, French
- Clickable language badges in main README linking to translation files
- Comprehensive documentation for all 252 public API items across 29 source files
- `PROJECT_COMPLETION_PLAN.md` with full project analysis and phased completion roadmap
- `.gitkeep` files in 11 empty placeholder directories to preserve monorepo structure
- `.gitleaks.toml` now tracked in repository (was previously gitignored)

### Changed
- Docker base image updated from `rust:1.93-slim` to `rust:1.94-slim`
- Version consistency enforced across all config files (CITATION.cff, package.json → 1.1.0)
- CITATION.cff date corrected to 2026-03-04
- README banner paths updated to `assets/banners/` directory
- CI/CD Pipeline: CodeQL updated to v4, Discord notification made resilient
- Dependencies updated via Dependabot (Rust crates, GitHub Actions, Docker)

### Fixed
- Removed orphan `&quot` file (454 lines of misnamed Rust source)
- Removed duplicate `src/core/Cargo.lock` (workspace uses root Cargo.lock)
- Removed duplicate banner SVGs from `assets/` (kept `assets/banners/`)
- Removed orphan `src/css/custom.css`
- Removed one-time fix script `fix_vpn_config_mtu.py`
- Removed 8 temporary/debug files from repository root
- Cleaned up 6 stale merged branches (local and remote)
- Fixed broken README links (banners, translations, asciinema placeholder)
- Commented out placeholder DOI in CITATION.cff

### Security
- All CI/CD workflows passing on main (CI/CD Pipeline, Security Scanning, Simple Test)
- Branch protection enforced on main (required: lint, build, test)
- Dependabot active for Rust crates, GitHub Actions, and Docker
- Security scanning: CodeQL v4, Gitleaks, Trivy, cargo-audit

### Documentation
- Zero `missing_docs` warnings (was 252)
- All public structs, enums, constants, methods, and modules documented
- Project completion plan with implementation status for all modules
- 8 translated README files with full project overview

## [1.1.0] - 2026-03-04

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