use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpStream, UdpSocket, lookup_host};
use tokio::sync::{Mutex, watch};
use tokio::task::JoinHandle;

use crate::Error;

use super::client::ASSOCIATE_COMMAND;
use super::{RemoteNetstack, SocksAddress};

const SOCKS_UDP_PREFIX: [u8; 3] = [0, 0, 0];
const MAX_UDP_PAYLOAD: usize = 65_507;
const FRAMED_UDP_HEADER_LENGTH: usize = 20;
const CONNECT_UDP_COMMAND: u8 = 0xe2;
const BIND_UDP_COMMAND: u8 = 0xe3;

enum UdpTransport {
    Association(Association),
    Framed(FramedTransport),
}

struct Association {
    socket: UdpSocket,
    relay: SocketAddr,
    control_closed: Mutex<watch::Receiver<bool>>,
    control_task: JoinHandle<()>,
}

struct FramedTransport {
    reader: Mutex<OwnedReadHalf>,
    writer: Mutex<OwnedWriteHalf>,
}

/// UDP socket transported through standard or extended `LazyCat` `RemoteSocks`.
pub struct RemoteUdpSocket {
    transport: UdpTransport,
    local_addr: SocksAddress,
    peer_addr: Option<SocksAddress>,
}

impl fmt::Debug for RemoteUdpSocket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let transport = match &self.transport {
            UdpTransport::Association(_) => "associate",
            UdpTransport::Framed(_) => "framed",
        };
        formatter
            .debug_struct("RemoteUdpSocket")
            .field("transport", &transport)
            .field("local_addr", &self.local_addr)
            .field("peer_addr", &self.peer_addr)
            .finish_non_exhaustive()
    }
}

impl RemoteUdpSocket {
    /// Local UDP address used by this transport.
    #[must_use]
    pub const fn local_addr(&self) -> &SocksAddress {
        &self.local_addr
    }

    /// Fixed remote peer for associated or connected sockets.
    #[must_use]
    pub const fn peer_addr(&self) -> Option<&SocksAddress> {
        self.peer_addr.as_ref()
    }

    /// Send one datagram to the fixed peer.
    ///
    /// # Errors
    ///
    /// Returns an error when this is a bound socket without a fixed peer, the
    /// control stream is closed, or the datagram cannot be sent.
    pub async fn send(&self, payload: &[u8]) -> Result<usize, Error> {
        let peer = self.peer_addr.as_ref().ok_or(Error::SocksUdpNotConnected)?;
        self.send_to(payload, peer).await
    }

    /// Receive one datagram from the fixed peer, filtering other sources.
    ///
    /// # Errors
    ///
    /// Returns an error when this is a bound socket without a fixed peer, the
    /// control stream closes, or a proxy datagram is malformed.
    pub async fn recv(&self, payload: &mut [u8]) -> Result<usize, Error> {
        let peer = self.peer_addr.as_ref().ok_or(Error::SocksUdpNotConnected)?;
        loop {
            let (length, source) = self.recv_from(payload).await?;
            if &source == peer {
                return Ok(length);
            }
        }
    }

    /// Send one datagram to an explicit target.
    ///
    /// Standard associations use RFC 1928 UDP headers. `LazyCat` `0xe2` and
    /// `0xe3` transports use the SDK's length/IP/port stream framing.
    ///
    /// # Errors
    ///
    /// Returns an error for oversized frames, unsupported target address
    /// forms, closed control streams, or transport write failures.
    pub async fn send_to(&self, payload: &[u8], target: &SocksAddress) -> Result<usize, Error> {
        if payload.len() > MAX_UDP_PAYLOAD {
            return Err(Error::SocksUdpPayloadTooLarge);
        }
        match &self.transport {
            UdpTransport::Association(association) => {
                send_associated(association, payload, target).await
            }
            UdpTransport::Framed(transport) => send_framed(transport, payload, target).await,
        }
    }

    /// Receive one datagram and its encoded source address.
    ///
    /// Standard-association packets not sent by the negotiated relay are
    /// ignored.
    ///
    /// # Errors
    ///
    /// Returns an error when a control stream closes or a datagram has invalid
    /// framing or address fields.
    pub async fn recv_from(&self, payload: &mut [u8]) -> Result<(usize, SocksAddress), Error> {
        match &self.transport {
            UdpTransport::Association(association) => recv_associated(association, payload).await,
            UdpTransport::Framed(transport) => recv_framed(transport, payload).await,
        }
    }
}

impl Drop for RemoteUdpSocket {
    fn drop(&mut self) {
        if let UdpTransport::Association(association) = &self.transport {
            association.control_task.abort();
        }
    }
}

