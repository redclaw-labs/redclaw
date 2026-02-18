use std::fmt;

/// A RedClaw error code with structured category and description
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorCode {
    /// Numeric code (e.g., 1001)
    pub code: u16,
    /// Short mnemonic (e.g., "CONFIG_NOT_FOUND")  
    pub mnemonic: &'static str,
    /// Human-readable description
    pub description: &'static str,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RC-{:04}", self.code)
    }
}

impl ErrorCode {
    /// Format as full diagnostic string
    pub fn diagnostic(&self) -> String {
        format!(
            "[RC-{:04}] {}: {}",
            self.code, self.mnemonic, self.description
        )
    }

    /// Get the category name from the code
    pub fn category(&self) -> &'static str {
        match self.code / 1000 {
            1 => "Configuration",
            2 => "Provider",
            3 => "Channel",
            4 => "Tool",
            5 => "Security",
            6 => "Runtime",
            7 => "Memory",
            8 => "Gateway",
            9 => "Hardware",
            _ => "Unknown",
        }
    }
}

/// Create a RedClaw error with code context
/// Usage: `rc_error!(CONFIG_NOT_FOUND, "path {} does not exist", path)`
#[macro_export]
macro_rules! rc_error {
    ($code:expr, $($arg:tt)*) => {
        anyhow::anyhow!("{} — {}", $code.diagnostic(), format!($($arg)*))
    };
}

/// Bail with a RedClaw error code
/// Usage: `rc_bail!(CONFIG_NOT_FOUND, "path {} does not exist", path)`
#[macro_export]
macro_rules! rc_bail {
    ($code:expr, $($arg:tt)*) => {
        anyhow::bail!("{} — {}", $code.diagnostic(), format!($($arg)*))
    };
}

// ═══════════════════════════════════════════════════════════════
// RC-1xxx: Configuration Errors
// ═══════════════════════════════════════════════════════════════

pub const CONFIG_NOT_FOUND: ErrorCode = ErrorCode {
    code: 1001,
    mnemonic: "CONFIG_NOT_FOUND",
    description: "Configuration file not found",
};

pub const CONFIG_PARSE_ERROR: ErrorCode = ErrorCode {
    code: 1002,
    mnemonic: "CONFIG_PARSE_ERROR",
    description: "Failed to parse configuration file",
};

pub const CONFIG_INVALID_VALUE: ErrorCode = ErrorCode {
    code: 1003,
    mnemonic: "CONFIG_INVALID_VALUE",
    description: "Invalid configuration value",
};

pub const CONFIG_MISSING_REQUIRED: ErrorCode = ErrorCode {
    code: 1004,
    mnemonic: "CONFIG_MISSING_REQUIRED",
    description: "Required configuration field missing",
};

pub const CONFIG_DIR_CREATE_FAILED: ErrorCode = ErrorCode {
    code: 1005,
    mnemonic: "CONFIG_DIR_CREATE_FAILED",
    description: "Failed to create configuration directory",
};

// ═══════════════════════════════════════════════════════════════
// RC-2xxx: Provider Errors
// ═══════════════════════════════════════════════════════════════

pub const PROVIDER_NOT_FOUND: ErrorCode = ErrorCode {
    code: 2001,
    mnemonic: "PROVIDER_NOT_FOUND",
    description: "Specified provider not found or not supported",
};

pub const PROVIDER_AUTH_FAILED: ErrorCode = ErrorCode {
    code: 2002,
    mnemonic: "PROVIDER_AUTH_FAILED",
    description: "Provider authentication failed",
};

pub const PROVIDER_RATE_LIMITED: ErrorCode = ErrorCode {
    code: 2003,
    mnemonic: "PROVIDER_RATE_LIMITED",
    description: "Provider rate limit exceeded",
};

pub const PROVIDER_TIMEOUT: ErrorCode = ErrorCode {
    code: 2004,
    mnemonic: "PROVIDER_TIMEOUT",
    description: "Provider request timed out",
};

