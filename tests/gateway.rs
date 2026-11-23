mod fixtures;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};

use fixtures::{AppKeyKind, TestPki};
use lzc_sdk::proto::common::box_service_client::BoxServiceClient;
use lzc_sdk::proto::common::end_device_service_client::EndDeviceServiceClient;
use lzc_sdk::proto::common::file_handler_client::FileHandlerClient;
use lzc_sdk::proto::common::file_handler_server;
use lzc_sdk::proto::common::file_transfer_service_client::FileTransferServiceClient;
use lzc_sdk::proto::common::h_client_service_client::HClientServiceClient;
use lzc_sdk::proto::common::iscsi_service_client::IscsiServiceClient;
use lzc_sdk::proto::common::message_service_client::MessageServiceClient;
use lzc_sdk::proto::common::peripheral_device_service_client::PeripheralDeviceServiceClient;
use lzc_sdk::proto::common::permission_manager_client::PermissionManagerClient as RuntimePermissionManagerClient;
use lzc_sdk::proto::common::user_manager_client::UserManagerClient;
use lzc_sdk::proto::dlna::remote_media_player_service_client::RemoteMediaPlayerServiceClient;
use lzc_sdk::proto::localdevice::ble_manager_client::BleManagerClient;
use lzc_sdk::proto::localdevice::client_client::ClientClient;
use lzc_sdk::proto::localdevice::cloud_driver_client::CloudDriverClient;
use lzc_sdk::proto::localdevice::contacts_manager_client::ContactsManagerClient;
use lzc_sdk::proto::localdevice::device_service_client::DeviceServiceClient;
use lzc_sdk::proto::localdevice::device_service_server::{DeviceService, DeviceServiceServer};
use lzc_sdk::proto::localdevice::dialog_manager_client::DialogManagerClient;
use lzc_sdk::proto::localdevice::local_launch_service_client::LocalLaunchServiceClient;
use lzc_sdk::proto::localdevice::network_manager_client::NetworkManagerClient;
use lzc_sdk::proto::localdevice::notification_service_client::NotificationServiceClient;
use lzc_sdk::proto::localdevice::notification_service_server::{
    NotificationService, NotificationServiceServer,
};
use lzc_sdk::proto::localdevice::oneway_sync_client::OnewaySyncClient;
use lzc_sdk::proto::localdevice::permission_manager_client::PermissionManagerClient;
use lzc_sdk::proto::localdevice::permission_manager_server::{
    PermissionManager, PermissionManagerServer,
};
use lzc_sdk::proto::localdevice::photo_library_client::PhotoLibraryClient;
use lzc_sdk::proto::localdevice::photo_library_server;
use lzc_sdk::proto::localdevice::remote_control_client::RemoteControlClient;
use lzc_sdk::proto::localdevice::rim_client::RimClient;
use lzc_sdk::proto::localdevice::user_config_client::UserConfigClient;
use lzc_sdk::proto::localdevice::{
    DeviceInfo, ListPermissionsReply, LocalServiceState, LocalServiceStatus, NotifyRequest,
    NotifyResponse, PermissionReply, PermissionRequest, QueryServiceStatusReply,
    QueryServiceStatusRequest, RequestAuthTokenRequest, RequestAuthTokenResponse,
};
use lzc_sdk::proto::sys::access_controler_service_client::AccessControlerServiceClient;
use lzc_sdk::proto::sys::btrfs_util_client::BtrfsUtilClient;
use lzc_sdk::proto::sys::dev_opt_service_client::DevOptServiceClient;
use lzc_sdk::proto::sys::dir_monitor_client::DirMonitorClient;
use lzc_sdk::proto::sys::network_manager_client::NetworkManagerClient as BoxNetworkManagerClient;
use lzc_sdk::proto::sys::package_manager_client::PackageManagerClient;
use lzc_sdk::proto::sys::tv_os_client::TvOsClient;
use lzc_sdk::proto::sys::version_info_service_client::VersionInfoServiceClient;
use lzc_sdk::{
    ApiGateway, AuthenticatedService, ClientCredentials, DeviceProxy, Error, ServiceState,
    peer_application,
};
use tempfile::tempdir;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Channel, Server};
use tonic::{Request, Response, Status};

