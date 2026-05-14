#![forbid(unsafe_code)]

use serde::{Deserialize, Deserializer, Serialize, de};
use std::error::Error;
use std::fmt::{self, Display};
use utoipa::ToSchema;

const MAX_NAME_LEN: usize = 128;
const MAX_DNS_NAME_LEN: usize = 253;
const MAX_CSR_PEM_BYTES: usize = 64 * 1024;
const MAX_TTL_SECONDS: u64 = 398 * 24 * 60 * 60;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, ToSchema)]
#[serde(transparent)]
pub struct CertificateProfileName(String);

impl CertificateProfileName {
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        validate_slug("profile", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CertificateProfileName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, ToSchema)]
#[serde(transparent)]
pub struct IssuerRef(String);

impl IssuerRef {
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        validate_slug("issuer", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for IssuerRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, ToSchema)]
#[serde(transparent)]
pub struct DnsName(String);

impl DnsName {
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let value = value.trim_end_matches('.').to_ascii_lowercase();
        validate_dns_name(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for DnsName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, ToSchema)]
#[serde(transparent)]
pub struct CertificateTtlSeconds(u64);

impl CertificateTtlSeconds {
    pub fn new(seconds: u64) -> Result<Self, ValidationError> {
        if seconds == 0 {
            return Err(ValidationError::new("ttl", "must be greater than zero"));
        }
        if seconds > MAX_TTL_SECONDS {
            return Err(ValidationError::new("ttl", "exceeds maximum supported TTL"));
        }
        Ok(Self(seconds))
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for CertificateTtlSeconds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(u64::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, ToSchema)]
#[serde(transparent)]
pub struct CsrPem(String);

impl CsrPem {
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.len() > MAX_CSR_PEM_BYTES {
            return Err(ValidationError::new("csr_pem", "CSR is too large"));
        }
        if !value.starts_with("-----BEGIN CERTIFICATE REQUEST-----") {
            return Err(ValidationError::new(
                "csr_pem",
                "missing CSR PEM begin marker",
            ));
        }
        if !value
            .trim_end()
            .ends_with("-----END CERTIFICATE REQUEST-----")
        {
            return Err(ValidationError::new(
                "csr_pem",
                "missing CSR PEM end marker",
            ));
        }
        if value
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\r')
        {
            return Err(ValidationError::new(
                "csr_pem",
                "contains hidden control bytes",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CsrPem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SignCsrRequest {
    pub profile: CertificateProfileName,
    pub issuer: IssuerRef,
    pub csr_pem: CsrPem,
    pub dns_sans: Vec<DnsName>,
    pub ttl: CertificateTtlSeconds,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SignedCertificate {
    pub certificate_pem: String,
    pub issuing_ca_pem: String,
    pub serial_number: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ValidationError {
    field: &'static str,
    message: &'static str,
}

impl ValidationError {
    pub fn new(field: &'static str, message: &'static str) -> Self {
        Self { field, message }
    }

    pub fn field(&self) -> &'static str {
        self.field
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}

impl Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.field, self.message)
    }
}

impl Error for ValidationError {}

fn validate_slug(field: &'static str, value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new(field, "must not be empty"));
    }
    if value.len() > MAX_NAME_LEN {
        return Err(ValidationError::new(field, "is too long"));
    }
    let valid = value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'_'
    });
    if !valid {
        return Err(ValidationError::new(
            field,
            "must contain only lowercase ASCII letters, digits, '-' or '_'",
        ));
    }
    Ok(())
}

fn validate_dns_name(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::new("dns_name", "must not be empty"));
    }
    if value.len() > MAX_DNS_NAME_LEN {
        return Err(ValidationError::new("dns_name", "is too long"));
    }
    if value.starts_with("*.") {
        return Err(ValidationError::new(
            "dns_name",
            "wildcards are profile policy, not request input",
        ));
    }
    for label in value.split('.') {
        validate_dns_label(label)?;
    }
    Ok(())
}

fn validate_dns_label(label: &str) -> Result<(), ValidationError> {
    if label.is_empty() {
        return Err(ValidationError::new("dns_name", "contains an empty label"));
    }
    if label.len() > 63 {
        return Err(ValidationError::new(
            "dns_name",
            "contains an overlong label",
        ));
    }
    if label.starts_with('-') || label.ends_with('-') {
        return Err(ValidationError::new(
            "dns_name",
            "label must not start or end with '-'",
        ));
    }
    if !label
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(ValidationError::new(
            "dns_name",
            "must contain only ASCII letters, digits, '-' and '.'",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_names_are_canonicalized() {
        let name = DnsName::parse("Api.Example.Internal.").unwrap();
        assert_eq!(name.as_str(), "api.example.internal");
    }

    #[test]
    fn dns_names_reject_unsafe_shapes() {
        for value in [
            "",
            "-bad.example",
            "bad-.example",
            "bad..example",
            "*.example",
            "bad_example",
        ] {
            assert!(DnsName::parse(value).is_err(), "{value}");
        }
    }

    #[test]
    fn slugs_reject_uppercase_and_spaces() {
        assert!(CertificateProfileName::parse("server-modern").is_ok());
        assert!(CertificateProfileName::parse("Server Modern").is_err());
    }

    #[test]
    fn ttl_is_bounded() {
        assert_eq!(CertificateTtlSeconds::new(3600).unwrap().get(), 3600);
        assert!(CertificateTtlSeconds::new(0).is_err());
        assert!(CertificateTtlSeconds::new(MAX_TTL_SECONDS + 1).is_err());
    }

    #[test]
    fn serde_deserialization_enforces_validation() {
        let invalid_dns = serde_json::from_str::<DnsName>(r#""*.example.internal""#);
        assert!(invalid_dns.is_err());

        let invalid_ttl = serde_json::from_str::<CertificateTtlSeconds>("0");
        assert!(invalid_ttl.is_err());
    }
}
