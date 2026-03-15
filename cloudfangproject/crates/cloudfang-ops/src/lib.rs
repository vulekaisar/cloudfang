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
        // We consider token invalid if it expires in less than 60 seconds
        let buffer = chrono::Duration::seconds(60);
        chrono::Utc::now() + buffer < self.expires_at
    }

    /// Re-authenticate if the token has expired.
    pub async fn ensure_authenticated(&mut self) -> OpsResult<()> {
        if !self.is_token_valid() {
            tracing::info!("Token expired or near expiration, re-authenticating...");
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
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("CloudFang-Agent/0.1.0")
            .build()
            .unwrap_or_default();

        Self {
            token,
            expires_at,
            catalog,
            credentials,
            client,
        }
    }

    /// Perform an HTTP request with automatic token refresh and retry logic.
    pub async fn request_with_retry(
        &mut self,
        method: reqwest::Method,
        url: &str,
        body: Option<serde_json::Value>,
    ) -> OpsResult<reqwest::Response> {
        self.ensure_authenticated().await?;

        let mut attempts = 0;
        let max_attempts = 3;
        let mut last_error = None;

        while attempts < max_attempts {
            if attempts > 0 {
                let delay = std::time::Duration::from_secs(2u64.pow(attempts));
                tracing::warn!("Retry attempt {} after {:?}...", attempts, delay);
                tokio::time::sleep(delay).await;
            }

            let mut req = self.client.request(method.clone(), url)
                .header("X-Auth-Token", &self.token);
            
            if let Some(ref b) = body {
                req = req.json(b);
            }

            match req.send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(resp);
                    } else if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
                        tracing::info!("Token likely expired (401), refreshing...");
                        self.ensure_authenticated().await?;
                    } else {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        last_error = Some(OpsError::ApiError { status: status.as_u16(), message: text });
                    }
                }
                Err(e) => {
                    last_error = Some(OpsError::HttpError(e));
                }
            }
            attempts += 1;
        }

        Err(last_error.unwrap_or_else(|| OpsError::AuthError(format!("Request failed after {} attempts", max_attempts))))
    }
}