#[test]
fn gateway_accessors_cover_the_official_client_sets() {
    fn runtime(gateway: &ApiGateway) {
        let _: BoxServiceClient<Channel> = gateway.box_service();
        let _: UserManagerClient<Channel> = gateway.users();
        let _: EndDeviceServiceClient<Channel> = gateway.devices();
        let _: HClientServiceClient<Channel> = gateway.hclients();
        let _: RuntimePermissionManagerClient<Channel> = gateway.permissions();
        let _: PeripheralDeviceServiceClient<Channel> = gateway.peripheral_device();
        let _: IscsiServiceClient<Channel> = gateway.iscsi();
        let _: FileTransferServiceClient<Channel> = gateway.file_transfer();
        let _: PackageManagerClient<Channel> = gateway.package_manager();
        let _: AccessControlerServiceClient<Channel> = gateway.access_controller();
        let _: BtrfsUtilClient<Channel> = gateway.btrfs();
        let _: DirMonitorClient<Channel> = gateway.dir_monitor();
        let _: BoxNetworkManagerClient<Channel> = gateway.network();
        let _: RemoteMediaPlayerServiceClient<Channel> = gateway.remote_media_player();
        let _: DevOptServiceClient<Channel> = gateway.device_options();
        let _: MessageServiceClient<Channel> = gateway.message();
        let _: TvOsClient<Channel> = gateway.tv_os();
        let _: VersionInfoServiceClient<Channel> = gateway.version();
    }

    fn device(proxy: &DeviceProxy) {
        type DeviceServiceTransport = AuthenticatedService<Channel>;
        let _: UserConfigClient<DeviceServiceTransport> = proxy.config();
        let _: DeviceServiceClient<DeviceServiceTransport> = proxy.device();
        let _: DialogManagerClient<DeviceServiceTransport> = proxy.dialog();
        let _: PhotoLibraryClient<DeviceServiceTransport> = proxy.photo_library();
        let _: NetworkManagerClient<DeviceServiceTransport> = proxy.network();
        let _: PermissionManagerClient<DeviceServiceTransport> = proxy.permission();
        let _: FileHandlerClient<DeviceServiceTransport> = proxy.file_handler();
        let _: RimClient<DeviceServiceTransport> = proxy.rim();
        let _: RemoteControlClient<DeviceServiceTransport> = proxy.remote_control();
        let _: ContactsManagerClient<DeviceServiceTransport> = proxy.contacts();
        let _: NotificationServiceClient<DeviceServiceTransport> = proxy.notification();
        let _: LocalLaunchServiceClient<DeviceServiceTransport> = proxy.local_launch();
        let _: ClientClient<DeviceServiceTransport> = proxy.client();
        let _: CloudDriverClient<DeviceServiceTransport> = proxy.cloud_driver();
        let _: OnewaySyncClient<DeviceServiceTransport> = proxy.oneway_sync();
        let _: BleManagerClient<DeviceServiceTransport> = proxy.ble();
    }

    let _ = runtime as fn(&ApiGateway);
    let _ = device as fn(&DeviceProxy);
}

#[tokio::test]
async fn notification_client_sends_authenticated_payload_and_preserves_rejection() {
    let pki = TestPki::new(AppKeyKind::Ed25519);
    let fixture = DeviceFixture::default();
    let (url, server) = start_server(&pki, fixture.clone()).await;
    let directory = tempdir().expect("tempdir");
    let credentials = ClientCredentials::load(pki.write_credentials(directory.path()))
        .await
        .expect("load credentials");
    let proxy = DeviceProxy::connect(&url, credentials)
        .await
        .expect("connect device proxy");
    let notification = NotifyRequest {
        title: "雁字渠道测试".to_owned(),
        body: "这是一条测试通知。".to_owned(),
        deeplink_url: None,
    };

    proxy
        .notification()
        .notify(notification.clone())
        .await
        .expect("notify");
    assert_eq!(
        fixture.notifications.lock().await.as_slice(),
        std::slice::from_ref(&notification)
    );

    fixture.reject_notifications.store(true, Ordering::SeqCst);
    let status = proxy
        .notification()
        .notify(notification)
        .await
        .expect_err("notification rejected");
    assert_eq!(status.code(), tonic::Code::PermissionDenied);
    assert_eq!(fixture.notifications.lock().await.len(), 1);
    server.abort();
}

#[tokio::test]
async fn service_status_maps_states_deduplicates_and_preserves_reasons() {
    let pki = TestPki::new(AppKeyKind::Ed25519);
    let status = DeviceFixture::default();
    let (url, server) = start_server(&pki, status.clone()).await;
    let directory = tempdir().expect("tempdir");
    let credentials = ClientCredentials::load(pki.write_credentials(directory.path()))
        .await
        .expect("load credentials");
    let proxy = DeviceProxy::connect(&url, credentials)
        .await
        .expect("connect device proxy");

    let photo = proxy
        .status()
        .photo_library()
        .query()
        .await
        .expect("photo status");
    assert_eq!(photo.state, ServiceState::Unavailable);
    assert_eq!(photo.reason, "main client is not reachable");

    let statuses = proxy
        .status()
        .services()
        .query_many([
            photo_library_server::SERVICE_NAME,
            file_handler_server::SERVICE_NAME,
            photo_library_server::SERVICE_NAME,
        ])
        .await
        .expect("query many statuses");
    assert_eq!(
        statuses[file_handler_server::SERVICE_NAME].state,
        ServiceState::Available
    );
    assert_eq!(
        proxy
            .status()
            .services()
            .query("missing.service")
            .await
            .expect("missing service state")
            .state,
        ServiceState::Unknown
    );
    let requests = status.requests.lock().await;
    assert_eq!(
        requests[1].len(),
        2,
        "duplicate service names must be removed"
    );
    server.abort();
}

