//! Nova Compute — VM Lifecycle Management

use crate::{OpenStackSession, OpsError, OpsResult};
use serde::{Deserialize, Serialize};

/// A Nova server (VM) instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "OS-EXT-STS:power_state")]
    pub power_state: Option<i32>,
    #[serde(rename = "OS-EXT-STS:vm_state")]
    pub vm_state: Option<String>,
    pub flavor: Option<FlavorRef>,
    pub addresses: Option<serde_json::Value>,
    pub created: Option<String>,
    pub updated: Option<String>,
    #[serde(rename = "OS-EXT-SRV-ATTR:host")]
    pub host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorRef {
    pub id: String,
}

#[derive(Deserialize)]
struct ServersResponse {
    servers: Vec<Server>,
}

#[derive(Deserialize)]
struct ServerResponse {
    server: Server,
}

/// Server diagnostics (CPU, memory, disk, network stats).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDiagnostics {
    #[serde(default)]
    pub cpu0_time: Option<u64>,
    #[serde(default)]
    pub memory: Option<u64>,
    #[serde(rename = "memory-actual", default)]
    pub memory_actual: Option<u64>,
    #[serde(rename = "memory-rss", default)]
    pub memory_rss: Option<u64>,
    #[serde(rename = "vda_read_req", default)]
    pub vda_read_req: Option<u64>,
    #[serde(rename = "vda_write_req", default)]
    pub vda_write_req: Option<u64>,
    #[serde(rename = "vda_read", default)]
    pub vda_read: Option<u64>,
    #[serde(rename = "vda_write", default)]
    pub vda_write: Option<u64>,
}

fn nova_url(session: &OpenStackSession) -> OpsResult<String> {
    Ok(session
        .endpoint("compute")?
        .trim_end_matches('/')
        .to_string())
}

/// List all servers (VMs) in the current project.
pub async fn list_servers(session: &mut OpenStackSession) -> OpsResult<Vec<Server>> {
    session.ensure_authenticated().await?;
    let url = format!("{}/servers/detail", nova_url(session)?);

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

    let body: ServersResponse = resp.json().await?;
    Ok(body.servers)
}

/// Get details of a single server.
pub async fn get_server(session: &mut OpenStackSession, server_id: &str) -> OpsResult<Server> {
    session.ensure_authenticated().await?;
    let url = format!("{}/servers/{}", nova_url(session)?, server_id);

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

    let body: ServerResponse = resp.json().await?;
    Ok(body.server)
}

/// Perform an action on a server (reboot, stop, start, pause, unpause).
pub async fn server_action(
    session: &mut OpenStackSession,
    server_id: &str,
    action: ServerAction,
) -> OpsResult<()> {
    session.ensure_authenticated().await?;
    let url = format!("{}/servers/{}/action", nova_url(session)?, server_id);

    let action_name = action.name().to_string();

    let body = match action {
        ServerAction::Reboot(reboot_type) => {
            serde_json::json!({ "reboot": { "type": reboot_type.as_str() } })
        }
        ServerAction::Start => serde_json::json!({ "os-start": null }),
        ServerAction::Stop => serde_json::json!({ "os-stop": null }),
        ServerAction::Pause => serde_json::json!({ "pause": null }),
        ServerAction::Unpause => serde_json::json!({ "unpause": null }),
        ServerAction::LiveMigrate {
            host,
            block_migration,
        } => {
            serde_json::json!({
                "os-migrateLive": {
                    "host": host,
                    "block_migration": block_migration,
                    "disk_over_commit": false
                }
            })
        }
    };

    let resp = session
        .http_client()
        .post(&url)
        .header("X-Auth-Token", &session.token)
        .json(&body)
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

    tracing::info!(
        "✅ Server action completed: {} on {}",
        action_name,
        server_id
    );
    Ok(())
}

/// Get server diagnostics (CPU, memory, disk I/O).
pub async fn get_diagnostics(
    session: &mut OpenStackSession,
    server_id: &str,
) -> OpsResult<ServerDiagnostics> {
    session.ensure_authenticated().await?;
    let url = format!("{}/servers/{}/diagnostics", nova_url(session)?, server_id);

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

    let diag: ServerDiagnostics = resp.json().await?;
    Ok(diag)
}

/// Get the console log output of a server.
pub async fn get_console_log(
    session: &mut OpenStackSession,
    server_id: &str,
    length: Option<u32>,
) -> OpsResult<String> {
    session.ensure_authenticated().await?;
    let url = format!("{}/servers/{}/action", nova_url(session)?, server_id);

    let body = serde_json::json!({
        "os-getConsoleOutput": {
            "length": length.unwrap_or(50)
        }
    });

    let resp = session
        .http_client()
        .post(&url)
        .header("X-Auth-Token", &session.token)
        .json(&body)
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

    #[derive(Deserialize)]
    struct ConsoleOutput {
        output: String,
    }
    let out: ConsoleOutput = resp.json().await?;
    Ok(out.output)
}

/// Actions that can be performed on a server.
#[derive(Debug, Clone)]
pub enum ServerAction {
    Reboot(RebootType),
    Start,
    Stop,
    Pause,
    Unpause,
    LiveMigrate {
        host: Option<String>,
        block_migration: bool,
    },
}

impl ServerAction {
    pub fn name(&self) -> &str {
        match self {
            Self::Reboot(_) => "reboot",
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Pause => "pause",
            Self::Unpause => "unpause",
            Self::LiveMigrate { .. } => "live-migrate",
        }
    }
}

#[derive(Debug, Clone)]
pub enum RebootType {
    Soft,
    Hard,
}

impl RebootType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Soft => "SOFT",
            Self::Hard => "HARD",
        }
    }
}
