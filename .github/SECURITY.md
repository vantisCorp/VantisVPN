# Security Policy for VANTISVPN

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 1.0.x   | ✅ Supported        |
| < 1.0.0 | ❌ Unsupported      |

## Reporting a Vulnerability

If you discover a security vulnerability in VANTISVPN, please report it responsibly.

### How to Report

1. **Email**: security@vantisvpn.com
2. **PGP Key**: Available at https://vantisvpn.com/pgp-key

### What to Include

Please include as much information as possible:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Proof of concept (if applicable)
- Suggested fix (if known)

### Response Timeline

We aim to respond to security reports within:
- **Initial Response**: Within 48 hours
- **Investigation**: Within 7 days
- **Fix Development**: Within 14 days
- **Public Disclosure**: After fix is released

### Security Research

We welcome responsible security research on VANTISVPN. Please:
- Test only systems you own or have explicit permission to test
- Don't disrupt service or access user data
- Report vulnerabilities through our secure channels
- Allow us reasonable time to fix issues before public disclosure

### Responsible Disclosure

We follow responsible disclosure practices:
- We will acknowledge receipt of your report
- We will keep you informed of our progress
- We will credit you for the discovery (if desired)
- We will work with you on a coordinated disclosure timeline

### Bug Bounty Program

We offer a bug bounty program for qualifying vulnerabilities:
- **Critical**: $5,000 - $10,000
- **High**: $2,000 - $5,000
- **Medium**: $500 - $2,000
- **Low**: $100 - $500

See https://vantisvpn.com/bug-bounty for details.

## Security Best Practices

### For Users
- Always use the latest version
- Enable automatic updates
- Use strong authentication (2FA recommended)
- Report suspicious activity

### For Developers
- Follow secure coding practices
- Never commit secrets or credentials
- Use dependency scanning
- Perform security reviews before merging

## Security Features

VANTISVPN includes these security features:
- Post-quantum cryptography (ML-KEM, ML-DSA)
- Zero-knowledge authentication
- RAM-only server architecture
- Zero-logs policy
- Secure boot
- Memory-safe Rust implementation
- Regular security audits
- Compliance certifications (SOC 2, HITRUST, PCI DSS)

## Security Audits

| Date | Auditor | Scope | Report |
|------|---------|-------|--------|
| 2026-02-15 | Independent Security Labs | Full system | Private |
| 2026-01-20 | Internal Team | Crypto module | Private |

## Incident Response

### Incident Response Team
- **Lead**: security@vantisvpn.com
- **Team**: security-team@vantisvpn.com

### Severity Levels
- **Critical**: Immediate threat, service-wide impact
- **High**: Major impact, requires urgent attention
- **Medium**: Limited impact, scheduled fix
- **Low**: Minor issue, backlogged

### Response Process
1. Detection and triage
2. Investigation and analysis
3. Containment and mitigation
4. Eradication and recovery
5. Post-incident review

## Security Resources

- **Documentation**: https://docs.vantisvpn.com/security
- **Whitepaper**: docs/SECURITY_WHITEPAPER.md
- **Compliance**: docs/compliance/
- **Contact**: security@vantisvpn.com

## Legal

By reporting a security vulnerability, you agree to:
- Allow us reasonable time to investigate and fix
- Not disclose the vulnerability publicly before we do
- Follow responsible disclosure practices
- Not use the vulnerability for malicious purposes

We reserve the right to:
- Credit you for the discovery
- Publicly disclose after fixing
- Decline bug bounty if violated

## Contact

- **Security Email**: security@vantisvpn.com
- **PGP Key**: https://vantisvpn.com/pgp-key
- **Security Website**: https://vantisvpn.com/security
- **Bug Bounty**: https://vantisvpn.com/bug-bounty