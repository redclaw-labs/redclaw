use super::rules::{PolicyAction, PolicyConfig, PolicyDecision};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Policy engine that evaluates tool execution requests against configured policies
pub struct PolicyEngine {
    config: PolicyConfig,
    rate_counters: RwLock<HashMap<String, Vec<Instant>>>,
}

impl PolicyEngine {
    pub fn new(config: PolicyConfig) -> Self {
        Self {
            config,
            rate_counters: RwLock::new(HashMap::new()),
        }
    }

    /// Evaluate whether a tool call is permitted
    pub fn evaluate(&self, tool_name: &str, channel: Option<&str>) -> PolicyDecision {
        // 1. Check channel-level restrictions first
        if let Some(ch) = channel {
            if let Some(ch_policy) = self.config.channels.get(ch) {
                // Check denied_tools
                if let Some(denied) = &ch_policy.denied_tools {
                    if denied.iter().any(|t| t == tool_name || t == "*") {
                        return PolicyDecision {
                            action: PolicyAction::Deny,
                            rule_source: format!("channel.{ch}.denied_tools"),
                            reason: Some(format!(
                                "Tool '{tool_name}' is denied for channel '{ch}'"
                            )),
                        };
                    }
                }
                // Check allowed_tools (if set, acts as allowlist)
                if let Some(allowed) = &ch_policy.allowed_tools {
                    if !allowed.iter().any(|t| t == tool_name || t == "*") {
                        return PolicyDecision {
                            action: PolicyAction::Deny,
                            rule_source: format!("channel.{ch}.allowed_tools"),
                            reason: Some(format!(
                                "Tool '{tool_name}' is not in allowlist for channel '{ch}'"
                            )),
                        };
                    }
                }
            }
        }

        // 2. Check tool-specific policy
        if let Some(tool_policy) = self.config.tools.get(tool_name) {
            // Check channel restrictions on the tool
            if let Some(ch) = channel {
                if let Some(denied) = &tool_policy.denied_channels {
                    if denied.contains(&ch.to_string()) {
                        return PolicyDecision {
                            action: PolicyAction::Deny,
                            rule_source: format!("tools.{tool_name}.denied_channels"),
                            reason: tool_policy.reason.clone(),
                        };
                    }
                }
                if let Some(allowed) = &tool_policy.allowed_channels {
                    if !allowed.contains(&ch.to_string()) {
                        return PolicyDecision {
                            action: PolicyAction::Deny,
                            rule_source: format!("tools.{tool_name}.allowed_channels"),
                            reason: tool_policy.reason.clone(),
                        };
                    }
                }
            }

            // Check rate limit
            if let Some(limit) = tool_policy.rate_limit {
                if self.is_rate_limited(tool_name, limit) {
                    return PolicyDecision {
                        action: PolicyAction::Deny,
                        rule_source: format!("tools.{tool_name}.rate_limit"),
                        reason: Some(format!("Rate limit exceeded ({limit}/min)")),
                    };
                }
            }

            return PolicyDecision {
                action: tool_policy.action,
                rule_source: format!("tools.{tool_name}"),
                reason: tool_policy.reason.clone(),
            };
        }

        // 3. Default action
        PolicyDecision {
            action: self.config.default_action,
            rule_source: "default".to_string(),
            reason: None,
        }
    }

    /// Record a tool call for rate limiting
    pub fn record_call(&self, tool_name: &str) {
        let mut counters = self
            .rate_counters
            .write()
            .expect("rate counters lock poisoned");
        let now = Instant::now();
        let entry = counters.entry(tool_name.to_string()).or_default();
        entry.push(now);
        // Prune entries older than 1 minute
        entry.retain(|t| now.duration_since(*t) < Duration::from_secs(60));
    }

