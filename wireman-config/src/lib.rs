//! Configuration module for `WireMan`.
//!
//! This module provides functionality for defining and reading the configuration
//! for `WireMan`. The config file is read from a JSON file to customize `WireMan`.
//!
//! The config contains:
//!
//! - `includes`: A list of include directories for `gRPC`.
//! - `files`: A list of .proto files to include.
//! - `default_address`: The default address of the `gRPC` server.
//! - `history_dir`: The folder path where the history should be kept
pub mod config;
pub mod error;
pub use config::Config;
