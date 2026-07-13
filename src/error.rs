use std::io;
use std::path::PathBuf;

use thiserror::Error;
use tonic::metadata::errors::InvalidMetadataValue;

/// Errors returned by the `LazyCat` SDK compatibility layer.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// A runtime credential could not be read.
    #[error("failed to read credential at {path}: {source}")]
    CredentialRead {
        /// Credential path that failed.
        path: PathBuf,
        /// Underlying I/O failure.
        #[source]
        source: io::Error,
    },

    /// A certificate file was empty, malformed, or unsupported.
    #[error("invalid certificate at {path}: {reason}")]
    InvalidCertificate {
        /// Certificate path that failed validation.
        path: PathBuf,
        /// Sanitized validation reason.
        reason: &'static str,
    },

    /// A private key file was empty, malformed, unsupported, or mismatched.
    #[error("invalid private key at {path}: {reason}")]
    InvalidPrivateKey {
        /// Private key path that failed validation.
        path: PathBuf,
        /// Sanitized validation reason.
        reason: &'static str,
    },

    /// A tonic channel could not be configured or connected.
    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    /// A gRPC method returned a status response.
    #[error("gRPC request failed: {0}")]
    GrpcStatus(#[from] tonic::Status),

    /// The runtime gateway override is not a host or HTTP(S) URI.
    #[error("invalid runtime API gateway address")]
    InvalidGatewayAddress,

    /// A device API URL is malformed or uses an unsupported scheme.
    #[error("invalid LazyCat device API URL")]
    InvalidDeviceUrl,

    /// The application private key cannot produce Go-compatible auth signatures.
    #[error("unsupported application private key for device authentication")]
    UnsupportedAuthSigningKey,

    /// Signing the application identity failed.
    #[error("failed to sign the application identity")]
    AuthSigning,

    /// The device returned a token without an expiry deadline.
    #[error("device auth token response has no deadline")]
    MissingTokenDeadline,

    /// The device returned an invalid token expiry deadline.
    #[error("device auth token response has an invalid deadline")]
    InvalidTokenDeadline,

    /// An auth token cannot be encoded as gRPC metadata.
    #[error("device auth token is not a valid gRPC metadata value")]
    InvalidAuthTokenMetadata,

    /// The device does not implement the optional service-status API.
    #[error("device service-status query is unsupported")]
    ServiceStatusUnsupported,

    /// `HPortalSys` returned a malformed or unsupported `RemoteSocks` URL.
    #[error("HPortalSys returned an invalid RemoteSocks endpoint")]
    InvalidRemoteSocksEndpoint,

    /// Resolving the `LazyCat` host-service address failed.
    #[error("failed to resolve host.lzcapp: {source}")]
    ServiceAddressLookup {
        /// Underlying DNS lookup failure.
        #[source]
        source: io::Error,
    },

    /// No resolved host-service candidate had a usable local route.
    #[error("no usable route to host.lzcapp")]
    ServiceAddressNoRoute {
        /// Last route-selection failure, when at least one candidate was tried.
        #[source]
        source: Option<io::Error>,
    },

    /// A SOCKS address is malformed or cannot be represented on the wire.
    #[error("invalid SOCKS address")]
    InvalidSocksAddress,

    /// A SOCKS domain or custom address exceeds the one-byte wire length.
    #[error("SOCKS address is longer than 255 bytes")]
    SocksAddressTooLong,

    /// A SOCKS peer used an address type not defined by RFC 1928.
    #[error("unsupported SOCKS address type {address_type:#04x}")]
    UnsupportedSocksAddressType {
        /// Unrecognized wire address type.
        address_type: u8,
    },

    /// A network operation used by `RemoteSocks` failed.
    #[error("RemoteSocks network error: {0}")]
    RemoteSocksIo(#[from] io::Error),

    /// Connecting to the `RemoteSocks` proxy exceeded the configured deadline.
    #[error("RemoteSocks connection timed out")]
    RemoteSocksTimeout,

    /// A SOCKS peer replied with an unexpected protocol version.
    #[error("unexpected SOCKS protocol version {version}")]
    UnexpectedSocksVersion {
        /// Version byte received from the peer.
        version: u8,
    },

    /// A SOCKS peer rejected every offered authentication method.
    #[error("SOCKS proxy rejected all authentication methods")]
    SocksNoAcceptableAuthentication,

    /// A SOCKS peer selected an authentication method this SDK does not support.
    #[error("unsupported SOCKS authentication method {method:#04x}")]
    UnsupportedSocksAuthentication {
        /// Authentication method selected by the peer.
        method: u8,
    },

    /// A SOCKS reply used a non-zero reserved byte.
    #[error("invalid SOCKS reserved byte {reserved:#04x}")]
    InvalidSocksReservedByte {
        /// Reserved byte received from the peer.
        reserved: u8,
    },

    /// A SOCKS command failed with the provided RFC 1928 reply code.
    #[error("SOCKS proxy command failed with reply code {code:#04x}")]
    SocksReply {
        /// SOCKS reply code returned by the peer.
        code: u8,
    },

    /// A SOCKS UDP datagram had invalid reserved or framing bytes.
    #[error("invalid SOCKS UDP datagram header")]
    InvalidSocksUdpHeader,

    /// SOCKS UDP fragmentation is not supported by this SDK.
    #[error("SOCKS UDP fragmentation is unsupported (fragment {fragment})")]
    UnsupportedSocksUdpFragment {
        /// Non-zero RFC 1928 fragment byte.
        fragment: u8,
    },

    /// A UDP payload cannot fit in one transport frame.
    #[error("RemoteSocks UDP payload is too large")]
    SocksUdpPayloadTooLarge,

    /// An operation requiring a fixed UDP peer was used on a bound socket.
    #[error("RemoteSocks UDP socket has no connected peer")]
    SocksUdpNotConnected,

    /// The `RemoteSocks` TCP control stream has closed.
    #[error("RemoteSocks UDP control stream closed")]
    RemoteSocksControlClosed,

    /// A SOCKS address cannot be used as a concrete UDP socket address.
    #[error("invalid RemoteSocks UDP socket address")]
    InvalidSocksUdpAddress,

    /// A value cannot be represented as gRPC metadata.
    #[error("invalid gRPC metadata value")]
    InvalidMetadataValue(#[source] InvalidMetadataValue),

    /// The request was not received over authenticated TLS.
    #[error("peer is not authenticated with a TLS client certificate")]
    UnauthenticatedPeer,

    /// `LazyCat` application identity requires exactly one peer certificate.
    #[error("expected exactly one peer certificate, got {count}")]
    InvalidPeerCertificateCount {
        /// Number of certificates presented by the peer.
        count: usize,
    },

    /// The peer certificate could not be decoded as X.509.
    #[error("invalid peer certificate")]
    InvalidPeerCertificate,
}
