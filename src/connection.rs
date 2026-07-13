use std::path::Path;
use std::sync::Arc;

use http::Uri;
use hyper_util::rt::TokioIo;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{CryptoProvider, ring, verify_tls12_signature, verify_tls13_signature};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, SignatureScheme};
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint};
use tower::service_fn;

use crate::{ClientCredentials, CredentialPaths, Error};

/// Runtime API Unix socket mounted into `LazyCat` applications.
pub const RUNTIME_SOCKET_PATH: &str = "/lzcapp/run/sys/lzc-apis.socket";
/// `HPortalSys` Unix socket mounted into `LazyCat` applications.
pub const PORTAL_SOCKET_PATH: &str = "/lzcapp/run/sys/portal-server.socket";

const API_GATEWAY_ADDRESS_ENV: &str = "LZCAPP_API_GATEWAY_ADDRESS";

/// Connect to the `LazyCat` runtime API using the mounted application identity.
///
/// # Errors
///
/// Returns an error when credentials cannot be loaded or the runtime endpoint
/// cannot be reached.
pub async fn connect_api() -> Result<Channel, Error> {
    let credentials = ClientCredentials::load(CredentialPaths::runtime()).await?;
    connect_api_with(credentials).await
}

/// Connect to the `LazyCat` runtime API using preloaded credentials.
///
/// `LZCAPP_API_GATEWAY_ADDRESS` has the same precedence as the official SDK:
/// a non-empty value selects a plaintext TCP endpoint; otherwise the runtime
/// mTLS Unix socket is used.
///
/// # Errors
///
/// Returns an error when the override is invalid or the endpoint cannot be
/// reached.
pub async fn connect_api_with(credentials: ClientCredentials) -> Result<Channel, Error> {
    let gateway = std::env::var(API_GATEWAY_ADDRESS_ENV)
        .ok()
        .filter(|address| !address.trim().is_empty());
    connect_api_with_target(
        credentials,
        gateway.as_deref(),
        Path::new(RUNTIME_SOCKET_PATH),
    )
    .await
}

async fn connect_api_with_target(
    credentials: ClientCredentials,
    gateway: Option<&str>,
    socket_path: &Path,
) -> Result<Channel, Error> {
    if let Some(gateway) = gateway {
        return Ok(Endpoint::from_shared(normalize_gateway_address(gateway)?)?
            .connect()
            .await?);
    }

    let endpoint = Endpoint::from_static("https://lazycat-runtime").tls_config_with_verifier(
        credentials.tls_config("lazycat-runtime"),
        compatibility_server_verifier(),
    )?;
    let socket_path = Arc::new(socket_path.to_owned());
    let connector = service_fn(move |_: Uri| {
        let socket_path = Arc::clone(&socket_path);
        async move {
            UnixStream::connect(socket_path.as_ref())
                .await
                .map(TokioIo::new)
        }
    });
    Ok(endpoint.connect_with_connector(connector).await?)
}

fn normalize_gateway_address(address: &str) -> Result<String, Error> {
    let address = address.trim();
    let candidate = if address.contains("://") {
        address.to_owned()
    } else {
        format!("http://{address}")
    };
    let uri = candidate
        .parse::<Uri>()
        .map_err(|_| Error::InvalidGatewayAddress)?;
    let authority = uri.authority().ok_or(Error::InvalidGatewayAddress)?;
    Ok(format!("http://{authority}"))
}

pub(crate) fn compatibility_server_verifier() -> Arc<dyn ServerCertVerifier> {
    // Go's InsecureSkipVerify skips chain and hostname validation while still
    // verifying the handshake signature. Keep this verifier private and scoped
    // to LazyCat compatibility transports.
    Arc::new(CompatibilityServerVerifier(ring::default_provider()))
}

#[derive(Debug)]
struct CompatibilityServerVerifier(CryptoProvider);

