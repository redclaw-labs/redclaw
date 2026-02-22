# Thiết lập Nextcloud Talk

Hướng dẫn này bao gồm tích hợp Nextcloud Talk (native) cho RedClaw.

## 1. Tích hợp này làm gì

- Nhận các sự kiện webhook Talk bot (inbound) qua `POST /nextcloud-talk`.
- Xác minh chữ ký webhook (HMAC-SHA256) khi có cấu hình secret.
- Gửi phản hồi của bot trở lại các phòng Talk thông qua Nextcloud OCS API.

## 2. Cấu hình

Thêm phần sau vào `~/.redclaw/config.toml`:

```toml
[channels_config.nextcloud_talk]
base_url = "https://cloud.example.com"
app_token = "nextcloud-talk-app-token"
webhook_secret = "optional-webhook-secret"
allowed_users = ["*"]
```

Tham chiếu trường:

- `base_url`: Base URL của Nextcloud.
- `app_token`: Bot app token, dùng làm `Authorization: Bearer <token>` cho OCS send API.
- `webhook_secret`: Secret chia sẻ để xác minh `X-Nextcloud-Talk-Signature`.
- `allowed_users`: Các actor ID Nextcloud được phép (`[]` chặn tất cả, `"*"` cho phép tất cả).

Ghi đè qua biến môi trường:

- `REDCLAW_NEXTCLOUD_TALK_WEBHOOK_SECRET` sẽ ghi đè `webhook_secret` khi được đặt.

## 3. Gateway endpoint

Chạy daemon hoặc gateway và expose endpoint webhook:

```bash
redclaw daemon
# or
redclaw gateway --host 127.0.0.1 --port 3000
```

Cấu hình URL webhook cho bot Nextcloud Talk thành:

- `https://<your-public-url>/nextcloud-talk`

## 4. Hợp đồng xác minh chữ ký

Khi `webhook_secret` được cấu hình, RedClaw sẽ xác minh:

- header `X-Nextcloud-Talk-Random`
- header `X-Nextcloud-Talk-Signature`

Công thức xác minh:

- `hex(hmac_sha256(secret, random + raw_request_body))`

Nếu xác minh thất bại, gateway trả về `401 Unauthorized`.

## 5. Hành vi định tuyến tin nhắn

- RedClaw bỏ qua các webhook event do bot tạo (`actorType = bots`).
- RedClaw bỏ qua các event không phải message/system.
- Định tuyến trả lời sử dụng Talk room token từ payload webhook.

## 6. Checklist kiểm tra nhanh

1. Đặt `allowed_users = ["*"]` cho lần kiểm tra đầu tiên.
2. Gửi tin nhắn thử trong phòng Talk mục tiêu.
3. Xác nhận RedClaw nhận và trả lời trong cùng phòng.
4. Siết `allowed_users` về danh sách actor ID cụ thể.

## 7. Khắc phục sự cố

- `404 Nextcloud Talk not configured`: thiếu `[channels_config.nextcloud_talk]`.
- `401 Invalid signature`: sai lệch giữa `webhook_secret`, random header, hoặc raw-body signing.
- Không có trả lời nhưng webhook `200`: event đã bị lọc (bot/system/người dùng không được phép/payload không phải message).
