use crate::load_config;
use serde::{Deserialize, Serialize};
use serde_yaml;
use serial_test::serial;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    fn create_temp_config(content: &str, filename: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let temp_path = file.path().to_path_buf();
        let dest_path = std::path::Path::new(filename);
        fs::copy(temp_path, dest_path).unwrap();
        file
    }

    #[test]
    #[serial]
    fn test_load_config_yml() {
        let config_content = r#"
name: test
value: 42
"#;
        let _file = create_temp_config(config_content, "config.yml");

        let config: Option<TestConfig> = load_config();
        assert!(config.is_some());
        assert_eq!(
            config.unwrap(),
            TestConfig {
                name: "test".to_string(),
                value: 42,
            }
        );

        fs::remove_file("config.yml").unwrap();
    }

    #[test]
    #[serial]
    fn test_load_config_yaml() {
        let config_content = r#"
name: test
value: 42
"#;
        let _file = create_temp_config(config_content, "config.yaml");

        let config: Option<TestConfig> = load_config();
        assert!(config.is_some());
        assert_eq!(
            config.unwrap(),
            TestConfig {
                name: "test".to_string(),
                value: 42,
            }
        );

        fs::remove_file("config.yaml").unwrap();
    }

    #[test]
    #[serial]
    fn test_load_config_with_env() {
        let config_content = r#"
name: prod
value: 100
"#;
        let _file = create_temp_config(config_content, "config.prod.yml");
        env::set_var("ENV", "prod");

        let config: Option<TestConfig> = load_config();
        assert!(config.is_some());
        assert_eq!(
            config.unwrap(),
            TestConfig {
                name: "prod".to_string(),
                value: 100,
            }
        );

        fs::remove_file("config.prod.yml").unwrap();
        env::remove_var("ENV");
    }
}