impl RemoteNetstack {
    /// Create a standard RFC 1928 UDP association with a fixed default target.
    ///
    /// The TCP control stream remains open for the lifetime of the returned
    /// socket. Closing it causes receive operations to return
    /// [`Error::RemoteSocksControlClosed`].
    ///
    /// # Errors
    ///
    /// Returns endpoint resolution, SOCKS negotiation, relay address, socket
    /// binding, or control-stream errors.
    pub async fn udp_associate(&self, target: SocksAddress) -> Result<RemoteUdpSocket, Error> {
        let unspecified = SocksAddress::Ip(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));
        let (relay, control) = self
            .perform_command(ASSOCIATE_COMMAND, &unspecified)
            .await?;
        let relay = resolve_socket_address(&relay).await?;
        let bind_address = unspecified_for(relay);
        let socket = UdpSocket::bind(bind_address).await?;
        let local_addr = SocksAddress::Ip(socket.local_addr()?);
        let (control_closed, control_task) = monitor_control_stream(control);
        Ok(RemoteUdpSocket {
            transport: UdpTransport::Association(Association {
                socket,
                relay,
                control_closed: Mutex::new(control_closed),
                control_task,
            }),
            local_addr,
            peer_addr: Some(target),
        })
    }

    /// Connect a UDP peer using the `LazyCat` `0xe2` stream extension.
    ///
    /// Domain targets are resolved locally, matching the official Go SDK.
    ///
    /// # Errors
    ///
    /// Returns endpoint resolution, DNS, SOCKS negotiation, address, or framed
    /// transport errors.
    pub async fn connect_udp(&self, target: SocksAddress) -> Result<RemoteUdpSocket, Error> {
        let target = resolve_udp_target(target).await?;
        let (local_addr, stream) = self.perform_command(CONNECT_UDP_COMMAND, &target).await?;
        Ok(framed_socket(stream, local_addr, Some(target)))
    }

    /// Bind a UDP socket using the `LazyCat` `0xe3` stream extension.
    ///
    /// Use [`RemoteUdpSocket::send_to`] and [`RemoteUdpSocket::recv_from`] with
    /// the returned socket because it has no fixed peer.
    ///
    /// # Errors
    ///
    /// Returns endpoint resolution, DNS, SOCKS negotiation, address, or framed
    /// transport errors.
    pub async fn bind_udp(&self, address: SocksAddress) -> Result<RemoteUdpSocket, Error> {
        let address = resolve_udp_target(address).await?;
        let (local_addr, stream) = self.perform_command(BIND_UDP_COMMAND, &address).await?;
        Ok(framed_socket(stream, local_addr, None))
    }
}

async fn send_associated(
    association: &Association,
    payload: &[u8],
    target: &SocksAddress,
) -> Result<usize, Error> {
    if *association.control_closed.lock().await.borrow() {
        return Err(Error::RemoteSocksControlClosed);
    }
    let address = target.encode()?;
    let frame_length = SOCKS_UDP_PREFIX.len() + address.len() + payload.len();
    if frame_length > MAX_UDP_PAYLOAD {
        return Err(Error::SocksUdpPayloadTooLarge);
    }
    let mut frame = Vec::with_capacity(frame_length);
    frame.extend_from_slice(&SOCKS_UDP_PREFIX);
    frame.extend_from_slice(&address);
    frame.extend_from_slice(payload);
    let written = association
        .socket
        .send_to(&frame, association.relay)
        .await?;
    if written != frame.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::WriteZero,
            "short SOCKS UDP datagram write",
        )
        .into());
    }
    Ok(payload.len())
}

async fn recv_associated(
    association: &Association,
    payload: &mut [u8],
) -> Result<(usize, SocksAddress), Error> {
    let mut packet = vec![0_u8; MAX_UDP_PAYLOAD];
    let mut control_closed = association.control_closed.lock().await;
    loop {
        if *control_closed.borrow() {
            return Err(Error::RemoteSocksControlClosed);
        }
        let received = tokio::select! {
            changed = control_closed.changed() => {
                let _ = changed;
                return Err(Error::RemoteSocksControlClosed);
            }
            received = association.socket.recv_from(&mut packet) => received?,
        };
        let (length, source) = received;
        if source != association.relay {
            continue;
        }
        if length < SOCKS_UDP_PREFIX.len() || packet[..2] != [0, 0] {
            return Err(Error::InvalidSocksUdpHeader);
        }
        if packet[2] != 0 {
            return Err(Error::UnsupportedSocksUdpFragment {
                fragment: packet[2],
            });
        }
        let (source, consumed) = SocksAddress::decode_prefix(&packet[3..length])?;
        let datagram = &packet[3 + consumed..length];
        let copied = copy_payload(payload, datagram);
        return Ok((copied, source));
    }
}

