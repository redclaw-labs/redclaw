<p align="center">
  <img src="logo.png" alt="RedClaw" width="200" />
</p>

<h1 align="center">RedClaw 🦀</h1>

<p align="center">
  <strong>Zero overhead. Zero compromise. Deploy anywhere. Swap anything.</strong>
</p>

<p align="center">
  <a href="https://x.com/redclawlabs?s=21"><img src="https://img.shields.io/badge/X-%40redclawlabs-000000?style=flat&logo=x&logoColor=white" alt="X: @redclawlabs" /></a>
  <a href="https://t.me/redclawlabs"><img src="https://img.shields.io/badge/Telegram-%40redclawlabs-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram: @redclawlabs" /></a>
  <a href="https://www.reddit.com/r/redclawlabs/"><img src="https://img.shields.io/badge/Reddit-r%2Fredclawlabs-FF4500?style=flat&logo=reddit&logoColor=white" alt="Reddit: r/redclawlabs" /></a>
</p>

<p align="center">
  🌐 Languages: <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ru.md">Русский</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="docs/README.fr.md">Français</a>
</p>

<p align="center">
  <a href="bootstrap.sh">One-click bootstrap</a> |
  <a href="docs/getting-started/README.md">Getting started</a> |
  <a href="docs/README.md">Docs hub</a> |
  <a href="docs/SUMMARY.md">Docs TOC</a>
</p>

<p align="center">
  <strong>Fast routing:</strong>
  <a href="docs/reference/README.md">Reference</a> ·
  <a href="docs/operations/README.md">Operations & deployment</a> ·
  <a href="docs/troubleshooting.md">Troubleshooting</a> ·
  <a href="docs/security/README.md">Security</a> ·
  <a href="docs/hardware/README.md">Hardware & peripherals</a> ·
  <a href="docs/contributing/README.md">Contributing & CI</a>
</p>

<p align="center">
  <a href="https://github.com/redclaw-labs/redclaw/actions/workflows/ci.yml"><img src="https://github.com/redclaw-labs/redclaw/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-red.svg" alt="License: MIT" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.92%2B-orange.svg" alt="Rust: 1.92+" /></a>
  <a href="CHANGELOG.md"><img src="https://img.shields.io/badge/version-1.1.0-crimson.svg" alt="Version" /></a>
  <a href="RUN_TESTS.md"><img src="https://img.shields.io/badge/tests-3000%2B-green.svg" alt="Tests" /></a>
  <a href="#providers"><img src="https://img.shields.io/badge/providers-30%2B-blue.svg" alt="Providers" /></a>
  <a href="#channels"><img src="https://img.shields.io/badge/channels-16-purple.svg" alt="Channels" /></a>
  <a href="#tools"><img src="https://img.shields.io/badge/tools-22-teal.svg" alt="Tools" /></a>
</p>

---

## Table of Contents

