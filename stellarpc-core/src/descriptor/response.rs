use super::DynamicMessageWrapper;
use prost_reflect::{MessageDescriptor, MethodDescriptor};

/// Holds all the necessary data for a grpc request
#[derive(Debug, Clone)]
pub struct ResponseMessage {
    pub message: DynamicMessageWrapper,
    method_desc: MethodDescriptor,
}

impl ResponseMessage {
    /// Construct `ResponseMessage` from the Descriptors.
    #[must_use]
    pub fn new(message_desc: MessageDescriptor, method_desc: MethodDescriptor) -> Self {
        let message = DynamicMessageWrapper::new(message_desc);
        Self {
            message,
            method_desc,
        }
    }

    /// Update the message
    pub fn set_message(&mut self, message: DynamicMessageWrapper) {
        self.message = message;
    }
}
