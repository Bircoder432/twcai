//! Types for responses API (OpenAI-compatible)

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request to create a response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponseRequest {
    /// Model for the response (ignored as agent has its own configuration)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Instructions for generating the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Input text or messages for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ResponseInput>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    /// Temperature for response generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Additional metadata for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Value>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Options for streaming (only when stream: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<Value>,
    /// Run model in background mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// Configuration for text response from model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    /// How the model should choose tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    /// Allow model to execute tool calls in parallel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Maximum number of built-in tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u32>,
    /// ID of previous response for multi-turn dialogs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Conversation this response belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Value>,
    /// Additional output data to include in model response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Whether to store the generated response for later retrieval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Alternative to temperature (nucleus sampling)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Number of most likely tokens to return at each position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// Truncation strategy for model response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,
    /// Service tier for request processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    /// Stable identifier for detecting policy violations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    /// Used by OpenAI for caching similar requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Reference to prompt template and its variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Value>,
    /// Configuration for reasoning models (gpt-5 and o-series)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Value>,
    /// Deprecated: use safety_identifier or prompt_cache_key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Input can be a string or array of messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseInput {
    /// Simple text input
    Text(String),
    /// Array of message objects
    Messages(Vec<Value>),
}

/// Token usage for response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the response
    pub completion_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
}

/// Response object (OpenAI-compatible)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Unique identifier for the response
    pub id: String,
    /// Object type - always "response"
    pub object: String,
    /// Unix timestamp of creation
    pub created_at: i64,
    /// Model identifier
    pub model: String,
    /// Response status
    pub status: String,
    /// Token usage information
    pub usage: ResponseUsage,
    /// Additional fields from API
    #[serde(flatten)]
    pub extra: Value,
}

/// Query parameters for getting a response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponseQuery {
    /// Additional fields to include in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Enable stream obfuscation for side-channel attack protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_obfuscation: Option<bool>,
    /// Event sequence number to start streaming after
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<u32>,
    /// Stream model response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}
