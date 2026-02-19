use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root policy configuration
#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct PolicyConfig {
    /// Default action when no rule matches
    #[serde(default = "default_action")]
    pub default_action: PolicyAction,

    /// Per-tool policies
    #[serde(default)]
    pub tools: HashMap<String, ToolPolicy>,

    /// Per-channel tool restrictions
    #[serde(default)]
    pub channels: HashMap<String, ChannelPolicy>,
}

fn default_action() -> PolicyAction {
    PolicyAction::Allow
}

/// Policy for a specific tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ToolPolicy {
    /// Allow or deny this tool
    pub action: PolicyAction,
    /// Optional: only allow from these channels
    pub allowed_channels: Option<Vec<String>>,
    /// Optional: deny from these channels
    pub denied_channels: Option<Vec<String>>,
    /// Optional: require confirmation before execution
    #[serde(default)]
    pub require_confirmation: bool,
    /// Optional: rate limit (max calls per minute)
    pub rate_limit: Option<u32>,
    /// Optional: human-readable reason for policy
    pub reason: Option<String>,
}

/// Channel-specific tool restrictions
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ChannelPolicy {
    /// Tools allowed for this channel
    pub allowed_tools: Option<Vec<String>>,
    /// Tools denied for this channel
    pub denied_tools: Option<Vec<String>>,
}

/// Policy decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PolicyAction {
    #[default]
    Allow,
    Deny,
    Audit, // Allow but log
}

/// Result of a policy evaluation
#[derive(Debug, Clone)]
pub struct PolicyDecision {
    pub action: PolicyAction,
    pub rule_source: String,
    pub reason: Option<String>,
}
