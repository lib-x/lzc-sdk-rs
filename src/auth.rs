use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use bytes::Bytes;
use secrecy::{ExposeSecret as _, SecretString};
use tokio::sync::Mutex;
use tokio::time::Instant;
use tonic::transport::Channel;

use crate::device_transport::connect_device_channel;
use crate::proto::localdevice::permission_manager_client::PermissionManagerClient;
use crate::proto::localdevice::{RequestAuthTokenRequest, RequestAuthTokenResponse};
use crate::{AuthenticatedService, ClientCredentials, Error};

const TOKEN_REFRESH_MARGIN: Duration = Duration::from_secs(30);

/// Secret device API token and its server-provided deadline.
pub struct AuthToken {
    token: SecretString,
    deadline: SystemTime,
    refresh_at: Instant,
}

impl AuthToken {
    /// Expose the token for explicit integration at an authentication boundary.
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        self.token.expose_secret()
    }

    /// Return the token expiry deadline supplied by the device.
    #[must_use]
    pub fn deadline(&self) -> SystemTime {
        self.deadline
    }

    fn should_refresh(&self, now: Instant) -> bool {
        now >= self.refresh_at
    }

    fn from_response(response: RequestAuthTokenResponse) -> Result<Self, Error> {
        let deadline = response.deadline.ok_or(Error::MissingTokenDeadline)?;
        let deadline = SystemTime::try_from(deadline).map_err(|_| Error::InvalidTokenDeadline)?;
        let now_system = SystemTime::now();
        let now_instant = Instant::now();
        let lifetime = deadline.duration_since(now_system).unwrap_or_default();
        let refresh_at = now_instant + lifetime.saturating_sub(TOKEN_REFRESH_MARGIN);
        Ok(Self {
            token: response.token.into(),
            deadline,
            refresh_at,
        })
    }
}

impl fmt::Debug for AuthToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthToken")
            .field("token", &"<redacted>")
            .field("deadline", &self.deadline)
            .finish_non_exhaustive()
    }
}

/// Single-flight provider for cached `LazyCat` device API tokens.
#[derive(Clone)]
pub struct TokenProvider {
    inner: Arc<TokenProviderInner>,
}

struct TokenProviderInner {
    channel: Channel,
    credentials: ClientCredentials,
    cached: Mutex<Option<Arc<AuthToken>>>,
}

impl TokenProvider {
    /// Connect an mTLS device channel and create its token provider.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid or the device channel cannot be
    /// established.
    pub async fn connect(api_url: &str, credentials: ClientCredentials) -> Result<Self, Error> {
        let channel = connect_device_channel(api_url, &credentials).await?;
        Ok(Self::new(channel, credentials))
    }

    /// Create a provider around an already connected raw device channel.
    #[must_use]
    pub fn new(channel: Channel, credentials: ClientCredentials) -> Self {
        Self {
            inner: Arc::new(TokenProviderInner {
                channel,
                credentials,
                cached: Mutex::new(None),
            }),
        }
    }

    /// Return a cached token or refresh it once for all concurrent callers.
    ///
    /// # Errors
    ///
    /// Returns an error when request signing or the `PermissionManager` RPC fails.
    pub async fn token(&self) -> Result<Arc<AuthToken>, Error> {
        let mut cached = self.inner.cached.lock().await;
        if let Some(token) = cached.as_ref() {
            if !token.should_refresh(Instant::now()) {
                return Ok(Arc::clone(token));
            }
        }

        let token = Arc::new(
            request_auth_token(self.inner.channel.clone(), &self.inner.credentials).await?,
        );
        *cached = Some(Arc::clone(&token));
        Ok(token)
    }

    /// Wrap this provider's channel with automatic auth-token metadata.
    #[must_use]
    pub fn authenticated_service(&self) -> AuthenticatedService<Channel> {
        AuthenticatedService::new(self.inner.channel.clone(), self.clone())
    }
}

impl fmt::Debug for TokenProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenProvider")
            .field("credentials", &self.inner.credentials)
            .field("cached_token", &"<redacted>")
            .finish_non_exhaustive()
    }
}

/// Request a fresh auth token over an unauthenticated mTLS channel.
///
/// # Errors
///
/// Returns an error when the identity cannot be signed, the RPC returns a
/// status, or the response deadline is missing or invalid.
pub async fn request_auth_token(
    channel: Channel,
    credentials: &ClientCredentials,
) -> Result<AuthToken, Error> {
    let material = credentials.auth_request_material()?;
    let request = RequestAuthTokenRequest {
        box_cert: Bytes::copy_from_slice(material.box_certificate),
        app_cert: Bytes::copy_from_slice(material.application_certificate),
        signature: Bytes::from(material.signature),
    };
    let response = PermissionManagerClient::new(channel)
        .request_auth_token(request)
        .await?
        .into_inner();
    AuthToken::from_response(response)
}