pub const PROVIDER_RESPONSE_ERROR: ErrorCode = ErrorCode {
    code: 2005,
    mnemonic: "PROVIDER_RESPONSE_ERROR",
    description: "Invalid or unexpected provider response",
};

pub const PROVIDER_MODEL_NOT_FOUND: ErrorCode = ErrorCode {
    code: 2006,
    mnemonic: "PROVIDER_MODEL_NOT_FOUND",
    description: "Requested model not available on provider",
};

// ═══════════════════════════════════════════════════════════════
// RC-3xxx: Channel Errors
// ═══════════════════════════════════════════════════════════════

pub const CHANNEL_NOT_FOUND: ErrorCode = ErrorCode {
    code: 3001,
    mnemonic: "CHANNEL_NOT_FOUND",
    description: "Specified channel not found or not supported",
};

pub const CHANNEL_AUTH_FAILED: ErrorCode = ErrorCode {
    code: 3002,
    mnemonic: "CHANNEL_AUTH_FAILED",
    description: "Channel authentication/token validation failed",
};

pub const CHANNEL_SEND_FAILED: ErrorCode = ErrorCode {
    code: 3003,
    mnemonic: "CHANNEL_SEND_FAILED",
    description: "Failed to send message through channel",
};

pub const CHANNEL_LISTEN_FAILED: ErrorCode = ErrorCode {
    code: 3004,
    mnemonic: "CHANNEL_LISTEN_FAILED",
    description: "Channel listener encountered an error",
};

pub const CHANNEL_HEALTH_CHECK_FAILED: ErrorCode = ErrorCode {
    code: 3005,
    mnemonic: "CHANNEL_HEALTH_CHECK_FAILED",
    description: "Channel health check failed",
};

// ═══════════════════════════════════════════════════════════════
// RC-4xxx: Tool Errors
// ═══════════════════════════════════════════════════════════════

pub const TOOL_NOT_FOUND: ErrorCode = ErrorCode {
    code: 4001,
    mnemonic: "TOOL_NOT_FOUND",
    description: "Specified tool not found",
};

pub const TOOL_EXECUTION_FAILED: ErrorCode = ErrorCode {
    code: 4002,
    mnemonic: "TOOL_EXECUTION_FAILED",
    description: "Tool execution failed",
};

pub const TOOL_INVALID_PARAMS: ErrorCode = ErrorCode {
    code: 4003,
    mnemonic: "TOOL_INVALID_PARAMS",
    description: "Invalid parameters passed to tool",
};

pub const TOOL_PERMISSION_DENIED: ErrorCode = ErrorCode {
    code: 4004,
    mnemonic: "TOOL_PERMISSION_DENIED",
    description: "Tool execution denied by security policy",
};

// ═══════════════════════════════════════════════════════════════
// RC-5xxx: Security Errors
// ═══════════════════════════════════════════════════════════════

pub const SECURITY_UNAUTHORIZED: ErrorCode = ErrorCode {
    code: 5001,
    mnemonic: "SECURITY_UNAUTHORIZED",
    description: "Unauthorized access attempt",
};

pub const SECURITY_PAIRING_FAILED: ErrorCode = ErrorCode {
    code: 5002,
    mnemonic: "SECURITY_PAIRING_FAILED",
    description: "Device pairing failed",
};

pub const SECURITY_SECRET_NOT_FOUND: ErrorCode = ErrorCode {
    code: 5003,
    mnemonic: "SECURITY_SECRET_NOT_FOUND",
    description: "Required secret not found in secret store",
};

pub const SECURITY_POLICY_VIOLATION: ErrorCode = ErrorCode {
    code: 5004,
    mnemonic: "SECURITY_POLICY_VIOLATION",
    description: "Action violates security policy",
};

// ═══════════════════════════════════════════════════════════════
// RC-6xxx: Runtime Errors
// ═══════════════════════════════════════════════════════════════

pub const RUNTIME_INIT_FAILED: ErrorCode = ErrorCode {
    code: 6001,
    mnemonic: "RUNTIME_INIT_FAILED",
    description: "Runtime initialization failed",
};

