#![allow(clippy::module_name_repetitions)]
pub mod message;
pub mod metadata;
pub mod request;
pub mod response;

pub use message::DynamicMessage;
pub use request::RequestMessage;
pub use response::ResponseMessage;

use crate::error::Error;
use crate::Result;
use prost_reflect::{DescriptorPool, MessageDescriptor, MethodDescriptor, ServiceDescriptor};
use std::path::Path;

#[derive(Default, Debug, Clone)]
pub struct ProtoDescriptor {
    pool: DescriptorPool,
}

impl ProtoDescriptor {
    // /// Instantiates a descriptor from a [`ProtoConfig`]
    // ///
    // /// # Errors
    // /// - Failed to compile proto `ProtoxCompileError`
    // /// - Failed to generate descriptor `DescriptorError`
    // pub fn from_config(cfg: &Config) -> Result<Self> {
    //     let files = cfg.files.clone();
    //     let includes = vec![cfg.workspace.clone()];
    //     Self::from_files(files, includes)
    // }

    /// Instantiate `DescriptorPool` from proto files and include paths
    ///
    /// # Errors
    /// - Failed to compile proto `ProtoxCompileError`
    /// - Failed to generate descriptor `DescriptorError`
    pub fn new(
        includes: impl IntoIterator<Item = impl AsRef<Path>>,
        files: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self> {
        // Compile proto files to file descriptors
        let file_desc_set = protox::compile(files, includes).map_err(Error::ProtoxCompileError)?;
        // Generate descriptor pool from file descriptor
        let pool = DescriptorPool::from_file_descriptor_set(file_desc_set)
            .map_err(Error::DescriptorError)?;
        Ok(Self { pool })
    }

    /// Returns a Service by its name
    #[must_use]
    pub fn get_service_by_name(&self, name: &str) -> Option<ServiceDescriptor> {
        self.pool.get_service_by_name(name)
    }

    /// Returns a Message by its name
    #[must_use]
    pub fn get_message_by_name(&self, name: &str) -> Option<MessageDescriptor> {
        self.pool.get_message_by_name(name)
    }

    /// Returns a Method of a service by its name
    #[must_use]
    pub fn get_method_by_name(
        &self,
        service_name: &str,
        method_name: &str,
    ) -> Option<MethodDescriptor> {
        self.get_service_by_name(service_name)?
            .methods()
            .find(|m| m.name() == method_name)
    }

    /// Returns all Services from the descriptor pool
    #[must_use]
    pub fn get_services(&self) -> Vec<ServiceDescriptor> {
        let mut services: Vec<ServiceDescriptor> = self.pool.services().collect();
        services.sort_by(|a, b| a.full_name().cmp(b.full_name()));
        services
    }
    // Returns all Methods of a given Service
    #[must_use]
    pub fn get_methods(&self, service: &ServiceDescriptor) -> Vec<MethodDescriptor> {
        let mut methods: Vec<MethodDescriptor> = service.methods().collect();
        methods.sort_by(|a, b| a.full_name().cmp(b.full_name()));
        methods
    }

    // Returns the request MessageDescriptor of a given Method
    #[must_use]
    pub fn get_request_descriptor(&self, method: &MethodDescriptor) -> MessageDescriptor {
        method.input()
    }

    // Returns the response MessageDescriptor of a given Method
    #[must_use]
    pub fn get_response_descriptor(&self, method: &MethodDescriptor) -> MessageDescriptor {
        method.output()
    }

    // Returns the request Message of a given Method
    #[must_use]
    pub fn get_request(&self, method: &MethodDescriptor) -> RequestMessage {
        RequestMessage::new(self.get_request_descriptor(method), method.clone())
    }

    // Returns the response Message of a given Method
    #[must_use]
    pub fn get_response(&self, method: &MethodDescriptor) -> ResponseMessage {
        ResponseMessage::new(self.get_response_descriptor(method), method.clone())
    }
}
