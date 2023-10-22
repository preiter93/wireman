use super::{metadata::Metadata, DynamicMessage};
use crate::{error::Error, Result};
use http::uri::PathAndQuery;
use prost_reflect::{MessageDescriptor, MethodDescriptor, SerializeOptions};
use std::str::FromStr;
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataValue},
    Request,
};

/// Holds all the necessary data for a gRPC request, including
/// the message, method descriptor, and optional metadata.
#[derive(Debug, Clone)]
pub struct RequestMessage {
    /// The gRPC message.
    pub message: DynamicMessage,
    method_desc: MethodDescriptor,
    metadata: Option<Metadata>,
}

impl RequestMessage {
    /// Create a new `RequestMessage` with the provided message
    /// descriptor and method descriptor.
    #[must_use]
    pub fn new(message_desc: MessageDescriptor, method_desc: MethodDescriptor) -> Self {
        let message = DynamicMessage::new(message_desc);
        Self {
            message,
            method_desc,
            metadata: None,
        }
    }

    /// Get the name of the message.
    #[must_use]
    pub fn message_name(&self) -> String {
        self.message_descriptor().name().to_string()
    }

    /// Get the message descriptor associated with the `RequestMessage`.
    #[must_use]
    pub fn message_descriptor(&self) -> MessageDescriptor {
        self.message.descriptor()
    }

    /// Get the method descriptor associated with the `RequestMessage`.
    #[must_use]
    pub fn method_descriptor(&self) -> MethodDescriptor {
        self.method_desc.clone()
    }

    /// Set a new message for the request.
    pub fn set_message(&mut self, message: DynamicMessage) {
        self.message = message;
    }

    /// Insert metadata into the request.
    ///
    /// # Errors
    ///
    /// - Failed to parse metadata value/key to ascii
    pub fn insert_metadata(&mut self, key: &str, val: &str) -> Result<()> {
        let key: MetadataKey<Ascii> = key.parse().map_err(|_| Error::ParseToAsciiError)?;
        let val: MetadataValue<Ascii> = val.parse().map_err(|_| Error::ParseToAsciiError)?;
        let map = self.metadata.get_or_insert(Metadata::new());
        map.insert(key, val);
        Ok(())
    }

    /// Get the metadata associated with the request.
    #[must_use]
    pub fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }

    /// Get the URI path for gRPC calls based on the method descriptor.
    ///
    /// # Panics
    ///
    /// Panics if constructing the path and query from a string fails.
    #[must_use]
    pub fn path(&self) -> PathAndQuery {
        let path = format!(
            "/{}/{}",
            self.method_desc.parent_service().full_name(),
            self.method_desc.name()
        );
        PathAndQuery::from_str(&path).unwrap()
    }

    /// Wrap the message in a `tonic::Request`.
    #[must_use]
    pub fn into_request(self) -> Request<RequestMessage> {
        let metadata = self.metadata().clone();
        let mut req = Request::new(self);
        if let Some(meta) = metadata {
            *req.metadata_mut() = meta.inner;
        }
        req
    }

    /// Serialize the `RequestMessage` to a JSON string.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if serialization to a JSON string fails.
    pub fn to_json(&self) -> Result<String> {
        let mut s = serde_json::Serializer::new(Vec::new());

        self.message
            .serialize_with_options(
                &mut s,
                &SerializeOptions::new()
                    .stringify_64_bit_integers(false)
                    .skip_default_fields(false),
            )
            .map_err(Error::SerializeJsonError)?;

        if let Some(metadata) = &self.metadata {
            metadata
                .serialize(&mut s)
                .map_err(Error::SerializeJsonError)?;
        }

        String::from_utf8(s.into_inner())
            .map_err(|_| Error::InternalError("FromUTF8Error".to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::ProtoDescriptor;

    use super::*;

    fn load_test_message(method: &str) -> RequestMessage {
        // The test files
        let files = vec!["test_files/test.proto"];
        let includes = vec!["."];

        // Generate the descriptor
        let desc = ProtoDescriptor::new(includes, files).unwrap();

        // Get the method and message
        let method = desc
            .get_method_by_name("proto.TestService", method)
            .unwrap();
        let request = method.input();
        RequestMessage::new(request, method)
    }

    #[test]
    fn test_into_requeset() {
        // given
        let mut given_message = load_test_message("Simple");
        given_message
            .insert_metadata("metadata-key", "metadata-value")
            .unwrap();

        // when
        let given_req = given_message.clone().into_request();

        // then
        let metadata = given_req.metadata();
        assert!(metadata.contains_key("metadata-key"));
        assert_eq!(
            metadata.get("metadata-key").unwrap().as_bytes(),
            "metadata-value".as_bytes()
        );
        let message = given_req.get_ref().clone();
        assert_eq!(
            message.method_descriptor(),
            given_message.method_descriptor()
        );
        assert_eq!(
            message.message_descriptor(),
            given_message.message_descriptor()
        );
    }

    #[test]
    fn test_to_json() {
        // given
        let mut given_message = load_test_message("Simple");
        given_message.insert_metadata("key", "value").unwrap();

        // when
        let json = given_message.to_json().unwrap();

        // then
        let expected_json = "{\"number\":0}{\"key\":\"value\"}";
        assert_eq!(json, expected_json);
    }
}