pub const RUNTIME_SANDBOX_ERROR: ErrorCode = ErrorCode {
    code: 6002,
    mnemonic: "RUNTIME_SANDBOX_ERROR",
    description: "Sandbox execution error",
};

pub const RUNTIME_RESOURCE_EXHAUSTED: ErrorCode = ErrorCode {
    code: 6003,
    mnemonic: "RUNTIME_RESOURCE_EXHAUSTED",
    description: "Runtime resource limit exceeded",
};

// ═══════════════════════════════════════════════════════════════
// RC-7xxx: Memory Errors
// ═══════════════════════════════════════════════════════════════

pub const MEMORY_STORE_FAILED: ErrorCode = ErrorCode {
    code: 7001,
    mnemonic: "MEMORY_STORE_FAILED",
    description: "Failed to store memory entry",
};

pub const MEMORY_RETRIEVE_FAILED: ErrorCode = ErrorCode {
    code: 7002,
    mnemonic: "MEMORY_RETRIEVE_FAILED",
    description: "Failed to retrieve memory entry",
};

pub const MEMORY_DB_ERROR: ErrorCode = ErrorCode {
    code: 7003,
    mnemonic: "MEMORY_DB_ERROR",
    description: "Memory database backend error",
};

// ═══════════════════════════════════════════════════════════════
// RC-8xxx: Gateway Errors
// ═══════════════════════════════════════════════════════════════

pub const GATEWAY_BIND_FAILED: ErrorCode = ErrorCode {
    code: 8001,
    mnemonic: "GATEWAY_BIND_FAILED",
    description: "Gateway failed to bind to address",
};

pub const GATEWAY_TLS_ERROR: ErrorCode = ErrorCode {
    code: 8002,
    mnemonic: "GATEWAY_TLS_ERROR",
    description: "Gateway TLS configuration error",
};

pub const GATEWAY_WEBHOOK_INVALID: ErrorCode = ErrorCode {
    code: 8003,
    mnemonic: "GATEWAY_WEBHOOK_INVALID",
    description: "Invalid webhook signature or payload",
};

// ═══════════════════════════════════════════════════════════════
// RC-9xxx: Hardware/Peripheral Errors
// ═══════════════════════════════════════════════════════════════

pub const HARDWARE_NOT_FOUND: ErrorCode = ErrorCode {
    code: 9001,
    mnemonic: "HARDWARE_NOT_FOUND",
    description: "Hardware peripheral not found or not connected",
};

pub const HARDWARE_COMM_ERROR: ErrorCode = ErrorCode {
    code: 9002,
    mnemonic: "HARDWARE_COMM_ERROR",
    description: "Hardware communication error",
};

pub const HARDWARE_FIRMWARE_ERROR: ErrorCode = ErrorCode {
    code: 9003,
    mnemonic: "HARDWARE_FIRMWARE_ERROR",
    description: "Hardware firmware error",
};

// ═══════════════════════════════════════════════════════════════
// Error Code Registry (for programmatic access)
// ═══════════════════════════════════════════════════════════════

