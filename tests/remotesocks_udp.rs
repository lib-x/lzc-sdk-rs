use std::time::Duration;
use std::{net::SocketAddr, vec};

use lzc_sdk::{Error, RemoteNetstack, RemoteSocksEndpoint, SocksAddress};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, UdpSocket};

#[tokio::test]
async fn udp_associate_uses_rfc1928_frames_and_filters_non_proxy_sources() {
    let tcp_proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP proxy fixture");
    let udp_relay = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("bind UDP relay fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", tcp_proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let relay_addr = udp_relay.local_addr().expect("relay address");
    let server = tokio::spawn(async move {
        let (mut control, _) = tcp_proxy.accept().await.expect("accept control stream");
        let mut greeting = [0_u8; 3];
        control
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        assert_eq!(greeting, [5, 1, 0]);
        control.write_all(&[5, 0]).await.expect("auth reply");

        let mut associate = [0_u8; 10];
        control
            .read_exact(&mut associate)
            .await
            .expect("read ASSOCIATE request");
        assert_eq!(associate, [5, 3, 0, 1, 0, 0, 0, 0, 0, 0]);
        let mut reply = vec![5, 0, 0, 1, 127, 0, 0, 1];
        reply.extend_from_slice(&relay_addr.port().to_be_bytes());
        control.write_all(&reply).await.expect("ASSOCIATE reply");

        let mut datagram = [0_u8; 512];
        let (length, client_addr) = udp_relay
            .recv_from(&mut datagram)
            .await
            .expect("receive associated datagram");
        assert_eq!(
            &datagram[..length],
            b"\x00\x00\x00\x03\x0bexample.com\x00\x35query"
        );

        let spoof = UdpSocket::bind("127.0.0.1:0")
            .await
            .expect("bind spoof source");
        spoof
            .send_to(b"\x00\x00\x00\x03\x0bexample.com\x00\x35evil", client_addr)
            .await
            .expect("send spoofed datagram");
        udp_relay
            .send_to(
                b"\x00\x00\x00\x03\x0bexample.com\x00\x35answer",
                client_addr,
            )
            .await
            .expect("send proxy datagram");
        tokio::time::sleep(Duration::from_secs(5)).await;
    });

    let target = SocksAddress::domain("example.com", 53).expect("target");
    let socket = RemoteNetstack::fixed(endpoint)
        .udp_associate(target.clone())
        .await
        .expect("UDP ASSOCIATE");
    assert_eq!(socket.peer_addr(), Some(&target));
    assert_eq!(socket.send(b"query").await.expect("send query"), 5);
    let mut payload = [0_u8; 32];
    let length = socket.recv(&mut payload).await.expect("receive answer");
    assert_eq!(&payload[..length], b"answer");
    server.abort();
}

#[tokio::test]
async fn connect_udp_uses_lazycat_e2_packet_framing() {
    let tcp_proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP proxy fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", tcp_proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let server = tokio::spawn(async move {
        let (mut stream, _) = tcp_proxy.accept().await.expect("accept proxy stream");
        let mut greeting = [0_u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        stream.write_all(&[5, 0]).await.expect("auth reply");

        let mut command = [0_u8; 10];
        stream
            .read_exact(&mut command)
            .await
            .expect("read ConnectUDP request");
        assert_eq!(command, [5, 0xe2, 0, 1, 8, 8, 8, 8, 0, 53]);
        stream
            .write_all(b"\x05\x00\x00\x01\x0a\x00\x00\x02\x9c\x40")
            .await
            .expect("ConnectUDP reply");

        let mut frame = [0_u8; 25];
        stream
            .read_exact(&mut frame)
            .await
            .expect("read framed datagram");
        assert_eq!(
            &frame,
            b"\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\x08\x08\x08\x08\x00\x35query"
        );

        let mut reply =
            b"\x00\x06\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\x08\x08\x08\x08\x00\x35"
                .to_vec();
        reply.extend_from_slice(b"answer");
        stream
            .write_all(&reply)
            .await
            .expect("write framed datagram");
    });

    let peer = SocksAddress::Ip(SocketAddr::from(([8, 8, 8, 8], 53)));
    let socket = RemoteNetstack::fixed(endpoint)
        .connect_udp(peer.clone())
        .await
        .expect("ConnectUDP");
    assert_eq!(
        socket.local_addr(),
        &SocksAddress::Ip(SocketAddr::from(([10, 0, 0, 2], 40_000)))
    );
    assert_eq!(socket.peer_addr(), Some(&peer));
    assert_eq!(socket.send(b"query").await.expect("send query"), 5);
    let mut payload = [0_u8; 32];
    let length = socket.recv(&mut payload).await.expect("receive answer");
    assert_eq!(&payload[..length], b"answer");
    server.await.expect("proxy fixture");
}

