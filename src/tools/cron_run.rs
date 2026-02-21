use super::traits::{Tool, ToolResult};
use crate::config::Config;
use crate::cron;
use crate::security::SecurityPolicy;
use async_trait::async_trait;
use serde_json::json;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::time::{self, Duration};

const RUN_TIMEOUT_SECS: u64 = 120;

/// Run an existing cron job immediately by id.
///
/// This executes the stored shell command once and persists the result using
/// `cron::reschedule_after_run`.
pub struct CronRunTool {
    config: Arc<Config>,
    security: Arc<SecurityPolicy>,
}

impl CronRunTool {
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

    async fn run_shell_command(&self, command: &str) -> (bool, String) {
        let child = match Command::new("sh")
            .arg("-lc")
            .arg(command)
            .current_dir(&self.config.workspace_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
        {
            Ok(child) => child,
            Err(e) => return (false, format!("spawn error: {e}")),
        };

        match time::timeout(Duration::from_secs(RUN_TIMEOUT_SECS), child.wait_with_output()).await {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let combined = format!(
                    "status={}\nstdout:\n{}\nstderr:\n{}",
                    output.status,
                    stdout.trim(),
                    stderr.trim()
                );
                (output.status.success(), combined)
            }
            Ok(Err(e)) => (false, format!("spawn error: {e}")),
            Err(_) => (false, format!("job timed out after {RUN_TIMEOUT_SECS}s")),
        }
    }
}

#[async_trait]
impl Tool for CronRunTool {
    fn name(&self) -> &str {
        "cron_run"
    }

    fn description(&self) -> &str {
        "Run a scheduled task immediately by id (shell command) and persist last status/output."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "id": {"type": "string"},
                "approved": {
                    "type": "boolean",
                    "description": "Set true to explicitly approve medium/high-risk shell commands in supervised mode",
                    "default": false
                }
            },
            "required": ["id"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        if let Some(blocked) = self.enforce_mutation_allowed("cron_run") {
            return Ok(blocked);
        }

        let id = args
            .get("id")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .ok_or_else(|| anyhow::anyhow!("Missing 'id' parameter"))?;

        let approved = args
            .get("approved")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        let Some(job) = cron::get_job(&self.config, id)? else {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Cron job '{id}' not found")),
            });
        };

        if let Err(reason) = self.security.validate_command_execution(&job.command, approved) {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(reason),
            });
        }

        let (success, output) = self.run_shell_command(&job.command).await;

        if let Err(e) = cron::reschedule_after_run(&self.config, &job, success, &output) {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to persist cron run result: {e}")),
            });
        }

        Ok(ToolResult {
            success,
            output: json!({
                "id": job.id,
                "success": success,
                "output": output,
            })
            .to_string(),
            error: None,
        })
    }
}
