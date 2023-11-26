mod template;

use self::template::apply_template_for_message;
use crate::{
    error::{Error, FROM_UTF8},
    Result,
};
use prost_reflect::{
    DeserializeOptions, DynamicMessage as DynMessage, MessageDescriptor, ReflectMessage,
    SerializeOptions,
};
use serde::{Serialize, Serializer};
use std::ops::{Deref, DerefMut};

/// Represents a dynamic gRPC message that can be used
/// with various message types.
#[derive(Debug, Clone)]
pub struct DynamicMessage {
    inner: DynMessage,
}

impl Deref for DynamicMessage {
    type Target = DynMessage;

    fn deref(&self) -> &DynMessage {
        &self.inner
    }
}

impl DerefMut for DynamicMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

type JsonSerializer = serde_json::Serializer<Vec<u8>>;

impl DynamicMessage {
    /// Create a new `DynamicMessage` from a `MessageDescriptor`.
    #[must_use]
    pub fn new(message_desc: MessageDescriptor) -> Self {
        let message = DynMessage::new(message_desc);
        Self { inner: message }
    }

    /// Get the name of the message as a String.
    #[must_use]
    pub fn message_name(&self) -> String {
        self.descriptor().name().to_string()
    }

    /// Get the message descriptor.
    #[must_use]
    pub fn descriptor(&self) -> MessageDescriptor {
        self.inner.descriptor()
    }

    /// Deserialize a `DynamicMessage` from a JSON string.
    ///
    /// # Errors
    ///
    /// - Failed to deserialize message.
    pub fn from_json(&mut self, json: &str) -> Result<()> {
        let mut de = serde_json::Deserializer::from_str(json);
        let msg = DynMessage::deserialize_with_options(
            self.descriptor(),
            &mut de,
            &DeserializeOptions::new(),
        )
        .map_err(Error::DeserializeMessage)?;
        de.end().map_err(Error::DeserializeMessage)?;
        self.inner = msg;
        Ok(())
    }

    /// Serialize a `DynamicMessage` to a JSON string.
    ///
    /// # Errors
    ///
    /// - Failed to convert utf8 to String
    /// - Failed to serialize message
    pub fn to_json(&self) -> Result<String> {
        let mut s = serde_json::Serializer::new(Vec::new());
        self.serialize(&mut s).map_err(Error::SerializeJsonError)?;
        String::from_utf8(s.into_inner()).map_err(|_| Error::Internal(FROM_UTF8.to_string()))
    }

    /// Apply default values to a `DynamicMessage`.
    pub fn apply_template(&mut self) {
        apply_template_for_message(self);
    }
}

impl Serialize for DynamicMessage {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize_with_options(
            serializer,
            &SerializeOptions::new()
                .stringify_64_bit_integers(false)
                .skip_default_fields(false),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::ProtoDescriptor;

    use super::*;

    #[test]
    fn test_template_nested() {
        // given
        let mut given_message = load_test_message("Nested");
        let expected_json = "{\"items\":[{\"number\":0,\"text\":\"\"}]}";

        // when
        given_message.apply_template();
        let json = given_message.to_json().unwrap();

        // then
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_template_repeated() {
        // given
        let mut given_message = load_test_message("Repeated");
        let expected_json = "{\"number\":[0]}";

        // when
        given_message.apply_template();
        let json = given_message.to_json().unwrap();

        // then
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_template_enum() {
        // given
        let mut given_message = load_test_message("Enum");
        let expected_json = "{\"color\":\"NONE\"}";

        // when
        given_message.apply_template();
        let json = given_message.to_json().unwrap();

        // then
        assert_eq!(json, expected_json);
    }

    fn load_test_message(method: &str) -> DynamicMessage {
        let files = vec!["test_files/test.proto"];
        let includes = vec!["."];

        let desc = ProtoDescriptor::new(includes, files).unwrap();

        let method = desc
            .get_method_by_name("proto.TestService", method)
            .unwrap();
        let request = method.input();
        DynamicMessage::new(request)
    }

    #[test]
    fn test_to_json() {
        // given
        let given_message = load_test_message("Simple");

        // when
        let json = given_message.to_json().unwrap();

        // then
        let expected_json = "{\"number\":0}";
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_from_json() {
        // given
        let mut given_message = load_test_message("Multiple");
        let given_json = "{\"number\":1}";
        given_message.from_json(given_json).unwrap();

        // when
        let json = given_message.to_json().unwrap();

        // then
        let expected_json = "{\"id\":\"\",\"number\":1}";
        assert_eq!(json, expected_json);
    }
}
