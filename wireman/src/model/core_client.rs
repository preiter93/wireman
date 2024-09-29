use crate::widgets::editor::ErrorKind;
use config::Config;
use core::{
    client::tls::TlsConfig,
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
    /// Proto path includes
    includes: Vec<String>,
    /// Proto files
    files: Vec<String>,
}

impl Default for CoreClient {
    fn default() -> Self {
        Self::new(&Config::default()).unwrap()
    }
}

#[derive(Debug, Clone)]
struct GrpcClientConfig(Config);

impl CoreClient {
    pub fn new(cfg: &Config) -> Result<Self, Box<dyn Error>> {
        let includes = cfg.includes();
        let files = cfg.files();
        let desc = ProtoDescriptor::new(includes.clone(), files.clone())?;
        let grpc = GrpcClientConfig(cfg.clone());
        Ok(Self {
            desc,
            grpc,
            includes,
            files,
        })
    }

    pub fn update_proto_descriptor(&mut self, desc: ProtoDescriptor) {
        self.desc = desc;
    }

    pub fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        self.desc = ProtoDescriptor::new(self.includes.clone(), self.files.clone())?;
        Ok(())
    }

    /// Return the proto Services
    pub fn get_services(&self) -> Vec<ServiceDescriptor> {
        self.desc.get_services()
    }

    /// Return a proto services by its name
    pub fn get_service_by_name(&self, name: &str) -> Option<ServiceDescriptor> {
        self.desc.get_service_by_name(name)
    }

    /// Returns the proto methods of a given service
    pub fn get_methods(&self, service: &ServiceDescriptor) -> Vec<MethodDescriptor> {
        self.desc.get_methods(service)
    }

    /// Returns a proto method by its name
    pub fn get_method_by_name(
        &self,
        service_name: &str,
        method_name: &str,
    ) -> Option<MethodDescriptor> {
        self.desc.get_method_by_name(service_name, method_name)
    }

    /// Returns the proto request of a given method
    pub fn get_request(&self, method: &MethodDescriptor) -> RequestMessage {
        let mut req = self.desc.get_request(method);
        req.message_mut().apply_template();
        req
    }

    /// Returns the default address as defined in the wireman.toml
    pub fn get_default_address(&self) -> String {
        self.grpc.0.server.default_address.clone()
    }

    /// Returns the tls config.
    pub fn get_tls_config(&self) -> Option<TlsConfig> {
        let tls_config = self.grpc.0.tls.clone();
        match (tls_config.use_native, tls_config.custom_cert) {
            (Some(use_native), _) => {
                if use_native {
                    return Some(TlsConfig::native());
                }
                None
            }
            (None, Some(custom)) => Some(TlsConfig::custom(custom).unwrap()),
            _ => Some(TlsConfig::native()),
        }
    }

    /// Makes a unary grpc call with a given Message and Method which is
    /// defined in [`ProtoMessage`]
    pub fn call_unary(
        req: &RequestMessage,
        tls: Option<TlsConfig>,
    ) -> Result<ResponseMessage, ErrorKind> {
        Ok(core::client::call_unary_blocking(req, tls)?)
    }

    /// Makes a unary grpc call with a given Message and Method which is
    /// defined in [`ProtoMessage`]
    pub async fn call_unary_async(
        req: &RequestMessage,
        tls: Option<TlsConfig>,
    ) -> Result<ResponseMessage, ErrorKind> {
        Ok(core::client::call_unary_async(req, tls).await?)
    }

    /// Return a grpcurl request
    pub fn get_grpcurl(
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
