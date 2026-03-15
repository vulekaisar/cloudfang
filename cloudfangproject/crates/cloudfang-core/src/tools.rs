//! Tool Registry — Registers OpenStack operations as callable tools for the LLM.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definition of a tool the LLM can call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Result of a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
}

/// Trait for executable tools.
#[async_trait]
pub trait Tool: Send + Sync {
    fn definition(&self) -> ToolDef;
    async fn execute(
        &self,
        session: &mut cloudfang_ops::OpenStackSession,
        args: serde_json::Value,
    ) -> anyhow::Result<ToolResult>;
}

/// Registry that holds all available tools.
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.definition().name.clone();
        tracing::info!("🔧 Registered tool: {}", name);
        self.tools.insert(name, tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    /// Get all tool definitions (for sending to the LLM).
    pub fn definitions(&self) -> Vec<ToolDef> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    pub async fn execute(
        &self,
        session: &mut cloudfang_ops::OpenStackSession,
        name: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        match self.tools.get(name) {
            Some(tool) => tool.execute(session, args).await,
            None => Ok(ToolResult {
                success: false,
                output: format!("Tool '{}' not found", name),
            }),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
