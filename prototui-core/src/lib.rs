#![allow(dead_code)]
#![allow(unused_variables)]
// #![warn(missing_docs)]

pub mod client;
mod config;
pub mod descriptor;
pub mod error;

pub use crate::config::ProtoConfig;
pub use crate::descriptor::ProtoDescriptor;
pub use crate::error::Result;

pub use prost_reflect::MessageDescriptor;
pub use prost_reflect::MethodDescriptor;
pub use prost_reflect::ServiceDescriptor;

pub use crate::client::call_unary;

/// Checks if all requirements are met and initializes the config.
///
/// # Errors
/// - config.json can not be loaded
pub fn init() -> error::Result<config::ProtoConfig> {
    init_from_file("./config.json")
}

/// Checks if all requirements are met and initializes the config.
///
/// # Errors
/// - config.json can not be loaded
pub fn init_from_file(config_file: &str) -> error::Result<config::ProtoConfig> {
    let cfg = config::ProtoConfig::load(config_file)?;

    Ok(cfg)
}
