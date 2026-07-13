#![allow(clippy::missing_panics_doc, dead_code)]

use std::fs;
use std::path::Path;

use ed25519_dalek::{Signature as Ed25519Signature, Verifier as _, VerifyingKey};
use lzc_sdk::CredentialPaths;
use lzc_sdk::proto::localdevice::RequestAuthTokenRequest;
use rcgen::{
    BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose, PKCS_ED25519, PKCS_RSA_SHA256,
};
use rsa::pkcs1::EncodeRsaPrivateKey as _;
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::pkcs8::{DecodePrivateKey as _, LineEnding};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tonic::transport::{Certificate, Identity, ServerTlsConfig};

#[derive(Clone, Copy, Debug)]
pub enum AppKeyKind {
    Ed25519,
    RsaPkcs1,
}

#[derive(Clone)]
pub struct TestPki {
    pub ca_certificate_pem: String,
    pub server_certificate_pem: String,
    pub server_private_key_pem: String,
    pub application_certificate_pem: String,
    pub application_private_key_pem: String,
    pub app_id: String,
    pub box_id: String,
    pub app_domain: String,
    verifier: SignatureVerifier,
}

#[derive(Clone)]
enum SignatureVerifier {
    Ed25519(VerifyingKey),
    Rsa(RsaPublicKey),
}

impl TestPki {
    pub fn new(kind: AppKeyKind) -> Self {
        let app_id = "app-123".to_owned();
        let box_id = "box-456".to_owned();
        let app_domain = "app.example.test".to_owned();

        let mut ca_params = CertificateParams::new(Vec::<String>::new()).expect("CA params");
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        ca_params
            .distinguished_name
            .push(DnType::CommonName, "LazyCat Test CA");
        ca_params
            .distinguished_name
            .push(DnType::from_oid(&[2, 5, 4, 5]), box_id.clone());
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

        let algorithm = match kind {
            AppKeyKind::Ed25519 => &PKCS_ED25519,
            AppKeyKind::RsaPkcs1 => &PKCS_RSA_SHA256,
        };
        let app_key = KeyPair::generate_for(algorithm).expect("application key");
        let mut app_params = CertificateParams::new(Vec::<String>::new()).expect("app params");
        app_params
            .distinguished_name
            .push(DnType::CommonName, app_domain.clone());
        app_params
            .distinguished_name
            .push(DnType::from_oid(&[2, 5, 4, 5]), app_id.clone());
        app_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        app_params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth];
        let app_certificate = app_params
            .signed_by(&app_key, &issuer)
            .expect("application certificate");

        let (application_private_key_pem, verifier) = match kind {
            AppKeyKind::Ed25519 => {
                let key = ed25519_dalek::SigningKey::from_pkcs8_der(&app_key.serialize_der())
                    .expect("decode Ed25519 test key");
                (
                    app_key.serialize_pem(),
                    SignatureVerifier::Ed25519(key.verifying_key()),
                )
            }
            AppKeyKind::RsaPkcs1 => {
                let key = RsaPrivateKey::from_pkcs8_der(&app_key.serialize_der())
                    .expect("decode RSA test key");
                let pem = key
                    .to_pkcs1_pem(LineEnding::LF)
                    .expect("encode PKCS#1 test key")
                    .to_string();
                let public_key = RsaPublicKey::from(&key);
                (pem, SignatureVerifier::Rsa(public_key))
            }
        };

        Self {
            ca_certificate_pem: ca_certificate.pem(),
            server_certificate_pem: server_certificate.pem(),
            server_private_key_pem: server_key.serialize_pem(),
            application_certificate_pem: app_certificate.pem(),
            application_private_key_pem,
            app_id,
            box_id,
            app_domain,
            verifier,
        }
    }

    pub fn write_credentials(&self, directory: &Path) -> CredentialPaths {
        let paths = CredentialPaths {
            box_certificate: directory.join("box.crt"),
            application_certificate: directory.join("app.crt"),
            application_private_key: directory.join("app.key"),
        };
        fs::write(&paths.box_certificate, &self.ca_certificate_pem).expect("write CA certificate");
        fs::write(
            &paths.application_certificate,
            &self.application_certificate_pem,
        )
        .expect("write application certificate");
        fs::write(
            &paths.application_private_key,
            &self.application_private_key_pem,
        )
        .expect("write application key");
        paths
    }

    pub fn server_tls_config(&self) -> ServerTlsConfig {
        ServerTlsConfig::new()
            .identity(Identity::from_pem(
                &self.server_certificate_pem,
                &self.server_private_key_pem,
            ))
            .client_ca_root(Certificate::from_pem(&self.ca_certificate_pem))
    }

    pub fn verifies(&self, request: &RequestAuthTokenRequest) -> bool {
        if request.box_cert.as_ref() != self.ca_certificate_pem.as_bytes()
            || request.app_cert.as_ref() != self.application_certificate_pem.as_bytes()
        {
            return false;
        }
        match &self.verifier {
            SignatureVerifier::Ed25519(key) => {
                let Ok(signature) = Ed25519Signature::from_slice(&request.signature) else {
                    return false;
                };
                key.verify(self.app_id.as_bytes(), &signature).is_ok()
            }
            SignatureVerifier::Rsa(key) => key
                .verify(
                    Pkcs1v15Sign::new_unprefixed(),
                    self.app_id.as_bytes(),
                    &request.signature,
                )
                .is_ok(),
        }
    }
}
