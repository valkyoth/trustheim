#![forbid(unsafe_code)]

use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    routing::post,
};
use std::sync::Arc;
use trustheim_api::{BackendCapabilitiesResponse, ErrorResponse, HealthResponse, ServiceStatus};
use trustheim_backend::{
    BackendError, BackendErrorKind, BackendHealth, CaBackend, RejectingBackend,
};
use trustheim_domain::{SignCsrRequest, SignedCertificate};
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
        .route("/api/v1/certificates/sign-csr", post(sign_csr))
        .route("/api/openapi.json", get(openapi))
        .with_state(state)
}

#[derive(OpenApi)]
#[openapi(
    paths(health, backend_capabilities, sign_csr),
    components(
        schemas(
            trustheim_api::HealthResponse,
            trustheim_api::ServiceStatus,
            trustheim_api::BackendCapabilitiesResponse,
            trustheim_api::ErrorResponse,
            trustheim_backend::BackendHealth,
            trustheim_backend::BackendCapabilities,
            trustheim_backend::BackendErrorKind,
            trustheim_backend::ProviderInfo,
            trustheim_domain::CertificateProfileName,
            trustheim_domain::IssuerRef,
            trustheim_domain::DnsName,
            trustheim_domain::CertificateTtlSeconds,
            trustheim_domain::CsrPem,
            trustheim_domain::SignCsrRequest,
            trustheim_domain::SignedCertificate
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

#[utoipa::path(
    post,
    path = "/api/v1/certificates/sign-csr",
    request_body = SignCsrRequest,
    responses(
        (status = 200, body = SignedCertificate),
        (status = 400, body = ErrorResponse),
        (status = 403, body = ErrorResponse),
        (status = 503, body = ErrorResponse)
    )
)]
async fn sign_csr(
    State(state): State<AppState>,
    request: Result<Json<SignCsrRequest>, JsonRejection>,
) -> impl IntoResponse {
    let Json(request) = match request {
        Ok(request) => request,
        Err(rejection) => {
            let response = ErrorResponse {
                code: BackendErrorKind::InvalidRequest,
                message: "invalid sign CSR request body".to_string(),
            };
            return (rejection.status(), Json(response)).into_response();
        }
    };

    match state.backend.sign_csr(request).await {
        Ok(certificate) => (StatusCode::OK, Json(certificate)).into_response(),
        Err(error) => backend_error_response(error).into_response(),
    }
}

async fn openapi() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}

fn backend_error_response(error: BackendError) -> (StatusCode, Json<ErrorResponse>) {
    let status = match error.kind {
        BackendErrorKind::Forbidden => StatusCode::FORBIDDEN,
        BackendErrorKind::InvalidRequest => StatusCode::BAD_REQUEST,
        BackendErrorKind::Unavailable
        | BackendErrorKind::Sealed
        | BackendErrorKind::Unsupported
        | BackendErrorKind::AuditUnavailable => StatusCode::SERVICE_UNAVAILABLE,
    };
    let response = ErrorResponse {
        code: error.kind,
        message: error.message,
    };
    (status, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn openapi_document_contains_health_path() {
        let document = serde_json::to_value(ApiDoc::openapi()).unwrap();
        assert!(document["paths"]["/health"].is_object());
        assert!(document["paths"]["/api/v1/backend/capabilities"].is_object());
        assert!(document["paths"]["/api/v1/certificates/sign-csr"].is_object());
    }
}
