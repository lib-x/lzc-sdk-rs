use std::future::Future;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use tokio::net::{UdpSocket, lookup_host};

use crate::Error;

const SERVICE_HOST: &str = "host.lzcapp";
const ROUTE_PROBE_PORT: u16 = 9;

/// Query the source IP selected by the kernel for reaching `host.lzcapp`.
///
/// The UDP socket is connected only to ask the kernel for a route; this
/// function does not send a packet to the resolved host.
///
/// # Errors
///
/// Returns a DNS error when `host.lzcapp` cannot be resolved, or a no-route
/// error containing the last socket failure when no candidate is reachable.
pub async fn query_service_address() -> Result<IpAddr, Error> {
    let candidates = lookup_host((SERVICE_HOST, ROUTE_PROBE_PORT))
        .await
        .map_err(|source| Error::ServiceAddressLookup { source })?;
    query_service_address_from(candidates, source_address_for).await
}

async fn query_service_address_from<I, F, Fut>(
    candidates: I,
    mut source_address: F,
) -> Result<IpAddr, Error>
where
    I: IntoIterator<Item = SocketAddr>,
    F: FnMut(SocketAddr) -> Fut,
    Fut: Future<Output = io::Result<IpAddr>>,
{
    let mut last_error = None;
    for candidate in candidates {
        match source_address(candidate).await {
            Ok(address) => return Ok(address),
            Err(error) => last_error = Some(error),
        }
    }
    Err(Error::ServiceAddressNoRoute { source: last_error })
}

async fn source_address_for(candidate: SocketAddr) -> io::Result<IpAddr> {
    let socket = UdpSocket::bind(unspecified_bind_address(candidate)).await?;
    socket.connect(candidate).await?;
    Ok(socket.local_addr()?.ip())
}

const fn unspecified_bind_address(candidate: SocketAddr) -> SocketAddr {
    match candidate {
        SocketAddr::V4(_) => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        SocketAddr::V6(_) => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0),
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::sync::Arc;

    use tokio::sync::Mutex;

    use crate::Error;

    use super::{query_service_address_from, source_address_for, unspecified_bind_address};

    #[tokio::test]
    async fn iterates_candidates_until_a_route_succeeds() {
        let first = SocketAddr::from(([192, 0, 2, 1], 9));
        let second = SocketAddr::from(([127, 0, 0, 1], 9));
        let visited = Arc::new(Mutex::new(Vec::new()));
        let result = query_service_address_from([first, second], {
            let visited = Arc::clone(&visited);
            move |candidate| {
                let visited = Arc::clone(&visited);
                async move {
                    visited.lock().await.push(candidate);
                    if candidate == first {
                        Err(io::Error::new(io::ErrorKind::NetworkUnreachable, "first"))
                    } else {
                        Ok(IpAddr::V4(Ipv4Addr::LOCALHOST))
                    }
                }
            }
        })
        .await
        .expect("second candidate succeeds");
        assert_eq!(result, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(*visited.lock().await, [first, second]);
    }

    #[tokio::test]
    async fn preserves_the_last_route_error() {
        let error = query_service_address_from(
            [
                SocketAddr::from(([192, 0, 2, 1], 9)),
                SocketAddr::from(([198, 51, 100, 1], 9)),
            ],
            |candidate| async move {
                let kind = if candidate.ip() == IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)) {
                    io::ErrorKind::NetworkUnreachable
                } else {
                    io::ErrorKind::ConnectionRefused
                };
                Err(io::Error::new(kind, "route probe"))
            },
        )
        .await
        .expect_err("all route probes fail");
        match error {
            Error::ServiceAddressNoRoute {
                source: Some(source),
            } => assert_eq!(source.kind(), io::ErrorKind::ConnectionRefused),
            other => panic!("unexpected error: {other}"),
        }
    }

    #[tokio::test]
    async fn reports_no_route_when_dns_returns_no_candidates() {
        let error =
            query_service_address_from([], |_| async { Ok(IpAddr::V4(Ipv4Addr::LOCALHOST)) })
                .await
                .expect_err("empty candidates must fail");
        assert!(matches!(
            error,
            Error::ServiceAddressNoRoute { source: None }
        ));
    }

    #[tokio::test]
    async fn selects_ipv4_and_ipv6_loopback_source_addresses() {
        let ipv4 = source_address_for(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9))
            .await
            .expect("IPv4 loopback route");
        assert_eq!(ipv4, IpAddr::V4(Ipv4Addr::LOCALHOST));

        let ipv6 = source_address_for(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 9))
            .await
            .expect("IPv6 loopback route");
        assert_eq!(ipv6, IpAddr::V6(Ipv6Addr::LOCALHOST));
    }

    #[test]
    fn binds_an_unspecified_socket_of_the_candidate_family() {
        assert_eq!(
            unspecified_bind_address(SocketAddr::from(([127, 0, 0, 1], 9))),
            SocketAddr::from(([0, 0, 0, 0], 0))
        );
        assert_eq!(
            unspecified_bind_address(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 9)),
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0)
        );
    }
}
