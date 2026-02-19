//! Cloud AI Client implementation

use std::sync::Arc;

use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::{ClientConfig, Result, TwcError};

/// Main client for Timeweb Cloud AI API
#[derive(Clone)]
pub struct CloudAIClient {
    pub(crate) config: ClientConfig,
}

/// Builder for CloudAIClient
pub struct ClientBuilder {
    base_url: Option<String>,
    token: Option<String>,
    timeout: Option<std::time::Duration>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: Some("https://agent.timeweb.cloud".to_string()),
            token: None,
            timeout: Some(std::time::Duration::from_secs(120)),
        }
    }
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL for API requests
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the authentication token
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, duration: std::time::Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<CloudAIClient> {
        let base_url = self
            .base_url
            .ok_or_else(|| TwcError::Configuration("Base URL is required".to_string()))?;

        let token = self
            .token
            .ok_or_else(|| TwcError::Configuration("Token is required".to_string()))?;

        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );

        let http_client = reqwest::Client::builder()
            .timeout(self.timeout.unwrap_or(std::time::Duration::from_secs(120)))
            .default_headers(headers)
            .build()
            .map_err(TwcError::Http)?;

        let config = ClientConfig {
            base_url: Arc::from(base_url.into_boxed_str()),
            token: Arc::from(token.into_boxed_str()),
            http_client,
        };

        Ok(CloudAIClient { config })
    }
}

impl CloudAIClient {
    /// Create a new client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Create a client from environment variables
    /// 
    /// Uses TWCAI_BASE_URL (optional, defaults to https://agent.timeweb.cloud)
    /// and TWCAI_API_TOKEN (required)
    pub fn from_env() -> Result<Self> {
        let base_url = std::env::var("TWCAI_BASE_URL")
            .unwrap_or_else(|_| "https://agent.timeweb.cloud".to_string());
        
        let token = std::env::var("TWCAI_API_TOKEN")
            .map_err(|_| TwcError::Configuration(
                "TWCAI_API_TOKEN environment variable not set".to_string()
            ))?;

        Self::builder()
            .base_url(base_url)
            .token(token)
            .build()
    }

    /// Get the client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }
}
