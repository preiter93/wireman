#![allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
use thiserror::Error;
pub use PTError as Error;

/// The result type for this library
pub type Result<T> = std::result::Result<T, PTError>;

/// The error type
#[derive(Error, Debug)]
pub enum PTError {
    /// Internal error
    #[error("internal error {0}")]
    InternalError(String),

    /// Error while reading the config file
    #[error("error reading config")]
    ReadConfigError(#[source] std::io::Error),

    /// Error while parsing the config
    #[error("error parsing config")]
    ParseConfigError(#[source] serde_json::Error),

    /// Failed to deserialize DynamicMessage from json
    #[error("error deserializing message from json")]
    DeserializeMessage(#[source] serde_json::Error),

    /// Protox failed to compile the proto files
    #[error("error compiling proto files")]
    ProtoxCompileError(#[source] protox::Error),

    /// Failed to create a grpc channel
    #[error("error creating grpc channel")]
    GrpcChannelCreateError(#[source] protox::Error),

    /// Grpc channel is not ready
    #[error("grpc channel is not ready: {0}")]
    GrpcNotReady(#[from] tonic::transport::Error),

    /// Failed to make a unary grpc call
    #[error("grpc: {0}")]
    GrpcError(GrpcStatus),

    /// Failed to load the custom TLS certificate
    #[error("failed to load custom TLS certificate")]
    LoadTLSCertificateError(#[source] std::io::Error),
}

impl From<tonic::Status> for PTError {
    fn from(status: tonic::Status) -> Self {
        Self::GrpcError(status.into())
    }
}

/// A status describing the result of a gRPC call
#[derive(Debug)]
pub struct GrpcStatus {
    /// The gRPC status code
    pub code: tonic::Code,
    /// The error message
    pub message: String,
}

impl std::fmt::Display for GrpcStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {:?}, message: {:?}", self.code, self.message)
    }
}

impl From<tonic::Status> for GrpcStatus {
    fn from(status: tonic::Status) -> Self {
        GrpcStatus {
            code: status.code(),
            message: status.message().to_owned(),
        }
    }
}
