#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use trustheim_domain::{IssuerRef, SignCsrRequest, SignedCertificate};
use utoipa::ToSchema;

pub type BackendFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, BackendError>> + Send + 'a>>;

pub trait CaBackend: Send + Sync {
    fn provider_info(&self) -> ProviderInfo;
    fn health(&self) -> BackendFuture<'_, BackendHealth>;
    fn capabilities(&self) -> BackendFuture<'_, BackendCapabilities>;
    fn sign_csr(&self, request: SignCsrRequest) -> BackendFuture<'_, SignedCertificate>;
    fn revoke_certificate(
        &self,
        request: RevokeCertificateRequest,
    ) -> BackendFuture<'_, RevocationResult>;
    fn read_issuer(&self, issuer: IssuerRef) -> BackendFuture<'_, IssuerInfo>;
    fn audit_status(&self) -> BackendFuture<'_, AuditStatus>;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ProviderInfo {
    pub provider: String,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct BackendHealth {
    pub available: bool,
    pub sealed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct BackendCapabilities {
    pub csr_signing: bool,
    pub backend_key_generation: bool,
    pub issuer_listing: bool,
    pub revocation: bool,
    pub crl_publication: bool,
    pub ocsp: bool,
    pub acme: bool,
    pub audit_status: bool,
    pub response_wrapping: bool,
    pub approle_or_equivalent: bool,
    pub mtls_client_auth: bool,
    pub hardware_seal: bool,
    pub hsm_backed_pki_signing: bool,
}

impl BackendCapabilities {
    pub fn none() -> Self {
        Self {
            csr_signing: false,
            backend_key_generation: false,
            issuer_listing: false,
            revocation: false,
            crl_publication: false,
            ocsp: false,
            acme: false,
            audit_status: false,
            response_wrapping: false,
            approle_or_equivalent: false,
            mtls_client_auth: false,
            hardware_seal: false,
            hsm_backed_pki_signing: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct RevokeCertificateRequest {
    pub serial_number: String,
    pub reason: RevocationReason,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RevocationReason {
    KeyCompromise,
    CessationOfOperation,
    Superseded,
    PrivilegeWithdrawn,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct RevocationResult {
    pub revoked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct IssuerInfo {
    pub issuer: String,
    pub active: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct AuditStatus {
    pub enabled: bool,
    pub sink_count: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum BackendErrorKind {
    Unavailable,
    Sealed,
    Unsupported,
    Forbidden,
    InvalidRequest,
    AuditUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct BackendError {
    pub kind: BackendErrorKind,
    pub message: String,
}

impl BackendError {
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self {
            kind: BackendErrorKind::Unsupported,
            message: message.into(),
        }
    }
}

#[derive(Default)]
pub struct RejectingBackend;

impl CaBackend for RejectingBackend {
    fn provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            provider: "not_configured".to_string(),
            version: None,
        }
    }

    fn health(&self) -> BackendFuture<'_, BackendHealth> {
        Box::pin(async {
            Ok(BackendHealth {
                available: false,
                sealed: false,
            })
        })
    }

    fn capabilities(&self) -> BackendFuture<'_, BackendCapabilities> {
        Box::pin(async { Ok(BackendCapabilities::none()) })
    }

    fn sign_csr(&self, _request: SignCsrRequest) -> BackendFuture<'_, SignedCertificate> {
        Box::pin(async { Err(BackendError::unsupported("no CA backend is configured")) })
    }

    fn revoke_certificate(
        &self,
        _request: RevokeCertificateRequest,
    ) -> BackendFuture<'_, RevocationResult> {
        Box::pin(async { Err(BackendError::unsupported("no CA backend is configured")) })
    }

    fn read_issuer(&self, _issuer: IssuerRef) -> BackendFuture<'_, IssuerInfo> {
        Box::pin(async { Err(BackendError::unsupported("no CA backend is configured")) })
    }

    fn audit_status(&self) -> BackendFuture<'_, AuditStatus> {
        Box::pin(async { Err(BackendError::unsupported("no CA backend is configured")) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rejecting_backend_has_no_capabilities() {
        let backend = RejectingBackend;
        let capabilities = backend.capabilities().await.unwrap();
        assert!(!capabilities.csr_signing);
        assert!(!capabilities.hsm_backed_pki_signing);
    }
}
