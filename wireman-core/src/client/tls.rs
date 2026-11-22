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

    /// Creates the `TlsConfig` from the TLS settings of the config. Returns
    /// `None` if TLS is disabled.
    ///
    /// # Errors
    ///
    /// Errors if the custom certificate cannot be read.
    pub fn from_settings(
        use_native: Option<bool>,
        custom_cert: Option<String>,
    ) -> Result<Option<Self>> {
        match (use_native, custom_cert) {
            (Some(true), _) => Ok(Some(Self::native())),
            (Some(false), _) => Ok(None),
            (None, Some(custom_cert)) => Ok(Some(Self::custom(custom_cert)?)),
            (None, None) => Ok(Some(Self::native())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_settings() {
        struct TestCase {
            name: &'static str,
            given_use_native: Option<bool>,
            given_custom_cert: Option<String>,
            expected_error: bool,
            expected_tls: bool,
        }

        let test_cases = vec![
            TestCase {
                name: "success: native default",
                given_use_native: None,
                given_custom_cert: None,
                expected_error: false,
                expected_tls: true,
            },
            TestCase {
                name: "success: tls disabled",
                given_use_native: Some(false),
                given_custom_cert: None,
                expected_error: false,
                expected_tls: false,
            },
            TestCase {
                name: "failure: missing cert file",
                given_use_native: None,
                given_custom_cert: Some(String::from("does-not-exist.pem")),
                expected_error: true,
                expected_tls: false,
            },
        ];

        for test_case in test_cases {
            let tls =
                TlsConfig::from_settings(test_case.given_use_native, test_case.given_custom_cert);

            assert_eq!(tls.is_err(), test_case.expected_error, "{}", test_case.name);
            assert_eq!(
                tls.is_ok_and(|tls| tls.is_some()),
                test_case.expected_tls,
                "{}",
                test_case.name
            );
        }
    }
}
