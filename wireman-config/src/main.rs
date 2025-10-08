use wireman_config::cli;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let _ = cli::parse(version);
}
