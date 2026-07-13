# Full LazyCat Rust SDK Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a complete Rust SDK matching the generated and handwritten capabilities of `/home/czyt/code/ref/lzc-sdk/lang/go`, publish it from `/home/czyt/code/rust/lzc-sdk-rs`, and replace Neko Webshell's local device-auth implementation with the SDK.

**Architecture:** Vendor the official proto tree and generate committed prost/tonic code with Buf. Build focused handwritten modules around tonic channels for runtime Unix sockets, mTLS device proxies, token metadata, HPortalSys, RemoteSocks, and compatibility helpers. Neko becomes a consumer of the SDK and no longer implements certificate signing or raw gRPC framing.

**Tech Stack:** Rust 2024, Rust 1.88 MSRV, Buf 1.71+, protoc-gen-prost/protoc-gen-tonic/protoc-gen-prost-crate 0.5.0, prost 0.14.4, tonic/tonic-prost 0.14.6, Tokio, rustls 0.23, x509-parser, ed25519-dalek, rsa, secrecy.

## Global Constraints

- The official repository `/home/czyt/code/ref/lzc-sdk` is read-only.
- Copy all 41 official proto files and the referenced `lzc-baseos-protos` `hserver.proto`; record source revisions.
- Generate both tonic clients and tonic servers for every service.
- Generated code is committed; SDK consumers do not require Buf or protoc.
- Preserve Rust 1.88 compatibility so Neko can consume the crate.
- Never log private keys, signatures, tickets, certificate bodies, or auth tokens.
- Keep the insecure device-server verifier private and scoped only to Go SDK compatibility.
- Each task ends with tests and a focused commit.
- Work inline in the repositories because the user explicitly authorized direct implementation and push.

---

### Task 1: Crate scaffold, upstream proto sync, and Buf generation

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `buf.yaml`
- Create: `buf.gen.yaml`
- Create: `scripts/sync-protos.sh`
- Create: `scripts/generate.sh`
- Create: `proto/UPSTREAM.md`
- Create: `proto/{common,dlna,localdevice,sys,baseos}/**/*.proto`
- Create: `src/gen/*.rs`
- Create: `src/gen/mod.rs`
- Create: `src/lib.rs`
- Modify: `.gitignore`

**Interfaces:**
- Consumes: official proto source tree and Buf CLI.
- Produces: `lzc_sdk::proto::{common,localdevice,sys,dlna,containerd}` with all generated message, client, and server modules. HPortalSys is part of `proto::sys` because its proto package is `cloud.lazycat.apis.sys`.

- [ ] **Step 1: Write the generation manifest test**

Create `tests/proto_manifest.rs` that recursively compares `proto/{common,dlna,localdevice,sys}` with the official source by relative path and SHA-256, while allowing only documented `proto/baseos/hserver.proto` as an extra vendored dependency.

- [ ] **Step 2: Run the test to verify the empty crate fails**

Run: `cargo test --test proto_manifest`
Expected: FAIL because `Cargo.toml`, source modules, and vendored proto files do not exist.

- [ ] **Step 3: Add the crate manifest and public proto module**

Use package metadata:

```toml
[package]
name = "lzc-sdk"
version = "0.1.0"
edition = "2024"
rust-version = "1.88"
license = "MIT"
description = "Rust SDK for LazyCat application and device APIs"
repository = "https://github.com/lib-x/lzc-sdk-rs"

[features]
default = ["proto_full", "gateway", "remotesocks"]
gateway = []
remotesocks = ["gateway"]
## @@protoc_insertion_point(features)

[dependencies]
bytes = "1.10.1"
ed25519-dalek = { version = "2.2.0", features = ["pkcs8"] }
futures-util = "0.3.31"
http = "1.3.1"
pin-project-lite = "0.2.16"
prost = "0.14.4"
prost-types = "0.14.4"
rsa = { version = "0.9.9", features = ["pem"] }
rustls = { version = "0.23.41", default-features = false, features = ["ring", "std"] }
rustls-pemfile = "2.2.0"
secrecy = "0.10.3"
socket2 = { version = "0.6.1", features = ["all"] }
thiserror = "2.0.17"
tokio = { version = "1.48.0", features = ["fs", "io-util", "macros", "net", "rt-multi-thread", "sync", "time"] }
tokio-rustls = { version = "0.26.4", default-features = false, features = ["ring"] }
tonic = { version = "0.14.6", features = ["tls-ring"] }
tonic-prost = "0.14.6"
tower = { version = "0.5.2", features = ["util"] }
url = "2.5.7"
x509-parser = "0.18.1"

[dev-dependencies]
rcgen = { version = "0.14.8", features = ["aws_lc_rs", "pem", "x509-parser"] }
sha2 = "0.10.9"
tempfile = "3.23.0"
tokio-stream = { version = "0.1.17", features = ["net"] }
```

