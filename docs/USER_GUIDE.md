# VANTISVPN User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Basic Usage](#basic-usage)
5. [Advanced Features](#advanced-features)
6. [Security Features](#security-features)
7. [Privacy Features](#privacy-features)
8. [Hardware Integration](#hardware-integration)
9. [Troubleshooting](#troubleshooting)
10. [FAQ](#faq)

---

## Introduction

### What is VANTISVPN?

VANTISVPN is a next-generation secure VPN system with military-grade security and post-quantum cryptography. It provides:

- **Military-Grade Security**: Post-quantum cryptography ready
- **Privacy by Design**: No logs, zero-knowledge architecture
- **Advanced Features**: Multi-hop routing, stealth mode, split tunneling
- **Hardware Integration**: Router OS, YubiKey 2FA, secure USB OS
- **Compliance**: PCI DSS, SOC 2, HITRUST certified

### Why Choose VANTISVPN?

- **Quantum-Resistant**: Ready for future quantum computers
- **IPv6 Native**: Full support for next-generation internet
- **Zero Logs**: We don't track your online activity
- **Open Source**: Fully auditable code
- **Fast**: QUIC/HTTP/3 for optimal performance

---

## Installation

### System Requirements

- **Operating System**: Windows 10+, macOS 10.15+, Linux (Ubuntu 20.04+)
- **RAM**: 4 GB minimum, 8 GB recommended
- **Disk Space**: 500 MB for installation
- **Network**: Internet connection for VPN servers

### Installing on Windows

1. Download the installer from [vantisvpn.com/download](https://vantisvpn.com/download)
2. Run `VantisVPN-Setup.exe`
3. Follow the installation wizard
4. Launch VANTISVPN from the Start menu

### Installing on macOS

1. Download the DMG file from [vantisvpn.com/download](https://vantisvpn.com/download)
2. Open `VantisVPN.dmg`
3. Drag VANTISVPN to Applications folder
4. Launch from Applications

### Installing on Linux

#### Ubuntu/Debian

```bash
# Download the package
wget https://download.vantisvpn.com/vantisvpn_1.0.0_amd64.deb

# Install
sudo dpkg -i vantisvpn_1.0.0_amd64.deb

# Fix dependencies if needed
sudo apt-get install -f
```

#### Fedora/RHEL

```bash
# Download the package
wget https://download.vantisvpn.com/vantisvpn-1.0.0-1.x86_64.rpm

# Install
sudo dnf install vantisvpn-1.0.0-1.x86_64.rpm
```

#### Arch Linux

```bash
# Install from AUR
yay -S vantisvpn
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisVPN.git
cd VantisVPN

# Build
cd src/core
cargo build --release

# Install
sudo cp target/release/vantisvpn /usr/local/bin/
```

---

## Quick Start

### First-Time Setup

1. **Launch VANTISVPN**
   - Windows: Start Menu → VANTISVPN
   - macOS: Applications → VANTISVPN
   - Linux: `vantisvpn` command

2. **Create Account**
   - Click "Create Account"
   - Enter email and password
   - Verify your email

3. **Choose a Server**
   - Select a server location from the list
   - Click "Connect"

4. **You're Connected!**
   - Your connection is now secure
   - Your IP is hidden
   - Your traffic is encrypted

### Connecting to VPN

1. Open VANTISVPN
2. Select a server from the list
3. Click the "Connect" button
4. Wait for connection (usually < 5 seconds)
5. You're now protected!

### Disconnecting

1. Click the "Disconnect" button
2. Your connection will be terminated
3. Your internet traffic will no longer go through VPN

---

## Basic Usage

### Server Selection

#### Manual Server Selection

1. Click on the server list
2. Browse by country or region
3. Select a server
4. Click "Connect"

#### Auto-Connect

1. Go to Settings → Connection
2. Enable "Auto-connect on startup"
3. Choose a default server
4. VANTISVPN will connect automatically on launch

#### Fastest Server

1. Click "Fastest Server" button
2. VANTISVPN will test all servers
3. Connect to the fastest available server

### Connection Modes

#### Standard Mode

- Single-hop connection
- Best performance
- Recommended for most users

#### Multi-Hop Mode

- 2-7 hop connection
- Enhanced privacy
- Slower but more secure

To enable:
1. Go to Settings → Connection
2. Select "Multi-Hop"
3. Choose number of hops (2-7)
4. Click "Connect"

#### Stealth Mode

- Traffic obfuscation
- Bypasses censorship
- Looks like normal HTTPS

To enable:
1. Go to Settings → Connection
2. Enable "Stealth Mode"
3. Click "Connect"

---

## Advanced Features

### Split Tunneling

Split tunneling allows you to choose which apps use VPN and which use your regular connection.

#### Setting Up Split Tunneling

1. Go to Settings → Split Tunneling
2. Enable "Split Tunneling"
3. Choose mode:
   - **Exclude Mode**: All apps use VPN except selected ones
   - **Include Mode**: Only selected apps use VPN

#### Adding Apps

1. Click "Add Application"
2. Select an application
3. Choose action (Include/Exclude)
4. Click "Save"

#### Example: Exclude Streaming Services

1. Enable Split Tunneling
2. Select "Exclude Mode"
3. Add Netflix, Hulu, Disney+
4. These services will use your regular connection

### Kill Switch

The kill switch protects your privacy by blocking internet if VPN disconnects.

#### Enabling Kill Switch

1. Go to Settings → Security
2. Enable "Kill Switch"
3. Choose mode:
   - **Soft**: Allow LAN connections
   - **Strict**: Block all connections

#### How It Works

- If VPN disconnects, internet is blocked
- Prevents data leaks
- Protects your privacy

### DNS Leak Protection

VANTISVPN automatically protects against DNS leaks.

#### Verifying DNS Protection

1. Connect to VPN
2. Visit [dnsleaktest.com](https://dnsleaktest.com)
3. You should see VPN DNS servers only

### Custom DNS

Use your own DNS servers with VPN.

#### Setting Custom DNS

1. Go to Settings → Network
2. Enable "Custom DNS"
3. Enter DNS servers:
   - Cloudflare: `1.1.1.1`, `1.0.0.1`
   - Google: `8.8.8.8`, `8.8.4.4`
   - OpenDNS: `208.67.222.222`, `208.67.220.220`
4. Click "Save"

---

## Security Features

### Two-Factor Authentication (2FA)

Add an extra layer of security with YubiKey or authenticator app.

#### Setting Up YubiKey 2FA

1. Go to Settings → Security
2. Click "Enable 2FA"
3. Choose "YubiKey"
4. Insert your YubiKey
5. Touch the button when prompted
6. 2FA is now enabled!

#### Setting Up Authenticator App

1. Go to Settings → Security
2. Click "Enable 2FA"
3. Choose "Authenticator App"
4. Scan QR code with your app
5. Enter verification code
6. 2FA is now enabled!

### Quantum Vault

Secure password manager built into VANTISVPN.

#### Creating Vault

1. Go to Settings → Quantum Vault
2. Click "Create Vault"
3. Set master password
4. Generate backup codes
5. Vault is ready!

#### Adding Passwords

1. Open Quantum Vault
2. Click "Add Entry"
3. Enter:
   - Username
   - Password
   - Website URL
4. Click "Save"

#### Auto-Fill Passwords

1. Visit a website
2. Click VANTISVPN icon in browser
3. Select password
4. Auto-fill!

### NetShield AI

AI-powered DNS blocking for malware, phishing, and tracking.

#### Enabling NetShield

1. Go to Settings → Security
2. Enable "NetShield AI"
3. Choose protection level:
   - **Basic**: Block malware
   - **Standard**: Block malware + phishing
   - **Advanced**: Block malware + phishing + tracking

#### Viewing Blocked Requests

1. Go to Settings → Security → NetShield
2. View blocked requests log
3. Whitelist domains if needed

### Remote Browser Isolation

Browse the web in an isolated environment for maximum security.

#### Using Remote Browser

1. Go to Tools → Remote Browser
2. Enter URL
3. Click "Open"
4. Browse in isolated environment

#### Benefits

- Malware can't infect your device
- Tracking cookies are blocked
- Complete isolation from your system

---

## Privacy Features

### Zero-Knowledge Login

Login without revealing your password to our servers.

#### How It Works

1. Your password never leaves your device
2. We use zero-knowledge proofs
3. We can't see your password
4. Maximum privacy

### Avantis ID

Anonymous digital identity for online services.

#### Creating Avantis ID

1. Go to Settings → Privacy → Avantis ID
2. Click "Create Identity"
3. Choose identity type:
   - **Anonymous**: No personal info
   - **Pseudonymous**: Minimal info
   - **Verified**: With verification
4. Identity created!

#### Using Avantis ID

1. Visit a supported service
2. Choose "Login with Avantis ID"
3. Approve on your device
4. Logged in!

### IP Rotator

Automatically rotate your IP address for enhanced privacy.

#### Enabling IP Rotator

1. Go to Settings → Privacy → IP Rotator
2. Enable "IP Rotator"
3. Set rotation interval:
   - Every 5 minutes
   - Every 15 minutes
   - Every 30 minutes
   - Every hour
4. Click "Save"

#### Manual Rotation

1. Click "Rotate IP" button
2. Your IP changes immediately
3. New location, new identity

### Anonymous Payments

Pay for VANTISVPN with cryptocurrency or cash.

#### Paying with Monero

1. Go to Account → Billing
2. Click "Pay with Monero"
3. Send XMR to provided address
4. Payment confirmed!

#### Paying with Lightning

1. Go to Account → Billing
2. Click "Pay with Lightning"
3. Scan QR code
4. Payment confirmed instantly!

#### Paying with Cash

1. Go to Account → Billing
2. Click "Pay with Cash"
3. Follow instructions
4. Mail payment
5. Account credited!

### GDPR Rights

Exercise your GDPR rights easily.

#### Request Data Access

1. Go to Settings → Privacy → GDPR
2. Click "Request My Data"
3. Receive data export within 30 days

#### Request Data Deletion

1. Go to Settings → Privacy → GDPR
2. Click "Delete My Data"
3. Confirm deletion
4. All data deleted within 30 days

#### Data Portability

1. Go to Settings → Privacy → GDPR
2. Click "Export My Data"
3. Download your data
4. Use with other services

---

## Hardware Integration

### Vantis Router OS

Install VANTISVPN on your router for whole-home protection.

#### Supported Routers

- VantisRouter Pro (recommended)
- VantisRouter Standard
- Custom builds (see documentation)

#### Installation

1. Download firmware from [vantisvpn.com/router](https://vantisvpn.com/router)
2. Access router admin panel
3. Go to Firmware Update
4. Upload firmware file
5. Wait for installation
6. Router reboots

#### Configuration

1. Access router at `192.168.1.1`
2. Login with admin credentials
3. Go to VPN Settings
4. Enter your VANTISVPN credentials
5. Connect!

#### Benefits

- All devices protected
- No per-device limits
- Centralized management
- Advanced features (QoS, firewall)

### YubiKey Integration

Use YubiKey for secure authentication.

#### Supported YubiKey Models

- YubiKey 5 Series
- YubiKey 4 Series
- YubiKey NEO
- Security Key by Yubico

#### Setting Up YubiKey

1. Insert YubiKey
2. Go to Settings → Security → YubiKey
3. Click "Register YubiKey"
4. Touch button when prompted
5. YubiKey registered!

#### Using YubiKey

1. When prompted for 2FA
2. Insert YubiKey
3. Touch button
4. Authenticated!

### Vantis OS

Secure, portable operating system on USB.

#### Creating Vantis OS USB

1. Download Vantis OS image
2. Download balenaEtcher
3. Flash image to USB drive
4. Boot from USB

#### Features

- Live mode: No persistence
- Persistent mode: Save settings
- Encrypted mode: Full encryption
- Tor integration
- Pre-installed security tools

#### Booting Vantis OS

1. Insert USB drive
2. Restart computer
3. Press boot menu key (F12, F2, Del)
4. Select USB drive
5. Boot into Vantis OS

---

## Troubleshooting

### Connection Issues

#### Can't Connect to VPN

**Symptoms**: Connection fails or times out

**Solutions**:
1. Check internet connection
2. Try different server
3. Disable firewall temporarily
4. Restart VANTISVPN
5. Reinstall VANTISVPN

#### Slow Connection Speed

**Symptoms**: VPN connection is slow

**Solutions**:
1. Try different server
2. Switch to Standard mode (from Multi-Hop)
3. Disable Stealth mode
4. Check your internet speed
5. Use wired connection instead of WiFi

#### Connection Drops Frequently

**Symptoms**: VPN disconnects randomly

**Solutions**:
1. Enable Kill Switch
2. Check network stability
3. Try different protocol
4. Update VANTISVPN
5. Contact support

### Application Issues

#### App Won't Launch

**Symptoms**: VANTISVPN won't start

**Solutions**:
1. Restart computer
2. Reinstall VANTISVPN
3. Check system requirements
4. Run as administrator
5. Check antivirus settings

#### App Crashes

**Symptoms**: VANTISVPN crashes unexpectedly

**Solutions**:
1. Update VANTISVPN
2. Clear app cache
3. Reinstall VANTISVPN
4. Submit crash report
5. Contact support

### Authentication Issues

#### Can't Login

**Symptoms**: Login fails

**Solutions**:
1. Check credentials
2. Reset password
3. Disable 2FA temporarily
4. Check account status
5. Contact support

#### 2FA Not Working

**Symptoms**: Can't complete 2FA

**Solutions**:
1. Check time sync on device
2. Use backup codes
3. Re-enable 2FA
4. Try different 2FA method
5. Contact support

### Performance Issues

#### High CPU Usage

**Symptoms**: VANTISVPN uses too much CPU

**Solutions**:
1. Disable Multi-Hop mode
2. Disable Stealth mode
3. Close other apps
4. Update VANTISVPN
5. Check for malware

#### High Memory Usage

**Symptoms**: VANTISVPN uses too much RAM

**Solutions**:
1. Restart VANTISVPN
2. Clear app cache
3. Update VANTISVPN
4. Close other apps
5. Check system resources

---

## FAQ

### General Questions

**Q: Is VANTISVPN free?**

A: VANTISVPN offers a free tier with limited features. Premium plans start at $9.99/month.

**Q: Can I use VANTISVPN on multiple devices?**

A: Yes! Premium plans support up to 10 devices simultaneously.

**Q: Does VANTISVPN keep logs?**

A: No. We have a strict no-logs policy verified by Big Four auditors.

**Q: Is VANTISVPN legal?**

A: Yes, VPNs are legal in most countries. Check your local laws.

**Q: Can VANTISVPN be detected?**

A: With Stealth mode enabled, VANTISVPN traffic looks like normal HTTPS.

### Technical Questions

**Q: What protocols does VANTISVPN support?**

A: We support WireGuard, QUIC/HTTP/3, and OpenVPN.

**Q: Does VANTISVPN support IPv6?**

A: Yes, VANTISVPN has full IPv6 native support.

**Q: What is post-quantum cryptography?**

A: Cryptography that resists attacks from quantum computers. We use ML-KEM and ML-DSA.

**Q: What is multi-hop routing?**

A: Routing your traffic through multiple VPN servers for enhanced privacy.

**Q: What is stealth mode?**

A: Traffic obfuscation that makes VPN traffic look like normal HTTPS.

### Security Questions

**Q: Is VANTISVPN secure?**

A: Yes, we use military-grade encryption and have multiple security certifications.

**Q: Can VANTISVPN be hacked?**

A: No system is 100% unhackable, but we use best practices and regular audits.

**Q: What happens if VPN disconnects?**

A: The kill switch blocks all internet traffic to prevent data leaks.

**Q: Does VANTISVPN protect against malware?**

A: Yes, NetShield AI blocks malware, phishing, and tracking.

**Q: Is my data encrypted?**

A: Yes, all traffic is encrypted with AES-256-GCM or ChaCha20-Poly1305.

### Privacy Questions

**Q: Can VANTISVPN see my traffic?**

A: No, we use zero-knowledge architecture and can't see your traffic.

**Q: Does VANTISVPN sell my data?**

A: No, we never sell or share your data.

**Q: Can VANTISVPN be forced to hand over data?**

A: We have no data to hand over due to our no-logs policy.

**Q: What is zero-knowledge login?**

A: Login method where your password never leaves your device.

**Q: Can I be tracked while using VANTISVPN?**

A: With proper configuration, tracking is extremely difficult.

### Billing Questions

**Q: What payment methods do you accept?**

A: Credit cards, PayPal, Monero, Lightning, and cash.

**Q: Do you offer refunds?**

A: Yes, we offer a 30-day money-back guarantee.

**Q: Can I cancel anytime?**

A: Yes, you can cancel anytime with no penalties.

**Q: Do you offer discounts?**

A: Yes, we offer discounts for annual plans and students.

**Q: Is my payment information secure?**

A: Yes, we use PCI DSS compliant payment processing.

---

## Support

### Getting Help

- **Knowledge Base**: [support.vantisvpn.com](https://support.vantisvpn.com)
- **Email**: support@vantisvpn.com
- **Live Chat**: Available 24/7
- **Phone**: +1-800-VANTIS-VPN

### Reporting Issues

1. Go to [github.com/vantisCorp/VantisVPN/issues](https://github.com/vantisCorp/VantisVPN/issues)
2. Click "New Issue"
3. Describe the problem
4. Include system information
5. Submit

### Security Issues

For security vulnerabilities, email: security@vantisvpn.com

Please include:
- Description of vulnerability
- Steps to reproduce
- Proof of concept
- Suggested fix

---

## Appendix

### Keyboard Shortcuts

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Connect/Disconnect | Ctrl+K | Cmd+K |
| Open Settings | Ctrl+, | Cmd+, |
| Quit | Ctrl+Q | Cmd+Q |
| Show/Hide | Ctrl+H | Cmd+H |

### Command Line Interface

```bash
# Connect to VPN
vantisvpn connect --server us-east

# Disconnect
vantisvpn disconnect

# Show status
vantisvpn status

# List servers
vantisvpn servers

# Show logs
vantisvpn logs

# Enable kill switch
vantisvpn killswitch enable

# Disable kill switch
vantisvpn killswitch disable
```

### Configuration File

Location:
- Windows: `%APPDATA%\VantisVPN\config.json`
- macOS: `~/Library/Application Support/VantisVPN/config.json`
- Linux: `~/.config/vantisvpn/config.json`

Example:
```json
{
  "auto_connect": true,
  "default_server": "us-east",
  "kill_switch": true,
  "stealth_mode": false,
  "multi_hop": false,
  "split_tunneling": {
    "enabled": true,
    "mode": "exclude",
    "apps": ["netflix.exe", "hulu.exe"]
  },
  "dns": {
    "custom": true,
    "servers": ["1.1.1.1", "1.0.0.1"]
  }
}
```

---

*Last Updated: 2024*
*Version: 1.0.0*