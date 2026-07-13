use tonic::Request;
use tonic::transport::server::{TcpConnectInfo, TlsConnectInfo};
use x509_parser::oid_registry::Oid;
use x509_parser::parse_x509_certificate;
use x509_parser::x509::X509Name;

use crate::Error;

/// Identity encoded in a `LazyCat` application client certificate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application {
    /// Application ID from the subject serial-number attribute.
    pub app_id: String,
    /// Box ID from the issuer serial-number attribute.
    pub box_id: String,
    /// Application domain from the subject common name.
    pub app_domain: String,
}

/// Extract the authenticated `LazyCat` application identity from a server request.
///
/// # Errors
///
/// Returns an error unless the request carries exactly one valid TLS peer
/// certificate.
pub fn peer_application<T>(request: &Request<T>) -> Result<Application, Error> {
    let certificates = request
        .extensions()
        .get::<TlsConnectInfo<TcpConnectInfo>>()
        .and_then(TlsConnectInfo::peer_certs);
    #[cfg(unix)]
    let certificates = certificates.or_else(|| {
        request
            .extensions()
            .get::<TlsConnectInfo<tonic::transport::server::UdsConnectInfo>>()
            .and_then(TlsConnectInfo::peer_certs)
    });
    let certificates = certificates.ok_or(Error::UnauthenticatedPeer)?;
    application_from_certificates(certificates.as_ref())
}

fn application_from_certificates(
    certificates: &[rustls::pki_types::CertificateDer<'static>],
) -> Result<Application, Error> {
    if certificates.len() != 1 {
        return Err(Error::InvalidPeerCertificateCount {
            count: certificates.len(),
        });
    }
    let (_, certificate) = parse_x509_certificate(certificates[0].as_ref())
        .map_err(|_| Error::InvalidPeerCertificate)?;
    Ok(Application {
        app_id: serial_number(certificate.subject()),
        box_id: serial_number(certificate.issuer()),
        app_domain: certificate
            .subject()
            .iter_common_name()
            .next()
            .and_then(|name| name.as_str().ok())
            .unwrap_or_default()
            .to_owned(),
    })
}

fn serial_number(name: &X509Name<'_>) -> String {
    let oid = Oid::from(&[2, 5, 4, 5]).expect("serialNumber OID is valid");
    name.iter_by_oid(&oid)
        .next()
        .and_then(|value| value.as_str().ok())
        .unwrap_or_default()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use rustls::pki_types::CertificateDer;

    use crate::Error;

    use super::application_from_certificates;

    #[test]
    fn rejects_zero_or_multiple_peer_certificates() {
        let zero = application_from_certificates(&[]).expect_err("zero certificates");
        assert!(matches!(
            zero,
            Error::InvalidPeerCertificateCount { count: 0 }
        ));

        let invalid = CertificateDer::from(vec![0_u8]);
        let multiple = application_from_certificates(&[invalid.clone(), invalid])
            .expect_err("multiple certificates");
        assert!(matches!(
            multiple,
            Error::InvalidPeerCertificateCount { count: 2 }
        ));
    }
}
