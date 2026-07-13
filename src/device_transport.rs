use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use http::header::{HeaderName, HeaderValue};
use http::{Request, Response};
use tonic::body::Body;
use tonic::transport::{Channel, Endpoint};
use tower::{Service, ServiceExt as _};
use url::{Host, Url};

use crate::connection::compatibility_server_verifier;
use crate::{ClientCredentials, Error, TokenProvider};

const AUTH_TOKEN_HEADER: HeaderName = HeaderName::from_static("lzc_dapi_auth_token");

pub(crate) async fn connect_device_channel(
    api_url: &str,
    credentials: &ClientCredentials,
) -> Result<Channel, Error> {
    let url = Url::parse(api_url).map_err(|_| Error::InvalidDeviceUrl)?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(Error::InvalidDeviceUrl);
    }
    let host = url.host().ok_or(Error::InvalidDeviceUrl)?;
    let domain_name = url.host_str().ok_or(Error::InvalidDeviceUrl)?;
    let port = url.port_or_known_default().ok_or(Error::InvalidDeviceUrl)?;
    let authority_host = match host {
        Host::Ipv6(address) => format!("[{address}]"),
        Host::Ipv4(address) => address.to_string(),
        Host::Domain(address) => address.to_owned(),
    };
    let endpoint = Endpoint::from_shared(format!("https://{authority_host}:{port}"))?
        .tls_config_with_verifier(
            credentials.tls_config(domain_name),
            compatibility_server_verifier(),
        )?;
    Ok(endpoint.connect().await?)
}

/// Tower service that injects a cached device auth token into every gRPC call.
#[derive(Clone)]
pub struct AuthenticatedService<S> {
    inner: S,
    provider: TokenProvider,
}

impl<S> AuthenticatedService<S> {
    /// Wrap an inner gRPC service with a token provider.
    #[must_use]
    pub fn new(inner: S, provider: TokenProvider) -> Self {
        Self { inner, provider }
    }
}

impl<S> fmt::Debug for AuthenticatedService<S> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticatedService")
            .field("provider", &self.provider)
            .finish_non_exhaustive()
    }
}

impl Service<Request<Body>> for AuthenticatedService<Channel> {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(context).map_err(Error::Transport)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let provider = self.provider.clone();
        let inner = self.inner.clone();
        Box::pin(async move {
            let token = provider.token().await?;
            let value = HeaderValue::try_from(token.expose_secret())
                .map_err(|_| Error::InvalidAuthTokenMetadata)?;
            request.headers_mut().insert(AUTH_TOKEN_HEADER, value);
            inner.oneshot(request).await.map_err(Error::Transport)
        })
    }
}
