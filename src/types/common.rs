//! Common types shared across API modules

use serde::{Deserialize, Serialize};

/// Token usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Usage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the generated completion
    pub completion_tokens: u32,
    /// Total number of tokens used in the request
    pub total_tokens: u32,
}

/// Function call definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionCall {
    /// The name of the function to call
    pub name: String,
}

/// Response format for text output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFormatText {
    /// The type of response format - always "text"
    #[serde(rename = "type")]
    pub format_type: String,
}

impl Default for ResponseFormatText {
    fn default() -> Self {
        Self {
            format_type: "text".to_string(),
        }
    }
}

/// Response format for JSON object output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFormatJsonObject {
    /// The type of response format - always "json_object"
    #[serde(rename = "type")]
    pub format_type: String,
}

impl Default for ResponseFormatJsonObject {
    fn default() -> Self {
        Self {
            format_type: "json_object".to_string(),
        }
    }
}

/// Response format for JSON schema output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFormatJsonSchema {
    /// The type of response format - always "json_schema"
    #[serde(rename = "type")]
    pub format_type: String,
    /// JSON schema definition
    pub json_schema: serde_json::Value,
}

/// Function tool definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionTool {
    /// The type of tool - always "function"
    #[serde(rename = "type")]
    pub tool_type: String,
    /// The function definition
    pub function: serde_json::Value,
}

/// Custom tool definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomTool {
    /// The type of tool - always "custom"
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Custom tool definition
    pub custom: serde_json::Value,
}

/// Text content item for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextContent {
    /// Content type - always "text"
    #[serde(rename = "type")]
    pub content_type: String,
    /// The text content
    pub text: String,
}

/// Image URL specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageUrl {
    /// The URL of the image
    pub url: String,
    /// The detail level of the image (low, high, auto)
    pub detail: Option<String>,
}

/// Image URL content item for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageUrlContent {
    /// Content type - always "image_url"
    #[serde(rename = "type")]
    pub content_type: String,
    /// Image URL object
    pub image_url: ImageUrl,
}

/// Input audio specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputAudio {
    /// Base64 encoded audio data
    pub data: String,
    /// Audio format (wav, mp3, m4a, ogg, flac, webm)
    pub format: String,
}

/// Input audio content item for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputAudioContent {
    /// Content type - always "input_audio"
    #[serde(rename = "type")]
    pub content_type: String,
    /// Input audio object
    pub input_audio: InputAudio,
}

/// File content item for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileContent {
    /// Content type - always "file"
    #[serde(rename = "type")]
    pub content_type: String,
    /// File object (OpenAI File type)
    pub file: serde_json::Value,
}

/// Refusal content item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefusalContent {
    /// Content type - always "refusal"
    #[serde(rename = "type")]
    pub content_type: String,
    /// Refusal message
    pub refusal: String,
}

/// Stream options for streaming responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamOptions {
    /// Whether to include usage information in streaming responses
    pub include_usage: Option<bool>,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// Model identifier
    pub id: String,
    /// Object type, always "model"
    pub object: String,
    /// Unix timestamp when the model was created
    pub created: i64,
    /// Organization that owns the model
    pub owned_by: String,
}

/// List of models response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelsResponse {
    /// Object type, always "list"
    pub object: String,
    /// Array of available models
    pub data: Vec<Model>,
}

/// Finish reason for completions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural stop point or stop sequence encountered
    Stop,
    /// Maximum token limit reached
    Length,
    /// Content was filtered
    ContentFilter,
    /// Model called a tool
    ToolCalls,
}