- [ ] **Step 4: Add deterministic proto sync**

`scripts/sync-protos.sh` accepts optional environment variables `LZC_SDK_GO_SOURCE` and `LZC_BASEOS_PROTO_SOURCE`, defaults to the provided local repositories/module cache, deletes only the owned vendored proto directories, copies exact files, and writes upstream commit/module revisions to `proto/UPSTREAM.md`. With `--check`, it stages the expected tree in a temporary directory and fails on any diff without modifying the repository.

- [ ] **Step 5: Add deterministic Buf generation**

`scripts/generate.sh` installs exactly version `0.5.0` of `protoc-gen-prost`, `protoc-gen-tonic`, and `protoc-gen-prost-crate` into `.tools/` when missing, then runs Buf format/lint/generate. `buf.gen.yaml` generates bytes as `bytes::Bytes`, tonic clients and servers, and a feature-aware `src/gen/mod.rs` without generating a second Cargo manifest. With `--check`, it generates into a temporary directory and compares it with `src/gen` without modifying tracked files.

- [ ] **Step 6: Sync and generate**

Run:

```bash
./scripts/sync-protos.sh
./scripts/generate.sh
```

Expected: 42 proto files are present (41 official plus HPortalSys), Buf lint succeeds, and generated Rust modules compile.

- [ ] **Step 7: Run generation and manifest checks**

Run:

```bash
cargo test --test proto_manifest
cargo check --all-features
git diff --check
```

Expected: PASS; a second `./scripts/generate.sh` produces no diff.

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml Cargo.lock rust-toolchain.toml buf.yaml buf.gen.yaml scripts proto src/gen src/lib.rs tests/proto_manifest.rs .gitignore
git commit -m "feat: generate complete LazyCat Rust APIs"
```

### Task 2: Credentials, channels, peer identity, and metadata

**Files:**
- Create: `src/error.rs`
- Create: `src/credentials.rs`
- Create: `src/connection.rs`
- Create: `src/peer.rs`
- Create: `src/metadata.rs`
- Create: `tests/connection.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `CredentialPaths`, `ClientCredentials`, `connect_api`, `connect_api_with`, `Application`, `peer_application`, and `with_real_uid`.
- Consumes: generated tonic services and fixed runtime paths.

- [ ] **Step 1: Write failing credential and connection tests**

Cover default paths, injected temp paths, redacted Debug, Unix-socket connection, `LZCAPP_API_GATEWAY_ADDRESS` precedence, and exact `X-Hc-User-Id` metadata spelling.

- [ ] **Step 2: Write failing peer identity tests**

Create a tonic request with TLS connect info containing one certificate and assert `Application { app_id, box_id, app_domain }`. Assert zero or multiple certificates return typed authentication errors.

- [ ] **Step 3: Run focused tests and confirm failure**

Run: `cargo test --test connection`
Expected: FAIL because the public connection and identity APIs are absent.

- [ ] **Step 4: Implement typed errors and credential loading**

Add a `#[non_exhaustive] Error` enum and `CredentialPaths::runtime()`. `ClientCredentials::load` stores PEM bytes in private fields, parses certificate/key material once, and exposes only methods needed to build tonic identities and auth requests.

- [ ] **Step 5: Implement runtime channels**

Use `Endpoint::connect_with_connector` and `tokio::net::UnixStream` for `/lzcapp/run/sys/lzc-apis.socket`. When `LZCAPP_API_GATEWAY_ADDRESS` is non-empty, normalize it to an HTTP URI and connect over plaintext TCP, matching the Go override behavior.

- [ ] **Step 6: Implement peer and metadata compatibility**

Parse the single peer certificate from tonic TLS extensions into `Application`. Add `with_real_uid<T>(request: &mut tonic::Request<T>, uid: &str) -> Result<(), Error>` that is a no-op for an empty UID.

- [ ] **Step 7: Verify**

Run:

