#![allow(clippy::cast_possible_truncation, clippy::module_name_repetitions)]
#![allow(dead_code)]
mod app;
mod commons;
mod context;
mod input;
mod model;
mod term;
mod view;
mod widgets;
use app::App;
use config::init_from_env;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = init_from_env()?;
    App::run(cfg).await?;

    Ok(())
}
