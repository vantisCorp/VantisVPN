# VANTISVPN Deployment Guide

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Deployment Architecture](#deployment-architecture)
4. [Server Deployment](#server-deployment)
5. [Client Deployment](#client-deployment)
6. [Router Deployment](#router-deployment)
7. [Docker Deployment](#docker-deployment)
8. [Kubernetes Deployment](#kubernetes-deployment)
9. [Monitoring and Logging](#monitoring-and-logging)
10. [Security Hardening](#security-hardening)
11. [Backup and Recovery](#backup-and-recovery)
12. [Scaling](#scaling)
13. [Troubleshooting](#troubleshooting)

---

## Overview

This guide covers deploying VANTISVPN in production environments, including server infrastructure, client applications, and hardware integration.

### Deployment Types

- **Server Deployment**: VPN server infrastructure
- **Client Deployment**: End-user applications
- **Router Deployment**: Router firmware
- **Container Deployment**: Docker and Kubernetes
- **Hardware Deployment**: Physical hardware integration

### Deployment Goals

- High availability (99.9% uptime)
- Scalability (10,000+ concurrent connections)
- Security (FIPS 140-3 compliant)
- Performance (< 50ms latency)
- Compliance (PCI DSS, SOC 2, HITRUST)

---

## Prerequisites

### System Requirements

#### Server Requirements

- **CPU**: 8 cores minimum, 16+ recommended
- **RAM**: 16 GB minimum, 32 GB+ recommended
- **Storage**: 100 GB SSD minimum
- **Network**: 10 Gbps preferred, 1 Gbps minimum
- **OS**: Ubuntu 22.04 LTS, Debian 12, or RHEL 9

#### Client Requirements

- **CPU**: 2 cores minimum
- **RAM**: 4 GB minimum
- **Storage**: 500 MB minimum
- **OS**: Windows 10+, macOS 10.15+, Linux (Ubuntu 20.04+)

#### Router Requirements

- **CPU**: 4 cores minimum
- **RAM**: 2 GB minimum
- **Storage**: 256 MB minimum
- **Network**: 1 Gbps minimum
- **Supported**: VantisRouter Pro/Standard

### Software Requirements

- **Rust**: 1.93.1 or later
- **Docker**: 24.0 or later
- **Kubernetes**: 1.28 or later (for K8s deployment)
- **Nginx**: 1.24 or later
- **PostgreSQL**: 15 or later (for user database)
- **Redis**: 7.0 or later (for caching)

### Network Requirements

- **Public IP**: Static IP recommended
- **DNS**: A records for VPN servers
- **Firewall**: Ports 443 (HTTPS), 51820 (WireGuard), 4433 (QUIC)
- **Load Balancer**: For high availability

---

## Deployment Architecture

### Recommended Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Load Balancer                        │
│                   (HAProxy/Nginx)                       │
└──────────────────┬──────────────────────────────────────┘
                   │
        ┌──────────┼──────────┐
        │          │          │
┌───────▼──────┐ ┌─▼────────┐ ┌─▼────────┐
│  VPN Server  │ │  VPN     │ │  VPN     │
│   Region 1   │ │  Server  │ │  Server  │
│  (Primary)   │ │ Region 2 │ │ Region 3 │
└──────┬───────┘ └───┬──────┘ └───┬──────┘
       │             │            │
       └─────────────┼────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
┌───────▼──────┐ ┌──▼──────┐ ┌──▼──────┐
│   Database   │ │  Redis  │ │ Monitor │
│  (PostgreSQL)│ │ (Cache) │ │ (Prom)  │
└──────────────┘ └─────────┘ └─────────┘
```

### Component Overview

- **Load Balancer**: Distributes traffic across VPN servers
- **VPN Servers**: Handle encrypted VPN connections
- **Database**: Stores user accounts and configuration
- **Redis**: Caches session data and configuration
- **Monitoring**: Tracks performance and availability

---

## Server Deployment

### Step 1: Prepare Server

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libpq-dev \
    redis-server \
    postgresql \
    postgresql-contrib \
    nginx \
    ufw

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone repository
git clone https://github.com/vantisCorp/VantisVPN.git
cd VantisVPN
```

### Step 2: Configure Firewall

```bash
# Configure UFW
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 443/tcp
sudo ufw allow 51820/udp
sudo ufw allow 4433/udp
sudo ufw enable
```

### Step 3: Build VANTISVPN

```bash
# Build core library
cd src/core
cargo build --release

# Build server components
cd ../server
cargo build --release

# Install binaries
sudo cp target/release/vantis-server /usr/local/bin/
sudo cp target/release/vantis-auth /usr/local/bin/
sudo chmod +x /usr/local/bin/vantis-*
```

### Step 4: Configure Database

```bash
# Create database
sudo -u postgres psql
CREATE DATABASE vantisvpn;
CREATE USER vantisvpn WITH PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE vantisvpn TO vantisvpn;
\q

# Run migrations
cd /workspace/VantisVPN/src/server/migrations
psql -U vantisvpn -d vantisvpn -f schema.sql
```

### Step 5: Configure VANTISVPN Server

```bash
# Create config directory
sudo mkdir -p /etc/vantisvpn
sudo mkdir -p /var/log/vantisvpn

# Create configuration file
sudo nano /etc/vantisvpn/server.toml
```

```toml
[server]
listen_address = "0.0.0.0:51820"
max_connections = 10000
log_level = "info"

[database]
host = "localhost"
port = 5432
database = "vantisvpn"
user = "vantisvpn"
password = "secure_password"
pool_size = 20

[redis]
host = "localhost"
port = 6379
pool_size = 10

[security]
enable_tls = true
tls_cert_path = "/etc/vantisvpn/cert.pem"
tls_key_path = "/etc/vantisvpn/key.pem"
enable_rate_limiting = true
max_requests_per_minute = 1000

[monitoring]
enable_prometheus = true
prometheus_port = 9090
enable_health_check = true
health_check_interval = 30
```

### Step 6: Generate TLS Certificates

```bash
# Create certificates directory
sudo mkdir -p /etc/vantisvpn/certs

# Generate self-signed certificate (for testing)
sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout /etc/vantisvpn/certs/key.pem \
    -out /etc/vantisvpn/certs/cert.pem

# For production, use Let's Encrypt
sudo apt install certbot
sudo certbot certonly --standalone -d vpn.example.com
sudo ln -s /etc/letsencrypt/live/vpn.example.com/fullchain.pem \
    /etc/vantisvpn/certs/cert.pem
sudo ln -s /etc/letsencrypt/live/vpn.example.com/privkey.pem \
    /etc/vantisvpn/certs/key.pem
```

### Step 7: Create Systemd Service

```bash
sudo nano /etc/systemd/system/vantisvpn.service
```

```ini
[Unit]
Description=VANTISVPN Server
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=vantisvpn
Group=vantisvpn
WorkingDirectory=/opt/vantisvpn
ExecStart=/usr/local/bin/vantis-server --config /etc/vantisvpn/server.toml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

```bash
# Create user
sudo useradd -r -s /bin/false vantisvpn
sudo chown -R vantisvpn:vantisvpn /etc/vantisvpn
sudo chown -R vantisvpn:vantisvpn /var/log/vantisvpn

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable vantisvpn
sudo systemctl start vantisvpn

# Check status
sudo systemctl status vantisvpn
```

### Step 8: Configure Load Balancer

```bash
sudo nano /etc/nginx/nginx.conf
```

```nginx
upstream vpn_servers {
    least_conn;
    server vpn1.example.com:51820;
    server vpn2.example.com:51820;
    server vpn3.example.com:51820;
}

server {
    listen 443 ssl http2;
    server_name vpn.example.com;

    ssl_certificate /etc/vantisvpn/certs/cert.pem;
    ssl_certificate_key /etc/vantisvpn/certs/key.pem;

    location / {
        proxy_pass http://vpn_servers;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

```bash
# Test and restart nginx
sudo nginx -t
sudo systemctl restart nginx
```

---

## Client Deployment

### Windows Deployment

#### MSI Installer

```bash
# Build Windows installer
cd src/ui
cargo build --release --target x86_64-pc-windows-msvc

# Create MSI using WiX
candle.exe -out vantisvpn.wixobj vantisvpn.wxs
light.exe -out vantisvpn.msi vantisvpn.wixobj
```

#### Deployment via GPO

1. Create GPO for software deployment
2. Copy MSI to network share
3. Assign GPO to target OUs
4. Clients will install automatically

#### Silent Installation

```cmd
msiexec /i vantisvpn.msi /quiet /norestart
```

### macOS Deployment

#### DMG Package

```bash
# Build macOS app
cd src/ui
cargo build --release --target x86_64-apple-darwin

# Create DMG
hdiutil create -volname "VANTISVPN" -srcfolder dist/ -ov -format UDZO vantisvpn.dmg
```

#### Deployment via MDM

1. Upload DMG to MDM server
2. Create deployment profile
3. Assign to devices
4. Devices will install automatically

#### Silent Installation

```bash
sudo installer -pkg /Volumes/VANTISVPN/vantisvpn.pkg -target /
```

### Linux Deployment

#### DEB Package (Ubuntu/Debian)

```bash
# Build DEB package
cd src/ui
cargo build --release
cargo deb

# Install
sudo dpkg -i vantisvpn_1.0.0_amd64.deb
```

#### RPM Package (Fedora/RHEL)

```bash
# Build RPM package
cd src/ui
cargo build --release
cargo rpm

# Install
sudo dnf install vantisvpn-1.0.0-1.x86_64.rpm
```

#### Deployment via Ansible

```yaml
# playbook.yml
---
- hosts: all
  become: yes
  tasks:
    - name: Install VANTISVPN
      apt:
        deb: /path/to/vantisvpn_1.0.0_amd64.deb
        state: present
```

---

## Router Deployment

### VantisRouter Pro Deployment

#### Flash Firmware

```bash
# Download firmware
wget https://download.vantisvpn.com/router/vantisrouter-pro-1.0.0.bin

# Access router via SSH
ssh admin@192.168.1.1

# Flash firmware
sysupgrade -v vantisrouter-pro-1.0.0.bin
```

#### Configure Router

```bash
# Access web interface
https://192.168.1.1

# Or configure via SSH
uci set vantisvpn.enabled='1'
uci set vantisvpn.server='vpn.example.com'
uci set vantisvpn.port='51820'
uci set vantisvpn.username='user@example.com'
uci set vantisvpn.password='password'
uci commit vantisvpn
/etc/init.d/vantisvpn restart
```

#### Verify Connection

```bash
# Check VPN status
vantisvpn status

# Check connection
ping -c 4 8.8.8.8

# Check IP
curl ifconfig.me
```

### Custom Router Deployment

#### OpenWrt

```bash
# Install WireGuard
opkg update
opkg install wireguard-tools

# Configure WireGuard
nano /etc/config/wireguard
```

```config
config wireguard 'vantisvpn'
    option enabled '1'
    option public_key 'server_public_key'
    option private_key 'client_private_key'
    option listen_port '51820'
    option fwmark '0x2000'

config wireguard_peer 'server'
    option public_key 'server_public_key'
    option endpoint_host 'vpn.example.com'
    option endpoint_port '51820'
    option route_allowed_ips '1'
    list allowed_ips '0.0.0.0/0'
    list allowed_ips '::/0'
    option persistent_keepalive '25'
```

```bash
# Start WireGuard
/etc/init.d/wireguard start
/etc/init.d/wireguard enable
```

---

## Docker Deployment

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  vantisvpn:
    image: vantisvpn/server:1.0.0
    container_name: vantisvpn
    ports:
      - "51820:51820/udp"
      - "4433:4433/udp"
      - "9090:9090"
    volumes:
      - ./config:/etc/vantisvpn
      - ./logs:/var/log/vantisvpn
      - ./certs:/etc/vantisvpn/certs
    environment:
      - RUST_LOG=info
      - DATABASE_URL=postgresql://vantisvpn:password@postgres:5432/vantisvpn
      - REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - redis
    restart: unless-stopped
    networks:
      - vantisvpn-network

  postgres:
    image: postgres:15-alpine
    container_name: vantisvpn-postgres
    environment:
      - POSTGRES_DB=vantisvpn
      - POSTGRES_USER=vantisvpn
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres-data:/var/lib/postgresql/data
    restart: unless-stopped
    networks:
      - vantisvpn-network

  redis:
    image: redis:7-alpine
    container_name: vantisvpn-redis
    volumes:
      - redis-data:/data
    restart: unless-stopped
    networks:
      - vantisvpn-network

  prometheus:
    image: prom/prometheus:latest
    container_name: vantisvpn-prometheus
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    restart: unless-stopped
    networks:
      - vantisvpn-network

  grafana:
    image: grafana/grafana:latest
    container_name: vantisvpn-grafana
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
    restart: unless-stopped
    networks:
      - vantisvpn-network

volumes:
  postgres-data:
  redis-data:
  prometheus-data:
  grafana-data:

networks:
  vantisvpn-network:
    driver: bridge
```

### Deploy with Docker Compose

```bash
# Create directories
mkdir -p config logs certs

# Create configuration
nano config/server.toml

# Start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f vantisvpn
```

### Dockerfile

```dockerfile
# Dockerfile
FROM rust:1.93.1-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:12-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/src/core/target/release/vantis-server /usr/local/bin/
COPY --from=builder /app/src/core/target/release/vantis-auth /usr/local/bin/

EXPOSE 51820/udp 4433/udp 9090

CMD ["vantis-server", "--config", "/etc/vantisvpn/server.toml"]
```

### Build and Push Image

```bash
# Build image
docker build -t vantisvpn/server:1.0.0 .

# Tag for registry
docker tag vantisvpn/server:1.0.0 registry.example.com/vantisvpn/server:1.0.0

# Push to registry
docker push registry.example.com/vantisvpn/server:1.0.0
```

---

## Kubernetes Deployment

### Deployment Manifest

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vantisvpn
  namespace: vantisvpn
spec:
  replicas: 3
  selector:
    matchLabels:
      app: vantisvpn
  template:
    metadata:
      labels:
        app: vantisvpn
    spec:
      containers:
      - name: vantisvpn
        image: vantisvpn/server:1.0.0
        ports:
        - containerPort: 51820
          protocol: UDP
          name: wireguard
        - containerPort: 4433
          protocol: UDP
          name: quic
        - containerPort: 9090
          protocol: TCP
          name: metrics
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: vantisvpn-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            configMapKeyRef:
              name: vantisvpn-config
              key: redis-url
        volumeMounts:
        - name: config
          mountPath: /etc/vantisvpn
        - name: certs
          mountPath: /etc/vantisvpn/certs
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 9090
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 9090
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: vantisvpn-config
      - name: certs
        secret:
          secretName: vantisvpn-certs
---
apiVersion: v1
kind: Service
metadata:
  name: vantisvpn
  namespace: vantisvpn
spec:
  selector:
    app: vantisvpn
  ports:
  - port: 51820
    targetPort: 51820
    protocol: UDP
    name: wireguard
  - port: 4433
    targetPort: 4433
    protocol: UDP
    name: quic
  - port: 9090
    targetPort: 9090
    protocol: TCP
    name: metrics
  type: LoadBalancer
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: vantisvpn-config
  namespace: vantisvpn
data:
  redis-url: "redis://redis:6379"
  server.toml: |
    [server]
    listen_address = "0.0.0.0:51820"
    max_connections = 10000
    log_level = "info"
---
apiVersion: v1
kind: Secret
metadata:
  name: vantisvpn-secrets
  namespace: vantisvpn
type: Opaque
stringData:
  database-url: "postgresql://vantisvpn:password@postgres:5432/vantisvpn"
---
apiVersion: v1
kind: Secret
metadata:
  name: vantisvpn-certs
  namespace: vantisvpn
type: kubernetes.io/tls
data:
  tls.crt: <base64-encoded-cert>
  tls.key: <base64-encoded-key>
```

### Deploy to Kubernetes

```bash
# Create namespace
kubectl create namespace vantisvpn

# Apply manifests
kubectl apply -f deployment.yaml

# Check status
kubectl get pods -n vantisvpn
kubectl get services -n vantisvpn

# View logs
kubectl logs -f deployment/vantisvpn -n vantisvpn
```

### Horizontal Pod Autoscaler

```yaml
# hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: vantisvpn-hpa
  namespace: vantisvpn
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: vantisvpn
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

---

## Monitoring and Logging

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'vantisvpn'
    static_configs:
      - targets: ['vantisvpn:9090']
    metrics_path: '/metrics'
```

### Grafana Dashboards

Import the following dashboards:

1. **VANTISVPN Overview**
   - Active connections
   - Bandwidth usage
   - Server health
   - Error rates

2. **Performance Metrics**
   - Latency
   - Throughput
   - CPU usage
   - Memory usage

3. **Security Metrics**
   - Failed authentication attempts
   - Rate limiting
   - DDoS attacks
   - Anomalies

### Log Aggregation

```bash
# Configure rsyslog
sudo nano /etc/rsyslog.d/vantisvpn.conf
```

```
if $programname == 'vantisvpn' then {
    action(type="omfile" file="/var/log/vantisvpn/vantisvpn.log")
    action(type="omfwd" target="logserver.example.com" protocol="tcp" port="514")
    stop
}
```

```bash
# Restart rsyslog
sudo systemctl restart rsyslog
```

### Alerting

```yaml
# alerts.yml
groups:
  - name: vantisvpn
    interval: 30s
    rules:
      - alert: HighConnectionCount
        expr: vantisvpn_connections > 9000
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High connection count on {{ $labels.instance }}"
          description: "Connection count is {{ $value }}"

      - alert: ServerDown
        expr: up{job="vantisvpn"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Server {{ $labels.instance }} is down"
          description: "Server has been down for more than 1 minute"

      - alert: HighLatency
        expr: vantisvpn_latency_ms > 100
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High latency on {{ $labels.instance }}"
          description: "Latency is {{ $value }}ms"
```

---

## Security Hardening

### System Hardening

```bash
# Disable root login
sudo sed -i 's/PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config

# Disable password authentication
sudo sed -i 's/PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config

# Enable fail2ban
sudo apt install fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban

# Configure fail2ban
sudo nano /etc/fail2ban/jail.local
```

```ini
[vantisvpn]
enabled = true
port = 51820,4433
filter = vantisvpn
logpath = /var/log/vantisvpn/vantisvpn.log
maxretry = 5
bantime = 3600
findtime = 600
```

### Network Hardening

```bash
# Configure iptables
sudo iptables -A INPUT -p udp --dport 51820 -m conntrack --ctstate NEW -m recent --set
sudo iptables -A INPUT -p udp --dport 51820 -m conntrack --ctstate NEW -m recent --update --seconds 60 --hitcount 10 -j DROP
sudo iptables -A INPUT -p udp --dport 51820 -j ACCEPT

# Save iptables rules
sudo iptables-save > /etc/iptables/rules.v4
```

### Application Security

```toml
# server.toml
[security]
enable_tls = true
tls_min_version = "1.3"
tls_cipher_suites = [
    "TLS_AES_256_GCM_SHA384",
    "TLS_CHACHA20_POLY1305_SHA256"
]
enable_rate_limiting = true
max_requests_per_minute = 1000
enable_ip_whitelist = false
enable_ip_blacklist = true
blacklist_file = "/etc/vantisvpn/blacklist.txt"
```

---

## Backup and Recovery

### Database Backup

```bash
# Create backup script
sudo nano /usr/local/bin/backup-vantisvpn.sh
```

```bash
#!/bin/bash
BACKUP_DIR="/backup/vantisvpn"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR

# Backup PostgreSQL
pg_dump -U vantisvpn vantisvpn | gzip > $BACKUP_DIR/postgres_$DATE.sql.gz

# Backup configuration
tar -czf $BACKUP_DIR/config_$DATE.tar.gz /etc/vantisvpn

# Backup certificates
tar -czf $BACKUP_DIR/certs_$DATE.tar.gz /etc/vantisvpn/certs

# Keep last 7 days
find $BACKUP_DIR -name "*.gz" -mtime +7 -delete
```

```bash
# Make executable
sudo chmod +x /usr/local/bin/backup-vantisvpn.sh

# Add to crontab
sudo crontab -e
```

```
0 2 * * * /usr/local/bin/backup-vantisvpn.sh
```

### Restore from Backup

```bash
# Restore PostgreSQL
gunzip < /backup/vantisvpn/postgres_20240101_020000.sql.gz | psql -U vantisvpn vantisvpn

# Restore configuration
tar -xzf /backup/vantisvpn/config_20240101_020000.tar.gz -C /

# Restore certificates
tar -xzf /backup/vantisvpn/certs_20240101_020000.tar.gz -C /

# Restart services
sudo systemctl restart vantisvpn
```

---

## Scaling

### Vertical Scaling

```bash
# Increase resources
sudo nano /etc/systemd/system/vantisvpn.service
```

```ini
[Service]
MemoryLimit=4G
CPUQuota=200%
```

### Horizontal Scaling

```bash
# Deploy additional servers
# Update load balancer configuration
# Test failover
```

### Database Scaling

```bash
# Enable connection pooling
# Add read replicas
# Implement sharding
```

---

## Troubleshooting

### Connection Issues

```bash
# Check service status
sudo systemctl status vantisvpn

# View logs
sudo journalctl -u vantisvpn -f

# Check network connectivity
netstat -tulpn | grep 51820

# Test WireGuard connection
wg show
```

### Performance Issues

```bash
# Check CPU usage
top

# Check memory usage
free -h

# Check disk usage
df -h

# Check network bandwidth
iftop
```

### Database Issues

```bash
# Check PostgreSQL status
sudo systemctl status postgresql

# Check database connections
sudo -u postgres psql -c "SELECT count(*) FROM pg_stat_activity;"

# Check slow queries
sudo -u postgres psql -c "SELECT * FROM pg_stat_statements ORDER BY total_time DESC LIMIT 10;"
```

---

## Appendix

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | - |
| `REDIS_URL` | Redis connection string | - |
| `RUST_LOG` | Log level | `info` |
| `SERVER_PORT` | Server port | `51820` |
| `MAX_CONNECTIONS` | Max concurrent connections | `10000` |

### Health Check Endpoints

- `GET /health` - Health check
- `GET /ready` - Readiness check
- `GET /metrics` - Prometheus metrics

### Support

- **Documentation**: [docs.vantisvpn.com](https://docs.vantisvpn.com)
- **Email**: support@vantisvpn.com
- **GitHub**: [github.com/vantisCorp/VantisVPN](https://github.com/vantisCorp/VantisVPN)

---

*Last Updated: 2024*
*Version: 1.0.0*