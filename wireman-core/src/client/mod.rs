#![allow(clippy::module_name_repetitions)]
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

/// Represents a `gRPC` client for making RPC calls.
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

    /// Make a unary `gRPC` call from the client.
    ///
    /// # Errors
    /// - `gRPC` client is not ready
    /// - Server call failed
    pub async fn unary(&mut self, req: &RequestMessage) -> Result<ResponseMessage> {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;
        let codec = codec::DynamicCodec::new(req.method_descriptor());
        let path = req.path();
        let request = req.clone().into_request();
        let response = self.grpc.unary(request, path, codec).await?.into_inner();
        Ok(response)
    }
}

/// Creates a new `gRPC` client and sends a message to a `gRPC` server.
/// This method is blocking.
///
/// # Errors
/// - Internal error calling the `gRPC` server
pub fn call_unary_blocking(req: &RequestMessage) -> Result<ResponseMessage> {
    let rt = create_runtime()?;
    let uri = Uri::try_from(req.address())
        .map_err(|_| Error::Internal(String::from("Failed to parse address")))?;
    let future = async move {
        let mut client = GrpcClient::new(uri, None);
        let response = client.unary(req).await?;
        Ok(response)
    };
    let result: Result<ResponseMessage> = rt.block_on(future);

    match result {
        Ok(response) => Ok(response),
        Err(err) => Err(Error::Internal(err.to_string())),
    }
}

/// Creates a new `gRPC` client and sends a message to a `gRPC` server.
/// This method is async.
///
/// # Errors
/// - Internal error calling the `gRPC` server
pub async fn call_unary_async(req: &RequestMessage) -> Result<ResponseMessage> {
    let uri = Uri::try_from(req.address())
        .map_err(|_| Error::Internal(String::from("Failed to parse address")))?;
    let mut client = GrpcClient::new(uri, None);
    client.unary(req).await
}

/// Creates a new Tokio runtime.
///
/// # Errors
/// - Internal: Failed to crate tokio runtime.
pub fn create_runtime() -> Result<Runtime> {
    Runtime::new().map_err(|_| Error::Internal(String::from("Failed to create runtime")))
}
