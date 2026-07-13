# Full LazyCat Rust SDK Design

**Date:** 2026-07-13
**Reference source:** `/home/czyt/code/ref/lzc-sdk`
**Target repository:** `/home/czyt/code/rust/lzc-sdk-rs`

## Goal

Build a production Rust SDK with functional parity to the official Go SDK in `lang/go`, then consume it from Neko Webshell instead of maintaining application-local implementations of LazyCat protocols.

“Complete” means both generated RPC coverage and the handwritten behavior that makes the Go SDK usable:

- all 41 proto files under `protos/{common,dlna,localdevice,sys}`;
- generated Rust messages plus tonic clients and servers for every service;
- runtime Unix-socket API connections;
- application mTLS credentials;
- `ApiGateway` and all named service clients exposed by the Go gateway;
- remote `DeviceProxy` clients with automatic device-token metadata;
- auth-token request signing and deadline-aware caching;
- peer application identity extraction from client certificates;
- real-user metadata compatibility;
- device service-status helpers;
- HPortalSys client and RemoteSocks endpoint discovery;
- RemoteSocks TCP, UDP, TCP bind, UDP bind, and custom-address operations;
- service-address discovery equivalent to `utils.QueryServiceAddress`.

## Source of Truth and Proto Sync

The official repository remains read-only. The Rust repository vendors exact proto sources beneath `proto/` and records their upstream commit (`81adfc8` at design time) in `proto/UPSTREAM.md`.

`scripts/sync-protos.sh` copies:

- `protos/common/**` to `proto/common/**`;
- `protos/dlna/**` to `proto/dlna/**`;
- `protos/localdevice/**` to `proto/localdevice/**`;
- `protos/sys/**` to `proto/sys/**`.

The Go SDK also depends on `lzc-baseos-protos` for `HPortalSys`. The Rust SDK vendors the referenced `baseos/hserver.proto` under `proto/baseos/hserver.proto` with its module version documented in `proto/UPSTREAM.md`.

Generated Rust files are committed under `src/gen/`. Consumers do not need Buf or protoc at compile time. SDK maintainers run `scripts/generate.sh`, which installs pinned local codegen plugins into `.tools/` and runs:

1. `buf format --diff --exit-code`;
2. `buf lint`;
3. `buf generate` with `protoc-gen-prost`, `protoc-gen-tonic`, and `protoc-gen-prost-crate` version `0.5.0`;
4. `cargo fmt --all -- --check` after generation.

The crate uses tonic/prost `0.14.6`, whose MSRV is Rust `1.88`, matching the Neko project floor.

## Module Layout

```text
src/
  lib.rs                 public exports and compatibility constants
  gen/                   Buf-generated messages, clients, and servers
  credentials.rs         credential paths, PEM loading, TLS identity
  connection.rs          TCP/TLS and Unix-socket tonic channels
  auth.rs                request signing, token acquisition, cache, peer identity
  metadata.rs            real-user metadata helper
  gateway.rs             ApiGateway and DeviceProxy composition
  service_status.rs      local service availability helpers
  hserver.rs             HPortalSys client and RemoteSocks endpoint resolution
  remotesocks/
    mod.rs               public RemoteNetstack and connection types
    address.rs           SOCKS5 and custom address codec
    client.rs            connect/bind command state machine and endpoint refresh
    tcp.rs               TCP connect and bind listener
    udp.rs               UDP associate/connect/bind framing
  service_address.rs     source-address discovery for host.lzcapp
```

Each module owns one protocol boundary. Generated code is never edited manually.

## Public API

### Generated protocols

The crate exposes packages using their protobuf hierarchy:

```rust
pub mod proto {
    pub mod common;
    pub mod localdevice;
    pub mod sys;
    pub mod dlna;
    pub mod baseos;
}
```

All generated client and server modules are public so the Rust SDK can replace the Go generated API for both callers and service implementations.

### Runtime credentials and connections

```rust
pub const CA_PATH: &str = "/lzcapp/run/certs/box.crt";
pub const APP_CERT_PATH: &str = "/lzcapp/run/certs/app.crt";
pub const APP_KEY_PATH: &str = "/lzcapp/run/certs/app.key";
pub const RUNTIME_SOCKET_PATH: &str = "/lzcapp/run/sys/lzc-apis.socket";
pub const PORTAL_SOCKET_PATH: &str = "/lzcapp/run/sys/portal-server.socket";

pub struct CredentialPaths { /* PathBuf fields */ }
pub struct ClientCredentials { /* redacted credential material */ }

pub async fn connect_api() -> Result<tonic::transport::Channel, Error>;
pub async fn connect_api_with(credentials: ClientCredentials) -> Result<tonic::transport::Channel, Error>;
```

`LZCAPP_API_GATEWAY_ADDRESS` retains the Go behavior: when present, connect to that address without mTLS; otherwise connect to the runtime Unix socket.

### ApiGateway

`ApiGateway::connect()` creates the runtime channel and exposes the same named service groups as the Go SDK:

- common: Box, Users, Devices, Permissions, PeripheralDevice, ISCSI, FileTransfer, Message;
- sys: PackageManager, AccessController, Btrfs, DirMonitor, TvOS, Version.

Rust field names use snake_case, and accessor methods are provided so adding future clients is non-breaking.

### DeviceProxy

