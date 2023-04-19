use crate::error::Grp3Error as Error;
use crate::Result;
use http::uri::PathAndQuery;
use prost_reflect::DeserializeOptions;
use prost_reflect::MethodDescriptor;
use prost_reflect::SerializeOptions;
use prost_reflect::{DynamicMessage, MessageDescriptor};
use std::str::FromStr;

/// Wrapper around `MessageDescriptor` and `DynamicMessage`
#[derive(Debug, Clone)]
pub struct ProtoMessage {
    message_desc: MessageDescriptor,
    method_desc: MethodDescriptor,
    message: DynamicMessage,
}

impl ProtoMessage {
    /// Construct `ProtoMessage` from a `MessageDescriptor`
    pub fn from_descriptor(message_desc: MessageDescriptor, method_desc: MethodDescriptor) -> Self {
        let message = DynamicMessage::new(message_desc.clone());
        Self {
            message_desc,
            method_desc,
            message,
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
}

#[cfg(test)]
mod test {
    use crate::ProtoDescriptor;

    use super::*;

    fn load_test_request() -> ProtoMessage {
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
        ProtoMessage::from_descriptor(request, method)
    }

    #[test]
    fn test_to_json() {
        // given
        let proto_message = load_test_request();

        // when
        let json = proto_message.to_json();

        // then
        let expected_json = "{\"number\":0}";
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_from_json() {
        // given
        let mut proto_message = load_test_request();
        let given_json = "{\"number\":1}";
        proto_message.from_json(given_json).unwrap();

        // when
        let json = proto_message.to_json();

        // then
        let expected_json = "{\"number\":1}";
        assert_eq!(json, expected_json);
    }
}
