---
slug: /architecture/overview
title: Architecture Overview
sidebar_position: 1
---

# Architecture Overview

VantisVPN is built on a modern, microservices architecture designed for security, performance, and scalability.

## System Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        A[Desktop Client]
        B[Mobile App]
        C[CLI Tool]
        D[Web Dashboard]
    end
    
    subgraph "API Gateway Layer"
        E[Load Balancer]
        F[API Gateway]
        G[Rate Limiter]
        H[Auth Service]
    end
    
    subgraph "Service Layer"
        I[Connection Service]
        J[Security Service]
        K[Server Service]
        L[Billing Service]
        M[Notification Service]
    end
    
    subgraph "Data Layer"
        N[(PostgreSQL)]
        O[(Redis)]
        P[(Elasticsearch)]
        Q[(S3 Storage)]
    end
    
    subgraph "Infrastructure"
        R[WireGuard Protocol]
        S[QUIC Protocol]
        T[Post-Quantum Crypto]
        U[CDN]
    end
    
    A --> E
    B --> E
    C --> E
    D --> E
    E --> F
    F --> G
    F --> H
    F --> I
    F --> J
    F --> K
    F --> L
    F --> M
    I --> N
    I --> O
    J --> T
    K --> R
    K --> S
    L --> N
    M --> P
    I --> Q
    F --> U
```

## Core Components

### 1. Client Layer

**Purpose**: User interfaces and client applications

**Components**:
- **Desktop Client**: Native apps for Windows, macOS, Linux
- **Mobile Apps**: iOS and Android applications
- **CLI Tool**: Command-line interface for power users
- **Web Dashboard**: Browser-based management interface

**Technologies**:
- Rust (Desktop)
- Swift/Kotlin (Mobile)
- React/Next.js (Web)

### 2. API Gateway Layer

**Purpose**: Request routing, authentication, and rate limiting

**Components**:
- **Load Balancer**: Distributes traffic across services
- **API Gateway**: Routes requests to appropriate services
- **Rate Limiter**: Enforces API usage limits
- **Auth Service**: Handles authentication and authorization

**Technologies**:
- NGINX
- Kong Gateway
- Redis (rate limiting)
- JWT (authentication)

### 3. Service Layer

**Purpose**: Business logic and core functionality

#### Connection Service
Manages VPN connections and routing
- Connection lifecycle management
- Server selection algorithm
- Real-time connection monitoring
- Fallback and failover logic

#### Security Service
Handles security features and cryptography
- Post-quantum cryptography (ML-KEM, ML-DSA)
- Key generation and rotation
- Certificate management
- Security audit logging

#### Server Service
Manages VPN server infrastructure
- Server provisioning and scaling
- Health monitoring
- Load balancing
- Geographic routing

#### Billing Service
Manages subscriptions and billing
- Payment processing
- Subscription management
- Usage tracking
- Invoice generation

#### Notification Service
Handles notifications and alerts
- Email notifications
- Push notifications
- Webhook delivery
- Real-time alerts

### 4. Data Layer

**Purpose**: Data persistence and caching

**Components**:
- **PostgreSQL**: Primary database
- **Redis**: Caching and session storage
- **Elasticsearch**: Search and analytics
- **S3 Storage**: File storage and backups

### 5. Infrastructure Layer

**Purpose**: Network protocols and security

**Components**:
- **WireGuard Protocol**: Fast, modern VPN protocol
- **QUIC Protocol**: UDP-based transport for better performance
- **Post-Quantum Crypto**: Quantum-resistant encryption
- **CDN**: Global content delivery network

## Data Flow

### Connection Establishment Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant G as API Gateway
    participant S as Security Service
    participant V as VPN Server
    participant D as Database
    
    C->>G: POST /connections
    G->>G: Authenticate API key
    G->>G: Check rate limits
    G->>S: Generate quantum keys
    S->>S: Create ML-KEM key pair
    S->>G: Return keys
    G->>D: Store connection details
    G->>C: Return connection config
    C->>V: WireGuard handshake
    V->>V: Verify keys
    V->>C: Handshake complete
    C->>D: Update connection status
```

