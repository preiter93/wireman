//! Module for all grpc related stuff
use crate::descriptor::message::MethodMessage;
use crate::error::Error;
use crate::ProtoTuiConfig;
use crate::Result;
use tls::TlsConfig;
use tokio::runtime::Runtime;
use tonic::transport::Uri;
use tonic::{client::Grpc, transport::Channel};

mod codec;
pub mod tls;

/// Creates a new gRPC client and sends a message to a gRPC server.
/// This method is blocking.
pub fn call_unary_blocking<T: Into<Uri>>(
    cfg: &ProtoTuiConfig,
    uri: T,
    req: &MethodMessage,
) -> Result<MethodMessage> {
    let rt = Runtime::new().unwrap();
    let future = async_call_unary(cfg, uri, req);
    let result = rt.block_on(future);
    match result {
        Ok(response) => Ok(response),
        Err(err) => Err(Error::InternalError(err.to_string())),
    }
}

async fn async_call_unary<T: Into<Uri>>(
    cfg: &ProtoTuiConfig,
    uri: T,
    req: &MethodMessage,
) -> Result<MethodMessage> {
    let mut client = GrpcClient::from_config(cfg, uri)?;
    let resp = client.unary(req).await?;
    Ok(resp)
}

#[derive(Clone, Debug)]
pub struct GrpcClient {
    grpc: Grpc<Channel>,
}

impl GrpcClient {
    /// Returns a new Grpc Client. if no tls is given, the standard tonic
    /// client is used.
    pub fn new<T: Into<Uri>>(uri: T, tls: Option<TlsConfig>) -> Result<Self> {
        let builder = Channel::builder(uri.into());
        let channel = if let Some(tls) = tls {
            // Build a channel with custom tls settings
            let connector = tls.get_connector_from_tls();
            builder.connect_with_connector_lazy(connector)
        } else {
            // The standard tonic channel
            builder.connect_lazy()
        };

        Ok(GrpcClient {
            grpc: Grpc::new(channel),
        })
    }

    /// Instantiates a client from a ProtoConfig
    pub fn from_config<T: Into<Uri>>(cfg: &ProtoTuiConfig, uri: T) -> Result<Self> {
        Self::new(uri, Some(cfg.tls.clone()))
    }

    /// Make a unary gRPC call from the client
    ///
    /// # Error
    /// - grpc client is not ready
    /// - server call failed
    pub async fn unary(&mut self, req: &MethodMessage) -> Result<MethodMessage> {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;
        let codec = codec::DynamicCodec::new(req.get_method_descriptor());
        // path
        let path = req.get_path();
        // make call
        let req = req.clone().into_request();
        let response = self.grpc.unary(req, path, codec).await?.into_inner();
        Ok(response)
    }

    /// Make a unary gRPC call from the client. The call is
    /// wrapped in a tokio runtime to run asynchronous asks.
    ///
    /// # Error
    /// - grpc client is not ready
    /// - server call failed
    pub fn unary_with_runtime(&mut self, req: &MethodMessage) -> Result<String> {
        let rt = Runtime::new().unwrap();
        let future = self.unary(req);
        let result = rt.block_on(future);
        match result {
            Ok(response) => Ok(response.to_json()),
            Err(err) => Err(Error::InternalError(err.to_string())),
        }
    }
}
