# Changelog

All notable changes to this project are documented in this file.

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
