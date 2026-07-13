mod fixtures;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};

use fixtures::{AppKeyKind, TestPki};
use futures_util::future::join_all;
use lzc_sdk::proto::localdevice::permission_manager_client::PermissionManagerClient;
use lzc_sdk::proto::localdevice::permission_manager_server::{
    PermissionManager, PermissionManagerServer,
};
use lzc_sdk::proto::localdevice::{
    ListPermissionsReply, PermissionReply, PermissionRequest, RequestAuthTokenRequest,
    RequestAuthTokenResponse,
};
use lzc_sdk::{ClientCredentials, Error, TokenProvider, peer_application};
use tempfile::tempdir;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};

#[tokio::test(start_paused = true)]
async fn ed25519_mtls_auth_cache_and_metadata_flow() {
    let pki = TestPki::new(AppKeyKind::Ed25519);
    let fixture = PermissionFixture::new(pki.clone());
    let (url, server) = start_server(&pki, fixture.clone()).await;
    let directory = tempdir().expect("tempdir");
    let credentials = ClientCredentials::load(pki.write_credentials(directory.path()))
        .await
        .expect("load Ed25519 credentials");
    let provider =
        TokenProvider::connect(&format!("{url}/ignored?query=yes#fragment"), credentials)
            .await
            .expect("connect device token provider");

    let results = join_all((0..16).map(|_| {
        let provider = provider.clone();
        async move { provider.token().await }
    }))
    .await;
    for result in results {
        let token = result.expect("concurrent token request");
        assert_eq!(token.expose_secret(), "token-1");
        assert!(!format!("{token:?}").contains("token-1"));
    }
    assert_eq!(fixture.auth_requests.load(Ordering::SeqCst), 1);

    let mut client = PermissionManagerClient::new(provider.authenticated_service());
    client
        .list_permissions(())
        .await
        .expect("authenticated PermissionManager call");
    assert_eq!(fixture.seen_tokens.lock().await.as_slice(), ["token-1"]);

    tokio::time::advance(Duration::from_secs(31)).await;
    assert_eq!(
        provider
            .token()
            .await
            .expect("refresh token")
            .expose_secret(),
        "token-2"
    );
    assert_eq!(fixture.auth_requests.load(Ordering::SeqCst), 2);
    server.abort();
}

#[tokio::test]
async fn rsa_pkcs1_auth_signature_is_go_compatible() {
    let pki = TestPki::new(AppKeyKind::RsaPkcs1);
    let fixture = PermissionFixture::new(pki.clone());
    let (url, server) = start_server(&pki, fixture.clone()).await;
    let directory = tempdir().expect("tempdir");
    let credentials = ClientCredentials::load(pki.write_credentials(directory.path()))
        .await
        .expect("load RSA credentials");
    let http_url = url.replacen("https://", "http://", 1);
    let provider = TokenProvider::connect(&http_url, credentials)
        .await
        .expect("connect RSA token provider");

    assert_eq!(
        provider
            .token()
            .await
            .expect("RSA auth token")
            .expose_secret(),
        "token-1"
    );
    assert_eq!(fixture.auth_requests.load(Ordering::SeqCst), 1);
    server.abort();
}

#[tokio::test]
async fn grpc_unauthenticated_status_is_preserved() {
    let pki = TestPki::new(AppKeyKind::Ed25519);
    let fixture = PermissionFixture::new(pki.clone());
    fixture.reject_auth.store(true, Ordering::SeqCst);
    let (url, server) = start_server(&pki, fixture).await;
    let directory = tempdir().expect("tempdir");
    let credentials = ClientCredentials::load(pki.write_credentials(directory.path()))
        .await
        .expect("load credentials");
    let provider = TokenProvider::connect(&url, credentials)
        .await
        .expect("connect token provider");

    let error = provider.token().await.expect_err("server rejects auth");
    assert!(
        matches!(error, Error::GrpcStatus(ref status) if status.code() == Code::Unauthenticated)
    );
    server.abort();
}

#[derive(Clone)]
struct PermissionFixture {
    pki: TestPki,
    auth_requests: Arc<AtomicUsize>,
    reject_auth: Arc<AtomicBool>,
    seen_tokens: Arc<Mutex<Vec<String>>>,
}

impl PermissionFixture {
    fn new(pki: TestPki) -> Self {
        Self {
            pki,
            auth_requests: Arc::new(AtomicUsize::new(0)),
            reject_auth: Arc::new(AtomicBool::new(false)),
            seen_tokens: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[tonic::async_trait]
impl PermissionManager for PermissionFixture {
    async fn get_permission(
        &self,
        _request: Request<PermissionRequest>,
    ) -> Result<Response<PermissionReply>, Status> {
        Ok(Response::new(PermissionReply { result: true }))
    }

    async fn request_permission(
        &self,
        _request: Request<PermissionRequest>,
    ) -> Result<Response<PermissionReply>, Status> {
        Ok(Response::new(PermissionReply { result: true }))
    }

    async fn list_permissions(
        &self,
        request: Request<()>,
    ) -> Result<Response<ListPermissionsReply>, Status> {
        let token = request
            .metadata()
            .get("lzc_dapi_auth_token")
            .ok_or_else(|| Status::unauthenticated("missing auth token"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("invalid auth token"))?;
        self.seen_tokens.lock().await.push(token.to_owned());
        Ok(Response::new(ListPermissionsReply {
            result: HashMap::new(),
        }))
    }

    async fn request_auth_token(
        &self,
        request: Request<RequestAuthTokenRequest>,
    ) -> Result<Response<RequestAuthTokenResponse>, Status> {
        let application = peer_application(&request)
            .map_err(|error| Status::unauthenticated(error.to_string()))?;
        if application.app_id != self.pki.app_id
            || application.box_id != self.pki.box_id
            || application.app_domain != self.pki.app_domain
        {
            return Err(Status::unauthenticated("unexpected client identity"));
        }
        if self.reject_auth.load(Ordering::SeqCst) {
            return Err(Status::unauthenticated("fixture rejection"));
        }
        let request = request.into_inner();
        if !self.pki.verifies(&request) {
            return Err(Status::permission_denied("invalid signature"));
        }
        let sequence = self.auth_requests.fetch_add(1, Ordering::SeqCst) + 1;
        Ok(Response::new(RequestAuthTokenResponse {
            token: format!("token-{sequence}"),
            deadline: Some(prost_types::Timestamp::from(
                SystemTime::now() + Duration::from_secs(60),
            )),
        }))
    }
}

async fn start_server(
    pki: &TestPki,
    fixture: PermissionFixture,
) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind device fixture");
    let address = listener.local_addr().expect("device fixture address");
    let tls = pki.server_tls_config();
    let server = tokio::spawn(async move {
        Server::builder()
            .tls_config(tls)
            .expect("device fixture TLS")
            .add_service(PermissionManagerServer::new(fixture))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .expect("serve device fixture");
    });
    (format!("https://{address}"), server)
}
