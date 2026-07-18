#![allow(clippy::module_name_repetitions)]
pub(crate) mod codec;
pub mod reflection;
pub mod tls;

use crate::descriptor::response::StreamingResponse;
use crate::descriptor::RequestMessage;
use crate::descriptor::ResponseMessage;
use crate::error::Error;
use crate::Result;
use tls::TlsConfig;
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
    ///
    /// # Errors
    ///
    /// Errors if tls config cannot be build.
    pub fn new<T: Into<Uri>>(uri: T, tls_config: Option<TlsConfig>) -> Result<Self> {
        let builder = Channel::builder(uri.into());

        let channel = if let Some(tls_config) = tls_config {
            builder.tls_config(tls_config.0)?.connect_lazy()
        } else {
            builder.connect_lazy()
        };

        Ok(GrpcClient {
            grpc: Grpc::new(channel),
        })
    }

    /// Make a unary `gRPC` call.
    ///
    /// # Errors
    /// - `gRPC` client is not ready
    /// - Server call failed
    pub async fn unary(&mut self, request: &RequestMessage) -> Result<ResponseMessage> {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;

        let path = request.path();
        let codec = request.codec();

        let request = request.clone().into();
        let response = self.grpc.unary(request, path, codec).await?;

        Ok(response.into_inner())
    }

    /// Make a server-streaming `gRPC` call.
    ///
    /// # Errors
    /// - `gRPC` client is not ready
    /// - Server call failed
    pub async fn server_streaming(
        &mut self,
        request: &RequestMessage,
    ) -> Result<StreamingResponse> {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;

        let path = request.path();
        let codec = request.codec();

        let request = request.clone().into();
        let response = self.grpc.server_streaming(request, path, codec).await?;

        Ok(StreamingResponse::new(response.into_inner()))
    }

    /// Make a client-streaming `gRPC` call.
    ///
    /// # Errors
    /// - `gRPC` client is not ready
    /// - Server call failed
    pub async fn client_streaming<S>(
        &mut self,
        head: &RequestMessage,
        messages: S,
    ) -> Result<ResponseMessage>
    where
        S: tokio_stream::Stream<Item = RequestMessage> + Send + 'static,
    {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;

        let path = head.path();
        let codec = head.codec();

        let request = into_streaming_request(head, messages);
        let response = self.grpc.client_streaming(request, path, codec).await?;

        Ok(response.into_inner())
    }

    /// Make a bidirectional-streaming `gRPC` call.
    ///
    /// # Errors
    /// - `gRPC` client is not ready
    /// - Server call failed
    pub async fn bidirectional_streaming<S>(
        &mut self,
        head: &RequestMessage,
        messages: S,
    ) -> Result<StreamingResponse>
    where
        S: tokio_stream::Stream<Item = RequestMessage> + Send + 'static,
    {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;

        let path = head.path();
        let codec = head.codec();

        let request = into_streaming_request(head, messages);
        let response = self.grpc.streaming(request, path, codec).await?;

        Ok(StreamingResponse::new(response.into_inner()))
    }
}

/// Builds a streaming `tonic` request from a message stream, applying the
/// metadata of the `head` message to the outgoing request.
fn into_streaming_request<S>(head: &RequestMessage, messages: S) -> tonic::Request<S>
where
    S: tokio_stream::Stream<Item = RequestMessage>,
{
    let metadata = head.metadata().clone();
    let mut req = tonic::Request::new(messages);
    if let Some(metadata) = metadata {
        *req.metadata_mut() = metadata.inner;
    }
    req
}

/// Creates a new `gRPC` client and sends a message to a `gRPC` server.
/// This method is async.
///
/// # Errors
/// - Internal error calling the `gRPC` server
pub async fn call_unary_async(
    request: &RequestMessage,
    tls: Option<TlsConfig>,
) -> Result<ResponseMessage> {
    let uri = request.uri()?;

    let mut client = GrpcClient::new(uri, tls)?;

    client.unary(request).await
}

/// Creates a new `gRPC` client and sends a message to a `gRPC` server.
/// This method is async.
///
/// # Errors
/// - Internal error calling the `gRPC` server
pub async fn call_server_streaming(
    req: &RequestMessage,
    tls: Option<TlsConfig>,
) -> Result<StreamingResponse> {
    let uri = req.uri()?;

    let mut client = GrpcClient::new(uri, tls)?;

    client.server_streaming(req).await
}

/// Creates a new `gRPC` client and streams messages to a `gRPC` server,
/// returning a single response (client-streaming RPC).
///
/// # Errors
/// - Internal error calling the `gRPC` server
pub async fn call_client_streaming<S>(
    head: &RequestMessage,
    messages: S,
    tls: Option<TlsConfig>,
) -> Result<ResponseMessage>
where
    S: tokio_stream::Stream<Item = RequestMessage> + Send + 'static,
{
    let uri = head.uri()?;

    let mut client = GrpcClient::new(uri, tls)?;

    client.client_streaming(head, messages).await
}

/// Creates a new `gRPC` client and streams messages to a `gRPC` server,
/// returning a stream of responses (bidirectional-streaming RPC).
///
/// # Errors
/// - Internal error calling the `gRPC` server
pub async fn call_bidirectional_streaming<S>(
    head: &RequestMessage,
    messages: S,
    tls: Option<TlsConfig>,
) -> Result<StreamingResponse>
where
    S: tokio_stream::Stream<Item = RequestMessage> + Send + 'static,
{
    let uri = head.uri()?;

    let mut client = GrpcClient::new(uri, tls)?;

    client.bidirectional_streaming(head, messages).await
}
