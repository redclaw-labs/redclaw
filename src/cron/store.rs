//! Cron storage helpers.
//!
//! Upstream splits cron persistence into a dedicated module. In this codebase,
//! the existing cron backend is currently implemented in `src/cron/mod.rs`.
//!
//! This module exists to preserve upstream file layout expectations during the
//! restream/merge process.

#![allow(dead_code)]

use crate::config::Config;
use anyhow::Result;

/// Placeholder: cron persistence is currently implemented in `crate::cron`.
pub fn health_check(_config: &Config) -> Result<()> {
    Ok(())
}
