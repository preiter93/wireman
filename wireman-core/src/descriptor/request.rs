use super::{metadata::Metadata, DynamicMessage};
use crate::client::codec::DynamicCodec;
use crate::{
    error::{Error, FROM_UTF8},
    Result,
};
use http::{uri::PathAndQuery, Uri};
use prost_reflect::{MessageDescriptor, MethodDescriptor};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::str::FromStr;
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataValue},
    Request,
};

/// Holds all the necessary data for a `gRPC` request, including
/// the message, method descriptor, and optional metadata.
#[derive(Debug, Clone)]
pub struct RequestMessage {
    /// The `gRPC` message.
    message: DynamicMessage,
    /// The `gRPC` method
    method_desc: MethodDescriptor,
    /// The requests metadata.
    metadata: Option<Metadata>,
    /// The host address.
    address: String,
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
            address: String::new(),
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

    /// Gets a reference to the message.
    #[must_use]
    pub fn message(&self) -> &DynamicMessage {
        &self.message
    }

    /// Gets a mutable reference to the message.
    #[must_use]
    pub fn message_mut(&mut self) -> &mut DynamicMessage {
        &mut self.message
    }

    /// Set a new message for the request.
    pub fn set_message(&mut self, message: DynamicMessage) {
        self.message = message;
    }

    /// Get the host address.
    #[must_use]
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get the host address as uri.
    ///
    /// # Errors
    /// - Failed to parse address to uri.
    pub fn uri(&self) -> Result<Uri> {
        Uri::try_from(self.address())
            .map_err(|_| Error::Internal(String::from("Failed to parse address")))
    }

    /// Sets the host address.
    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

    /// Get the metadata associated with the request.
    #[must_use]
    pub fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
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

    /// Get the URI path for `gRPC` calls based on the method descriptor.
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

    /// Return the dynamic codec based on the method descriptor.
    #[must_use]
    pub fn codec(&self) -> DynamicCodec {
        DynamicCodec::new(self.method_descriptor())
    }

    /// Serialize the `RequestMessage` to a JSON string.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if serialization to a JSON string fails.
    pub fn to_json(&self) -> Result<String> {
        let mut s = serde_json::Serializer::new(Vec::new());
        self.serialize(&mut s)
            .map_err(|_| Error::Internal(String::from("failed to serialize message")))?;
        String::from_utf8(s.into_inner()).map_err(|_| Error::Internal(FROM_UTF8.to_string()))
    }
}

impl From<RequestMessage> for Request<RequestMessage> {
    fn from(value: RequestMessage) -> Self {
        let metadata = value.metadata().clone();
        let mut req = Request::new(value);
        if let Some(meta) = metadata {
            *req.metadata_mut() = meta.inner;
        }
        req
    }
}

impl Serialize for RequestMessage {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RequestMessage", 3)?;
        state.serialize_field("message", &self.message)?;
        if let Some(metadata) = &self.metadata {
            state.serialize_field("metadata", &metadata)?;
        }
        state.serialize_field("address", &self.address)?;
        state.end()
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
    fn test_into_request() {
        // given
        let mut given_message = load_test_message("Simple");
        given_message
            .insert_metadata("metadata-key", "metadata-value")
            .unwrap();
        let method_descriptor = given_message.method_descriptor().clone();
        let message_descriptor = given_message.message_descriptor().clone();

        // when
        let given_req: Request<RequestMessage> = given_message.into();

        // then
        let metadata = given_req.metadata();
        assert!(metadata.contains_key("metadata-key"));
        assert_eq!(metadata.get("metadata-key").unwrap(), "metadata-value");
        assert_eq!(given_req.get_ref().method_descriptor(), method_descriptor);
        assert_eq!(given_req.get_ref().message_descriptor(), message_descriptor);
    }

    #[test]
    fn test_to_json() {
        // given
        let mut given_message = load_test_message("Simple");
        given_message.insert_metadata("key", "value").unwrap();
        given_message.set_address("localhost:50051");

        // when
        let json = given_message.to_json().unwrap();

        // then
        let expected_json = "{\"message\":{\"number\":0},\"metadata\":{\"key\":\"value\"},\"address\":\"localhost:50051\"}";
        assert_eq!(json, expected_json);
    }
}
