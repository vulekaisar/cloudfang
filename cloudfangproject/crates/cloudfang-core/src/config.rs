//! CloudFang Configuration — loaded from `cloudfang.toml`

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Top-level configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFangConfig {
    pub openstack: OpenStackConfig,
    pub llm: LlmConfig,
    pub store: StoreConfig,
    pub hands: HandsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenStackConfig {
    pub auth_url: String,
    pub username: String,
    pub password: String,
    pub project_name: String,
    #[serde(default = "default_domain")]
    pub domain_name: String,
}

fn default_domain() -> String {
    "Default".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    #[serde(default = "default_provider")]
    pub provider: String,
    pub api_key: Option<String>,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

fn default_provider() -> String {
    "openai".to_string()
}
fn default_model() -> String {
    "gpt-4o-mini".to_string()
}
fn default_base_url() -> String {
    "https://api.openai.com/v1".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreConfig {
    #[serde(default = "default_db_path")]
    pub db_path: String,
}

fn default_db_path() -> String {
    "cloudfang.db".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandsConfig {
    #[serde(default = "default_monitor_interval")]
    pub monitor_interval_secs: u64,
    #[serde(default = "default_backup_cron")]
    pub backup_cron: String,
    #[serde(default = "default_scale_interval")]
    pub scale_check_interval_secs: u64,
}

fn default_monitor_interval() -> u64 {
    300
} // 5 minutes
fn default_backup_cron() -> String {
    "0 2 * * *".to_string()
} // Daily at 2 AM
fn default_scale_interval() -> u64 {
    900
} // 15 minutes

impl CloudFangConfig {
    /// Load config from a TOML file.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Convert OpenStack config to credentials.
    pub fn to_credentials(&self) -> cloudfang_ops::OpenStackCredentials {
        let password = std::env::var("OS_PASSWORD").unwrap_or_else(|_| self.openstack.password.clone());
        let username = std::env::var("OS_USERNAME").unwrap_or_else(|_| self.openstack.username.clone());
        let project_name = std::env::var("OS_PROJECT_NAME").unwrap_or_else(|_| self.openstack.project_name.clone());
        let auth_url = std::env::var("OS_AUTH_URL").unwrap_or_else(|_| self.openstack.auth_url.clone());
        let domain_name = std::env::var("OS_DOMAIN_NAME").unwrap_or_else(|_| self.openstack.domain_name.clone());

        cloudfang_ops::OpenStackCredentials {
            auth_url,
            username,
            password,
            project_name,
            domain_name,
        }
    }
}
