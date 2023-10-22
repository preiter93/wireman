use super::DynamicMessage;
use prost_reflect::{MessageDescriptor, MethodDescriptor};

/// Holds all the necessary data for a gRPC response.
#[derive(Debug, Clone)]
pub struct ResponseMessage {
    pub message: DynamicMessage,
    method_desc: MethodDescriptor,
}

impl ResponseMessage {
    /// Create a new `ResponseMessage` with the provided message descriptor and method descriptor.
    #[must_use]
    pub fn new(message_desc: MessageDescriptor, method_desc: MethodDescriptor) -> Self {
        let message = DynamicMessage::new(message_desc);
        Self {
            message,
            method_desc,
        }
    }

    // Set a new message for the response.
    pub fn set_message(&mut self, message: DynamicMessage) {
        self.message = message;
    }
}
