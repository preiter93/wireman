use crate::{install::install, setup::setup};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "wireman", version)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Runs a health check and prompts configuration details.
    Check,
    /// Setup wireman and create a default configuration file.
    #[command(aliases = ["setup", "install"])]
    Init,
}

pub fn parse() {
    let app = App::parse();
    match app.command {
        Command::Check => {
            let _ = setup(true);
        }
        Command::Init => {
            install();
        }
    }
}
