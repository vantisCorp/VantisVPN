# Installation Guide

This guide will help you install VantisVPN on your system.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Linux Installation](#linux-installation)
- [macOS Installation](#macos-installation)
- [Windows Installation](#windows-installation)
- [Docker Installation](#docker-installation)
- [Building from Source](#building-from-source)
- [Post-Installation](#post-installation)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before installing VantisVPN, ensure you have:

- Root/administrator access
- Compatible operating system (see below)
- At least 50 MB of free disk space
- Network connectivity

## Linux Installation

### Ubuntu/Debian

```bash
# Add the VantisVPN repository
curl -fsSL https://apt.vantisvpn.com/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/vantisvpn.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/vantisvpn.gpg] https://apt.vantisvpn.com stable main" | sudo tee /etc/apt/sources.list.d/vantisvpn.list > /dev/null

# Update package list and install
sudo apt update
sudo apt install vantisvpn

# Enable and start the service
sudo systemctl enable vantisvpn
sudo systemctl start vantisvpn
```

### Fedora/RHEL/CentOS

```bash
# Add the VantisVPN repository
sudo dnf config-manager --add-repo https://yum.vantisvpn.com/vantisvpn.repo

# Install VantisVPN
sudo dnf install vantisvpn

# Enable and start the service
sudo systemctl enable vantisvpn
sudo systemctl start vantisvpn
```

### Arch Linux

```bash
# Install from AUR
yay -S vantisvpn
# or
paru -S vantisvpn

# Enable and start the service
sudo systemctl enable vantisvpn
sudo systemctl start vantisvpn
```

### Manual Installation

```bash
# Download the latest release
wget https://github.com/vantisCorp/VantisVPN/releases/latest/download/vantisvpn-linux-amd64.tar.gz

# Extract and install
tar -xzf vantisvpn-linux-amd64.tar.gz
sudo cp vantisvpn /usr/local/bin/
sudo chmod +x /usr/local/bin/vantisvpn

# Create systemd service
sudo tee /etc/systemd/system/vantisvpn.service > /dev/null <<EOF
[Unit]
Description=VantisVPN Service
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/vantisvpn daemon
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable vantisvpn
sudo systemctl start vantisvpn
```

## macOS Installation

### Using Homebrew

```bash
# Tap the VantisVPN repository
brew tap vantisvpn/tap

# Install VantisVPN
brew install vantisvpn

# Start the service
brew services start vantisvpn
```

### Manual Installation

```bash
# Download the latest release
curl -L -o vantisvpn-macos-amd64.zip https://github.com/vantisCorp/VantisVPN/releases/latest/download/vantisvpn-macos-amd64.zip

# Extract and install
unzip vantisvpn-macos-amd64.zip
sudo cp vantisvpn /usr/local/bin/
sudo chmod +x /usr/local/bin/vantisvpn

# Verify installation
vantisvpn --version
```

## Windows Installation

### Using Winget

```powershell
# Install using winget
winget install VantisVPN.VantisVPN

# Start the service (optional)
sc.exe start VantisVPN
```

### Using Chocolatey

```powershell
# Install using chocolatey
choco install vantisvpn

# Start the service
Start-Service -Name VantisVPN
```

### Manual Installation

1. Download the latest release from [GitHub Releases](https://github.com/vantisCorp/VantisVPN/releases/latest)
2. Extract the ZIP file
3. Run `vantisvpn-installer.exe` as Administrator
4. Follow the installation wizard

## Docker Installation

### Using Docker Hub

```bash
# Pull the latest image
docker pull vantisvpn/vantisvpn:latest

# Run VantisVPN
docker run -d \
  --name vantisvpn \
  --cap-add=NET_ADMIN \
  --device=/dev/net/tun \
  --restart unless-stopped \
  vantisvpn/vantisvpn:latest

# View logs
docker logs -f vantisvpn
```

### Using Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  vantisvpn:
    image: vantisvpn/vantisvpn:latest
    container_name: vantisvpn
    cap_add:
      - NET_ADMIN
    devices:
      - /dev/net/tun
    restart: unless-stopped
    network_mode: host
    environment:
      - VANTISVPN_CONFIG=/etc/vantisvpn/config.yaml
    volumes:
      - ./config:/etc/vantisvpn
```

```bash
# Start the container
docker-compose up -d

# View logs
docker-compose logs -f
```

## Building from Source

### Prerequisites

- Rust 1.82 or later
- Git
- OpenSSL development libraries

### Build Steps

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisVPN.git
cd VantisVPN

# Build the project
cargo build --release

# The binary will be in target/release/vantisvpn
sudo cp target/release/vantisvpn /usr/local/bin/
```

### Build with Features

```bash
# Build with all features
cargo build --release --all-features

# Build specific features
cargo build --release --features "quic websocket"
```

## Post-Installation

### Verification

```bash
# Check version
vantisvpn --version

# Check status
vantisvpn status

# List available servers
vantisvpn list-servers
```

### Configuration

```bash
# Generate default configuration
vantisvpn config init

# Edit configuration
vantisvpn config edit

# Validate configuration
vantisvpn config validate
```

### Connection

```bash
# Connect to a server
vantisvpn connect us-west-1

# Disconnect
vantisvpn disconnect

# Show connection status
vantisvpn status
```

## Troubleshooting

### Common Issues

#### Service won't start

```bash
# Check service logs
sudo journalctl -u vantisvpn -f

# Check if port is in use
sudo lsof -i :443

# Check permissions
ls -la /usr/local/bin/vantisvpn
```

#### Connection fails

```bash
# Enable debug logging
vantisvpn --log-level debug connect us-west-1

# Check firewall rules
sudo iptables -L -n -v

# Test connectivity
ping -c 3 vantisvpn.com
```

#### DNS issues

```bash
# Check DNS configuration
cat /etc/resolv.conf

# Flush DNS cache
sudo systemd-resolve --flush-caches
```

### Getting Help

If you encounter issues not covered here:

- Check the [Troubleshooting](Troubleshooting) wiki page
- Search [GitHub Issues](https://github.com/vantisCorp/VantisVPN/issues)
- Ask on [Discord](https://discord.gg/A5MzwsRj7D)
- Email support@vantisvpn.com

## Next Steps

- [Quick Start Guide](Quick-Start)
- [Configuration Guide](Configuration)
- [Troubleshooting](Troubleshooting)