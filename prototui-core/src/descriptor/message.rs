use crate::{error::Error, Result};
use http::uri::PathAndQuery;
use prost_reflect::{
    DeserializeOptions, DynamicMessage, MessageDescriptor, MethodDescriptor, SerializeOptions,
};
use std::str::FromStr;
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue},
    Request,
};

/// Wrapper around `MessageDescriptor` and `DynamicMessage`
#[derive(Debug, Clone)]
pub struct MethodMessage {
    message_desc: MessageDescriptor,
    method_desc: MethodDescriptor,
    message: DynamicMessage,
    metadata: Option<MetadataMap>,
}

impl MethodMessage {
    /// Construct `ProtoMessage` from a `MessageDescriptor`
    pub fn from_descriptor(message_desc: MessageDescriptor, method_desc: MethodDescriptor) -> Self {
        let message = DynamicMessage::new(message_desc.clone());
        Self {
            message_desc,
            method_desc,
            message,
            metadata: None,
        }
    }

    /// Returns the Message name
    pub fn message_name(&self) -> &str {
        self.message_desc.name()
    }

    /// Returns the message descriptor
    pub fn get_message_descriptor(&self) -> MessageDescriptor {
        self.message_desc.clone()
    }

    /// Returns the method descriptor
    pub fn get_method_descriptor(&self) -> MethodDescriptor {
        self.method_desc.clone()
    }

    /// Returns the dynamic message
    pub fn get_message(&self) -> DynamicMessage {
        self.message.clone()
    }

    /// Set a new message
    pub fn set_message(&mut self, message: DynamicMessage) {
        self.message = message;
    }

    /// Insert metadata
    pub fn insert_metadata(&mut self, key: &str, val: &str) {
        let val: MetadataValue<Ascii> = val.parse().unwrap();
        let map = self.metadata.get_or_insert(MetadataMap::new());
        let key: MetadataKey<Ascii> = key.parse().unwrap();
        map.insert(key, val);
    }

    /// Get the metadata
    pub fn get_metadata(&self) -> &Option<MetadataMap> {
        &self.metadata
    }

    /// Deserialize a ProtoMessage from a json string
    pub fn from_json(&mut self, json: &str) -> Result<()> {
        let mut de = serde_json::Deserializer::from_str(json);
        let msg = DynamicMessage::deserialize_with_options(
            self.message_desc.clone(),
            &mut de,
            &DeserializeOptions::new(),
        )
        .map_err(Error::DeserializeMessage)?;
        de.end().map_err(Error::DeserializeMessage)?;
        self.message = msg;
        Ok(())
    }

    /// Serialize a ProtoMessage to a json string
    pub fn to_json(&self) -> String {
        let mut s = serde_json::Serializer::new(Vec::new());
        self.message
            .serialize_with_options(
                &mut s,
                &SerializeOptions::new()
                    .stringify_64_bit_integers(false)
                    .skip_default_fields(false),
            )
            .unwrap();

        String::from_utf8(s.into_inner()).unwrap()
    }

    /// Returns the uri path for grpc calls
    pub fn get_path(&self) -> PathAndQuery {
        let path = format!(
            "/{}/{}",
            self.method_desc.parent_service().full_name(),
            self.method_desc.name()
        );
        PathAndQuery::from_str(&path).unwrap()
    }

    /// Wrap the message in a `tonic::Request`
    pub fn into_request(self) -> Request<MethodMessage> {
        let mut req = Request::new(self.clone());
        // add metadata
        if let Some(meta) = self.get_metadata() {
            *req.metadata_mut() = meta.clone();
        }
        req
    }
}

#[cfg(test)]
mod test {
    use crate::ProtoDescriptor;

    use super::*;

    fn load_test_request() -> MethodMessage {
        // The test files
        let files = vec!["test_files/test.proto"];
        let includes = vec!["."];

        // Generate the descriptor
        let desc = ProtoDescriptor::from_files(files, includes).unwrap();

        // Get the method and message
        let method = desc
            .get_method_by_name("proto.TestService", "GetNameOfMonth")
            .unwrap();
        let request = method.input();
        MethodMessage::from_descriptor(request, method)
    }

    #[test]
    fn test_to_json() {
        // given
        let given_message = load_test_request();

        // when
        let json = given_message.to_json();

        // then
        let expected_json = "{\"number\":0}";
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_from_json() {
        // given
        let mut given_message = load_test_request();
        let given_json = "{\"number\":1}";
        given_message.from_json(given_json).unwrap();

        // when
        let json = given_message.to_json();

        // then
        let expected_json = "{\"number\":1}";
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_into_requeset() {
        // given
        let mut given_message = load_test_request();
        given_message.insert_metadata("metadata-key", "metadata-value");

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
            message.get_method_descriptor(),
            given_message.get_method_descriptor()
        );
        assert_eq!(
            message.get_message_descriptor(),
            given_message.get_message_descriptor()
        );
    }
}
