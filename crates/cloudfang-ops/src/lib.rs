//! # CloudFang Ops — OpenStack API Client
//!
//! Provides typed Rust clients for OpenStack services:
//! - **Keystone** (Identity v3) — Authentication & token management
//! - **Nova** (Compute) — VM lifecycle management
//! - **Neutron** (Networking) — Network/subnet/port operations
//! - **Cinder** (Block Storage) — Volume management
//! - **Glance** (Image) — Image listing
//! - **Heat** (Orchestration) — Stack management
//! - **Metrics** — Ceilometer/Gnocchi metrics collection

pub mod cinder;
pub mod glance;
pub mod heat;
pub mod keystone;
pub mod metrics;
pub mod neutron;
pub mod nova;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors from OpenStack API operations.
#[derive(Error, Debug)]
pub enum OpsError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("Token expired")]
    TokenExpired,

    #[error("Service endpoint not found: {0}")]
    EndpointNotFound(String),

    #[error("Deserialization error: {0}")]
    DeserializeError(#[from] serde_json::Error),
}

pub type OpsResult<T> = Result<T, OpsError>;

/// OpenStack authentication credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenStackCredentials {
    pub auth_url: String,
    pub username: String,
    pub password: String,
    pub project_name: String,
    pub domain_name: String,
}

/// Authenticated session holding the token and service catalog.
#[derive(Debug, Clone)]
pub struct OpenStackSession {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub catalog: Vec<ServiceEndpoint>,
    pub credentials: OpenStackCredentials,
    client: reqwest::Client,
}

/// A service endpoint from the Keystone catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub service_type: String,
    pub url: String,
    pub region: String,
}

impl OpenStackSession {
    /// Create a new session by authenticating with Keystone.
    pub async fn new(credentials: OpenStackCredentials) -> OpsResult<Self> {
        let session = keystone::authenticate(credentials).await?;
        Ok(session)
    }

    /// Check if the current token is still valid.
    pub fn is_token_valid(&self) -> bool {
        chrono::Utc::now() < self.expires_at
    }

    /// Re-authenticate if the token has expired.
    pub async fn ensure_authenticated(&mut self) -> OpsResult<()> {
        if !self.is_token_valid() {
            tracing::info!("Token expired, re-authenticating...");
            let new_session = keystone::authenticate(self.credentials.clone()).await?;
            self.token = new_session.token;
            self.expires_at = new_session.expires_at;
            self.catalog = new_session.catalog;
        }
        Ok(())
    }

    /// Get the endpoint URL for a given service type.
    pub fn endpoint(&self, service_type: &str) -> OpsResult<&str> {
        self.catalog
            .iter()
            .find(|ep| ep.service_type == service_type)
            .map(|ep| ep.url.as_str())
            .ok_or_else(|| OpsError::EndpointNotFound(service_type.to_string()))
    }

    /// Get a reference to the HTTP client.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Build from raw parts (used by keystone::authenticate).
    pub(crate) fn from_parts(
        token: String,
        expires_at: chrono::DateTime<chrono::Utc>,
        catalog: Vec<ServiceEndpoint>,
        credentials: OpenStackCredentials,
    ) -> Self {
        Self {
            token,
            expires_at,
            catalog,
            credentials,
            client: reqwest::Client::new(),
        }
    }
}
