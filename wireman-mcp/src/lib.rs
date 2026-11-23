//! An MCP server that lets AI agents list, describe and call the gRPC methods
//! of a wireman configuration.
use futures::StreamExt;
use rmcp::{
    handler::server::wrapper::Parameters, tool, tool_handler, tool_router, transport::stdio,
    ServerHandler, ServiceExt,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    time::Duration,
};
use wireman_config::Config;
use wireman_core::{
    client::{send, tls::TlsConfig},
    descriptor::schema::{comment, json_schema},
    features::{expand_auth_header, wireman_call},
    MethodDescriptor, ProtoDescriptor,
};

/// The address used when neither the agent nor the wireman config gives one.
const DEFAULT_ADDRESS: &str = "http://localhost:50051";

/// The most responses `call_method` returns.
const MAX_RESPONSES: usize = 100;

/// How long `call_method` waits for responses, since some streams never end.
const MAX_WAIT: Duration = Duration::from_secs(30);

/// Runs the MCP server on stdin and stdout until the client disconnects.
///
/// # Errors
///
/// Errors if the protos cannot be loaded or the connection to the client fails.
pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let descriptor = ProtoDescriptor::new(config.includes(), config.files())?;

    let server = WiremanServer { config, descriptor }.serve(stdio()).await?;
    server.waiting().await?;

    Ok(())
}

#[derive(Clone)]
struct WiremanServer {
    config: Config,
    descriptor: ProtoDescriptor,
}

#[derive(Deserialize, JsonSchema)]
struct DescribeMethodRequest {
    /// The full method name, for example proto.Greeter.SayHello
    method: String,
}

#[derive(Deserialize, JsonSchema)]
struct CallMethodRequest {
    /// The full method name, for example proto.Greeter.SayHello
    method: String,
    /// The request message as JSON, as described by describe_method
    request: Value,
    /// Additional headers, for example {"x-trace-id": "abc"}. They are sent as given.
    #[serde(default)]
    headers: BTreeMap<String, String>,
    /// The server address, for example http://localhost:50051. Defaults to the address in the wireman config, or http://localhost:50051.
    address: Option<String>,
}

#[tool_handler(
    name = "wireman-mcp",
    instructions = "Use list_services to find a gRPC method, describe_method to get the JSON Schema and an example of its request, and call_method to send it, or call_command to get a wireman call command the user can run themselves."
)]
impl ServerHandler for WiremanServer {}

#[tool_router]
impl WiremanServer {
    #[tool(description = "Lists the gRPC services and methods from the wireman config")]
    async fn list_services(&self) -> String {
        let services: Vec<Value> = self
            .descriptor
            .get_services()
            .iter()
            .map(|service| {
                let methods: Vec<Value> = self
                    .descriptor
                    .get_methods(service)
                    .iter()
                    .map(|method| {
                        json!({
                            "method": method.full_name(),
                            "kind": method_kind(method),
                            "description": comment(&method.parent_file(), method.path()),
                        })
                    })
                    .collect();

                json!({
                    "service": service.full_name(),
                    "description": comment(&service.parent_file(), service.path()),
                    "methods": methods,
                })
            })
            .collect();

        Value::Array(services).to_string()
    }

    #[tool(
        description = "Describes a gRPC method: its kind, comment, the JSON Schema of its request and an example request"
    )]
    async fn describe_method(
        &self,
        Parameters(request): Parameters<DescribeMethodRequest>,
    ) -> Result<String, String> {
        let method = self
            .descriptor
            .get_method_by_full_name(&request.method)
            .ok_or(format!("method {} not found", request.method))?;

        let mut example = self.descriptor.get_request(&method);
        example.message_mut().apply_template();
        let example = serde_json::to_value(example.message()).map_err(|err| error_message(&err))?;

        let description = json!({
            "method": method.full_name(),
            "kind": method_kind(&method),
            "description": comment(&method.parent_file(), method.path()),
            "request_schema": json_schema(&method.input()),
            "example_request": example,
        });
        Ok(description.to_string())
    }

    #[tool(
        description = "Calls a gRPC method and returns the responses received within 30 seconds as a JSON array (at most 100). Client and bidirectional streaming methods send the request as their only message."
    )]
    async fn call_method(
        &self,
        Parameters(request): Parameters<CallMethodRequest>,
    ) -> Result<String, String> {
        let address = self.address(request.address);

        // Only the headers from the config may run commands like `$(getToken.sh)`.
        let mut headers: Vec<(String, String)> = self
            .config
            .default_headers()
            .into_iter()
            .map(|(key, value)| (key, expand_auth_header(&value)))
            .collect();
        headers.extend(request.headers);

        let tls = &self.config.tls;
        let tls = TlsConfig::from_settings(tls.use_native, tls.custom_cert.clone())
            .map_err(|err| error_message(&err))?;

        let mut messages = Vec::new();
        let collect = async {
            let mut responses = send(
                &self.descriptor,
                &request.method,
                &request.request.to_string(),
                &address,
                &headers,
                tls,
            )
            .await
            .map_err(|err| error_message(&err))?
            .take(MAX_RESPONSES);

            while let Some(response) = responses.next().await {
                let response = response.map_err(|err| error_message(&err))?;
                let message =
                    serde_json::to_value(&response.message).map_err(|err| error_message(&err))?;
                messages.push(message);
            }
            Ok::<(), String>(())
        };
        let result = tokio::time::timeout(MAX_WAIT, collect).await;

        // A stream that is still open after the time limit returns what arrived so far.
        match result {
            Ok(result) => result?,
            Err(_) if messages.is_empty() => {
                return Err(format!("no response within {} seconds", MAX_WAIT.as_secs()));
            }
            Err(_) => {}
        }

        Ok(Value::Array(messages).to_string())
    }

    #[tool(
        description = "Returns a wireman call shell command that sends the request, without sending it, so the user can run it themselves"
    )]
    async fn call_command(
        &self,
        Parameters(request): Parameters<CallMethodRequest>,
    ) -> Result<String, String> {
        let method = self
            .descriptor
            .get_method_by_full_name(&request.method)
            .ok_or(format!("method {} not found", request.method))?;

        let mut headers: HashMap<String, String> =
            self.config.default_headers().into_iter().collect();
        headers.extend(request.headers);
        let address = self.address(request.address);

        Ok(wireman_call(
            &method,
            &request.request.to_string(),
            &headers,
            &address,
        ))
    }
}

impl WiremanServer {
    /// Returns the given address, or else the one from the config, or else [`DEFAULT_ADDRESS`].
    fn address(&self, address: Option<String>) -> String {
        address
            .or(self.config.server.default_address.clone())
            .unwrap_or(DEFAULT_ADDRESS.to_string())
    }
}

fn method_kind(method: &MethodDescriptor) -> &'static str {
    match (method.is_client_streaming(), method.is_server_streaming()) {
        (false, false) => "unary",
        (false, true) => "server streaming",
        (true, false) => "client streaming",
        (true, true) => "bidirectional streaming",
    }
}

/// Returns an error and its causes, one per line.
fn error_message(err: &dyn Error) -> String {
    let mut message = err.to_string();

    let mut source = err.source();
    while let Some(cause) = source {
        message.push_str(&format!("\n  caused by: {cause}"));
        source = cause.source();
    }
    message
}
