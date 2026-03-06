# Contributing to VantisVPN

First off, thanks for taking the time to contribute! 🎉

The following is a set of guidelines for contributing to VantisVPN. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Security](#security)

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to conduct@vantisvpn.com.

### Our Standards

- Be respectful and inclusive
- Welcome newcomers warmly
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- **Rust** 1.82 or later
- **Node.js** 20.x (for frontend development)
- **Git** 2.30 or later
- **Docker** (optional, for containerized development)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   gh repo clone your-username/VantisVPN
   cd VantisVPN
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/vantisCorp/VantisVPN.git
   ```

## Development Setup

### Using DevContainer (Recommended)

1. Open the repository in VS Code
2. When prompted, click "Reopen in Container"
3. Wait for the container to build and dependencies to install

### Manual Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Build the project
cargo build

# Run tests
cargo test

# Run linting
cargo clippy --all-targets --all-features
cargo fmt --all -- --check
```

### Useful Commands

| Command | Description |
|---------|-------------|
| `make build` | Build the project |
| `make test` | Run all tests |
| `make lint` | Run linting |
| `make fmt` | Format code |
| `make security-scan` | Run security scans |
| `make docs` | Generate documentation |
| `make clean` | Clean build artifacts |

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/vantisCorp/VantisVPN/issues)
2. If not, create a new issue using the [Bug Report template](https://github.com/vantisCorp/VantisVPN/issues/new?template=bug_report.yml)
3. Include as much detail as possible

### Suggesting Enhancements

1. Check existing [Feature Requests](https://github.com/vantisCorp/VantisVPN/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement)
2. Create a new feature request using the [Feature Request template](https://github.com/vantisCorp/VantisVPN/issues/new?template=feature_request.yml)

### Working on Issues

1. Look for issues labeled `good first issue` or `help wanted`
2. Comment on the issue to let others know you're working on it
3. Create a branch for your work:
   ```bash
   git checkout -b feature/issue-number-description
   ```

## Pull Request Process

### Before Submitting

1. **Update your branch** with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run all tests**:
   ```bash
   cargo test --all-features
   ```

3. **Run linting**:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   ```

4. **Run security scan**:
   ```bash
   make security-scan
   ```

### Submitting the PR

1. Push your branch:
   ```bash
   git push https://x-access-token:$GITHUB_TOKEN@github.com/your-username/VantisVPN.git feature/issue-number-description
   ```

2. Create a Pull Request using `gh pr create`

3. Fill out the PR template completely

4. Link any related issues

### PR Requirements

- All tests must pass
- Code must be formatted correctly
- No clippy warnings
- Security scan must pass
- At least one approval from a maintainer
- All conversations must be resolved

### After Approval

1. Squash your commits if requested
2. A maintainer will merge your PR

## Coding Standards

### Rust Style Guide

We follow the official [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md).

### Key Points

- Use `clippy` for linting
- Format code with `rustfmt`
- Document all public items
- Write tests for new functionality
- Keep functions small and focused

### Documentation

```rust
/// Short description.
///
/// Longer description if needed.
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// use vantis_core::function_name;
/// let result = function_name(param);
/// ```
pub fn function_name(param: Type) -> ReturnType {
    // ...
}
```

### Error Handling

- Use `thiserror` for library errors
- Use `anyhow` for application errors
- Provide meaningful error messages
- Include context in errors

## Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/).

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Code style (formatting, etc.) |
| `refactor` | Code refactoring |
| `perf` | Performance improvement |
| `test` | Adding or modifying tests |
| `chore` | Maintenance tasks |
| `ci` | CI/CD changes |
| `security` | Security improvements |

### Examples

```
feat(crypto): add post-quantum key exchange support

fix(network): resolve QUIC connection timeout issue

docs(readme): update installation instructions

security(crypto): update to ML-KEM-768
```

### Commit Message Rules

1. Use the imperative mood ("add feature" not "added feature")
2. First line should be 50 characters or less
3. Body should explain what and why, not how
4. Reference issues and PRs in the footer

## Security

### Reporting Security Issues

**Do not report security vulnerabilities through public GitHub issues.**

Instead, email security@vantisvpn.com with:

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact

See our [Security Policy](SECURITY.md) for more details.

### Security Best Practices

- Never commit secrets or credentials
- Use environment variables for sensitive data
- Run `make security-scan` before committing
- Keep dependencies updated
- Follow the principle of least privilege

## Getting Help

- **Discord**: https://discord.gg/A5MzwsRj7D
- **GitHub Discussions**: https://github.com/vantisCorp/VantisVPN/discussions
- **Email**: dev@vantisvpn.com

## Recognition

Contributors are recognized in our:

- [Contributors page](https://github.com/vantisCorp/VantisVPN/graphs/contributors)
- Release notes
- README.md contributors section

Thank you for contributing to VantisVPN! 🙏