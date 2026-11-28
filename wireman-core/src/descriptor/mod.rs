#![allow(clippy::module_name_repetitions)]
pub mod message;
pub mod metadata;
pub mod reflection_request;
pub mod request;
pub mod response;
pub mod schema;

pub use message::DynamicMessage;
pub use reflection_request::ReflectionRequest;
pub use request::RequestMessage;
pub use response::ResponseMessage;

use crate::{client::reflection::build_file_descriptor_set, error::Error, Result};
use prost_reflect::{DescriptorPool, MessageDescriptor, MethodDescriptor, ServiceDescriptor};
use std::path::Path;

#[derive(Default, Debug, Clone)]
pub struct ProtoDescriptor {
    pool: DescriptorPool,
}

impl ProtoDescriptor {
    /// Instantiate `DescriptorPool` from proto files and include paths
    ///
    /// # Errors
    /// - Failed to compile proto `ProtoxCompileError`
    pub fn new(
        includes: impl IntoIterator<Item = impl AsRef<Path>>,
        files: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Result<Self> {
        // protox::compile drops custom options like google.api.http.
        let pool = protox::Compiler::new(includes)
            .map_err(Error::ProtoxCompileError)?
            .open_files(files)
            .map_err(Error::ProtoxCompileError)?
            .descriptor_pool();
        Ok(Self { pool })
    }

    /// Instantiates a `DescriptorPool` from a grpc server that supports
    /// reflection.
    ///
    /// # Errors
    /// Errors if server reflection or dependency resolving fails.
    pub async fn from_reflection(request: ReflectionRequest) -> Result<Self> {
        let file_descriptor_set = build_file_descriptor_set(request).await?;

        let pool = DescriptorPool::from_file_descriptor_set(file_descriptor_set)
            .map_err(|e| Error::Internal(format!("err {e}")))?;

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

    /// Returns a Method by its full name like `proto.Greeter.SayHello` or
    /// `proto.Greeter/SayHello`.
    #[must_use]
    pub fn get_method_by_full_name(&self, full_name: &str) -> Option<MethodDescriptor> {
        let (service_name, method_name) = full_name.rsplit_once(['/', '.'])?;
        self.get_method_by_name(service_name, method_name)
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

    /// Returns the request Message of a given Method, filled from a JSON string.
    ///
    /// # Errors
    ///
    /// Errors if the JSON does not match the request message.
    pub fn request_from_json(
        &self,
        method: &MethodDescriptor,
        json: &str,
    ) -> Result<RequestMessage> {
        let mut request = self.get_request(method);
        request.message_mut().from_json(json)?;
        Ok(request)
    }

    // Returns the response Message of a given Method
    #[must_use]
    pub fn get_response(&self, method: &MethodDescriptor) -> ResponseMessage {
        ResponseMessage::new(self.get_response_descriptor(method), method.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_from_json() {
        struct TestCase {
            name: &'static str,
            given_json: &'static str,
            expected_error: bool,
            expected_json: &'static str,
        }

        let test_cases = vec![
            TestCase {
                name: "success: valid json",
                given_json: "{\"number\":5}",
                expected_error: false,
                expected_json: "{\"number\":5}",
            },
            TestCase {
                name: "failure: unknown field",
                given_json: "{\"unknown\":5}",
                expected_error: true,
                expected_json: "",
            },
        ];

        let descriptor = ProtoDescriptor::new(vec!["."], vec!["test_files/test.proto"]).unwrap();
        let method = descriptor
            .get_method_by_name("proto.TestService", "Simple")
            .unwrap();

        for test_case in test_cases {
            let request = descriptor.request_from_json(&method, test_case.given_json);

            assert_eq!(
                request.is_err(),
                test_case.expected_error,
                "{}",
                test_case.name
            );
            let json = request
                .map(|request| request.message().to_json().unwrap())
                .unwrap_or_default();
            assert_eq!(json, test_case.expected_json, "{}", test_case.name);
        }
    }

    #[test]
    fn test_get_method_by_full_name() {
        struct TestCase {
            name: &'static str,
            given_full_name: &'static str,
            expected_method: Option<&'static str>,
        }

        let test_cases = vec![
            TestCase {
                name: "success: dot form",
                given_full_name: "proto.TestService.Simple",
                expected_method: Some("proto.TestService.Simple"),
            },
            TestCase {
                name: "success: slash form",
                given_full_name: "proto.TestService/Simple",
                expected_method: Some("proto.TestService.Simple"),
            },
            TestCase {
                name: "failure: no service part",
                given_full_name: "Simple",
                expected_method: None,
            },
            TestCase {
                name: "failure: unknown method",
                given_full_name: "proto.TestService.Unknown",
                expected_method: None,
            },
        ];

        let descriptor = ProtoDescriptor::new(vec!["."], vec!["test_files/test.proto"]).unwrap();

        for test_case in test_cases {
            let method = descriptor.get_method_by_full_name(test_case.given_full_name);

            let full_name = method.as_ref().map(MethodDescriptor::full_name);
            assert_eq!(full_name, test_case.expected_method, "{}", test_case.name);
        }
    }

    #[test]
    fn test_custom_options_are_kept() {
        struct TestCase {
            name: &'static str,
            given_service: &'static str,
            given_method: &'static str,
            expected_json: &'static str,
        }

        let test_cases = vec![TestCase {
            name: "success: method option is kept",
            given_service: "options.OrderService",
            given_method: "GetOrder",
            expected_json: r#"{"[options.http]":{"get":"/v1/orders/{id}"}}"#,
        }];

        let descriptor = ProtoDescriptor::new(vec!["."], vec!["test_files/options.proto"]).unwrap();

        for test_case in test_cases {
            let method = descriptor
                .get_method_by_name(test_case.given_service, test_case.given_method)
                .unwrap();

            let json = serde_json::to_string(&method.options()).unwrap();
            assert_eq!(json, test_case.expected_json, "{}", test_case.name);
        }
    }
}
