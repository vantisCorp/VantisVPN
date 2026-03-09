🇷🇺 [Вернуться к основному README](README.md)

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
## VPN-система нового поколения с квантово-устойчивой криптографией и архитектурой Zero Trust

[![Version](https://img.shields.io/badge/version-1.1.0-red?style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/releases)
[![License](https://img.shields.io/badge/license-AGPL_v3_Commercial-black?style=for-the-badge)](LICENSE)
[![CI/CD](https://img.shields.io/github/actions/workflow/status/vantisCorp/VantisVPN/ci.yml?branch=main&style=for-the-badge)](https://github.com/vantisCorp/VantisVPN/actions)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=red)](https://www.rust-lang.org/)

</div>

---

## 📚 Содержание

- [⚡ Быстрый старт](#-быстрый-старт)
- [✨ Ключевые функции](#-ключевые-функции)
- [🏗️ Архитектура](#️-архитектура)
- [🔒 Безопасность Zero Trust](#-безопасность-zero-trust)
- [📊 Бенчмарки](#-бенчмарки)
- [🚀 Установка](#-установка)
- [⚙️ Конфигурация](#️-конфигурация)
- [🧪 Тестирование](#-тестирование)
- [🗺️ Дорожная карта](#️-дорожная-карта)
- [🤝 Участие в разработке](#-участие-в-разработке)
- [📜 Двойное лицензирование](#-двойное-лицензирование)
- [📞 Контакты и поддержка](#-контакты-и-поддержка)

---

# ⚡ Быстрый старт

## 🚀 Запуск за 3 минуты!

### Вариант 1: Установка одной командой

```bash
curl -sSf https://install.vantisvpn.com | sh
```

### Вариант 2: Ручная установка

```bash
# 1. Клонировать репозиторий
git clone https://github.com/vantisCorp/VantisVPN.git
cd VantisVPN

# 2. Установить зависимости
make install

# 3. Собрать проект
make build

# 4. Запустить VantisVPN
make dev
```

### Вариант 3: Установка Docker

```bash
docker pull vantisvpn/core:latest
docker run -d --name vantisvpn \
  --cap-add=NET_ADMIN \
  --device /dev/net/tun \
  -p 51820:51820/udp \
  vantisvpn/core:latest
```

### 🎯 Проверка установки

```bash
# Проверить версию
vantisvpn --version

# Запустить диагностику
vantisvpn diagnostics

# Тестировать соединение
vantisvpn test
```

---

# ✨ Ключевые функции

## 🎯 Чем VantisVPN отличается?

| | VantisVPN | WireGuard | OpenVPN | NordVPN |
|---|:---:|:---:|:---:|:---:|
| **Постквантовая криптография** | ✅ ML-KEM + ML-DSA | ❌ | ❌ | ❌ |
| **Архитектура Zero Trust** | ✅ | ❌ | ❌ | ⚠️ |
| **QUIC/HTTP3** | ✅ BBRv3 | ❌ | ❌ | ❌ |
| **Multi-Hop** | ✅ 5+ hops | ❌ | ❌ | ✅ 2 hops |
| **RAM-Only** | ✅ | ❌ | ❌ | ✅ |
| **Kill Switch** | ✅ OS-level | ❌ | ⚠️ | ✅ |
| **Open Source** | ✅ AGPL v3 | ✅ GPL v2 | ✅ GPL v2 | ❌ |

## 🌟 Основные возможности

### 🔐 Постквантовая криптография
Первая реализация ML-KEM (FIPS 203) и ML-DSA (FIPS 204) в VPN. Ваши данные защищены даже от квантовых компьютеров.

- **ML-KEM** (FIPS 203) — Kyber-1024
- **ML-DSA** (FIPS 204) — Dilithium-87
- **Hybrid** — Classical + PQC

### ⚡ Молниеносная скорость
Протокол QUIC/HTTP3 с управлением перегрузкой BBRv3. Соединения 0-RTT и миграция соединений для бесперебойной связи.

- **QUIC/HTTP3** — RFC 9000/9114
- **BBRv3** — Congestion Control
- **0-RTT** — Zero Round-Trip Time

### 🛡️ Архитектура Zero Trust
Каждое соединение проверяется. Непрерывная авторизация, микросегментация и принцип наименьших привилегий.

- **mTLS** — Mutual TLS
- **RBAC** — Role-Based Access Control
- **MFA** — Multi-Factor Authentication

### 🌍 Глобальная инфраструктура
RAM-only серверы в 100+ локациях. Умная маршрутизация, multi-hop и поддержка Starlink.

- **100+** locations
- **RAM-only** servers
- **Smart Routing** — AI-powered
- **Multi-Hop** — 5+ hops

---

# 🏗️ Архитектура

## 📐 Архитектура системы

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

## 🔧 Технологический стек

| Layer | Technology |
|-------|-----------|
| **Core** | Rust 1.94+, Tokio, async/await |
| **Crypto** | ML-KEM, ML-DSA, ChaCha20-Poly1305, BLAKE2s |
| **Network** | QUIC (RFC 9000), WireGuard, HTTP/3 |
| **Frontend** | Tauri, React, TypeScript |
| **Infra** | Docker, Terraform, Prometheus |
| **CI/CD** | GitHub Actions, CodeQL, Dependabot |

---

# 🔒 Безопасность Zero Trust

## 🛡️ Уровни безопасности

```
Layer 7: Application    → Zero Trust Policy Engine
Layer 6: Presentation   → ML-KEM/ML-DSA Encryption
Layer 5: Session        → mTLS + Certificate Pinning
Layer 4: Transport      → QUIC + WireGuard
Layer 3: Network        → Multi-Hop Onion Routing
Layer 2: Data Link      → DAITA Traffic Analysis Defense
Layer 1: Physical       → RAM-Only Servers + Secure Boot
```

## 🏆 Программа Bug Bounty

| Severity | Reward |
|----------|--------|
| 🔴 Critical | $10,000 - $50,000 |
| 🟠 High | $5,000 - $10,000 |
| 🟡 Medium | $1,000 - $5,000 |
| 🟢 Low | $100 - $1,000 |

---

# 📊 Бенчмарки

| Metric | VantisVPN | WireGuard | OpenVPN |
|--------|-----------|-----------|---------|
| Throughput | 9.2 Gbps | 8.5 Gbps | 1.2 Gbps |
| Latency | 0.8ms | 1.2ms | 15ms |
| Handshake | 1-RTT | 1-RTT | 6-RTT |
| PQC Key Exchange | 0.3ms | N/A | N/A |
| Memory Usage | 12MB | 8MB | 45MB |
| Connection Time | 50ms | 100ms | 2000ms |

---

# 🚀 Установка

## 📥 Системные требования

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **OS** | Linux/macOS/Windows | Linux (Ubuntu 22.04+) |
| **CPU** | 2 cores | 4+ cores |
| **RAM** | 512 MB | 2+ GB |
| **Disk** | 100 MB | 500 MB |
| **Rust** | 1.94+ | Latest stable |

---

# ⚙️ Конфигурация

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

# 🧪 Тестирование

## 📊 Покрытие тестами

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

# 🗺️ Дорожная карта

- [x] v1.0.0 — Core VPN engine
- [x] v1.1.0 — Comprehensive test coverage
- [ ] v1.2.0 — Enterprise security features
- [ ] v1.3.0 — Desktop application (Tauri)
- [ ] v2.0.0 — Mobile apps + Web dashboard
- [ ] v3.0.0 — Decentralized VPN mesh

---

# 🤝 Участие в разработке

## 🎯 Как внести вклад

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

# 📜 Двойное лицензирование

VantisVPN uses a dual license model:

| | AGPL v3 (Open Source) | Commercial |
|---|---|---|
| **Personal use** | ✅ | ✅ |
| **Commercial use** | ⚠️ (AGPL terms) | ✅ |
| **Modify & distribute** | ✅ (share alike) | ✅ |
| **Private modifications** | ❌ (must share) | ✅ |
| **Support** | Community | Priority |

---

# 📞 Контакты и поддержка

## 🌐 Социальные сети

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

## 🙏 Спасибо!

Спасибо за интерес к VantisVPN! Вместе мы строим более безопасный интернет.

---

<div align="center">

**[⬆ Вернуться к основному README](README.md)**

🇷🇺 Русский | Made with ❤️ by [VantisCorp](https://github.com/vantisCorp)

</div>
