use std::ops::{Deref, DerefMut};

use super::DynamicMessage;
use prost_reflect::{MessageDescriptor, MethodDescriptor};

/// Holds all the necessary data for a `gRPC` response.
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

/// Streaming requests and responses.
pub struct StreamingResponse {
    pub inner: tonic::Streaming<ResponseMessage>,
}

impl StreamingResponse {
    pub fn new(inner: tonic::Streaming<ResponseMessage>) -> Self {
        Self { inner }
    }
}

impl Deref for StreamingResponse {
    type Target = tonic::Streaming<ResponseMessage>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for StreamingResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
