//! RedClaw Error Code System
//!
//! Structured error codes for better diagnostics and troubleshooting.
//! Format: RC-CXXX where C = category (1-9), XXX = specific error.
//!
//! # Categories
//! - RC-1xxx: Configuration errors
//! - RC-2xxx: Provider errors  
//! - RC-3xxx: Channel errors
//! - RC-4xxx: Tool errors
//! - RC-5xxx: Security errors
//! - RC-6xxx: Runtime errors
//! - RC-7xxx: Memory errors
//! - RC-8xxx: Gateway errors
//! - RC-9xxx: Hardware/Peripheral errors

mod codes;

pub use codes::*;
