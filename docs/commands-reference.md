# RedClaw Commands Reference

This reference is derived from the current CLI surface (`redclaw --help`).

Last verified: **February 19, 2026**.

## Top-Level Commands

| Command | Purpose |
|---|---|
| `onboard` | Initialize workspace/config quickly or interactively |
| `agent` | Run interactive chat or single-message mode |
| `gateway` | Start webhook and WhatsApp HTTP gateway |
| `daemon` | Start supervised runtime (gateway + channels + optional heartbeat/scheduler) |
| `service` | Manage user-level OS service lifecycle |
| `doctor` | Run diagnostics and freshness checks |
| `status` | Print current configuration and system summary |
| `cron` | Manage scheduled tasks |
| `models` | Refresh provider model catalogs |
| `providers` | List provider IDs, aliases, and active provider |
| `channel` | Manage channels and channel health checks |
| `integrations` | Inspect integration details |
| `skills` | List/install/remove skills |
| `migrate` | Import from external runtimes (currently OpenClaw) |
| `config` | Export machine-readable config schema |
| `hardware` | Discover and introspect USB hardware |
| `peripheral` | Configure and flash peripherals |

## Command Groups

### `onboard`

- `redclaw onboard`
- `redclaw onboard --interactive`
- `redclaw onboard --channels-only`
- `redclaw onboard --api-key <KEY> --provider <ID> --memory <sqlite|lucid|markdown|none>`

### `agent`

- `redclaw agent`
- `redclaw agent -m "Hello"`
- `redclaw agent --provider <ID> --model <MODEL> --temperature <0.0-2.0>`
- `redclaw agent --peripheral <board:path>`

### `gateway` / `daemon`

- `redclaw gateway [--host <HOST>] [--port <PORT>]`
- `redclaw daemon [--host <HOST>] [--port <PORT>]`

### `service`

- `redclaw service install`
- `redclaw service start`
- `redclaw service stop`
- `redclaw service status`
- `redclaw service uninstall`

### `cron`

- `redclaw cron list`
- `redclaw cron add <expr> [--tz <IANA_TZ>] <command>`
- `redclaw cron add-at <rfc3339_timestamp> <command>`
- `redclaw cron add-every <every_ms> <command>`
- `redclaw cron once <delay> <command>`
- `redclaw cron remove <id>`
- `redclaw cron pause <id>`
- `redclaw cron resume <id>`

### `models`

- `redclaw models refresh`
- `redclaw models refresh --provider <ID>`
- `redclaw models refresh --force`

`models refresh` currently supports live catalog refresh for provider IDs: `openrouter`, `openai`, `anthropic`, `groq`, `mistral`, `deepseek`, `xai`, `together-ai`, `gemini`, `ollama`, `astrai`, `venice`, `fireworks`, `cohere`, `moonshot`, `glm`, `zai`, `qwen`, and `nvidia`.

### `channel`

- `redclaw channel list`
- `redclaw channel start`
- `redclaw channel doctor`
- `redclaw channel bind-telegram <IDENTITY>`
- `redclaw channel add <type> <json>`
- `redclaw channel remove <name>`

Runtime in-chat commands (Telegram/Discord while channel server is running):

- `/models`
- `/models <provider>`
- `/model`
- `/model <model-id>`

`add/remove` currently route you back to managed setup/manual config paths (not full declarative mutators yet).

### `integrations`

- `redclaw integrations info <name>`

### `skills`

- `redclaw skills list`
- `redclaw skills install <source>`
- `redclaw skills remove <name>`

Skill manifests (`SKILL.toml`) support `prompts` and `[[tools]]`; both are injected into the agent system prompt at runtime, so the model can follow skill instructions without manually reading skill files.

### `migrate`

- `redclaw migrate openclaw [--source <path>] [--dry-run]`

### `config`

- `redclaw config schema`

`config schema` prints a JSON Schema (draft 2020-12) for the full `config.toml` contract to stdout.

### `hardware`

- `redclaw hardware discover`
- `redclaw hardware introspect <path>`
- `redclaw hardware info [--chip <chip_name>]`

### `peripheral`

- `redclaw peripheral list`
- `redclaw peripheral add <board> <path>`
- `redclaw peripheral flash [--port <serial_port>]`
- `redclaw peripheral setup-uno-q [--host <ip_or_host>]`
- `redclaw peripheral flash-nucleo`

## Validation Tip

To verify docs against your current binary quickly:

```bash
redclaw --help
redclaw <command> --help
```
