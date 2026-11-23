use crate::widgets::editor::pretty_format_json;
use futures::StreamExt;
use std::error::Error;
use wireman_config::{cli::Args, init_from_env};
use wireman_core::{
    client::{send, tls::TlsConfig},
    features::{expand_auth_header, expand_header},
    ProtoDescriptor,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Sends one request to a gRPC method and prints all responses as JSON.
pub async fn run(
    args: &Args,
    method_name: &str,
    data: &str,
    headers: &[String],
    address: Option<&str>,
) -> Result<()> {
    let (config, _) = init_from_env(args)?;
    let descriptor = ProtoDescriptor::new(config.includes(), config.files())?;

    let address = address
        .map(str::to_string)
        .or(config.server.default_address.clone())
        .ok_or("no address given and no default address in the config")?;

    let mut all_headers = Vec::new();
    if let Some(auth) = config.server.default_auth_header.as_deref() {
        if !auth.is_empty() {
            all_headers.push(("authorization".to_string(), auth.to_string()));
        }
    }
    for header in headers {
        let (key, value) = header
            .split_once(':')
            .ok_or(format!("header {header} is not in the form \"key: value\""))?;
        all_headers.push((key.trim().to_string(), value.trim().to_string()));
    }

    // Header commands like `$(getToken.sh)` come from the user, so they are run here.
    let all_headers: Vec<_> = all_headers
        .iter()
        .map(|(key, value)| (expand_header(key), expand_auth_header(value)))
        .collect();

    let tls = TlsConfig::from_settings(config.tls.use_native, config.tls.custom_cert.clone())?;

    let mut responses = send(&descriptor, method_name, data, &address, &all_headers, tls).await?;
    while let Some(response) = responses.next().await {
        let json = response?.message.to_json()?;
        println!("{}", pretty_format_json(&json).unwrap_or(json));
    }

    Ok(())
}

/// Prints an error and its causes to stderr.
pub fn print_error(err: &dyn Error) {
    eprintln!("Error: {err}");

    let mut source = err.source();
    while let Some(cause) = source {
        eprintln!("  caused by: {cause}");
        source = cause.source();
    }
}
