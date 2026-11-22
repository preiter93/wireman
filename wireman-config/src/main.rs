use wireman_config::cli::{self, Command};
use wireman_config::{check, install};

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let args = cli::parse(version);

    match args.command {
        Some(Command::Check) => check(&args),
        Some(Command::Init) => install(),
        None => {}
    }
}
