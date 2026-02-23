# VANTISVPN - Architecture Overview

## Executive Summary

VANTISVPN jest zaprojektowany jako bezpieczny system typu "Privacy by Design", wykorzystujący nowoczesną architekturę mikrousług z rdzeniem napisanym w języku Rust dla maksymalnego bezpieczeństwa pamięci i wydajności.

## Architektura High-Level

```
┌─────────────────────────────────────────────────────────────┐
│                      VANTISVPN SYSTEM                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │   UI Layer   │  │   Network    │  │   Crypto     │        │
│  │   (Tauri)    │  │   Layer      │  │   Layer      │        │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘        │
│         │                 │                 │                │
│         └─────────────────┼─────────────────┘                │
│                           │                                  │
│                    ┌──────▼──────┐                           │
│                    │  Rust Core  │                           │
│                    │  (Shared)   │                           │
│                    └──────┬──────┘                           │
│                           │                                  │
│         ┌─────────────────┼─────────────────┐                │
│         │                 │                 │                │
│  ┌──────▼──────┐  ┌──────▼──────┐  ┌──────▼──────┐          │
│  │   Windows   │  │   macOS     │  │    Linux    │          │
│  │   Driver    │  │   Driver    │  │   Driver    │          │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ Encrypted Tunnel
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                  SERVER INFRASTRUCTURE                       │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │  RAM-Only    │  │  MultiHop+   │  │   Smart      │        │
│  │  Servers     │  │  Routing     │  │   AI Router  │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Komponenty Systemu

### 1. Rust Core (Shared Library)

Wspólna biblioteka napisana w języku Rust, zawierająca:
- Logikę kryptograficzną
- Implementację protokołów sieciowych
- Zarządzanie tunelami
- Obsługę błędów

**Zalety:**
- Bezpieczeństwo pamięci (brak buffer overflow)
- Wydajność porównywalna z C/C++
- Cross-platform (Windows, macOS, Linux, Android, iOS)

### 2. UI Layer (Tauri)

Interfejs użytkownika oparty na frameworku Tauri:
- Ultralekki (w przeciwieństwie do Electrona)
- Bezpieczny (minimalna powierzchnia ataku)
- Natywne zintegrowanie z systemem

**Funkcje:**
- Context-Aware UI (adaptacja do sytuacji)
- Wizualizacja 3D tras pakietów
- Biometryczna autoryzacja

### 3. Network Layer

Warstwa odpowiedzialna za komunikację sieciową:
- Implementacja WireGuard (Modified)
- Obsługa QUIC/HTTP/3
- Kernel Bypass (DPDK/eBPF)
- Obsługa IPv6 i IPv4

### 4. Crypto Layer

Warstwa kryptograficzna:
- Post-kwantowa wymiana kluczy (Kyber/ML-KEM)
- Podpisy post-kwantowe (Dilithium/ML-DSA)
- Szyfrowanie symetryczne (ChaCha20-Poly1305)
- Hashowanie (BLAKE2s)

## Architektura Modułowa (Microservices)

System jest podzielony na niezależne kontenery:

```
┌─────────────────────────────────────────────────────┐
│              Microservices Architecture              │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────┐    ┌─────────────┐                │
│  │   GUI App   │    │  Network    │                │
│  │  Service    │◄──►│  Service    │                │
│  └─────────────┘    └─────────────┘                │
│         │                   │                      │
│         │                   │                      │
│         ▼                   ▼                      │
│  ┌─────────────┐    ┌─────────────┐                │
│  │   Crypto    │    │   Tunnel    │                │
│  │  Service    │◄──►│   Service   │                │
│  └─────────────┘    └─────────────┘                │
│         │                   │                      │
│         │                   │                      │
│         ▼                   ▼                      │
│  ┌─────────────┐    ┌─────────────┐                │
│  │   Config    │    │    DNS      │                │
│  │  Service    │    │   Service   │                │
│  └─────────────┘    └─────────────┘                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Zalety mikrousług:**
- Izolacja awarii (awaria jednego serwisu nie wpływa na inne)
- Łatwiejsze utrzymanie i rozwój
- Niezależne wdrażanie
- Skalowalność

## Privacy by Design

### Techniczne Uniemożliwienie Zbierania Logów

Architektura systemu jest zaprojektowana tak, aby technicznie uniemożliwić zbieranie jakichkolwiek logów:

1. **Brak możliwości zapisu IP** - Adres IP użytkownika nigdy nie jest zapisywany w pamięci serwera
2. **Brak timestampów** - Czas połączenia nie jest rejestrowany
3. **Ephemeral Keys** - Klucze sesji są niszczone po rozłączeniu
4. **RAM-Only Servers** - Serwery działają wyłącznie w pamięci RAM
5. **No Persistent Storage** - Brak trwałego przechowywania danych

### Flow Bezpiecznego Połączenia

```
User Device                    VANTISVPN Server                    Internet
     │                              │                                │
     │  1. Generate ephemeral keys │                                │
     │  ──────────────────────────►│                                │
     │                              │  2. Keys discarded after     │
     │                              │     connection ends          │
     │                              │                                │
     │  3. Encrypted tunnel (QUIC) │                                │
     │  ◄──────────────────────────┤                                │
     │                              │                                │
     │  4. Traffic forwarding       │                                │
     │  ◄──────────────────────────┼─────────────────────────────►  │
     │                              │                                │
     │  5. No logs stored           │                                │
     │                              │                                │
```

## Compliance Standards

### FIPS 140-3
- Wykorzystanie bibliotek kryptograficznych certyfikowanych przez NIST
- Modułowa architektura ułatwia certyfikację

### ISO/IEC 27001 & 27002
- Polityki bezpieczeństwa informacji
- Zarządzanie ryzykiem
- Ciągłe doskonalenie

### DoDI 8310.01 (IPv6)
- Natywna obsługa protokołu IPv6
- Wymóg militarny

## Reproducible Builds

System budowania pozwala każdemu zweryfikować, że kod źródłowy odpowiada plikowi instalacyjnemu:

```bash
# Krok 1: Budowanie z deterministycznym środowiskem
docker build -t vantis-build .

# Krok 2: Pobranie oficjalnego builda
wget https://download.vantisvpn.com/v0.1.0.tar.gz

# Krok 3: Weryfikacja hash
sha256sum v0.1.0.tar.gz
sha256sum build/v0.1.0.tar.gz

# Powinny być identyczne
```

## Następne Kroki

1. Szczegółowa dokumentacja każdego komponentu
2. Specyfikacja API
3. Plan implementacji
4. Testy jednostkowe i integracyjne