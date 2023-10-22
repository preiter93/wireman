use crate::commons::editor::ErrorKind;
use config::Config;
use core::{
    descriptor::{RequestMessage, ResponseMessage},
    features::grpcurl,
    MethodDescriptor, ProtoDescriptor, ServiceDescriptor,
};
use http::Uri;
use std::{collections::HashMap, error::Error};

/// The [`CoreClient`] calls the proto descriptor and grpc client of the
/// core package.  
#[derive(Debug, Clone)]
pub struct CoreClient {
    /// The proto descriptor
    desc: ProtoDescriptor,
    /// Config to create a new grpc client
    grpc: GrpcClientConfig,
}

#[derive(Debug, Clone)]
struct GrpcClientConfig(Config);

impl CoreClient {
    pub fn new(cfg: Config) -> Result<Self, Box<dyn Error>> {
        let desc = ProtoDescriptor::new(cfg.includes.clone(), cfg.files.clone())?;
        let grpc = GrpcClientConfig(cfg);
        Ok(Self { desc, grpc })
    }

    /// Return the proto Services
    pub fn get_services(&self) -> Vec<ServiceDescriptor> {
        self.desc.get_services()
    }

    /// Returns the proto methods of a given service
    pub fn get_methods(&self, service: &ServiceDescriptor) -> Vec<MethodDescriptor> {
        self.desc.get_methods(service)
    }

    /// Returns the proto request of a given method
    pub fn get_request(&self, method: &MethodDescriptor) -> RequestMessage {
        let mut req = self.desc.get_request(method);
        req.message.apply_template();
        req
    }

    /// Returns the default address as defined in the config.json
    pub fn get_default_address(&self) -> String {
        self.grpc.0.address.clone()
    }

    /// Makes a unary grpc call with a given Message and Method which is
    /// defined in [`ProtoMessage`]
    pub fn call_unary(
        &self,
        req: &RequestMessage,
        address: &str,
    ) -> Result<ResponseMessage, ErrorKind> {
        let uri = Uri::try_from(address).map_err(|_| ErrorKind {
            kind: "ParseAddressError".to_string(),
            msg: String::new(),
        })?;
        let resp = core::client::call_unary_blocking(uri, req)?;
        Ok(resp)
    }

    /// Return a grpcurl request
    pub fn grpcurl(
        &self,
        message: &str,
        method_desc: &MethodDescriptor,
        metadata: &HashMap<String, String>,
        address: &str,
    ) -> Result<String, String> {
        Ok(grpcurl(
            &self.grpc.0.includes,
            Uri::try_from(address).map_err(|_| "Failed to parse address")?,
            message,
            method_desc,
            metadata,
        ))
    }
}
