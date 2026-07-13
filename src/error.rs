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

    /// The runtime gateway override is not a host or HTTP(S) URI.
    #[error("invalid runtime API gateway address")]
    InvalidGatewayAddress,

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
