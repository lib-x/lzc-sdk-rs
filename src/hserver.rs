use std::fmt;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use http::Uri;
use hyper_util::rt::TokioIo;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint};
use tower::service_fn;
use url::{Host, Url};

use crate::proto::sys::h_portal_sys_client::HPortalSysClient as GeneratedHPortalSysClient;
use crate::proto::sys::{RemoteSocksRequest, remote_socks_request};
use crate::{Error, PORTAL_SOCKET_PATH};

/// Physical network location where `HPortalSys` should expose `RemoteSocks`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemoteLocation {
    /// Use the `HServer`'s local physical network stack.
    Local,
    /// Use the physical network stack of a connected `HClient` peer.
    Remote {
        /// `HClient` peer identifier.
        target: String,
    },
}

impl From<&str> for RemoteLocation {
    fn from(target: &str) -> Self {
        if target.is_empty() {
            Self::Local
        } else {
            Self::Remote {
                target: target.to_owned(),
            }
        }
    }
}

impl From<String> for RemoteLocation {
    fn from(target: String) -> Self {
        if target.is_empty() {
            Self::Local
        } else {
            Self::Remote { target }
        }
    }
}

/// Validated `RemoteSocks` server endpoint returned by `HPortalSys`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteSocksEndpoint {
    host: String,
    port: u16,
    authority: String,
    resolves_hostname_remotely: bool,
}

impl RemoteSocksEndpoint {
    /// Endpoint host without IPv6 brackets.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Endpoint TCP port.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Endpoint authority suitable for `TcpStream::connect`.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Whether the endpoint used the `socks5h` scheme.
    #[must_use]
    pub const fn resolves_hostname_remotely(&self) -> bool {
        self.resolves_hostname_remotely
    }
}

impl FromStr for RemoteSocksEndpoint {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value).map_err(|_| Error::InvalidRemoteSocksEndpoint)?;
        let resolves_hostname_remotely = match url.scheme() {
            "socks5" => false,
            "socks5h" => true,
            _ => return Err(Error::InvalidRemoteSocksEndpoint),
        };
        if !url.username().is_empty()
            || url.password().is_some()
            || !matches!(url.path(), "" | "/")
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(Error::InvalidRemoteSocksEndpoint);
        }
        let (host, ipv6) = match url.host() {
            Some(Host::Domain(host)) if !host.is_empty() => (host.to_owned(), false),
            Some(Host::Ipv4(host)) => (host.to_string(), false),
            Some(Host::Ipv6(host)) => (host.to_string(), true),
            _ => return Err(Error::InvalidRemoteSocksEndpoint),
        };
        let port = url.port().ok_or(Error::InvalidRemoteSocksEndpoint)?;
        let authority = if ipv6 {
            format!("[{host}]:{port}")
        } else {
            format!("{host}:{port}")
        };
        Ok(Self {
            host,
            port,
            authority,
            resolves_hostname_remotely,
        })
    }
}

impl fmt::Display for RemoteSocksEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scheme = if self.resolves_hostname_remotely {
            "socks5h"
        } else {
            "socks5"
        };
        write!(formatter, "{scheme}://{}", self.authority)
    }
}

/// Client for the `HPortalSys` service mounted in `LazyCat` applications.
#[derive(Clone, Debug)]
pub struct HServerClient {
    channel: Channel,
}

impl HServerClient {
    /// Connect to the application `HPortalSys` Unix socket.
    ///
    /// # Errors
    ///
    /// Returns an error when the mounted portal socket cannot be reached.
    pub async fn connect() -> Result<Self, Error> {
        Self::connect_at(Path::new(PORTAL_SOCKET_PATH)).await
    }

    /// Construct a wrapper around an existing tonic channel.
    #[must_use]
    pub const fn from_channel(channel: Channel) -> Self {
        Self { channel }
    }

    /// Create a generated client exposing every `HPortalSys` RPC.
    #[must_use]
    pub fn client(&self) -> GeneratedHPortalSysClient<Channel> {
        GeneratedHPortalSysClient::new(self.channel.clone())
    }

