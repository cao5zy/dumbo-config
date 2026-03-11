# dumbo-config  
dumbo-config is a flexible configuration loader that provides three different levels of APIs to meet various configuration loading needs.

## Core APIs

dumbo-config provides three main entry functions, arranged from simple to complex use cases:

### 1. `load_config` - Automatic Configuration File Search

The simplest usage method that automatically searches for configuration files in the following order:
1. `config.{ENV}.yml`
2. `config.{ENV}.yaml`
3. `config.yml`
4. `config.yaml`

Where `ENV` is the value of the environment variable "ENV". If "ENV" is not set, it defaults to searching `config.yml` and `config.yaml`.

```rust
use dumbo_config::load_config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    name: String,
    value: i32,
}

// Automatically search and load configuration
let config: Option<AppConfig> = load_config();
```

**Use Case**: Standard project structure with configuration files in the project root directory using default naming conventions.

---

### 2. `load_named_config` - Specify Configuration File Path

Use this function to specify a specific path when the configuration file is not in the default location or uses a non-standard name.

```rust
use dumbo_config::load_named_config;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AppConfig {
    name: String,
    value: i32,
}

// Load configuration from a specified path
let config_path = Path::new("configs/production.yaml");
let config: Option<AppConfig> = load_named_config(config_path);
```

**Use Case**: Configuration files located in non-standard locations, or when you need to dynamically select configuration files based on runtime conditions.

---

### 3. `load_config_with_param` - Advanced Configuration Loading

The most powerful loading method that supports loading configuration from both files and environment variables, with comprehensive error handling.

#### LoadingParam Structure

The `load_config_with_param` function accepts a `LoadingParam` parameter that defines the configuration loading sources:

```rust
pub struct LoadingParam {
    /// Configuration file path, if None then do not load from file
    pub file: Option<&'static Path>,
    /// Environment variable prefix configuration, if None then do not load from environment variables
    pub env_prefix: Option<EnvConfig>,
}
```

**Field Description**:
- `file`: Configuration file path, set to `Some(path)` to load from file
- `env_prefix`: Environment variable prefix configuration, set to `Some(EnvConfig)` to load from environment variables

#### Usage Examples

```rust
use dumbo_config::{LoadingParam, EnvConfig, load_config_with_param};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AppConfig {
    database_url: String,
    port: u16,
    debug: bool,
}

// Load from file only
let param = LoadingParam {
    file: Some(Path::new("config.yaml")),
    env_prefix: None,  // Do not load from environment variables
};
let config: AppConfig = load_config_with_param(&param)?;

// Load from environment variables only
let param = LoadingParam {
    file: None,  // Do not load from file
    env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
};
let config: AppConfig = load_config_with_param(&param)?;

// Load from both file and environment variables (environment variables take precedence)
let param = LoadingParam {
    file: Some(Path::new("config.yaml")),
    env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
};
let config: AppConfig = load_config_with_param(&param)?;
```

**Use Cases**:
- Need to override file configuration with environment variables
- Need detailed error information
- Production environments require stricter configuration validation

**Return Value**: `Result<T, ConfigError>`, providing comprehensive error handling.

#### Best Practice: File + Environment Variable Combined Configuration

It is recommended to configure both file and environment variables simultaneously. **Environment variables will automatically override values in the configuration file**. This allows you to use the same code across different environments, only overriding configuration items that need to change through environment variables.

**Advantages**:
- ✅ Simple code, no environment judgment logic required
- ✅ config.yml serves as the default configuration template with default values for all configuration items
- ✅ Production environment only needs to set environment variables that need to be overridden (such as sensitive information)
- ✅ Flexible, can partially override configuration

**Configuration Comparison Example**:

Assuming your application configuration structure is as follows:
```rust
#[derive(Debug, Deserialize)]
struct AppConfig {
    database_url: String,
    port: u16,
    debug: bool,
}
```

**config.yml (Default Configuration)**:
```yaml
database_url: "postgres://localhost:5432/dev_db"
port: 3000
debug: true
```

**.env (Production Environment Override)**:
```bash
# Only override configuration items that need to change
MY_APP__DATABASE_URL=postgres://prod-server:5432/prod_db
MY_APP__DEBUG=false
# port is not set, will use default value 3000 from config.yml
```

