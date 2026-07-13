use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use lzc_sdk::{Error, RemoteNetstack, RemoteSocksEndpoint, SocksAddress};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[test]
fn socks_address_codec_matches_rfc1928_and_lazycat_custom_fixtures() {
    let fixtures = [
        (
            SocksAddress::Ip(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                8080,
            )),
            b"\x01\x7f\x00\x00\x01\x1f\x90".as_slice(),
        ),
        (
            SocksAddress::Ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 53)),
            b"\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x35"
                .as_slice(),
        ),
        (
            SocksAddress::domain("example.com", 443).expect("domain address"),
            b"\x03\x0bexample.com\x01\xbb".as_slice(),
        ),
        (
            SocksAddress::custom("unix", "/tmp/socket").expect("custom address"),
            b"\x03\x1ddW5peA.L3RtcC9zb2NrZXQ.custom\x00\x00".as_slice(),
        ),
    ];

    for (address, bytes) in fixtures {
        assert_eq!(address.encode().expect("encode address"), bytes);
        assert_eq!(
            SocksAddress::decode(bytes).expect("decode address"),
            address
        );
    }
}

#[test]
fn socks_address_codec_rejects_malformed_or_oversized_input() {
    for encoded in [
        b"".as_slice(),
        b"\x02\x00\x00".as_slice(),
        b"\x01\x7f\x00\x00\x01\x00".as_slice(),
        b"\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00".as_slice(),
        b"\x03\x03ab\x00\x50".as_slice(),
        b"\x03\x01\xff\x00\x50".as_slice(),
        b"\x03\x01a\x00\x50\x00".as_slice(),
        b"\x03\x0c%%%%.custom\x00\x00".as_slice(),
    ] {
        assert!(SocksAddress::decode(encoded).is_err(), "{encoded:?}");
    }

    let too_long = "a".repeat(256);
    assert!(matches!(
        SocksAddress::domain(too_long, 80),
        Err(Error::SocksAddressTooLong)
    ));
    assert!(SocksAddress::domain("bad host", 80).is_err());
    assert!(SocksAddress::domain("2001:db8::1", 80).is_err());
    assert!("missing-port".parse::<SocksAddress>().is_err());
    assert!("[::1]".parse::<SocksAddress>().is_err());
}

#[tokio::test]
async fn remote_tcp_connect_performs_no_auth_handshake_and_preserves_addresses() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind proxy fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", listener.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.expect("accept proxy connection");

        let mut greeting = [0_u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        assert_eq!(greeting, [0x05, 0x01, 0x00]);
        stream
            .write_all(&[0x05, 0x00])
            .await
            .expect("write auth reply");

        let mut request = [0_u8; 3 + 2 + 11 + 2];
        stream
            .read_exact(&mut request)
            .await
            .expect("read CONNECT request");
        assert_eq!(request, *b"\x05\x01\x00\x03\x0bexample.com\x00\x16");
        stream
            .write_all(b"\x05\x00\x00\x01\x7f\x00\x00\x01\x9c\x40")
            .await
            .expect("write CONNECT reply");

        let mut payload = [0_u8; 4];
        stream.read_exact(&mut payload).await.expect("read payload");
        assert_eq!(&payload, b"ping");
        stream.write_all(b"pong").await.expect("write payload");
    });

    let netstack = RemoteNetstack::fixed(endpoint);
    let target = SocksAddress::domain("example.com", 22).expect("target");
    let mut stream = netstack
        .connect_tcp(target.clone())
        .await
        .expect("SOCKS CONNECT");
    assert_eq!(
        stream.local_addr(),
        &SocksAddress::Ip(SocketAddr::from(([127, 0, 0, 1], 40_000)))
    );
    assert_eq!(stream.peer_addr(), &target);
    stream
        .write_all(b"ping")
        .await
        .expect("write tunneled data");
    let mut response = [0_u8; 4];
    stream
        .read_exact(&mut response)
        .await
        .expect("read tunneled data");
    assert_eq!(&response, b"pong");
    server.await.expect("proxy fixture");
}

