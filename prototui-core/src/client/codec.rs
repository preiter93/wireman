//! This file is taken from `https://github.com/andrewhickman/grpc-client`
use crate::descriptor::message::MethodMessage;
use prost_reflect::prost::Message;
use prost_reflect::{DynamicMessage, MethodDescriptor};
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

#[derive(Debug, Clone)]
pub struct DynamicCodec(MethodDescriptor);

impl DynamicCodec {
    pub fn new(desc: MethodDescriptor) -> Self {
        DynamicCodec(desc)
    }
}

impl Codec for DynamicCodec {
    type Encode = MethodMessage;
    type Decode = MethodMessage;

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
    type Item = MethodMessage;
    type Error = Status;

    fn encode(&mut self, request: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        debug_assert_eq!(request.get_message_descriptor(), self.0.input());
        request
            .get_message()
            .encode(dst)
            .expect("insufficient space for message");
        Ok(())
    }
}

impl Decoder for DynamicCodec {
    type Item = MethodMessage;
    type Error = Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let mut message = DynamicMessage::new(self.0.output());
        message
            .merge(src)
            .map_err(|err| Status::internal(err.to_string()))?;
        let mut response = MethodMessage::from_descriptor(self.0.output(), self.0.clone());
        response.set_message(message);
        Ok(Some(response))
    }
}
