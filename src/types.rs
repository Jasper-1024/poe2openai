use crate::utils::deserialize_content;
use poe_api_process::types::{Tool, ToolCall};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub prompt_tokens_details: PromptTokensDetails,
}

#[derive(Serialize)]
pub struct PromptTokensDetails {
    pub cached_tokens: u32,
}

#[derive(Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    pub stream_options: Option<StreamOptions>,
}

#[derive(Deserialize)]
pub struct StreamOptions {
    pub include_usage: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: String,
    #[serde(deserialize_with = "deserialize_content")]
    pub content: String,
}

#[derive(Deserialize)]
pub struct ContentItem {
    pub text: String,
}

#[derive(Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct CompletionChoice {
    pub index: u32,
    pub message: CompletionMessage,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
}

#[derive(Serialize)]
pub struct CompletionMessage {
    pub role: String,
    pub content: String,
    pub refusal: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Serialize)]
pub struct Choice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub refusal: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize)]
pub struct OpenAIErrorResponse {
    pub error: OpenAIError,
}

#[derive(Serialize)]
pub struct OpenAIError {
    pub message: String,
    pub r#type: String,
    pub code: String,
    pub param: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) enable: Option<bool>,
    pub(crate) models: std::collections::HashMap<String, ModelConfig>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub(crate) struct ModelConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mapping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) replace_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enable: Option<bool>,
}