#[tokio::test]
async fn remote_tcp_bind_waits_for_the_second_reply_before_accepting() {
    let proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind proxy fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let server = tokio::spawn(async move {
        let (mut stream, _) = proxy.accept().await.expect("accept proxy connection");
        let mut greeting = [0_u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        stream.write_all(&[5, 0]).await.expect("auth reply");

        let mut bind_request = [0_u8; 10];
        stream
            .read_exact(&mut bind_request)
            .await
            .expect("read BIND request");
        assert_eq!(bind_request, [5, 2, 0, 1, 0, 0, 0, 0, 0, 0]);
        stream
            .write_all(b"\x05\x00\x00\x01\x7f\x00\x00\x01\xd6\xd8")
            .await
            .expect("write first BIND reply");

        tokio::task::yield_now().await;
        stream
            .write_all(b"\x05\x00\x00\x01\xc0\x00\x02\x0a\x0d\x05")
            .await
            .expect("write second BIND reply");
        stream
            .write_all(b"bound")
            .await
            .expect("write accepted payload");
    });

    let netstack = RemoteNetstack::fixed(endpoint);
    let listener = netstack
        .bind_tcp(SocksAddress::Ip(SocketAddr::from(([0, 0, 0, 0], 0))))
        .await
        .expect("SOCKS BIND");
    assert_eq!(
        listener.local_addr(),
        &SocksAddress::Ip(SocketAddr::from(([127, 0, 0, 1], 55_000)))
    );
    let mut stream = listener.accept().await.expect("accept bound stream");
    assert_eq!(stream.local_addr(), listener.local_addr());
    assert_eq!(
        stream.peer_addr(),
        &SocksAddress::Ip(SocketAddr::from(([192, 0, 2, 10], 3333)))
    );
    let mut payload = [0_u8; 5];
    stream.read_exact(&mut payload).await.expect("read payload");
    assert_eq!(&payload, b"bound");
    server.await.expect("proxy fixture");
}

#[tokio::test]
async fn remote_tcp_rejects_malformed_authentication_and_command_replies() {
    assert!(matches!(
        connect_with_proxy_replies(&[4, 0], None).await,
        Error::UnexpectedSocksVersion { version: 4 }
    ));
    assert!(matches!(
        connect_with_proxy_replies(&[5, 0xff], None).await,
        Error::SocksNoAcceptableAuthentication
    ));
    assert!(matches!(
        connect_with_proxy_replies(&[5, 2], None).await,
        Error::UnsupportedSocksAuthentication { method: 2 }
    ));
    assert!(matches!(
        connect_with_proxy_replies(&[5, 0], Some(&[5, 5, 0])).await,
        Error::SocksReply { code: 5 }
    ));
    assert!(matches!(
        connect_with_proxy_replies(&[5, 0], Some(&[5, 0, 1])).await,
        Error::InvalidSocksReservedByte { reserved: 1 }
    ));
    assert!(matches!(
        connect_with_proxy_replies(&[5, 0], Some(&[5, 0, 0, 2])).await,
        Error::UnsupportedSocksAddressType { address_type: 2 }
    ));
}

async fn connect_with_proxy_replies(
    auth_reply: &'static [u8],
    command_reply: Option<&'static [u8]>,
) -> Error {
    let proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind proxy fixture");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let server = tokio::spawn(async move {
        let (mut stream, _) = proxy.accept().await.expect("accept proxy connection");
        let mut greeting = [0_u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        stream
            .write_all(auth_reply)
            .await
            .expect("write auth reply");
        if let Some(command_reply) = command_reply {
            let mut request = [0_u8; 10];
            stream
                .read_exact(&mut request)
                .await
                .expect("read CONNECT request");
            stream
                .write_all(command_reply)
                .await
                .expect("write command reply");
        }
    });
    let error = RemoteNetstack::fixed(endpoint)
        .connect_tcp(SocksAddress::Ip(SocketAddr::from(([1, 1, 1, 1], 80))))
        .await
        .expect_err("fixture reply must fail");
    server.await.expect("proxy fixture");
    error
}

#[tokio::test]
async fn remote_tcp_refreshes_a_failed_endpoint_once_and_caches_the_replacement() {
    let unavailable = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("reserve unavailable address");
    let unavailable_endpoint: RemoteSocksEndpoint =
        format!("socks5://{}", unavailable.local_addr().unwrap())
            .parse()
            .expect("unavailable endpoint");
    drop(unavailable);

    let proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind replacement proxy");
    let replacement_endpoint: RemoteSocksEndpoint =
        format!("socks5://{}", proxy.local_addr().unwrap())
            .parse()
            .expect("replacement endpoint");
    let server = tokio::spawn(serve_successful_connects(proxy, 2));

    let resolutions = Arc::new(AtomicUsize::new(0));
    let netstack = RemoteNetstack::with_endpoint_resolver({
        let resolutions = Arc::clone(&resolutions);
        move || {
            let index = resolutions.fetch_add(1, Ordering::SeqCst);
            let endpoint = if index == 0 {
                unavailable_endpoint.clone()
            } else {
                replacement_endpoint.clone()
            };
            async move { Ok(endpoint) }
        }
    });
    let target = SocksAddress::Ip(SocketAddr::from(([203, 0, 113, 7], 22)));

    drop(
        netstack
            .connect_tcp(target.clone())
            .await
            .expect("changed endpoint retry"),
    );
    drop(
        netstack
            .connect_tcp(target)
            .await
            .expect("cached replacement endpoint"),
    );

    assert_eq!(resolutions.load(Ordering::SeqCst), 2);
    server.await.expect("replacement proxy fixture");
}

async fn serve_successful_connects(proxy: TcpListener, count: usize) {
    for _ in 0..count {
        let (mut stream, _) = proxy.accept().await.expect("accept proxy connection");
        let mut greeting = [0_u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        stream.write_all(&[5, 0]).await.expect("write auth reply");
        let mut request = [0_u8; 10];
        stream
            .read_exact(&mut request)
            .await
            .expect("read CONNECT request");
        assert_eq!(&request[..3], &[5, 1, 0]);
        stream
            .write_all(b"\x05\x00\x00\x01\x7f\x00\x00\x01\x9c\x40")
            .await
            .expect("write CONNECT reply");
    }
}

#[tokio::test]
async fn remote_tcp_deadline_covers_a_stalled_socks_handshake() {
    let proxy = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind stalled proxy");
    let endpoint: RemoteSocksEndpoint = format!("socks5://{}", proxy.local_addr().unwrap())
        .parse()
        .expect("proxy endpoint");
    let server = tokio::spawn(async move {
        let (mut stream, _) = proxy.accept().await.expect("accept proxy connection");
        let mut greeting = [0_u8; 3];
        stream
            .read_exact(&mut greeting)
            .await
            .expect("read greeting");
        tokio::time::sleep(Duration::from_secs(5)).await;
    });

    let result = tokio::time::timeout(
        Duration::from_millis(500),
        RemoteNetstack::fixed(endpoint)
            .with_dial_timeout(Duration::from_millis(50))
            .connect_tcp(SocksAddress::Ip(SocketAddr::from(([1, 1, 1, 1], 80)))),
    )
    .await
    .expect("SDK must enforce its own deadline");
    assert!(matches!(result, Err(Error::RemoteSocksTimeout)));
    server.abort();
}
