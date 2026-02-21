# RedClaw Commands Reference

This reference is derived from the current CLI surface (`redclaw --help`).

Last verified: **February 20, 2026**.

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
| `completions` | Generate shell completion scripts to stdout |
| `hardware` | Discover and introspect USB hardware |
| `peripheral` | Configure and flash peripherals |

## Command Groups

### `onboard`

- `redclaw onboard`
- `redclaw onboard --interactive`
- `redclaw onboard --channels-only`
- `redclaw onboard --force`
- `redclaw onboard --api-key <KEY> --provider <ID> --memory <sqlite|lucid|markdown|none>`
- `redclaw onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none>`
- `redclaw onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none> --force`

`onboard` safety behavior:

- If `config.toml` already exists, `onboard` asks for explicit confirmation before overwrite.
- In non-interactive environments, existing `config.toml` causes a safe refusal unless `--force` is passed.
- Use `redclaw onboard --channels-only` when you only need to rotate channel tokens/allowlists.

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
- `redclaw service restart`
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

Notes:

- Mutating schedule/cron actions require `cron.enabled = true`.
- Shell command payloads for schedule creation (`create` / `add` / `once`) are validated by security command policy before job persistence.

### `models`

- `redclaw models refresh`
- `redclaw models refresh --provider <ID>`
- `redclaw models refresh --force`

`models refresh` currently supports live catalog refresh for provider IDs: `openrouter`, `openai`, `anthropic`, `groq`, `mistral`, `deepseek`, `xai`, `together-ai`, `gemini`, `ollama`, `llamacpp`, `astrai`, `venice`, `fireworks`, `cohere`, `moonshot`, `glm`, `zai`, `qwen`, and `nvidia`.

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

Channel runtime also watches `config.toml` and hot-applies updates to:
- `default_provider`
- `default_model`
- `default_temperature`
- `api_key` / `api_url` (for the default provider)
- `reliability.*` provider retry settings

`add/remove` currently route you back to managed setup/manual config paths (not full declarative mutators yet).

### `integrations`

- `redclaw integrations info <name>`

### `skills`

- `redclaw skills list`
- `redclaw skills install <source>`
- `redclaw skills remove <name>`

`<source>` accepts git remotes (`https://...`, `http://...`, `ssh://...`, and `git@host:owner/repo.git`) or a local filesystem path.

Skill manifests (`SKILL.toml`) support `prompts` and `[[tools]]`; both are injected into the agent system prompt at runtime, so the model can follow skill instructions without manually reading skill files.

### `migrate`

- `redclaw migrate openclaw [--source <path>] [--dry-run]`

### `config`

- `redclaw config schema`

`config schema` prints a JSON Schema (draft 2020-12) for the full `config.toml` contract to stdout.

### `completions`

- `redclaw completions bash`
- `redclaw completions fish`
- `redclaw completions zsh`
- `redclaw completions powershell`
- `redclaw completions elvish`

`completions` is stdout-only by design so scripts can be sourced directly without log/warning contamination.

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
