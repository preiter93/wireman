use crate::error::Error;
use std::path::Path;
pub mod message;
use self::message::MethodMessage;
use crate::ProtoConfig;
use crate::Result;
use prost_reflect::{DescriptorPool, MessageDescriptor, MethodDescriptor, ServiceDescriptor};

#[derive(Default, Debug, Clone)]
pub struct ProtoDescriptor {
    pool: DescriptorPool,
}

impl ProtoDescriptor {
    /// Instantiates a descriptor from a ProtoConfig
    pub fn from_config(cfg: ProtoConfig) -> Result<Self> {
        let files = cfg.files.clone();
        let includes = vec![cfg.workspace];
        Self::from_files(files, includes)
    }

    /// Instantiate Descriptor from proto files and include paths
    pub fn from_files(
        files: impl IntoIterator<Item = impl AsRef<Path>>,
        includes: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self> {
        // Compile proto files to file descriptors
        let file_desc_set = protox::compile(files, includes).map_err(Error::ProtoxCompileError)?;
        // Generate descriptor pool from file descriptor
        let pool = DescriptorPool::from_file_descriptor_set(file_desc_set).unwrap();
        Ok(Self { pool })
    }

    /// Returns a Service by its name
    pub fn get_service_by_name(&self, name: &str) -> Option<ServiceDescriptor> {
        self.pool.get_service_by_name(name)
    }

    /// Returns a Message by its name
    pub fn get_message_by_name(&self, name: &str) -> Option<MessageDescriptor> {
        self.pool.get_message_by_name(name)
    }

    /// Returns a Method of a service by its name
    pub fn get_method_by_name(
        &self,
        service_name: &str,
        method_name: &str,
    ) -> Option<MethodDescriptor> {
        self.get_service_by_name(service_name)
            .unwrap()
            .methods()
            .find(|m| m.name() == method_name)
    }

    /// Returns all Services from the descriptor pool
    pub fn get_services(&self) -> Vec<ServiceDescriptor> {
        self.pool.services().collect()
    }
    // Returns all Methods of a given Service
    pub fn get_methods(&self, service: &ServiceDescriptor) -> Vec<MethodDescriptor> {
        service.methods().collect()
    }

    // Returns the request MessageDescriptor of a given Method
    pub fn get_request_descriptor(&self, method: &MethodDescriptor) -> MessageDescriptor {
        method.input()
    }

    // Returns the response MessageDescriptor of a given Method
    pub fn get_response_descriptor(&self, method: &MethodDescriptor) -> MessageDescriptor {
        method.output()
    }

    // Returns the request Message of a given Method
    pub fn get_request(&self, method: &MethodDescriptor) -> MethodMessage {
        MethodMessage::from_descriptor(self.get_request_descriptor(method), method.clone())
    }

    // Returns the response Message of a given Method
    pub fn get_response(&self, method: &MethodDescriptor) -> MethodMessage {
        MethodMessage::from_descriptor(self.get_response_descriptor(method), method.clone())
    }
}
