//! Heat Orchestration — Stack Management

use crate::{OpenStackSession, OpsError, OpsResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    pub id: String,
    pub stack_name: String,
    pub stack_status: String,
    pub stack_status_reason: Option<String>,
    pub creation_time: Option<String>,
    pub updated_time: Option<String>,
}

pub async fn list_stacks(session: &mut OpenStackSession) -> OpsResult<Vec<Stack>> {
    session.ensure_authenticated().await?;
    let url = format!(
        "{}/stacks",
        session.endpoint("orchestration")?.trim_end_matches('/')
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
        stacks: Vec<Stack>,
    }
    Ok(resp.json::<R>().await?.stacks)
}

pub async fn get_stack(
    session: &mut OpenStackSession,
    stack_name: &str,
    stack_id: &str,
) -> OpsResult<Stack> {
    session.ensure_authenticated().await?;
    let url = format!(
        "{}/stacks/{}/{}",
        session.endpoint("orchestration")?.trim_end_matches('/'),
        stack_name,
        stack_id
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
        stack: Stack,
    }
    Ok(resp.json::<R>().await?.stack)
}
