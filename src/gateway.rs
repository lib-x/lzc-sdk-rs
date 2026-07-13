use std::fmt;
use std::sync::Arc;

use tonic::transport::Channel;

use crate::proto::common::box_service_client::BoxServiceClient;
use crate::proto::common::end_device_service_client::EndDeviceServiceClient;
use crate::proto::common::file_handler_client::FileHandlerClient;
use crate::proto::common::file_transfer_service_client::FileTransferServiceClient;
use crate::proto::common::iscsi_service_client::IscsiServiceClient;
use crate::proto::common::message_service_client::MessageServiceClient;
use crate::proto::common::peripheral_device_service_client::PeripheralDeviceServiceClient;
use crate::proto::common::permission_manager_client::PermissionManagerClient as RuntimePermissionManagerClient;
use crate::proto::common::user_manager_client::UserManagerClient;
use crate::proto::localdevice::contacts_manager_client::ContactsManagerClient;
use crate::proto::localdevice::device_service_client::DeviceServiceClient;
use crate::proto::localdevice::dialog_manager_client::DialogManagerClient;
use crate::proto::localdevice::network_manager_client::NetworkManagerClient;
use crate::proto::localdevice::permission_manager_client::PermissionManagerClient;
use crate::proto::localdevice::photo_library_client::PhotoLibraryClient;
use crate::proto::localdevice::remote_control_client::RemoteControlClient;
use crate::proto::localdevice::rim_client::RimClient;
use crate::proto::localdevice::user_config_client::UserConfigClient;
use crate::proto::sys::access_controler_service_client::AccessControlerServiceClient;
use crate::proto::sys::btrfs_util_client::BtrfsUtilClient;
use crate::proto::sys::dir_monitor_client::DirMonitorClient;
use crate::proto::sys::package_manager_client::PackageManagerClient;
use crate::proto::sys::tv_os_client::TvOsClient;
use crate::proto::sys::version_info_service_client::VersionInfoServiceClient;
use crate::{
    AuthToken, AuthenticatedService, ClientCredentials, CredentialPaths, DeviceProxyStatus, Error,
    TokenProvider, connect_api_with,
};

type DeviceServiceTransport = AuthenticatedService<Channel>;

/// Complete client composition for the `LazyCat` runtime API gateway.
#[derive(Clone)]
pub struct ApiGateway {
    channel: Channel,
    credentials: ClientCredentials,
}

impl ApiGateway {
    /// Load runtime credentials and connect the API gateway.
    ///
    /// # Errors
    ///
    /// Returns an error when credentials cannot be loaded or the runtime API
    /// cannot be reached.
    pub async fn connect() -> Result<Self, Error> {
        let credentials = ClientCredentials::load(CredentialPaths::runtime()).await?;
        Self::connect_with(credentials).await
    }

    /// Connect the API gateway with preloaded credentials.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime API cannot be reached.
    pub async fn connect_with(credentials: ClientCredentials) -> Result<Self, Error> {
        let channel = connect_api_with(credentials.clone()).await?;
        Ok(Self::from_channel(channel, credentials))
    }

    /// Compose a gateway around an existing runtime channel.
    #[must_use]
    pub fn from_channel(channel: Channel, credentials: ClientCredentials) -> Self {
        Self {
            channel,
            credentials,
        }
    }

    /// Connect an authenticated proxy for a remote `LazyCat` device API.
    ///
    /// # Errors
    ///
    /// Returns an error when the device URL or mTLS connection is invalid.
    pub async fn device_proxy(&self, api_url: &str) -> Result<DeviceProxy, Error> {
        DeviceProxy::connect(api_url, self.credentials.clone()).await
    }

