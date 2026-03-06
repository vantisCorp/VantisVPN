# VantisVPN Wiki

## Welcome to VantisVPN

VantisVPN is a next-generation secure VPN system featuring post-quantum cryptography, Zero Trust architecture, and QUIC protocol support.

## Quick Links

### Getting Started
- [Installation Guide](Installation-Guide)
- [Quick Start](Quick-Start)
- [Configuration](Configuration)

### Architecture
- [System Architecture](System-Architecture)
- [Cryptographic Design](Cryptographic-Design)
- [Network Protocol](Network-Protocol)

### Development
- [Contributing Guide](../CONTRIBUTING.md)
- [Development Setup](Development-Setup)
- [API Documentation](API-Documentation)

### Security
- [Security Model](Security-Model)
- [Post-Quantum Cryptography](Post-Quantum-Cryptography)
- [Vulnerability Reporting](../SECURITY.md)

## Features Overview

| Feature | Description |
|---------|-------------|
| 🔐 **Post-Quantum Crypto** | ML-KEM & ML-DSA ready (NIST FIPS 203/204) |
| 🛡️ **Zero Trust** | Never trust, always verify |
| 🚀 **QUIC Protocol** | Modern transport with built-in encryption |
| 🌐 **Cross-Platform** | Linux, Windows, macOS |
| 📱 **Multi-Platform** | Desktop, Mobile, CLI, Web |
| 🔧 **Open Source** | AGPL-3.0 licensed |

## System Requirements

### Minimum Requirements
- **OS**: Linux kernel 4.18+, Windows 10+, macOS 11+
- **RAM**: 256 MB minimum, 512 MB recommended
- **CPU**: x86_64 or ARM64 processor
- **Storage**: 50 MB disk space

### Recommended
- **RAM**: 1 GB or more
- **CPU**: Multi-core processor
- **Network**: UDP traffic allowed

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        VantisVPN                             │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Desktop   │  │   Mobile    │  │     CLI     │         │
│  │   Client    │  │   Client    │  │   Client    │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │                 │
│         └────────────────┼────────────────┘                 │
│                          │                                  │
│  ┌───────────────────────┴───────────────────────┐         │
│  │              Core VPN Engine                   │         │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐         │         │
│  │  │ Crypto  │ │ Network │ │ Tunnel  │         │         │
│  │  │ Layer   │ │ Layer   │ │ Manager │         │         │
│  │  └─────────┘ └─────────┘ └─────────┘         │         │
│  └───────────────────────────────────────────────┘         │
│                          │                                  │
│  ┌───────────────────────┴───────────────────────┐         │
│  │              Protocol Stack                    │         │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐         │         │
│  │  │  QUIC   │ │WireGuard│ │  TCP/   │         │         │
│  │  │ (Main)  │ │ (Alt)   │ │  UDP    │         │         │
│  │  └─────────┘ └─────────┘ └─────────┘         │         │
│  └───────────────────────────────────────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

## Getting Help

- 💬 **Discord**: [https://discord.gg/A5MzwsRj7D](https://discord.gg/A5MzwsRj7D)
- 📧 **Email**: support@vantisvpn.com
- 📖 **Docs**: [https://docs.vantisvpn.com](https://docs.vantisvpn.com)
- 🐛 **Issues**: [GitHub Issues](https://github.com/vantisCorp/VantisVPN/issues)

## Contributing

We welcome contributions! See our [Contributing Guide](../CONTRIBUTING.md) for details.

## License

VantisVPN is licensed under AGPL-3.0-or-later. See [LICENSE](../LICENSE) for details.