//! Configuration module for `WireMan`.
//!
//! This module provides functionality for defining and reading the configuration
//! for `WireMan`. The config file is read from a JSON file to customize `WireMan`.
//!
//! The config contains:
//!
//! - `includes`: A list of include directories for `gRPC`.
//! - `files`: A list of .proto files to include.
//! - `server`
//!   - `default_address`: The default address of the `gRPC` server.
//! - `history`
//!   - `directory`: The folder path where the history should be kept
pub mod cli;
pub mod config;
pub mod error;
mod setup;
pub use config::Config;
pub use setup::init_from_env;

/// This env is used to read the path for the `WireMan` config.
/// If it is not set, the config is expected in the current
/// directory.
pub const ENV_CONFIG_DIR: &str = "WIREMAN_CONFIG_DIR";

/// The wireman config filename
pub const CONFIG_FNAME: &str = "wireman.toml";