#[tokio::test]
async fn unimplemented_service_status_has_typed_error() {
    let pki = TestPki::new(AppKeyKind::Ed25519);
    let status = DeviceFixture::default();
    status.unimplemented.store(true, Ordering::SeqCst);
    let (url, server) = start_server(&pki, status).await;
    let directory = tempdir().expect("tempdir");
    let credentials = ClientCredentials::load(pki.write_credentials(directory.path()))
        .await
        .expect("load credentials");
    let proxy = DeviceProxy::connect(&url, credentials)
        .await
        .expect("connect device proxy");

    let error = proxy
        .status()
        .services()
        .query("photo")
        .await
        .expect_err("fixture returns unimplemented");
    assert!(matches!(error, Error::ServiceStatusUnsupported));
    server.abort();
}

#[derive(Clone, Default)]
struct DeviceFixture {
    unimplemented: Arc<AtomicBool>,
    reject_notifications: Arc<AtomicBool>,
    requests: Arc<Mutex<Vec<Vec<String>>>>,
    notifications: Arc<Mutex<Vec<NotifyRequest>>>,
}

#[tonic::async_trait]
impl NotificationService for DeviceFixture {
    async fn notify(
        &self,
        request: Request<NotifyRequest>,
    ) -> Result<Response<NotifyResponse>, Status> {
        require_token(&request)?;
        if self.reject_notifications.load(Ordering::SeqCst) {
            return Err(Status::permission_denied("notifications disabled"));
        }
        self.notifications.lock().await.push(request.into_inner());
        Ok(Response::new(NotifyResponse {}))
    }
}

#[tonic::async_trait]
impl DeviceService for DeviceFixture {
    async fn query(&self, _request: Request<()>) -> Result<Response<DeviceInfo>, Status> {
        Ok(Response::new(DeviceInfo::default()))
    }

    async fn query_service_status(
        &self,
        request: Request<QueryServiceStatusRequest>,
    ) -> Result<Response<QueryServiceStatusReply>, Status> {
        require_token(&request)?;
        if self.unimplemented.load(Ordering::SeqCst) {
            return Err(Status::unimplemented("status API unavailable"));
        }
        self.requests
            .lock()
            .await
            .push(request.get_ref().service_name.clone());
        Ok(Response::new(QueryServiceStatusReply {
            services: HashMap::from([
                (
                    photo_library_server::SERVICE_NAME.to_owned(),
                    LocalServiceStatus {
                        state: LocalServiceState::Unavailable.into(),
                        reason: "main client is not reachable".to_owned(),
                    },
                ),
                (
                    file_handler_server::SERVICE_NAME.to_owned(),
                    LocalServiceStatus {
                        state: LocalServiceState::Available.into(),
                        reason: String::new(),
                    },
                ),
                (
                    "unknown.service".to_owned(),
                    LocalServiceStatus {
                        state: 99,
                        reason: String::new(),
                    },
                ),
            ]),
        }))
    }
}

#[derive(Clone)]
struct AuthFixture {
    pki: TestPki,
}

#[tonic::async_trait]
impl PermissionManager for AuthFixture {
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
        _request: Request<()>,
    ) -> Result<Response<ListPermissionsReply>, Status> {
        Ok(Response::new(ListPermissionsReply::default()))
    }

    async fn request_auth_token(
        &self,
        request: Request<RequestAuthTokenRequest>,
    ) -> Result<Response<RequestAuthTokenResponse>, Status> {
        peer_application(&request).map_err(|error| Status::unauthenticated(error.to_string()))?;
        if !self.pki.verifies(request.get_ref()) {
            return Err(Status::permission_denied("invalid signature"));
        }
        Ok(Response::new(RequestAuthTokenResponse {
            token: "gateway-token".to_owned(),
            deadline: Some(prost_types::Timestamp::from(
                SystemTime::now() + Duration::from_secs(300),
            )),
        }))
    }
}

async fn start_server(
    pki: &TestPki,
    device: DeviceFixture,
) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind gateway fixture");
    let address = listener.local_addr().expect("gateway fixture address");
    let tls = pki.server_tls_config();
    let auth = AuthFixture { pki: pki.clone() };
    let server = tokio::spawn(async move {
        Server::builder()
            .tls_config(tls)
            .expect("gateway fixture TLS")
            .add_service(PermissionManagerServer::new(auth))
            .add_service(DeviceServiceServer::new(device.clone()))
            .add_service(NotificationServiceServer::new(device))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .expect("serve gateway fixture");
    });
    (format!("https://{address}"), server)
}

fn require_token<T>(request: &Request<T>) -> Result<(), Status> {
    match request.metadata().get("lzc_dapi_auth_token") {
        Some(token) if token == "gateway-token" => Ok(()),
        _ => Err(Status::unauthenticated("missing device token")),
    }
}
