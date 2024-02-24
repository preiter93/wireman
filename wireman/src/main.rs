#![allow(clippy::cast_possible_truncation, clippy::module_name_repetitions)]
#![allow(dead_code)]
mod app;
mod context;
mod input;
mod model;
mod term;
mod view;
mod widgets;
use app::App;
use config::{cli, init_from_env};
use std::{env, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// test
#[tokio::main]
async fn main() -> Result<()> {
    if env::args_os().count() > 1 {
        cli::parse();
        return Ok(());
    }

    let cfg = init_from_env()?;
    App::run(cfg).await?;

    Ok(())
}
