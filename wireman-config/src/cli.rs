use crate::{install::install, setup::setup};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "wireman", version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,

    /// Optional path to the configuration file
    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Runs a health check and prompts configuration details.
    Check,
    /// Setup wireman and create a default configuration file.
    #[command(aliases = ["setup", "install"])]
    Init,
}

#[must_use]
pub fn parse() -> Args {
    let args = Args::parse();
    match args.command {
        Some(Command::Check) => {
            let _ = setup(true, &args);
        }
        Some(Command::Init) => {
            install();
        }
        None => {}
    }
    args
}
