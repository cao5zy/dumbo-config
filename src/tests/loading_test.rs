#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ConfigError, EnvConfig, LoadingParam};
    use log::{debug, error, info, warn};
    use serde::{Deserialize, Serialize};
    use std::path::Path;

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestConfig {
        database_url: String,
        port: u16,
        debug: bool,
    }

    #[test]
    fn test_invalid_loading_param_both_none() {
        info!("Starting test: test_invalid_loading_param_both_none");
        let param = LoadingParam {
            file: None,
            env_prefix: None,
        };

        let result = crate::loading::load_config_with_param::<TestConfig>(&param);
        debug!("Result of load_config_with_param: {:?}", result);
        assert!(matches!(result, Err(ConfigError::InvalidLoadingParam)));
        info!("Completed test: test_invalid_loading_param_both_none successfully");
    }

    #[test]
    fn test_invalid_env_config_prefix_contains_separator() {
        info!("Starting test: test_invalid_env_config_prefix_contains_separator");
        let param = LoadingParam {
            file: None,
            env_prefix: Some(EnvConfig::new(
                "TEST_CONFIG".to_string(),
                Some("_".to_string()),
            )),
        };

        let result = crate::loading::load_config_with_param::<TestConfig>(&param);
        debug!("Result of load_config_with_param: {:?}", result);
        assert!(matches!(result, Err(ConfigError::InvalidEnvConfig { .. })));
        info!("Completed test: test_invalid_env_config_prefix_contains_separator successfully");
    }

    #[test]
    fn test_valid_env_config() {
        info!("Starting test: test_valid_env_config");
        let param = LoadingParam {
            file: None,
            env_prefix: Some(EnvConfig::new("TEST".to_string(), Some("_".to_string()))),
        };

        // This should not return InvalidEnvConfig error
        // (though it might fail for other reasons like missing env vars)
        let result = crate::loading::validate_loading_params(&param);
        debug!("Result of validate_loading_params: {:?}", result);
        assert!(result.is_ok());
        info!("Completed test: test_valid_env_config successfully");
    }

    #[test]
    fn test_env_config_default_separator() {
        info!("Starting test: test_env_config_default_separator");
        let env_config = EnvConfig::new("TEST".to_string(), None);
        debug!("Created EnvConfig with default separator");
        assert_eq!(env_config.get_separator(), "__");
        info!("Completed test: test_env_config_default_separator successfully");
    }

    #[test]
    fn test_env_config_custom_separator() {
        info!("Starting test: test_env_config_custom_separator");
        let env_config = EnvConfig::new("TEST".to_string(), Some("-".to_string()));
        debug!("Created EnvConfig with custom separator: -");
        assert_eq!(env_config.get_separator(), "-");
        info!("Completed test: test_env_config_custom_separator successfully");
    }

    #[test]
    fn test_no_env_vars_with_prefix() {
        info!("Starting test: test_no_env_vars_with_prefix");
        // Use a prefix that is very unlikely to exist in the environment
        let unique_prefix = "NONEXISTENT_PREFIX_12345";

        // Verify that no environment variables exist with this prefix
        let env_vars_with_prefix: Vec<String> = std::env::vars()
            .filter(|(key, _)| key.starts_with(unique_prefix))
            .map(|(key, _)| key)
            .collect();
        assert!(
            env_vars_with_prefix.is_empty(),
            "Test environment has unexpected variables with prefix {}",
            unique_prefix
        );

        let param = LoadingParam {
            file: None,
            env_prefix: Some(EnvConfig::new(unique_prefix.to_string(), None)),
        };

        // This should not return an error, but rather load an empty configuration
        // which will fail to deserialize into TestConfig due to missing fields
        let result = crate::loading::load_config_with_param::<TestConfig>(&param);
        debug!("Result of load_config_with_param: {:?}", result);

        // The result should be an error, but not EnvPrefixNotFound
        // It should be a config::ConfigError due to missing required fields
        match result {
            Err(ConfigError::Config(_)) => {
                // This is expected - config crate error due to missing fields
                info!("Got expected config error due to missing fields");
            }
            Err(e) => {
                panic!("Unexpected error type: {:?}", e);
            }
            Ok(_) => {
                panic!("Expected an error due to missing configuration fields");
            }
        }
        info!("Completed test: test_no_env_vars_with_prefix successfully");
    }
}
