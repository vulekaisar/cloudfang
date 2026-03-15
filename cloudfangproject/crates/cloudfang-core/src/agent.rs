//! Agent Loop — The core agent runtime.
//!
//! Receives tasks, consults the LLM, executes tools, and reports results.

use crate::llm::{LlmClient, LlmResponse, Message};
use crate::tools::ToolRegistry;
use anyhow::Result;

/// The main agent that processes tasks.
pub struct Agent {
    llm: LlmClient,
    tools: ToolRegistry,
}

impl Agent {
    pub fn new(llm: LlmClient, tools: ToolRegistry) -> Self {
        Self { llm, tools }
    }

    /// Process a user query or automated task through the agent loop.
    /// The agent will call tools as needed and return the final response.
    pub async fn process(
        &self,
        session: &mut cloudfang_ops::OpenStackSession,
        input: &str,
    ) -> Result<String> {
        let mut messages = vec![Message {
            role: "user".to_string(),
            content: input.to_string(),
        }];

        let tool_defs = self.tools.definitions();
        let max_iterations = 5;

        for iteration in 0..max_iterations {
            tracing::debug!("Agent loop iteration {}", iteration + 1);

            let response = self.llm.chat(&messages, &tool_defs).await?;

            match response {
                LlmResponse::Text(text) => {
                    tracing::info!("Agent response: {}", &text[..text.len().min(100)]);
                    return Ok(text);
                }
                LlmResponse::ToolCall {
                    tool_name,
                    arguments,
                } => {
                    tracing::info!("🔧 Agent calling tool: {}", tool_name);

                    let result = self
                        .tools
                        .execute(session, &tool_name, arguments.clone())
                        .await?;

                    // Add assistant tool call and tool result to conversation
                    messages.push(Message {
                        role: "assistant".to_string(),
                        content: format!("[Calling tool: {}]", tool_name),
                    });
                    messages.push(Message {
                        role: "user".to_string(),
                        content: format!(
                            "Tool '{}' returned (success={}): {}",
                            tool_name, result.success, result.output
                        ),
                    });
                }
            }
        }

        Ok("Agent reached maximum iterations without a final response.".to_string())
    }

    /// Get the tool registry for registration.
    pub fn tools_mut(&mut self) -> &mut ToolRegistry {
        &mut self.tools
    }
}
