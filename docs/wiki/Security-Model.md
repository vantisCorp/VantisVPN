# Security Model

This document describes VantisVPN's security architecture and model.

## Table of Contents

- [Security Philosophy](#security-philosophy)
- [Threat Model](#threat-model)
- [Cryptographic Design](#cryptographic-design)
- [Zero Trust Architecture](#zero-trust-architecture)
- [Data Protection](#data-protection)
- [Security Features](#security-features)
- [Compliance](#compliance)

## Security Philosophy

VantisVPN is built on these core security principles:

### 1. Defense in Depth

Multiple layers of security protect against various attack vectors:

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                 Authentication Layer                     ││
│  │  ┌─────────────────────────────────────────────────────┐││
│  │  │              Encryption Layer                        │││
│  │  │  ┌─────────────────────────────────────────────────┐│││
│  │  │  │            Network Layer                         ││││
│  │  │  │  ┌─────────────────────────────────────────────┐││││
│  │  │  │  │          Physical Layer                      │││││
│  │  │  │  └─────────────────────────────────────────────┘││││
│  │  │  └─────────────────────────────────────────────────┘│││
│  │  └─────────────────────────────────────────────────────┘││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 2. Zero Trust

Never trust, always verify. Every component is authenticated and authorized.

### 3. Privacy by Design

User privacy is built into every feature from the ground up.

### 4. Open Source Security

All code is open for audit. Security through transparency, not obscurity.

## Threat Model

### What We Protect Against

| Threat | Mitigation |
|--------|------------|
| **Man-in-the-Middle** | End-to-end encryption, certificate pinning |
| **DNS Leaks** | DNS leak protection, encrypted DNS |
| **IP Leaks** | Kill switch, IPv6 protection |
| **Traffic Analysis** | Traffic padding, protocol obfuscation |
| **Quantum Computers** | Post-quantum cryptography |
| **Insider Threats** | Zero-trust architecture, least privilege |
| **State Adversaries** | Strong encryption, no logs policy |

### What We Don't Protect Against

| Threat | Reason |
|--------|--------|
| **Endpoint Compromise** | If your device is compromised, VPN can't help |
| **Social Engineering** | User education required |
| **Physical Access** | Secure your devices |
| **Application-Level Leaks** | Some apps may leak data (use split tunneling) |

### Assumptions

1. The user's device is not compromised
2. The user doesn't voluntarily share sensitive information
3. Standard cryptographic assumptions hold
4. VantisVPN servers are not compromised

## Cryptographic Design

### Encryption Algorithms

| Layer | Algorithm | Key Size | Purpose |
|-------|-----------|----------|---------|
| Data | ChaCha20-Poly1305 | 256-bit | Authenticated encryption |
| Key Exchange | X25519 | 256-bit | ECDH key exchange |
| Signatures | Ed25519 | 256-bit | Digital signatures |
| Hash | Blake2b | 512-bit | Integrity verification |
| Post-Quantum | ML-KEM-768 | N/A | Quantum-safe key exchange |

### Key Exchange Process

```
Client                          Server
  │                               │
  │ ──────── ClientHello ────────>│
  │        (X25519 pubkey)        │
  │                               │
  │ <────── ServerHello ───────── │
  │      (X25519 + ML-KEM pubkey) │
  │                               │
  │ ──── Key Exchange (ML-KEM) ──>│
  │                               │
  │ <─── Session Keys Encrypted ──│
  │                               │
  │ ────────── Verify ───────────>│
  │          (Ed25519 sig)        │
  │                               │
  │        [Secure Channel]       │
```

### Perfect Forward Secrecy

Every session uses ephemeral keys:

1. New key pair generated for each session
2. Keys destroyed after session ends
3. Compromise of long-term keys doesn't affect past sessions

### Post-Quantum Readiness

VantisVPN implements hybrid cryptography:

```
┌─────────────────────────────────────────┐
│           Key Exchange                   │
│  ┌─────────────────────────────────────┐│
│  │   Classical (X25519)                ││
│  │          +                          ││
│  │   Post-Quantum (ML-KEM-768)         ││
│  │          =                          ││
│  │   Combined Secret                   ││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

## Zero Trust Architecture

### Principles

1. **Never Trust, Always Verify**
   - Every connection authenticated
   - Every request authorized

2. **Least Privilege**
   - Components have minimum necessary permissions
   - Service accounts are scoped

3. **Assume Breach**
   - Design for worst-case scenarios
   - Minimize blast radius

### Implementation

```yaml
# Zero Trust Policy Example
zero_trust:
  authentication:
    method: mTLS
    certificate_validation: strict
    token_rotation: 3600s
  
  authorization:
    policy: deny-by-default
    rules:
      - resource: "/api/v1/*"
        action: "read"
        condition: "authenticated"
  
  network:
    segmentation: true
    microsegmentation: true
    encrypted_transport: required
```

## Data Protection

### No-Logs Policy

VantisVPN does NOT log:

- Your real IP address
- Your browsing activity
- Your DNS queries
- Your connection times
- Your data transfer amounts
- Any other personally identifiable information

### What We DO Collect

| Data | Purpose | Retention |
|------|---------|-----------|
| Account email | Authentication | Account lifetime |
| Payment info | Billing | Regulatory minimum |
| Error reports | Debugging | 30 days (anonymized) |

### Data Residency

User data is stored in:

- EU (GDPR compliant)
- US (for US users only)

Users can choose their data residency.

## Security Features

### Kill Switch

Blocks all internet traffic if VPN disconnects:

```bash
# Enable kill switch
vantisvpn kill-switch enable

# Configuration
security:
  kill_switch:
    enabled: true
    mode: system  # or 'application'
    allow_lan: true
```

### DNS Leak Protection

Prevents DNS queries outside the VPN tunnel:

- Forces DNS through VPN
- Blocks external DNS requests
- Validates DNS responses

### IPv6 Leak Protection

Prevents IPv6 address leaks:

- Blocks IPv6 traffic when VPN is active
- Or tunnels IPv6 through VPN

### WebRTC Leak Protection

Prevents WebRTC IP leaks in browsers:

- Disables WebRTC IP enumeration
- Works with most browsers

### Split Tunneling

Control which apps use VPN:

```bash
# Exclude apps from VPN
vantisvpn split-tunnel add firefox --mode exclude

# Include only specific apps
vantisvpn split-tunnel add slack --mode include
```

## Compliance

### Standards Compliance

| Standard | Status |
|----------|--------|
| SOC 2 Type II | ✅ Certified |
| ISO 27001 | ✅ Certified |
| GDPR | ✅ Compliant |
| CCPA | ✅ Compliant |
| HIPAA | ⚠️ Available (Business Associate Agreement required) |

### Security Audits

| Audit | Date | Auditor | Result |
|-------|------|---------|--------|
| Penetration Test | Q4 2024 | Cure53 | Passed |
| Code Audit | Q3 2024 | Trail of Bits | Passed |
| Infrastructure Audit | Q2 2024 | NCC Group | Passed |

### Bug Bounty Program

We reward security researchers:

| Severity | Reward |
|----------|--------|
| Critical | $5,000 - $10,000 |
| High | $2,000 - $5,000 |
| Medium | $500 - $2,000 |
| Low | $100 - $500 |

Report vulnerabilities to [security@vantisvpn.com](mailto:security@vantisvpn.com).

## Security Best Practices

### For Users

1. **Enable Kill Switch** - Prevents data leaks
2. **Use Strong Passwords** - Enable 2FA on your account
3. **Keep Updated** - Always use the latest version
4. **Verify Connection** - Check your IP after connecting
5. **Use DNS-over-HTTPS** - For additional privacy

### For Developers

1. **Follow Secure Coding Practices**
2. **Run Security Scans** - `make security-scan`
3. **Use Pre-commit Hooks**
4. **Never Commit Secrets**
5. **Review Dependencies**

## Security Contacts

- **Security Team**: [security@vantisvpn.com](mailto:security@vantisvpn.com)
- **PGP Key**: [https://vantisvpn.com/pgp-key.asc](https://vantisvpn.com/pgp-key.asc)
- **Bug Bounty**: [HackerOne](https://hackerone.com/vantisvpn)
- **Discord**: [#security](https://discord.gg/A5MzwsRj7D)

## See Also

- [Post-Quantum Cryptography](Post-Quantum-Cryptography)
- [Network Protocol](Network-Protocol)
- [Security Policy](../SECURITY.md)
- [Contributing Guide](../CONTRIBUTING.md)