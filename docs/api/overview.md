---
slug: /api/overview
title: API Overview
sidebar_position: 1
---

# API Overview

VantisVPN provides a comprehensive RESTful API for programmatic control and automation of VPN operations.

## Base URL

```
https://api.vantisvpn.com/v1
```

## Authentication

All API requests require authentication using an API key in the header:

```bash
Authorization: Bearer YOUR_API_KEY
```

### Obtaining an API Key

1. Log in to your VantisVPN dashboard
2. Navigate to Settings → API Keys
3. Click "Generate New Key"
4. Copy and securely store your API key

## Rate Limiting

- **Free Tier**: 1,000 requests/hour
- **Pro Tier**: 10,000 requests/hour
- **Enterprise**: Unlimited

Rate limit headers are included in all responses:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1609459200
```

## Response Format

All API responses return JSON:

```json
{
  "success": true,
  "data": { ... },
  "message": "Operation completed successfully",
  "timestamp": "2026-03-06T12:00:00Z"
}
```

## Error Handling

### Error Response Format

```json
{
  "success": false,
  "error": {
    "code": "INVALID_API_KEY",
    "message": "The provided API key is invalid or expired",
    "details": { ... }
  },
  "timestamp": "2026-03-06T12:00:00Z"
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| INVALID_API_KEY | 401 | API key is invalid or expired |
| RATE_LIMIT_EXCEEDED | 429 | Rate limit has been exceeded |
| INVALID_REQUEST | 400 | Request parameters are invalid |
| SERVER_ERROR | 500 | Internal server error |
| NOT_FOUND | 404 | Resource not found |

## API Endpoints

### Connection Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/connections` | Create new connection |
| GET | `/connections/{id}` | Get connection details |
| GET | `/connections` | List all connections |
| DELETE | `/connections/{id}` | Terminate connection |
| PUT | `/connections/{id}` | Update connection settings |

### Server Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/servers` | List available servers |
| GET | `/servers/{id}` | Get server details |
| GET | `/servers/{id}/status` | Get server status |
| POST | `/servers/{id}/test` | Test server connectivity |

### Account Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/account` | Get account details |
| PUT | `/account` | Update account settings |
| GET | `/account/usage` | Get usage statistics |
| GET | `/account/billing` | Get billing information |

### Security Features

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/security/keys` | List encryption keys |
| POST | `/security/keys` | Generate new key pair |
| DELETE | `/security/keys/{id}` | Delete encryption key |
| GET | `/security/audit` | Get security audit log |

## Webhooks

VantisVPN supports webhooks for real-time event notifications:

### Supported Events

- `connection.created` - New connection established
- `connection.terminated` - Connection ended
- `server.status_changed` - Server status updated
- `security.alert` - Security event detected
- `billing.charge` - Billing event occurred

### Setting Up Webhooks

```bash
POST /webhooks
Content-Type: application/json

{
  "url": "https://your-server.com/webhook",
  "events": ["connection.created", "connection.terminated"],
  "secret": "your_webhook_secret"
}
```

### Webhook Payload Example

```json
{
  "event": "connection.created",
  "timestamp": "2026-03-06T12:00:00Z",
  "data": {
    "connection_id": "conn_123abc",
    "server_id": "srv_456def",
    "ip_address": "192.168.1.100",
    "protocol": "wireguard"
  }
}
```

## WebSocket API

For real-time updates, VantisVPN provides a WebSocket API:

### Connection

```javascript
const ws = new WebSocket('wss://api.vantisvpn.com/v1/stream');

ws.onopen = () => {
  // Send authentication
  ws.send(JSON.stringify({
    action: 'authenticate',
    api_key: 'YOUR_API_KEY'
  }));
};
```

### Real-time Events

- Connection status updates
- Speed/latency metrics
- Security alerts
- Server load information

## SDKs

Official SDKs are available for popular programming languages:

- [Rust SDK](https://github.com/vantisCorp/vantisvpn-rust)
- [Python SDK](https://github.com/vantisCorp/vantisvpn-python)
- [JavaScript SDK](https://github.com/vantisCorp/vantisvpn-js)
- [Go SDK](https://github.com/vantisCorp/vantisvpn-go)

## Examples

### Creating a Connection (Rust)

```rust
use vantisvpn::{Client, ConnectionConfig};

let client = Client::new("YOUR_API_KEY");

let config = ConnectionConfig::builder()
    .server_id("us-east-1")
    .protocol(Protocol::WireGuard)
    .quantum_enabled(true)
    .build();

let connection = client.connect(&config).await?;
println!("Connected: {}", connection.id());
```

### Creating a Connection (Python)

```python
from vantisvpn import Client

client = Client(api_key="YOUR_API_KEY")

config = {
    "server_id": "us-east-1",
    "protocol": "wireguard",
    "quantum_enabled": True
}

connection = client.connect(**config)
print(f"Connected: {connection.id}")
```

### Creating a Connection (JavaScript)

```javascript
const { Client } = require('@vantisvpn/sdk');

const client = new Client({ apiKey: 'YOUR_API_KEY' });

const config = {
  serverId: 'us-east-1',
  protocol: 'wireguard',
  quantumEnabled: true
};

const connection = await client.connect(config);
console.log(`Connected: ${connection.id}`);
```

## Best Practices

1. **Always use HTTPS** for API requests
2. **Implement retry logic** for transient failures
3. **Cache API keys** securely, never hardcode
4. **Use webhooks** instead of polling for updates
5. **Monitor rate limits** and implement backoff
6. **Validate webhook signatures** for security
7. **Use connection pooling** for better performance
8. **Handle errors gracefully** with proper logging

## Testing

Use the sandbox environment for testing:

```
https://sandbox-api.vantisvpn.com/v1
```

Sandbox API keys don't affect production data.

## Support

- **Documentation**: https://docs.vantisvpn.com
- **API Status**: https://status.vantisvpn.com
- **GitHub Issues**: https://github.com/vantisCorp/VantisVPN/issues
- **Email**: api-support@vantisvpn.com

---

*API Version: 1.0.0* | *Last Updated: March 6, 2026*