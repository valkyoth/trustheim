#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use trustheim_backend::{
    AuditStatus, BackendCapabilities, BackendError, BackendFuture, BackendHealth, CaBackend,
    IssuerInfo, ProviderInfo, RevocationResult, RevokeCertificateRequest,
};
use trustheim_domain::{IssuerRef, SignCsrRequest, SignedCertificate};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpenBaoConfig {
    pub addr: String,
    pub ca_cert_path: Option<String>,
    pub client_cert_path: Option<String>,
    pub client_key_path: Option<String>,
    pub issuers: Vec<OpenBaoIssuerMapping>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpenBaoIssuerMapping {
    pub name: String,
    pub mount: String,
    pub role: String,
}

pub struct OpenBaoBackend {
    config: OpenBaoConfig,
}

impl OpenBaoBackend {
    pub fn new(config: OpenBaoConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &OpenBaoConfig {
        &self.config
    }
}

impl CaBackend for OpenBaoBackend {
    fn provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            provider: "openbao".to_string(),
            version: None,
        }
    }

    fn health(&self) -> BackendFuture<'_, BackendHealth> {
        Box::pin(async {
            Err(BackendError::unsupported(
                "OpenBao network client is planned for v0.5 and is not implemented yet",
            ))
        })
    }

    fn capabilities(&self) -> BackendFuture<'_, BackendCapabilities> {
        Box::pin(async { Ok(BackendCapabilities::none()) })
    }

    fn sign_csr(&self, _request: SignCsrRequest) -> BackendFuture<'_, SignedCertificate> {
        Box::pin(async {
            Err(BackendError::unsupported(
                "OpenBao signing is planned for v0.6 and is not implemented yet",
            ))
        })
    }

    fn revoke_certificate(
        &self,
        _request: RevokeCertificateRequest,
    ) -> BackendFuture<'_, RevocationResult> {
        Box::pin(async {
            Err(BackendError::unsupported(
                "OpenBao revocation is planned for v0.7 and is not implemented yet",
            ))
        })
    }

    fn read_issuer(&self, _issuer: IssuerRef) -> BackendFuture<'_, IssuerInfo> {
        Box::pin(async {
            Err(BackendError::unsupported(
                "OpenBao issuer reads are planned for v0.5 and are not implemented yet",
            ))
        })
    }

    fn audit_status(&self) -> BackendFuture<'_, AuditStatus> {
        Box::pin(async {
            Err(BackendError::unsupported(
                "OpenBao audit status is planned for v0.5 and is not implemented yet",
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trustheim_backend::CaBackend;

    #[test]
    fn provider_identity_is_openbao() {
        let backend = OpenBaoBackend::new(OpenBaoConfig {
            addr: "https://openbao.example.test:8200".to_string(),
            ca_cert_path: None,
            client_cert_path: None,
            client_key_path: None,
            issuers: Vec::new(),
        });

        assert_eq!(backend.provider_info().provider, "openbao");
    }
}
