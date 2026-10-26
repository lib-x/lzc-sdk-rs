# Changelog

All notable changes to this project are documented in this file.

## [0.2.0] - 2026-10-26

### Added

- HClient management bindings and the `ApiGateway::hclients()` accessor.
- Local IP address enumeration, multi-fan configuration, and updated HPortalSys
  origin, proxy, and advertised-address management APIs from upstream.
- Pinned upstream Git submodules and daily/manual GitHub Actions synchronization
  that regenerates bindings and builds a separate update branch.

### Changed

- Updated official SDK protocols to `f5bd0470167da6df10f5ebb396525d9a12706d7c`
  and BaseOS protocols to `a8715f13eac24edd3d307d50bde42abc6800b728`.
- Protobuf synchronization reads local submodule checkouts and records their
  full commit IDs; published crates remain independent of those checkouts.

### Breaking changes

- `NetworkManager::ws_discovery` now returns a stream of `WSDiscoveryDevice`
  messages instead of a `WSDiscoveryResponse` containing all devices.
- Removed upstream `Client::login_tv`, `TVLoginInfo`,
  `NetworkManager::native_http_request`, and its request/response types.
- Removed upstream HPortalSys `set_relay` and `dump_peers` RPCs and the
  `RegisterBoxServiceRequest::extra_info` field; renamed
  `ChangeTrustEndDeviceRequest::cid` to `peer_id`.

## [0.1.1] - 2026-07-14

### Changed

- Updated the packaged README to install `lzc-sdk` from crates.io and link to
  crates.io, docs.rs, and the GitHub source repository.

## [0.1.0] - 2026-07-13

### Added

- Vendored 41 official LazyCat SDK protobuf definitions and the BaseOS
  `HPortalSys` definition in Buf package-directory layout.
- Committed prost/tonic messages, clients, and servers for every vendored
  service.
- Runtime Unix-socket mTLS connections, credential loading, peer application
  identity, and real-user metadata compatibility.
- Complete `ApiGateway` and authenticated `DeviceProxy` client composition.
- Ed25519 and RSA-compatible device token acquisition with deadline-aware,
  single-flight caching.
- Device service-status helpers with availability mapping and compatibility
  handling for older devices.
- HPortalSys, RemoteSocks endpoint discovery, and `host.lzcapp` source-address
  selection.
- RemoteSocks address/custom-network codecs, TCP CONNECT/BIND, standard UDP
  ASSOCIATE, and LazyCat ConnectUDP/BindUDP extensions.
- Deterministic protobuf sync, Buf generation, manifests, tests, examples, and
  CI release gates.
