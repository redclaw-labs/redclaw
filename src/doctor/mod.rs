//! RedClaw System Doctor — Comprehensive health diagnostics
//!
//! Checks configuration, providers, channels, memory, security, cost,
//! and policy subsystems.

use crate::config::Config;
use crate::memory::MemoryBackendKind;
use serde::Serialize;
use std::fmt;
use std::path::{Path, PathBuf};

/// Overall health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Individual check result
#[derive(Debug, Clone, Serialize)]
pub struct CheckResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub details: Option<String>,
}

/// Full diagnostic report
#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticReport {
    pub timestamp: String,
    pub version: String,
    pub overall: HealthStatus,
    pub checks: Vec<CheckResult>,
    pub summary: ReportSummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportSummary {
    pub total: usize,
    pub healthy: usize,
    pub warnings: usize,
    pub critical: usize,
}

impl DiagnosticReport {
    pub fn new() -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            overall: HealthStatus::Unknown,
            checks: Vec::new(),
            summary: ReportSummary {
                total: 0,
                healthy: 0,
                warnings: 0,
                critical: 0,
            },
        }
    }

    pub fn add_check(&mut self, check: CheckResult) {
        self.checks.push(check);
    }

    /// Compute overall status (worst of all checks)
    pub fn finalize(&mut self) {
        let mut healthy = 0usize;
        let mut warnings = 0usize;
        let mut critical = 0usize;
        let mut unknown = 0usize;

        for c in &self.checks {
            match c.status {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Warning => warnings += 1,
                HealthStatus::Critical => critical += 1,
                HealthStatus::Unknown => unknown += 1,
            }
        }

        self.summary = ReportSummary {
            total: self.checks.len(),
            healthy,
            warnings,
            critical,
        };

        self.overall = if critical > 0 {
            HealthStatus::Critical
        } else if warnings > 0 {
            HealthStatus::Warning
        } else if unknown > 0 {
            HealthStatus::Unknown
        } else {
            HealthStatus::Healthy
        };
    }
}

impl fmt::Display for DiagnosticReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use redclaw::cli::Theme;

        let (overall_symbol, overall_style) = status_symbol_and_style(self.overall);
        writeln!(
            f,
            "{} {}",
            Theme::header().apply_to("🩺 RedClaw Doctor"),
            Theme::dim().apply_to("(system health diagnostics)")
        )?;
        writeln!(f, "  Version:   {}", self.version)?;
        writeln!(f, "  Timestamp: {}", self.timestamp)?;
        writeln!(
            f,
            "  Overall:   {} {}",
            overall_style.apply_to(overall_symbol),
            overall_style.apply_to(format!("{:?}", self.overall))
        )?;
        writeln!(
            f,
            "  Summary:   {} total | {} healthy | {} warnings | {} critical",
            self.summary.total, self.summary.healthy, self.summary.warnings, self.summary.critical
        )?;

        if self.checks.is_empty() {
            return writeln!(f, "  {} no checks executed", Theme::dim().apply_to("ℹ️"));
        }

        writeln!(f)?;
        for check in &self.checks {
            let (symbol, style) = status_symbol_and_style(check.status);
            writeln!(
                f,
                "  {} {}: {}",
                style.apply_to(symbol),
                Theme::emphasis().apply_to(&check.name),
                check.message
            )?;
            if let Some(details) = check.details.as_deref().filter(|d| !d.trim().is_empty()) {
                for line in details.lines() {
                    writeln!(f, "      {}", Theme::dim().apply_to(line))?;
                }
            }
        }

        Ok(())
    }
}

fn status_symbol_and_style(status: HealthStatus) -> (&'static str, console::Style) {
    use redclaw::cli::Theme;
    match status {
        HealthStatus::Healthy => ("✓", Theme::success()),
        HealthStatus::Warning => ("⚠", Theme::warning()),
        HealthStatus::Critical => ("✗", Theme::error()),
        HealthStatus::Unknown => ("?", Theme::dim()),
    }
}

fn non_empty(s: &str) -> bool {
    !s.trim().is_empty()
}

fn check_result(
    name: impl Into<String>,
    status: HealthStatus,
    message: impl Into<String>,
    details: Option<String>,
) -> CheckResult {
    CheckResult {
        name: name.into(),
        status,
        message: message.into(),
        details,
    }
}

fn is_localhost(host: &str) -> bool {
    matches!(host.trim(), "127.0.0.1" | "localhost" | "::1")
}

fn provider_requires_api_key(provider: &str) -> bool {
    // Ollama is local and does not use an API key.
    if provider.eq_ignore_ascii_case("ollama") {
        return false;
    }
    // Most remote providers require some form of token.
    true
}

fn is_provider_name_recognized(provider: &str) -> bool {
    let p = provider.trim();
    if p.is_empty() {
        return false;
    }

    if p.starts_with("custom:") || p.starts_with("anthropic-custom:") {
        return true;
    }

    // Keep in sync with `crate::providers::create_provider` names.
    matches!(
        p,
        "openrouter"
            | "anthropic"
            | "openai"
            | "ollama"
            | "gemini"
            | "google"
            | "google-gemini"
            | "venice"
            | "vercel"
            | "vercel-ai"
            | "cloudflare"
            | "cloudflare-ai"
            | "moonshot"
            | "kimi"
            | "synthetic"
            | "opencode"
            | "opencode-zen"
            | "zai"
            | "z.ai"
            | "glm"
            | "zhipu"
            | "minimax"
            | "bedrock"
            | "aws-bedrock"
            | "qianfan"
            | "baidu"
            | "qwen"
            | "dashscope"
            | "qwen-intl"
            | "dashscope-intl"
            | "qwen-us"
            | "dashscope-us"
            | "together"
            | "together-ai"
            | "fireworks"
            | "fireworks-ai"
            | "perplexity"
            | "cohere"
            | "groq"
            | "mistral"
            | "deepseek"
            | "xai"
            | "grok"
    )
}

