use core::{
    descriptor::message::MethodMessage, MethodDescriptor, ProtoDescriptor, ServiceDescriptor,
};

use super::request::ErrorKind;

/// The [CoreClient] calls the proto descriptor and gRPC client of the
/// core package.  
#[derive(Debug, Clone)]
pub struct CoreClient {
    /// The proto descriptor retrievs info of the services, methods and messages
    desc: ProtoDescriptor,
}

impl CoreClient {
    pub fn new(desc: ProtoDescriptor) -> Self {
        Self { desc }
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
    pub fn get_request(&self, method: &MethodDescriptor) -> MethodMessage {
        self.desc.get_request(method)
    }

    /// Makes a unary grpc call with a given Message and Method which is
    /// defined in ProtoMessage
    pub fn call_unary(
        &self,
        method: &MethodDescriptor,
        message: &str,
    ) -> Result<String, ErrorKind> {
        // Parse message to json
        let mut req = self.get_request(method);
        req.from_json(message)?;

        // Call gRPC client
        let resp = core::call_unary(&req)?;
        Ok(resp)
    }
}
