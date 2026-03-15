//! Operational Tools — Real OpenStack operations for the LLM.

use async_trait::async_trait;
use crate::tools::{Tool, ToolDef, ToolResult};
use cloudfang_ops::OpenStackSession;
use serde_json::json;

/// Tool to list virtual machine instances.
pub struct ListServersTool;

#[async_trait]
impl Tool for ListServersTool {
    fn definition(&self) -> ToolDef {
        ToolDef {
            name: "list_servers".to_string(),
            description: "Lists all virtual machine instances in the current project.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {}
            }),
        }
    }

    async fn execute(
        &self,
        session: &mut OpenStackSession,
        _args: serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        let servers = cloudfang_ops::nova::list_servers(session).await?;
        let output = servers
            .iter()
            .map(|s| format!("{} ({}) - Status: {}", s.name, s.id, s.status))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ToolResult {
            success: true,
            output: if output.is_empty() { "No VMs found.".to_string() } else { output },
        })
    }
}

/// Tool to perform actions on a virtual machine (reboot, start, stop).
pub struct ServerActionTool;

#[async_trait]
impl Tool for ServerActionTool {
    fn definition(&self) -> ToolDef {
        ToolDef {
            name: "server_action".to_string(),
            description: "Performs an action on a virtual machine instance.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "server_id": { "type": "string", "description": "The ID of the server" },
                    "action": { 
                        "type": "string", 
                        "enum": ["reboot", "start", "stop", "pause", "unpause"],
                        "description": "The action to perform"
                    }
                },
                "required": ["server_id", "action"]
            }),
        }
    }

    async fn execute(
        &self,
        session: &mut OpenStackSession,
        args: serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        let server_id = args["server_id"].as_str().ok_or_else(|| anyhow::anyhow!("Missing server_id"))?;
        let action_str = args["action"].as_str().ok_or_else(|| anyhow::anyhow!("Missing action"))?;

        let action = match action_str {
            "reboot" => cloudfang_ops::nova::ServerAction::Reboot(cloudfang_ops::nova::RebootType::Soft),
            "start" => cloudfang_ops::nova::ServerAction::Start,
            "stop" => cloudfang_ops::nova::ServerAction::Stop,
            "pause" => cloudfang_ops::nova::ServerAction::Pause,
            "unpause" => cloudfang_ops::nova::ServerAction::Unpause,
            _ => anyhow::bail!("Invalid action: {}", action_str),
        };

        match cloudfang_ops::nova::server_action(session, server_id, action).await {
            Ok(_) => Ok(ToolResult {
                success: true,
                output: format!("Successfully executed {} on server {}", action_str, server_id),
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: format!("Failed to execute {} on server {}: {}", action_str, server_id, e),
            }),
        }
    }
}