fn resolve_configured_api_key_present(provider: &str, config: &Config) -> bool {
    if config
        .api_key
        .as_deref()
        .map(str::trim)
        .is_some_and(|s| !s.is_empty())
    {
        return true;
    }

    // Mirror the provider env resolution logic, but only check existence.
    let provider_env_candidates: &[&str] = match provider {
        "anthropic" => &["ANTHROPIC_OAUTH_TOKEN", "ANTHROPIC_API_KEY"],
        "openrouter" => &["OPENROUTER_API_KEY"],
        "openai" => &["OPENAI_API_KEY"],
        "venice" => &["VENICE_API_KEY"],
        "groq" => &["GROQ_API_KEY"],
        "mistral" => &["MISTRAL_API_KEY"],
        "deepseek" => &["DEEPSEEK_API_KEY"],
        "xai" | "grok" => &["XAI_API_KEY"],
        "together" | "together-ai" => &["TOGETHER_API_KEY"],
        "fireworks" | "fireworks-ai" => &["FIREWORKS_API_KEY"],
        "perplexity" => &["PERPLEXITY_API_KEY"],
        "cohere" => &["COHERE_API_KEY"],
        "moonshot" | "kimi" => &["MOONSHOT_API_KEY"],
        "glm" | "zhipu" => &["GLM_API_KEY"],
        "minimax" => &["MINIMAX_API_KEY"],
        "qianfan" | "baidu" => &["QIANFAN_API_KEY"],
        "qwen" | "dashscope" | "qwen-intl" | "dashscope-intl" | "qwen-us" | "dashscope-us" => {
            &["DASHSCOPE_API_KEY"]
        }
        "zai" | "z.ai" => &["ZAI_API_KEY"],
        "synthetic" => &["SYNTHETIC_API_KEY"],
        "opencode" | "opencode-zen" => &["OPENCODE_API_KEY"],
        "vercel" | "vercel-ai" => &["VERCEL_API_KEY"],
        "cloudflare" | "cloudflare-ai" => &["CLOUDFLARE_API_KEY"],
        _ => &[],
    };

    for env_var in provider_env_candidates {
        if std::env::var(env_var).is_ok_and(|v| non_empty(&v)) {
            return true;
        }
    }

    for env_var in ["REDCLAW_API_KEY", "API_KEY"] {
        if std::env::var(env_var).is_ok_and(|v| non_empty(&v)) {
            return true;
        }
    }

    false
}

fn parse_provider_endpoint_if_custom(provider: &str) -> Option<String> {
    let p = provider.trim();
    let raw = p
        .strip_prefix("custom:")
        .or_else(|| p.strip_prefix("anthropic-custom:"))?;

    Some(raw.trim().to_string())
}

fn validate_http_url(raw: &str) -> Result<(), String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("URL is empty".to_string());
    }
    let parsed = reqwest::Url::parse(trimmed).map_err(|e| e.to_string())?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        other => Err(format!("unsupported URL scheme '{other}'")),
    }
}

fn check_config(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();

    out.push(if config.config_path.exists() {
        check_result(
            "config.file",
            HealthStatus::Healthy,
            format!("config file found at {}", config.config_path.display()),
            None,
        )
    } else {
        check_result(
            "config.file",
            HealthStatus::Warning,
            format!(
                "config file missing at {} (config may have been constructed programmatically)",
                config.config_path.display()
            ),
            None,
        )
    });

    out.push(if config.workspace_dir.exists() {
        check_result(
            "config.workspace_dir",
            HealthStatus::Healthy,
            format!("workspace dir exists: {}", config.workspace_dir.display()),
            None,
        )
    } else {
        check_result(
            "config.workspace_dir",
            HealthStatus::Critical,
            format!("workspace dir missing: {}", config.workspace_dir.display()),
            Some("Run `redclaw onboard` or create the directory.".to_string()),
        )
    });

    let provider = config
        .default_provider
        .as_deref()
        .map(str::trim)
        .unwrap_or("");
    out.push(if non_empty(provider) {
        check_result(
            "config.provider",
            HealthStatus::Healthy,
            format!("default_provider configured: {provider}"),
            None,
        )
    } else {
        check_result(
            "config.provider",
            HealthStatus::Critical,
            "default_provider is not set".to_string(),
            Some("Set `default_provider` (or env `REDCLAW_PROVIDER`).".to_string()),
        )
    });

    let configured_channels = count_configured_channels(&config.channels_config);
    out.push(if configured_channels > 0 {
        check_result(
            "config.channels",
            HealthStatus::Healthy,
            format!("{configured_channels} channel(s) configured"),
            None,
        )
    } else {
        check_result(
            "config.channels",
            HealthStatus::Critical,
            "no channels configured (cli=false and no channel configs present)".to_string(),
            Some("Enable `channels_config.cli = true` or configure a channel.".to_string()),
        )
    });

    out.push(if non_empty(&config.memory.backend) {
        check_result(
            "config.memory",
            HealthStatus::Healthy,
            format!("memory backend configured: {}", config.memory.backend),
            None,
        )
    } else {
        check_result(
            "config.memory",
            HealthStatus::Critical,
            "memory.backend is empty".to_string(),
            Some("Set `memory.backend` to sqlite/lucid/markdown/none.".to_string()),
        )
    });

    out.push(check_system_prompt_sources(config));
    out
}

fn check_system_prompt_sources(config: &Config) -> CheckResult {
    let workspace_dir = &config.workspace_dir;

    let aieos_configured = crate::identity::is_aieos_configured(&config.identity)
        && crate::identity::load_aieos_identity(&config.identity, workspace_dir)
            .ok()
            .flatten()
            .is_some();

    if aieos_configured {
        return check_result(
            "config.system_prompt",
            HealthStatus::Healthy,
            "AIEOS identity configured (system prompt will be generated from identity)".to_string(),
            None,
        );
    }

    let files = [
        "AGENTS.md",
        "SOUL.md",
        "TOOLS.md",
        "IDENTITY.md",
        "USER.md",
        "HEARTBEAT.md",
        "BOOTSTRAP.md",
        "MEMORY.md",
    ];

    let mut present = Vec::new();
    for file in files {
        let p = workspace_dir.join(file);
        if p.exists() {
            present.push(file);
        }
    }

    if present.is_empty() {
        check_result(
            "config.system_prompt",
            HealthStatus::Warning,
            "no workspace identity/prompt files found".to_string(),
            Some("Expected one of: AGENTS.md, SOUL.md, IDENTITY.md, USER.md, etc.".to_string()),
        )
    } else {
        check_result(
            "config.system_prompt",
            HealthStatus::Healthy,
            "workspace prompt files present".to_string(),
            Some(format!("Found: {}", present.join(", "))),
        )
    }
}

