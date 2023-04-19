//! Module for all grpc related stuff
use crate::descriptor::message::MethodMessage;
use crate::error::Error;
use crate::Result;
use tokio::runtime::Runtime;
use tonic::transport::Uri;
use tonic::IntoRequest;
use tonic::{client::Grpc, transport::Channel};

mod codec;

#[derive(Clone, Debug)]
pub struct Client {
    grpc: Grpc<Channel>,
}

impl Client {
    pub fn new<T: Into<Uri>>(uri: T) -> Result<Self> {
        let uri = uri.into();
        let channel = Channel::builder(uri).connect_lazy();
        Ok(Client {
            grpc: Grpc::new(channel),
        })
    }

    pub async fn unary(&mut self, req: &MethodMessage) -> Result<MethodMessage> {
        self.grpc.ready().await.map_err(Error::GrpcNotReady)?;
        let codec = codec::DynamicCodec::new(req.get_method_descriptor());
        let path = req.get_path();
        let resp = self
            .grpc
            .unary(req.clone().into_request(), path, codec)
            .await?
            .into_inner();
        Ok(resp)
    }
}

pub fn call_unary(req: &MethodMessage) -> Result<String> {
    let rt = Runtime::new().unwrap();
    let future = async_call(req);
    let result = rt.block_on(future);
    match result {
        Ok(response) => Ok(response.to_json()),
        Err(err) => Err(Error::InternalError(err.to_string())),
    }
    // Ok(resp)
}

async fn async_call(req: &MethodMessage) -> Result<MethodMessage> {
    let mut client = Client::new(Uri::from_static("http://localhost:50051"))?;
    let resp = client.unary(req).await?;
    Ok(resp)
}