#[tokio::test]
async fn bind_udp_uses_lazycat_e3_packet_framing_with_explicit_peers() {
    let tcp_proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP proxy fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", tcp_proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let server = tokio::spawn(async move {
        let (mut stream, _) = tcp_proxy.accept().await.expect("accept proxy stream");
        let mut greeting = [0_u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        stream.write_all(&[5, 0]).await.expect("auth reply");

        let mut command = [0_u8; 10];
        stream
            .read_exact(&mut command)
            .await
            .expect("read BindUDP request");
        assert_eq!(command, [5, 0xe3, 0, 1, 0, 0, 0, 0, 0, 0]);
        stream
            .write_all(b"\x05\x00\x00\x01\x0a\x00\x00\x02\x9c\x41")
            .await
            .expect("BindUDP reply");

        let mut frame = [0_u8; 24];
        stream
            .read_exact(&mut frame)
            .await
            .expect("read framed datagram");
        assert_eq!(
            &frame,
            b"\x00\x04\x20\x01\x0d\xb8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x14\xe9ping"
        );

        stream
            .write_all(
                b"\x00\x04\x20\x01\x0d\xb8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x14\xeapong",
            )
            .await
            .expect("write framed datagram");
    });

    let socket = RemoteNetstack::fixed(endpoint)
        .bind_udp(SocksAddress::Ip(SocketAddr::from(([0, 0, 0, 0], 0))))
        .await
        .expect("BindUDP");
    assert_eq!(
        socket.local_addr(),
        &SocksAddress::Ip(SocketAddr::from(([10, 0, 0, 2], 40_001)))
    );
    assert_eq!(socket.peer_addr(), None);
    assert!(matches!(
        socket.send(b"missing peer").await,
        Err(Error::SocksUdpNotConnected)
    ));

    let target: SocksAddress = "[2001:db8::1]:5353".parse().expect("IPv6 target");
    assert_eq!(
        socket.send_to(b"ping", &target).await.expect("send packet"),
        4
    );
    let mut payload = [0_u8; 16];
    let (length, source) = socket
        .recv_from(&mut payload)
        .await
        .expect("receive packet");
    assert_eq!(&payload[..length], b"pong");
    assert_eq!(
        source,
        "[2001:db8::2]:5354"
            .parse::<SocksAddress>()
            .expect("IPv6 source")
    );
    server.await.expect("proxy fixture");
}

#[tokio::test]
async fn udp_associate_rejects_reserved_fragment_and_address_header_errors() {
    assert!(matches!(
        receive_invalid_associated_datagram(b"\x01\x00\x00\x01\x7f\x00\x00\x01\x00\x35x").await,
        Error::InvalidSocksUdpHeader
    ));
    assert!(matches!(
        receive_invalid_associated_datagram(b"\x00\x00\x01\x01\x7f\x00\x00\x01\x00\x35x").await,
        Error::UnsupportedSocksUdpFragment { fragment: 1 }
    ));
    assert!(matches!(
        receive_invalid_associated_datagram(b"\x00\x00\x00\x02").await,
        Error::UnsupportedSocksAddressType { address_type: 2 }
    ));
}

#[tokio::test]
async fn udp_associate_lifetime_tracks_the_tcp_control_stream() {
    let tcp_proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP proxy fixture");
    let udp_relay = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("bind UDP relay fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", tcp_proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let relay_addr = udp_relay.local_addr().expect("relay address");
    let server = tokio::spawn(async move {
        let (mut control, _) = tcp_proxy.accept().await.expect("accept control stream");
        complete_associate_handshake(&mut control, relay_addr).await;
    });
    let socket = RemoteNetstack::fixed(endpoint)
        .udp_associate(SocksAddress::Ip(SocketAddr::from(([1, 1, 1, 1], 53))))
        .await
        .expect("UDP ASSOCIATE");
    server.await.expect("proxy fixture");

    let error = tokio::time::timeout(Duration::from_millis(500), socket.recv(&mut [0_u8; 8]))
        .await
        .expect("control close must wake receiver")
        .expect_err("closed control stream must fail");
    assert!(matches!(error, Error::RemoteSocksControlClosed));
}

async fn receive_invalid_associated_datagram(datagram: &'static [u8]) -> Error {
    let tcp_proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP proxy fixture");
    let udp_relay = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("bind UDP relay fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", tcp_proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let relay_addr = udp_relay.local_addr().expect("relay address");
    let server = tokio::spawn(async move {
        let (mut control, _) = tcp_proxy.accept().await.expect("accept control stream");
        complete_associate_handshake(&mut control, relay_addr).await;
        let mut trigger = [0_u8; 64];
        let (_, client_addr) = udp_relay
            .recv_from(&mut trigger)
            .await
            .expect("receive trigger");
        udp_relay
            .send_to(datagram, client_addr)
            .await
            .expect("send invalid datagram");
        tokio::time::sleep(Duration::from_secs(5)).await;
    });
    let socket = RemoteNetstack::fixed(endpoint)
        .udp_associate(SocksAddress::Ip(SocketAddr::from(([1, 1, 1, 1], 53))))
        .await
        .expect("UDP ASSOCIATE");
    socket.send(b"trigger").await.expect("send trigger");
    let error = socket
        .recv_from(&mut [0_u8; 8])
        .await
        .expect_err("invalid datagram must fail");
    server.abort();
    error
}

async fn complete_associate_handshake(control: &mut tokio::net::TcpStream, relay_addr: SocketAddr) {
    let mut greeting = [0_u8; 3];
    control
        .read_exact(&mut greeting)
        .await
        .expect("read greeting");
    control.write_all(&[5, 0]).await.expect("auth reply");
    let mut associate = [0_u8; 10];
    control
        .read_exact(&mut associate)
        .await
        .expect("read ASSOCIATE request");
    let mut reply = vec![5, 0, 0, 1, 127, 0, 0, 1];
    reply.extend_from_slice(&relay_addr.port().to_be_bytes());
    control.write_all(&reply).await.expect("ASSOCIATE reply");
}
