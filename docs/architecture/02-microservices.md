# VANTISVPN - Microservices Architecture

## Overview

VANTISVPN is built on a microservices architecture that provides:
- **Isolation**: Each service runs in its own container
- **Resilience**: Failure of one service doesn't affect others
- **Scalability**: Services can be scaled independently
- **Maintainability**: Clear separation of concerns

## Service Components

### 1. Core Service (Rust Library)
- **Purpose**: Shared cryptographic and networking logic
- **Language**: Rust
- **Responsibilities**:
  - Cryptographic operations (PQC, classical)
  - Network protocol implementation
  - Tunnel management
  - Key management

### 2. Network Service
- **Purpose**: Handles all network communication
- **Language**: Rust
- **Responsibilities**:
  - QUIC transport layer
  - WireGuard-like protocol
  - Packet routing
  - Connection management

### 3. Crypto Service
- **Purpose**: Cryptographic operations
- **Language**: Rust
- **Responsibilities**:
  - Key generation and management
  - Encryption/decryption
  - Post-quantum cryptography
  - Secure memory handling

### 4. Tunnel Service
- **Purpose**: VPN tunnel lifecycle management
- **Language**: Rust
- **Responsibilities**:
  - Tunnel creation/teardown
  - State management
  - Statistics tracking
  - Reconnection logic

### 5. UI Service (Tauri)
- **Purpose**: User interface
- **Language**: Rust + JavaScript/TypeScript
- **Responsibilities**:
  - User interaction
  - Settings management
  - Connection visualization
  - Status display

### 6. DNS Service
- **Purpose**: DNS resolution and filtering
- **Language**: Rust
- **Responsibilities**:
  - DNS over HTTPS (DoH)
  - Ad blocking
  - Phishing protection
  - DNS caching

### 7. Config Service
- **Purpose**: Configuration management
- **Language**: Rust
- **Responsibilities**:
  - Settings persistence
  - Profile management
  - Configuration validation

## Communication Patterns

### Service Communication
```
┌─────────────────────────────────────────────────┐
│              Service Mesh Layer                  │
├─────────────────────────────────────────────────┤
│                                                   │
│  ┌──────────────┐    ┌──────────────┐           │
│  │   UI App     │◄──►│  Core        │           │
│  │   Service    │    │  Service     │           │
│  └──────────────┘    └──────┬───────┘           │
│                            │                     │
│              ┌─────────────┼─────────────┐       │
│              ▼             ▼             ▼       │
│      ┌──────────────┐ ┌──────────┐ ┌────────┐  │
│      │   Network    │ │  Crypto  │ │ Tunnel │  │
│      │   Service    │ │ Service  │ │Service │  │
│      └──────────────┘ └──────────┘ └────────┘  │
│                                                   │
└─────────────────────────────────────────────────┘
```

### Message Formats

Services communicate using:
- **gRPC**: Internal service communication (high performance)
- **JSON over HTTP/REST**: External APIs
- **Message Queues**: Asynchronous events (RabbitMQ/Redis)

## Containerization

Each service runs in its own container:
```dockerfile
# Example: Core Service Container
FROM rust:1.75-slim
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY src ./src
CMD ["./target/release/vantis-core"]
```

## Security Isolation

### Network Isolation
- Each service in separate network namespace
- Service-to-service communication encrypted
- No direct internet access for sensitive services

### Process Isolation
- Separate processes for each service
- Resource limits (CPU, memory)
- Seccomp filters for syscall restrictions

### Memory Isolation
- ASLR (Address Space Layout Randomization)
- Memory protection (NX, DEP)
- Secure memory handling for sensitive data

## Service Discovery

Services discover each other through:
- **Consul**: Service registration and health checks
- **DNS-based**: DNS SRV records
- **Environment variables**: For static configurations

## Deployment Strategy

### Development
- Docker Compose for local development
- Hot reload for UI service
- Shared volume for logs

### Production
- Kubernetes orchestration
- Horizontal Pod Autoscaling
- Rolling updates with zero downtime
- Blue-green deployments

## Monitoring

### Metrics
- Prometheus for metrics collection
- Grafana for visualization
- Custom metrics for each service:
  - Connection counts
  - Throughput (bytes/sec)
  - Latency
  - Error rates

### Logging
- Structured JSON logging
- Centralized log aggregation (ELK stack)
- Log rotation and retention policies
- Sensitive data redaction

### Tracing
- Distributed tracing with Jaeger
- Request correlation IDs
- Performance bottleneck identification

## Failover and Recovery

### Service Health
- Health check endpoints (`/health`)
- Dependency health checks
- Graceful degradation

### Automatic Recovery
- Kubernetes restart policies
- Circuit breakers for cascading failure prevention
- Retry logic with exponential backoff

## Future Enhancements

- **Edge Services**: Deploy services closer to users
- **Service Mesh**: Istio for advanced traffic management
- **Serverless Components**: Lambda/Cloud Functions for burst traffic
- **Multi-Region Deployment**: Geographic distribution