#![allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
use thiserror::Error as ThisError;

/// The result type for this library
pub type Result<T> = std::result::Result<T, Error>;

/// The error type
#[derive(ThisError, Debug)]
pub enum Error {
    /// Internal error
    #[error("internal error {0}")]
    Internal(String),

    /// The method was not found in the proto descriptor
    #[error("method {0} not found")]
    MethodNotFound(String),

    /// Failed to deserialize `DynamicMessage` from json
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

    /// Failed to serialize proto messages
    #[error("failed to serialize proto message")]
    SerializeJsonError(#[source] serde_json::Error),

    /// Failed to serialize proto messages
    #[error("failed to serialize the message")]
    SerializeMessageError(String),

    /// Failed to parse to ascii
    #[error("error parsing to ascii")]
    ParseToAsciiError,

    /// Failed to read or write the history
    #[error("history: {0}")]
    HistoryError(String),
}

impl From<tonic::Status> for Error {
    fn from(status: tonic::Status) -> Self {
        Self::GrpcError(status.into())
    }
}

/// A status describing the result of a grpc call
#[derive(Debug)]
pub struct GrpcStatus {
    /// The grpc status code
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

pub const FROM_UTF8: &str = "From UTF8 error";
