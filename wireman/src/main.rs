#![allow(clippy::cast_possible_truncation, clippy::module_name_repetitions)]
#![allow(dead_code)]
mod app;
mod context;
mod events;
mod model;
mod term;
mod view;
mod widgets;
use app::App;
use config::{cli, init_from_env};
use std::{env, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    if env::args_os().count() > 1 {
        cli::parse();
        return Ok(());
    }

    match init_from_env() {
        Ok(cfg) => App::run(cfg).await?,
        Err(err) => match err {
            config::error::Error::SetupError(err) => {
                println!("Setup error: {err}");
                println!("Did you install wireman?");
                println!("   wireman install");
            }
            _ => println!("An error occured:\n{err}"),
        },
    }
    // App::run(cfg).await?;

    Ok(())
}
