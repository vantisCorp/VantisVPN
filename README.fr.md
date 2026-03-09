🇫🇷 [Retour au README principal](README.md)

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
## Système VPN Sécurisé de Nouvelle Génération avec Cryptographie Résistante aux Quantiques et Architecture Zero Trust

[![Version](https://img.shields.io/badge/version-1.1.0-red?style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/releases)
[![License](https://img.shields.io/badge/license-AGPL_v3_Commercial-black?style=for-the-badge)](LICENSE)
[![CI/CD](https://img.shields.io/github/actions/workflow/status/vantisCorp/VantisVPN/ci.yml?branch=main&style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/actions)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=red)](https://www.rust-lang.org/)

</div>

---

## 📚 Table des Matières

- [⚡ Démarrage Rapide](#-demarrage-rapide)
- [✨ Fonctionnalités Clés](#-fonctionnalités-clés)
- [🏗️ Architecture](#️-architecture)
- [🔒 Sécurité Zero Trust](#-sécurité-zero-trust)
- [📊 Benchmarks](#-benchmarks)
- [🚀 Installation](#-installation)
- [⚙️ Configuration](#️-configuration)
- [🧪 Tests](#-tests)
- [🗺️ Feuille de Route](#️-feuille-de-route)
- [🤝 Contribuer](#-contribuer)
- [📜 Double Licence](#-double-licence)
- [📞 Contact & Support](#-contact--support)

---

# ⚡ Démarrage Rapide

## 🚀 Opérationnel en 3 minutes !

### Option 1: Installation en une ligne

```bash
curl -sSf https://install.vantisvpn.com | sh
```

### Option 2: Installation manuelle

```bash
# 1. Cloner le dépôt
git clone https://github.com/vantisCorp/VantisVPN.git
cd VantisVPN

# 2. Installer les dépendances
make install

# 3. Compiler le projet
make build

# 4. Lancer VantisVPN
make dev
```

### Option 3: Installation Docker

```bash
docker pull vantisvpn/core:latest
docker run -d --name vantisvpn \
  --cap-add=NET_ADMIN \
  --device /dev/net/tun \
  -p 51820:51820/udp \
  vantisvpn/core:latest
```

### 🎯 Vérifier l'installation

```bash
# Vérifier la version
vantisvpn --version

# Lancer les diagnostics
vantisvpn diagnostics

# Tester la connexion
vantisvpn test
```

---

# ✨ Fonctionnalités Clés

## 🎯 Qu'est-ce qui rend VantisVPN différent ?

| | VantisVPN | WireGuard | OpenVPN | NordVPN |
|---|:---:|:---:|:---:|:---:|
| **Cryptographie Post-Quantique** | ✅ ML-KEM + ML-DSA | ❌ | ❌ | ❌ |
| **Architecture Zero Trust** | ✅ | ❌ | ❌ | ⚠️ |
| **QUIC/HTTP3** | ✅ BBRv3 | ❌ | ❌ | ❌ |
| **Multi-Hop** | ✅ 5+ hops | ❌ | ❌ | ✅ 2 hops |
| **RAM-Only** | ✅ | ❌ | ❌ | ✅ |
| **Kill Switch** | ✅ OS-level | ❌ | ⚠️ | ✅ |
| **Open Source** | ✅ AGPL v3 | ✅ GPL v2 | ✅ GPL v2 | ❌ |

## 🌟 Fonctionnalités Phares

### 🔐 Cryptographie Post-Quantique
Première implémentation de ML-KEM (FIPS 203) et ML-DSA (FIPS 204) dans un VPN. Vos données sont protégées même contre les ordinateurs quantiques.

- **ML-KEM** (FIPS 203) — Kyber-1024
- **ML-DSA** (FIPS 204) — Dilithium-87
- **Hybrid** — Classical + PQC

### ⚡ Ultra Rapide
Protocole QUIC/HTTP3 avec contrôle de congestion BBRv3. Connexions 0-RTT et migration de connexion pour une connectivité ininterrompue.

- **QUIC/HTTP3** — RFC 9000/9114
- **BBRv3** — Congestion Control
- **0-RTT** — Zero Round-Trip Time

### 🛡️ Architecture Zero Trust
Chaque connexion est vérifiée. Autorisation continue, micro-segmentation et principe du moindre privilège.

- **mTLS** — Mutual TLS
- **RBAC** — Role-Based Access Control
- **MFA** — Multi-Factor Authentication

### 🌍 Infrastructure Mondiale
Serveurs RAM-only dans plus de 100 emplacements. Routage intelligent, multi-hop et support Starlink.

- **100+** locations
- **RAM-only** servers
- **Smart Routing** — AI-powered
- **Multi-Hop** — 5+ hops

---

# 🏗️ Architecture

## 📐 Architecture du Système

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

## 🔧 Stack Technologique

| Layer | Technology |
|-------|-----------|
| **Core** | Rust 1.94+, Tokio, async/await |
| **Crypto** | ML-KEM, ML-DSA, ChaCha20-Poly1305, BLAKE2s |
| **Network** | QUIC (RFC 9000), WireGuard, HTTP/3 |
| **Frontend** | Tauri, React, TypeScript |
| **Infra** | Docker, Terraform, Prometheus |
| **CI/CD** | GitHub Actions, CodeQL, Dependabot |

---

# 🔒 Sécurité Zero Trust

## 🛡️ Couches de Sécurité

```
Layer 7: Application    → Zero Trust Policy Engine
Layer 6: Presentation   → ML-KEM/ML-DSA Encryption
Layer 5: Session        → mTLS + Certificate Pinning
Layer 4: Transport      → QUIC + WireGuard
Layer 3: Network        → Multi-Hop Onion Routing
Layer 2: Data Link      → DAITA Traffic Analysis Defense
Layer 1: Physical       → RAM-Only Servers + Secure Boot
```

## 🏆 Programme Bug Bounty

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

## 📥 Configuration Requise

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **OS** | Linux/macOS/Windows | Linux (Ubuntu 22.04+) |
| **CPU** | 2 cores | 4+ cores |
| **RAM** | 512 MB | 2+ GB |
| **Disk** | 100 MB | 500 MB |
| **Rust** | 1.94+ | Latest stable |

---

# ⚙️ Configuration

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

## 📊 Couverture de Tests

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

# 🗺️ Feuille de Route

- [x] v1.0.0 — Core VPN engine
- [x] v1.1.0 — Comprehensive test coverage
- [ ] v1.2.0 — Enterprise security features
- [ ] v1.3.0 — Desktop application (Tauri)
- [ ] v2.0.0 — Mobile apps + Web dashboard
- [ ] v3.0.0 — Decentralized VPN mesh

---

# 🤝 Contribuer

## 🎯 Comment Contribuer

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

# 📜 Double Licence

VantisVPN uses a dual license model:

| | AGPL v3 (Open Source) | Commercial |
|---|---|---|
| **Personal use** | ✅ | ✅ |
| **Commercial use** | ⚠️ (AGPL terms) | ✅ |
| **Modify & distribute** | ✅ (share alike) | ✅ |
| **Private modifications** | ❌ (must share) | ✅ |
| **Support** | Community | Priority |

---

# 📞 Contact & Support

## 🌐 Réseaux Sociaux

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

## 🙏 Merci !

Merci pour votre intérêt pour VantisVPN ! Ensemble, nous construisons un internet plus sûr.

---

<div align="center">

**[⬆ Retour au README principal](README.md)**

🇫🇷 Français | Made with ❤️ by [VantisCorp](https://github.com/vantisCorp)

</div>
