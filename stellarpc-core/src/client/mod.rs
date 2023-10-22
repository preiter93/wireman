//! Module for all grpc related stuff
mod codec;
pub mod tls;

use crate::descriptor::RequestMessage;
use crate::descriptor::ResponseMessage;
use crate::error::Error;
use crate::Result;
use tls::TlsConfig;
use tokio::runtime::Runtime;
use tonic::transport::Uri;
use tonic::{client::Grpc, transport::Channel};

/// Represents a gRPC client for making RPC calls.
#[derive(Clone, Debug)]
pub struct GrpcClient {
    grpc: Grpc<Channel>,
}

impl GrpcClient {
    /// Returns a new Grpc Client. if no tls is given, the standard tonic
    /// client is used.
    pub fn new<T: Into<Uri>>(uri: T, tls: Option<TlsConfig>) -> Self {
        let builder = Channel::builder(uri.into());
        let channel = if let Some(tls) = tls {
            // Build a channel with custom tls settings
            let connector = tls.get_connector_from_tls();
            builder.connect_with_connector_lazy(connector)
        } else {
            // The standard tonic channel
            builder.connect_lazy()
        };

        GrpcClient {
            grpc: Grpc::new(channel),
        }
    }

    /// Make a unary gRPC call from the client.
    ///
    /// # Errors
    /// - gRPC client is not ready
    /// - Server call failed
    pub async fn unary(&mut self, req: &RequestMessage) -> Result<ResponseMessage> {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;
        let codec = codec::DynamicCodec::new(req.method_descriptor());
        let path = req.path();
        let request = req.clone().into_request();
        let response = self.grpc.unary(request, path, codec).await?.into_inner();
        Ok(response)
    }

    /// Make a unary grpc call from the client. The call is
    /// wrapped in a tokio runtime to run asynchronously.
    ///
    /// # Errors
    /// - gRPC client is not ready
    /// - Server call failed
    pub fn unary_with_runtime(&mut self, req: &RequestMessage) -> Result<String> {
        let runtime = create_runtime()?;
        let future = self.unary(req);
        let result = runtime.block_on(future);

        match result {
            Ok(response) => Ok(response.message.to_json()?),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }
}

/// Creates a new gRPC client and sends a message to a gRPC server.
/// This method is blocking.
///
/// # Errors
/// - Internal error calling the gRPC server
pub fn call_unary_blocking<T: Into<Uri>>(uri: T, req: &RequestMessage) -> Result<ResponseMessage> {
    let runtime = create_runtime()?;
    let future = async_call_unary(uri, req);
    let result = runtime.block_on(future);

    match result {
        Ok(response) => Ok(response),
        Err(err) => Err(Error::InternalError(err.to_string())),
    }
}

/// Creates a new gRPC client and sends a message to a gRPC server.
/// This method is non-blocking.
///
/// # Errors
/// - Internal error calling the gRPC server
async fn async_call_unary<T: Into<Uri>>(uri: T, req: &RequestMessage) -> Result<ResponseMessage> {
    let mut client = GrpcClient::new(uri, None);
    let response = client.unary(req).await?;
    Ok(response)
}

/// Creates a new Tokio runtime.
fn create_runtime() -> Result<Runtime> {
    Runtime::new().map_err(|_| Error::InternalError("Failed to create runtime".to_string()))
}
