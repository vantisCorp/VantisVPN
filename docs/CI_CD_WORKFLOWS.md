# CI/CD Workflows Documentation

## Overview

This repository uses GitHub Actions for continuous integration and continuous deployment. The CI/CD pipeline is configured to automatically build, test, and validate the VANTISVPN codebase.

## Workflow Triggers

The CI/CD pipeline is triggered by:
- Pushes to `main` and `develop` branches
- Pull requests targeting `main` or `develop` branches
- Manual workflow dispatch via GitHub UI

## Jobs

### 1. Build and Test

**Purpose**: Validates code builds and tests pass across all platforms

**Matrix Strategy**:
- **Operating Systems**: Ubuntu, macOS, Windows
- **Rust Version**: Stable

**Steps**:
1. Checkout code
2. Install Rust toolchain with clippy and rustfmt
3. Cache cargo registry and build artifacts
4. Check code formatting
5. Run Clippy linter
6. Build the workspace
7. Run all tests
8. Generate documentation

**Artifacts**: None

### 2. Security Audit

**Purpose**: Scans dependencies for security vulnerabilities

**Platform**: Ubuntu Latest

**Steps**:
1. Checkout code
2. Install Rust toolchain
3. Install `cargo-audit`
4. Run security audit

**Exit Criteria**: Fails if high/critical vulnerabilities found

### 3. Code Coverage

**Purpose**: Generates test coverage reports

**Platform**: Ubuntu Latest

**Steps**:
1. Checkout code
2. Install Rust toolchain
3. Install `cargo-tarpaulin`
4. Generate coverage report in XML format
5. Upload to Codecov (optional, requires CODECOV_TOKEN)

**Artifacts**: Coverage reports in `./coverage/` directory

### 4. Release Build

**Purpose**: Creates optimized release builds

**Platform**: Ubuntu Latest

**Dependencies**: Requires successful completion of Build & Test and Security Audit

**Steps**:
1. Checkout code
2. Install Rust toolchain
3. Build release version with optimizations
4. Upload build artifacts

**Artifacts**: Release binaries in `target/release/` (retained for 7 days)

## Environment Variables

- `CARGO_TERM_COLOR`: `always` - Enables colored cargo output
- `RUST_BACKTRACE`: `1` - Enables backtraces for debugging
- `CODECOV_TOKEN`: Optional - Required for Codecov integration

## Secrets Required

- `CODECOV_TOKEN` (Optional) - For uploading coverage reports to Codecov.io

## Caching

The workflow uses `Swatinem/rust-cache@v2` to cache:
- Cargo registry
- Git dependencies
- Build artifacts

Cache key format: `{os}-cargo-{Cargo.lock hash}`

## Fail-Fast Behavior

The Build and Test job uses `fail-fast: false`, meaning:
- Jobs will continue running even if one fails
- Provides visibility into failures across all platforms
- Security and release jobs depend on successful completion

## Actions Used

- `actions/checkout@v4` - Checkout repository code
- `dtolnay/rust-toolchain@stable` - Install Rust toolchain
- `Swatinem/rust-cache@v2` - Cache cargo dependencies
- `codecov/codecov-action@v4` - Upload coverage to Codecov
- `actions/upload-artifact@v4` - Upload build artifacts

## Status

**Current Status**: ⚠️ **GitHub Actions are currently disabled for this repository**

To enable CI/CD:
1. Go to Repository Settings → Actions → General
2. Enable "Allow all actions and reusable workflows"
3. Ensure GitHub Pro/Enterprise plan or Actions minutes are available

## Local Testing

To test workflows locally:

```bash
# Install act for local GitHub Actions testing
brew install act  # macOS
# or
cargo install act-action  # Rust

# Run workflow locally
act push -j build-and-test
```

## Troubleshooting

### Workflows Not Running
- Check if Actions are enabled in repository settings
- Verify repository has sufficient Actions minutes
- Check workflow permissions are set to "write"

### Build Failures
- Check Rust version compatibility
- Verify all dependencies are available
- Review build logs for specific errors

### Test Failures
- Run tests locally: `cargo test --workspace`
- Check for environment-specific issues
- Review test output for failing tests

## Contributing

When contributing:
1. Ensure all tests pass locally
2. Run `cargo fmt` before committing
3. Address any Clippy warnings
4. Create a PR against `main` or `develop` branch
5. The CI pipeline will automatically run

## Future Enhancements

Potential improvements:
- Add integration tests job
- Add performance benchmarking
- Add automated release creation
- Add Docker image building and pushing
- Add security scanning with CodeQL
- Add dependency updates automation