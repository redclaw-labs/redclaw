
<div align="center">
  <img src="logo.png" alt="RedClaw" width="200">
  <h1>RedClaw</h1>
  <p><strong>Battle-hardened AI agent runtime. Built in Rust.</strong></p>

  <!-- Badges row -->
  [![CI](https://github.com/redclaw-labs/redclaw/actions/workflows/ci.yml/badge.svg)](https://github.com/redclaw-labs/redclaw/actions/workflows/ci.yml)
  [![License: MIT](https://img.shields.io/badge/License-MIT-red.svg)](LICENSE)
  [![Rust](https://img.shields.io/badge/Rust-1.92%2B-orange.svg)](https://www.rust-lang.org/)
  [![Version](https://img.shields.io/badge/version-1.0.0-crimson.svg)](CHANGELOG.md)
  [![Tests](https://img.shields.io/badge/tests-3000%2B-green.svg)](RUN_TESTS.md)
  [![Providers](https://img.shields.io/badge/providers-30%2B-blue.svg)](#providers)
  [![Channels](https://img.shields.io/badge/channels-14-purple.svg)](#channels)
</div>

---

## Table of Contents

- [Why RedClaw?](#why-redclaw)
- [Performance](#performance)
- [Features](#features)
- [Quick Start](#quick-start)
  - [From Source](#from-source)
  - [Configuration](#configuration)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)
- [Community](#community)

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
| Channels | 14 | 3 | 1 | 0 | 0 | 1 |
| Tools | 18+ | 5+ | 10+ | 5+ | 5+ | 10+ |
| Hardware | Yes | No | No | No | No | No |
| Security | ★★★★★ | ★★☆☆☆ | ★★☆☆☆ | ★★★☆☆ | ★★★☆☆ | ★★★★☆ |

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

### 📡 14 Communication Channels

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
- DingTalk
- Lark
- QQ

### 🛠️ 18+ Built-in Tools

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

1) Build and install
2) Run `onboard`
3) Start chatting

### From Source

```bash
git clone https://github.com/redclaw-labs/redclaw.git
cd redclaw
cargo build --release
```

### Configuration

RedClaw reads `~/.redclaw/config.toml` (typically created by `redclaw onboard`).

Minimal example:
```toml
api_key = "sk-REPLACE_ME"
default_provider = "openrouter"
default_model = "anthropic/claude-3.5-sonnet"
default_temperature = 0.7

[memory]
backend = "sqlite"
auto_save = true
```

Run:
```bash
redclaw onboard --interactive
redclaw agent -m "Hello, RedClaw!"
```

## Documentation

Start in the [docs/](docs/) directory. A few useful entry points:
- [docs/sandboxing.md](docs/sandboxing.md)
- [docs/agnostic-security.md](docs/agnostic-security.md)
- [docs/audit-logging.md](docs/audit-logging.md)
- [docs/network-deployment.md](docs/network-deployment.md)

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
