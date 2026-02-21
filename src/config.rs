use serde::Deserialize;
use serde_yaml;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn load_config_from_file<T, P>(path: P) -> Option<T>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path> + std::fmt::Debug,
{
    // 1. 将传入的路径转换为绝对路径
    let path_buf: PathBuf = path.as_ref().to_path_buf();
    let absolute_path = match path_buf.canonicalize() {
        Ok(abs_path) => abs_path,
        Err(e) => {
            log::warn!(
                "failed to convert path to absolute path: {:?}, error: {}",
                path_buf,
                e
            );
            return None;
        }
    };

    // 2. 日志输出绝对路径（更易排查问题）
    log::debug!("trying to read config file from: {:?}", absolute_path);

    // 3. 使用绝对路径打开文件
    let mut file = File::open(&absolute_path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    // 4. 反序列化配置内容
    match serde_yaml::from_str(&contents) {
        Ok(config) => Some(config),
        Err(e) => {
            // 输出详细的反序列化错误，包含文件路径和具体错误信息
            log::error!(
                "failed to deserialize config file: {:?}, error: {}",
                absolute_path,
                e
            );
            // 可选：输出配置文件内容片段，方便定位格式问题（如果内容不敏感）
            // log::debug!("config file contents: {}", contents);
            None
        }
    }
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

/// Loads configuration from a specific YAML file path.
///
/// This function is similar to `load_config_from_file` but more explicitly accepts
/// a path parameter and provides better documentation. It is useful when you need
/// to load configuration from non-standard locations or specifically named files.
///
/// # Arguments
/// * `config_path` - The path to the configuration file to load.
///
/// # Returns
/// `Some(T)` if the configuration file is found and parsed successfully,
/// `None` otherwise.
///
/// # Example
/// ```
/// use serde::Deserialize;
/// use std::path::Path;
///
/// #[derive(Deserialize)]
/// struct AppConfig {
///     server_port: u16,
///     debug_mode: bool,
/// }
///
/// if let Some(config) = dumbo_config::load_named_config::<AppConfig>(Path::new("custom-config.yml")) {
///     println!("Server port: {}", config.server_port);
/// } else {
///     eprintln!("Failed to load configuration");
/// }
/// ```
pub fn load_named_config<T>(config_path: &Path) -> Option<T>
where
    T: for<'de> Deserialize<'de>,
{
    load_config_from_file(config_path)
}
