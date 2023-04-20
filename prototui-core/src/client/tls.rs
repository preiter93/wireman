use crate::error::{PTError as Error, Result};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_rustls::{ConfigBuilderExt, HttpsConnector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// The TLS config of the gRPC client. If skip verification is
/// true, the certificates are not verified. This should be used
/// carefully. If custom_certs are set, these are used.
/// By default, we use native certificates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct TlsConfig {
    /// Skip the certificate verification.
    /// Should only be used for testing.
    #[serde(default)]
    skip_verification: bool,
    /// Custom certificates
    custom_certs: Option<String>,
}

/// Certificate verfication
enum TlsMode {
    /// Use native certificates
    NativeCerts,
    /// Skips certificate verification
    SkipVerification,
    /// Use custom certificates
    CustomCerts,
}

impl TlsConfig {
    /// Instantiate a `TlsConfig`
    pub fn new(skip_verification: bool, custom_certs: Option<String>) -> Self {
        Self {
            skip_verification,
            custom_certs,
        }
    }

    /// Returns the https connector matching the tls config
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

    /// Returns the clients tls config as `rustls::ClientConfig`
    fn get_client_config(&self) -> rustls::ClientConfig {
        get_client_config_native_certs()
    }

    /// Returns the certificate verification mode
    fn mode(&self) -> TlsMode {
        if self.skip_verification {
            return TlsMode::SkipVerification;
        }
        if let Some(certs) = &self.custom_certs {
            if !certs.is_empty() {
                return TlsMode::CustomCerts;
            }
        }
        TlsMode::NativeCerts
    }
}

/// Defaults TLS client config with native roots.
fn get_client_config_native_certs() -> rustls::ClientConfig {
    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_native_roots()
        .with_no_client_auth()
}

/// TLS client config without certificiate verification. Should
/// be used with caution.
fn get_client_config_no_tls_check() -> Arc<rustls::ClientConfig> {
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();
    Arc::new(config)
}

/// TLS client config with a custom certificate
fn get_client_config_with_custom_certs(ca_cert: &str) -> Result<Arc<rustls::ClientConfig>> {
    let certs = load_certs(ca_cert)?;
    let mut roots = rustls::RootCertStore::empty();
    roots.add_parsable_certificates(&certs);
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();
    Ok(Arc::new(config))
}

// Load public certificate from file.
fn load_certs(filename: &str) -> Result<Vec<Vec<u8>>> {
    // Open certificate file.
    let certfile =
        std::fs::File::open(filename).map_err(|err| Error::LoadTLSCertificateError(err))?;
    let mut reader = std::io::BufReader::new(certfile);

    // Load and return certificate.
    Ok(rustls_pemfile::certs(&mut reader).map_err(|err| Error::LoadTLSCertificateError(err))?)
}

/// Implementation of `ServerCertVerifier` that verifies everything as trustworthy.
/// Code from `https://quinn-rs.github.io/quinn/quinn/certificate.html`
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> std::result::Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