**Unified Loading Code**:
```rust
use dumbo_config::{LoadingParam, EnvConfig, load_config_with_param};
use std::path::Path;

fn load_app_config() -> Result<AppConfig, ConfigError> {
    // Configure both file and environment variables
    // Development environment: only use config.yml
    // Production environment: config.yml + environment variable override
    let param = LoadingParam {
        file: Some(Path::new("config.yml")),
        env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
    };
    
    load_config_with_param(&param)
}
```

**Workflow**:
1. First load all configuration items from `config.yml` as default values
2. Then load environment variables with `MY_APP__` prefix
3. Environment variables will override configuration items with the same name in config.yml
4. Configuration items without environment variables set keep the default values from config.yml

---

## Environment Variable Configuration

When using `load_config_with_param`, you can configure environment variable loading through `EnvConfig`.

### EnvConfig Structure

```rust
pub struct EnvConfig {
    /// Environment variable prefix, e.g., "MY_APP"
    pub name: String,
    /// Environment variable separator, defaults to "__"
    pub separator: Option<String>,
}
```

**Creation Methods**:
```rust
// Use default separator "__"
let env_config = EnvConfig::new("MY_APP".to_string(), None);

// Use custom separator
let env_config = EnvConfig::new("MY_APP".to_string(), Some("_".to_string()));
```

### Environment Variable Naming Rules

Given the following configuration structure:
```rust
struct DatabaseConfig {
    host: String,
    port: u16,
    credentials: Credentials,
}

struct Credentials {
    username: String,
    password: String,
}
```

With `EnvConfig::new("MY_APP".to_string(), None)` (default separator "__"), the corresponding environment variables are:
```bash
# Top-level fields
export MY_APP__HOST="localhost"
export MY_APP__PORT="5432"

# Nested fields (using double underscore as separator)
export MY_APP__CREDENTIALS__USERNAME="myuser"
export MY_APP__CREDENTIALS__PASSWORD="mypass"
```

**Note**: The environment variable prefix should not contain the separator character. For example, if your prefix is "RESUME_AGENT" and separator is "_", this will cause a configuration loading error.

---

## Logging and Debugging

The library provides detailed logging at the INFO level:

- Always logs which configuration sources are being used
- When `env_prefix` is set and the `SHOW_SETTINGS` environment variable is set to "true" (case-insensitive), logs that configuration was loaded successfully
- If no environment variables are found with the specified prefix, a warning is logged and configuration loading continues without environment variables

**Note**: The `SHOW_SETTINGS` environment variable is only checked when `env_prefix` is configured. This is because if no `env_prefix` is set, it means the user is loading configuration directly from files, and they can simply inspect the configuration files directly to view the settings. When environment variables are used for configuration (via `env_prefix`), the actual values may not be easily visible, so `SHOW_SETTINGS` provides a way to log the loaded configuration for debugging purposes.

To enable configuration debugging, set the `SHOW_SETTINGS` environment variable if the `env_prefix` is 'MY_APP':

```bash
export MY_APP__SHOW_SETTINGS=true
./your-application
```

Supported values for `SHOW_SETTINGS` (case-insensitive): "true", "1", "yes", "on"

---

## Error Handling

`load_config_with_param` provides comprehensive error handling, with all errors wrapped in the `ConfigError` enum:

- **InvalidLoadingParam**: Both `file` and `env_prefix` fields of `LoadingParam` are `None` - tells operations staff that at least one source needs to be configured
- **InvalidEnvConfig**: Environment prefix contains separator character
- **FileNotFound**: Specified configuration file does not exist
- **ShowSettingsParseError**: SHOW_SETTINGS environment variable cannot be parsed as boolean

`load_config` and `load_named_config` return `Option<T>`, returning `None` on loading failure, with error information logged.

---

## Feature Summary

- ✅ Three loading APIs to adapt to different use cases
- ✅ Environment variable support with configurable prefix and custom separator
- ✅ Enhanced logging with loading source information
- ✅ SHOW_SETTINGS environment variable for debugging configuration
- ✅ Comprehensive error handling with operations-friendly error messages
