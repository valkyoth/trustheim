#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use trustheim_backend::{BackendCapabilities, BackendErrorKind, BackendHealth, ProviderInfo};
use utoipa::ToSchema;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: ServiceStatus,
    pub backend: BackendHealth,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    Ok,
    Degraded,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct BackendCapabilitiesResponse {
    pub provider: ProviderInfo,
    pub capabilities: BackendCapabilities,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub code: BackendErrorKind,
    pub message: String,
}
