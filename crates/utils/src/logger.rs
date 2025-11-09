//! Logging configuration and utilities

use log::LevelFilter;
use std::env;

/// Initialize the logger with default configuration
pub fn init_logger() {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .init();
}

/// Initialize logger with custom level
pub fn init_logger_with_level(level: LevelFilter) {
    env_logger::Builder::from_default_env()
        .filter_level(level)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .init();
}
