use std::sync::Arc;

use http::Uri;
use hyper_util::rt::TokioIo;
use lzc_sdk::proto::sys::h_portal_sys_server::{HPortalSys, HPortalSysServer};
use lzc_sdk::proto::sys::{
    ChangeRoleReqeust, ChangeTrustEndDeviceReply, ChangeTrustEndDeviceRequest,
    CheckPasswordRequest, CreateUserRequest, DeleteUserRequest, EmitBoxServiceChangedRequest,
    GetPasswordHashRequest, GetPasswordHashResponse, HServerInfo, ListEndDeviceReply,
    ListEndDeviceRequest, ListUsersReply, PeersInfo, QueryBoxServicePeerCredRequest,
    QueryBoxServicePeerCredResponse, QueryRoleReply, RegisterBoxServiceReply,
    RegisterBoxServiceRequest, RemoteSocksReply, RemoteSocksRequest, ResetHServerReply,
    ResetHServerRequest, ResetPasswordRequest, SetPasswordHashRequest, SetRelayRequest,
    SetupHServerReply, SetupHServerRequest, UserId, remote_socks_request,
};
use lzc_sdk::{Error, HServerClient, RemoteSocksEndpoint};
use tempfile::tempdir;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;
use tokio_stream::wrappers::{ReceiverStream, UnixListenerStream};
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Request, Response, Status};
use tower::service_fn;

#[tokio::test]
async fn hserver_uses_uds_and_maps_local_and_remote_locations() {
    let fixture = PortalFixture::default();
    let (_directory, channel, server) = start_portal(fixture.clone()).await;
    let portal = HServerClient::from_channel(channel);

    let local = portal
        .remote_socks_endpoint("")
        .await
        .expect("local SOCKS endpoint");
    assert_eq!(local.authority(), "127.0.0.1:1080");
    assert!(!local.resolves_hostname_remotely());

    let remote = portal
        .remote_socks_endpoint("peer-42")
        .await
        .expect("remote SOCKS endpoint");
    assert_eq!(remote.authority(), "[::1]:2080");
    assert!(remote.resolves_hostname_remotely());

    let requests = fixture.requests.lock().await;
    assert_eq!(
        requests.as_slice(),
        [
            RemoteSocksRequest {
                location_type: remote_socks_request::LocationType::Local.into(),
                target: String::new(),
            },
            RemoteSocksRequest {
                location_type: remote_socks_request::LocationType::Remote.into(),
                target: "peer-42".to_owned(),
            },
        ]
    );
    server.abort();
}

#[tokio::test]
async fn hserver_exposes_the_complete_generated_client() {
    let fixture = PortalFixture::default();
    let (_directory, channel, server) = start_portal(fixture).await;
    let portal = HServerClient::from_channel(channel);

    let info = portal
        .client()
        .query_h_server_info(())
        .await
        .expect("query HServer info")
        .into_inner();
    assert_eq!(info.box_id, "box-fixture");
    server.abort();
}

#[tokio::test]
async fn hserver_rejects_invalid_remote_socks_endpoints() {
    let fixture = PortalFixture::default();
    let (_directory, channel, server) = start_portal(fixture).await;
    let portal = HServerClient::from_channel(channel);

    let error = portal
        .remote_socks_endpoint("invalid-endpoint")
        .await
        .expect_err("HTTP endpoint must be rejected");
    assert!(matches!(error, Error::InvalidRemoteSocksEndpoint));
    assert!("socks5://127.0.0.1".parse::<RemoteSocksEndpoint>().is_err());
    server.abort();
}

async fn start_portal(
    fixture: PortalFixture,
) -> (tempfile::TempDir, Channel, tokio::task::JoinHandle<()>) {
    let directory = tempdir().expect("tempdir");
    let socket_path = Arc::new(directory.path().join("portal.socket"));
    let listener = UnixListener::bind(socket_path.as_ref()).expect("bind portal socket");
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(HPortalSysServer::new(fixture))
            .serve_with_incoming(UnixListenerStream::new(listener))
            .await
            .expect("serve HPortalSys fixture");
    });

    let endpoint = Endpoint::from_static("http://lazycat-hportal");
    let channel = endpoint
        .connect_with_connector(service_fn(move |_: Uri| {
            let socket_path = Arc::clone(&socket_path);
            async move {
                UnixStream::connect(socket_path.as_ref())
                    .await
                    .map(TokioIo::new)
            }
        }))
        .await
        .expect("connect portal UDS");
    (directory, channel, server)
}