    fn is_rate_limited(&self, tool_name: &str, limit: u32) -> bool {
        let counters = self
            .rate_counters
            .read()
            .expect("rate counters lock poisoned");
        if let Some(calls) = counters.get(tool_name) {
            let now = Instant::now();
            let recent = calls
                .iter()
                .filter(|t| now.duration_since(**t) < Duration::from_secs(60))
                .count();
            recent >= limit as usize
        } else {
            false
        }
    }

    /// Get policy summary for doctor/diagnostics
    pub fn summary(&self) -> PolicySummary {
        PolicySummary {
            default_action: self.config.default_action,
            tool_rules: self.config.tools.len(),
            channel_rules: self.config.channels.len(),
        }
    }
}

#[derive(Debug)]
pub struct PolicySummary {
    pub default_action: PolicyAction,
    pub tool_rules: usize,
    pub channel_rules: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{ChannelPolicy, ToolPolicy};

    fn test_config() -> PolicyConfig {
        let mut tools = HashMap::new();
        tools.insert(
            "shell".to_string(),
            ToolPolicy {
                action: PolicyAction::Audit,
                allowed_channels: Some(vec!["terminal".to_string()]),
                denied_channels: None,
                require_confirmation: true,
                rate_limit: Some(10),
                reason: Some("Shell access is sensitive".to_string()),
            },
        );
        tools.insert(
            "browser".to_string(),
            ToolPolicy {
                action: PolicyAction::Deny,
                allowed_channels: None,
                denied_channels: None,
                require_confirmation: false,
                rate_limit: None,
                reason: Some("Browser disabled by policy".to_string()),
            },
        );

        let mut channels = HashMap::new();
        channels.insert(
            "discord".to_string(),
            ChannelPolicy {
                allowed_tools: Some(vec!["memory_read".to_string(), "web_search".to_string()]),
                denied_tools: None,
            },
        );

        PolicyConfig {
            default_action: PolicyAction::Allow,
            tools,
            channels,
        }
    }

    #[test]
    fn default_allows() {
        let engine = PolicyEngine::new(test_config());
        let decision = engine.evaluate("memory_read", None);
        assert_eq!(decision.action, PolicyAction::Allow);
    }

    #[test]
    fn tool_policy_denies_browser() {
        let engine = PolicyEngine::new(test_config());
        let decision = engine.evaluate("browser", None);
        assert_eq!(decision.action, PolicyAction::Deny);
    }

    #[test]
    fn tool_policy_audits_shell() {
        let engine = PolicyEngine::new(test_config());
        let decision = engine.evaluate("shell", Some("terminal"));
        assert_eq!(decision.action, PolicyAction::Audit);
    }

    #[test]
    fn shell_denied_from_wrong_channel() {
        let engine = PolicyEngine::new(test_config());
        let decision = engine.evaluate("shell", Some("discord"));
        assert_eq!(decision.action, PolicyAction::Deny);
    }

    #[test]
    fn channel_allowlist_restricts() {
        let engine = PolicyEngine::new(test_config());
        // discord only allows memory_read and web_search
        let decision = engine.evaluate("file_write", Some("discord"));
        assert_eq!(decision.action, PolicyAction::Deny);
    }

    #[test]
    fn channel_allowlist_permits() {
        let engine = PolicyEngine::new(test_config());
        let decision = engine.evaluate("memory_read", Some("discord"));
        assert_eq!(decision.action, PolicyAction::Allow);
    }

    #[test]
    fn rate_limiting_works() {
        let engine = PolicyEngine::new(test_config());
        // Shell has rate_limit: 10
        for _ in 0..10 {
            engine.record_call("shell");
        }
        let decision = engine.evaluate("shell", Some("terminal"));
        assert_eq!(decision.action, PolicyAction::Deny);
        assert!(decision.reason.unwrap().contains("Rate limit"));
    }

    #[test]
    fn summary_counts() {
        let engine = PolicyEngine::new(test_config());
        let summary = engine.summary();
        assert_eq!(summary.tool_rules, 2);
        assert_eq!(summary.channel_rules, 1);
    }
}