    #[must_use]
    pub fn box_service(&self) -> BoxServiceClient<Channel> {
        BoxServiceClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn users(&self) -> UserManagerClient<Channel> {
        UserManagerClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn devices(&self) -> EndDeviceServiceClient<Channel> {
        EndDeviceServiceClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn permissions(&self) -> RuntimePermissionManagerClient<Channel> {
        RuntimePermissionManagerClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn peripheral_device(&self) -> PeripheralDeviceServiceClient<Channel> {
        PeripheralDeviceServiceClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn iscsi(&self) -> IscsiServiceClient<Channel> {
        IscsiServiceClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn file_transfer(&self) -> FileTransferServiceClient<Channel> {
        FileTransferServiceClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn package_manager(&self) -> PackageManagerClient<Channel> {
        PackageManagerClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn access_controller(&self) -> AccessControlerServiceClient<Channel> {
        AccessControlerServiceClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn btrfs(&self) -> BtrfsUtilClient<Channel> {
        BtrfsUtilClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn dir_monitor(&self) -> DirMonitorClient<Channel> {
        DirMonitorClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn message(&self) -> MessageServiceClient<Channel> {
        MessageServiceClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn tv_os(&self) -> TvOsClient<Channel> {
        TvOsClient::new(self.channel.clone())
    }

    #[must_use]
    pub fn version(&self) -> VersionInfoServiceClient<Channel> {
        VersionInfoServiceClient::new(self.channel.clone())
    }
}

impl fmt::Debug for ApiGateway {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApiGateway")
            .field("credentials", &self.credentials)
            .finish_non_exhaustive()
    }
}

/// Complete authenticated client composition for a `LazyCat` device API.
#[derive(Clone)]
pub struct DeviceProxy {
    provider: TokenProvider,
    service: DeviceServiceTransport,
    status: Arc<DeviceProxyStatus>,
}

impl DeviceProxy {
    /// Connect all device clients over one authenticated mTLS channel.
    ///
    /// # Errors
    ///
    /// Returns an error when the device URL or mTLS connection is invalid.
    pub async fn connect(api_url: &str, credentials: ClientCredentials) -> Result<Self, Error> {
        let provider = TokenProvider::connect(api_url, credentials).await?;
        let service = provider.authenticated_service();
        let status = Arc::new(DeviceProxyStatus::new(service.clone()));
        Ok(Self {
            provider,
            service,
            status,
        })
    }

    /// Return the current cached token, refreshing it when necessary.
    ///
    /// # Errors
    ///
    /// Returns an error when token signing or acquisition fails.
    pub async fn get_auth_token(&self) -> Result<Arc<AuthToken>, Error> {
        self.provider.token().await
    }

    #[must_use]
    pub fn config(&self) -> UserConfigClient<DeviceServiceTransport> {
        UserConfigClient::new(self.service.clone())
    }

    #[must_use]
    pub fn device(&self) -> DeviceServiceClient<DeviceServiceTransport> {
        DeviceServiceClient::new(self.service.clone())
    }

    #[must_use]
    pub fn dialog(&self) -> DialogManagerClient<DeviceServiceTransport> {
        DialogManagerClient::new(self.service.clone())
    }

    #[must_use]
    pub fn photo_library(&self) -> PhotoLibraryClient<DeviceServiceTransport> {
        PhotoLibraryClient::new(self.service.clone())
    }

    #[must_use]
    pub fn network(&self) -> NetworkManagerClient<DeviceServiceTransport> {
        NetworkManagerClient::new(self.service.clone())
    }

    #[must_use]
    pub fn permission(&self) -> PermissionManagerClient<DeviceServiceTransport> {
        PermissionManagerClient::new(self.service.clone())
    }

    #[must_use]
    pub fn file_handler(&self) -> FileHandlerClient<DeviceServiceTransport> {
        FileHandlerClient::new(self.service.clone())
    }

    #[must_use]
    pub fn rim(&self) -> RimClient<DeviceServiceTransport> {
        RimClient::new(self.service.clone())
    }

    #[must_use]
    pub fn remote_control(&self) -> RemoteControlClient<DeviceServiceTransport> {
        RemoteControlClient::new(self.service.clone())
    }

    #[must_use]
    pub fn contacts(&self) -> ContactsManagerClient<DeviceServiceTransport> {
        ContactsManagerClient::new(self.service.clone())
    }

    #[must_use]
    pub fn status(&self) -> DeviceProxyStatus {
        self.status.as_ref().clone()
    }
}

impl fmt::Debug for DeviceProxy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceProxy")
            .field("provider", &self.provider)
            .finish_non_exhaustive()
    }
}
