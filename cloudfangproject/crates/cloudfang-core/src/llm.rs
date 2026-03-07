//! LLM Client — Wrapper around async-openai for intelligent ops decisions.
//! Supports OpenAI API and any OpenAI-compatible endpoint (e.g., Ollama).

use crate::config::LlmConfig;
use crate::tools::ToolDef;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A message in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// An LLM response that may contain a tool call.
#[derive(Debug, Clone)]
pub enum LlmResponse {
    /// Plain text response.
    Text(String),
    /// The LLM wants to call a tool.
    ToolCall {
        tool_name: String,
        arguments: serde_json::Value,
    },
}

/// The LLM client.
pub struct LlmClient {
    config: LlmConfig,
    client: reqwest::Client,
    system_prompt: String,
}

impl LlmClient {
    pub fn new(config: LlmConfig, system_prompt: &str) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
            system_prompt: system_prompt.to_string(),
        }
    }

    /// Send a chat completion request with available tools.
    pub async fn chat(&self, messages: &[Message], tools: &[ToolDef]) -> Result<LlmResponse> {
        let api_key = self
            .config
            .api_key
            .clone()
            .or_else(|| std::env::var("OPENAI_API_KEY").ok())
            .unwrap_or_default();

        // Build messages array with system prompt
        let mut all_messages = vec![serde_json::json!({
            "role": "system",
            "content": self.system_prompt
        })];
        for msg in messages {
            all_messages.push(serde_json::json!({
                "role": msg.role,
                "content": msg.content
            }));
        }

        // Build request body
        let mut body = serde_json::json!({
            "model": self.config.model,
            "messages": all_messages,
        });

        // Add tools if available
        if !tools.is_empty() {
            let tool_defs: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters
                        }
                    })
                })
                .collect();
            body["tools"] = serde_json::json!(tool_defs);
        }

        let url = format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("LLM API error ({}): {}", status, text);
        }

        let resp_body: serde_json::Value = resp.json().await?;

        // Parse response — check for tool calls first
        if let Some(tool_calls) = resp_body["choices"][0]["message"]["tool_calls"].as_array() {
            if let Some(tc) = tool_calls.first() {
                let name = tc["function"]["name"].as_str().unwrap_or("").to_string();
                let args_str = tc["function"]["arguments"].as_str().unwrap_or("{}");
                let arguments: serde_json::Value =
                    serde_json::from_str(args_str).unwrap_or_default();
                return Ok(LlmResponse::ToolCall {
                    tool_name: name,
                    arguments,
                });
            }
        }

        // Plain text response
        let content = resp_body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(LlmResponse::Text(content))
    }
}
