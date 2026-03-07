//! Keystone v3 Identity — Authentication & Token Management

use crate::{OpenStackCredentials, OpenStackSession, OpsError, OpsResult, ServiceEndpoint};
use serde::{Deserialize, Serialize};

/// Keystone v3 authentication request body.
#[derive(Serialize)]
struct AuthRequest {
    auth: AuthBody,
}

#[derive(Serialize)]
struct AuthBody {
    identity: Identity,
    scope: Scope,
}

#[derive(Serialize)]
struct Identity {
    methods: Vec<String>,
    password: PasswordAuth,
}

#[derive(Serialize)]
struct PasswordAuth {
    user: UserAuth,
}

#[derive(Serialize)]
struct UserAuth {
    name: String,
    password: String,
    domain: DomainRef,
}

#[derive(Serialize)]
struct DomainRef {
    name: String,
}

#[derive(Serialize)]
struct Scope {
    project: ProjectScope,
}

#[derive(Serialize)]
struct ProjectScope {
    name: String,
    domain: DomainRef,
}

/// Keystone token response (partial).
#[derive(Deserialize, Debug)]
struct TokenResponse {
    token: TokenData,
}

#[derive(Deserialize, Debug)]
struct TokenData {
    expires_at: String,
    catalog: Option<Vec<CatalogEntry>>,
}

#[derive(Deserialize, Debug)]
struct CatalogEntry {
    #[serde(rename = "type")]
    service_type: String,
    endpoints: Vec<EndpointEntry>,
}

#[derive(Deserialize, Debug)]
struct EndpointEntry {
    url: String,
    region_id: Option<String>,
    interface: String,
}

/// Authenticate with Keystone v3 and return an OpenStackSession.
pub async fn authenticate(creds: OpenStackCredentials) -> OpsResult<OpenStackSession> {
    let client = reqwest::Client::new();
    let auth_url = format!("{}/auth/tokens", creds.auth_url.trim_end_matches('/'));

    let body = AuthRequest {
        auth: AuthBody {
            identity: Identity {
                methods: vec!["password".to_string()],
                password: PasswordAuth {
                    user: UserAuth {
                        name: creds.username.clone(),
                        password: creds.password.clone(),
                        domain: DomainRef {
                            name: creds.domain_name.clone(),
                        },
                    },
                },
            },
            scope: Scope {
                project: ProjectScope {
                    name: creds.project_name.clone(),
                    domain: DomainRef {
                        name: creds.domain_name.clone(),
                    },
                },
            },
        },
    };

    tracing::info!("Authenticating with Keystone at {}", auth_url);

    let response = client.post(&auth_url).json(&body).send().await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let msg = response.text().await.unwrap_or_default();
        return Err(OpsError::AuthError(format!("HTTP {status}: {msg}")));
    }

    // Extract token from X-Subject-Token header
    let token = response
        .headers()
        .get("X-Subject-Token")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .ok_or_else(|| OpsError::AuthError("Missing X-Subject-Token header".into()))?;

    let token_resp: TokenResponse = response.json().await?;

    // Parse expiration
    let expires_at = chrono::DateTime::parse_from_rfc3339(&token_resp.token.expires_at)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| OpsError::AuthError(format!("Invalid expires_at: {e}")))?;

    // Extract service catalog — prefer "public" interface
    let catalog = token_resp
        .token
        .catalog
        .unwrap_or_default()
        .into_iter()
        .flat_map(|entry| {
            entry
                .endpoints
                .into_iter()
                .filter(|ep| ep.interface == "public")
                .map(move |ep| ServiceEndpoint {
                    service_type: entry.service_type.clone(),
                    url: ep.url,
                    region: ep.region_id.unwrap_or_default(),
                })
        })
        .collect();

    tracing::info!(
        "✅ Authenticated successfully, token expires at {}",
        expires_at
    );

    Ok(OpenStackSession::from_parts(
        token, expires_at, catalog, creds,
    ))
}

/// List all projects visible to the authenticated user.
pub async fn list_projects(session: &mut OpenStackSession) -> OpsResult<Vec<Project>> {
    session.ensure_authenticated().await?;
    let url = format!(
        "{}/projects",
        session.endpoint("identity")?.trim_end_matches('/')
    );

    let resp = session
        .http_client()
        .get(&url)
        .header("X-Auth-Token", &session.token)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status().as_u16();
        let msg = resp.text().await.unwrap_or_default();
        return Err(OpsError::ApiError {
            status,
            message: msg,
        });
    }

    let body: ProjectsResponse = resp.json().await?;
    Ok(body.projects)
}

#[derive(Deserialize, Debug)]
struct ProjectsResponse {
    projects: Vec<Project>,
}

/// An OpenStack project (tenant).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
}
