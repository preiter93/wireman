use crate::{install::install, setup::setup};
use clap::{CommandFactory, FromArgMatches};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "wireman")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,

    /// Optional path to the configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Use local protobuf files
    #[arg(short, long)]
    pub local_protos: bool,
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
pub fn parse(version: &'static str) -> Args {
    let matches = Args::command().version(version).get_matches();
    let args = Args::from_arg_matches(&matches).unwrap();
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
