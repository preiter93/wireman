use core::{
    descriptor::message::ProtoMessage, MethodDescriptor, ProtoDescriptor, ServiceDescriptor,
};

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
    pub fn get_request(&self, method: &MethodDescriptor) -> ProtoMessage {
        self.desc.get_request(method)
    }

    /// Makes a unary grpc call with a given Message and Method which is
    /// defined in ProtoMessage
    pub fn call_unary(&self, req: &ProtoMessage) -> Result<String, String> {
        core::call_unary(req).map_err(|err| err.to_string())
    }
}
