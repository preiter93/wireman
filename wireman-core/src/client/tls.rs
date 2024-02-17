#![allow(clippy::module_name_repetitions)]
use crate::error::{Error, Result};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use hyper_rustls::HttpsConnectorBuilder;
use rustls::pki_types::CertificateDer;
use rustls::RootCertStore;
use serde::{Deserialize, Serialize};

/// The TLS config of the grpc client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct TlsConfig {
    /// Custom certificates
    custom_cert: Option<String>,
}

impl TlsConfig {
    /// Create a new `TlsConfig` with optional custom certificates.
    #[must_use]
    pub fn new(custom_cert: Option<String>) -> Self {
        Self { custom_cert }
    }

    /// Get the HTTPS connector based on the TLS configuration.
    #[must_use]
    pub fn get_connector_from_tls(&self) -> HttpsConnector<HttpConnector> {
        let tls = self.get_client_config();

        let mut http = HttpConnector::new();
        http.enforce_http(false);

        HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_or_http()
            .enable_http2()
            .wrap_connector(http)
    }

    /// Get the client's TLS configuration as a `rustls::ClientConfig`.
    pub fn get_client_config(&self) -> rustls::ClientConfig {
        if let Some(cert) = &self.custom_cert {
            return get_client_config_with_custom_certs(cert).unwrap();
        }
        get_client_config_native_certs()
    }
}

/// Defaults TLS client config with native roots.
fn get_client_config_native_certs() -> rustls::ClientConfig {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };
    rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}

/// Get TLS client configuration with a custom certificate.
fn get_client_config_with_custom_certs(ca_cert: &str) -> Result<rustls::ClientConfig> {
    let roots = root_store_from_cert_file(ca_cert)?;
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    Ok(config)
}

// Load public certificate from file.
fn root_store_from_cert_file(filename: &str) -> Result<RootCertStore> {
    // Open certificate file.
    let certfile = std::fs::File::open(filename).map_err(Error::LoadTLSCertificateError)?;
    let mut reader = std::io::BufReader::new(certfile);

    // Read the certificates
    let certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut reader)
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();

    // Parse the certificates
    let mut roots = rustls::RootCertStore::empty();
    roots.add_parsable_certificates(certs);

    Ok(roots)
}
