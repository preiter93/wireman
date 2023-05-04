use std::ops::{Deref, DerefMut};

use crate::{error::Error, Result};
use prost_reflect::{
    DeserializeOptions, DynamicMessage, MessageDescriptor, ReflectMessage, SerializeOptions,
};

/// Wrapper of `DynamicMessage`
#[derive(Debug, Clone)]
pub struct Message {
    inner: DynamicMessage,
}

impl Deref for Message {
    type Target = DynamicMessage;

    fn deref(&self) -> &DynamicMessage {
        &self.inner
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Message {
    /// Construct a `Message` from a `MessageDescriptor`
    #[must_use]
    pub fn new(message_desc: MessageDescriptor) -> Self {
        let message = DynamicMessage::new(message_desc.clone());
        Self { inner: message }
    }

    /// Returns the Message name
    #[must_use]
    pub fn message_name(&self) -> String {
        self.descriptor().name().to_string()
    }

    /// Returns the message descriptor
    #[must_use]
    pub fn descriptor(&self) -> MessageDescriptor {
        self.inner.descriptor().clone()
    }

    /// Deserialize a `ProtoMessage` from a json string
    ///
    /// # Errors
    /// - Failed to deserialize message
    pub fn from_json(&mut self, json: &str) -> Result<()> {
        let mut de = serde_json::Deserializer::from_str(json);
        let msg = DynamicMessage::deserialize_with_options(
            self.descriptor(),
            &mut de,
            &DeserializeOptions::new(),
        )
        .map_err(Error::DeserializeMessage)?;
        de.end().map_err(Error::DeserializeMessage)?;
        self.inner = msg;
        Ok(())
    }

    /// Serialize a `ProtoMessage` to a json string
    ///
    /// # Errors
    /// - Failed to convert utf8 to String
    /// - Failed to serialize message
    pub fn to_json(&self) -> Result<String> {
        let mut s = serde_json::Serializer::new(Vec::new());
        self.inner
            .serialize_with_options(
                &mut s,
                &SerializeOptions::new()
                    .stringify_64_bit_integers(false)
                    .skip_default_fields(false),
            )
            .map_err(Error::SerializeProtoMessage)?;

        String::from_utf8(s.into_inner())
            .map_err(|_| Error::InternalError("FromUTF8Error".to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::ProtoDescriptor;

    use super::*;

    // #[test]
    // fn test_message() {
    //     // given
    //     let given_message = load_test_request("Repeated");
    //
    //     // when
    //     // let message = given_message.get_message();
    //     // println!("{:?}", message);
    //     // println!("{:?}", message.to_text_format());
    //     // println!("{:?}", given_message.to_json());
    //     let desc = given_message.descriptor();
    //     // println!("{:?}", desc.full_name());
    //     for field in desc.fields() {
    //         println!("{:?}", field);
    //         let x = field.is_list();
    //         println!("is list {:?}", x);
    //     }
    //
    //     assert_eq!(1, 2);
    // }

    fn load_test_message(method: &str) -> Message {
        let files = vec!["test_files/test.proto"];
        let includes = vec!["."];

        let desc = ProtoDescriptor::from_files(files, includes).unwrap();

        let method = desc
            .get_method_by_name("proto.TestService", method)
            .unwrap();
        let request = method.input();
        Message::new(request)
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
        let mut given_message = load_test_message("Simple");
        let given_json = "{\"number\":1}";
        given_message.from_json(given_json).unwrap();

        // when
        let json = given_message.to_json().unwrap();

        // then
        let expected_json = "{\"number\":1}";
        assert_eq!(json, expected_json);
    }
}
