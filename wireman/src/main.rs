#![allow(clippy::cast_possible_truncation, clippy::module_name_repetitions)]
#![allow(dead_code)]
mod app;
mod call;
mod context;
mod events;
mod model;
mod term;
mod view;
mod widgets;
use app::App;
use std::error::Error;
use wireman_config::cli::{self, Command};
use wireman_config::{check, init_from_env, install};
use wireman_theme::Theme;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let args = cli::parse(version);

    if let Some(command) = &args.command {
        match command {
            Command::Check => check(&args),
            Command::Init => install(),
            Command::Call {
                method,
                data,
                header,
                address,
            } => {
                if let Err(err) = call::run(&args, method, data, header, address.as_deref()).await {
                    call::print_error(err.as_ref());
                    std::process::exit(1);
                }
            }
            Command::Mcp => {
                if let Err(err) = run_mcp(&args).await {
                    call::print_error(err.as_ref());
                    std::process::exit(1);
                }
            }
        }
        return Ok(());
    }

    match init_from_env(&args) {
        Ok((cfg, file)) => {
            Theme::init(&wireman_theme::Config::new(cfg.ui.skin.clone()));
            App::run(cfg, file).await?;
        }
        Err(err) => {
            if let wireman_config::error::Error::SetupError(err) = err {
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

#[cfg(feature = "mcp")]
async fn run_mcp(args: &cli::Args) -> Result<()> {
    let (config, _) = init_from_env(args)?;
    wireman_mcp::run(config).await
}

#[cfg(not(feature = "mcp"))]
async fn run_mcp(_: &cli::Args) -> Result<()> {
    Err("wireman was built without the mcp feature".into())
}