#[derive(Clone, Default)]
struct PortalFixture {
    requests: Arc<Mutex<Vec<RemoteSocksRequest>>>,
}

#[tonic::async_trait]
impl HPortalSys for PortalFixture {
    async fn query_h_server_info(
        &self,
        _request: Request<()>,
    ) -> Result<Response<HServerInfo>, Status> {
        Ok(Response::new(HServerInfo {
            box_id: "box-fixture".to_owned(),
            ..HServerInfo::default()
        }))
    }

    async fn remote_socks(
        &self,
        request: Request<RemoteSocksRequest>,
    ) -> Result<Response<RemoteSocksReply>, Status> {
        let request = request.into_inner();
        let server_url = match request.target.as_str() {
            "" => "socks5://127.0.0.1:1080",
            "invalid-endpoint" => "http://127.0.0.1:1080",
            _ => "socks5h://[::1]:2080",
        };
        self.requests.lock().await.push(request);
        Ok(Response::new(RemoteSocksReply {
            server_url: server_url.to_owned(),
        }))
    }

    type RegisterBoxServiceStream = ReceiverStream<Result<RegisterBoxServiceReply, Status>>;

    async fn list_users(&self, _request: Request<()>) -> Result<Response<ListUsersReply>, Status> {
        Err(Status::unimplemented("list_users"))
    }

    async fn create_user(
        &self,
        _request: Request<CreateUserRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("create_user"))
    }

    async fn delete_user(
        &self,
        _request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("delete_user"))
    }

    async fn reset_password(
        &self,
        _request: Request<ResetPasswordRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("reset_password"))
    }

    async fn check_password(
        &self,
        _request: Request<CheckPasswordRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("check_password"))
    }

    async fn get_password_hash(
        &self,
        _request: Request<GetPasswordHashRequest>,
    ) -> Result<Response<GetPasswordHashResponse>, Status> {
        Err(Status::unimplemented("get_password_hash"))
    }

    async fn set_password_hash(
        &self,
        _request: Request<SetPasswordHashRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("set_password_hash"))
    }

    async fn query_role(
        &self,
        _request: Request<UserId>,
    ) -> Result<Response<QueryRoleReply>, Status> {
        Err(Status::unimplemented("query_role"))
    }

    async fn change_role(
        &self,
        _request: Request<ChangeRoleReqeust>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("change_role"))
    }

    async fn change_trust_end_device(
        &self,
        _request: Request<ChangeTrustEndDeviceRequest>,
    ) -> Result<Response<ChangeTrustEndDeviceReply>, Status> {
        Err(Status::unimplemented("change_trust_end_device"))
    }

    async fn list_end_devices(
        &self,
        _request: Request<ListEndDeviceRequest>,
    ) -> Result<Response<ListEndDeviceReply>, Status> {
        Err(Status::unimplemented("list_end_devices"))
    }

    async fn setup_h_server(
        &self,
        _request: Request<SetupHServerRequest>,
    ) -> Result<Response<SetupHServerReply>, Status> {
        Err(Status::unimplemented("setup_h_server"))
    }

    async fn reset_h_server(
        &self,
        _request: Request<ResetHServerRequest>,
    ) -> Result<Response<ResetHServerReply>, Status> {
        Err(Status::unimplemented("reset_h_server"))
    }

    async fn register_box_service(
        &self,
        _request: Request<RegisterBoxServiceRequest>,
    ) -> Result<Response<Self::RegisterBoxServiceStream>, Status> {
        Err(Status::unimplemented("register_box_service"))
    }

    async fn emit_box_service_changed(
        &self,
        _request: Request<EmitBoxServiceChangedRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("emit_box_service_changed"))
    }

    async fn query_box_service_peer_cred(
        &self,
        _request: Request<QueryBoxServicePeerCredRequest>,
    ) -> Result<Response<QueryBoxServicePeerCredResponse>, Status> {
        Err(Status::unimplemented("query_box_service_peer_cred"))
    }

    async fn set_relay(&self, _request: Request<SetRelayRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("set_relay"))
    }

    async fn dump_peers(&self, _request: Request<()>) -> Result<Response<PeersInfo>, Status> {
        Err(Status::unimplemented("dump_peers"))
    }
}
