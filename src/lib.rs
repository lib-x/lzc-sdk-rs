#![forbid(unsafe_code)]
#![doc = "Rust SDK for `LazyCat` application and device APIs."]

mod auth;
mod connection;
mod credentials;
mod device_transport;
mod error;
mod gateway;
mod hserver;
mod metadata;
mod peer;
#[cfg(feature = "remotesocks")]
mod remotesocks;
mod service_address;
mod service_status;

#[allow(clippy::all, clippy::pedantic, non_camel_case_types)]
mod generated {
    include!("gen/mod.rs");
}

pub mod proto {
    pub use crate::generated::cloud::lazycat::apis::{common, localdevice, sys};
    pub use crate::generated::io::containerd;
    pub use crate::generated::lzc::dlna;
}

pub use auth::{AuthToken, TokenProvider, request_auth_token};
pub use connection::{PORTAL_SOCKET_PATH, RUNTIME_SOCKET_PATH, connect_api, connect_api_with};
pub use credentials::{APP_CERT_PATH, APP_KEY_PATH, CA_PATH, ClientCredentials, CredentialPaths};
pub use device_transport::AuthenticatedService;
pub use error::Error;
pub use gateway::{ApiGateway, DeviceProxy};
pub use hserver::{HServerClient, RemoteLocation, RemoteSocksEndpoint};
pub use metadata::{REAL_UID_METADATA_KEY, with_real_uid};
pub use peer::{Application, peer_application};
#[cfg(feature = "remotesocks")]
pub use remotesocks::{
    RemoteNetstack, RemoteTcpListener, RemoteTcpStream, RemoteUdpSocket, SocksAddress,
};
pub use service_address::query_service_address;
pub use service_status::{
    DeviceProxyStatus, ServiceState, ServiceStatus, ServiceStatusQuerier, ServiceStatusRegistry,
};
