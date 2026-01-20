
# dumbo-config  
dumbo-config is a config loader.  


## Features  
- Load project configurations  
- Enhanced logging with loading source information
- Environment variable support with prefix and custom separator
- SHOW_SETTINGS environment variable for debugging configuration
- Comprehensive error handling with运维-friendly error messages


## Quick Start  
Your configuration file.
```yaml
name: "test config"
value: 32
```

### configuration file name
The following file names is qualified.
- config.yml
- config.yaml
- config.{ENV}.yml
- config.{ENV}.yaml
Where `ENV` is the value of the environment variable "ENV". If "ENV" is not set, it defaults to searching `config.yml` and `config.yaml`.

You can also use `load_named_config` with specified config file.

Rust file for loading TestConfig
```rust
use dumbo_config::{load_config, load_named_config};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
}

...

let config: Option<TestConfig> = load_config();

let config_path: Path = ...;

let config: Option<TestConfig> = load_named_config(&config_path);
```


## Advanced Configuration Loading

For more advanced configuration loading scenarios, you can use the `load_config_with_param` function with `LoadingParam`.

### Loading Parameters

The `LoadingParam` struct allows you to specify both file and environment variable sources:

```rust
use dumbo_config::{LoadingParam, EnvConfig, load_config_with_param};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
struct AppConfig {
    database_url: String,
    port: u16,
    debug: bool,
}

// Load from file only
let param = LoadingParam {
    file: Some(Path::new("config.yaml")),
    env_prefix: None,
};
let config: AppConfig = load_config_with_param(&param)?;

// Load from environment variables only
let param = LoadingParam {
    file: None,
    env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
};
let config: AppConfig = load_config_with_param(&param)?;

// Load from both file and environment variables (env vars take precedence)
let param = LoadingParam {
    file: Some(Path::new("config.yaml")),
    env_prefix: Some(EnvConfig::new("MY_APP".to_string(), None)),
};
let config: AppConfig = load_config_with_param(&param)?;
```

### Environment Configuration

The `EnvConfig` struct defines how to load configuration from environment variables:

- `name`: The prefix for environment variables (e.g., "MYAPP" for variables like "MYAPP_DATABASE_URL")
- `separator`: The separator character used in environment variable names. 鉴于Rust中的字段名称通常也用"_"分割，因此，环境变量的分割符默认为"__"

**Example with MY_APP prefix:**
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

With `EnvConfig::new("MY_APP".to_string(), None)`, the corresponding environment variables would be:
```bash
# Top-level fields
export MY_APP__HOST="localhost"
export MY_APP__PORT="5432"

# Nested fields (using double underscore as separator)
export MY_APP__CREDENTIALS__USERNAME="myuser"
export MY_APP__CREDENTIALS__PASSWORD="mypass"
```

**Note**: The environment variable prefix should not contain the separator character. For example, if your prefix is "RESUME_AGENT" and separator is "_", this will cause a configuration loading error.

### Logging and Debugging

The library provides detailed logging at the INFO level:

- Always logs which configuration sources are being used
- When `env_prefix` is set and the `SHOW_SETTINGS` environment variable is set to "true" (case-insensitive), logs that configuration was loaded successfully
- If no environment variables are found with the specified prefix, a warning is logged and configuration loading continues without environment variables

**Note**: The `SHOW_SETTINGS` environment variable is only checked when `env_prefix` is configured. This is because if no `env_prefix` is set, it means the user is loading configuration directly from files, and they can simply inspect the configuration files directly to view the settings. When environment variables are used for configuration (via `env_prefix`), the actual values may not be easily visible, so `SHOW_SETTINGS` provides a way to log the loaded configuration for debugging purposes.

To enable configuration debugging, set the `SHOW_SETTINGS` environment variable if the `env_prefix` is 'MY_APP'

```bash
export MY_APP__SHOW_SETTINGS=true
./your-application
```

Supported values for `SHOW_SETTINGS` (case-insensitive): "true", "1", "yes", "on"

### Error Handling

The library provides comprehensive error handling with运维-friendly error messages:

- **InvalidLoadingParam**: Both file and env_prefix are None - tells Ops staff what needs to be configured
- **InvalidEnvConfig**: Environment prefix contains separator character
- **FileNotFound**: Specified configuration file does not exist
- **ShowSettingsParseError**: SHOW_SETTINGS environment variable cannot be parsed as boolean

All errors are wrapped in the `ConfigError` enum and implement the standard `Error` trait.
