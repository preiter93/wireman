//! This file is taken from `https://github.com/andrewhickman/grpc-client`
use crate::descriptor::{DynamicMessage, RequestMessage, ResponseMessage};
use prost_reflect::prost::Message;
use prost_reflect::MethodDescriptor;
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

/// DynamicCodec is a customizable gRPC codec that can handle requests and responses with
/// dynamic message descriptors.
#[derive(Debug, Clone)]
pub struct DynamicCodec(MethodDescriptor);

impl DynamicCodec {
    /// Create a new DynamicCodec with the provided [`MethodDescriptor`].
    pub fn new(desc: MethodDescriptor) -> Self {
        DynamicCodec(desc)
    }
}

impl Codec for DynamicCodec {
    type Encode = RequestMessage;
    type Decode = ResponseMessage;

    type Encoder = DynamicCodec;
    type Decoder = DynamicCodec;

    fn encoder(&mut self) -> Self::Encoder {
        self.clone()
    }

    fn decoder(&mut self) -> Self::Decoder {
        self.clone()
    }
}

impl Encoder for DynamicCodec {
    type Item = RequestMessage;
    type Error = Status;

    // Encodes a request message and writes it to the destination buffer.
    fn encode(&mut self, request: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        // Ensure the request message descriptor matches the codec's input descriptor.
        debug_assert_eq!(request.message_descriptor(), self.0.input());

        // Encode the request message into the destination buffer.
        request
            .message
            .encode(dst)
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(())
    }
}

impl Decoder for DynamicCodec {
    type Item = ResponseMessage;
    type Error = Status;

    /// Decodes a response message from the source buffer.
    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        // Create a dynamic message with the codec's output descriptor.
        let mut message = DynamicMessage::new(self.0.output());

        // Merge the source buffer into the dynamic message.
        message
            .merge(src)
            .map_err(|err| Status::internal(err.to_string()))?;

        // Create a response message and set its message to the decoded dynamic message.
        let mut response = ResponseMessage::new(self.0.output(), self.0.clone());
        response.set_message(message);

        Ok(Some(response))
    }
}