`ApiGateway::device_proxy(url)` accepts only `http` or `https` URLs with a host and port. It builds an mTLS HTTP/2 channel to that device and exposes the Go DeviceProxy client set:

- Config;
- Device;
- Dialog;
- PhotoLibrary;
- Network;
- Permission;
- FileHandler;
- Rim;
- RemoteControl;
- Contact.

Every device RPC receives `lzc_dapi_auth_token`. A shared async token provider obtains the token using an unauthenticated mTLS PermissionManager client and refreshes it before its protobuf deadline. Concurrent refreshes are single-flight.

### Authentication compatibility

`request_auth_token` matches the Go implementation:

1. read the box certificate, application certificate, and private key;
2. read the application certificate subject serial number;
3. sign the raw serial bytes with Ed25519 or unprefixed RSA PKCS#1 v1.5;
4. send the original PEM box/app certificates and signature;
5. return a secret token plus protobuf deadline.

`AuthToken` redacts the token from `Debug` and exposes it only through `expose_secret()`.

`peer_application(&tonic::Request<T>)` returns application ID, box ID, and application domain from exactly one peer certificate.

`with_real_uid(request, uid)` inserts `X-Hc-User-Id`, matching the deprecated Go compatibility helper while documenting that it must not be used as an authorization boundary.

### Service status

The SDK exposes `ServiceStatusQuerier` and `ServiceStatusRegistry` with the Go state mapping:

- unknown;
- available;
- unavailable.

An unimplemented QueryServiceStatus RPC maps to `Error::ServiceStatusUnsupported`.

### HPortalSys and RemoteSocks

`HServerClient::connect()` opens the portal Unix socket. `RemoteNetstack::for_target(target)` requests a local or remote SOCKS endpoint through HPortalSys.

Rust cannot reproduce Go's `net.Conn`/`net.Listener` types exactly, so parity is capability-based:

- `connect_tcp` for SOCKS CONNECT;
- `connect_udp` for the LazyCat `0xe2` command;
- `bind_tcp` for SOCKS BIND and accepting the peer reply;
- `bind_udp` for LazyCat `0xe3`;
- `udp_associate` for standard SOCKS5 UDP associate;
- custom network/address encoding used by the Go RemoteSocks client.

The proxy endpoint is cached and refreshed once after a failed connection, matching the Go client.

### Service address

`query_service_address()` resolves `host.lzcapp`, probes each address with an unconnected UDP socket, and returns the selected source address. This provides the same result as the Go netlink route lookup without requiring Linux netlink in the public API.

## TLS Compatibility

The Go SDK sets a client certificate, includes the box CA, negotiates HTTP/2, and disables server-certificate verification. The Rust SDK preserves this compatibility mode for device endpoints through a narrowly scoped rustls verifier while still presenting the application client identity.

Runtime API connections keep their Go transport behavior:

- Unix socket by default;
- plaintext TCP when `LZCAPP_API_GATEWAY_ADDRESS` is set.

The insecure device verifier is not exported as a general-purpose option.

## Error and Secret Handling

The crate has one non-exhaustive `Error` enum with typed variants for URL, I/O, certificate, private key, signature, TLS, transport, gRPC status, metadata, SOCKS protocol, DNS, and unsupported operations.

Errors may include sanitized host/port and RPC status codes. They must never include:

- private-key bytes;
- certificate signature bytes;
- terminal tickets;
- device auth tokens;
- URL queries.

## Test Strategy

### Generated API coverage

- Buf lint/build/generate succeeds from a clean checkout.
- A compile test imports every generated package and representative client/server type.
- A manifest test verifies the vendored proto file list matches the official source list.

### Authentication and transport

- generated CA/server/app certificates are used by a tonic TLS server that requires and validates the app client certificate;
- HTTP/2 is negotiated;
- the server verifies Ed25519 and RSA signatures over the certificate serial;
- gRPC trailers and non-zero status codes are handled by tonic;
- token deadline caching and single-flight refresh are deterministic under paused Tokio time;
- token/debug/error strings are redacted.

### Gateway and compatibility helpers

- Unix connector and `LZCAPP_API_GATEWAY_ADDRESS` precedence;
- every ApiGateway accessor is backed by the shared channel;
- every DeviceProxy accessor uses authenticated transport;
- peer application extraction and real UID metadata;
- service-status mapping including Unimplemented.

### RemoteSocks

- byte-exact IPv4, IPv6, domain, and custom address codecs;
- authentication and each command code;
- TCP connect and two-stage bind;
- UDP associate/connect/bind framing;
- cached endpoint retry after connection failure;
- cancellation and timeouts close underlying sockets.

### Neko integration

- replace `src/device_api_auth.rs` with an adapter around `lzc_sdk::ApiGateway`/`DeviceProxy`;
- remove hand-written gRPC framing and direct certificate signing from Neko;
- keep the existing remote workspace/ticket/WebSocket behavior;
- add an integration test proving Neko consumes the SDK token path against the mTLS fixture.

## Delivery Sequence

1. Proto vendoring and Buf generation.
2. Core transport, credentials, and auth.
3. ApiGateway, DeviceProxy, metadata, peer identity, service status.
4. HPortalSys, service address, and RemoteSocks.
5. Documentation, examples, CI, and SDK release tag.
6. Neko SDK integration, regression verification, version bump, tag, and push.

Each phase ends in a compiling, tested commit. The official Go repository is never modified.
