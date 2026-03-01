//! Data models for the CloudFang store.

use serde::{Deserialize, Serialize};

/// An incident record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub timestamp: String,
    pub severity: String, // "info", "warning", "critical"
    pub resource_id: String,
    pub resource_name: Option<String>,
    pub description: String,
    pub action_taken: Option<String>,
    pub resolved: bool,
}

impl Incident {
    pub fn new(
        severity: &str,
        resource_id: &str,
        resource_name: Option<&str>,
        description: &str,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            severity: severity.to_string(),
            resource_id: resource_id.to_string(),
            resource_name: resource_name.map(String::from),
            description: description.to_string(),
            action_taken: None,
            resolved: false,
        }
    }

    pub fn severity_emoji(&self) -> &str {
        match self.severity.as_str() {
            "critical" => "🔴",
            "warning" => "🟡",
            "info" => "🔵",
            _ => "⚪",
        }
    }
}
