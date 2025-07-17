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
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::parse();
    if args.command.is_some() {
        return Ok(());
    }

    match init_from_env(&args) {
        Ok((cfg, file)) => App::run(cfg, file).await?,
        Err(err) => {
            if let config::error::Error::SetupError(err) = err {
                println!("Setup error: {err}");
                println!();
            } else {
                println!("An unknown error occurred:\n{err}");
                println!();
            }
            println!("Did you install wireman?");
            println!("   wireman install");
            println!();
            println!("Is your configuration correct?");
            println!("   wireman check");
        }
    }
    // App::run(cfg).await?;

    Ok(())
}
