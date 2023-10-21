pub mod config;
pub mod error;
pub use config::Config;

/// Checks if all requirements are met and initializes the config.
///
/// # Errors
/// - config.json can not be loaded
pub fn init() -> error::Result<config::Config> {
    init_from_file("./config.json")
}

/// Checks if all requirements are met and initializes the config.
///
/// # Errors
/// - config.json can not be loaded
pub fn init_from_file(config_file: &str) -> error::Result<config::Config> {
    let cfg = config::Config::load(config_file)?;

    Ok(cfg)
}
