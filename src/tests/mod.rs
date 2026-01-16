#[cfg(test)]
mod config_test;
#[cfg(test)]
mod loading_test;
#[cfg(test)]
mod show_settings_test;

#[cfg(test)]
use log::{debug, error, info, warn};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();
}
