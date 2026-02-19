//! TWCai - Rust client library for Timeweb Cloud AI API
//!
//! Provides OpenAI-compatible interfaces for AI agent interactions including:
//! - Chat completions with multimodal support (text, image, audio)
//! - Response management
//! - Conversation lifecycle management
//!
//! # Example
//! ```
//! use twcai::{CloudAIClient, types::*};
//! use twcai::api::AgentClientExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudAIClient::builder()
//!     .base_url("https://agent.timeweb.cloud")
//!     .token(std::env::var("TWCAI_API_TOKEN")?)
//!     .build()?;
//!
//! let request = ChatCompletionRequest {
//!     messages: vec![ChatMessage::user("Hello!")],
//!     ..Default::default()
//! };
//!
//! let response = client
//!     .chat_completions("agent-123", request)
//!     .await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

pub mod api;
mod client;
mod error;
pub mod types;

pub use client::{ClientBuilder, CloudAIClient};
pub use error::{Result, TwcError};

use std::sync::Arc;

/// Shared HTTP client configuration
#[derive(Clone)]
pub struct ClientConfig {
    /// Base URL for API requests
    pub base_url: Arc<str>,
    /// Authentication token
    pub token: Arc<str>,
    /// HTTP client instance
    pub http_client: reqwest::Client,
}

impl ClientConfig {
    /// Create authorization header value
    pub(crate) fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}