fn check_providers(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();

    let provider = config
        .default_provider
        .as_deref()
        .map(str::trim)
        .unwrap_or("");

    out.push(if is_provider_name_recognized(provider) {
        check_result(
            "provider.name",
            HealthStatus::Healthy,
            format!("provider recognized: {provider}"),
            None,
        )
    } else {
        check_result(
            "provider.name",
            if non_empty(provider) {
                HealthStatus::Critical
            } else {
                HealthStatus::Unknown
            },
            if non_empty(provider) {
                format!("unrecognized provider: {provider}")
            } else {
                "provider not configured".to_string()
            },
            Some(
                "See `redclaw models refresh --provider <name>` for supported providers."
                    .to_string(),
            ),
        )
    });

    if let Some(endpoint) = parse_provider_endpoint_if_custom(provider) {
        out.push(match validate_http_url(&endpoint) {
            Ok(()) => check_result(
                "provider.endpoint",
                HealthStatus::Healthy,
                "custom provider endpoint looks valid".to_string(),
                Some(endpoint),
            ),
            Err(e) => check_result(
                "provider.endpoint",
                HealthStatus::Critical,
                "custom provider endpoint is invalid".to_string(),
                Some(format!("{endpoint} ({e})")),
            ),
        });
    } else {
        out.push(check_result(
            "provider.endpoint",
            HealthStatus::Healthy,
            "provider uses built-in default endpoint".to_string(),
            None,
        ));
    }

    let model = config.default_model.as_deref().map(str::trim).unwrap_or("");
    out.push(if non_empty(model) {
        check_result(
            "provider.model",
            HealthStatus::Healthy,
            format!("default model set: {model}"),
            None,
        )
    } else {
        check_result(
            "provider.model",
            HealthStatus::Critical,
            "default_model is not set".to_string(),
            Some("Set `default_model` (or env `REDCLAW_MODEL`).".to_string()),
        )
    });

    out.push(if (0.0..=2.0).contains(&config.default_temperature) {
        check_result(
            "provider.temperature",
            HealthStatus::Healthy,
            format!("temperature ok: {}", config.default_temperature),
            None,
        )
    } else {
        check_result(
            "provider.temperature",
            HealthStatus::Warning,
            format!(
                "temperature out of expected range (0.0–2.0): {}",
                config.default_temperature
            ),
            None,
        )
    });

    let api_key_present = resolve_configured_api_key_present(provider, config);
    out.push(if provider_requires_api_key(provider) {
        if api_key_present {
            check_result(
                "provider.api_key",
                HealthStatus::Healthy,
                "API key present (config or env)".to_string(),
                None,
            )
        } else {
            check_result(
                "provider.api_key",
                HealthStatus::Critical,
                "API key missing".to_string(),
                Some(
                    "Set `api_key` or relevant provider env var (e.g. OPENROUTER_API_KEY)."
                        .to_string(),
                ),
            )
        }
    } else {
        check_result(
            "provider.api_key",
            HealthStatus::Healthy,
            "provider does not require an API key".to_string(),
            None,
        )
    });

    out
}

fn count_configured_channels(cfg: &crate::config::ChannelsConfig) -> usize {
    let mut n = 0usize;
    if cfg.cli {
        n += 1;
    }
    for present in [
        cfg.telegram.is_some(),
        cfg.discord.is_some(),
        cfg.slack.is_some(),
        cfg.webhook.is_some(),
        cfg.imessage.is_some(),
        cfg.matrix.is_some(),
        cfg.whatsapp.is_some(),
        cfg.email.is_some(),
        cfg.irc.is_some(),
        cfg.lark.is_some(),
        cfg.dingtalk.is_some(),
    ] {
        if present {
            n += 1;
        }
    }
    n
}

