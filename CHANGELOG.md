# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-02-18

### 🎉 First Public Release

Battle-hardened AI agent runtime built in Rust. High performance, high security, fully modular.

### Added

#### Core Runtime
- Trait-driven modular architecture (`Provider`, `Channel`, `Tool`, `Memory`, `Observer`, `Peripheral`)
- Async Tokio-based agent orchestration loop
- CLI with `run`, `config`, `init`, `pair` commands
- Native runtime adapter
- Workspace and skills system

#### Model Providers (10)
- anthropic
- compatible
- copilot
- gemini
- glm
- ollama
- openai
- openrouter
- reliable
- router
- Resilient provider wrapper with automatic retry and fallback
- Custom endpoint support via `custom:` prefix
- Provider-agnostic message format

#### Communication Channels (14)
- cli
- dingtalk
- discord
- email_channel
- imessage
- irc
- lark
- matrix
- mattermost
- qq
- signal
- slack
- telegram
- whatsapp
- Unified `Channel` trait with `send`/`listen`/`health_check`
- Per-channel allowlist and authentication
- Typing indicators and message formatting

#### Built-in Tools (18)
- browser
- browser_open
- composio
- delegate
- file_read
- file_write
- git_operations
- hardware_board_info
- hardware_memory_map
- hardware_memory_read
- http_request
- image_info
- memory_forget
- memory_recall
- memory_store
- schedule
- screenshot
- shell
- Structured `ToolResult` return type
- Parameter validation and sanitization
- Configurable tool permissions

#### Memory System
- Markdown file-based memory backend
- SQLite memory backend
- Vector embeddings support
- Memory search, merge, and recall

#### Security
- Pairing-based device authentication
- Encrypted secret store
- Security policy engine
- Deny-by-default access control
- Network/filesystem scope restrictions

#### Hardware Peripherals
- STM32 microcontroller support
- Raspberry Pi GPIO control
- Arduino integration
- `Peripheral` trait with `tools()` exposure

#### Observability
- Cost tracking per provider call
- Structured logging with configurable levels
- `Observer` trait for custom integrations

#### Gateway
- Webhook/gateway HTTP server
- REST API endpoints
- Health check endpoints

#### Configuration
- TOML-based configuration schema
- Multi-source config loading and merging
- Environment variable overrides
- Per-channel and per-provider settings

#### Developer Experience
- 1400+ tests
- Comprehensive documentation
- Contributing guide
- Security policy (SECURITY.md)
- Code of Conduct (CODE_OF_CONDUCT.md)

### Performance
- ~8MB release binary (macOS arm64 build)

[1.0.0]: https://github.com/redclaw-labs/redclaw/releases/tag/v1.0.0
