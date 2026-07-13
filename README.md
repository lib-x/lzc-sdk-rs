# LazyCat Rust SDK

Rust client/server bindings and runtime helpers for LazyCat application and
device APIs. The crate covers the generated protobuf surface and the
handwritten behavior provided by the official Go SDK: runtime gateway access,
device mTLS authentication, token caching, service status, HPortalSys,
RemoteSocks TCP/UDP, peer identity, metadata compatibility, and service-address
discovery.

## Requirements

- Rust 1.88 or newer.
- A LazyCat application runtime for APIs that use mounted sockets or
  credentials.

Applications consuming this crate do **not** need Go, GOPATH, Buf, or protoc.
All 42 protobuf definitions are vendored under `proto/`, and generated Rust
sources are committed under `src/gen/`. Cargo builds never read an external SDK
checkout or a maintainer's local path.

## Installation

Until the crate is published to a registry, pin the Git release tag:

```toml
[dependencies]
lzc-sdk = { git = "https://github.com/lib-x/lzc-sdk-rs", tag = "v0.1.0" }
```

Default features expose the full protobuf, gateway, and RemoteSocks APIs.

## Runtime API gateway

```rust,no_run
use lzc_sdk::ApiGateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gateway = ApiGateway::connect().await?;
    let version = gateway.version().get(()).await?.into_inner();
    println!("LazyCat runtime version: {}", version.version);
    Ok(())
}
```

`ApiGateway::connect()` loads the mounted application credentials and connects
to `/lzcapp/run/sys/lzc-apis.socket` with mTLS. A non-empty
`LZCAPP_API_GATEWAY_ADDRESS` selects the official SDK's plaintext development
override instead.

The gateway exposes generated clients for Box, Users, Devices, Permissions,
PeripheralDevice, ISCSI, FileTransfer, PackageManager, AccessController,
Btrfs, DirMonitor, Message, TvOS, and Version services.

## Remote device APIs

```rust,no_run
use lzc_sdk::ApiGateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gateway = ApiGateway::connect().await?;
    let device = gateway
        .device_proxy("https://device-api.example.invalid")
        .await?;

    let status = device.status().photo_library().query().await?;
    println!("photo library: {} ({})", status.state, status.reason);

    // Generated device clients automatically receive lzc_dapi_auth_token.
    let _device_client = device.device();
    Ok(())
}
```

`DeviceProxy` uses the mounted application certificate for mTLS, signs the
official `RequestAuthToken` request with Ed25519 or RSA PKCS#1 v1.5 as
appropriate, caches tokens until 30 seconds before their deadline, and injects
the token into every generated device client request. Token and private-key
Debug output is redacted.

Available device clients cover Config, Device, Dialog, PhotoLibrary, Network,
Permission, FileHandler, RIM, RemoteControl, and Contacts.

## HPortalSys and RemoteSocks

```rust,no_run
use lzc_sdk::{HServerClient, RemoteNetstack, SocksAddress};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hserver = HServerClient::connect().await?;

    // Empty target uses the HServer network stack. A peer ID selects HClient.
    let netstack = RemoteNetstack::new(hserver, "");
    let mut stream = netstack
        .connect_tcp(SocksAddress::domain("example.com", 80)?)
        .await?;
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;
    Ok(())
}
```

RemoteSocks support includes:

- RFC 1928 NoAuth, CONNECT, two-stage BIND, and UDP ASSOCIATE;
- LazyCat ConnectUDP (`0xe2`) and BindUDP (`0xe3`) packet streams;
- IPv4, IPv6, domains, and LazyCat custom-network addresses;
- cached HPortalSys endpoints with one retry only when the endpoint changes;
- source filtering, bounded frame sizes, and control-stream lifetime tracking.

Use `RemoteNetstack::udp_associate`, `connect_udp`, or `bind_udp` to construct a
`RemoteUdpSocket`. Connected sockets support `send`/`recv`; bound sockets use
`send_to`/`recv_from`.

## Compatibility helpers

`query_service_address().await` resolves `host.lzcapp` and asks the kernel which
source IP it would use, without sending a packet.

`with_real_uid(&mut request, uid)` adds the deprecated `X-Hc-User-Id`
compatibility field. It is not an authorization boundary.

Servers can call `peer_application(&request)` to extract the authenticated
application ID, box ID, and application domain from exactly one TLS client
certificate. TCP and Unix-domain tonic TLS connection metadata are supported.

## Generated protocols

All generated messages, clients, and servers are available under:

- `lzc_sdk::proto::common`
- `lzc_sdk::proto::localdevice`
- `lzc_sdk::proto::sys`
- `lzc_sdk::proto::dlna`
- `lzc_sdk::proto::containerd`

`HPortalSys` belongs to `proto::sys`, matching its protobuf package.

## Maintaining protobuf sources

Maintainers need Git, Buf 1.71+, and network access:

```bash
./scripts/sync-protos.sh --check
./scripts/generate.sh --check
```

`sync-protos.sh` fetches pinned upstream revisions by default, rewrites imports
to Buf package directories, formats the tree, and verifies source/transformed
SHA-256 manifests. Source directories can be supplied explicitly through
`LZC_SDK_SOURCE` and `LZC_BASEOS_SOURCE`; those are maintenance-only inputs and
are never used by Cargo builds.

## Verification

```bash
cargo +1.88.0 fmt --all -- --check
cargo +1.88.0 clippy --all-features --all-targets -- -D warnings
cargo +1.88.0 test --all-features
cargo +1.88.0 doc --all-features --no-deps
cargo +1.88.0 package --allow-dirty
```

## License

MIT