fn check_channels(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let cfg = &config.channels_config;

    out.push(if cfg.cli {
        check_result(
            "channels.cli",
            HealthStatus::Healthy,
            "CLI channel enabled".to_string(),
            None,
        )
    } else {
        check_result(
            "channels.cli",
            HealthStatus::Unknown,
            "CLI channel disabled".to_string(),
            None,
        )
    });

    if let Some(tg) = cfg.telegram.as_ref() {
        out.push(if non_empty(&tg.bot_token) {
            check_result(
                "channels.telegram.token",
                HealthStatus::Healthy,
                "telegram bot_token configured".to_string(),
                None,
            )
        } else {
            check_result(
                "channels.telegram.token",
                HealthStatus::Critical,
                "telegram bot_token is empty".to_string(),
                None,
            )
        });

        out.push(if tg.allowed_users.is_empty() {
            check_result(
                "channels.telegram.allowed_users",
                HealthStatus::Warning,
                "telegram allowed_users is empty (deny-all)".to_string(),
                Some("Add at least one username or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.telegram.allowed_users",
                HealthStatus::Healthy,
                format!("telegram allowlist entries: {}", tg.allowed_users.len()),
                None,
            )
        });
    }

    if let Some(dc) = cfg.discord.as_ref() {
        out.push(if non_empty(&dc.bot_token) {
            check_result(
                "channels.discord.token",
                HealthStatus::Healthy,
                "discord bot_token configured".to_string(),
                None,
            )
        } else {
            check_result(
                "channels.discord.token",
                HealthStatus::Critical,
                "discord bot_token is empty".to_string(),
                None,
            )
        });

        out.push(if dc.allowed_users.is_empty() {
            check_result(
                "channels.discord.allowed_users",
                HealthStatus::Warning,
                "discord allowed_users is empty (deny-all)".to_string(),
                Some("Add at least one user ID or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.discord.allowed_users",
                HealthStatus::Healthy,
                format!("discord allowlist entries: {}", dc.allowed_users.len()),
                None,
            )
        });
    }

    if let Some(sl) = cfg.slack.as_ref() {
        out.push(if non_empty(&sl.bot_token) {
            check_result(
                "channels.slack.token",
                HealthStatus::Healthy,
                "slack bot_token configured".to_string(),
                None,
            )
        } else {
            check_result(
                "channels.slack.token",
                HealthStatus::Critical,
                "slack bot_token is empty".to_string(),
                None,
            )
        });

        out.push(if sl.allowed_users.is_empty() {
            check_result(
                "channels.slack.allowed_users",
                HealthStatus::Warning,
                "slack allowed_users is empty (deny-all)".to_string(),
                Some("Add at least one user ID or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.slack.allowed_users",
                HealthStatus::Healthy,
                format!("slack allowlist entries: {}", sl.allowed_users.len()),
                None,
            )
        });

        out.push(
            if sl
                .channel_id
                .as_deref()
                .map(str::trim)
                .is_some_and(|s| !s.is_empty())
            {
                check_result(
                    "channels.slack.channel_id",
                    HealthStatus::Healthy,
                    "slack channel_id configured".to_string(),
                    None,
                )
            } else {
                check_result(
                    "channels.slack.channel_id",
                    HealthStatus::Warning,
                    "slack channel_id missing (listen/send may not work as expected)".to_string(),
                    None,
                )
            },
        );
    }

    if let Some(wh) = cfg.webhook.as_ref() {
        out.push(if wh.port > 0 {
            check_result(
                "channels.webhook.port",
                HealthStatus::Healthy,
                format!("webhook port configured: {}", wh.port),
                None,
            )
        } else {
            check_result(
                "channels.webhook.port",
                HealthStatus::Critical,
                "webhook port is 0".to_string(),
                None,
            )
        });
        out.push(
            if wh
                .secret
                .as_deref()
                .map(str::trim)
                .is_some_and(|s| !s.is_empty())
            {
                check_result(
                    "channels.webhook.secret",
                    HealthStatus::Healthy,
                    "webhook secret configured".to_string(),
                    None,
                )
            } else {
                check_result(
                    "channels.webhook.secret",
                    HealthStatus::Warning,
                    "webhook secret missing (unauthenticated webhooks)".to_string(),
                    None,
                )
            },
        );
    }

    if let Some(mx) = cfg.matrix.as_ref() {
        out.push(
            if non_empty(&mx.homeserver) && non_empty(&mx.access_token) && non_empty(&mx.room_id) {
                check_result(
                    "channels.matrix.core",
                    HealthStatus::Healthy,
                    "matrix homeserver/access_token/room_id configured".to_string(),
                    None,
                )
            } else {
                check_result(
                    "channels.matrix.core",
                    HealthStatus::Critical,
                    "matrix homeserver/access_token/room_id missing".to_string(),
                    None,
                )
            },
        );

        out.push(if mx.allowed_users.is_empty() {
            check_result(
                "channels.matrix.allowed_users",
                HealthStatus::Warning,
                "matrix allowed_users is empty (deny-all)".to_string(),
                Some("Add at least one user ID or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.matrix.allowed_users",
                HealthStatus::Healthy,
                format!("matrix allowlist entries: {}", mx.allowed_users.len()),
                None,
            )
        });
    }

    if let Some(wa) = cfg.whatsapp.as_ref() {
        out.push(
            if non_empty(&wa.access_token)
                && non_empty(&wa.phone_number_id)
                && non_empty(&wa.verify_token)
            {
                check_result(
                    "channels.whatsapp.core",
                    HealthStatus::Healthy,
                    "whatsapp access_token/phone_number_id/verify_token configured".to_string(),
                    None,
                )
            } else {
                check_result(
                    "channels.whatsapp.core",
                    HealthStatus::Critical,
                    "whatsapp required fields missing".to_string(),
                    None,
                )
            },
        );

        let app_secret_present = wa
            .app_secret
            .as_deref()
            .map(str::trim)
            .is_some_and(|s| !s.is_empty())
            || std::env::var("REDCLAW_WHATSAPP_APP_SECRET")
                .ok()
                .is_some_and(|v| non_empty(&v));

        out.push(if app_secret_present {
            check_result(
                "channels.whatsapp.app_secret",
                HealthStatus::Healthy,
                "whatsapp app_secret present (config or env)".to_string(),
                None,
            )
        } else {
            check_result(
                "channels.whatsapp.app_secret",
                HealthStatus::Warning,
                "whatsapp app_secret missing (signature verification may fail)".to_string(),
                Some(
                    "Set WhatsApp app secret in config or REDCLAW_WHATSAPP_APP_SECRET".to_string(),
                ),
            )
        });

        out.push(if wa.allowed_numbers.is_empty() {
            check_result(
                "channels.whatsapp.allowed_numbers",
                HealthStatus::Warning,
                "whatsapp allowed_numbers is empty (deny-all)".to_string(),
                Some("Add at least one E.164 number or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.whatsapp.allowed_numbers",
                HealthStatus::Healthy,
                format!("whatsapp allowlist entries: {}", wa.allowed_numbers.len()),
                None,
            )
        });
    }

    if let Some(em) = cfg.email.as_ref() {
        out.push(
            if non_empty(&em.imap_host)
                && non_empty(&em.smtp_host)
                && non_empty(&em.username)
                && non_empty(&em.password)
                && non_empty(&em.from_address)
            {
                check_result(
                    "channels.email.core",
                    HealthStatus::Healthy,
                    "email channel core config present".to_string(),
                    None,
                )
            } else {
                check_result(
                    "channels.email.core",
                    HealthStatus::Critical,
                    "email channel missing imap/smtp/credentials/from_address".to_string(),
                    None,
                )
            },
        );

        out.push(if em.allowed_senders.is_empty() {
            check_result(
                "channels.email.allowed_senders",
                HealthStatus::Warning,
                "email allowed_senders is empty (deny-all)".to_string(),
                Some("Add at least one sender/domain or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.email.allowed_senders",
                HealthStatus::Healthy,
                format!("email allowlist entries: {}", em.allowed_senders.len()),
                None,
            )
        });
    }

    if let Some(irc) = cfg.irc.as_ref() {
        out.push(if non_empty(&irc.server) && non_empty(&irc.nickname) {
            check_result(
                "channels.irc.core",
                HealthStatus::Healthy,
                "IRC server and nickname configured".to_string(),
                None,
            )
        } else {
            check_result(
                "channels.irc.core",
                HealthStatus::Critical,
                "IRC server/nickname missing".to_string(),
                None,
            )
        });

        out.push(if irc.channels.is_empty() {
            check_result(
                "channels.irc.channels",
                HealthStatus::Warning,
                "IRC channels list empty (will not join any channels)".to_string(),
                None,
            )
        } else {
            check_result(
                "channels.irc.channels",
                HealthStatus::Healthy,
                format!("IRC channels configured: {}", irc.channels.len()),
                None,
            )
        });
    }

    if let Some(lark) = cfg.lark.as_ref() {
        out.push(if non_empty(&lark.app_id) && non_empty(&lark.app_secret) {
            check_result(
                "channels.lark.core",
                HealthStatus::Healthy,
                "lark app_id/app_secret configured".to_string(),
                None,
            )
        } else {
            check_result(
                "channels.lark.core",
                HealthStatus::Critical,
                "lark app_id/app_secret missing".to_string(),
                None,
            )
        });

        out.push(if lark.allowed_users.is_empty() {
            check_result(
                "channels.lark.allowed_users",
                HealthStatus::Warning,
                "lark allowed_users is empty (deny-all)".to_string(),
                Some("Add at least one user ID/union ID or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.lark.allowed_users",
                HealthStatus::Healthy,
                format!("lark allowlist entries: {}", lark.allowed_users.len()),
                None,
            )
        });
    }

    if let Some(dt) = cfg.dingtalk.as_ref() {
        out.push(
            if non_empty(&dt.client_id) && non_empty(&dt.client_secret) {
                check_result(
                    "channels.dingtalk.core",
                    HealthStatus::Healthy,
                    "dingtalk client_id/client_secret configured".to_string(),
                    None,
                )
            } else {
                check_result(
                    "channels.dingtalk.core",
                    HealthStatus::Critical,
                    "dingtalk client_id/client_secret missing".to_string(),
                    None,
                )
            },
        );

        out.push(if dt.allowed_users.is_empty() {
            check_result(
                "channels.dingtalk.allowed_users",
                HealthStatus::Warning,
                "dingtalk allowed_users is empty (deny-all)".to_string(),
                Some("Add at least one staff ID or '*'".to_string()),
            )
        } else {
            check_result(
                "channels.dingtalk.allowed_users",
                HealthStatus::Healthy,
                format!("dingtalk allowlist entries: {}", dt.allowed_users.len()),
                None,
            )
        });
    }

    // No duplicates are possible with the current config shape (each channel type is a single optional section).
    out.push(check_result(
        "channels.duplicates",
        HealthStatus::Healthy,
        "no duplicate channel types detected".to_string(),
        None,
    ));

    out
}

fn check_memory(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let backend_raw = config.memory.backend.trim();
    let kind = crate::memory::classify_memory_backend(backend_raw);

    out.push(match kind {
        MemoryBackendKind::Sqlite | MemoryBackendKind::Lucid | MemoryBackendKind::Markdown => {
            check_result(
                "memory.backend",
                HealthStatus::Healthy,
                format!("memory backend recognized: {backend_raw}"),
                None,
            )
        }
        MemoryBackendKind::Postgres => check_result(
            "memory.backend",
            HealthStatus::Healthy,
            format!("memory backend recognized: {backend_raw}"),
            None,
        ),
        MemoryBackendKind::None => check_result(
            "memory.backend",
            HealthStatus::Warning,
            "memory backend is 'none' (persistence disabled)".to_string(),
            None,
        ),
        MemoryBackendKind::Unknown => check_result(
            "memory.backend",
            HealthStatus::Warning,
            format!("unknown memory backend '{backend_raw}' (will fall back at runtime)"),
            None,
        ),
    });

    let workspace_ok = config.workspace_dir.exists();
    out.push(if workspace_ok {
        check_result(
            "memory.workspace",
            HealthStatus::Healthy,
            "workspace directory present".to_string(),
            None,
        )
    } else {
        check_result(
            "memory.workspace",
            HealthStatus::Critical,
            "workspace directory missing".to_string(),
            Some(config.workspace_dir.display().to_string()),
        )
    });

    // Non-invasive write check: can we create a small file in workspace?
    out.push(
        match can_write_probe_file(&config.workspace_dir, ".doctor_write_probe") {
            Ok(()) => check_result(
                "memory.fs_write",
                HealthStatus::Healthy,
                "workspace appears writable".to_string(),
                None,
            ),
            Err(e) => check_result(
                "memory.fs_write",
                HealthStatus::Critical,
                "workspace is not writable".to_string(),
                Some(e),
            ),
        },
    );

    // Backend-specific storage path checks.
    let memory_dir = config.workspace_dir.join("memory");
    out.push(if memory_dir.exists() {
        check_result(
            "memory.storage_dir",
            HealthStatus::Healthy,
            format!("memory dir exists: {}", memory_dir.display()),
            None,
        )
    } else {
        check_result(
            "memory.storage_dir",
            HealthStatus::Warning,
            format!(
                "memory dir missing: {} (will be created on first run)",
                memory_dir.display()
            ),
            None,
        )
    });

    if matches!(kind, MemoryBackendKind::Sqlite | MemoryBackendKind::Lucid) {
        let db_path = config.workspace_dir.join("memory").join("brain.db");
        out.push(if db_path.exists() {
            match std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&db_path)
            {
                Ok(_) => check_result(
                    "memory.sqlite",
                    HealthStatus::Healthy,
                    "sqlite brain.db is accessible".to_string(),
                    Some(db_path.display().to_string()),
                ),
                Err(e) => check_result(
                    "memory.sqlite",
                    HealthStatus::Critical,
                    "sqlite brain.db is not accessible".to_string(),
                    Some(format!("{} ({e})", db_path.display())),
                ),
            }
        } else {
            check_result(
                "memory.sqlite",
                HealthStatus::Warning,
                "sqlite brain.db not found (will be created on first run)".to_string(),
                Some(db_path.display().to_string()),
            )
        });
    }

    // Embeddings: warn if configured but no API key is present.
    let embed_provider = config.memory.embedding_provider.trim();
    if embed_provider == "none" {
        out.push(check_result(
            "memory.embeddings",
            HealthStatus::Healthy,
            "embedding_provider is 'none'".to_string(),
            None,
        ));
    } else {
        let key_present = resolve_configured_api_key_present(
            config.default_provider.as_deref().unwrap_or(""),
            config,
        );
        out.push(if key_present {
            check_result(
                "memory.embeddings",
                HealthStatus::Healthy,
                format!("embedding_provider enabled: {embed_provider}"),
                None,
            )
        } else {
            check_result(
                "memory.embeddings",
                HealthStatus::Warning,
                format!("embedding_provider '{embed_provider}' may require an API key"),
                Some("Set `api_key` or provider env var if embeddings fail.".to_string()),
            )
        });
    }

    // Weight sanity (not fatal, but helps avoid confusing search behavior).
    if !(0.0..=1.0).contains(&config.memory.vector_weight)
        || !(0.0..=1.0).contains(&config.memory.keyword_weight)
    {
        out.push(check_result(
            "memory.search_weights",
            HealthStatus::Warning,
            "vector/keyword weights should be within 0.0–1.0".to_string(),
            Some(format!(
                "vector_weight={}, keyword_weight={}",
                config.memory.vector_weight, config.memory.keyword_weight
            )),
        ));
    } else {
        out.push(check_result(
            "memory.search_weights",
            HealthStatus::Healthy,
            "search weights in range".to_string(),
            Some(format!(
                "vector_weight={}, keyword_weight={}",
                config.memory.vector_weight, config.memory.keyword_weight
            )),
        ));
    }

    out
}

fn check_security(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();

    out.push(if config.autonomy.allowed_commands.is_empty() {
        check_result(
            "security.allowed_commands",
            HealthStatus::Critical,
            "allowed_commands is empty".to_string(),
            Some("Populate `autonomy.allowed_commands` to permit tool execution.".to_string()),
        )
    } else {
        check_result(
            "security.allowed_commands",
            HealthStatus::Healthy,
            format!(
                "allowed_commands entries: {}",
                config.autonomy.allowed_commands.len()
            ),
            None,
        )
    });

    out.push(
        if config.gateway.require_pairing && config.gateway.paired_tokens.is_empty() {
            check_result(
                "security.gateway_pairing",
                HealthStatus::Warning,
                "gateway pairing required but no paired tokens found".to_string(),
                Some(
                    "Run `redclaw gateway` and pair a client, or disable require_pairing."
                        .to_string(),
                ),
            )
        } else if config.gateway.require_pairing {
            check_result(
                "security.gateway_pairing",
                HealthStatus::Healthy,
                format!("paired tokens: {}", config.gateway.paired_tokens.len()),
                None,
            )
        } else {
            check_result(
                "security.gateway_pairing",
                HealthStatus::Warning,
                "gateway pairing disabled".to_string(),
                Some("Consider enabling require_pairing for safer defaults.".to_string()),
            )
        },
    );

    out.push(
        if !is_localhost(&config.gateway.host) && !config.gateway.allow_public_bind {
            check_result(
                "security.gateway_bind",
                HealthStatus::Critical,
                format!(
                    "gateway host is public ('{}') but allow_public_bind=false",
                    config.gateway.host
                ),
                Some("Set REDCLAW_ALLOW_PUBLIC_BIND=1 or configure a tunnel.".to_string()),
            )
        } else {
            check_result(
                "security.gateway_bind",
                HealthStatus::Healthy,
                format!(
                    "gateway bind ok: {}:{}",
                    config.gateway.host, config.gateway.port
                ),
                None,
            )
        },
    );

    out.push(if config.autonomy.workspace_only {
        check_result(
            "security.workspace_only",
            HealthStatus::Healthy,
            "workspace_only enabled".to_string(),
            None,
        )
    } else {
        check_result(
            "security.workspace_only",
            HealthStatus::Warning,
            "workspace_only disabled".to_string(),
            Some("Disabling workspace_only increases blast radius for file tools.".to_string()),
        )
    });

    out.push(
        match (
            config.autonomy.level,
            config.autonomy.block_high_risk_commands,
        ) {
            (crate::security::AutonomyLevel::Full, false) => check_result(
                "security.risk_controls",
                HealthStatus::Warning,
                "full autonomy with block_high_risk_commands=false".to_string(),
                Some("Consider enabling block_high_risk_commands for safer defaults.".to_string()),
            ),
            (_, _) => check_result(
                "security.risk_controls",
                HealthStatus::Healthy,
                "risk controls configured".to_string(),
                None,
            ),
        },
    );

    out
}

fn resolve_cost_log_path(config: &Config) -> PathBuf {
    if let Some(raw) = config
        .cost
        .log_path
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let p = PathBuf::from(raw);
        return if p.is_absolute() {
            p
        } else {
            config.workspace_dir.join(p)
        };
    }

    // Keep it simple for doctor: match the common runtime default.
    config.workspace_dir.join("state").join("costs.jsonl")
}

fn check_cost(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();

    out.push(if config.cost.enabled {
        check_result(
            "cost.enabled",
            HealthStatus::Healthy,
            "cost tracking enabled".to_string(),
            None,
        )
    } else {
        check_result(
            "cost.enabled",
            HealthStatus::Unknown,
            "cost tracking disabled".to_string(),
            None,
        )
    });

    if config.cost.enabled {
        let daily = config.cost.effective_daily_limit_usd();
        let monthly = config.cost.monthly_limit_usd;
        let session = config.cost.session_budget_usd;
        let per_req = config.cost.per_request_budget_usd;

        out.push(if daily <= 0.0 && monthly <= 0.0 && session <= 0.0 && per_req <= 0.0 {
            check_result(
                "cost.budgets",
                HealthStatus::Warning,
                "no budget limits configured (all limits are 0/unlimited)".to_string(),
                Some("Set daily_budget_usd/monthly_limit_usd/session_budget_usd/per_request_budget_usd.".to_string()),
            )
        } else {
            check_result(
                "cost.budgets",
                HealthStatus::Healthy,
                "budget limits configured".to_string(),
                Some(format!(
                    "daily_limit_usd={daily}, monthly_limit_usd={monthly}, session_budget_usd={session}, per_request_budget_usd={per_req}"
                )),
            )
        });

        out.push(if config.cost.auto_pause {
            check_result(
                "cost.auto_pause",
                HealthStatus::Healthy,
                "auto_pause enabled".to_string(),
                None,
            )
        } else {
            check_result(
                "cost.auto_pause",
                HealthStatus::Warning,
                "auto_pause disabled".to_string(),
                Some("Agent will not automatically pause when exceeding budget.".to_string()),
            )
        });

        let log_path = resolve_cost_log_path(config);
        out.push(check_result(
            "cost.log_path",
            HealthStatus::Healthy,
            "cost log path resolved".to_string(),
            Some(log_path.display().to_string()),
        ));
        out.push(match check_parent_dir_exists(&log_path) {
            Ok(()) => check_result(
                "cost.log_parent_dir",
                HealthStatus::Healthy,
                "cost log parent directory exists".to_string(),
                Some(parent_display(&log_path)),
            ),
            Err(e) => check_result(
                "cost.log_parent_dir",
                HealthStatus::Warning,
                "cost log parent directory missing".to_string(),
                Some(e),
            ),
        });
    }

    out
}

fn parent_display(path: &Path) -> String {
    path.parent()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<no parent>".to_string())
}

fn check_parent_dir_exists(path: &Path) -> Result<(), String> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if parent.exists() {
        Ok(())
    } else {
        Err(format!("missing: {}", parent.display()))
    }
}

fn check_policy(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();

    let tool_rules = config.policy.tools.len();
    let channel_rules = config.policy.channels.len();
    out.push(check_result(
        "policy.summary",
        HealthStatus::Healthy,
        "policy config loaded".to_string(),
        Some(format!(
            "default_action={:?}, tool_rules={tool_rules}, channel_rules={channel_rules}",
            config.policy.default_action
        )),
    ));

    if tool_rules == 0 && channel_rules == 0 {
        out.push(check_result(
            "policy.rules",
            HealthStatus::Warning,
            "no policy rules configured".to_string(),
            Some(format!(
                "default_action={:?} (this may be overly permissive)",
                config.policy.default_action
            )),
        ));
        return out;
    }

    // Detect obvious footguns.
    let mut any_allow = false;
    for tool in config.policy.tools.values() {
        if tool.action != crate::policy::PolicyAction::Deny {
            any_allow = true;
            break;
        }
    }
    if !any_allow && config.policy.default_action == crate::policy::PolicyAction::Deny {
        out.push(check_result(
            "policy.deny_all",
            HealthStatus::Warning,
            "policy default_action=deny and no allow/audit tool rules found".to_string(),
            Some("This can effectively deny all tool calls.".to_string()),
        ));
    } else {
        out.push(check_result(
            "policy.deny_all",
            HealthStatus::Healthy,
            "policy allows some tool calls (or default_action is not deny)".to_string(),
            None,
        ));
    }

    // Spot wildcard allowlists at channel level.
    let mut wildcard_channels = Vec::new();
    for (name, ch) in &config.policy.channels {
        if ch
            .allowed_tools
            .as_ref()
            .is_some_and(|tools| tools.iter().any(|t| t == "*"))
        {
            wildcard_channels.push(name.clone());
        }
    }

    out.push(if wildcard_channels.is_empty() {
        check_result(
            "policy.wildcards",
            HealthStatus::Healthy,
            "no channel allowlist wildcards detected".to_string(),
            None,
        )
    } else {
        check_result(
            "policy.wildcards",
            HealthStatus::Warning,
            "channel policy allows '*' tools".to_string(),
            Some(format!("channels: {}", wildcard_channels.join(", "))),
        )
    });

    out
}

fn check_environment(config: &Config) -> Vec<CheckResult> {
    let mut out = Vec::new();

    // Only treat ~/.redclaw as required when the config path is the default layout.
    let home = directories::UserDirs::new()
        .map(|u| u.home_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let default_redclaw_dir = home.join(".redclaw");
    let using_default_dir = config
        .config_path
        .parent()
        .is_some_and(|p| p == default_redclaw_dir);

    out.push(if using_default_dir {
        if default_redclaw_dir.exists() {
            check_result(
                "env.home_dir",
                HealthStatus::Healthy,
                format!("RedClaw home exists: {}", default_redclaw_dir.display()),
                None,
            )
        } else {
            check_result(
                "env.home_dir",
                HealthStatus::Warning,
                format!("RedClaw home missing: {}", default_redclaw_dir.display()),
                Some(
                    "On first run, this is created automatically by config init/onboard."
                        .to_string(),
                ),
            )
        }
    } else {
        check_result(
            "env.home_dir",
            HealthStatus::Healthy,
            "non-default config path in use; ~/.redclaw existence not required".to_string(),
            Some(format!("config_path={}", config.config_path.display())),
        )
    });

    out.push(if config.workspace_dir.exists() {
        check_result(
            "env.workspace_dir",
            HealthStatus::Healthy,
            format!("workspace exists: {}", config.workspace_dir.display()),
            None,
        )
    } else {
        check_result(
            "env.workspace_dir",
            HealthStatus::Critical,
            format!("workspace missing: {}", config.workspace_dir.display()),
            None,
        )
    });

    out.push(match std::env::var("REDCLAW_WORKSPACE") {
        Ok(ws) if non_empty(&ws) => {
            let p = PathBuf::from(ws.trim());
            if p.exists() {
                check_result(
                    "env.REDCLAW_WORKSPACE",
                    HealthStatus::Healthy,
                    "REDCLAW_WORKSPACE set".to_string(),
                    Some(p.display().to_string()),
                )
            } else {
                check_result(
                    "env.REDCLAW_WORKSPACE",
                    HealthStatus::Warning,
                    "REDCLAW_WORKSPACE points to missing path".to_string(),
                    Some(p.display().to_string()),
                )
            }
        }
        Ok(_) => check_result(
            "env.REDCLAW_WORKSPACE",
            HealthStatus::Warning,
            "REDCLAW_WORKSPACE is set but empty".to_string(),
            None,
        ),
        Err(_) => check_result(
            "env.REDCLAW_WORKSPACE",
            HealthStatus::Healthy,
            "REDCLAW_WORKSPACE not set".to_string(),
            None,
        ),
    });

    out.push(check_env_var_presence(
        "env.REDCLAW_API_KEY",
        "REDCLAW_API_KEY",
    ));
    out.push(check_env_var_presence(
        "env.REDCLAW_PROVIDER",
        "REDCLAW_PROVIDER",
    ));
    out.push(check_env_var_presence("env.REDCLAW_MODEL", "REDCLAW_MODEL"));
    out
}

fn check_env_var_presence(check_name: &str, var: &str) -> CheckResult {
    match std::env::var(var) {
        Ok(v) if non_empty(&v) => check_result(
            check_name,
            HealthStatus::Healthy,
            format!("{var} is set"),
            None,
        ),
        Ok(_) => check_result(
            check_name,
            HealthStatus::Warning,
            format!("{var} is set but empty"),
            None,
        ),
        Err(_) => check_result(
            check_name,
            HealthStatus::Healthy,
            format!("{var} not set"),
            None,
        ),
    }
}

fn can_write_probe_file(dir: &Path, filename: &str) -> Result<(), String> {
    if !dir.exists() {
        return Err(format!("directory missing: {}", dir.display()));
    }

    let probe = dir.join(filename);
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&probe)
    {
        Ok(mut f) => {
            use std::io::Write;
            let _ = f.write_all(b"probe");
            let _ = std::fs::remove_file(&probe);
            Ok(())
        }
        Err(e) => Err(format!("{} ({e})", probe.display())),
    }
}

/// Run all diagnostic checks
pub fn run_diagnostics(config: &Config) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    for check in check_config(config) {
        report.add_check(check);
    }
    for check in check_providers(config) {
        report.add_check(check);
    }
    for check in check_channels(config) {
        report.add_check(check);
    }
    for check in check_memory(config) {
        report.add_check(check);
    }
    for check in check_security(config) {
        report.add_check(check);
    }
    for check in check_cost(config) {
        report.add_check(check);
    }
    for check in check_policy(config) {
        report.add_check(check);
    }
    for check in check_environment(config) {
        report.add_check(check);
    }

    report.finalize();
    report
}

// Legacy entrypoint used by older callsites.
// Main now prefers `run_diagnostics` and handles exit codes.
pub fn run(config: &Config) -> anyhow::Result<()> {
    let report = run_diagnostics(config);
    println!("{report}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ChannelsConfig, Config, CostConfig, TelegramConfig};
    use crate::policy::{PolicyAction, PolicyConfig, ToolPolicy};
    use tempfile::TempDir;

    fn tmp_config(tmp: &TempDir) -> Config {
        let mut config = Config::default();
        config.workspace_dir = tmp.path().join("workspace");
        config.config_path = tmp.path().join("config.toml");
        config
    }

    #[test]
    fn report_summary_counts() {
        let mut report = DiagnosticReport::new();
        report.add_check(check_result("a", HealthStatus::Healthy, "ok", None));
        report.add_check(check_result("b", HealthStatus::Warning, "warn", None));
        report.add_check(check_result("c", HealthStatus::Critical, "bad", None));
        report.finalize();

        assert_eq!(report.summary.total, 3);
        assert_eq!(report.summary.healthy, 1);
        assert_eq!(report.summary.warnings, 1);
        assert_eq!(report.summary.critical, 1);
        assert_eq!(report.overall, HealthStatus::Critical);
    }

    #[test]
    fn minimal_config_produces_warnings_but_not_panic() {
        let tmp = TempDir::new().unwrap();
        let mut config = tmp_config(&tmp);
        std::fs::create_dir_all(&config.workspace_dir).unwrap();
        std::fs::write(config.workspace_dir.join("AGENTS.md"), "# Agents\n").unwrap();

        // Minimal but runnable: use ollama to avoid requiring an API key.
        config.default_provider = Some("ollama".to_string());
        config.default_model = Some("llama3".to_string());
        config.channels_config = ChannelsConfig::default();
        config.channels_config.cli = true;

        let report = run_diagnostics(&config);

        assert_ne!(report.summary.total, 0);
        assert!(matches!(
            report.overall,
            HealthStatus::Warning | HealthStatus::Unknown
        ));

        // Display formatting should be stable.
        let rendered = format!("{report}");
        assert!(rendered.contains("RedClaw Doctor"));
        assert!(rendered.contains("Summary"));
    }

    #[test]
    fn full_config_can_be_healthy() {
        let tmp = TempDir::new().unwrap();
        let mut config = tmp_config(&tmp);
        std::fs::write(&config.config_path, "# test config\n").unwrap();
        std::fs::create_dir_all(&config.workspace_dir).unwrap();
        std::fs::create_dir_all(config.workspace_dir.join("memory")).unwrap();
        std::fs::write(config.workspace_dir.join("AGENTS.md"), "# Agents\n").unwrap();
        std::fs::write(config.workspace_dir.join("memory").join("brain.db"), "").unwrap();

        config.default_provider = Some("openrouter".to_string());
        config.api_key = Some("sk-test".to_string());
        config.default_model = Some("anthropic/claude-sonnet-4-20250514".to_string());

        // At least one non-cli channel configured.
        config.channels_config.cli = true;
        config.channels_config.telegram = Some(TelegramConfig {
            bot_token: "tg-token".to_string(),
            allowed_users: vec!["*".to_string()],
            stream_mode: crate::config::StreamMode::default(),
            draft_update_interval_ms: 1000,
            mention_only: false,
        });

        // Cost: set a real limit and a log path whose parent exists.
        let mut cost = CostConfig::default();
        cost.enabled = true;
        cost.daily_budget_usd = 5.0;
        cost.auto_pause = true;
        cost.log_path = Some("state/costs.jsonl".to_string());
        config.cost = cost;
        std::fs::create_dir_all(config.workspace_dir.join("state")).unwrap();

        // Gateway pairing: mark paired token present.
        config.gateway.require_pairing = true;
        config.gateway.paired_tokens = vec!["paired-token".to_string()];
        config.gateway.host = "127.0.0.1".to_string();

        // Policy: add at least one explicit rule.
        let mut policy = PolicyConfig::default();
        policy.default_action = PolicyAction::Deny;
        policy.tools.insert(
            "memory_read".to_string(),
            ToolPolicy {
                action: PolicyAction::Allow,
                allowed_channels: None,
                denied_channels: None,
                require_confirmation: false,
                rate_limit: None,
                reason: None,
            },
        );
        config.policy = policy;

        let report = run_diagnostics(&config);

        let non_healthy: Vec<String> = report
            .checks
            .iter()
            .filter(|c| c.status != HealthStatus::Healthy)
            .map(|c| format!("{}: {:?} — {}", c.name, c.status, c.message))
            .collect();
        assert!(
            non_healthy.is_empty(),
            "expected fully healthy report; got: {}\n\n{report}",
            non_healthy.join(" | ")
        );

        assert_eq!(report.overall, HealthStatus::Healthy);
        assert_eq!(report.summary.critical, 0);
        assert_eq!(report.summary.warnings, 0);
    }

    #[test]
    fn provider_custom_url_validation() {
        let tmp = TempDir::new().unwrap();
        let mut config = tmp_config(&tmp);
        std::fs::create_dir_all(&config.workspace_dir).unwrap();
        std::fs::write(config.workspace_dir.join("AGENTS.md"), "# Agents\n").unwrap();
        config.default_provider = Some("custom:https://example.com".to_string());
        config.default_model = Some("gpt-4o".to_string());
        config.api_key = Some("sk-test".to_string());

        let checks = check_providers(&config);
        let endpoint = checks
            .iter()
            .find(|c| c.name == "provider.endpoint")
            .expect("endpoint check missing");
        assert_eq!(endpoint.status, HealthStatus::Healthy);
    }

    #[test]
    fn display_includes_status_symbols() {
        let mut report = DiagnosticReport::new();
        report.add_check(check_result("ok", HealthStatus::Healthy, "ok", None));
        report.add_check(check_result("warn", HealthStatus::Warning, "warn", None));
        report.finalize();
        let rendered = format!("{report}");
        assert!(rendered.contains('✓'));
        assert!(rendered.contains('⚠'));
    }
}