/// All registered error codes
pub const ALL_ERROR_CODES: &[ErrorCode] = &[
    // Config
    CONFIG_NOT_FOUND,
    CONFIG_PARSE_ERROR,
    CONFIG_INVALID_VALUE,
    CONFIG_MISSING_REQUIRED,
    CONFIG_DIR_CREATE_FAILED,
    // Provider
    PROVIDER_NOT_FOUND,
    PROVIDER_AUTH_FAILED,
    PROVIDER_RATE_LIMITED,
    PROVIDER_TIMEOUT,
    PROVIDER_RESPONSE_ERROR,
    PROVIDER_MODEL_NOT_FOUND,
    // Channel
    CHANNEL_NOT_FOUND,
    CHANNEL_AUTH_FAILED,
    CHANNEL_SEND_FAILED,
    CHANNEL_LISTEN_FAILED,
    CHANNEL_HEALTH_CHECK_FAILED,
    // Tool
    TOOL_NOT_FOUND,
    TOOL_EXECUTION_FAILED,
    TOOL_INVALID_PARAMS,
    TOOL_PERMISSION_DENIED,
    // Security
    SECURITY_UNAUTHORIZED,
    SECURITY_PAIRING_FAILED,
    SECURITY_SECRET_NOT_FOUND,
    SECURITY_POLICY_VIOLATION,
    // Runtime
    RUNTIME_INIT_FAILED,
    RUNTIME_SANDBOX_ERROR,
    RUNTIME_RESOURCE_EXHAUSTED,
    // Memory
    MEMORY_STORE_FAILED,
    MEMORY_RETRIEVE_FAILED,
    MEMORY_DB_ERROR,
    // Gateway
    GATEWAY_BIND_FAILED,
    GATEWAY_TLS_ERROR,
    GATEWAY_WEBHOOK_INVALID,
    // Hardware
    HARDWARE_NOT_FOUND,
    HARDWARE_COMM_ERROR,
    HARDWARE_FIRMWARE_ERROR,
];

/// Look up an error code by its numeric value
pub fn lookup_code(code: u16) -> Option<&'static ErrorCode> {
    ALL_ERROR_CODES.iter().find(|c| c.code == code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_display() {
        assert_eq!(format!("{}", CONFIG_NOT_FOUND), "RC-1001");
        assert_eq!(format!("{}", PROVIDER_AUTH_FAILED), "RC-2002");
        assert_eq!(format!("{}", HARDWARE_FIRMWARE_ERROR), "RC-9003");
    }

    #[test]
    fn error_code_diagnostic() {
        let diag = CONFIG_NOT_FOUND.diagnostic();
        assert!(diag.contains("RC-1001"));
        assert!(diag.contains("CONFIG_NOT_FOUND"));
        assert!(diag.contains("Configuration file not found"));
    }

    #[test]
    fn error_code_category() {
        assert_eq!(CONFIG_NOT_FOUND.category(), "Configuration");
        assert_eq!(PROVIDER_AUTH_FAILED.category(), "Provider");
        assert_eq!(CHANNEL_AUTH_FAILED.category(), "Channel");
        assert_eq!(TOOL_EXECUTION_FAILED.category(), "Tool");
        assert_eq!(SECURITY_UNAUTHORIZED.category(), "Security");
        assert_eq!(RUNTIME_INIT_FAILED.category(), "Runtime");
        assert_eq!(MEMORY_STORE_FAILED.category(), "Memory");
        assert_eq!(GATEWAY_BIND_FAILED.category(), "Gateway");
        assert_eq!(HARDWARE_NOT_FOUND.category(), "Hardware");
    }

    #[test]
    fn error_code_lookup() {
        assert_eq!(lookup_code(1001), Some(&CONFIG_NOT_FOUND));
        assert_eq!(lookup_code(2002), Some(&PROVIDER_AUTH_FAILED));
        assert_eq!(lookup_code(9999), None);
    }

    #[test]
    fn all_codes_unique() {
        let mut codes: Vec<u16> = ALL_ERROR_CODES.iter().map(|c| c.code).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(
            codes.len(),
            ALL_ERROR_CODES.len(),
            "Duplicate error codes found!"
        );
    }

    #[test]
    fn all_mnemonics_unique() {
        let mut mnemonics: Vec<&str> = ALL_ERROR_CODES.iter().map(|c| c.mnemonic).collect();
        mnemonics.sort_unstable();
        mnemonics.dedup();
        assert_eq!(
            mnemonics.len(),
            ALL_ERROR_CODES.len(),
            "Duplicate mnemonics found!"
        );
    }

    #[test]
    fn rc_error_macro() {
        let err = rc_error!(
            CONFIG_NOT_FOUND,
            "path {} does not exist",
            "/home/.redclaw/config.toml"
        );
        let msg = err.to_string();
        assert!(msg.contains("RC-1001"));
        assert!(msg.contains("path /home/.redclaw/config.toml does not exist"));
    }
}
