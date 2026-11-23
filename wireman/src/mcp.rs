use crate::call::{self, error_message, find_method, send};
use futures::StreamExt;
use rmcp::{
    handler::server::wrapper::Parameters, tool, tool_handler, tool_router, transport::stdio,
    ServerHandler, ServiceExt,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::BTreeMap, time::Duration};
use wireman_config::{cli::Args, init_from_env, Config};
use wireman_core::{
    descriptor::schema::{comment, json_schema},
    MethodDescriptor, ProtoDescriptor,
};

/// The most responses `call_method` returns.
const MAX_RESPONSES: usize = 100;

/// How long `call_method` waits for responses, since some streams never end.
const MAX_WAIT: Duration = Duration::from_secs(30);

/// Runs the MCP server on stdin and stdout until the client disconnects.
pub async fn run(args: &Args) -> call::Result<()> {
    let (config, _) = init_from_env(args)?;
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
    /// Additional headers, for example {"x-trace-id": "abc"}
    #[serde(default)]
    headers: BTreeMap<String, String>,
    /// The server address, for example http://localhost:50051. Defaults to the address in the wireman config.
    address: Option<String>,
}

#[tool_handler(
    name = "wireman",
    instructions = "Use list_services to find a gRPC method, describe_method to get the JSON Schema and an example of its request, and call_method to send it."
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
        let method = find_method(&self.descriptor, &request.method)
            .map_err(|err| error_message(err.as_ref()))?;

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
        let headers: Vec<(String, String)> = request.headers.into_iter().collect();

        let mut messages = Vec::new();
        let collect = async {
            let mut responses = send(
                &self.config,
                &self.descriptor,
                &request.method,
                &request.request.to_string(),
                &headers,
                request.address.as_deref(),
            )
            .await
            .map_err(|err| error_message(err.as_ref()))?
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
}

fn method_kind(method: &MethodDescriptor) -> &'static str {
    match (method.is_client_streaming(), method.is_server_streaming()) {
        (false, false) => "unary",
        (false, true) => "server streaming",
        (true, false) => "client streaming",
        (true, true) => "bidirectional streaming",
    }
}
