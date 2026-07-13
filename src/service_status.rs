use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use tonic::Code;
use tonic::transport::Channel;

use crate::proto::common::file_handler_server;
use crate::proto::localdevice::device_service_client::DeviceServiceClient;
use crate::proto::localdevice::photo_library_server;
use crate::proto::localdevice::{LocalServiceState, LocalServiceStatus, QueryServiceStatusRequest};
use crate::{AuthenticatedService, Error};

type DeviceServiceTransport = AuthenticatedService<Channel>;

/// Public service availability state used across device API versions.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ServiceState {
    #[default]
    Unknown,
    Available,
    Unavailable,
}

impl ServiceState {
    /// Return the Go-compatible lowercase state name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Available => "available",
            Self::Unavailable => "unavailable",
        }
    }
}

impl fmt::Display for ServiceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Availability and optional device-provided reason for a local service.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ServiceStatus {
    pub state: ServiceState,
    pub reason: String,
}

/// Query helper bound to one generated service name.
#[derive(Clone, Debug)]
pub struct ServiceStatusQuerier {
    registry: ServiceStatusRegistry,
    service_name: Arc<str>,
}

impl ServiceStatusQuerier {
    fn new(registry: ServiceStatusRegistry, service_name: &'static str) -> Self {
        Self {
            registry,
            service_name: service_name.into(),
        }
    }

    /// Query this helper's bound service.
    ///
    /// # Errors
    ///
    /// Returns a typed unsupported error or the underlying gRPC status.
    pub async fn query(&self) -> Result<ServiceStatus, Error> {
        self.registry.query(self.service_name.as_ref()).await
    }
}

/// Query helper for arbitrary local service names.
#[derive(Clone, Debug)]
pub struct ServiceStatusRegistry {
    service: DeviceServiceTransport,
}

impl ServiceStatusRegistry {
    pub(crate) fn new(service: DeviceServiceTransport) -> Self {
        Self { service }
    }

    /// Query one service, returning `Unknown` when the device omits it.
    ///
    /// # Errors
    ///
    /// Returns a typed unsupported error or the underlying gRPC status.
    pub async fn query(&self, service_name: &str) -> Result<ServiceStatus, Error> {
        let mut statuses = self.query_many([service_name]).await?;
        Ok(statuses.remove(service_name).unwrap_or_default())
    }

    /// Query several services in one RPC after removing duplicate names.
    ///
    /// # Errors
    ///
    /// Returns a typed unsupported error or the underlying gRPC status.
    pub async fn query_many<I, S>(
        &self,
        service_names: I,
    ) -> Result<HashMap<String, ServiceStatus>, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut seen = HashSet::new();
        let service_name = service_names
            .into_iter()
            .filter_map(|name| {
                let name = name.as_ref();
                if seen.insert(name.to_owned()) {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect();
        let response = DeviceServiceClient::new(self.service.clone())
            .query_service_status(QueryServiceStatusRequest { service_name })
            .await
            .map_err(|status| {
                if status.code() == Code::Unimplemented {
                    Error::ServiceStatusUnsupported
                } else {
                    Error::GrpcStatus(status)
                }
            })?
            .into_inner();
        Ok(response
            .services
            .into_iter()
            .map(|(name, status)| (name, convert_status(status)))
            .collect())
    }
}

fn convert_status(status: LocalServiceStatus) -> ServiceStatus {
    let state = match LocalServiceState::try_from(status.state).unwrap_or_default() {
        LocalServiceState::Available => ServiceState::Available,
        LocalServiceState::Unavailable => ServiceState::Unavailable,
        LocalServiceState::Unknown => ServiceState::Unknown,
    };
    ServiceStatus {
        state,
        reason: status.reason,
    }
}

/// Named status helpers exposed by a device proxy.
#[derive(Clone, Debug)]
pub struct DeviceProxyStatus {
    photo_library: ServiceStatusQuerier,
    file_handler: ServiceStatusQuerier,
    services: ServiceStatusRegistry,
}

impl DeviceProxyStatus {
    pub(crate) fn new(service: DeviceServiceTransport) -> Self {
        let services = ServiceStatusRegistry::new(service);
        Self {
            photo_library: ServiceStatusQuerier::new(
                services.clone(),
                photo_library_server::SERVICE_NAME,
            ),
            file_handler: ServiceStatusQuerier::new(
                services.clone(),
                file_handler_server::SERVICE_NAME,
            ),
            services,
        }
    }

    #[must_use]
    pub fn photo_library(&self) -> ServiceStatusQuerier {
        self.photo_library.clone()
    }

    #[must_use]
    pub fn file_handler(&self) -> ServiceStatusQuerier {
        self.file_handler.clone()
    }

    #[must_use]
    pub fn services(&self) -> ServiceStatusRegistry {
        self.services.clone()
    }
}
