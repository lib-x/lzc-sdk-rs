use std::fmt;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rustls::crypto::ring;
use rustls::sign::CertifiedKey;
use secrecy::{ExposeSecret as _, SecretSlice};
use tonic::transport::{ClientTlsConfig, Identity};
use x509_parser::parse_x509_certificate;

use crate::Error;

/// Runtime box certificate path used by `LazyCat` applications.
pub const CA_PATH: &str = "/lzcapp/run/certs/box.crt";
/// Runtime application certificate path used by `LazyCat` applications.
pub const APP_CERT_PATH: &str = "/lzcapp/run/certs/app.crt";
/// Runtime application private-key path used by `LazyCat` applications.
pub const APP_KEY_PATH: &str = "/lzcapp/run/certs/app.key";

/// Files that make up a `LazyCat` application client identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialPaths {
    /// Box certificate sent to the device authentication API.
    pub box_certificate: PathBuf,
    /// Application certificate presented for mTLS.
    pub application_certificate: PathBuf,
    /// Application private key presented for mTLS and request signing.
    pub application_private_key: PathBuf,
}

impl CredentialPaths {
    /// Return the fixed credential paths mounted into `LazyCat` applications.
    #[must_use]
    pub fn runtime() -> Self {
        Self {
            box_certificate: CA_PATH.into(),
            application_certificate: APP_CERT_PATH.into(),
            application_private_key: APP_KEY_PATH.into(),
        }
    }
}

impl Default for CredentialPaths {
    fn default() -> Self {
        Self::runtime()
    }
}

#[derive(Clone)]
pub struct ClientCredentials {
    material: Arc<CredentialMaterial>,
}

struct CredentialMaterial {
    paths: CredentialPaths,
    box_certificate_pem: Vec<u8>,
    application_certificate_pem: Vec<u8>,
    application_private_key_pem: SecretSlice<u8>,
}

impl ClientCredentials {
    /// Load and validate certificate and private-key material from `paths`.
    ///
    /// # Errors
    ///
    /// Returns a typed error if a file cannot be read, a certificate cannot be
    /// decoded, or the application key is unsupported or does not match its
    /// certificate.
    pub async fn load(paths: CredentialPaths) -> Result<Self, Error> {
        let box_certificate_pem = read_credential(&paths.box_certificate).await?;
        let application_certificate_pem = read_credential(&paths.application_certificate).await?;
        let application_private_key_pem = read_credential(&paths.application_private_key).await?;

        validate_certificates(&paths.box_certificate, &box_certificate_pem)?;
        let application_certificates =
            validate_certificates(&paths.application_certificate, &application_certificate_pem)?;
        validate_application_key(
            &paths.application_private_key,
            &application_private_key_pem,
            application_certificates,
        )?;

        Ok(Self {
            material: Arc::new(CredentialMaterial {
                paths,
                box_certificate_pem,
                application_certificate_pem,
                application_private_key_pem: application_private_key_pem.into(),
            }),
        })
    }

    /// Return the paths from which this identity was loaded.
    #[must_use]
    pub fn paths(&self) -> &CredentialPaths {
        &self.material.paths
    }

    pub(crate) fn tls_config(&self) -> ClientTlsConfig {
        ClientTlsConfig::new()
            .domain_name("lazycat-runtime")
            .identity(Identity::from_pem(
                &self.material.application_certificate_pem,
                self.material.application_private_key_pem.expose_secret(),
            ))
            .assume_http2(true)
    }
}

impl fmt::Debug for ClientCredentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClientCredentials")
            .field("paths", &self.material.paths)
            .field(
                "box_certificate",
                &RedactedBytes(&self.material.box_certificate_pem),
            )
            .field(
                "application_certificate",
                &RedactedBytes(&self.material.application_certificate_pem),
            )
            .field(
                "application_private_key",
                &self.material.application_private_key_pem,
            )
            .finish()
    }
}

struct RedactedBytes<'a>(&'a [u8]);

impl fmt::Debug for RedactedBytes<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _length = self.0.len();
        formatter.write_str("<redacted>")
    }
}

async fn read_credential(path: &Path) -> Result<Vec<u8>, Error> {
    tokio::fs::read(path)
        .await
        .map_err(|source| Error::CredentialRead {
            path: path.to_owned(),
            source,
        })
}

fn validate_certificates(
    path: &Path,
    pem: &[u8],
) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>, Error> {
    let certificates = rustls_pemfile::certs(&mut Cursor::new(pem))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::InvalidCertificate {
            path: path.to_owned(),
            reason: "malformed PEM certificate",
        })?;
    if certificates.is_empty() {
        return Err(Error::InvalidCertificate {
            path: path.to_owned(),
            reason: "no certificate found",
        });
    }
    if certificates
        .iter()
        .any(|certificate| parse_x509_certificate(certificate.as_ref()).is_err())
    {
        return Err(Error::InvalidCertificate {
            path: path.to_owned(),
            reason: "invalid X.509 certificate",
        });
    }
    Ok(certificates)
}

fn validate_application_key(
    path: &Path,
    pem: &[u8],
    certificate_chain: Vec<rustls::pki_types::CertificateDer<'static>>,
) -> Result<(), Error> {
    let key = rustls_pemfile::private_key(&mut Cursor::new(pem))
        .map_err(|_| Error::InvalidPrivateKey {
            path: path.to_owned(),
            reason: "malformed PEM private key",
        })?
        .ok_or_else(|| Error::InvalidPrivateKey {
            path: path.to_owned(),
            reason: "no private key found",
        })?;
    CertifiedKey::from_der(certificate_chain, key, &ring::default_provider()).map_err(|_| {
        Error::InvalidPrivateKey {
            path: path.to_owned(),
            reason: "unsupported key or certificate mismatch",
        }
    })?;
    Ok(())
}
