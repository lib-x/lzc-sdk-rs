use tonic::Request;
use tonic::metadata::MetadataValue;

use crate::Error;

/// Go-compatible spelling of the deprecated real-user metadata field.
pub const REAL_UID_METADATA_KEY: &str = "X-Hc-User-Id";
const REAL_UID_WIRE_KEY: &str = "x-hc-user-id";

/// Append the deprecated real-user metadata field to a gRPC request.
///
/// An empty UID is a no-op. This compatibility metadata must never be treated
/// as an authorization boundary because callers can forge it.
///
/// # Errors
///
/// Returns an error when `uid` contains bytes that are invalid in an ASCII
/// gRPC metadata value.
pub fn with_real_uid<T>(request: &mut Request<T>, uid: &str) -> Result<(), Error> {
    if uid.is_empty() {
        return Ok(());
    }
    let value = MetadataValue::try_from(uid).map_err(Error::InvalidMetadataValue)?;
    request.metadata_mut().append(REAL_UID_WIRE_KEY, value);
    Ok(())
}
