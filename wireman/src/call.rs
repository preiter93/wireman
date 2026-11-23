use crate::widgets::editor::pretty_format_json;
use futures::StreamExt;
use std::error::Error;
use wireman_config::{cli::Args, init_from_env};
use wireman_core::{
    client::{call, tls::TlsConfig},
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

    let (service_name, method_name) =
        split_method_name(method_name).ok_or(format!("{method_name} is not a full method name"))?;
    let method = descriptor
        .get_method_by_name(service_name, method_name)
        .ok_or(format!("method {service_name}.{method_name} not found"))?;
    let mut request = descriptor.request_from_json(&method, data)?;

    let address = address
        .map(str::to_string)
        .or(config.server.default_address.clone())
        .ok_or("no address given and no default address in the config")?;
    request.set_address(&address);

    if let Some(auth) = config.server.default_auth_header.as_deref() {
        if !auth.is_empty() {
            request.insert_metadata("authorization", &expand_auth_header(auth))?;
        }
    }
    for header in headers {
        let (key, value) = header
            .split_once(':')
            .ok_or(format!("header {header} is not in the form \"key: value\""))?;
        request.insert_metadata(
            &expand_header(key.trim()),
            &expand_auth_header(value.trim()),
        )?;
    }

    let tls = TlsConfig::from_settings(config.tls.use_native, config.tls.custom_cert.clone())?;

    // Client- and bidirectional-streaming methods send the request as their only message.
    let messages = futures::stream::iter(vec![request.clone()]);
    let mut responses = call(&request, messages, tls).await;
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

/// Splits a full method name like `proto.Greeter.SayHello` or
/// `proto.Greeter/SayHello` into its service and method name.
fn split_method_name(name: &str) -> Option<(&str, &str)> {
    name.rsplit_once(['/', '.'])
}
