use clap::{CommandFactory, FromArgMatches};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "wireman")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,

    /// Optional path to the configuration file
    #[arg(short, long, global = true)]
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
    /// Sends one request to a gRPC method and prints the response as JSON.
    Call {
        /// The full method name, for example proto.Greeter.SayHello
        method: String,
        /// The request message as JSON
        #[arg(short, long, default_value = "{}")]
        data: String,
        /// An additional header as "key: value", can be repeated
        #[arg(short = 'H', long)]
        header: Vec<String>,
        /// The server address, defaults to the address in the config
        #[arg(short, long)]
        address: Option<String>,
    },
    /// Runs an MCP server on stdin and stdout, so AI agents can list and call gRPC methods.
    Mcp,
}

#[must_use]
pub fn parse(version: &'static str) -> Args {
    let matches = Args::command().version(version).get_matches();
    Args::from_arg_matches(&matches).unwrap()
}
