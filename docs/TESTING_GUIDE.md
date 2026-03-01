# VANTISVPN Testing Guide

## Overview

This guide covers the testing infrastructure for VANTISVPN, including unit tests, integration tests, benchmarks, and continuous integration.

## Test Structure

### Unit Tests

Unit tests are located within each module and test individual functions and methods in isolation.

**Location:** `src/core/*.rs` (within each module)

**Running unit tests:**
```bash
cd src/core
cargo test --lib
```

**Running specific unit tests:**
```bash
cd src/core
cargo test --lib test_name
```

### Integration Tests

Integration tests verify that multiple components work together correctly.

**Location:** `src/core/tests/`

**Running integration tests:**
```bash
cd src/core
cargo test --test integration_test
```

### Benchmarks

Benchmarks measure the performance of critical cryptographic operations.

**Location:** `src/core/benches/`

**Running benchmarks:**
```bash
cd src/core
cargo bench
```

**Benchmark results:** Results are saved in `src/core/target/criterion/`

## Test Coverage

### Current Test Coverage

- **Cryptographic Operations:** Key generation, encryption/decryption, hashing
- **Network Protocols:** WireGuard, QUIC, Stealth protocol
- **Security Features:** Kill switch, split tunneling, zero trust
- **Privacy Features:** Zero-knowledge login, IP rotation, GDPR compliance
- **Hardware Integration:** Router OS, YubiKey, Vantis OS

### Test Statistics

- **Total test files:** 30+
- **Unit tests:** 20+
- **Integration tests:** 5+
- **Benchmarks:** 6+

## Running Tests

### Quick Test Run

Use the provided test script for a comprehensive test run:

```bash
./run_tests.sh
```

This script will:
1. Check code compilation
2. Run unit tests
3. Run integration tests
4. Run clippy lints
5. Check code formatting
6. Optionally run benchmarks

### Manual Test Execution

#### Run all tests:
```bash
cd src/core
cargo test
```

#### Run tests with output:
```bash
cd src/core
cargo test -- --nocapture
```

#### Run tests in release mode (faster):
```bash
cd src/core
cargo test --release
```

#### Run specific test:
```bash
cd src/core
cargo test test_name
```

#### Run tests in specific module:
```bash
cd src/core
cargo test crypto::hash
```

## Writing Tests

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function_to_test(input);
        
        // Assert
        assert_eq!(result, "expected");
    }
}
```

### Integration Test Example

```rust
use vantis_core::crypto::keys::EphemeralKeyPair;
use vantis_core::crypto::cipher::Cipher;

#[test]
fn test_integration_scenario() {
    let key_pair = EphemeralKeyPair::generate().unwrap();
    let cipher = Cipher::new(key_pair.public_key().as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    
    let plaintext = b"test message";
    let ciphertext = cipher.encrypt(plaintext).unwrap();
    let decrypted = cipher.decrypt(&ciphertext).unwrap();
    
    assert_eq!(plaintext.to_vec(), decrypted);
}
```

### Benchmark Example

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        b.iter(|| {
            black_box(function_to_test())
        })
    });
}

criterion_group!(benches, bench_function);
criterion_main!(benches);
```

## Continuous Integration

### CI/CD Pipeline

The CI/CD pipeline automatically runs tests on every push and pull request.

**Configuration:** `.github/workflows/ci.yml`

**CI checks:**
- Code compilation on Linux, macOS, Windows
- Unit tests
- Integration tests
- Clippy lints
- Code formatting checks
- Security audits

### GitHub Actions

View CI results at:
```
https://github.com/vantisCorp/VantisVPN/actions
```

## Test Best Practices

### 1. Test Isolation

Each test should be independent and not rely on the state of other tests.

```rust
#[test]
fn test_independent() {
    // Setup
    let data = create_test_data();
    
    // Test
    let result = process_data(data);
    
    // Cleanup (if needed)
    cleanup_test_data();
}
```

### 2. Descriptive Test Names

Use clear, descriptive names that explain what the test verifies.

```rust
// Good
#[test]
fn test_encryption_with_valid_key_produces_ciphertext() {
    // ...
}

// Bad
#[test]
fn test_enc() {
    // ...
}
```

### 3. Test Edge Cases

Test boundary conditions and error cases.

```rust
#[test]
fn test_empty_input() {
    let result = process_data(b"");
    assert!(result.is_err());
}

#[test]
fn test_maximum_input() {
    let large_input = vec![0u8; MAX_SIZE];
    let result = process_data(&large_input);
    assert!(result.is_ok());
}
```

### 4. Use Assertions Effectively

Use appropriate assertion macros for different scenarios.

```rust
// Equality
assert_eq!(expected, actual);

// Inequality
assert_ne!(unexpected, actual);

// Boolean
assert!(condition);

// Error handling
assert!(result.is_err());
assert!(result.is_ok());
```

## Debugging Tests

### Running Tests with Debug Output

```bash
cd src/core
cargo test -- --nocapture
```

### Running Single Test with Backtrace

```bash
cd src/core
RUST_BACKTRACE=1 cargo test test_name
```

### Printing Test Output

```rust
#[test]
fn test_with_output() {
    println!("Debug information: {:?}", data);
    assert!(condition);
}
```

## Performance Testing

### Benchmarking

Run benchmarks to measure performance:

```bash
cd src/core
cargo bench
```

### Profiling

Use `cargo flamegraph` for detailed profiling:

```bash
cd src/core
cargo install flamegraph
cargo flamegraph --bench crypto_bench
```

## Test Coverage

### Installing Coverage Tool

```bash
cargo install cargo-tarpaulin
```

### Running Coverage Analysis

```bash
cd src/core
cargo tarpaulin --out Html
```

Coverage report will be generated in `tarpaulin-report.html`

## Troubleshooting

### Tests Timing Out

If tests timeout, try:
1. Running tests in release mode: `cargo test --release`
2. Running specific tests: `cargo test test_name`
3. Increasing timeout in CI configuration

### Flaky Tests

If tests are flaky (sometimes pass, sometimes fail):
1. Check for race conditions in async code
2. Ensure proper cleanup between tests
3. Use deterministic test data
4. Check for external dependencies (network, filesystem)

### Compilation Errors in Tests

If tests don't compile:
1. Check that all imports are correct
2. Verify that test-only dependencies are in `[dev-dependencies]`
3. Ensure test code uses the same API as production code

## Contributing Tests

When contributing new features:

1. **Write tests first** (Test-Driven Development)
2. Ensure all tests pass before submitting
3. Add integration tests for new features
4. Update this documentation if adding new test types
5. Run the full test suite: `./run_tests.sh`

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/index.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

## Support

For questions or issues with testing:
- Open an issue on GitHub: https://github.com/vantisCorp/VantisVPN/issues
- Check existing issues for similar problems
- Review CI logs for detailed error messages