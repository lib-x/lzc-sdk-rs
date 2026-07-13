use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::Error;

use super::client::{BIND_COMMAND, CONNECT_COMMAND, read_reply};
use super::{RemoteNetstack, SocksAddress};

/// TCP stream transported through a `LazyCat` `RemoteSocks` proxy.
#[derive(Debug)]
pub struct RemoteTcpStream {
    inner: TcpStream,
    local_addr: SocksAddress,
    peer_addr: SocksAddress,
}

/// TCP listener implemented with the two-stage SOCKS BIND command.
#[derive(Debug)]
pub struct RemoteTcpListener {
    netstack: RemoteNetstack,
    local_addr: SocksAddress,
    pending: Mutex<Option<TcpStream>>,
}

impl RemoteTcpListener {
    /// Address assigned by the `RemoteSocks` proxy after the first BIND reply.
    #[must_use]
    pub const fn local_addr(&self) -> &SocksAddress {
        &self.local_addr
    }

    /// Wait for the second BIND reply and return the accepted proxy stream.
    ///
    /// Further calls issue another BIND request using the assigned local
    /// address, matching the reusable listener behavior of the official SDK.
    ///
    /// # Errors
    ///
    /// Returns endpoint resolution, connection, negotiation, or BIND reply
    /// errors.
    pub async fn accept(&self) -> Result<RemoteTcpStream, Error> {
        let pending = self.pending.lock().await.take();
        let (local_addr, mut stream) = if let Some(stream) = pending {
            (self.local_addr.clone(), stream)
        } else {
            self.netstack
                .perform_command(BIND_COMMAND, &self.local_addr)
                .await?
        };
        let peer_addr = read_reply(&mut stream).await?;
        Ok(RemoteTcpStream::new(stream, local_addr, peer_addr))
    }
}

impl RemoteTcpStream {
    /// Address assigned by the `RemoteSocks` proxy.
    #[must_use]
    pub const fn local_addr(&self) -> &SocksAddress {
        &self.local_addr
    }

    /// Requested remote address.
    #[must_use]
    pub const fn peer_addr(&self) -> &SocksAddress {
        &self.peer_addr
    }

    /// Consume the wrapper and return the underlying proxy TCP stream.
    #[must_use]
    pub fn into_inner(self) -> TcpStream {
        self.inner
    }

    pub(crate) const fn new(
        inner: TcpStream,
        local_addr: SocksAddress,
        peer_addr: SocksAddress,
    ) -> Self {
        Self {
            inner,
            local_addr,
            peer_addr,
        }
    }
}

impl RemoteNetstack {
    /// Connect to a TCP or custom-network address through `RemoteSocks`.
    ///
    /// # Errors
    ///
    /// Returns endpoint resolution, proxy connection, SOCKS negotiation, or
    /// command reply errors.
    pub async fn connect_tcp(&self, address: SocksAddress) -> Result<RemoteTcpStream, Error> {
        let (local_addr, stream) = self.perform_command(CONNECT_COMMAND, &address).await?;
        Ok(RemoteTcpStream::new(stream, local_addr, address))
    }

    /// Bind a TCP or custom-network listener through `RemoteSocks`.
    ///
    /// This method waits for the first SOCKS BIND reply. Call
    /// [`RemoteTcpListener::accept`] to wait for the second reply carrying the
    /// connecting peer address.
    ///
    /// # Errors
    ///
    /// Returns endpoint resolution, proxy connection, SOCKS negotiation, or
    /// first-stage BIND reply errors.
    pub async fn bind_tcp(&self, address: SocksAddress) -> Result<RemoteTcpListener, Error> {
        let (local_addr, stream) = self.perform_command(BIND_COMMAND, &address).await?;
        Ok(RemoteTcpListener {
            netstack: self.clone(),
            local_addr,
            pending: Mutex::new(Some(stream)),
        })
    }
}

impl AsyncRead for RemoteTcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(context, buffer)
    }
}

impl AsyncWrite for RemoteTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.inner).poll_write(context, buffer)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_flush(context)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_shutdown(context)
    }
}
