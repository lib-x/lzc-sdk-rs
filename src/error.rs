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
