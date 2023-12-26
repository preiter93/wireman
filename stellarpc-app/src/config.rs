use crate::term::Term;
pub use config::Config;
use std::{error::Error, path::PathBuf, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct AppConfig {
    pub history: PathBuf,
}

impl AppConfig {
    pub fn new(env: &Config) -> Result<Self> {
        let path = env.history();
        let path_str = if path.is_empty() { "./history" } else { &path };
        let history = PathBuf::from_str(path_str).map_err(|err| {
            Term::stop().unwrap();
            err
        })?;
        Ok(AppConfig { history })
    }
}
