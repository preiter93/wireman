use super::{metadata::Metadata, DynamicMessage};
use crate::{error::Error, Result};
use http::uri::PathAndQuery;
use prost_reflect::{MessageDescriptor, MethodDescriptor};
use std::str::FromStr;
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataValue},
    Request,
};

/// Holds all the necessary data for a grpc request
#[derive(Debug, Clone)]
pub struct RequestMessage {
    pub message: DynamicMessage,
    method_desc: MethodDescriptor,
    metadata: Option<Metadata>,
}

impl RequestMessage {
    /// Instantiate a `RequestMessage` from the Descriptors.
    #[must_use]
    pub fn new(message_desc: MessageDescriptor, method_desc: MethodDescriptor) -> Self {
        let message = DynamicMessage::new(message_desc);
        Self {
            method_desc,
            message,
            metadata: None,
        }
    }

    /// Returns the Message name.
    #[must_use]
    pub fn message_name(&self) -> String {
        self.message_descriptor().name().to_string()
    }

    /// Returns the message descriptor.
    #[must_use]
    pub fn message_descriptor(&self) -> MessageDescriptor {
        self.message.descriptor()
    }

    /// Returns the method descriptor.
    #[must_use]
    pub fn method_descriptor(&self) -> MethodDescriptor {
        self.method_desc.clone()
    }

    /// Set a new message.
    pub fn set_message(&mut self, message: DynamicMessage) {
        self.message = message;
    }

    /// Insert metadata
    ///
    /// # Errors
    /// - Failed to parse metadata value/key to ascii
    pub fn insert_metadata(&mut self, key: &str, val: &str) -> Result<()> {
        let key: MetadataKey<Ascii> = key.parse().map_err(|_| Error::ParseToAsciiError)?;
        let val: MetadataValue<Ascii> = val.parse().map_err(|_| Error::ParseToAsciiError)?;
        let map = self.metadata.get_or_insert(Metadata::new());
        map.insert(key, val);
        Ok(())
    }

    /// Get the metadata
    #[must_use]
    pub fn get_metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }

    // /// Deserialize a `ProtoMessage` from a json string
    // ///
    // /// # Errors
    // /// - Failed to deserialize message
    // pub fn from_json(&mut self, json: &str) -> Result<()> {
    //     let mut de = serde_json::Deserializer::from_str(json);
    //     let msg = DynamicMessage::deserialize_with_options(
    //         self.get_message_descriptor(),
    //         &mut de,
    //         &DeserializeOptions::new(),
    //     )
    //     .map_err(Error::DeserializeMessage)?;
    //     de.end().map_err(Error::DeserializeMessage)?;
    //     *self.message = msg;
    //     Ok(())
    // }
    //
    // /// Serialize a `ProtoMessage` to a json string
    // ///
    // /// # Errors
    // /// - Failed to convert utf8 to String
    // /// - Failed to serialize message
    // pub fn to_json(&self) -> Result<String> {
    //     let mut s = serde_json::Serializer::new(Vec::new());
    //     self.message
    //         .serialize_with_options(
    //             &mut s,
    //             &SerializeOptions::new()
    //                 .stringify_64_bit_integers(false)
    //                 .skip_default_fields(false),
    //         )
    //         .map_err(Error::SerializeProtoMessage)?;
    //
    //     String::from_utf8(s.into_inner())
    //         .map_err(|_| Error::InternalError("FromUTF8Error".to_string()))
    // }

    /// Returns the uri path for grpc calls
    ///
    /// # Panics
    /// - Unwrapping path and query from str
    #[must_use]
    pub fn get_path(&self) -> PathAndQuery {
        let path = format!(
            "/{}/{}",
            self.method_desc.parent_service().full_name(),
            self.method_desc.name()
        );
        PathAndQuery::from_str(&path).unwrap()
    }

    /// Wrap the message in a `tonic::Request`
    #[must_use]
    pub fn into_request(self) -> Request<RequestMessage> {
        let metadata = self.get_metadata().clone();
        let mut req = Request::new(self);
        if let Some(meta) = metadata {
            *req.metadata_mut() = meta.inner;
        }
        req
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
        let desc = ProtoDescriptor::from_files(files, includes).unwrap();

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
}
