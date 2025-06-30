use serde::Deserialize;
use serde_yaml;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn load_config_from_file<T, P>(path: P) -> Option<T>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<std::path::Path>,
{
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    serde_yaml::from_str(&contents).ok()
}

/// Loads configuration from environment-specific or default YAML files.
///
/// This function searches for configuration files in the following order:
/// 1. `config.{ENV}.yml`
/// 2. `config.{ENV}.yaml`
/// 3. `config.yml`
/// 4. `config.yaml`
///
/// Where `ENV` is the value of the environment variable "ENV". If "ENV" is not set,
/// it defaults to searching `config.yml` and `config.yaml`.
///
/// # Returns
/// `Some(T)` if a valid configuration file is found and parsed successfully,
/// `None` otherwise.
///
/// # Example
/// ```
/// use serde::Deserialize;
/// 
/// #[derive(Deserialize)]
/// struct AppConfig {
///     server_port: u16,
///     debug_mode: bool,
/// }
///
/// // Set ENV variable (optional)
/// std::env::set_var("ENV", "production");
///
/// if let Some(config) = dumbo_config::load_config::<AppConfig>() {
///     println!("Server port: {}", config.server_port);
/// } else {
///     eprintln!("Failed to load configuration");
/// }
/// ```
pub fn load_config<T>() -> Option<T>
where
    T: for<'de> Deserialize<'de>,
{
    let env_var = env::var("ENV").ok();

    let candidates = match env_var.as_deref() {
        Some(env) if !env.is_empty() => {
            vec![
                format!("config.{}.yml", env),
                format!("config.{}.yaml", env),
            ]
        }
        _ => vec!["config.yml".to_string(), "config.yaml".to_string()],
    };

    for file_name in candidates {
        if let Some(config) = load_config_from_file::<T, _>(&file_name) {
            return Some(config);
        }
    }

    None
}