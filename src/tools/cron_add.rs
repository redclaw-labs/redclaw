use super::traits::{Tool, ToolResult};
use crate::config::Config;
use crate::cron;
use crate::security::SecurityPolicy;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::json;
use std::sync::Arc;

/// Add a cron job (shell-based) via the tool surface.
///
/// Note: this implementation targets the existing `crate::cron` backend.
pub struct CronAddTool {
    config: Arc<Config>,
    security: Arc<SecurityPolicy>,
}

impl CronAddTool {
    pub fn new(config: Arc<Config>, security: Arc<SecurityPolicy>) -> Self {
        Self { config, security }
    }

    fn enforce_mutation_allowed(&self, action: &str) -> Option<ToolResult> {
        if !self.security.can_act() {
            return Some(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!(
                    "Security policy: read-only mode, cannot perform '{action}'"
                )),
            });
        }

        if self.security.is_rate_limited() {
            return Some(ToolResult {
                success: false,
                output: String::new(),
                error: Some("Rate limit exceeded: too many actions in the last hour".to_string()),
            });
        }

        if !self.security.record_action() {
            return Some(ToolResult {
                success: false,
                output: String::new(),
                error: Some("Rate limit exceeded: action budget exhausted".to_string()),
            });
        }

        None
    }
}

#[async_trait]
impl Tool for CronAddTool {
    fn name(&self) -> &str {
        "cron_add"
    }

    fn description(&self) -> &str {
        "Create a scheduled task (shell command). Supports cron expressions and one-shot schedules."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "schedule": {
                    "type": "object",
                    "additionalProperties": false,
                    "properties": {
                        "kind": {"type": "string", "enum": ["cron", "once", "at", "every_ms"]},
                        "expr": {"type": "string"},
                        "delay": {"type": "string"},
                        "at": {"type": "string"},
                        "every_ms": {"type": "integer", "minimum": 1}
                    },
                    "required": ["kind"]
                },
                "command": {"type": "string"},
                "approved": {
                    "type": "boolean",
                    "description": "Set true to explicitly approve medium/high-risk shell commands in supervised mode",
                    "default": false
                }
            },
            "required": ["schedule", "command"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        if let Some(blocked) = self.enforce_mutation_allowed("cron_add") {
            return Ok(blocked);
        }

        let schedule = args
            .get("schedule")
            .ok_or_else(|| anyhow::anyhow!("Missing 'schedule' parameter"))?;

        let command = args
            .get("command")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .ok_or_else(|| anyhow::anyhow!("Missing 'command' parameter"))?;

        if command.is_empty() {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some("'command' must not be empty".to_string()),
            });
        }

        let approved = args
            .get("approved")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        if let Err(reason) = self.security.validate_command_execution(command, approved) {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(reason),
            });
        }

        let kind = schedule
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or("cron");

        let job = match kind {
            "cron" => {
                let expr = schedule
                    .get("expr")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing schedule.expr for kind=cron"))?;
                cron::add_job(&self.config, expr, command)?
            }
            "once" => {
                let delay = schedule
                    .get("delay")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing schedule.delay for kind=once"))?;
                cron::add_once(&self.config, delay, command)?
            }
            "at" => {
                let at = schedule
                    .get("at")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing schedule.at for kind=at"))?;
                let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(at)
                    .map_err(|e| anyhow::anyhow!("Invalid RFC3339 timestamp for schedule.at: {e}"))?
                    .with_timezone(&Utc);
                cron::add_once_at(&self.config, parsed, command)?
            }
            "every_ms" => {
                let every_ms = schedule
                    .get("every_ms")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| anyhow::anyhow!("Missing schedule.every_ms for kind=every_ms"))?;
                cron::add_every_ms(&self.config, every_ms, command)?
            }
            other => {
                return Ok(ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some(format!(
                        "Unknown schedule.kind '{other}'. Use cron|once|at|every_ms."
                    )),
                });
            }
        };

        Ok(ToolResult {
            success: true,
            output: json!({
                "id": job.id,
                "expression": job.expression,
                "next_run": job.next_run.to_rfc3339(),
                "paused": job.paused,
                "one_shot": job.one_shot,
            })
            .to_string(),
            error: None,
        })
    }
}
