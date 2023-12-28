//! Configuration module for `WireMan`.
//!
//! This module provides functionality for defining and reading the configuration
//! for `WireMan`. The config file is read from a JSON file to customize `WireMan`.
//!
//! The config contains:
//!
//! - `includes`: A list of include directories for `gRPC`.
//! - `files`: A list of .proto files to include.
//! - `address`: The address of the `gRPC` server.
pub mod config;
pub mod error;
pub use config::Config;