async fn send_framed(
    transport: &FramedTransport,
    payload: &[u8],
    target: &SocksAddress,
) -> Result<usize, Error> {
    let target = concrete_ip_address(target)?;
    let mut header = [0_u8; FRAMED_UDP_HEADER_LENGTH];
    header[..2].copy_from_slice(
        &u16::try_from(payload.len())
            .map_err(|_| Error::SocksUdpPayloadTooLarge)?
            .to_be_bytes(),
    );
    header[2..18].copy_from_slice(&ip_octets(target.ip()));
    header[18..].copy_from_slice(&target.port().to_be_bytes());
    let mut writer = transport.writer.lock().await;
    writer.write_all(&header).await?;
    writer.write_all(payload).await?;
    Ok(payload.len())
}

async fn recv_framed(
    transport: &FramedTransport,
    payload: &mut [u8],
) -> Result<(usize, SocksAddress), Error> {
    let mut reader = transport.reader.lock().await;
    let mut header = [0_u8; FRAMED_UDP_HEADER_LENGTH];
    reader.read_exact(&mut header).await?;
    let length = usize::from(u16::from_be_bytes([header[0], header[1]]));
    let mut datagram = vec![0_u8; length];
    reader.read_exact(&mut datagram).await?;
    if length > MAX_UDP_PAYLOAD {
        return Err(Error::SocksUdpPayloadTooLarge);
    }
    let octets: [u8; 16] = header[2..18]
        .try_into()
        .map_err(|_| Error::InvalidSocksUdpHeader)?;
    let ipv6 = Ipv6Addr::from(octets);
    let ip = ipv6.to_ipv4_mapped().map_or(IpAddr::V6(ipv6), IpAddr::V4);
    let port = u16::from_be_bytes([header[18], header[19]]);
    let copied = copy_payload(payload, &datagram);
    Ok((copied, SocksAddress::Ip(SocketAddr::new(ip, port))))
}

fn framed_socket(
    stream: TcpStream,
    local_addr: SocksAddress,
    peer_addr: Option<SocksAddress>,
) -> RemoteUdpSocket {
    let (reader, writer) = stream.into_split();
    RemoteUdpSocket {
        transport: UdpTransport::Framed(FramedTransport {
            reader: Mutex::new(reader),
            writer: Mutex::new(writer),
        }),
        local_addr,
        peer_addr,
    }
}

fn monitor_control_stream(control: TcpStream) -> (watch::Receiver<bool>, JoinHandle<()>) {
    let (sender, receiver) = watch::channel(false);
    let task = tokio::spawn(async move {
        let (mut reader, writer) = control.into_split();
        let _writer = writer;
        let mut buffer = [0_u8; 1];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) | Err(_) => break,
                Ok(_) => {}
            }
        }
        let _ = sender.send(true);
    });
    (receiver, task)
}

async fn resolve_socket_address(address: &SocksAddress) -> Result<SocketAddr, Error> {
    match address {
        SocksAddress::Ip(address) => Ok(*address),
        SocksAddress::Domain { host, port } => lookup_host((host.as_str(), *port))
            .await?
            .next()
            .ok_or(Error::InvalidSocksUdpAddress),
        SocksAddress::Custom { .. } => Err(Error::InvalidSocksUdpAddress),
    }
}

async fn resolve_udp_target(address: SocksAddress) -> Result<SocksAddress, Error> {
    match address {
        SocksAddress::Ip(_) => Ok(address),
        SocksAddress::Domain { .. } => {
            Ok(SocksAddress::Ip(resolve_socket_address(&address).await?))
        }
        SocksAddress::Custom { .. } => Err(Error::InvalidSocksUdpAddress),
    }
}

fn concrete_ip_address(address: &SocksAddress) -> Result<SocketAddr, Error> {
    match address {
        SocksAddress::Ip(address) => Ok(*address),
        SocksAddress::Domain { .. } | SocksAddress::Custom { .. } => {
            Err(Error::InvalidSocksUdpAddress)
        }
    }
}

const fn unspecified_for(address: SocketAddr) -> SocketAddr {
    match address {
        SocketAddr::V4(_) => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        SocketAddr::V6(_) => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0),
    }
}

fn ip_octets(ip: IpAddr) -> [u8; 16] {
    match ip {
        IpAddr::V4(ip) => ip.to_ipv6_mapped().octets(),
        IpAddr::V6(ip) => ip.octets(),
    }
}

fn copy_payload(destination: &mut [u8], source: &[u8]) -> usize {
    let copied = destination.len().min(source.len());
    destination[..copied].copy_from_slice(&source[..copied]);
    copied
}
