#![forbid(unsafe_code)]

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use std::sync::Arc;
use trustheim_api::{BackendCapabilitiesResponse, ErrorResponse, HealthResponse, ServiceStatus};
use trustheim_backend::{BackendError, BackendHealth, CaBackend, RejectingBackend};
use utoipa::OpenApi;

#[derive(Clone)]
pub struct AppState {
    backend: Arc<dyn CaBackend>,
}

impl AppState {
    pub fn new(backend: Arc<dyn CaBackend>) -> Self {
        Self { backend }
    }
}

pub fn router() -> Router {
    router_with_state(AppState::new(Arc::new(RejectingBackend)))
}

pub fn router_with_state(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/backend/capabilities", get(backend_capabilities))
        .route("/api/openapi.json", get(openapi))
        .with_state(state)
}

#[derive(OpenApi)]
#[openapi(
    paths(health, backend_capabilities),
    components(
        schemas(
            trustheim_api::HealthResponse,
            trustheim_api::ServiceStatus,
            trustheim_api::BackendCapabilitiesResponse,
            trustheim_api::ErrorResponse,
            trustheim_backend::BackendHealth,
            trustheim_backend::BackendCapabilities,
            trustheim_backend::BackendErrorKind,
            trustheim_backend::ProviderInfo
        )
    ),
    tags((name = "trustheim", description = "Trustheim CA orchestration API"))
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, body = HealthResponse))
)]
async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let backend = state.backend.health().await.unwrap_or(BackendHealth {
        available: false,
        sealed: false,
    });
    let status = if backend.available && !backend.sealed {
        ServiceStatus::Ok
    } else {
        ServiceStatus::Degraded
    };

    Json(HealthResponse { status, backend })
}

#[utoipa::path(
    get,
    path = "/api/v1/backend/capabilities",
    responses(
        (status = 200, body = BackendCapabilitiesResponse),
        (status = 503, body = ErrorResponse)
    )
)]
async fn backend_capabilities(State(state): State<AppState>) -> impl IntoResponse {
    match state.backend.capabilities().await {
        Ok(capabilities) => {
            let response = BackendCapabilitiesResponse {
                provider: state.backend.provider_info(),
                capabilities,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => backend_error_response(error).into_response(),
    }
}

async fn openapi() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}

fn backend_error_response(error: BackendError) -> (StatusCode, Json<ErrorResponse>) {
    let response = ErrorResponse {
        code: error.kind,
        message: error.message,
    };
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn openapi_document_contains_health_path() {
        let document = serde_json::to_value(ApiDoc::openapi()).unwrap();
        assert!(document["paths"]["/health"].is_object());
        assert!(document["paths"]["/api/v1/backend/capabilities"].is_object());
    }
}