    /// Resolve a validated `RemoteSocks` endpoint for a local or remote target.
    ///
    /// Empty string targets select [`RemoteLocation::Local`]; non-empty strings
    /// select [`RemoteLocation::Remote`], matching the official Go SDK.
    ///
    /// # Errors
    ///
    /// Returns an error when `HPortalSys` rejects the request or returns an
    /// unsupported or malformed SOCKS URL.
    pub async fn remote_socks_endpoint(
        &self,
        location: impl Into<RemoteLocation>,
    ) -> Result<RemoteSocksEndpoint, Error> {
        let (location_type, target) = match location.into() {
            RemoteLocation::Local => (remote_socks_request::LocationType::Local, String::new()),
            RemoteLocation::Remote { target } => {
                (remote_socks_request::LocationType::Remote, target)
            }
        };
        let reply = self
            .client()
            .remote_socks(RemoteSocksRequest {
                location_type: location_type.into(),
                target,
            })
            .await?
            .into_inner();
        reply.server_url.parse()
    }

    async fn connect_at(socket_path: &Path) -> Result<Self, Error> {
        let endpoint = Endpoint::from_static("http://lazycat-hportal");
        let socket_path = Arc::new(socket_path.to_owned());
        let connector = service_fn(move |_: Uri| {
            let socket_path = Arc::clone(&socket_path);
            async move {
                UnixStream::connect(socket_path.as_ref())
                    .await
                    .map(TokioIo::new)
            }
        });
        let channel = endpoint.connect_with_connector(connector).await?;
        Ok(Self { channel })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tempfile::tempdir;
    use tokio::net::UnixListener;
    use tokio_stream::wrappers::UnixListenerStream;
    use tonic::transport::Server;
    use tonic::{Request, Response, Status};

    use crate::proto::sys::VersionInfo;
    use crate::proto::sys::version_info_service_client::VersionInfoServiceClient;
    use crate::proto::sys::version_info_service_server::{
        VersionInfoService, VersionInfoServiceServer,
    };

    use super::RemoteSocksEndpoint;

    #[tokio::test]
    async fn connects_to_an_injected_portal_unix_socket() {
        let directory = tempdir().expect("tempdir");
        let socket_path = directory.path().join("portal.socket");
        let listener = UnixListener::bind(&socket_path).expect("bind portal socket");
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(VersionInfoServiceServer::new(PlainVersionService))
                .serve_with_incoming(UnixListenerStream::new(listener))
                .await
                .expect("serve tonic fixture");
        });

        let portal = super::HServerClient::connect_at(&socket_path)
            .await
            .expect("connect portal socket");
        let response = VersionInfoServiceClient::new(portal.channel)
            .get(())
            .await
            .expect("query fixture")
            .into_inner();
        assert_eq!(response.version, "portal-uds");
        server.abort();
    }

    #[test]
    fn parses_supported_remote_socks_urls() {
        let domain =
            RemoteSocksEndpoint::from_str("socks5://proxy.lzcapp:1080").expect("domain endpoint");
        assert_eq!(domain.host(), "proxy.lzcapp");
        assert_eq!(domain.port(), 1080);
        assert_eq!(domain.to_string(), "socks5://proxy.lzcapp:1080");

        let ipv6 =
            RemoteSocksEndpoint::from_str("socks5h://[2001:db8::1]:2080").expect("IPv6 endpoint");
        assert_eq!(ipv6.host(), "2001:db8::1");
        assert_eq!(ipv6.authority(), "[2001:db8::1]:2080");
        assert!(ipv6.resolves_hostname_remotely());
    }

    #[test]
    fn rejects_ambiguous_remote_socks_urls() {
        for value in [
            "http://127.0.0.1:1080",
            "socks5://127.0.0.1",
            "socks5://user@127.0.0.1:1080",
            "socks5://127.0.0.1:1080/path",
            "socks5://127.0.0.1:1080?query=yes",
        ] {
            assert!(RemoteSocksEndpoint::from_str(value).is_err(), "{value}");
        }
    }

    #[derive(Debug)]
    struct PlainVersionService;

    #[tonic::async_trait]
    impl VersionInfoService for PlainVersionService {
        async fn get(&self, _request: Request<()>) -> Result<Response<VersionInfo>, Status> {
            Ok(Response::new(VersionInfo {
                version: "portal-uds".to_owned(),
            }))
        }
    }
}
