//! AI Agents Client API endpoints
//!
//! Provides methods for:
//! - Simple agent calls
//! - Chat completions (OpenAI-compatible)
//! - Text completions (legacy)
//! - Model listing
//! - Widget embed code

use reqwest::header::AUTHORIZATION;

use crate::{CloudAIClient, Result, TwcError, types::*};

/// Extension trait for agent client operations
pub trait AgentClientExt {
    /// Call AI agent with simple message
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/call
    fn call_agent(
        &self,
        agent_access_id: &str,
        request: AgentCallRequest,
    ) -> impl std::future::Future<Output = Result<AgentCallResponse>> + Send;

    /// OpenAI-compatible chat completions
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/v1/chat/completions
    fn chat_completions(
        &self,
        agent_access_id: &str,
        request: ChatCompletionRequest,
    ) -> impl std::future::Future<Output = Result<ChatCompletionResponse>> + Send;

    /// OpenAI-compatible text completions (legacy)
    ///
    /// POST /api/v1/cloud-ai/agents/{agent_access_id}/v1/completions
    #[deprecated(since = "0.1.0", note = "Use chat_completions instead")]
    fn text_completions(
        &self,
        agent_access_id: &str,
        request: TextCompletionRequest,
    ) -> impl std::future::Future<Output = Result<TextCompletionResponse>> + Send;

    /// List available models
    ///
    /// GET /api/v1/cloud-ai/agents/{agent_access_id}/v1/models
    fn list_models(
        &self,
        agent_access_id: &str,
    ) -> impl std::future::Future<Output = Result<ModelsResponse>> + Send;

    /// Get widget embed JavaScript code
    ///
    /// GET /api/v1/cloud-ai/agents/{agent_access_id}/embed.js
    fn get_embed_code(
        &self,
        agent_access_id: &str,
        collapsed: Option<bool>,
        referer: &str,
        origin: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;
}

impl AgentClientExt for CloudAIClient {
    async fn call_agent(
        &self,
        agent_access_id: &str,
        request: AgentCallRequest,
    ) -> Result<AgentCallResponse> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/call",
            self.config.base_url, agent_access_id
        );

        let response = self
            .config
            .http_client
            .post(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .header("x-proxy-source", "twcai-rust")
            .json(&request)
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn chat_completions(
        &self,
        agent_access_id: &str,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/chat/completions",
            self.config.base_url, agent_access_id
        );

        let response = self
            .config
            .http_client
            .post(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .header("x-proxy-source", "twcai-rust")
            .json(&request)
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    #[allow(deprecated)]
    async fn text_completions(
        &self,
        agent_access_id: &str,
        request: TextCompletionRequest,
    ) -> Result<TextCompletionResponse> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/completions",
            self.config.base_url, agent_access_id
        );

        let response = self
            .config
            .http_client
            .post(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .header("x-proxy-source", "twcai-rust")
            .json(&request)
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn list_models(&self, agent_access_id: &str) -> Result<ModelsResponse> {
        let url = format!(
            "{}/api/v1/cloud-ai/agents/{}/v1/models",
            self.config.base_url, agent_access_id
        );

        let response = self
            .config
            .http_client
            .get(&url)
            .header(AUTHORIZATION, self.config.auth_header())
            .send()
            .await
            .map_err(TwcError::Http)?;

        handle_response(response).await
    }

    async fn get_embed_code(
        &self,
        agent_access_id: &str,
        collapsed: Option<bool>,
        referer: &str,
        origin: &str,
    ) -> Result<String> {
        let mut url = format!(
            "{}/api/v1/cloud-ai/agents/{}/embed.js",
            self.config.base_url, agent_access_id
        );

        if let Some(collapsed) = collapsed {
            url.push_str(&format!("?collapsed={}", collapsed));
        }

        let response = self
            .config
            .http_client
            .get(&url)
            .header("referer", referer)
            .header("origin", origin)
            .send()
            .await
            .map_err(TwcError::Http)?;

        let status = response.status();
        if status.is_success() {
            response.text().await.map_err(TwcError::Http)
        } else {
            Err(TwcError::from_status(status, None))
        }
    }
}

/// Handle HTTP response and parse JSON or return appropriate error
async fn handle_response<T: serde::de::DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();

    if status.is_success() {
        response.json::<T>().await.map_err(TwcError::Http)
    } else {
        let text = response.text().await.ok();
        Err(TwcError::from_status(status, text))
    }
}

/// Request for text completions (legacy)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TextCompletionRequest {
    /// The prompt to generate completions for
    pub prompt: String,
    /// The model to use for completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Sampling temperature (0-2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Nucleus sampling parameter (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// How many completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Whether to stream back partial progress
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Include log probabilities on most likely tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    /// Echo back the prompt in addition to completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    /// Up to 4 sequences where API stops generating further tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Presence penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Frequency penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Generates best_of completions server-side and returns the "best"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    /// Unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Log probabilities for text completion
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TextCompletionLogprobs {
    /// Tokens chosen by the model
    pub tokens: Vec<String>,
    /// Log probability of each token
    pub token_logprobs: Vec<f32>,
    /// Top logprobs for each token
    pub top_logprobs: serde_json::Value,
    /// Character offsets for each token
    pub text_offset: Vec<u32>,
}

/// Choice in text completion response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TextCompletionChoice {
    /// The generated text
    pub text: String,
    /// The index of this choice
    pub index: u32,
    /// Log probability information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<TextCompletionLogprobs>,
    /// The reason the model stopped generating tokens
    pub finish_reason: String,
}

/// Token usage for text completion
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TextCompletionUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the generated completion
    pub completion_tokens: u32,
    /// Total number of tokens used
    pub total_tokens: u32,
}

/// Text completion response (legacy)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TextCompletionResponse {
    /// Unique identifier for the completion
    pub id: String,
    /// Object type - always "text_completion"
    pub object: String,
    /// Unix timestamp when the completion was created
    pub created: i64,
    /// The model used for completion
    pub model: String,
    /// Array of completion choices
    pub choices: Vec<TextCompletionChoice>,
    /// Usage statistics for the completion
    pub usage: TextCompletionUsage,
}