impl ServerCertVerifier for CompatibilityServerVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        certificate: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            certificate,
            signature,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        certificate: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            certificate,
            signature,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use rcgen::{
        BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer,
        KeyPair, KeyUsagePurpose,
    };
    use tempfile::tempdir;
    use tokio::net::{TcpListener, UnixListener};
    use tokio_stream::wrappers::{TcpListenerStream, UnixListenerStream};
    use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
    use tonic::{Request, Response, Status};

    use crate::peer_application;
    use crate::proto::sys::VersionInfo;
    use crate::proto::sys::version_info_service_client::VersionInfoServiceClient;
    use crate::proto::sys::version_info_service_server::{
        VersionInfoService, VersionInfoServiceServer,
    };
    use crate::{ClientCredentials, CredentialPaths};

    use super::{connect_api_with_target, normalize_gateway_address};

    #[test]
    fn normalizes_plaintext_gateway_overrides() {
        assert_eq!(
            normalize_gateway_address("127.0.0.1:1234").expect("host and port"),
            "http://127.0.0.1:1234"
        );
        assert_eq!(
            normalize_gateway_address("https://gateway.example:8443/path").expect("HTTP URI"),
            "http://gateway.example:8443"
        );
    }

    #[tokio::test]
    async fn connects_to_runtime_uds_with_client_identity() {
        let directory = tempdir().expect("tempdir");
        let certificates = test_certificates();
        let credentials = write_client_credentials(directory.path(), &certificates).await;
        let socket_path = directory.path().join("runtime.socket");
        let listener = UnixListener::bind(&socket_path).expect("bind Unix socket");
        let tls = ServerTlsConfig::new()
            .identity(Identity::from_pem(
                &certificates.server_certificate_pem,
                &certificates.server_private_key_pem,
            ))
            .client_ca_root(Certificate::from_pem(&certificates.ca_certificate_pem));
        let server = tokio::spawn(async move {
            Server::builder()
                .tls_config(tls)
                .expect("server TLS config")
                .add_service(VersionInfoServiceServer::new(PeerIdentityService))
                .serve_with_incoming(UnixListenerStream::new(listener))
                .await
                .expect("serve runtime fixture");
        });

        let channel = connect_api_with_target(credentials, None, &socket_path)
            .await
            .expect("connect runtime UDS");
        let response = VersionInfoServiceClient::new(channel)
            .get(())
            .await
            .expect("query identity")
            .into_inner();

        assert_eq!(response.version, "app-123|box-456|app.example.test");
        server.abort();
    }

    #[tokio::test]
    async fn gateway_override_precedes_runtime_socket() {
        let directory = tempdir().expect("tempdir");
        let certificates = test_certificates();
        let credentials = write_client_credentials(directory.path(), &certificates).await;
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind TCP fixture");
        let address = listener.local_addr().expect("TCP fixture address");
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(VersionInfoServiceServer::new(PlainVersionService))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .expect("serve TCP fixture");
        });

        let channel = connect_api_with_target(
            credentials,
            Some(&address.to_string()),
            Path::new("/socket/that/does/not/exist"),
        )
        .await
        .expect("connect gateway override");
        let response = VersionInfoServiceClient::new(channel)
            .get(())
            .await
            .expect("query plaintext fixture")
            .into_inner();

        assert_eq!(response.version, "plaintext");
        server.abort();
    }

    #[derive(Debug)]
    struct PeerIdentityService;

    #[tonic::async_trait]
    impl VersionInfoService for PeerIdentityService {
        async fn get(&self, request: Request<()>) -> Result<Response<VersionInfo>, Status> {
            let application = peer_application(&request)
                .map_err(|error| Status::unauthenticated(error.to_string()))?;
            Ok(Response::new(VersionInfo {
                version: format!(
                    "{}|{}|{}",
                    application.app_id, application.box_id, application.app_domain
                ),
            }))
        }
    }

    #[derive(Debug)]
    struct PlainVersionService;

    #[tonic::async_trait]
    impl VersionInfoService for PlainVersionService {
        async fn get(&self, _request: Request<()>) -> Result<Response<VersionInfo>, Status> {
            Ok(Response::new(VersionInfo {
                version: "plaintext".to_owned(),
            }))
        }
    }

    #[allow(clippy::struct_field_names)]
    struct TestCertificates {
        ca_certificate_pem: String,
        server_certificate_pem: String,
        server_private_key_pem: String,
        client_certificate_pem: String,
        client_private_key_pem: String,
    }

    fn test_certificates() -> TestCertificates {
        let mut ca_params = CertificateParams::new(Vec::<String>::new()).expect("CA params");
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        ca_params
            .distinguished_name
            .push(DnType::CommonName, "LazyCat Test CA");
        ca_params
            .distinguished_name
            .push(DnType::from_oid(&[2, 5, 4, 5]), "box-456");
        ca_params.key_usages = vec![
            KeyUsagePurpose::DigitalSignature,
            KeyUsagePurpose::KeyCertSign,
            KeyUsagePurpose::CrlSign,
        ];
        let ca_key = KeyPair::generate().expect("CA key");
        let ca_certificate = ca_params.self_signed(&ca_key).expect("CA certificate");
        let issuer = Issuer::new(ca_params, ca_key);

        let server_key = KeyPair::generate().expect("server key");
        let mut server_params =
            CertificateParams::new(["localhost".to_owned()]).expect("server params");
        server_params
            .distinguished_name
            .push(DnType::CommonName, "localhost");
        server_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        server_params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
        let server_certificate = server_params
            .signed_by(&server_key, &issuer)
            .expect("server certificate");

        let client_key = KeyPair::generate().expect("client key");
        let mut client_params =
            CertificateParams::new(Vec::<String>::new()).expect("client params");
        client_params
            .distinguished_name
            .push(DnType::CommonName, "app.example.test");
        client_params
            .distinguished_name
            .push(DnType::from_oid(&[2, 5, 4, 5]), "app-123");
        client_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        client_params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth];
        let client_certificate = client_params
            .signed_by(&client_key, &issuer)
            .expect("client certificate");

        TestCertificates {
            ca_certificate_pem: ca_certificate.pem(),
            server_certificate_pem: server_certificate.pem(),
            server_private_key_pem: server_key.serialize_pem(),
            client_certificate_pem: client_certificate.pem(),
            client_private_key_pem: client_key.serialize_pem(),
        }
    }

    async fn write_client_credentials(
        directory: &Path,
        certificates: &TestCertificates,
    ) -> ClientCredentials {
        let paths = CredentialPaths {
            box_certificate: directory.join("box.crt"),
            application_certificate: directory.join("app.crt"),
            application_private_key: directory.join("app.key"),
        };
        fs::write(&paths.box_certificate, &certificates.ca_certificate_pem)
            .expect("write CA certificate");
        fs::write(
            &paths.application_certificate,
            &certificates.client_certificate_pem,
        )
        .expect("write client certificate");
        fs::write(
            &paths.application_private_key,
            &certificates.client_private_key_pem,
        )
        .expect("write client key");
        ClientCredentials::load(paths)
            .await
            .expect("load client credentials")
    }
}
