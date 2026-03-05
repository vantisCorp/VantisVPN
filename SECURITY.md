# Security Policy

## Supported Versions

| Version | Supported          |
|---------|-------------------|
| 1.1.x   | :white_check_mark: Yes |
| 1.0.x   | :x: No              |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please send an email to our security team:

- **Email**: security@vantisvpn.com
- **PGP Key**: Available at [https://vantisvpn.com/pgp-key.asc](https://vantisvpn.com/pgp-key.asc)

### What to Include

When reporting a vulnerability, please include:

1. A description of the vulnerability
2. Steps to reproduce the issue
3. Affected versions
4. Potential impact
5. Suggested fix (if known)

### Response Timeline

We aim to respond to security reports within **48 hours** and provide a fix within **7 days** for critical vulnerabilities.

## Security Features

VantisVPN implements multiple layers of security:

### Cryptography

- **Post-Quantum Ready**: Prepared for ML-KEM and ML-DSA (NIST FIPS 203/204)
- **ChaCha20-Poly1305**: Authenticated encryption
- **X25519**: Elliptic Curve Diffie-Hellman key exchange
- **Ed25519**: Digital signatures
- **Blake2**: Cryptographic hash functions
- **Zeroization**: Secure memory clearing with zeroize

### Network Security

- **QUIC Protocol**: Modern transport with built-in encryption
- **WireGuard**: Secure VPN protocol
- **Perfect Forward Secrecy**: Ephemeral key exchange
- **No Logs**: Zero-logging policy

### Development Security

- **Gitleaks**: Automated secrets scanning
- **Pre-commit Hooks**: Code quality and security checks
- **GPG Signing**: Commit and tag verification
- **SBOM Generation**: Software Bill of Materials
- **Dependabot**: Automated dependency updates

### Zero Trust Architecture

- **Least Privilege**: Minimal access permissions
- **Defense in Depth**: Multiple security layers
- **Continuous Verification**: Ongoing security validation
- **Assume Breach**: Security designed for worst-case scenarios

## Dependency Management

### Current Security Status

All dependencies are regularly updated to their latest stable versions:

| Dependency | Version | Last Updated | Security Status |
|------------|---------|--------------|-----------------|
| Tokio | 1.50.0 | 2025-01 | ✅ Secure |
| Rand | 0.10.0 | 2025-01 | ✅ Secure |
| Quinn | 0.11.9 | 2025-01 | ✅ Secure |
| Serde | 1.0.228 | 2025-01 | ✅ Secure |
| Thiserror | 2.0.18 | 2025-01 | ✅ Secure |

### Unmaintained Dependencies

**bincode** was replaced with **postcard** due to maintenance concerns.

## Security Best Practices

### For Users

1. Always use the latest version of VantisVPN
2. Verify GPG signatures on releases
3. Use strong, unique passwords
4. Enable two-factor authentication where available
5. Keep your operating system updated
6. Use antivirus software

### For Developers

1. Follow secure coding practices
2. Run `make security-scan` before committing
3. Use pre-commit hooks
4. Never commit secrets or credentials
5. Review dependencies regularly
6. Follow the principle of least privilege

## Bug Bounty Program

We offer a bug bounty program for security vulnerabilities:

| Severity | Reward |
|----------|--------|
| Critical | $5,000 - $10,000 |
| High | $2,000 - $5,000 |
| Medium | $500 - $2,000 |
| Low | $100 - $500 |

### Bounty Rules

1. No exploitation of systems
2. Responsible disclosure
3. Give us time to fix before public disclosure
4. Original vulnerabilities only
5. Follow our Responsible Disclosure Policy

## Security Audits

### Completed Audits

- **Date**: TBD
- **Auditor**: TBD
- **Report**: [Link](#)

### Planned Audits

- **Q2 2025**: Independent security audit planned

## Responsible Disclosure Policy

We follow a responsible disclosure policy:

1. **Discovery**: Security researcher discovers vulnerability
2. **Reporting**: Researcher reports to security@vantisvpn.com
3. **Acknowledgment**: We acknowledge within 48 hours
4. **Investigation**: We investigate and validate the issue
5. **Fix**: We develop and test a fix
6. **Deployment**: We deploy the fix
7. **Disclosure**: We coordinate public disclosure with the researcher

## Contact

- **Security Team**: security@vantisvpn.com
- **PGP Key**: https://vantisvpn.com/pgp-key.asc
- **Security Channel**: https://discord.gg/A5MzwsRj7D (tag @security)

## References

- [NIST Post-Quantum Cryptography Standardization](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CVE Database](https://cve.mitre.org/)
- [Common Vulnerability Scoring System (CVSS)](https://www.first.org/cvss/)