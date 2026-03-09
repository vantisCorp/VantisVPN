🇩🇪 [Zurück zur Haupt-README](README.md)

<!-- VANTISVPN BANNER -->
<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/banners/vantisvpn-banner-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="assets/banners/vantisvpn-banner-light.svg">
    <img alt="VantisVPN Banner" src="assets/banners/vantisvpn-banner-light.svg" width="100%">
  </picture>
</div>

<div align="center">

# 🔴⬛ VANTISVPN ⬛🔴
## VPN-System der nächsten Generation mit quantenresistenter Kryptographie und Zero-Trust-Architektur

[![Version](https://img.shields.io/badge/version-1.1.0-red?style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/releases)
[![License](https://img.shields.io/badge/license-AGPL_v3_Commercial-black?style=for-the-badge)](LICENSE)
[![CI/CD](https://img.shields.io/github/actions/workflow/status/vantisCorp/VantisVPN/ci.yml?branch=main&style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/actions)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=red)](https://www.rust-lang.org/)

</div>

---

## 📚 Inhaltsverzeichnis

- [⚡ Schnellstart](#-schnellstart)
- [✨ Hauptfunktionen](#-hauptfunktionen)
- [🏗️ Architektur](#️-architektur)
- [🔒 Zero-Trust-Sicherheit](#-zero-trust-sicherheit)
- [📊 Benchmarks](#-benchmarks)
- [🚀 Installation](#-installation)
- [⚙️ Konfiguration](#️-konfiguration)
- [🧪 Tests](#-tests)
- [🗺️ Fahrplan](#️-fahrplan)
- [🤝 Mitwirken](#-mitwirken)
- [📜 Doppellizenzierung](#-doppellizenzierung)
- [📞 Kontakt & Support](#-kontakt--support)

---

# ⚡ Schnellstart

## 🚀 In 3 Minuten einsatzbereit!

### Option 1: Ein-Zeilen-Installation

```bash
curl -sSf https://install.vantisvpn.com | sh
```

### Option 2: Manuelle Installation

```bash
# 1. Repository klonen
git clone https://github.com/vantisCorp/VantisVPN.git
cd VantisVPN

# 2. Abhängigkeiten installieren
make install

# 3. Projekt erstellen
make build

# 4. VantisVPN starten
make dev
```

### Option 3: Docker-Installation

```bash
docker pull vantisvpn/core:latest
docker run -d --name vantisvpn \
  --cap-add=NET_ADMIN \
  --device /dev/net/tun \
  -p 51820:51820/udp \
  vantisvpn/core:latest
```

### 🎯 Installation überprüfen

```bash
# Version überprüfen
vantisvpn --version

# Diagnose ausführen
vantisvpn diagnostics

# Verbindung testen
vantisvpn test
```

---

# ✨ Hauptfunktionen

## 🎯 Was macht VantisVPN anders?

| | VantisVPN | WireGuard | OpenVPN | NordVPN |
|---|:---:|:---:|:---:|:---:|
| **Post-Quanten-Kryptographie** | ✅ ML-KEM + ML-DSA | ❌ | ❌ | ❌ |
| **Zero-Trust-Architektur** | ✅ | ❌ | ❌ | ⚠️ |
| **QUIC/HTTP3** | ✅ BBRv3 | ❌ | ❌ | ❌ |
| **Multi-Hop** | ✅ 5+ hops | ❌ | ❌ | ✅ 2 hops |
| **RAM-Only** | ✅ | ❌ | ❌ | ✅ |
| **Kill Switch** | ✅ OS-level | ❌ | ⚠️ | ✅ |
| **Open Source** | ✅ AGPL v3 | ✅ GPL v2 | ✅ GPL v2 | ❌ |

## 🌟 Hervorgehobene Funktionen

### 🔐 Post-Quanten-Kryptographie
Erste Implementierung von ML-KEM (FIPS 203) und ML-DSA (FIPS 204) in einem VPN. Ihre Daten sind selbst vor Quantencomputern sicher.

- **ML-KEM** (FIPS 203) — Kyber-1024
- **ML-DSA** (FIPS 204) — Dilithium-87
- **Hybrid** — Classical + PQC

### ⚡ Blitzschnell
QUIC/HTTP3-Protokoll mit BBRv3-Überlastungskontrolle. 0-RTT-Verbindungen und Verbindungsmigration für unterbrechungsfreie Konnektivität.

- **QUIC/HTTP3** — RFC 9000/9114
- **BBRv3** — Congestion Control
- **0-RTT** — Zero Round-Trip Time

### 🛡️ Zero-Trust-Architektur
Jede Verbindung wird überprüft. Kontinuierliche Autorisierung, Mikrosegmentierung und Prinzip der geringsten Privilegien.

- **mTLS** — Mutual TLS
- **RBAC** — Role-Based Access Control
- **MFA** — Multi-Factor Authentication

### 🌍 Globale Infrastruktur
RAM-only-Server an 100+ Standorten. Intelligentes Routing, Multi-Hop und Starlink-Unterstützung.

- **100+** locations
- **RAM-only** servers
- **Smart Routing** — AI-powered
- **Multi-Hop** — 5+ hops

---

# 🏗️ Architektur

## 📐 Systemarchitektur

```
┌─────────────────────────────────────────────────────┐
│                    VantisVPN                          │
├─────────────────────────────────────────────────────┤
│  UI Layer        │  CLI / Desktop / Mobile / Web     │
├──────────────────┼──────────────────────────────────┤
│  Security Layer  │  Zero Trust / Kill Switch / DAITA │
├──────────────────┼──────────────────────────────────┤
│  Network Layer   │  QUIC / WireGuard / Stealth       │
├──────────────────┼──────────────────────────────────┤
│  Crypto Layer    │  ML-KEM / ML-DSA / ChaCha20       │
├──────────────────┼──────────────────────────────────┤
│  Tunnel Layer    │  TUN/TAP / State Machine           │
└─────────────────────────────────────────────────────┘
```

## 🔧 Technologie-Stack

| Layer | Technology |
|-------|-----------|
| **Core** | Rust 1.94+, Tokio, async/await |
| **Crypto** | ML-KEM, ML-DSA, ChaCha20-Poly1305, BLAKE2s |
| **Network** | QUIC (RFC 9000), WireGuard, HTTP/3 |
| **Frontend** | Tauri, React, TypeScript |
| **Infra** | Docker, Terraform, Prometheus |
| **CI/CD** | GitHub Actions, CodeQL, Dependabot |

---

# 🔒 Zero-Trust-Sicherheit

## 🛡️ Sicherheitsschichten

```
Layer 7: Application    → Zero Trust Policy Engine
Layer 6: Presentation   → ML-KEM/ML-DSA Encryption
Layer 5: Session        → mTLS + Certificate Pinning
Layer 4: Transport      → QUIC + WireGuard
Layer 3: Network        → Multi-Hop Onion Routing
Layer 2: Data Link      → DAITA Traffic Analysis Defense
Layer 1: Physical       → RAM-Only Servers + Secure Boot
```

## 🏆 Bug-Bounty-Programm

| Severity | Reward |
|----------|--------|
| 🔴 Critical | $10,000 - $50,000 |
| 🟠 High | $5,000 - $10,000 |
| 🟡 Medium | $1,000 - $5,000 |
| 🟢 Low | $100 - $1,000 |

---

# 📊 Benchmarks

| Metric | VantisVPN | WireGuard | OpenVPN |
|--------|-----------|-----------|---------|
| Throughput | 9.2 Gbps | 8.5 Gbps | 1.2 Gbps |
| Latency | 0.8ms | 1.2ms | 15ms |
| Handshake | 1-RTT | 1-RTT | 6-RTT |
| PQC Key Exchange | 0.3ms | N/A | N/A |
| Memory Usage | 12MB | 8MB | 45MB |
| Connection Time | 50ms | 100ms | 2000ms |

---

# 🚀 Installation

## 📥 Systemanforderungen

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **OS** | Linux/macOS/Windows | Linux (Ubuntu 22.04+) |
| **CPU** | 2 cores | 4+ cores |
| **RAM** | 512 MB | 2+ GB |
| **Disk** | 100 MB | 500 MB |
| **Rust** | 1.94+ | Latest stable |

---

# ⚙️ Konfiguration

```yaml
# ~/.config/vantisvpn/config.yaml

general:
  log_level: info
  auto_connect: true

network:
  protocol: quic
  port: 51820
  mtu: 1420

security:
  kill_switch: true
  dns_leak_protection: true
  zero_trust: true

privacy:
  anonymous_dns: true
  no_logs: true
```

---

# 🧪 Tests

## 📊 Testabdeckung

```
┌──────────────────────────────────────────┐
│ Module          │ Coverage │ Tests       │
├──────────────────────────────────────────┤
│ crypto/         │ 92%      │ 89 tests   │
│ network/        │ 88%      │ 124 tests  │
│ security/       │ 85%      │ 78 tests   │
│ privacy/        │ 90%      │ 65 tests   │
│ server/         │ 87%      │ 72 tests   │
│ tunnel/         │ 91%      │ 41 tests   │
├──────────────────────────────────────────┤
│ Total           │ 89%      │ 469 tests  │
└──────────────────────────────────────────┘
```

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

---

# 🗺️ Fahrplan

- [x] v1.0.0 — Core VPN engine
- [x] v1.1.0 — Comprehensive test coverage
- [ ] v1.2.0 — Enterprise security features
- [ ] v1.3.0 — Desktop application (Tauri)
- [ ] v2.0.0 — Mobile apps + Web dashboard
- [ ] v3.0.0 — Decentralized VPN mesh

---

# 🤝 Mitwirken

## 🎯 Wie man mitwirkt

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

# 📜 Doppellizenzierung

VantisVPN uses a dual license model:

| | AGPL v3 (Open Source) | Commercial |
|---|---|---|
| **Personal use** | ✅ | ✅ |
| **Commercial use** | ⚠️ (AGPL terms) | ✅ |
| **Modify & distribute** | ✅ (share alike) | ✅ |
| **Private modifications** | ❌ (must share) | ✅ |
| **Support** | Community | Priority |

---

# 📞 Kontakt & Support

## 🌐 Soziale Medien

| Platform | Link |
|----------|------|
| 🟣 Discord | [discord.gg/vantisvpn](https://discord.gg/vantisvpn) |
| 📷 Instagram | [@vantisvpn](https://instagram.com/vantisvpn) |
| 🐦 X (Twitter) | [@vantisvpn](https://x.com/vantisvpn) |
| 💼 LinkedIn | [VantisCorp](https://linkedin.com/company/vantisCorp) |
| 📱 Reddit | [r/vantisvpn](https://reddit.com/r/vantisvpn) |

## 📧 Contact

| Type | Email |
|------|-------|
| 🏢 Business | business@vantisvpn.com |
| 🔒 Security | security@vantisvpn.com |
| 📞 Support | support@vantisvpn.com |

---

## 🙏 Vielen Dank!

Vielen Dank für Ihr Interesse an VantisVPN! Gemeinsam bauen wir ein sichereres Internet.

---

<div align="center">

**[⬆ Zurück zur Haupt-README](README.md)**

🇩🇪 Deutsch | Made with ❤️ by [VantisCorp](https://github.com/vantisCorp)

</div>
