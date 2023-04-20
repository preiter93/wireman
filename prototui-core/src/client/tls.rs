use crate::error::{PTError as Error, Result};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_rustls::{ConfigBuilderExt, HttpsConnector};
use serde::{Deserialize, Serialize};

/// The TLS config of the gRPC client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct TlsConfig {
    /// Custom certificates
    custom_cert: Option<String>,
}

impl TlsConfig {
    /// Instantiate a `TlsConfig`
    pub fn new(custom_cert: Option<String>) -> Self {
        Self { custom_cert }
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
        if let Some(cert) = &self.custom_cert {
            return get_client_config_with_custom_certs(cert).unwrap();
        }
        get_client_config_native_certs()
    }
}

/// Defaults TLS client config with native roots.
fn get_client_config_native_certs() -> rustls::ClientConfig {
    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_native_roots()
        .with_no_client_auth()
}

/// TLS client config with a custom certificate
fn get_client_config_with_custom_certs(ca_cert: &str) -> Result<rustls::ClientConfig> {
    let certs = load_certs(ca_cert)?;
    let mut roots = rustls::RootCertStore::empty();
    roots.add_parsable_certificates(&certs);
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();
    Ok(config)
}

// Load public certificate from file.
fn load_certs(filename: &str) -> Result<Vec<Vec<u8>>> {
    // Open certificate file.
    let certfile = std::fs::File::open(filename).map_err(Error::LoadTLSCertificateError)?;
    let mut reader = std::io::BufReader::new(certfile);

    // Load and return certificate.
    rustls_pemfile::certs(&mut reader).map_err(Error::LoadTLSCertificateError)
}

// /// TLS client config without certificiate verification. Should
// /// be used with caution.
// fn get_client_config_no_tls_check() -> Arc<rustls::ClientConfig> {
//     let config = rustls::ClientConfig::builder()
//         .with_safe_defaults()
//         .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
//         .with_no_client_auth();
//     Arc::new(config)
// }

// /// Implementation of `ServerCertVerifier` that verifies everything as trustworthy.
// /// Code from `https://quinn-rs.github.io/quinn/quinn/certificate.html`
// struct SkipServerVerification;
//
// impl SkipServerVerification {
//     fn new() -> Arc<Self> {
//         Arc::new(Self)
//     }
// }
//
// impl rustls::client::ServerCertVerifier for SkipServerVerification {
//     fn verify_server_cert(
//         &self,
//         _end_entity: &rustls::Certificate,
//         _intermediates: &[rustls::Certificate],
//         _server_name: &rustls::ServerName,
//         _scts: &mut dyn Iterator<Item = &[u8]>,
//         _ocsp_response: &[u8],
//         _now: std::time::SystemTime,
//     ) -> std::result::Result<rustls::client::ServerCertVerified, rustls::Error> {
//         Ok(rustls::client::ServerCertVerified::assertion())
//     }
// }
