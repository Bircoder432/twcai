//! Error types for TWCai library

use thiserror::Error;

/// Result type alias for TWCai operations
pub type Result<T> = std::result::Result<T, TwcError>;

/// Main error type for TWCai operations
#[derive(Error, Debug)]
pub enum TwcError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing failed
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Authentication failed (401)
    #[error("Authentication failed - invalid or expired token")]
    Unauthorized,

    /// Resource not found (404)
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Domain not whitelisted or agent suspended (403)
    #[error("Access forbidden - domain not whitelisted or agent suspended")]
    Forbidden,

    /// Invalid request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Server error (5xx)
    #[error("Server error: {status} - {message}")]
    ServerError {
        /// HTTP status code
        status: u16,
        /// Error message from server
        message: String,
    },

    /// Client configuration error
    #[error("Client configuration error: {0}")]
    Configuration(String),

    /// Response cancelled
    #[error("Response was cancelled")]
    Cancelled,
}

impl TwcError {
    /// Create error from HTTP status code and optional message
    pub(crate) fn from_status(status: reqwest::StatusCode, message: Option<String>) -> Self {
        match status.as_u16() {
            401 => TwcError::Unauthorized,
            403 => TwcError::Forbidden,
            404 => TwcError::NotFound(message.unwrap_or_else(|| "Resource not found".to_string())),
            500..=599 => TwcError::ServerError {
                status: status.as_u16(),
                message: message.unwrap_or_else(|| "Internal server error".to_string()),
            },
            _ => TwcError::InvalidRequest(message.unwrap_or_else(|| "Bad request".to_string())),
        }
    }
}
