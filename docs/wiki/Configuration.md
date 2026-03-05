# Configuration Guide

Complete guide to configuring VantisVPN.

## Table of Contents

- [Configuration Files](#configuration-files)
- [Basic Configuration](#basic-configuration)
- [Network Settings](#network-settings)
- [Security Settings](#security-settings)
- [Advanced Settings](#advanced-settings)
- [Environment Variables](#environment-variables)

## Configuration Files

VantisVPN uses YAML configuration files located at:

| Location | Description |
|----------|-------------|
| `/etc/vantisvpn/config.yaml` | System-wide configuration |
| `~/.config/vantisvpn/config.yaml` | User configuration |
| `./vantisvpn.yaml` | Local directory configuration |

### Configuration Priority

Configuration is loaded in the following order (later values override earlier ones):

1. System configuration (`/etc/vantisvpn/`)
2. User configuration (`~/.config/vantisvpn/`)
3. Local configuration (`./vantisvpn.yaml`)
4. Environment variables
5. Command-line arguments

## Basic Configuration

### Default Configuration File

```yaml
# VantisVPN Configuration File
# Generated: 2025-01-XX

# General Settings
general:
  # Auto-connect on startup
  auto_connect: false
  # Default server (empty = fastest)
  default_server: ""
  # Protocol: quic, wireguard, openvpn
  protocol: quic
  # Log level: trace, debug, info, warn, error
  log_level: info

# Account Settings
account:
  # Your VantisVPN account email
  email: ""
  # Authentication method: token, password, sso
  auth_method: token

# Network Settings
network:
  # DNS servers to use (comma-separated)
  dns:
    - 1.1.1.1
    - 1.0.0.1
  # DNS over HTTPS
  dns_over_https: true
  # IPv6 support
  ipv6: true
  # Local network bypass
  lan_bypass: true

# Security Settings
security:
  # Kill switch (block internet if VPN disconnects)
  kill_switch: false
  # DNS leak protection
  dns_leak_protection: true
  # WebRTC leak protection
  webrtc_leak_protection: true
  # Encryption level: standard, high, quantum-safe
  encryption: quantum-safe

# Advanced Settings
advanced:
  # MTU size
  mtu: 1400
  # Connection timeout (seconds)
  timeout: 30
  # Reconnect attempts
  reconnect_attempts: 3
  # Background mode
  background: true
```

### Managing Configuration

```bash
# Initialize configuration with defaults
vantisvpn config init

# Show current configuration
vantisvpn config show

# Edit configuration in editor
vantisvpn config edit

# Validate configuration
vantisvpn config validate

# Set a specific value
vantisvpn config set general.protocol wireguard

# Get a specific value
vantisvpn config get general.protocol
```

## Network Settings

### DNS Configuration

```yaml
network:
  dns:
    # Primary DNS
    - 1.1.1.1
    # Secondary DNS
    - 1.0.0.1
  # Enable DNS over HTTPS
  dns_over_https: true
  # Custom DNS servers for specific domains
  custom_dns:
    "internal.company.com": 10.0.0.1
```

### Split Tunneling

```yaml
network:
  split_tunnel:
    # Mode: exclude (VPN bypass) or include (VPN only)
    mode: exclude
    # Applications to exclude/include
    applications:
      - firefox
      - chrome
    # Domains to exclude/include
    domains:
      - localhost
      - "*.internal.company.com"
    # IP addresses to exclude/include
    ips:
      - 10.0.0.0/8
      - 192.168.0.0/16
```

### Port Forwarding

```yaml
network:
  port_forwarding:
    # Enable port forwarding
    enabled: true
    # Requested ports (empty = automatic)
    ports:
      - 8080
      - 8443
```

## Security Settings

### Kill Switch

```yaml
security:
  kill_switch:
    # Enable kill switch
    enabled: true
    # Mode: system (block all) or application (block specific)
    mode: system
    # Allow LAN when kill switch is active
    allow_lan: true
```

### Encryption

```yaml
security:
  encryption:
    # Level: standard, high, quantum-safe
    level: quantum-safe
    # Cipher suite (for advanced users)
    cipher: chacha20-poly1305
    # Key exchange: x25519, ml-kem (post-quantum)
    key_exchange: ml-kem
    # Forward secrecy
    forward_secrecy: true
```

### Authentication

```yaml
security:
  authentication:
    # Two-factor authentication
    two_factor: true
    # Token refresh interval (hours)
    token_refresh: 24
    # Session timeout (hours)
    session_timeout: 720
```

## Advanced Settings

### Performance Tuning

```yaml
advanced:
  # Number of worker threads
  worker_threads: 4
  # Buffer size (KB)
  buffer_size: 64
  # TCP buffer size
  tcp_buffer: 128KB
  # UDP buffer size
  udp_buffer: 128KB
  # Connection pooling
  connection_pool: true
```

### Logging

```yaml
advanced:
  logging:
    # Log file location
    file: /var/log/vantisvpn/vantisvpn.log
    # Max file size (MB)
    max_size: 100
    # Number of backup files
    max_backups: 5
    # Log format: json, text
    format: json
    # Include timestamps
    timestamps: true
```

### Proxy Settings

```yaml
advanced:
  proxy:
    # Proxy type: http, socks5
    type: socks5
    # Proxy address
    address: 127.0.0.1:1080
    # Proxy authentication
    username: ""
    password: ""
```

## Environment Variables

VantisVPN can be configured using environment variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `VANTISVPN_CONFIG` | Configuration file path | `/etc/vantisvpn/custom.yaml` |
| `VANTISVPN_SERVER` | Default server | `us-west-1` |
| `VANTISVPN_PROTOCOL` | VPN protocol | `quic` |
| `VANTISVPN_LOG_LEVEL` | Log level | `debug` |
| `VANTISVPN_KILL_SWITCH` | Enable kill switch | `true` |
| `VANTISVPN_DNS` | DNS servers | `1.1.1.1,8.8.8.8` |
| `VANTISVPN_NO_IPV6` | Disable IPv6 | `true` |
| `VANTISVPN_BACKGROUND` | Background mode | `true` |

### Example Usage

```bash
# Run with environment variables
VANTISVPN_SERVER=eu-central-1 VANTISVPN_PROTOCOL=wireguard vantisvpn connect

# Use custom config file
VANTISVPN_CONFIG=/path/to/config.yaml vantisvpn start
```

## Server Selection

### Server Configuration

```yaml
servers:
  # Favorite servers
  favorites:
    - us-west-1
    - eu-central-1
  # Country preferences
  preferred_countries:
    - US
    - DE
    - NL
  # Server types
  types:
    - standard
    - p2p
    - streaming
  # Minimum latency (ms)
  max_latency: 100
```

### Load Balancing

```yaml
servers:
  load_balancing:
    # Mode: fastest, random, round-robin
    mode: fastest
    # Check interval (seconds)
    check_interval: 300
    # Failover on error
    failover: true
```

## Profile Configuration

You can create multiple configuration profiles:

```bash
# Create a profile
vantisvpn profile create work

# List profiles
vantisvpn profile list

# Switch profile
vantisvpn profile use work

# Delete profile
vantisvpn profile delete work
```

### Profile File Structure

```yaml
# ~/.config/vantisvpn/profiles/work.yaml
name: work
inherits: default
servers:
  favorites:
    - us-west-1
security:
  kill_switch: true
network:
  split_tunnel:
    mode: include
    applications:
      - slack
      - zoom
```

## Validation

### Validate Configuration

```bash
# Validate current configuration
vantisvpn config validate

# Validate specific file
vantisvpn config validate --file /path/to/config.yaml

# Show warnings
vantisvpn config validate --warnings
```

## Examples

### Gaming Profile

```yaml
name: gaming
general:
  protocol: wireguard
servers:
  types:
    - gaming
  max_latency: 50
security:
  kill_switch: true
network:
  lan_bypass: true
advanced:
  mtu: 1500
  buffer_size: 128
```

### Streaming Profile

```yaml
name: streaming
general:
  protocol: quic
servers:
  types:
    - streaming
security:
  kill_switch: false
network:
  dns:
    - 8.8.8.8
```

### Privacy-First Profile

```yaml
name: privacy
general:
  protocol: wireguard
security:
  kill_switch: true
  encryption: quantum-safe
  dns_leak_protection: true
  webrtc_leak_protection: true
network:
  ipv6: false
  dns:
    - 9.9.9.9
    - 149.112.112.112
```

## Getting Help

For configuration issues:

```bash
# Show configuration help
vantisvpn config --help

# Show all configuration options
vantisvpn config options

# Export configuration
vantisvpn config export > my-config.yaml

# Import configuration
vantisvpn config import my-config.yaml
```

## See Also

- [Installation Guide](Installation-Guide)
- [Quick Start](Quick-Start)
- [Troubleshooting](Troubleshooting)
- [Security Model](Security-Model)