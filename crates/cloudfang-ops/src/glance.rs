//! Glance Image Service — Image Listing

use crate::{OpenStackSession, OpsError, OpsResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub size: Option<u64>,
    pub disk_format: Option<String>,
    pub container_format: Option<String>,
    pub visibility: Option<String>,
    pub created_at: Option<String>,
}

pub async fn list_images(session: &mut OpenStackSession) -> OpsResult<Vec<Image>> {
    session.ensure_authenticated().await?;
    let url = format!(
        "{}/v2/images",
        session.endpoint("image")?.trim_end_matches('/')
    );
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
        images: Vec<Image>,
    }
    Ok(resp.json::<R>().await?.images)
}
