mod config;

pub use config::{load_config, load_named_config};

#[cfg(test)]
mod tests;
