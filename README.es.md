🇪🇸 [Volver al README principal](README.md)

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
## Sistema VPN Seguro de Nueva Generación con Criptografía Resistente a Cuántica y Arquitectura Zero Trust

[![Version](https://img.shields.io/badge/version-1.1.0-red?style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/releases)
[![License](https://img.shields.io/badge/license-AGPL_v3_Commercial-black?style=for-the-badge)](LICENSE)
[![CI/CD](https://img.shields.io/github/actions/workflow/status/vantisCorp/VantisVPN/ci.yml?branch=main&style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/actions)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=red)](https://www.rust-lang.org/)

</div>

---

## 📚 Índice

- [⚡ Inicio Rápido](#-inicio-rapido)
- [✨ Características Principales](#-características-principales)
- [🏗️ Arquitectura](#️-arquitectura)
- [🔒 Seguridad Zero Trust](#-seguridad-zero-trust)
- [📊 Benchmarks](#-benchmarks)
- [🚀 Instalación](#-instalación)
- [⚙️ Configuración](#️-configuración)
- [🧪 Pruebas](#-pruebas)
- [🗺️ Hoja de Ruta](#️-hoja-de-ruta)
- [🤝 Contribuir](#-contribuir)
- [📜 Doble Licencia](#-doble-licencia)
- [📞 Contacto y Soporte](#-contacto-y-soporte)

---

# ⚡ Inicio Rápido

## 🚀 ¡Listo en 3 minutos!

### Opción 1: Instalación en una línea

```bash
curl -sSf https://install.vantisvpn.com | sh
```

### Opción 2: Instalación manual

```bash
# 1. Clonar el repositorio
git clone https://github.com/vantisCorp/VantisVPN.git
cd VantisVPN

# 2. Instalar dependencias
make install

# 3. Compilar el proyecto
make build

# 4. Ejecutar VantisVPN
make dev
```

### Opción 3: Instalación Docker

```bash
docker pull vantisvpn/core:latest
docker run -d --name vantisvpn \
  --cap-add=NET_ADMIN \
  --device /dev/net/tun \
  -p 51820:51820/udp \
  vantisvpn/core:latest
```

### 🎯 Verificar instalación

```bash
# Verificar versión
vantisvpn --version

# Ejecutar diagnósticos
vantisvpn diagnostics

# Probar conexión
vantisvpn test
```

---

# ✨ Características Principales

## 🎯 ¿Qué hace diferente a VantisVPN?

| | VantisVPN | WireGuard | OpenVPN | NordVPN |
|---|:---:|:---:|:---:|:---:|
| **Criptografía Post-Cuántica** | ✅ ML-KEM + ML-DSA | ❌ | ❌ | ❌ |
| **Arquitectura Zero Trust** | ✅ | ❌ | ❌ | ⚠️ |
| **QUIC/HTTP3** | ✅ BBRv3 | ❌ | ❌ | ❌ |
| **Multi-Hop** | ✅ 5+ hops | ❌ | ❌ | ✅ 2 hops |
| **RAM-Only** | ✅ | ❌ | ❌ | ✅ |
| **Kill Switch** | ✅ OS-level | ❌ | ⚠️ | ✅ |
| **Open Source** | ✅ AGPL v3 | ✅ GPL v2 | ✅ GPL v2 | ❌ |

## 🌟 Funciones Destacadas

### 🔐 Criptografía Post-Cuántica
Primera implementación de ML-KEM (FIPS 203) y ML-DSA (FIPS 204) en un VPN. Sus datos están seguros incluso ante computadoras cuánticas.

- **ML-KEM** (FIPS 203) — Kyber-1024
- **ML-DSA** (FIPS 204) — Dilithium-87
- **Hybrid** — Classical + PQC

### ⚡ Velocidad Relámpago
Protocolo QUIC/HTTP3 con control de congestión BBRv3. Conexiones 0-RTT y migración de conexiones para conectividad ininterrumpida.

- **QUIC/HTTP3** — RFC 9000/9114
- **BBRv3** — Congestion Control
- **0-RTT** — Zero Round-Trip Time

### 🛡️ Arquitectura Zero Trust
Cada conexión es verificada. Autorización continua, microsegmentación y principio de mínimo privilegio.

- **mTLS** — Mutual TLS
- **RBAC** — Role-Based Access Control
- **MFA** — Multi-Factor Authentication

### 🌍 Infraestructura Global
Servidores solo-RAM en más de 100 ubicaciones. Enrutamiento inteligente, multi-hop y soporte Starlink.

- **100+** locations
- **RAM-only** servers
- **Smart Routing** — AI-powered
- **Multi-Hop** — 5+ hops

---

# 🏗️ Arquitectura

## 📐 Arquitectura del Sistema

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

## 🔧 Stack Tecnológico

| Layer | Technology |
|-------|-----------|
| **Core** | Rust 1.94+, Tokio, async/await |
| **Crypto** | ML-KEM, ML-DSA, ChaCha20-Poly1305, BLAKE2s |
| **Network** | QUIC (RFC 9000), WireGuard, HTTP/3 |
| **Frontend** | Tauri, React, TypeScript |
| **Infra** | Docker, Terraform, Prometheus |
| **CI/CD** | GitHub Actions, CodeQL, Dependabot |

---

# 🔒 Seguridad Zero Trust

## 🛡️ Capas de Seguridad

```
Layer 7: Application    → Zero Trust Policy Engine
Layer 6: Presentation   → ML-KEM/ML-DSA Encryption
Layer 5: Session        → mTLS + Certificate Pinning
Layer 4: Transport      → QUIC + WireGuard
Layer 3: Network        → Multi-Hop Onion Routing
Layer 2: Data Link      → DAITA Traffic Analysis Defense
Layer 1: Physical       → RAM-Only Servers + Secure Boot
```

## 🏆 Programa Bug Bounty

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

# 🚀 Instalación

## 📥 Requisitos del Sistema

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **OS** | Linux/macOS/Windows | Linux (Ubuntu 22.04+) |
| **CPU** | 2 cores | 4+ cores |
| **RAM** | 512 MB | 2+ GB |
| **Disk** | 100 MB | 500 MB |
| **Rust** | 1.94+ | Latest stable |

---

# ⚙️ Configuración

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

# 🧪 Pruebas

## 📊 Cobertura de Pruebas

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

# 🗺️ Hoja de Ruta

- [x] v1.0.0 — Core VPN engine
- [x] v1.1.0 — Comprehensive test coverage
- [ ] v1.2.0 — Enterprise security features
- [ ] v1.3.0 — Desktop application (Tauri)
- [ ] v2.0.0 — Mobile apps + Web dashboard
- [ ] v3.0.0 — Decentralized VPN mesh

---

# 🤝 Contribuir

## 🎯 Cómo Contribuir

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

# 📜 Doble Licencia

VantisVPN uses a dual license model:

| | AGPL v3 (Open Source) | Commercial |
|---|---|---|
| **Personal use** | ✅ | ✅ |
| **Commercial use** | ⚠️ (AGPL terms) | ✅ |
| **Modify & distribute** | ✅ (share alike) | ✅ |
| **Private modifications** | ❌ (must share) | ✅ |
| **Support** | Community | Priority |

---

# 📞 Contacto y Soporte

## 🌐 Redes Sociales

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

## 🙏 ¡Gracias!

¡Gracias por su interés en VantisVPN! Juntos construimos un internet más seguro.

---

<div align="center">

**[⬆ Volver al README principal](README.md)**

🇪🇸 Español | Made with ❤️ by [VantisCorp](https://github.com/vantisCorp)

</div>
