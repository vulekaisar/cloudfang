//! Neutron Networking — Network, Subnet, Port, Floating IP operations

use crate::{OpenStackSession, OpsError, OpsResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "admin_state_up")]
    pub admin_state_up: bool,
    pub subnets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub id: String,
    pub name: String,
    pub network_id: String,
    pub cidr: String,
    pub gateway_ip: Option<String>,
    pub enable_dhcp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub id: String,
    pub name: Option<String>,
    pub network_id: String,
    pub status: String,
    pub device_id: Option<String>,
    pub device_owner: Option<String>,
    pub fixed_ips: Vec<FixedIp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedIp {
    pub subnet_id: String,
    pub ip_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatingIp {
    pub id: String,
    pub floating_ip_address: String,
    pub fixed_ip_address: Option<String>,
    pub port_id: Option<String>,
    pub status: String,
}

fn neutron_url(session: &OpenStackSession) -> OpsResult<String> {
    Ok(session
        .endpoint("network")?
        .trim_end_matches('/')
        .to_string())
}

pub async fn list_networks(session: &mut OpenStackSession) -> OpsResult<Vec<Network>> {
    session.ensure_authenticated().await?;
    let url = format!("{}/v2.0/networks", neutron_url(session)?);
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
        networks: Vec<Network>,
    }
    Ok(resp.json::<R>().await?.networks)
}

pub async fn list_subnets(session: &mut OpenStackSession) -> OpsResult<Vec<Subnet>> {
    session.ensure_authenticated().await?;
    let url = format!("{}/v2.0/subnets", neutron_url(session)?);
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
        subnets: Vec<Subnet>,
    }
    Ok(resp.json::<R>().await?.subnets)
}

pub async fn list_ports(session: &mut OpenStackSession) -> OpsResult<Vec<Port>> {
    session.ensure_authenticated().await?;
    let url = format!("{}/v2.0/ports", neutron_url(session)?);
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
        ports: Vec<Port>,
    }
    Ok(resp.json::<R>().await?.ports)
}

pub async fn list_floating_ips(session: &mut OpenStackSession) -> OpsResult<Vec<FloatingIp>> {
    session.ensure_authenticated().await?;
    let url = format!("{}/v2.0/floatingips", neutron_url(session)?);
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
        floatingips: Vec<FloatingIp>,
    }
    Ok(resp.json::<R>().await?.floatingips)
}
