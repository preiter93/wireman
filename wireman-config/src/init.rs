use std::{env::var, error::Error};

use crate::{Config, ENV_CONFIG_DIR};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn init_from_env() -> Result<Config> {
    fn env_file() -> String {
        if let Ok(current_dir) = std::env::current_dir() {
            let config_path = current_dir.join("config.toml");
            if config_path.exists() && config_path.is_file() {
                return format!("{}/config.toml", current_dir.to_str().unwrap());
            }
        }
        var(ENV_CONFIG_DIR).unwrap_or("config.toml".to_string())
    }
    let cfg_file = env_file();
    let cfg = Config::load(&cfg_file)?;
    Ok(cfg)
}
