#[cfg(test)]
mod tests {
    use super::*;
    use crate::loading::test_should_show_settings;
    use crate::models::{EnvConfig, LoadingParam};
    use log::{debug, info};
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::path::Path;

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestConfig {
        database_url: String,
        port: u16,
        debug: bool,
    }

    // Helper function to ensure environment variable cleanup
    fn with_env_var<F>(key: &str, value: &str, test_fn: F)
    where
        F: FnOnce(),
    {
        env::set_var(key, value);
        test_fn();
        env::remove_var(key);
    }

    // Helper function to ensure environment variable cleanup for multiple values
    fn with_env_var_cleanup<F>(key: &str, test_fn: F)
    where
        F: FnOnce(),
    {
        test_fn();
        if env::var_os(key).is_some() {
            env::remove_var(key);
        }
    }

    #[test]
    fn test_should_show_settings_no_env_prefix() {
        info!("Starting test: test_should_show_settings_no_env_prefix");

        // Test with no env prefix (only file)
        let param = LoadingParam {
            file: Some(Path::new("dummy.yaml")),
            env_prefix: None,
        };

        // Even if SHOW_SETTINGS is true, should return false when no env_prefix
        with_env_var("TEST__SHOW_SETTINGS", "true", || {
            let result = test_should_show_settings(&param);
            assert_eq!(result, false);
        });

        info!("Completed test: test_should_show_settings_no_env_prefix successfully");
    }

    #[test]
    fn test_should_show_settings_with_env_prefix_and_show_settings_true() {
        info!("Starting test: test_should_show_settings_with_env_prefix_and_show_settings_true");

        let param = LoadingParam {
            file: None,
            env_prefix: Some(EnvConfig::new("TEST".to_string(), Some("__".to_string()))),
        };

        // Test various truthy values for SHOW_SETTINGS
        let truthy_values = vec![
            "true", "TRUE", "True", "1", "yes", "YES", "Yes", "on", "ON", "On",
        ];

        for value in truthy_values {
            with_env_var("TEST__SHOW_SETTINGS", value, || {
                let result = test_should_show_settings(&param);
                assert_eq!(result, true, "Failed for SHOW_SETTINGS value: {}", value);
            });
        }

        info!("Completed test: test_should_show_settings_with_env_prefix_and_show_settings_true successfully");
    }

    #[test]
    fn test_should_show_settings_with_env_prefix_and_show_settings_false() {
        info!("Starting test: test_should_show_settings_with_env_prefix_and_show_settings_false");

        let param = LoadingParam {
            file: None,
            env_prefix: Some(EnvConfig::new("TEST".to_string(), Some("__".to_string()))),
        };

        // Test falsy values for SHOW_SETTINGS
        let falsy_values = vec![
            "false", "FALSE", "False", "0", "no", "NO", "No", "off", "OFF", "Off", "invalid",
        ];

        for value in falsy_values {
            with_env_var("TEST__SHOW_SETTINGS", value, || {
                let result = test_should_show_settings(&param);
                assert_eq!(result, false, "Failed for SHOW_SETTINGS value: {}", value);
            });
        }

        // Test when SHOW_SETTINGS is not set at all
        // Ensure it's not set before testing
        if env::var_os("TEST__SHOW_SETTINGS").is_some() {
            env::remove_var("TEST__SHOW_SETTINGS");
        }
        let result = test_should_show_settings(&param);
        assert_eq!(result, false);

        info!("Completed test: test_should_show_settings_with_env_prefix_and_show_settings_false successfully");
    }

    #[test]
    fn test_should_show_settings_both_file_and_env_prefix() {
        info!("Starting test: test_should_show_settings_both_file_and_env_prefix");

        // When both file and env_prefix are provided, env_prefix takes precedence
        let param = LoadingParam {
            file: Some(Path::new("dummy.yaml")),
            env_prefix: Some(EnvConfig::new("TEST".to_string(), Some("__".to_string()))),
        };

        // SHOW_SETTINGS=true should return true
        with_env_var("TEST__SHOW_SETTINGS", "true", || {
            let result = test_should_show_settings(&param);
            assert_eq!(result, true);
        });

        // SHOW_SETTINGS=false should return false
        with_env_var("TEST__SHOW_SETTINGS", "false", || {
            let result = test_should_show_settings(&param);
            assert_eq!(result, false);
        });

        info!("Completed test: test_should_show_settings_both_file_and_env_prefix successfully");
    }

    #[test]
    fn test_integration_show_settings_behavior() {
        info!("Starting test: test_integration_show_settings_behavior");

        // Set up environment variables for actual integration test
        let env_vars = vec![
            ("TEST__DATABASE_URL", "postgresql://localhost/test"),
            ("TEST__PORT", "5432"),
            ("TEST__DEBUG", "true"),
            ("TEST__SHOW_SETTINGS", "true"),
        ];

        // Set all environment variables
        for (key, value) in &env_vars {
            env::set_var(key, value);
        }

        let param = LoadingParam {
            file: None,
            env_prefix: Some(EnvConfig::new("TEST".to_string(), Some("__".to_string()))),
        };

        // This should work and the should_show_settings logic should be exercised
        let result = crate::loading::load_config_with_param::<TestConfig>(&param);
        assert!(result.is_ok());

        // Clean up all environment variables
        for (key, _) in env_vars {
            env::remove_var(key);
        }

        info!("Completed test: test_integration_show_settings_behavior successfully");
    }
}
