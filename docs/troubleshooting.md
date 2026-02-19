# RedClaw Troubleshooting

This guide focuses on common setup/runtime failures and fast resolution paths.

Last verified: **February 18, 2026**.

## Installation / Bootstrap

### `cargo` not found

Symptom:

- bootstrap exits with `cargo is not installed`

Fix:

```bash
./bootstrap.sh --install-rust
```

Or install from <https://rustup.rs/>.

### Missing system build dependencies

Symptom:

- build fails due to compiler or `pkg-config` issues

Fix:

```bash
./bootstrap.sh --install-system-deps
```

### `redclaw` command not found after install

Symptom:

- install succeeds but shell cannot find `redclaw`

Fix:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
which redclaw
```

Persist in your shell profile if needed.

## Runtime / Gateway

### Gateway unreachable

Checks:

```bash
redclaw status
redclaw doctor
```

Verify `~/.redclaw/config.toml`:

- `[gateway].host` (default `127.0.0.1`)
- `[gateway].port` (default `3000`)
- `allow_public_bind` only when intentionally exposing LAN/public interfaces

### Pairing / auth failures on webhook

Checks:

1. Ensure pairing completed (`/pair` flow)
2. Ensure bearer token is current
3. Re-run diagnostics:

```bash
redclaw doctor
```

## Channel Issues

### Telegram conflict: `terminated by other getUpdates request`

Cause:

- multiple pollers using same bot token

Fix:

- keep only one active runtime for that token
- stop extra `redclaw daemon` / `redclaw channel start` processes

### Channel unhealthy in `channel doctor`

Checks:

```bash
redclaw channel doctor
```

Then verify channel-specific credentials + allowlist fields in config.

## Service Mode

### Service installed but not running

Checks:

```bash
redclaw service status
```

Recovery:

```bash
redclaw service stop
redclaw service start
```

Linux logs:

```bash
journalctl --user -u redclaw.service -f
```

## Legacy Installer Compatibility

Both still work:

```bash
curl -fsSL https://raw.githubusercontent.com/redclaw-labs/redclaw/main/scripts/bootstrap.sh | bash
curl -fsSL https://raw.githubusercontent.com/redclaw-labs/redclaw/main/scripts/install.sh | bash
```

`install.sh` is a compatibility entry and forwards/falls back to bootstrap behavior.

## Still Stuck?

Collect and include these outputs when filing an issue:

```bash
redclaw --version
redclaw status
redclaw doctor
redclaw channel doctor
```

Also include OS, install method, and sanitized config snippets (no secrets).

## Related Docs

- [operations-runbook.md](operations-runbook.md)
- [one-click-bootstrap.md](one-click-bootstrap.md)
- [channels-reference.md](channels-reference.md)
- [network-deployment.md](network-deployment.md)
