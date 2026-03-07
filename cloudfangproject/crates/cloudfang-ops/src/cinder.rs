//! Cinder Block Storage — Volume Management

use crate::{OpenStackSession, OpsError, OpsResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub size: u32,
    pub volume_type: Option<String>,
    pub bootable: Option<String>,
    pub attachments: Vec<VolumeAttachment>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeAttachment {
    pub server_id: String,
    pub device: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub volume_id: String,
    pub size: u32,
    pub created_at: Option<String>,
}

fn cinder_url(session: &OpenStackSession) -> OpsResult<String> {
    Ok(session
        .endpoint("volumev3")?
        .trim_end_matches('/')
        .to_string())
}

pub async fn list_volumes(session: &mut OpenStackSession) -> OpsResult<Vec<Volume>> {
    session.ensure_authenticated().await?;
    let url = format!("{}/volumes/detail", cinder_url(session)?);
    let resp = session
        .http_client()
        .get(&url)
        .header("X-Auth-Token", &session.token)
        .send()
        .await?;
    if !resp.status().is_success() {
        let s = resp.status().as_u16();
        return Err(OpsError::ApiError {
            status: s,
            message: resp.text().await.unwrap_or_default(),
        });
    }
    #[derive(Deserialize)]
    struct R {
        volumes: Vec<Volume>,
    }
    Ok(resp.json::<R>().await?.volumes)
}

pub async fn create_snapshot(
    session: &mut OpenStackSession,
    volume_id: &str,
    name: &str,
    description: Option<&str>,
) -> OpsResult<Snapshot> {
    session.ensure_authenticated().await?;
    let url = format!("{}/snapshots", cinder_url(session)?);

    let body = serde_json::json!({
        "snapshot": {
            "volume_id": volume_id,
            "name": name,
            "description": description.unwrap_or(""),
            "force": true
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
        let s = resp.status().as_u16();
        return Err(OpsError::ApiError {
            status: s,
            message: resp.text().await.unwrap_or_default(),
        });
    }
    #[derive(Deserialize)]
    struct R {
        snapshot: Snapshot,
    }
    Ok(resp.json::<R>().await?.snapshot)
}

pub async fn list_snapshots(session: &mut OpenStackSession) -> OpsResult<Vec<Snapshot>> {
    session.ensure_authenticated().await?;
    let url = format!("{}/snapshots/detail", cinder_url(session)?);
    let resp = session
        .http_client()
        .get(&url)
        .header("X-Auth-Token", &session.token)
        .send()
        .await?;
    if !resp.status().is_success() {
        let s = resp.status().as_u16();
        return Err(OpsError::ApiError {
            status: s,
            message: resp.text().await.unwrap_or_default(),
        });
    }
    #[derive(Deserialize)]
    struct R {
        snapshots: Vec<Snapshot>,
    }
    Ok(resp.json::<R>().await?.snapshots)
}

pub async fn delete_snapshot(session: &mut OpenStackSession, snapshot_id: &str) -> OpsResult<()> {
    session.ensure_authenticated().await?;
    let url = format!("{}/snapshots/{}", cinder_url(session)?, snapshot_id);
    let resp = session
        .http_client()
        .delete(&url)
        .header("X-Auth-Token", &session.token)
        .send()
        .await?;
    if !resp.status().is_success() {
        let s = resp.status().as_u16();
        return Err(OpsError::ApiError {
            status: s,
            message: resp.text().await.unwrap_or_default(),
        });
    }
    tracing::info!("🗑️ Deleted snapshot {}", snapshot_id);
    Ok(())
}