- [📢 Announcement Board](#announcement-board)
- [Project Overview](#project-overview)
- [Why RedClaw?](#why-redclaw)
- [Performance](#performance)
- [One-click Bootstrap](#one-click-bootstrap)
- [Features](#features)
- [Quick Start](#quick-start)
- [Subscription Auth (OpenAI Codex / Claude Code)](#subscription-auth-openai-codex--claude-code)
- [Architecture](#architecture)
- [Memory System (Full-stack search engine)](#memory-system-full-stack-search-engine)
- [Security Defaults (Important)](#security-defaults-important)
- [Common Config Snippet](#common-config-snippet)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)
- [Community](#community)

## 📢 Announcement Board

Used for important notices (breaking changes, security announcements, maintenance windows, release blockers, etc.).

| Date (UTC) | Severity | Announcement | Recommended action |
|---|---|---|---|
| 2026-02-19 | _Urgent_ | We have **no relationship** with `openagen/redclaw` or `redclaw.org`. `redclaw.org` currently points to the `openagen/redclaw` fork, and that domain/repo is impersonating our official website and project. | Do not trust any information, binaries, fundraising, or “official” statements from those sources. Only trust `github.com/redclaw-labs/redclaw` and the verified social accounts linked in the badges above. |
| 2026-02-19 | _Important_ | We currently **do not have an official website**. If you see any investment/fundraising activity in the name of RedClaw, treat it as suspicious. | Cross-check everything against this repo; follow the official X/Reddit/Telegram accounts for updates. |
| 2026-02-19 | _Important_ | Anthropic updated “Authentication and Credential Use” terms (2026-02-19). The terms state OAuth authentication (Free/Pro/Max) is only for Claude Code and Claude.ai; using OAuth tokens from Claude Free/Pro/Max for other products/tools/services (including Agent SDKs) is not allowed and may violate Consumer Terms of Service. | To reduce risk, do not attempt Claude Code OAuth integration for now. Source: [Authentication and Credential Use](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use). |

## Project Overview

RedClaw is an autonomous agent runtime optimized for performance, resource efficiency, and composability:

- Rust-native, single-binary deploy across ARM / x86 / RISC-V.
- Trait-driven architecture: `Provider` / `Channel` / `Tool` / `Memory` are swappable by configuration.
- Secure-by-default: pairing, explicit allowlists, sandboxing, and scoped access.

## Why RedClaw?

RedClaw is a Rust-built agent runtime that ships as a single small binary and stays fast under real workloads.
It’s security-first by default (pairing, secrets, allowlists, workspace scoping) and designed for “deploy anywhere” environments.
Everything important is trait-driven and swappable (providers, channels, tools, memory, runtime), so you can change integrations without rewriting your agent.

## Performance

![RedClaw vs Others](benchmark.jpeg)

| Feature | RedClaw | OpenClaw | Auto-GPT | CrewAI | LangGraph | Goose |
|---------|---------|----------|----------|--------|-----------|-------|
| Language | Rust | TypeScript | Python | Python | Python/JS | Rust |
| Binary | 3.4MB | N/A (Node) | N/A (Python) | N/A (Python) | N/A | ~15MB |
| Memory | <5MB | >1GB | >500MB | >200MB | >300MB | ~50MB |
| Startup | <10ms | >2s | >5s | >3s | >2s | ~100ms |
| Providers | 30+ | 5+ | 3+ | 10+ | 10+ | 5+ |
| Channels | 16 | 3 | 1 | 0 | 0 | 1 |
| Tools | 22 | 5+ | 10+ | 5+ | 5+ | 10+ |
| Hardware | Yes | No | No | No | No | No |
| Security | ★★★★★ | ★★☆☆☆ | ★★☆☆☆ | ★★★☆☆ | ★★★☆☆ | ★★★★☆ |

### Reproducible measurement (local)

Bench numbers change with source/toolchain; measure in your target environment:

```bash
cargo build --release
ls -lh target/release/redclaw

/usr/bin/time -l target/release/redclaw --help
/usr/bin/time -l target/release/redclaw status
```

Sample (macOS arm64, 2026-02-18):

- Release binary: `8.8M`
- `redclaw --help`: ~`0.02s`, peak RAM ~`3.9MB`
- `redclaw status`: ~`0.01s`, peak RAM ~`4.1MB`

## One-click Bootstrap

```bash
git clone https://github.com/redclaw-labs/redclaw.git
cd redclaw
./bootstrap.sh
```

Optional environment initialization: `./bootstrap.sh --install-system-deps --install-rust` (may require `sudo`).

Details: [docs/one-click-bootstrap.md](docs/one-click-bootstrap.md).

## Features

<a name="providers"></a>

### 🤖 30+ Model Providers

RedClaw includes first-party providers plus OpenAI-compatible adapters and aliases.

Highlights:
- OpenAI (`openai`)
- Anthropic (`anthropic`)
- Google Gemini (`gemini` / `google`)
- OpenRouter (`openrouter`)
- Ollama (`ollama`)
- Groq, Mistral, xAI, DeepSeek, Together, Fireworks, Perplexity, Cohere
- Amazon Bedrock (`bedrock` / `aws-bedrock`)
- Qwen / DashScope, GLM / Zhipu, Moonshot / Kimi, MiniMax
- NVIDIA NIM (`nvidia` / `nvidia-nim`)

Custom endpoints:
- `custom:https://your-api.com` (OpenAI-compatible)
- `anthropic-custom:https://your-api.com` (Anthropic-compatible)

<a name="channels"></a>

### 📡 16 Communication Channels

- CLI
- Telegram
- Discord
- Slack
- WhatsApp
- Matrix
- IRC
- iMessage
- Email
- Signal
- Mattermost
- Nextcloud Talk
- DingTalk
- Lark
- Webhook (Linq)
- QQ

<a name="tools"></a>

### 🛠️ 22 Built-in Tools

Key tools shipped in-tree:
- Shell execution (native runtime + sandbox policies)
- File read/write (workspace-scoped)
- Memory store/recall/forget
- Scheduling
- Git operations
- HTTP requests (domain-allowlisted)
- Browser open + browser automation (optional)
- Screenshot + image inspection
- Hardware helpers (board info + memory map/read)
- Delegation tool (optional, when extra agents are configured)

### 🔩 Hardware Peripherals

- STM32
- Raspberry Pi GPIO
- Arduino

### 🔒 Security-First

- Pairing for gateway access
- Encrypted local secrets
- Workspace-scoped file access
- Sender allowlists for inbound channels
- Fail-fast behavior for unsupported or unsafe configs

### 🧠 Memory System

- Backends: Markdown and SQLite
- Hybrid retrieval: keyword search + vector similarity
- Embedding provider support (including OpenAI and no-op)

### 📊 Observability

- Cost tracking
- Structured logging
- Prometheus metrics

## Quick Start

### Homebrew (macOS/Linuxbrew)

```bash
brew install redclaw
```

### From source (recommended for development)

```bash
git clone https://github.com/redclaw-labs/redclaw.git
cd redclaw
cargo build --release --locked
cargo install --path . --force --locked

# Fast onboarding (non-interactive)
redclaw onboard --api-key sk-... --provider openrouter

# Or interactive wizard
redclaw onboard --interactive

# One-shot chat
redclaw agent -m "Hello, RedClaw!"

# Start gateway (default: 127.0.0.1:3000)
redclaw gateway

# Long-running mode
redclaw daemon
```

## Subscription Auth (OpenAI Codex / Claude Code)

RedClaw supports native subscription-style auth profiles (multiple accounts, static encrypted storage).

- Config file: `~/.redclaw/auth-profiles.json`
- Encryption key: `~/.redclaw/.secret_key`
- Profile ID format: `<provider>:<profile_name>` (example: `openai-codex:work`)

OpenAI Codex OAuth (ChatGPT subscription):

```bash
# Recommended for server/headless environments
redclaw auth login --provider openai-codex --device-code

# Browser/callback flow (with paste fallback)
redclaw auth login --provider openai-codex --profile default
redclaw auth paste-redirect --provider openai-codex --profile default

# Check / refresh / switch profiles
redclaw auth status
redclaw auth refresh --provider openai-codex --profile default
redclaw auth use --provider openai-codex --profile work
```

Claude Code / Anthropic setup-token:

```bash
# Paste subscription/setup token (Authorization header mode)
redclaw auth paste-token --provider anthropic --profile default --auth-kind authorization

# Alias
redclaw auth setup-token --provider anthropic --profile default
```

Run the agent with subscription auth:

```bash
redclaw agent --provider openai-codex -m "hello"
redclaw agent --provider openai-codex --auth-profile openai-codex:work -m "hello"

# Anthropic supports API key and auth token environment variables:
# ANTHROPIC_AUTH_TOKEN, ANTHROPIC_OAUTH_TOKEN, ANTHROPIC_API_KEY
redclaw agent --provider anthropic -m "hello"
```

## Architecture

Every subsystem is a **Trait** — swap implementations with config, without rewriting the agent.

<p align="center">
  <img src="docs/architecture.svg" alt="RedClaw architecture" width="900" />
</p>

| Subsystem | Trait | Built-in implementations | Extension path |
|--------|-------|----------|----------|
| **AI model** | `Provider` | See `redclaw providers` (built-in providers + aliases, plus custom endpoints) | `custom:https://your-api.com` (OpenAI-compatible) or `anthropic-custom:https://your-api.com` |
| **Channels** | `Channel` | CLI, Telegram, Discord, Slack, Mattermost, iMessage, Matrix, Signal, WhatsApp, Email, IRC, Lark, DingTalk, QQ, Webhook (Linq), Nextcloud Talk | Any messaging API |
| **Memory** | `Memory` | SQLite hybrid search, PostgreSQL backend, Lucid bridge, Markdown file, explicit `none` backend, snapshot/hydrate, optional response cache | Any persistence backend |
| **Tools** | `Tool` | shell/file/memory, cron/schedule, git, proxy_config, browser, http_request, screenshot/image_info, composio (opt-in), delegate, hardware tools | Any capability |
| **Observability** | `Observer` | Noop, Log, Multi | Prometheus, OTel |
| **Runtime** | `RuntimeAdapter` | Native, Docker (sandbox) | Add via adapter; unsupported kinds fail fast |
| **Security** | `SecurityPolicy` | Gateway pairing, sandboxing, allowlists, rate limits, filesystem scoping, encrypted secrets | — |
| **Identity** | `IdentityConfig` | OpenClaw (markdown), AIEOS v1.1 (JSON) | Any identity format |
| **Tunnel** | `Tunnel` | None, Cloudflare, Tailscale, ngrok, Custom | Any tunnel tool |
| **Heartbeat** | Engine | HEARTBEAT.md periodic tasks | — |
| **Skills** | Loader | TOML manifest + SKILL.md instruction | Community skill packs |
| **Integrations** | Registry | 70+ integrations across 9 categories | Plugin system |

### Runtime support (current)

- ✅ Supported: `runtime.kind = "native"` or `runtime.kind = "docker"`
- 🚧 Planned (not implemented): WASM / edge runtime

If you configure an unsupported `runtime.kind`, RedClaw exits with an explicit error instead of silently falling back.

## Memory System (Full-stack search engine)

Fully in-tree, zero external services — no Pinecone, Elasticsearch, or LangChain required:

| Layer | Implementation |
|------|------|
| **Vector DB** | Embeddings stored as BLOBs in SQLite, cosine similarity search |
| **Keyword search** | FTS5 virtual tables, BM25 scoring |
| **Hybrid merge** | Custom weighted merge function (`vector.rs`) |
| **Embeddings** | Trait `EmbeddingProvider` — OpenAI, custom URL, or noop |
| **Chunking** | Line-based Markdown chunker that preserves heading structure |
| **Cache** | SQLite `embedding_cache` table, LRU policy |
| **Safe re-index** | Atomic FTS5 rebuild + re-embed missing vectors |

The agent automatically recalls/stores/manages memory via tools.

```toml
[memory]
backend = "sqlite"             # "sqlite", "lucid", "postgres", "markdown", "none"
auto_save = true
embedding_provider = "none"    # "none", "openai", "custom:https://..."
vector_weight = 0.7
keyword_weight = 0.3
```

## Security Defaults (Important)

- Gateway default bind: `127.0.0.1:3000`
- Gateway default pairing: `require_pairing = true`
- Default deny public bind: `allow_public_bind = false`
- Channel allowlist semantics:
  - empty list `[]` => deny-by-default
  - `"*"` => allow all (only when you fully understand the risk)

## Common Config Snippet

RedClaw reads `~/.redclaw/config.toml` (typically created by `redclaw onboard`).

```toml
api_key = "sk-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4.6"
default_temperature = 0.7

[memory]
backend = "sqlite"             # sqlite | lucid | postgres | markdown | none
auto_save = true
embedding_provider = "none"    # none | openai | custom:https://...

[gateway]
host = "127.0.0.1"
port = 3000
require_pairing = true
allow_public_bind = false
```

## Documentation

Start in the [docs/](docs/) directory. A few useful entry points:
- [docs/sandboxing.md](docs/sandboxing.md)
- [docs/agnostic-security.md](docs/agnostic-security.md)
- [docs/audit-logging.md](docs/audit-logging.md)
- [docs/network-deployment.md](docs/network-deployment.md)

### Docs navigation (recommended start here)

- Docs overview (English): [docs/README.md](docs/README.md)
- Unified Table of Contents (TOC): [docs/SUMMARY.md](docs/SUMMARY.md)
- Command reference: [docs/commands-reference.md](docs/commands-reference.md)
- Config reference: [docs/config-reference.md](docs/config-reference.md)
- Provider reference: [docs/providers-reference.md](docs/providers-reference.md)
- Channel reference: [docs/channels-reference.md](docs/channels-reference.md)
- Operations runbook: [docs/operations-runbook.md](docs/operations-runbook.md)
- Troubleshooting: [docs/troubleshooting.md](docs/troubleshooting.md)
- Docs inventory: [docs/docs-inventory.md](docs/docs-inventory.md)
- Project triage snapshot (2026-02-18): [docs/project-triage-snapshot-2026-02-18.md](docs/project-triage-snapshot-2026-02-18.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Implement a trait, submit a PR:

- CI workflow guide: [docs/ci-map.md](docs/ci-map.md)
- New `Provider` → `src/providers/`
- New `Channel` → `src/channels/`
- New `Observer` → `src/observability/`
- New `Tool` → `src/tools/`
- New `Memory` → `src/memory/`
- New `Tunnel` → `src/tunnel/`
- New `Skill` → `~/.redclaw/workspace/skills/<name>/`

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT — see [LICENSE](LICENSE).

See [CHANGELOG.md](CHANGELOG.md) for release history.

## Community

- [GitHub Issues](https://github.com/redclaw-labs/redclaw/issues)
- [GitHub Discussions](https://github.com/redclaw-labs/redclaw/discussions)

```bash
cargo test               # 3,000+ tests (actual: 3,041)
cargo clippy             # Lint (0 warnings)
cargo fmt                # Format

# Run the SQLite vs Markdown benchmark
cargo test --test memory_comparison -- --nocapture
```

### Pre-push hook

A git hook runs `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` before every push. Enable it once:

```bash
git config core.hooksPath .githooks
```

### Build troubleshooting (Linux OpenSSL errors)

If you see an `openssl-sys` build error, sync dependencies and rebuild with the repository lockfile:

```bash
git pull
cargo build --release --locked
cargo install --path . --force --locked
```

RedClaw is configured to use `rustls` for HTTP/TLS dependencies; `--locked` keeps the transitive graph deterministic on fresh environments.

To skip the hook when you need a quick push during development:

```bash
git push --no-verify
```

## Collaboration & Docs

For high-throughput collaboration and consistent reviews:

- Contribution guide: [CONTRIBUTING.md](CONTRIBUTING.md)
- PR workflow policy: [docs/pr-workflow.md](docs/pr-workflow.md)
- Reviewer playbook (triage + deep review): [docs/reviewer-playbook.md](docs/reviewer-playbook.md)
- CI ownership and triage map: [docs/ci-map.md](docs/ci-map.md)
- Security disclosure policy: [SECURITY.md](SECURITY.md)

## Support

RedClaw is an open-source project maintained with passion. If you find it useful and would like to support its continued development, hardware for testing, and coffee for the maintainer, you can support me here:

<a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>

### 🙏 Special Thanks

A heartfelt thank you to the communities and institutions that inspire and fuel this open-source work:

- **Harvard University** — for fostering intellectual curiosity and pushing the boundaries of what's possible.
- **MIT** — for championing open knowledge, open source, and the belief that technology should be accessible to everyone.
- **Sundai Club** — for the community, the energy, and the relentless drive to build things that matter.
- **The World & Beyond** 🌍✨ — to every contributor, dreamer, and builder out there making open source a force for good. This is for you.

We're building in the open because the best ideas come from everywhere. If you're reading this, you're part of it. Welcome. 🦀❤️

---

**RedClaw** — Zero overhead. Zero compromise. Deploy anywhere. Swap anything. 🦀

## Star History

<p align="center">
  <a href="https://www.star-history.com/#redclaw-labs/redclaw&Date">
    <img src="https://api.star-history.com/svg?repos=redclaw-labs/redclaw&type=Date" alt="Star History Chart" />
  </a>

</p>
