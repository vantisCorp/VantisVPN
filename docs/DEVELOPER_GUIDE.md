# VANTISVPN Developer Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Development Environment](#development-environment)
4. [Project Structure](#project-structure)
5. [Coding Standards](#coding-standards)
6. [Testing](#testing)
7. [Building](#building)
8. [Debugging](#debugging)
9. [Contributing](#contributing)
10. [Pull Request Process](#pull-request-process)
11. [Release Process](#release-process)
12. [Architecture](#architecture)
13. [Performance Optimization](#performance-optimization)
14. [Security Best Practices](#security-best-practices)

---

## Introduction

Welcome to the VANTISVPN developer guide! This document will help you get started with contributing to the VANTISVPN project.

### Project Goals

- **Security First**: Military-grade security with post-quantum cryptography
- **Privacy by Design**: Zero-logs, zero-knowledge architecture
- **Performance**: Low latency, high throughput
- **Usability**: Easy to use for end users
- **Open Source**: Fully auditable and transparent

### Development Philosophy

- **Memory Safety**: Rust ensures memory safety
- **Zero-Cost Abstractions**: No runtime overhead
- **Async/Await**: Non-blocking I/O with Tokio
- **Modular Design**: Clean separation of concerns
- **Test-Driven**: Comprehensive test coverage

---

## Getting Started

### Prerequisites

- **Rust**: 1.93.1 or later
- **Cargo**: Included with Rust
- **Git**: 2.30 or later
- **Docker**: 24.0 or later (optional)
- **Make**: For building

### Clone the Repository

```bash
git clone https://github.com/vantisCorp/VantisVPN.git
cd VANTISVPN
```

### Install Dependencies

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install development tools
cargo install cargo-watch
cargo install cargo-edit
cargo install cargo-audit
cargo install cargo-outdated
```

### Build the Project

```bash
# Build debug version
cd src/core
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

---

## Development Environment

### IDE Setup

#### VS Code

Install the following extensions:
- rust-analyzer
- CodeLLDB
- Even Better TOML
- Error Lens

#### IntelliJ IDEA

Install the Rust plugin:
- File → Settings → Plugins
- Search for "Rust"
- Install and restart

#### Vim/Neovim

Install vim-plug and add to `.vimrc`:
```vim
Plug 'rust-lang/rust.vim'
Plug 'simrat39/rust-tools.nvim'
```

### Configuration

#### VS Code Settings

Create `.vscode/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.procMacro.enable": true,
  "editor.formatOnSave": true,
  "editor.rulers": [100]
}
```

#### Git Hooks

Install pre-commit hooks:
```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install
```

---

## Project Structure

```
VANTISVPN/
├── docs/                    # Documentation
│   ├── architecture/        # Architecture docs
│   ├── API_DOCUMENTATION.md
│   ├── USER_GUIDE.md
│   ├── DEPLOYMENT_GUIDE.md
│   └── DEVELOPER_GUIDE.md
├── src/
│   ├── core/               # Core Rust library
│   │   ├── crypto/         # Cryptographic primitives
│   │   ├── network/        # Network protocols
│   │   ├── tunnel/         # Tunnel management
│   │   ├── server/         # Server infrastructure
│   │   ├── security/       # Security features
│   │   ├── privacy/        # Privacy features
│   │   ├── ui/             # UI components
│   │   ├── audit/          # Audit and compliance
│   │   ├── hardware/       # Hardware integration
│   │   ├── error.rs        # Error types
│   │   ├── config.rs       # Configuration
│   │   ├── utils.rs        # Utilities
│   │   └── lib.rs          # Library entry point
│   ├── ui/                 # Tauri UI application
│   └── tests/              # Integration tests
├── specs/                  # Specifications
├── .github/                # GitHub workflows
│   └── workflows/          # CI/CD pipelines
├── Cargo.toml              # Workspace configuration
├── Cargo.lock              # Dependency lock file
├── Makefile                # Build automation
├── docker-compose.yml      # Docker services
├── Dockerfile              # Container image
└── README.md               # Project README
```

### Module Responsibilities

#### Crypto Module
- Key generation and management
- Encryption/decryption
- Post-quantum cryptography
- Hashing and random numbers

#### Network Module
- WireGuard protocol
- QUIC/HTTP/3
- Stealth protocol
- Multi-hop routing

#### Tunnel Module
- Tunnel lifecycle management
- State machine
- Connection pooling

#### Server Module
- RAM-only architecture
- Confidential computing
- Secure boot
- Smart routing

#### Security Module
- Kill switch
- Split tunneling
- Remote browser isolation
- NetShield AI
- Quantum vault
- Zero trust

#### Privacy Module
- Zero-knowledge login
- Avantis ID
- IP rotator
- Anonymous payments
- GDPR compliance

#### UI Module
- Tauri framework
- Context-aware UI
- 3D visualization
- Biometric auth

#### Audit Module
- No-logs audit
- Security pentest
- Compliance (PCI DSS, SOC 2, HITRUST)

#### Hardware Module
- Router OS
- YubiKey integration
- Vantis OS

---

## Coding Standards

### Rust Style Guide

Follow the official Rust style guide:
- Use `cargo fmt` for formatting
- Maximum line length: 100 characters
- Use 4 spaces for indentation
- Use snake_case for variables and functions
- Use PascalCase for types and traits

### Naming Conventions

```rust
// Variables and functions: snake_case
let user_id = 123;
fn calculate_hash(data: &[u8]) -> Vec<u8> {}

// Types and traits: PascalCase
struct UserProfile {}
trait Hashable {}

// Constants: SCREAMING_SNAKE_CASE
const MAX_CONNECTIONS: usize = 10000;

// Acronyms: keep uppercase
let http_client = HttpClient::new();
let api_key = "abc123";
```

### Documentation

```rust
/// Brief description of what this does.
///
/// More detailed explanation if needed.
///
/// # Examples
///
/// ```
/// use vantis_core::crypto::hash::Hash;
///
/// let hash = Hash::new()?;
/// let digest = hash.compute(b"hello")?;
/// ```
///
/// # Errors
///
/// Returns an error if the hash cannot be computed.
///
/// # Panics
///
/// Panics if the input is empty.
pub fn example_function(input: &[u8]) -> Result<Vec<u8>> {
    // Implementation
}
```

### Error Handling

```rust
// Use Result<T, VantisError> for fallible functions
pub fn connect_to_server(address: &str) -> Result<Connection> {
    let socket = TcpStream::connect(address)
        .map_err(|e| VantisError::Network(e.to_string()))?;
    
    Ok(Connection::new(socket))
}

// Use Option<T> for optional values
pub fn get_config(key: &str) -> Option<String> {
    CONFIG.get(key).cloned()
}

// Use ? operator for error propagation
pub fn process_data(data: &[u8]) -> Result<Vec<u8>> {
    let decrypted = decrypt(data)?;
    let parsed = parse(&decrypted)?;
    Ok(parsed)
}
```

### Async/Await

```rust
// Use async/await for I/O operations
pub async fn connect_vpn(server: &str) -> Result<Connection> {
    let socket = TcpStream::connect(server).await?;
    Ok(Connection::new(socket))
}

// Use tokio::spawn for concurrent tasks
pub async fn handle_connections() {
    let listener = TcpListener::bind("0.0.0.0:51820").await?;
    
    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(socket, addr).await
        });
    }
}
```

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_computation() {
        let hash = Hash::new().unwrap();
        let digest = hash.compute(b"test").unwrap();
        assert_eq!(digest.len(), 32);
    }

    #[tokio::test]
    async fn test_async_connection() {
        let conn = connect_vpn("localhost:51820").await.unwrap();
        assert!(conn.is_connected());
    }
}
```

---

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_hash_computation

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Integration Tests

```bash
# Run integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test integration_test
```

### Benchmarking

```rust
#[cfg(test)]
mod benches {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_hash(c: &mut Criterion) {
        let hash = Hash::new().unwrap();
        c.bench_function("hash_compute", |b| {
            b.iter(|| hash.compute(black_box(b"test data")))
        });
    }

    criterion_group!(benches, bench_hash);
    criterion_main!(benches);
}
```

```bash
# Run benchmarks
cargo bench
```

### Code Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View coverage report
open html/index.html
```

### Fuzzing

```rust
// fuzz/fuzz_targets/hash.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use vantis_core::crypto::hash::Hash;

fuzz_target!(|data: &[u8]| {
    if let Ok(hash) = Hash::new() {
        let _ = hash.compute(data);
    }
});
```

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Run fuzzer
cargo +nightly fuzz run hash
```

---

## Building

### Debug Build

```bash
# Build debug version
cargo build

# Run debug version
cargo run
```

### Release Build

```bash
# Build release version
cargo build --release

# Run release version
cargo run --release
```

### Cross-Compilation

```bash
# Add target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cargo build --target x86_64-pc-windows-gnu

# Build for macOS
cargo build --target x86_64-apple-darwin

# Build for Linux ARM
cargo build --target aarch64-unknown-linux-gnu
```

### Using Make

```bash
# Build all
make build

# Build release
make release

# Run tests
make test

# Run linter
make lint

# Format code
make fmt

# Clean build artifacts
make clean
```

---

## Debugging

### Using GDB

```bash
# Build with debug symbols
cargo build

# Run with GDB
gdb target/debug/vantis-server

# GDB commands
(gdb) break main
(gdb) run
(gdb) next
(gdb) print variable
(gdb) continue
```

### Using LLDB

```bash
# Run with LLDB
lldb target/debug/vantis-server

# LLDB commands
(lldb) breakpoint set --name main
(lldb) run
(lldb) next
(lldb) frame variable
(lldb) continue
```

### Logging

```rust
use tracing::{info, warn, error};

pub fn process_request(request: Request) {
    info!("Processing request: {:?}", request);
    
    match process(&request) {
        Ok(result) => info!("Request processed successfully"),
        Err(e) => error!("Request failed: {}", e),
    }
}
```

```bash
# Set log level
export RUST_LOG=debug
cargo run

# Set log level for specific module
export RUST_LOG=vantis_core::network=trace
cargo run
```

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin vantis-server

# View flamegraph
flamegraph.svg
```

---

## Contributing

### Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write tests
5. Run tests and linter
6. Commit your changes
7. Push to your fork
8. Create a pull request

### Branch Naming

```
feature/feature-name
bugfix/bug-description
hotfix/critical-fix
refactor/refactor-description
docs/documentation-update
test/test-improvement
```

### Commit Messages

Follow conventional commits:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test changes
- `chore`: Build process or auxiliary tool changes

Examples:
```
feat(crypto): add ML-KEM key generation

Implement ML-KEM (Kyber) key generation for all
security levels (512, 768, 1024).

Closes #123
```

```
fix(network): resolve connection timeout issue

The connection timeout was not being properly handled,
causing the client to hang indefinitely.

Fixes #456
```

---

## Pull Request Process

### Creating a Pull Request

1. Update documentation
2. Add tests
3. Ensure all tests pass
4. Update CHANGELOG.md
5. Create PR with descriptive title
6. Link to related issues

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review performed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings generated
- [ ] Tests added/updated
- [ ] All tests passing
```

### Code Review

- Be constructive and respectful
- Focus on code quality and design
- Suggest improvements
- Ask questions for clarification
- Approve when satisfied

### Merging

- Ensure CI passes
- At least one approval required
- Resolve all conflicts
- Squash commits if needed
- Delete branch after merge

---

## Release Process

### Versioning

Follow semantic versioning: `MAJOR.MINOR.PATCH`

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality
- **PATCH**: Backwards-compatible bug fixes

### Pre-Release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite
4. Create release branch
5. Tag release
6. Build release artifacts
7. Test release artifacts

### Release

1. Create GitHub release
2. Upload release artifacts
7. Update documentation
8. Announce release

### Post-Release

1. Merge release branch to main
2. Create next development version
3. Update roadmap
4. Close related issues

---

## Architecture

### Design Principles

1. **Modularity**: Clear separation of concerns
2. **Testability**: Easy to test components
3. **Performance**: Zero-cost abstractions
4. **Security**: Memory safety and type safety
5. **Maintainability**: Clean, readable code

### Component Interaction

```
┌─────────────┐
│   UI Layer  │
└──────┬──────┘
       │
┌──────▼──────┐
│  Tunnel     │
│  Manager    │
└──────┬──────┘
       │
┌──────▼──────┐
│  Network    │
│  Layer      │
└──────┬──────┘
       │
┌──────▼──────┐
│  Crypto     │
│  Layer      │
└─────────────┘
```

### Data Flow

1. User initiates connection
2. UI sends request to Tunnel Manager
3. Tunnel Manager creates tunnel
4. Network Layer establishes connection
5. Crypto Layer encrypts data
6. Data sent through tunnel
7. Response decrypted and returned

### State Management

```rust
// Use state machine for complex state transitions
pub enum TunnelState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error(String),
}

impl TunnelState {
    pub fn transition(&self, event: Event) -> Result<Self> {
        match (self, event) {
            (TunnelState::Disconnected, Event::Connect) => {
                Ok(TunnelState::Connecting)
            }
            (TunnelState::Connecting, Event::Connected) => {
                Ok(TunnelState::Connected)
            }
            // ... more transitions
            _ => Err(VantisError::InvalidState),
        }
    }
}
```

---

## Performance Optimization

### Profiling

```bash
# Use cargo-flamegraph
cargo install flamegraph
cargo flamegraph --bin vantis-server

# Use perf (Linux)
perf record -g ./target/release/vantis-server
perf report

# Use Instruments (macOS)
instruments -t "Time Profiler" ./target/release/vantis-server
```

### Optimization Techniques

#### 1. Use Efficient Data Structures

```rust
// Use Vec for sequential access
let mut data = Vec::new();

// Use HashMap for O(1) lookups
let mut cache = HashMap::new();

// Use BTreeMap for ordered data
let mut sorted = BTreeMap::new();
```

#### 2. Avoid Unnecessary Allocations

```rust
// Bad: Creates new String
let result = format!("Hello, {}", name);

// Good: Uses String slice
let result = format!("Hello, {}", name);
```

#### 3. Use Iterators

```rust
// Bad: Creates intermediate Vec
let filtered: Vec<_> = items.iter()
    .filter(|x| x > 0)
    .collect();

// Good: Uses iterator chain
let filtered = items.iter()
    .filter(|x| x > 0);
```

#### 4. Use Async/Await

```rust
// Bad: Blocking I/O
let data = read_file_blocking("data.txt")?;

// Good: Async I/O
let data = read_file_async("data.txt").await?;
```

#### 5. Use Zero-Copy

```rust
// Bad: Copies data
let copy = data.clone();

// Good: Uses reference
let reference = &data;
```

### Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_encryption(c: &mut Criterion) {
    let cipher = Cipher::new(key, CipherSuite::ChaCha20Poly1305).unwrap();
    let data = vec![0u8; 1024];
    
    c.bench_function("encrypt_1kb", |b| {
        b.iter(|| cipher.encrypt(black_box(&data), nonce))
    });
}

criterion_group!(benches, bench_encryption);
criterion_main!(benches);
```

---

## Security Best Practices

### Memory Safety

```rust
// Use Rust's ownership system
let data = vec![0u8; 1024];
process(data); // data is moved, no use-after-free

// Use Arc for shared ownership
let shared = Arc::new(data);
let clone = Arc::clone(&shared);
```

### Cryptographic Security

```rust
// Use constant-time comparison
use subtle::ConstantTimeEq;

fn compare_hashes(a: &[u8], b: &[u8]) -> bool {
    a.ct_eq(b).into()
}

// Use secure random
use rand::RngCore;

let mut key = [0u8; 32];
let mut rng = rand::thread_rng();
rng.fill_bytes(&mut key);
```

### Input Validation

```rust
// Validate all inputs
pub fn process_input(input: &str) -> Result<Vec<u8>> {
    if input.len() > MAX_INPUT_SIZE {
        return Err(VantisError::InvalidInput("Input too large".to_string()));
    }
    
    if !input.is_ascii() {
        return Err(VantisError::InvalidInput("Non-ASCII input".to_string()));
    }
    
    Ok(input.as_bytes().to_vec())
}
```

### Error Handling

```rust
// Never panic in production code
pub fn safe_divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        return Err(VantisError::DivisionByZero);
    }
    Ok(a / b)
}

// Use Result instead of Option for errors
pub fn get_user(id: u32) -> Result<User> {
    USERS.get(&id)
        .cloned()
        .ok_or_else(|| VantisError::NotFound("User not found".to_string()))
}
```

### Secrets Management

```rust
// Never log secrets
pub fn process_password(password: &str) {
    // Bad: logs password
    // println!("Processing password: {}", password);
    
    // Good: doesn't log password
    println!("Processing password");
    
    // Zeroize after use
    use zeroize::Zeroize;
    let mut secret = password.as_bytes().to_vec();
    secret.zeroize();
}
```

---

## Resources

### Documentation

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Tools

- [cargo-watch](https://github.com/passcod/cargo-watch)
- [cargo-edit](https://github.com/killercup/cargo-edit)
- [cargo-audit](https://github.com/RustSec/cargo-audit)
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated)

### Community

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/rust)

### VANTISVPN Specific

- [GitHub Issues](https://github.com/vantisCorp/VantisVPN/issues)
- [GitHub Discussions](https://github.com/vantisCorp/VantisVPN/discussions)
- [Email: dev@vantisvpn.com](mailto:dev@vantisvpn.com)

---

## Appendix

### Useful Commands

```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Audit dependencies for vulnerabilities
cargo audit

# Generate documentation
cargo doc --open

# Check for unused dependencies
cargo +nightly udeps

# Clean build artifacts
cargo clean

# Check what would be built
cargo check
```

### Environment Variables

```bash
# Set log level
export RUST_LOG=debug

# Set test threads
export RUST_TEST_THREADS=4

# Set backtrace
export RUST_BACKTRACE=1
```

### Git Aliases

```bash
# Add useful aliases
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.visual '!gitk'
```

---

*Last Updated: 2024*
*Version: 1.0.0*