```bash
cargo test --test connection
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings
```

Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add src/error.rs src/credentials.rs src/connection.rs src/peer.rs src/metadata.rs src/lib.rs tests/connection.rs
git commit -m "feat: add LazyCat runtime credentials and channels"
```

### Task 3: Device authentication, token cache, and authenticated gRPC service

**Files:**
- Create: `src/auth.rs`
- Create: `src/device_transport.rs`
- Create: `tests/device_auth.rs`
- Create: `tests/fixtures.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `AuthToken`, `TokenProvider`, and `AuthenticatedService<Channel>`.
- Consumes: `ClientCredentials` and generated PermissionManager client.

- [ ] **Step 1: Write the full mTLS failing test**

Generate a CA, server certificate, Ed25519 app certificate, and application key. Start a tonic TLS server requiring the application client certificate. The PermissionManager fixture verifies the request certificate serial signature and returns a token with protobuf deadline.

- [ ] **Step 2: Add RSA compatibility and gRPC-status tests**

Generate a PKCS#1 RSA key/certificate fixture and verify raw unprefixed PKCS#1 v1.5 signing. Return `Unauthenticated` from the server and assert the SDK preserves the tonic status without attempting to decode raw frames.

- [ ] **Step 3: Add deterministic cache tests**

With Tokio time paused, assert concurrent callers share one refresh, cached tokens are reused before deadline, and refresh happens before expiration using a fixed safety margin.

- [ ] **Step 4: Run tests and confirm failure**

Run: `cargo test --test device_auth`
Expected: FAIL because the token provider and authenticated service do not exist.

- [ ] **Step 5: Implement Go-compatible auth request signing**

Read the certificate subject serial, sign raw bytes with Ed25519 or RSA PKCS#1 v1.5 unprefixed mode, and send original PEM certificate bytes through the generated PermissionManager client.

- [ ] **Step 6: Implement device TLS compatibility**

Build a tonic endpoint with application identity, HTTP/2 assumption, and a private rustls verifier that matches Go's scoped `InsecureSkipVerify` behavior. Reject non-HTTP(S) URLs and strip query/fragment data from the gRPC endpoint.

- [ ] **Step 7: Implement authenticated service middleware**

Wrap a cloned tonic Channel in a Tower service whose `call` future resolves a token through `TokenProvider`, inserts `lzc_dapi_auth_token`, and dispatches the original HTTP/2 request. Never expose the token through Debug or errors.

- [ ] **Step 8: Verify**

Run:

```bash
cargo test --test device_auth
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings
```

Expected: PASS, including mandatory client-certificate validation.

- [ ] **Step 9: Commit**

```bash
git add src/auth.rs src/device_transport.rs src/lib.rs tests/device_auth.rs tests/fixtures.rs
git commit -m "feat: add authenticated LazyCat device transport"
```

### Task 4: Complete ApiGateway, DeviceProxy, and service-status wrappers

**Files:**
- Create: `src/gateway.rs`
- Create: `src/service_status.rs`
- Create: `tests/gateway.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `ApiGateway`, `DeviceProxy`, `ServiceStatusQuerier`, `ServiceStatusRegistry`, `ServiceStatus`, and `ServiceState`.
- Consumes: generated clients, runtime channel, and authenticated device service.

- [ ] **Step 1: Write compile-time gateway coverage tests**

Assert `ApiGateway` exposes Box, Users, Devices, Permissions, PeripheralDevice, ISCSI, FileTransfer, PackageManager, AccessControler, Btrfs, DirMonitor, Message, TvOS, and Version clients. Assert `DeviceProxy` exposes Config, Device, Dialog, PhotoLibrary, Network, Permission, FileHandler, Rim, RemoteControl, and Contact clients.

- [ ] **Step 2: Write service-status behavior tests**

Port the three official Go tests for available/unavailable/unknown state mapping, `QueryMany`, and `Unimplemented` conversion.

- [ ] **Step 3: Run tests and confirm failure**

Run: `cargo test --test gateway`
Expected: FAIL because wrapper types do not exist.

- [ ] **Step 4: Implement ApiGateway composition**

Construct every client from clones of one runtime Channel. Expose snake_case public accessors and keep the channel privately cloneable for future services.

- [ ] **Step 5: Implement DeviceProxy composition**

Construct every device client from clones of one `AuthenticatedService<Channel>`. Add `get_auth_token`, `status()`, and cheap Clone semantics; channel shutdown follows tonic Channel ownership rather than an explicit close method.

- [ ] **Step 6: Implement service-status wrappers**

Map the generated local service enum to the public `ServiceState`, preserve reason strings, deduplicate batch names, and return `Error::ServiceStatusUnsupported` for `Code::Unimplemented`.

- [ ] **Step 7: Verify and commit**

Run all tests/clippy, then:

```bash
git add src/gateway.rs src/service_status.rs src/lib.rs tests/gateway.rs
git commit -m "feat: add complete LazyCat gateway wrappers"
```

### Task 5: HPortalSys and service-address compatibility

**Files:**
- Create: `src/hserver.rs`
- Create: `src/service_address.rs`
- Create: `tests/hserver.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `HServerClient`, `RemoteLocation`, `query_service_address`, and a RemoteSocks endpoint resolver.
- Consumes: generated baseos HPortalSys client.

