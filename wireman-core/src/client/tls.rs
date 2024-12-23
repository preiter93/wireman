#![allow(clippy::module_name_repetitions)]
use crate::error::{Error, Result};
use tonic::transport::{Certificate, ClientTlsConfig};

/// The TLS config of the grpc client.
#[derive(Debug, Clone)]
pub struct TlsConfig(pub(super) ClientTlsConfig);

impl TlsConfig {
    /// Create a new `TlsConfig` with native certificate.
    #[must_use]
    pub fn native() -> Self {
        Self(ClientTlsConfig::new().with_enabled_roots())
    }

    /// Create a new `TlsConfig` with a custom certificate.
    ///
    /// # Errors
    ///
    /// Errors if root certificates cannot be read from path.
    pub fn custom(cert_path: String) -> Result<Self> {
        let pem = std::fs::read_to_string(cert_path).map_err(Error::LoadTLSCertificateError)?;
        let ca = Certificate::from_pem(pem);
        let tls = ClientTlsConfig::new().ca_certificate(ca);

        Ok(Self(tls))
    }
}
