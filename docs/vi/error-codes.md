# Mã lỗi RedClaw

RedClaw sử dụng các mã lỗi có cấu trúc (RC-xxxx) để chẩn đoán ổn định và dễ tự động hóa.

## Định dạng

```
RC-CXXX
│  │ │
│  │ └── Số lỗi cụ thể (001-999)
│  └──── Chữ số hạng mục (1-9)
└─────── Tiền tố RedClaw
```

## Nhóm mã

### RC-1xxx: Cấu hình

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-1001 | CONFIG_NOT_FOUND | Không tìm thấy tệp cấu hình |
| RC-1002 | CONFIG_PARSE_ERROR | Không thể parse cấu hình |
| RC-1003 | CONFIG_INVALID_VALUE | Giá trị cấu hình không hợp lệ |
| RC-1004 | CONFIG_MISSING_REQUIRED | Thiếu trường bắt buộc |
| RC-1005 | CONFIG_DIR_CREATE_FAILED | Không thể tạo thư mục cấu hình |

### RC-2xxx: Provider

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-2001 | PROVIDER_NOT_FOUND | Không tìm thấy provider |
| RC-2002 | PROVIDER_AUTH_FAILED | Xác thực thất bại |
| RC-2003 | PROVIDER_RATE_LIMITED | Vượt giới hạn tốc độ (rate limit) |
| RC-2004 | PROVIDER_TIMEOUT | Hết thời gian chờ (timeout) |
| RC-2005 | PROVIDER_RESPONSE_ERROR | Phản hồi không hợp lệ |
| RC-2006 | PROVIDER_MODEL_NOT_FOUND | Model không khả dụng |

### RC-3xxx: Channel

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-3001 | CHANNEL_NOT_FOUND | Không tìm thấy channel |
| RC-3002 | CHANNEL_AUTH_FAILED | Xác thực channel thất bại |
| RC-3003 | CHANNEL_SEND_FAILED | Gửi thất bại |
| RC-3004 | CHANNEL_LISTEN_FAILED | Lỗi listener |
| RC-3005 | CHANNEL_HEALTH_CHECK_FAILED | Health check thất bại |

### RC-4xxx: Tool

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-4001 | TOOL_NOT_FOUND | Không tìm thấy tool |
| RC-4002 | TOOL_EXECUTION_FAILED | Thực thi thất bại |
| RC-4003 | TOOL_INVALID_PARAMS | Tham số không hợp lệ |
| RC-4004 | TOOL_PERMISSION_DENIED | Bị từ chối quyền |

### RC-5xxx: Security

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-5001 | SECURITY_UNAUTHORIZED | Truy cập không được phép |
| RC-5002 | SECURITY_PAIRING_FAILED | Pairing thất bại |
| RC-5003 | SECURITY_SECRET_NOT_FOUND | Không tìm thấy secret |
| RC-5004 | SECURITY_POLICY_VIOLATION | Vi phạm policy |

### RC-6xxx: Runtime

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-6001 | RUNTIME_INIT_FAILED | Khởi tạo thất bại |
| RC-6002 | RUNTIME_SANDBOX_ERROR | Lỗi sandbox |
| RC-6003 | RUNTIME_RESOURCE_EXHAUSTED | Tài nguyên cạn kiệt |

### RC-7xxx: Memory

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-7001 | MEMORY_STORE_FAILED | Lưu thất bại |
| RC-7002 | MEMORY_RETRIEVE_FAILED | Truy xuất thất bại |
| RC-7003 | MEMORY_DB_ERROR | Lỗi cơ sở dữ liệu |

### RC-8xxx: Gateway

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-8001 | GATEWAY_BIND_FAILED | Bind thất bại |
| RC-8002 | GATEWAY_TLS_ERROR | Lỗi TLS |
| RC-8003 | GATEWAY_WEBHOOK_INVALID | Webhook không hợp lệ |

### RC-9xxx: Phần cứng

| Code | Mnemonic | Mô tả |
|------|----------|-------|
| RC-9001 | HARDWARE_NOT_FOUND | Không tìm thấy phần cứng |
| RC-9002 | HARDWARE_COMM_ERROR | Lỗi giao tiếp |
| RC-9003 | HARDWARE_FIRMWARE_ERROR | Lỗi firmware |

## Dùng mã lỗi trong code

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

## Gỡ rối theo mã

Khi bạn thấy lỗi như `[RC-2002] PROVIDER_AUTH_FAILED`, hãy kiểm tra:

1. API key đã được đặt (`REDCLAW_API_KEY`)
2. Provider có hỗ trợ model bạn chọn
3. Kết nối mạng đến endpoint của provider