- [ ] **Step 1: Write failing portal and address tests**

Test Unix-socket HPortalSys connection, local versus remote `RemoteSocksRequest` mapping, endpoint parsing, DNS candidate iteration, IPv4/IPv6 source-address selection, and the no-route error.

- [ ] **Step 2: Implement HServerClient**

Connect to `/lzcapp/run/sys/portal-server.socket` over plaintext tonic UDS. Expose the generated client plus `remote_socks_endpoint(target)` that selects Local for an empty target and Remote otherwise.

- [ ] **Step 3: Implement source-address discovery**

Resolve `host.lzcapp`; for each result bind an unspecified UDP socket of the same family, connect it to the result, and return `local_addr().ip()`. Preserve the last meaningful error if no candidate succeeds.

- [ ] **Step 4: Verify and commit**

Run focused/all tests and clippy, then commit as `feat: add HPortalSys and service address helpers`.

### Task 6: RemoteSocks address codec and TCP operations

**Files:**
- Create: `src/remotesocks/mod.rs`
- Create: `src/remotesocks/address.rs`
- Create: `src/remotesocks/client.rs`
- Create: `src/remotesocks/tcp.rs`
- Create: `tests/remotesocks_tcp.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produces: `RemoteNetstack`, `RemoteTcpStream`, `RemoteTcpListener`, `SocksAddress`, and custom network/address support.
- Consumes: `HServerClient` endpoint resolver.

- [ ] **Step 1: Port byte-level address tests**

Cover IPv4, IPv6, domain, custom network, invalid lengths, invalid UTF-8, and port encoding against byte fixtures derived from the Go `spec` package.

- [ ] **Step 2: Port SOCKS authentication/command tests**

Use an in-process fake proxy to assert version 5, NoAuth negotiation, CONNECT (`0x01`), BIND (`0x02`), custom address encoding, reply errors, and cancellation timeouts.

- [ ] **Step 3: Implement endpoint caching and retry**

Cache parsed `socks5`/`socks5h` host:port. On connection failure, resolve once without cache and retry only if the endpoint changed, matching Go behavior.

- [ ] **Step 4: Implement TCP connect and two-stage bind**

`connect_tcp` returns a Tokio stream after CONNECT success. `bind_tcp` returns a listener after the first BIND reply; `accept` waits for the second reply and returns a stream plus peer address.

- [ ] **Step 5: Verify and commit**

Run focused/all tests and clippy, then commit as `feat: add RemoteSocks TCP support`.

### Task 7: RemoteSocks UDP and packet operations

**Files:**
- Create: `src/remotesocks/udp.rs`
- Create: `tests/remotesocks_udp.rs`
- Modify: `src/remotesocks/mod.rs`
- Modify: `src/remotesocks/client.rs`

**Interfaces:**
- Produces: `RemoteUdpSocket` and public `udp_associate`, `connect_udp`, and `bind_udp` methods.
- Consumes: SOCKS codec/client from Task 6.

- [ ] **Step 1: Port UDP frame tests**

Cover standard SOCKS UDP headers, source filtering, IPv4/IPv6/domain targets, malformed prefixes, the LazyCat ConnectUDP command `0xe2`, and BindUDP command `0xe3`.

- [ ] **Step 2: Implement standard UDP associate**

Keep the TCP control stream alive, bind a local UDP socket, wrap outgoing/incoming SOCKS5 UDP frames, and close the packet socket when the control stream ends.

- [ ] **Step 3: Implement connected and bound UDP extensions**

For `0xe2`, wrap the returned stream as a connected packet transport with fixed remote address. For `0xe3`, expose send/receive operations with the bound address returned by the proxy.

- [ ] **Step 4: Verify and commit**

Run focused/all tests and clippy, then commit as `feat: complete RemoteSocks UDP support`.

### Task 8: SDK documentation, examples, CI, and release

**Files:**
- Create: `README.md`
- Create: `examples/api_gateway.rs`
- Create: `examples/device_proxy.rs`
- Create: `.github/workflows/ci.yml`
- Create: `CHANGELOG.md`
- Modify: `Cargo.toml`

**Interfaces:**
- Produces: documented public SDK version `0.1.0` and reproducible CI.

- [ ] **Step 1: Write public usage examples**

Document runtime gateway creation, device proxy creation, token retrieval, service-status queries, peer identity, real UID metadata, HPortalSys, and RemoteSocks TCP/UDP use.

- [ ] **Step 2: Add CI gates**

CI runs proto sync verification without modifying files, Buf format/lint/generate determinism, rustfmt, clippy `-D warnings`, all-feature tests, docs, and `cargo package --allow-dirty`.

- [ ] **Step 3: Run release verification**

Run:

```bash
./scripts/sync-protos.sh --check
./scripts/generate.sh --check
cargo fmt --all -- --check
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
cargo package --allow-dirty
```

Expected: PASS.

- [ ] **Step 4: Commit, tag, and push SDK**

```bash
git add README.md CHANGELOG.md examples .github Cargo.toml Cargo.lock
git commit -m "release: prepare lzc-sdk-rs v0.1.0"
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

