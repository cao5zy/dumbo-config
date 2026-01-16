use crate::models::{ConfigError, EnvConfig, LoadingParam};
use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::Path;

/// Loads configuration using the specified loading parameters.
///
/// This function supports loading from both configuration files and environment variables.
/// Environment variables have higher priority than configuration files.
///
/// # Arguments
/// * `param` - The loading parameters specifying where to load configuration from
///
/// # Returns
/// * `Ok(T)` - Successfully loaded configuration
/// * `Err(ConfigError)` - Error during configuration loading
///
/// # Logging
/// * Always logs the loading parameters at info level
/// * If env_prefix is set and SHOW_SETTINGS=true, logs the loaded configuration at info level
pub fn load_config_with_param<T>(param: &LoadingParam) -> Result<T, ConfigError>
where
    T: for<'de> Deserialize<'de> + serde::Serialize,
{
    // Log the loading parameters
    log_loading_params(param);

    // Validate loading parameters
    validate_loading_params(param)?;

    // Build configuration sources
    let mut config_builder = Config::builder();

    // Add file source if specified
    if let Some(file_path) = param.file {
        config_builder = add_file_source(config_builder, file_path)?;
    }

    // Add environment variable source if specified
    if let Some(env_config) = &param.env_prefix {
        config_builder = add_env_source(config_builder, env_config)?;
    }

    // Build the configuration
    let config = config_builder.build()?;

    // Try to deserialize into the target type
    let result: T = config.try_deserialize()?;

    // Check if we should show settings
    if should_show_settings(param) {
        log_loaded_config(&result);
    }

    Ok(result)
}

/// Validates the loading parameters and returns appropriate errors
pub fn validate_loading_params(param: &LoadingParam) -> Result<(), ConfigError> {
    // Check if both sources are None
    if param.file.is_none() && param.env_prefix.is_none() {
        return Err(ConfigError::InvalidLoadingParam);
    }

    // Validate environment configuration if present
    if let Some(env_config) = &param.env_prefix {
        validate_env_config(env_config)?;
    }

    Ok(())
}

/// Validates environment configuration
fn validate_env_config(env_config: &EnvConfig) -> Result<(), ConfigError> {
    let separator = env_config.get_separator();
    if env_config.name.contains(separator) {
        return Err(ConfigError::InvalidEnvConfig {
            prefix: env_config.name.clone(),
            separator: separator.to_string(),
        });
    }
    Ok(())
}

/// Adds file source to the configuration builder
fn add_file_source(
    config_builder: config::ConfigBuilder<config::builder::DefaultState>,
    file_path: &Path,
) -> Result<config::ConfigBuilder<config::builder::DefaultState>, ConfigError> {
    // Check if file exists
    if !file_path.exists() {
        return Err(ConfigError::FileNotFound(file_path.to_path_buf()));
    }

    // Determine file format from extension
    let format = get_file_format(file_path);

    // Add source and return new builder
    Ok(config_builder.add_source(File::from(file_path).format(format)))
}

/// Gets the file format based on file extension
fn get_file_format(file_path: &Path) -> FileFormat {
    match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => FileFormat::Json,
        Some("yaml") | Some("yml") => FileFormat::Yaml,
        Some("toml") => FileFormat::Toml,
        Some("ini") => FileFormat::Ini,
        // Remove Properties as it's not supported in current config version
        _ => FileFormat::Yaml, // Default to YAML
    }
}

/// Adds environment variable source to the configuration builder
fn add_env_source(
    config_builder: config::ConfigBuilder<config::builder::DefaultState>,
    env_config: &EnvConfig,
) -> Result<config::ConfigBuilder<config::builder::DefaultState>, ConfigError> {
    let prefix = &env_config.name;
    let separator = env_config.get_separator();

    // Check if any environment variables exist with this prefix
    let env_vars_with_prefix: Vec<String> = env::vars()
        .filter(|(key, _)| key.starts_with(prefix))
        .map(|(key, _)| key)
        .collect();

    // If no environment variables found with this prefix, return error
    if env_vars_with_prefix.is_empty() {
        return Err(ConfigError::EnvPrefixNotFound(prefix.clone()));
    }

    // Add source and return new builder
    Ok(config_builder.add_source(
        config::Environment::with_prefix(prefix)
            .separator(separator)
            .try_parsing(true),
    ))
}

/// Checks if SHOW_SETTINGS environment variable is set to true
fn should_show_settings(param: &LoadingParam) -> bool {
    if let Some(env_prefix) = &param.env_prefix {
        let env_full_name = format!(
            "{}{}SHOW_SETTINGS",
            &env_prefix.name,
            &env_prefix.get_separator()
        );
        match env::var(&env_full_name) {
            Ok(value) => {
                let lower_value = value.to_lowercase();
                lower_value == "true"
                    || lower_value == "1"
                    || lower_value == "yes"
                    || lower_value == "on"
            }
            Err(_) => {
                log::warn!("{} not set, return false", &env_full_name);
                false
            }
        }
    } else {
        false
    }
}

/// Logs the loading parameters
fn log_loading_params(param: &LoadingParam) {
    if let Some(file_path) = param.file {
        log::info!("Loading configuration from file: {:?}", file_path);
    }

    if let Some(env_config) = &param.env_prefix {
        log::info!(
            "Loading configuration from environment variables with prefix: '{}' and separator: '{}'",
            env_config.name,
            env_config.get_separator()
        );
    }
}

/// Logs that configuration was loaded (when SHOW_SETTINGS is enabled)
fn log_loaded_config<T>(config: &T)
where
    T: serde::Serialize,
{
    match serde_json::to_string_pretty(config) {
        Ok(serialized) => {
            log::info!(
                "Configuration loaded successfully (SHOW_SETTINGS enabled):\n{}",
                serialized
            );
        }
        Err(e) => {
            log::warn!("Failed to serialize configuration for logging: {}", e);
            log::info!("Configuration loaded successfully (SHOW_SETTINGS enabled)");
        }
    }
}

// Expose should_show_settings for testing purposes
#[cfg(test)]
pub fn test_should_show_settings(param: &LoadingParam) -> bool {
    should_show_settings(param)
}