### Security Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Security Service
    participant Q as PQC Module
    participant D as Database
    
    C->>S: Request encryption
    S->>Q: Generate ML-KEM keys
    Q->>Q: Create key pair
    Q->>S: Return keys
    S->>Q: Derive shared secret
    Q->>Q: KEM encapsulation
    Q->>S: Return ciphertext + secret
    S->>S: Derive session keys
    S->>D: Store audit log
    S->>C: Return public key + ciphertext
```

## Scalability

### Horizontal Scaling

- **Services**: Stateless services can scale horizontally
- **Database**: Read replicas for read-heavy workloads
- **Caching**: Redis cluster for distributed caching
- **CDN**: Global edge caching for static content

### Auto-scaling

- **CPU-based**: Scale services based on CPU usage
- **Memory-based**: Scale based on memory pressure
- **Request-based**: Scale based on request rate
- **Custom metrics**: Scale based on business metrics

## Security Architecture

### Defense in Depth

1. **Network Layer**: DDoS protection, rate limiting
2. **Application Layer**: Input validation, output encoding
3. **Data Layer**: Encryption at rest, access controls
4. **Cryptographic Layer**: Post-quantum algorithms, key rotation

### Zero Trust Principles

- **Never Trust, Always Verify**: Every request is authenticated
- **Least Privilege**: Minimal access by default
- **Assume Breach**: Designed with breach in mind
- **Continuous Monitoring**: Real-time security monitoring

## Performance Optimization

### Caching Strategy

- **API Responses**: Cache for 5 minutes
- **Server Lists**: Cache for 1 hour
- **User Sessions**: Cache in Redis
- **Static Content**: CDN caching

### Load Balancing

- **Round-robin**: Even distribution
- **Least connections**: Route to least busy server
- **Geographic**: Route to nearest server
- **Health-based**: Route to healthy servers only

## Monitoring & Observability

### Metrics

- **Connection metrics**: Active connections, success rate, latency
- **Server metrics**: CPU, memory, network I/O
- **Security metrics**: Auth failures, rate limit hits
- **Business metrics**: Active users, revenue, churn

### Logging

- **Structured logs**: JSON-formatted logs
- **Log levels**: DEBUG, INFO, WARN, ERROR
- **Log aggregation**: Centralized logging with ELK stack
- **Log retention**: 30 days for logs, 1 year for audit logs

### Tracing

- **Distributed tracing**: OpenTelemetry
- **Trace sampling**: 1% for production
- **Trace storage**: 7 days for traces

## Deployment Architecture

### Production Environment

```mermaid
graph LR
    subgraph "Region: US-East"
        A[Load Balancer]
        B[API Gateway]
        C[Services]
        D[Database]
    end
    
    subgraph "Region: EU-West"
        E[Load Balancer]
        F[API Gateway]
        G[Services]
        H[Database Replica]
    end
    
    subgraph "Region: AP-Southeast"
        I[Load Balancer]
        J[API Gateway]
        K[Services]
        L[Database Replica]
    end
    
    A --> B
    B --> C
    C --> D
    E --> F
    F --> G
    G --> H
    I --> J
    J --> K
    K --> L
    D -.-> H
    H -.-> L
```

### Multi-Region Deployment

- **US-East**: Primary region for US customers
- **EU-West**: European data center for GDPR compliance
- **AP-Southeast**: Asia-Pacific region
- **Database replication**: Multi-primary replication
- **Failover**: Automatic failover between regions

## Technology Stack

### Backend
- **Language**: Rust
- **Framework**: Tokio, Actix-web
- **Database**: PostgreSQL 15
- **Cache**: Redis 7
- **Message Queue**: RabbitMQ
- **Search**: Elasticsearch 8

### Frontend
- **Web**: Next.js 14, React 18
- **Mobile**: Swift (iOS), Kotlin (Android)
- **Desktop**: Tauri (Rust + Web)

### Infrastructure
- **Cloud**: AWS
- **Containers**: Docker, Kubernetes
- **CI/CD**: GitHub Actions
- **Monitoring**: Prometheus, Grafana
- **Logging**: ELK Stack

---

*Last Updated: March 6, 2026*