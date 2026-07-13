use std::fs;

use lzc_sdk::{
    APP_CERT_PATH, APP_KEY_PATH, CA_PATH, ClientCredentials, CredentialPaths, Error,
    RUNTIME_SOCKET_PATH, peer_application, with_real_uid,
};
use rcgen::{CertifiedKey, generate_simple_self_signed};
use tempfile::tempdir;
use tonic::Request;

#[test]
fn runtime_credential_paths_match_lazycat_contract() {
    let paths = CredentialPaths::runtime();

    assert_eq!(paths.box_certificate.to_string_lossy(), CA_PATH);
    assert_eq!(
        paths.application_certificate.to_string_lossy(),
        APP_CERT_PATH
    );
    assert_eq!(
        paths.application_private_key.to_string_lossy(),
        APP_KEY_PATH
    );
    assert_eq!(RUNTIME_SOCKET_PATH, "/lzcapp/run/sys/lzc-apis.socket");
}

#[tokio::test]
async fn credentials_load_from_injected_paths_and_redact_debug() {
    let directory = tempdir().expect("tempdir");
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(["sdk.test".to_owned()]).expect("test certificate");
    let certificate_pem = cert.pem();
    let private_key_pem = signing_key.serialize_pem();

    let paths = CredentialPaths {
        box_certificate: directory.path().join("box.crt"),
        application_certificate: directory.path().join("app.crt"),
        application_private_key: directory.path().join("app.key"),
    };
    fs::write(&paths.box_certificate, &certificate_pem).expect("write box certificate");
    fs::write(&paths.application_certificate, &certificate_pem)
        .expect("write application certificate");
    fs::write(&paths.application_private_key, &private_key_pem).expect("write application key");

    let credentials = ClientCredentials::load(paths.clone())
        .await
        .expect("load credentials");
    let debug = format!("{credentials:?}");

    assert!(debug.contains("ClientCredentials"));
    assert!(debug.contains(&paths.application_certificate.display().to_string()));
    assert!(!debug.contains("BEGIN CERTIFICATE"));
    assert!(!debug.contains("BEGIN PRIVATE KEY"));
    assert!(!debug.contains(private_key_pem.trim()));
}

#[test]
fn real_uid_metadata_matches_grpc_wire_key() {
    let mut request = Request::new(());

    with_real_uid(&mut request, "user-42").expect("valid metadata");

    assert_eq!(
        request
            .metadata()
            .get("x-hc-user-id")
            .expect("real UID metadata"),
        "user-42"
    );
}

#[test]
fn empty_real_uid_is_a_noop_and_invalid_values_are_rejected() {
    let mut request = Request::new(());
    with_real_uid(&mut request, "").expect("empty UID is a no-op");
    assert!(request.metadata().get("x-hc-user-id").is_none());

    let error = with_real_uid(&mut request, "invalid\nuid").expect_err("invalid metadata value");
    assert!(matches!(error, Error::InvalidMetadataValue(_)));
}

#[test]
fn peer_application_requires_tls_client_identity() {
    let request = Request::new(());

    let error = peer_application(&request).expect_err("request has no TLS peer certificate");

    assert!(matches!(error, Error::UnauthenticatedPeer));
}
