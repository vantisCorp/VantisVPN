# Quick Start Guide

Get started with VantisVPN in under 5 minutes!

## Installation

### One-Line Install

#### Linux
```bash
curl -fsSL https://install.vantisvpn.com/install.sh | sudo sh
```

#### macOS
```bash
brew install vantisvpn
```

#### Windows
```powershell
winget install VantisVPN.VantisVPN
```

For detailed installation instructions, see the [Installation Guide](Installation-Guide).

## Basic Usage

### 1. Connect to VPN

```bash
# Connect to the fastest server automatically
vantisvpn connect

# Connect to a specific server
vantisvpn connect us-west-1

# Connect to a specific country
vantisvpn connect --country US
```

### 2. Check Status

```bash
# Check connection status
vantisvpn status

# Show detailed information
vantisvpn status --detailed
```

### 3. Disconnect

```bash
# Disconnect from VPN
vantisvpn disconnect
```

## Configuration

### Initial Setup

```bash
# Generate default configuration
vantisvpn config init

# View current configuration
vantisvpn config show

# Edit configuration
vantisvpn config edit
```

### Common Configuration Options

```bash
# Enable auto-connect on startup
vantisvpn config set auto-connect true

# Set preferred protocol
vantisvpn config set protocol quic

# Enable kill switch
vantisvpn config set kill-switch true

# Set DNS server
vantisvpn config set dns 1.1.1.1
```

## Server Selection

### List Available Servers

```bash
# List all servers
vantisvpn list-servers

# List servers by country
vantisvpn list-servers --country US

# Find fastest server
vantisvpn fastest
```

### Filter Servers

```bash
# Find servers with P2P support
vantisvpn list-servers --features p2p

# Find servers with streaming support
vantisvpn list-servers --features streaming

# Find servers with low latency
vantisvpn list-servers --latency <50
```

## Advanced Usage

### Split Tunneling

```bash
# Add application to split tunnel
vantisvpn split-tunnel add firefox

# Remove application
vantisvpn split-tunnel remove firefox

# List split-tunnel rules
vantisvpn split-tunnel list
```

### Port Forwarding

```bash
# Request port forwarding
vantisvpn port-forward request

# Show forwarded ports
vantisvpn port-forward show
```

### Kill Switch

```bash
# Enable kill switch
vantisvpn kill-switch enable

# Disable kill switch
vantisvpn kill-switch disable

# Check kill switch status
vantisvpn kill-switch status
```

## GUI Applications

### Desktop App

VantisVPN provides a native desktop application for Linux, macOS, and Windows.

Features:
- One-click connect
- Server map with latency indicators
- Auto-connect on startup
- Split tunneling GUI
- Connection statistics

Download from [vantisvpn.com/download](https://vantisvpn.com/download).

### Mobile App

Available on iOS and Android with features:
- Auto-connect on untrusted networks
- Widget for quick connect
- Split tunneling by app
- Background connection

Download from App Store or Google Play.

## Common Commands Reference

| Command | Description |
|---------|-------------|
| `vantisvpn connect` | Connect to VPN |
| `vantisvpn disconnect` | Disconnect from VPN |
| `vantisvpn status` | Show connection status |
| `vantisvpn list-servers` | List available servers |
| `vantisvpn config` | Manage configuration |
| `vantisvpn fastest` | Find fastest server |
| `vantisvpn kill-switch` | Manage kill switch |
| `vantisvpn logs` | View logs |

## Testing Your Connection

### Verify IP Address

```bash
# Show current IP
vantisvpn ip

# Show detailed IP information
vantisvpn ip --detailed
```

### Speed Test

```bash
# Run speed test
vantisvpn speed-test

# Speed test with specific server
vantisvpn speed-test us-west-1
```

### DNS Leak Test

```bash
# Test for DNS leaks
vantisvpn dns-leak-test
```

## Troubleshooting

### Connection Issues

```bash
# Check service status
vantisvpn service status

# Restart service
vantisvpn service restart

# View logs
vantisvpn logs --follow
```

### Performance Issues

```bash
# Run diagnostics
vantisvpn diagnose

# Check system requirements
vantisvpn check-system
```

## Getting Help

- **Documentation**: [https://docs.vantisvpn.com](https://docs.vantisvpn.com)
- **Discord**: [https://discord.gg/A5MzwsRj7D](https://discord.gg/A5MzwsRj7D)
- **Email**: support@vantisvpn.com
- **Issues**: [GitHub Issues](https://github.com/vantisCorp/VantisVPN/issues)

## Next Steps

- Read the [Configuration Guide](Configuration)
- Learn about [Security Features](../SECURITY.md)
- Explore [Advanced Configuration](Configuration)
- Join the [Discord Community](https://discord.gg/A5MzwsRj7D)

## Tips & Tricks

### 1. Auto-Connect

Connect automatically when on untrusted networks:

```bash
vantisvpn config set auto-connect untrusted
```

### 2. Favorite Servers

Save your favorite servers for quick access:

```bash
vantisvpn favorite add us-west-1
vantisvpn favorite list
vantisvpn connect --favorite
```

### 3. Schedule Connections

Connect/disconnect on a schedule:

```bash
vantisvpn schedule add "09:00" connect us-west-1
vantisvpn schedule add "18:00" disconnect
```

### 4. Notifications

Get desktop notifications:

```bash
vantisvpn config set notifications true
```

### 5. Custom DNS

Use custom DNS servers:

```bash
vantisvpn config set dns 8.8.8.8,8.8.4.4
```

## Security Best Practices

1. **Always use the latest version**: `vantisvpn update`
2. **Enable kill switch**: Prevents data leaks if VPN disconnects
3. **Use 2FA**: Enable two-factor authentication on your account
4. **Regular audits**: Periodically check for DNS leaks
5. **Secure configuration**: Use strong encryption settings

## Update

```bash
# Check for updates
vantisvpn check-update

# Update to latest version
vantisvpn update
```

---

You're all set! Enjoy secure, private internet access with VantisVPN. 🚀

For more advanced configuration and features, explore the full [documentation](Home).