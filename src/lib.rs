//! dumbo-config - A configuration loading library with enhanced logging and error handling
//!
//! This library provides a flexible way to load configuration from files and environment variables,
//! with detailed logging and comprehensive error handling.

pub mod config;
pub mod loading;
pub mod models;

// Re-export commonly used types from models
pub use models::{ConfigError, EnvConfig, LoadingParam};

// Re-export the new loading function
pub use loading::load_config_with_param;

// Keep backward compatibility with existing functions
pub use config::{load_config, load_config_from_file, load_named_config};

#[cfg(test)]
mod tests;
