# Contributing to VANTISVPN

Thank you for your interest in contributing to VANTISVPN! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Security](#security)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors. We expect all contributors to:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

### Our Standards

Examples of behavior that contributes to a positive environment:

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Git
- Docker (optional, for containerized development)
- Make (optional, for using Makefile)

### Setting Up Development Environment

1. **Clone the repository:**
   ```bash
   gh repo clone vantisCorp/VantisVPN
   cd VantisVPN
   ```

2. **Install Rust toolchain:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Install development dependencies:**
   ```bash
   make install-deps
   ```

4. **Build the project:**
   ```bash
   make build
   ```

5. **Run tests:**
   ```bash
   make test
   ```

## Development Workflow

### Branching Strategy

We use a simplified Git flow:

- `main` - Production-ready code
- `develop` - Integration branch for features
- `feature/*` - Feature branches
- `bugfix/*` - Bug fix branches
- `hotfix/*` - Urgent production fixes

### Creating a Feature Branch

1. Ensure your `main` branch is up to date:
   ```bash
   git checkout main
   git pull origin main
   ```

2. Create a new feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. Make your changes and commit:
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

### Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

**Examples:**
```
feat(crypto): add post-quantum key exchange

Implement Kyber (ML-KEM) for post-quantum secure key exchange.
This provides protection against quantum computer attacks.

Closes #123
```

```
fix(network): resolve IPv6 connection issue

Fixed a bug where IPv6 connections would fail when
the MTU was set above 1500 bytes.

Fixes #456
```

## Coding Standards

### Rust Guidelines

1. **Follow Rust API Guidelines:**
   - Use `rustfmt` for formatting
   - Run `clippy` for linting
   - Document all public APIs

2. **Error Handling:**
   - Use `Result<T, E>` for fallible operations
   - Use `thiserror` for custom error types
   - Provide helpful error messages

3. **Memory Safety:**
   - Avoid `unsafe` code unless absolutely necessary
   - Use `zeroize` for sensitive data
   - Implement `Drop` for cleanup

4. **Concurrency:**
   - Prefer async/await over threads
   - Use `tokio` for async runtime
   - Be careful with shared state

### Security Guidelines

1. **Cryptographic Operations:**
   - Never implement your own crypto
   - Use vetted libraries only
   - Follow NIST/FIPS guidelines

2. **Input Validation:**
   - Validate all external input
   - Use type-safe parsing
   - Sanitize user data

3. **Secret Management:**
   - Never hardcode secrets
   - Use environment variables
   - Implement zeroization

### Code Style

```rust
// Good: Clear, documented, type-safe
/// Encrypts data using ChaCha20-Poly1305
///
/// # Arguments
/// * `plaintext` - Data to encrypt
/// * `key` - 32-byte encryption key
///
/// # Returns
/// Encrypted ciphertext with authentication tag
///
/// # Errors
/// Returns an error if the key is invalid
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // Implementation
}

// Bad: Unclear, undocumented, unsafe
pub fn enc(d: &[u8], k: &[u8]) -> Vec<u8> {
    // Unsafe implementation
}
```

## Testing

### Unit Tests

Write unit tests for all functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let plaintext = b"test data";
        let key = [0u8; 32];
        let ciphertext = encrypt(plaintext, &key).unwrap();
        assert_ne!(plaintext, ciphertext.as_slice());
    }
}
```

### Integration Tests

Add integration tests in the `tests/` directory:

```rust
// tests/integration_test.rs
use vantis_core::*;

#[test]
fn test_full_workflow() {
    // Test complete workflow
}
```

### Test Coverage

Aim for >80% code coverage. Run coverage with:

```bash
make test-coverage
```

### Running Tests

```bash
# Run all tests
make test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

## Documentation

### Inline Documentation

Document all public APIs:

```rust
/// Represents a VPN tunnel connection
///
/// # Example
///
/// ```rust
/// use vantis_core::Tunnel;
///
/// let tunnel = Tunnel::new("test".to_string());
/// ```
pub struct Tunnel {
    // ...
}
```

### README Updates

Update the README when:
- Adding new features
- Changing build instructions
- Updating dependencies
- Changing project structure

### Architecture Docs

Update architecture documentation when:
- Changing system design
- Adding new components
- Modifying data flow
- Updating protocols

## Submitting Changes

### Pull Request Process

1. **Update your branch:**
   ```bash
   git checkout main
   git pull origin main
   git checkout feature/your-feature
   git rebase main
   ```

2. **Run all checks:**
   ```bash
   make ci
   ```

3. **Push your changes:**
   ```bash
   git push origin feature/your-feature
   ```

4. **Create a Pull Request:**
   ```bash
   gh pr create --title "feat: your feature" --body "Description of changes"
   ```

### Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Commit messages follow conventions
- [ ] No merge conflicts
- [ ] Security review completed (if applicable)

### Review Process

1. Automated checks must pass
2. At least one maintainer approval required
3. Address all review comments
4. Squash commits if requested
5. Update PR description if needed

## Security

### Reporting Vulnerabilities

If you discover a security vulnerability, please:

1. **Do NOT** create a public issue
2. Email us at: security@vantisvpn.com
3. Include details about the vulnerability
4. Allow us time to fix before disclosure

### Security Best Practices

- Never commit secrets or keys
- Use environment variables for configuration
- Implement proper authentication
- Validate all inputs
- Use secure cryptographic libraries
- Follow OWASP guidelines
- Regular security audits

### Security Review Process

All changes go through security review:

1. Automated security scanning
2. Manual code review
3. Dependency vulnerability check
4. Penetration testing (for major releases)

## Getting Help

### Resources

- **Documentation**: Check the `docs/` directory
- **Issues**: Search existing GitHub issues
- **Discussions**: Use GitHub Discussions for questions
- **Email**: dev@vantisvpn.com

### Community

- Join our Discord server
- Follow us on Twitter
- Subscribe to our newsletter

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project website
- Annual contributor report

Thank you for contributing to VANTISVPN! 🚀