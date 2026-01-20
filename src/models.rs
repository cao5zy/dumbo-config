use std::fmt;
use std::path::Path;

const DEFAULT_SEPERATOR: &str = "__";

/// Environment configuration for loading settings from environment variables
pub struct EnvConfig {
    pub name: String,              // Environment variable prefix
    pub separator: Option<String>, // Environment variable separator, defaults to "_"
}

impl EnvConfig {
    /// Creates a new EnvConfig with the given name and optional separator
    pub fn new(name: String, separator: Option<String>) -> Self {
        Self { name, separator }
    }

    /// Gets the separator, defaulting to "_" if not specified
    pub fn get_separator(&self) -> &str {
        self.separator.as_deref().unwrap_or(DEFAULT_SEPERATOR)
    }
}

/// Loading parameters for configuration
///
/// Note: env_prefix has higher priority than file, meaning if both are present,
/// settings from env_prefix will override those from file.
pub struct LoadingParam<'a> {
    pub file: Option<&'a Path>,        // Configuration file path
    pub env_prefix: Option<EnvConfig>, // Environment variable configuration
}

/// Configuration loading errors
#[derive(Debug)]
pub enum ConfigError {
    /// Wrapped config crate error
    Config(config::ConfigError),
    /// Configuration file not found
    FileNotFound(std::path::PathBuf),
    /// SHOW_SETTINGS environment variable cannot be parsed as boolean
    ShowSettingsParseError(String),
    /// Invalid loading parameter: both file and env_prefix are None
    InvalidLoadingParam,
    /// Invalid environment configuration: env prefix contains separator
    InvalidEnvConfig { prefix: String, separator: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Config(err) => write!(f, "Config error: {}", err),
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {:?}", path)
            }
            ConfigError::ShowSettingsParseError(value) => {
                write!(
                    f,
                    "Cannot parse SHOW_SETTINGS environment variable '{}' as boolean",
                    value
                )
            }
            ConfigError::InvalidLoadingParam => {
                write!(f, "No configuration source provided. Please configure at least one of:\n\
                          - Configuration file (set the 'file' parameter)\n\
                          - Environment variables (set the 'env_prefix' parameter with a valid prefix)")
            }
            ConfigError::InvalidEnvConfig { prefix, separator } => {
                write!(f, "Invalid environment configuration: env prefix '{}' contains separator '{}'.\n\
                          This will cause configuration loading to fail. Please choose a prefix that doesn't contain the separator,\n\
                          or use a different separator character.", prefix, separator)
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Config(err) => Some(err),
            _ => None,
        }
    }
}

impl From<config::ConfigError> for ConfigError {
    fn from(err: config::ConfigError) -> Self {
        ConfigError::Config(err)
    }
}