### Task 9: Replace Neko's hand-written device authentication with the SDK

**Files:**
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/Cargo.toml`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/Cargo.lock`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/src/client_terminal.rs`
- Delete: `/home/czyt/code/rust/lazycat-neko-webshell/src/device_api_auth.rs`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/src/main.rs`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/src/client_terminal.rs` tests module with the SDK source-boundary and adapter regression tests.

**Interfaces:**
- Consumes: released/path `lzc-sdk` `ApiGateway::device_proxy` and `DeviceProxy::get_auth_token`.
- Produces: the existing Neko `SecretString` token path with no local TLS/gRPC/signature implementation.

- [ ] **Step 1: Add a failing source-boundary regression test**

Assert Neko depends on `lzc-sdk`, has no raw RequestAuthToken path constant, no `encode_grpc_request`, no `http2_prior_knowledge`, and no local certificate signing implementation.

- [ ] **Step 2: Add SDK dependency and adapter**

Use the local SDK path while developing. Replace `device_api_auth::resolve_auth_token` with SDK construction and token retrieval. Map SDK errors to the existing sanitized `ClientTerminalError` boundary.

- [ ] **Step 3: Remove obsolete dependencies and code**

Delete direct auth-only dependencies such as `h2`, local `tokio-rustls` test support, manual x509/signing dependencies if no other Neko module uses them, and remove `mod device_api_auth`.

- [ ] **Step 4: Run Neko verification**

Run:

```bash
npm test
npm run typecheck
cargo test --locked
cargo clippy --all-targets -- -D warnings
./scripts/build-release.sh
lzc-cli project lint --release
```

Expected: PASS; the release remains one static musl provider binary.

- [ ] **Step 5: Runtime verification**

Install/deploy the release, request provider version, refresh the instance list, select the RUNNING remote client, confirm `/api/workspace?name=client:` returns 200, and verify terminal WebSocket attach and live input/output.

### Task 10: Neko release, memory, and push

**Files:**
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/Cargo.toml`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/Cargo.lock`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/package.json`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/package-lock.json`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/package.yml`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/README.md`
- Modify: `/home/czyt/code/rust/lazycat-neko-webshell/README.en.md`

- [ ] **Step 1: Bump Neko to 0.5.27 consistently**

Set the Neko version to `0.5.27` in `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`, `package.yml`, `README.md`, and `README.en.md`; verify no stale `0.5.26` current-version metadata remains.

- [ ] **Step 2: Run final verification from a clean build**

Repeat frontend tests, Rust tests/clippy, musl release, LPK lint, and real remote terminal runtime check.

- [ ] **Step 3: Commit with regression context**

The commit message must state that the previous HTTP/2-only repair recurred because Neko duplicated the SDK protocol and that this change moves the boundary to the tested Rust SDK.

- [ ] **Step 4: Tag and push**

Create the new Neko version tag, push `main`, and push the tag. Confirm local and remote refs point to the release commit.

- [ ] **Step 5: Save the durable learning**

Record that LazyCat device authentication must use the shared SDK and that HTTP/2-only tests without mandatory client certificate validation are insufficient regression evidence.
