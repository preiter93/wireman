use crate::widgets::editor::pretty_format_json;
use futures::StreamExt;
use std::error::Error;
use wireman_config::{cli::Args, init_from_env, Config};
use wireman_core::{
    client::{call, tls::TlsConfig, ResponseStream},
    features::{expand_auth_header, expand_header},
    MethodDescriptor, ProtoDescriptor,
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
    let headers = headers
        .iter()
        .map(|header| parse_header(header))
        .collect::<Result<Vec<_>>>()?;

    let mut responses = send(&config, &descriptor, method_name, data, &headers, address).await?;
    while let Some(response) = responses.next().await {
        let json = response?.message.to_json()?;
        println!("{}", pretty_format_json(&json).unwrap_or(json));
    }

    Ok(())
}

/// Sends one request to a gRPC method and returns its responses.
pub async fn send(
    config: &Config,
    descriptor: &ProtoDescriptor,
    method_name: &str,
    data: &str,
    headers: &[(String, String)],
    address: Option<&str>,
) -> Result<ResponseStream> {
    let method = find_method(descriptor, method_name)?;
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
    for (key, value) in headers {
        request.insert_metadata(&expand_header(key), &expand_header(value))?;
    }

    let tls = TlsConfig::from_settings(config.tls.use_native, config.tls.custom_cert.clone())?;

    // Client- and bidirectional-streaming methods send the request as their only message.
    let messages = futures::stream::iter(vec![request.clone()]);
    Ok(call(&request, messages, tls).await)
}

/// Finds a method by its full name like `proto.Greeter.SayHello` or
/// `proto.Greeter/SayHello`.
pub fn find_method(descriptor: &ProtoDescriptor, name: &str) -> Result<MethodDescriptor> {
    let (service_name, method_name) = name
        .rsplit_once(['/', '.'])
        .ok_or(format!("{name} is not a full method name"))?;

    let method = descriptor
        .get_method_by_name(service_name, method_name)
        .ok_or(format!("method {service_name}.{method_name} not found"))?;
    Ok(method)
}

/// Returns an error and its causes, one per line.
pub fn error_message(err: &dyn Error) -> String {
    let mut message = err.to_string();

    let mut source = err.source();
    while let Some(cause) = source {
        message.push_str(&format!("\n  caused by: {cause}"));
        source = cause.source();
    }
    message
}

/// Parses a header in the form `key: value`.
fn parse_header(header: &str) -> Result<(String, String)> {
    let (key, value) = header
        .split_once(':')
        .ok_or(format!("header {header} is not in the form \"key: value\""))?;
    Ok((key.trim().to_string(), value.trim().to_string()))
}
