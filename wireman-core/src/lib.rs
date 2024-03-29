#![allow(dead_code)]
#![allow(unused_variables)]

pub mod client;
pub mod descriptor;
pub mod error;
pub mod features;

pub use crate::descriptor::ProtoDescriptor;
pub use crate::error::Result;

pub use prost_reflect::MessageDescriptor;
pub use prost_reflect::MethodDescriptor;
pub use prost_reflect::ServiceDescriptor;
