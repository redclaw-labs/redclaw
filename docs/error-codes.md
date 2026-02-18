# RedClaw Error Codes

RedClaw uses structured error codes (RC-xxxx) for reliable diagnostics.

## Format

```
RC-CXXX
│  │ │
│  │ └── Specific error number (001-999)
│  └──── Category digit (1-9)
└─────── RedClaw prefix
```

## Categories

### RC-1xxx: Configuration

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-1001 | CONFIG_NOT_FOUND | Configuration file not found |
| RC-1002 | CONFIG_PARSE_ERROR | Failed to parse configuration |
| RC-1003 | CONFIG_INVALID_VALUE | Invalid configuration value |
| RC-1004 | CONFIG_MISSING_REQUIRED | Required field missing |
| RC-1005 | CONFIG_DIR_CREATE_FAILED | Failed to create config directory |

### RC-2xxx: Provider

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-2001 | PROVIDER_NOT_FOUND | Provider not found |
| RC-2002 | PROVIDER_AUTH_FAILED | Authentication failed |
| RC-2003 | PROVIDER_RATE_LIMITED | Rate limit exceeded |
| RC-2004 | PROVIDER_TIMEOUT | Request timed out |
| RC-2005 | PROVIDER_RESPONSE_ERROR | Invalid response |
| RC-2006 | PROVIDER_MODEL_NOT_FOUND | Model not available |

### RC-3xxx: Channel

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-3001 | CHANNEL_NOT_FOUND | Channel not found |
| RC-3002 | CHANNEL_AUTH_FAILED | Channel auth failed |
| RC-3003 | CHANNEL_SEND_FAILED | Send failed |
| RC-3004 | CHANNEL_LISTEN_FAILED | Listener error |
| RC-3005 | CHANNEL_HEALTH_CHECK_FAILED | Health check failed |

### RC-4xxx: Tool

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-4001 | TOOL_NOT_FOUND | Tool not found |
| RC-4002 | TOOL_EXECUTION_FAILED | Execution failed |
| RC-4003 | TOOL_INVALID_PARAMS | Invalid parameters |
| RC-4004 | TOOL_PERMISSION_DENIED | Permission denied |

### RC-5xxx: Security

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-5001 | SECURITY_UNAUTHORIZED | Unauthorized access |
| RC-5002 | SECURITY_PAIRING_FAILED | Pairing failed |
| RC-5003 | SECURITY_SECRET_NOT_FOUND | Secret not found |
| RC-5004 | SECURITY_POLICY_VIOLATION | Policy violation |

### RC-6xxx: Runtime

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-6001 | RUNTIME_INIT_FAILED | Initialization failed |
| RC-6002 | RUNTIME_SANDBOX_ERROR | Sandbox error |
| RC-6003 | RUNTIME_RESOURCE_EXHAUSTED | Resource exhausted |

### RC-7xxx: Memory

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-7001 | MEMORY_STORE_FAILED | Store failed |
| RC-7002 | MEMORY_RETRIEVE_FAILED | Retrieve failed |
| RC-7003 | MEMORY_DB_ERROR | Database error |

### RC-8xxx: Gateway

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-8001 | GATEWAY_BIND_FAILED | Bind failed |
| RC-8002 | GATEWAY_TLS_ERROR | TLS error |
| RC-8003 | GATEWAY_WEBHOOK_INVALID | Invalid webhook |

### RC-9xxx: Hardware

| Code | Mnemonic | Description |
|------|----------|-------------|
| RC-9001 | HARDWARE_NOT_FOUND | Hardware not found |
| RC-9002 | HARDWARE_COMM_ERROR | Communication error |
| RC-9003 | HARDWARE_FIRMWARE_ERROR | Firmware error |

## Using Error Codes in Code

```rust
use redclaw::errors::*;

// Bail with error code
rc_bail!(PROVIDER_NOT_FOUND, "provider '{}' is not registered", name);

// Create error without bailing
let err = rc_error!(CONFIG_PARSE_ERROR, "invalid TOML at line {}", line);

// Look up code by number
if let Some(code) = lookup_code(2002) {
    println!("{}: {}", code, code.description);
}
```

## Troubleshooting by Code

When you see an error like `[RC-2002] PROVIDER_AUTH_FAILED`, check:

1. API key is set (`REDCLAW_API_KEY`)
2. Provider supports the chosen model
3. Network connectivity to provider endpoint
