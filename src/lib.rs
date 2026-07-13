#![forbid(unsafe_code)]
#![doc = "Rust SDK for `LazyCat` application and device APIs."]

mod connection;
mod credentials;
mod error;
mod metadata;
mod peer;

#[allow(clippy::all, clippy::pedantic, non_camel_case_types)]
mod generated {
    include!("gen/mod.rs");
}

pub mod proto {
    pub use crate::generated::cloud::lazycat::apis::{common, localdevice, sys};
    pub use crate::generated::io::containerd;
    pub use crate::generated::lzc::dlna;
}

pub use connection::{PORTAL_SOCKET_PATH, RUNTIME_SOCKET_PATH, connect_api, connect_api_with};
pub use credentials::{APP_CERT_PATH, APP_KEY_PATH, CA_PATH, ClientCredentials, CredentialPaths};
pub use error::Error;
pub use metadata::{REAL_UID_METADATA_KEY, with_real_uid};
pub use peer::{Application, peer_application};
