use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::{Error, HServerClient, RemoteLocation, RemoteSocksEndpoint};

use super::SocksAddress;

pub(crate) const CONNECT_COMMAND: u8 = 0x01;
pub(crate) const BIND_COMMAND: u8 = 0x02;

const SOCKS_VERSION: u8 = 0x05;
const NO_AUTHENTICATION: u8 = 0x00;
const NO_ACCEPTABLE_AUTHENTICATION: u8 = 0xff;
const DEFAULT_DIAL_TIMEOUT: Duration = Duration::from_secs(60);

type ResolverFuture = Pin<Box<dyn Future<Output = Result<RemoteSocksEndpoint, Error>> + Send>>;
type EndpointResolver = dyn Fn() -> ResolverFuture + Send + Sync;

struct Shared {
    resolver: Arc<EndpointResolver>,
    cached_endpoint: Mutex<Option<RemoteSocksEndpoint>>,
}

/// Async `LazyCat` `RemoteSocks` network stack.
#[derive(Clone)]
pub struct RemoteNetstack {
    shared: Arc<Shared>,
    dial_timeout: Duration,
}

impl fmt::Debug for RemoteNetstack {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RemoteNetstack")
            .field("dial_timeout", &self.dial_timeout)
            .finish_non_exhaustive()
    }
}

impl RemoteNetstack {
    /// Resolve proxy endpoints through `HPortalSys` for the requested location.
    #[must_use]
    pub fn new(hserver: HServerClient, location: impl Into<RemoteLocation>) -> Self {
        let location = location.into();
        Self::with_endpoint_resolver(move || {
            let hserver = hserver.clone();
            let location = location.clone();
            async move { hserver.remote_socks_endpoint(location).await }
        })
    }

    /// Use a fixed `RemoteSocks` proxy endpoint.
    #[must_use]
    pub fn fixed(endpoint: RemoteSocksEndpoint) -> Self {
        Self::with_endpoint_resolver(move || {
            let endpoint = endpoint.clone();
            async move { Ok(endpoint) }
        })
    }

    /// Use an asynchronous endpoint resolver compatible with the Go SDK's
    /// dynamic address factory.
    #[must_use]
    pub fn with_endpoint_resolver<F, Fut>(resolver: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<RemoteSocksEndpoint, Error>> + Send + 'static,
    {
        let resolver = Arc::new(move || Box::pin(resolver()) as ResolverFuture);
        Self {
            shared: Arc::new(Shared {
                resolver,
                cached_endpoint: Mutex::new(None),
            }),
            dial_timeout: DEFAULT_DIAL_TIMEOUT,
        }
    }

    /// Override the default 60-second proxy connection deadline.
    #[must_use]
    pub const fn with_dial_timeout(mut self, dial_timeout: Duration) -> Self {
        self.dial_timeout = dial_timeout;
        self
    }

    pub(crate) async fn perform_command(
        &self,
        command: u8,
        address: &SocksAddress,
    ) -> Result<(SocksAddress, TcpStream), Error> {
        match timeout(
            self.dial_timeout,
            self.perform_command_with_deadline(command, address),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => Err(Error::RemoteSocksTimeout),
        }
    }

    async fn perform_command_with_deadline(
        &self,
        command: u8,
        address: &SocksAddress,
    ) -> Result<(SocksAddress, TcpStream), Error> {
        let mut stream = self.connect_proxy().await?;
        negotiate_no_authentication(&mut stream).await?;
        stream.write_all(&[SOCKS_VERSION, command, 0]).await?;
        address.write_to(&mut stream).await?;
        let bound_address = read_reply(&mut stream).await?;
        Ok((bound_address, stream))
    }

    async fn connect_proxy(&self) -> Result<TcpStream, Error> {
        let endpoint = self.proxy_endpoint(true).await?;
        match self.dial_endpoint(&endpoint).await {
            Ok(stream) => Ok(stream),
            Err(first_error) => {
                let Ok(refreshed) = self.proxy_endpoint(false).await else {
                    return Err(first_error);
                };
                if refreshed == endpoint {
                    return Err(first_error);
                }
                self.dial_endpoint(&refreshed).await
            }
        }
    }

    async fn dial_endpoint(&self, endpoint: &RemoteSocksEndpoint) -> Result<TcpStream, Error> {
        TcpStream::connect(endpoint.authority())
            .await
            .map_err(Error::from)
    }

    async fn proxy_endpoint(&self, with_cache: bool) -> Result<RemoteSocksEndpoint, Error> {
        let mut cached = self.shared.cached_endpoint.lock().await;
        if with_cache && let Some(endpoint) = cached.as_ref() {
            return Ok(endpoint.clone());
        }
        let endpoint = (self.shared.resolver)().await?;
        *cached = Some(endpoint.clone());
        Ok(endpoint)
    }
}

async fn negotiate_no_authentication(stream: &mut TcpStream) -> Result<(), Error> {
    stream
        .write_all(&[SOCKS_VERSION, 1, NO_AUTHENTICATION])
        .await?;
    let mut response = [0_u8; 2];
    stream.read_exact(&mut response).await?;
    if response[0] != SOCKS_VERSION {
        return Err(Error::UnexpectedSocksVersion {
            version: response[0],
        });
    }
    match response[1] {
        NO_AUTHENTICATION => Ok(()),
        NO_ACCEPTABLE_AUTHENTICATION => Err(Error::SocksNoAcceptableAuthentication),
        method => Err(Error::UnsupportedSocksAuthentication { method }),
    }
}

pub(crate) async fn read_reply(stream: &mut TcpStream) -> Result<SocksAddress, Error> {
    let mut header = [0_u8; 3];
    stream.read_exact(&mut header).await?;
    if header[0] != SOCKS_VERSION {
        return Err(Error::UnexpectedSocksVersion { version: header[0] });
    }
    if header[2] != 0 {
        return Err(Error::InvalidSocksReservedByte {
            reserved: header[2],
        });
    }
    if header[1] != 0 {
        return Err(Error::SocksReply { code: header[1] });
    }
    SocksAddress::read_from(stream).await
}
