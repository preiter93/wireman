use prost_reflect::{MessageDescriptor, MethodDescriptor};

use super::Message;

/// Holds all the necessary data for a grpc request
#[derive(Debug, Clone)]
pub struct ResponseMessage {
    pub message: Message,
    method_desc: MethodDescriptor,
}

impl ResponseMessage {
    /// Construct `ResponseMessage` from the Descriptors.
    #[must_use]
    pub fn new(message_desc: MessageDescriptor, method_desc: MethodDescriptor) -> Self {
        let message = Message::new(message_desc.clone());
        Self {
            method_desc,
            message,
        }
    }

    /// Update the message
    pub fn set_message(&mut self, message: Message) {
        self.message = message;
    }
}
